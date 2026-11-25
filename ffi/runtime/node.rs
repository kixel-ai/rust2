//! The runtime of the Node-API addon the crate's `node` feature builds, which `node/rt.ts`
//! loads: Node-API, found in the process that loads the addon rather than linked, so the
//! one library serves every other language too; the scratch memory the TypeScript lowers
//! values into; the table of handles' addresses the TypeScript holds by id, which counts
//! the calls holding each; the conversions each generated native function makes of its own
//! arguments, holds, and failed status; and the calls into JavaScript.
//!
//! JavaScript runs only on its thread, so a call into it from any other, and the drop of
//! an implementation there, waits in a queue for that thread to run: from its event loop,
//! which a thread-safe function tells of each, or, while it waits in a call from
//! JavaScript, from that wait. A call waits so only while JavaScript holds a token Rust
//! may call through from another thread: then it runs on a worker thread, and each side
//! spins briefly for the other before it parks. The first
//! environment to load the addon keeps it for the process's life, as the table and the
//! scratch memory are its: any other's load throws, even once the first has exited.

use std::cell::{Cell, UnsafeCell};
use std::collections::VecDeque;
use std::ffi::{CStr, c_char, c_void};
use std::mem::{ManuallyDrop, MaybeUninit};
use std::panic::AssertUnwindSafe;
use std::ptr::{null, null_mut};
use std::sync::atomic::{AtomicBool, AtomicPtr, AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex, MutexGuard, OnceLock};
use std::time::{Duration, Instant};

use crate::rt;

pub type Env = *mut c_void;
pub type Value = *mut c_void;
pub type Info = *mut c_void;
pub type Callback = unsafe extern "C" fn(Env, Info) -> Value;
type Ref = *mut c_void;
type Tsfn = *mut c_void;
type Status = i32;
type CallJs = unsafe extern "C" fn(Env, Value, *mut c_void, *mut c_void);
type Finalize = unsafe extern "C" fn(Env, *mut c_void, *mut c_void);

/// What the addon calls of Node-API, each found by name in the process.
struct Api {
	create_function: unsafe extern "C" fn(Env, *const c_char, usize, Callback, *mut c_void, *mut Value) -> Status,
	set_named_property: unsafe extern "C" fn(Env, Value, *const c_char, Value) -> Status,
	get_named_property: unsafe extern "C" fn(Env, Value, *const c_char, *mut Value) -> Status,
	get_cb_info: unsafe extern "C" fn(Env, Info, *mut usize, *mut Value, *mut Value, *mut *mut c_void) -> Status,
	get_value_int64: unsafe extern "C" fn(Env, Value, *mut i64) -> Status,
	get_value_bool: unsafe extern "C" fn(Env, Value, *mut bool) -> Status,
	get_value_bigint_int64: unsafe extern "C" fn(Env, Value, *mut i64, *mut bool) -> Status,
	get_typedarray_info: unsafe extern "C" fn(Env, Value, *mut i32, *mut usize, *mut *mut c_void, *mut Value, *mut usize) -> Status,
	create_int32: unsafe extern "C" fn(Env, i32, *mut Value) -> Status,
	create_uint32: unsafe extern "C" fn(Env, u32, *mut Value) -> Status,
	create_double: unsafe extern "C" fn(Env, f64, *mut Value) -> Status,
	create_bigint_int64: unsafe extern "C" fn(Env, i64, *mut Value) -> Status,
	create_object: unsafe extern "C" fn(Env, *mut Value) -> Status,
	get_value_double: unsafe extern "C" fn(Env, Value, *mut f64) -> Status,
	get_boolean: unsafe extern "C" fn(Env, bool, *mut Value) -> Status,
	create_string_utf8: unsafe extern "C" fn(Env, *const c_char, usize, *mut Value) -> Status,
	get_undefined: unsafe extern "C" fn(Env, *mut Value) -> Status,
	create_external_arraybuffer:
		unsafe extern "C" fn(Env, *mut c_void, usize, *const c_void, *mut c_void, *mut Value) -> Status,
	create_reference: unsafe extern "C" fn(Env, Value, u32, *mut Ref) -> Status,
	get_reference_value: unsafe extern "C" fn(Env, Ref, *mut Value) -> Status,
	call_function: unsafe extern "C" fn(Env, Value, Value, usize, *const Value, *mut Value) -> Status,
	open_handle_scope: unsafe extern "C" fn(Env, *mut *mut c_void) -> Status,
	close_handle_scope: unsafe extern "C" fn(Env, *mut c_void) -> Status,
	throw_range_error: unsafe extern "C" fn(Env, *const c_char, *const c_char) -> Status,
	get_and_clear_last_exception: unsafe extern "C" fn(Env, *mut Value) -> Status,
	coerce_to_string: unsafe extern "C" fn(Env, Value, *mut Value) -> Status,
	get_value_string_utf8: unsafe extern "C" fn(Env, Value, *mut c_char, usize, *mut usize) -> Status,
	throw: unsafe extern "C" fn(Env, Value) -> Status,
	throw_error: unsafe extern "C" fn(Env, *const c_char, *const c_char) -> Status,
	adjust_external_memory: unsafe extern "C" fn(Env, i64, *mut i64) -> Status,
	create_threadsafe_function: unsafe extern "C" fn(
		Env,
		Value,
		Value,
		Value,
		usize,
		usize,
		*mut c_void,
		Option<Finalize>,
		*mut c_void,
		Option<CallJs>,
		*mut Tsfn,
	) -> Status,
	call_threadsafe_function: unsafe extern "C" fn(Tsfn, *mut c_void, i32) -> Status,
	ref_threadsafe_function: unsafe extern "C" fn(Env, Tsfn) -> Status,
	unref_threadsafe_function: unsafe extern "C" fn(Env, Tsfn) -> Status,
}

static API: OnceLock<Api> = OnceLock::new();

#[inline]
fn api() -> &'static Api {
	// SAFETY: set as the addon registers, before any call can reach this.
	unsafe { API.get().unwrap_unchecked() }
}

#[cfg(unix)]
unsafe extern "C" {
	fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
}

/// The handle `dlsym` searches every loaded object by: null on Linux, and -2 on the BSDs,
/// macOS, and Solaris's family, where null searches only from the caller.
#[cfg(any(target_os = "linux", target_os = "android"))]
const RTLD_DEFAULT: *mut c_void = null_mut();
#[cfg(all(unix, not(any(target_os = "linux", target_os = "android"))))]
const RTLD_DEFAULT: *mut c_void = -2isize as *mut c_void;

#[cfg(windows)]
#[link(name = "kernel32")]
unsafe extern "system" {
	fn GetModuleHandleW(name: *const u16) -> *mut c_void;
	fn GetProcAddress(module: *mut c_void, name: *const c_char) -> *mut c_void;
	fn VirtualAlloc(at: *mut c_void, size: usize, kind: u32, protect: u32) -> *mut c_void;
}

/// The process's function of `name`, which Node, Bun, and Deno each export.
fn symbol(name: &CStr) -> *mut c_void {
	#[cfg(unix)]
	// SAFETY: a C string, searched for in what the process loaded.
	let found = unsafe { dlsym(RTLD_DEFAULT, name.as_ptr()) };
	#[cfg(windows)]
	// SAFETY: the process's own module, searched by a C string.
	let found = unsafe { GetProcAddress(GetModuleHandleW(null()), name.as_ptr()) };
	if found.is_null() {
		rt::fail(
			"node",
			&format!("the process that loaded the addon has no {}, so it is no Node-API host", name.to_string_lossy()),
		);
	}
	found
}

/// Find Node-API in the process.
fn resolve() -> Api {
	macro_rules! find {
		($name:literal) => {
			// SAFETY: Node-API declares the function so.
			unsafe { std::mem::transmute(symbol($name)) }
		};
	}
	Api {
		create_function: find!(c"napi_create_function"),
		set_named_property: find!(c"napi_set_named_property"),
		get_named_property: find!(c"napi_get_named_property"),
		get_cb_info: find!(c"napi_get_cb_info"),
		get_value_int64: find!(c"napi_get_value_int64"),
		get_value_bool: find!(c"napi_get_value_bool"),
		get_value_bigint_int64: find!(c"napi_get_value_bigint_int64"),
		get_typedarray_info: find!(c"napi_get_typedarray_info"),
		create_int32: find!(c"napi_create_int32"),
		create_uint32: find!(c"napi_create_uint32"),
		create_double: find!(c"napi_create_double"),
		create_bigint_int64: find!(c"napi_create_bigint_int64"),
		create_object: find!(c"napi_create_object"),
		get_value_double: find!(c"napi_get_value_double"),
		get_boolean: find!(c"napi_get_boolean"),
		create_string_utf8: find!(c"napi_create_string_utf8"),
		get_undefined: find!(c"napi_get_undefined"),
		create_external_arraybuffer: find!(c"napi_create_external_arraybuffer"),
		create_reference: find!(c"napi_create_reference"),
		get_reference_value: find!(c"napi_get_reference_value"),
		call_function: find!(c"napi_call_function"),
		open_handle_scope: find!(c"napi_open_handle_scope"),
		close_handle_scope: find!(c"napi_close_handle_scope"),
		throw_range_error: find!(c"napi_throw_range_error"),
		get_and_clear_last_exception: find!(c"napi_get_and_clear_last_exception"),
		coerce_to_string: find!(c"napi_coerce_to_string"),
		get_value_string_utf8: find!(c"napi_get_value_string_utf8"),
		throw: find!(c"napi_throw"),
		throw_error: find!(c"napi_throw_error"),
		adjust_external_memory: find!(c"napi_adjust_external_memory"),
		create_threadsafe_function: find!(c"napi_create_threadsafe_function"),
		call_threadsafe_function: find!(c"napi_call_threadsafe_function"),
		ref_threadsafe_function: find!(c"napi_ref_threadsafe_function"),
		unref_threadsafe_function: find!(c"napi_unref_threadsafe_function"),
	}
}

/// Abort on a Node-API call that failed: only a value of the wrong type, which the
/// generated TypeScript never passes, or a host gone, fails one.
#[inline]
fn ok(status: Status, what: &str) {
	if status != 0 {
		failed(status, what);
	}
}

#[cold]
#[inline(never)]
fn failed(status: Status, what: &str) -> ! {
	rt::fail("node", &format!("{what}: Node-API failed with status {status}"))
}

// The scratch memory: one reservation as large as wasm32's memory, which the TypeScript's
// stack takes from its start up and the calls into JavaScript take from its end down, each
// committing what it reaches. One `ArrayBuffer` views it all, so no view ever detaches.

// Node's TypeScript reads the 64-bit little-endian layouts of the targets it serves.
#[cfg(not(all(target_pointer_width = "64", target_endian = "little")))]
compile_error!("the node feature builds only for a 64-bit little-endian target, whose layouts Node's binding reads");

/// How large the scratch memory is.
const SCRATCH: usize = 1 << 32;
/// How much is committed at a time.
const CHUNK: usize = 1 << 20;

/// What only JavaScript's thread touches, which every call from JavaScript and every call
/// into it runs on.
struct Js<T>(UnsafeCell<T>);

// SAFETY: only JavaScript's thread reaches its value, as each use says.
unsafe impl<T> Sync for Js<T> {}

impl<T> Js<T> {
	/// # Safety
	/// Called on JavaScript's thread, and no other borrow of the value is live.
	#[allow(clippy::mut_from_ref)]
	unsafe fn get(&self) -> &mut T {
		// SAFETY: the caller's contract.
		unsafe { &mut *self.0.get() }
	}
}

/// The scratch memory's address, set once as the addon registers.
static BASE: AtomicUsize = AtomicUsize::new(0);

/// The ends of the scratch memory each side has committed, and how far down the calls into
/// JavaScript have taken it.
struct Scratch {
	/// How far up the stack is committed.
	low: usize,
	/// Where the calls into JavaScript have committed down to.
	high: usize,
	/// Where the calls into JavaScript next take from, down.
	top: usize,
}

static SCRATCH_STATE: Js<Scratch> = Js(UnsafeCell::new(Scratch {
	low: 0,
	high: SCRATCH,
	top: SCRATCH,
}));

#[inline]
fn base() -> usize {
	BASE.load(Ordering::Relaxed)
}

/// Reserve the scratch memory, committing none of it.
fn reserve() -> usize {
	#[cfg(unix)]
	{
		unsafe extern "C" {
			fn mmap(at: *mut c_void, len: usize, prot: i32, flags: i32, fd: i32, offset: i64) -> *mut c_void;
		}
		// `MAP_PRIVATE | MAP_ANONYMOUS | MAP_NORESERVE`, numbered per kernel and, on Linux, per
		// architecture; the BSDs and macOS reserve lazily without it.
		#[cfg(all(
			any(target_os = "linux", target_os = "android"),
			not(any(target_arch = "mips64", target_arch = "mips64r6", target_arch = "powerpc64"))
		))]
		const FLAGS: i32 = 0x02 | 0x20 | 0x4000;
		#[cfg(all(target_os = "linux", target_arch = "powerpc64"))]
		const FLAGS: i32 = 0x02 | 0x20 | 0x40;
		#[cfg(all(any(target_os = "linux", target_os = "android"), any(target_arch = "mips64", target_arch = "mips64r6")))]
		const FLAGS: i32 = 0x02 | 0x800 | 0x400;
		#[cfg(any(target_os = "illumos", target_os = "solaris"))]
		const FLAGS: i32 = 0x02 | 0x100 | 0x40;
		#[cfg(any(target_vendor = "apple", target_os = "freebsd", target_os = "netbsd", target_os = "openbsd", target_os = "dragonfly"))]
		const FLAGS: i32 = 0x0002 | 0x1000;
		#[cfg(not(any(
			target_os = "linux",
			target_os = "android",
			target_os = "illumos",
			target_os = "solaris",
			target_vendor = "apple",
			target_os = "freebsd",
			target_os = "netbsd",
			target_os = "openbsd",
			target_os = "dragonfly",
		)))]
		compile_error!("the node feature knows no anonymous mmap flags for this target's kernel");
		// SAFETY: a new anonymous mapping, inaccessible until committed.
		let at = unsafe { mmap(null_mut(), SCRATCH, 0, FLAGS, -1, 0) };
		if at as isize == -1 {
			rt::fail("node", "the addon could not reserve its scratch memory");
		}
		at as usize
	}
	#[cfg(windows)]
	{
		// SAFETY: a new reservation, inaccessible until committed.
		let at = unsafe { VirtualAlloc(null_mut(), SCRATCH, 0x2000, 0x01) };
		if at.is_null() {
			rt::fail("node", "the addon could not reserve its scratch memory");
		}
		at as usize
	}
}

/// Commit the scratch memory from `from` up to `to`, offsets of it.
fn commit(from: usize, to: usize) {
	if from >= to {
		return;
	}
	let at = (base() + from) as *mut c_void;
	#[cfg(unix)]
	{
		unsafe extern "C" {
			fn mprotect(at: *mut c_void, len: usize, prot: i32) -> i32;
		}
		// SAFETY: a page-aligned range of the reservation.
		if unsafe { mprotect(at, to - from, 0x1 | 0x2) } != 0 {
			rt::fail("node", "the addon could not commit its scratch memory");
		}
	}
	#[cfg(windows)]
	// SAFETY: a page-aligned range of the reservation.
	if unsafe { VirtualAlloc(at, to - from, 0x1000, 0x04) }.is_null() {
		rt::fail("node", "the addon could not commit its scratch memory");
	}
}

/// `$commit(end)`: commit the stack up to `end`, returning how far it is committed. Past
/// what the calls into JavaScript took, it throws a `RangeError`.
unsafe extern "C" fn native_commit(env: Env, info: Info) -> Value {
	// SAFETY: a call from JavaScript.
	unsafe {
		let cx = Cx::<1>::new(env, info);
		let end = cx.int(0) as usize;
		let scratch = SCRATCH_STATE.get();
		if end > scratch.high {
			let message = c"a call's values outgrow the 4 GiB of scratch memory the addon lends";
			(api().throw_range_error)(env, null(), message.as_ptr());
			return null_mut();
		}
		let to = end.div_ceil(CHUNK).saturating_mul(CHUNK).min(scratch.high);
		commit(scratch.low, to);
		scratch.low = to;
		cx.number(to as f64)
	}
}

/// A place of `size` bytes aligned to `align` the calls into JavaScript take, below what
/// they took: its offset.
///
/// # Safety
/// On JavaScript's thread.
unsafe fn take(size: usize, align: usize) -> usize {
	// SAFETY: the caller's contract.
	let scratch = unsafe { SCRATCH_STATE.get() };
	let at = (scratch.top.saturating_sub(size)) & !(align - 1);
	if at < scratch.low + CHUNK {
		rt::fail("node", "a call into JavaScript's values outgrow the scratch memory the addon lends");
	}
	if at < scratch.high {
		let from = (at / CHUNK) * CHUNK;
		commit(from, scratch.high);
		scratch.high = from;
	}
	scratch.top = at;
	at
}

// Handles, by their id, which the TypeScript holds: 0 is null. An id is a slot's index in
// the table and, above it, the slot's generation, which each free bumps, so an id read before
// its handle closed never names the object that takes its slot next, whatever JavaScript ran
// between the read and the call. Each slot counts the calls holding it, so one JavaScript
// closes meanwhile is freed by its last.

/// A handle's slot: its address, its generation, how many calls hold it, and, once
/// JavaScript closed it while a call held it, the free its last hold makes.
struct Slot {
	p: *const c_void,
	generation: u32,
	holds: u32,
	/// [`OPEN`], or once closed while held, the free [`close`] takes.
	closing: u32,
}

/// A slot no one closed.
const OPEN: u32 = u32::MAX;

/// How many generations a slot has, so an id stays a safe integer, 2^53 at most: a slot freed
/// in its last is never reused, so no id names two objects.
const GENERATIONS: u32 = 1 << 21;

struct Table {
	slots: Vec<Slot>,
	free: Vec<u32>,
}

static TABLE: Js<Table> = Js(UnsafeCell::new(Table {
	slots: Vec::new(),
	free: Vec::new(),
}));

/// Each object type's free, which [`close`] takes by its index plus one, 0 being a lent
/// handle's, which forgets its id alone.
static FREES: OnceLock<&'static [unsafe fn(*const c_void)]> = OnceLock::new();

/// The id of a new slot holding `p`, or 0 for null.
///
/// # Safety
/// On JavaScript's thread.
#[inline]
unsafe fn index(p: *const c_void) -> i64 {
	if p.is_null() {
		return 0;
	}
	// SAFETY: the caller's contract.
	let table = unsafe { TABLE.get() };
	let i = match table.free.pop() {
		Some(i) => i,
		None => {
			if table.slots.is_empty() {
				table.slots.push(Slot { p: null(), generation: 0, holds: 0, closing: OPEN });
			}
			table.slots.push(Slot { p: null(), generation: 0, holds: 0, closing: OPEN });
			(table.slots.len() - 1) as u32
		}
	};
	let slot = &mut table.slots[i as usize];
	slot.p = p;
	slot.closing = OPEN;
	i64::from(slot.generation) << 32 | i64::from(i)
}

/// The index of the slot `id` names, if it holds that id's handle still: not for a closed
/// handle's, -1, nor one freed since, nor for any number the addon never gave.
///
/// # Safety
/// On JavaScript's thread.
#[inline]
unsafe fn find(id: i64) -> Option<u32> {
	// A negative id's generation is past every slot's.
	let (i, generation) = (id as u32, (id >> 32) as u32);
	// SAFETY: the caller's contract.
	let table = unsafe { TABLE.get() };
	match table.slots.get(i as usize) {
		Some(slot) if slot.generation == generation && !slot.p.is_null() => Some(i),
		_ => None,
	}
}

/// The slot `id` names, which holds its handle, or else the addon aborts: one that a call
/// holds or no one closed.
///
/// # Safety
/// On JavaScript's thread.
#[inline]
unsafe fn slot(id: i64) -> (u32, &'static mut Slot) {
	// SAFETY: the caller's contract.
	match unsafe { find(id) } {
		// SAFETY: as above, and `find` found it.
		Some(i) => (i, unsafe { &mut TABLE.get().slots[i as usize] }),
		None => rt::fail("node", "a handle's id is not one the table holds"),
	}
}

/// The address of `id`.
///
/// # Safety
/// On JavaScript's thread.
#[inline]
unsafe fn address(id: i64) -> *const c_void {
	if id == 0 {
		return null();
	}
	// SAFETY: the caller's contract.
	unsafe { slot(id) }.1.p
}

/// The index and address of `id`, held until [`release`], or `None` for a closed or freed
/// handle's.
///
/// # Safety
/// On JavaScript's thread.
#[inline]
unsafe fn hold(id: i64) -> Option<(u32, *const c_void)> {
	if id == 0 {
		return Some((0, null()));
	}
	// SAFETY: the caller's contract.
	let i = unsafe { find(id) }?;
	// SAFETY: as above.
	let slot = unsafe { &mut TABLE.get().slots[i as usize] };
	slot.holds += 1;
	Some((i, slot.p))
}

/// Release a hold of the slot of index `i`, freeing it if it was its last of a closed handle.
///
/// # Safety
/// On JavaScript's thread, with no exception pending, as the free may call into it, and `i`
/// held.
#[inline]
unsafe fn release(i: u32) {
	if i == 0 {
		return;
	}
	// SAFETY: the caller's contract.
	let slot = unsafe { &mut TABLE.get().slots[i as usize] };
	slot.holds -= 1;
	if slot.holds == 0 && slot.closing != OPEN {
		let free = slot.closing;
		// SAFETY: as above.
		unsafe { close(i, free) };
	}
}

/// Forget the slot of index `i` and free its handle with `free`, 1 plus its index in
/// [`FREES`], or forget it alone for 0.
///
/// # Safety
/// On JavaScript's thread, with no call holding `i`, which holds a handle.
unsafe fn close(i: u32, free: u32) {
	// SAFETY: the caller's contract.
	let p = unsafe { forget(i) };
	if free != 0 {
		let frees = FREES.get().copied().unwrap_or_default();
		let Some(&f) = frees.get(free as usize - 1) else {
			rt::fail("node", "a handle's free is not the addon's");
		};
		// SAFETY: a handle of the type the free is of, which no call holds.
		unsafe { serve(|| f(p)) };
	}
}

/// Free the slot of index `i`, which holds a handle, returning its address: its next
/// generation is the slot's, unless this was its last.
///
/// # Safety
/// On JavaScript's thread.
#[inline]
unsafe fn forget(i: u32) -> *const c_void {
	// SAFETY: the caller's contract.
	let table = unsafe { TABLE.get() };
	let slot = &mut table.slots[i as usize];
	let p = std::mem::replace(&mut slot.p, null());
	slot.generation += 1;
	if slot.generation < GENERATIONS {
		table.free.push(i);
	}
	p
}

/// A handle a call holds, released once it drops.
pub struct Hold {
	i: u32,
	p: *const c_void,
}

impl Hold {
	/// The handle's address, read as it was held, so a worker the call runs on reads no table.
	#[inline]
	pub fn p(&self) -> *const c_void {
		self.p
	}
}

impl Drop for Hold {
	#[inline]
	fn drop(&mut self) {
		// SAFETY: on JavaScript's thread, which a stub drops its holds on before it throws.
		unsafe { release(self.i) }
	}
}

/// What a native function returns once it threw.
pub const THROWN: Value = null_mut();

/// The largest integer a number holds exactly, 2^53 - 1.
const MAX_SAFE: f64 = 9_007_199_254_740_991.0;

/// A call from JavaScript: its environment and its `N` arguments.
pub struct Cx<const N: usize> {
	env: Env,
	argv: [Value; N],
}

impl<const N: usize> Cx<N> {
	/// # Safety
	/// A call from JavaScript, of `env` and `info`.
	#[inline]
	pub unsafe fn new(env: Env, info: Info) -> Self {
		let mut argc = N;
		let mut argv = [null_mut(); N];
		// SAFETY: the caller's contract; argv holds `argc`.
		ok(unsafe { (api().get_cb_info)(env, info, &mut argc, argv.as_mut_ptr(), null_mut(), null_mut()) }, "a call's arguments");
		Self { env, argv }
	}

	/// Argument `n`, a number, as an integer.
	#[inline]
	pub fn int(&self, n: usize) -> i64 {
		let mut v = 0;
		// SAFETY: a value of this call.
		ok(unsafe { (api().get_value_int64)(self.env, self.argv[n], &mut v) }, "a number");
		v
	}

	/// Argument `n`, a `bigint`, as its 64 bits.
	#[inline]
	pub fn big(&self, n: usize) -> i64 {
		let mut v = 0;
		let mut lossless = false;
		// SAFETY: a value of this call.
		ok(unsafe { (api().get_value_bigint_int64)(self.env, self.argv[n], &mut v, &mut lossless) }, "a bigint");
		v
	}

	/// Release a refused call's holds without exposing its pending exception to cleanup.
	pub fn refused<T>(&self, held: T) -> Value {
		// SAFETY: on JavaScript's thread; this path has a pending conversion exception.
		unsafe {
			let mut exception = null_mut();
			ok((api().get_and_clear_last_exception)(self.env, &mut exception), "take a conversion exception");
			drop(held);
			ok((api().throw)(self.env, exception), "restore a conversion exception");
		}
		THROWN
	}

	/// Argument `n`, an offset in the scratch memory, as its address, or null for 0.
	#[inline]
	pub fn at(&self, n: usize) -> *mut u8 {
		match self.int(n) as usize {
			0 => null_mut(),
			at => (base() + at) as *mut u8,
		}
	}

	/// Argument `n`, a handle's id, held until the [`Hold`] drops; a closed or freed one
	/// throws, naming its type, `what`.
	///
	/// # Safety
	/// On JavaScript's thread.
	#[inline]
	pub unsafe fn hold(&self, n: usize, what: &str) -> Result<Hold, ()> {
		// SAFETY: the caller's contract.
		match unsafe { hold(self.id(n)) } {
			Some((i, p)) => Ok(Hold { i, p }),
			None => {
				self.throw(&format!("{what} is closed"));
				Err(())
			}
		}
	}

	/// Argument `n`, a handle's id, or -1, no slot's, for a number no id is: read as a
	/// double, as an id past a small integer's range is one, which reading it as an integer
	/// would take a slower way to.
	#[inline]
	fn id(&self, n: usize) -> i64 {
		let mut v = 0.0;
		// SAFETY: a value of this call.
		ok(unsafe { (api().get_value_double)(self.env, self.argv[n], &mut v) }, "a handle's id");
		match v as i64 {
			id if id as f64 == v => id,
			_ => -1,
		}
	}

	/// Argument `n`, an `rt.u64`: a safe integer, or else what `rt.ts`'s `u64Big` makes of
	/// it, which throws for what no `u64` is.
	#[inline]
	pub fn u64(&self, n: usize) -> Result<u64, ()> {
		match self.safe(n) {
			Some(v) if v >= 0.0 => Ok(v as u64),
			_ => self.widened(n, WIDE).map(|v| v as u64),
		}
	}

	/// Argument `n`, an `rt.i64`, as [`Cx::u64`] takes a `u64`.
	#[inline]
	pub fn i64(&self, n: usize) -> Result<i64, ()> {
		match self.safe(n) {
			Some(v) => Ok(v as i64),
			None => self.widened(n, WIDE + 1),
		}
	}

	/// Argument `n`, if a safe integer.
	#[inline]
	fn safe(&self, n: usize) -> Option<f64> {
		let mut v = 0.0;
		// SAFETY: a value of this call.
		let status = unsafe { (api().get_value_double)(self.env, self.argv[n], &mut v) };
		(status == 0 && v.abs() <= MAX_SAFE && v.trunc() == v).then_some(v)
	}

	/// Argument `n` through the runtime's import `import`, which makes a `bigint` of it or
	/// throws, leaving its exception pending.
	#[cold]
	#[inline(never)]
	fn widened(&self, n: usize, import: usize) -> Result<i64, ()> {
		// SAFETY: on JavaScript's thread, with the imports `$init` took.
		unsafe {
			let Some(&f) = IMPORTS.get().get(import) else {
				rt::fail("node", "a call before the binding's load gave its imports");
			};
			let mut function = null_mut();
			ok((api().get_reference_value)(self.env, f, &mut function), "an import");
			let mut v = null_mut();
			match (api().call_function)(self.env, self.undefined(), function, 1, &self.argv[n], &mut v) {
				0 => Ok(Cx { env: self.env, argv: [v] }.big(0)),
				_ => Err(()),
			}
		}
	}

	/// `v` as an `rt.u64`: a number while a safe integer, and its limbs past that.
	#[inline]
	pub fn u64v(&self, v: u64) -> Value {
		match v as f64 <= MAX_SAFE {
			true => self.number(v as f64),
			false => self.limbs(self.uint32((v >> 32) as u32), v as u32),
		}
	}

	/// `v` as an `rt.i64`, as [`Cx::u64v`] makes a `u64`'s.
	#[inline]
	pub fn i64v(&self, v: i64) -> Value {
		match (v as f64).abs() <= MAX_SAFE {
			true => self.number(v as f64),
			false => self.limbs(self.int32((v >> 32) as i32), v as u32),
		}
	}

	/// The limbs `{ hi, lo }` of a 64-bit integer past 2^53, as `rt.ts` makes them.
	#[cold]
	fn limbs(&self, hi: Value, lo: u32) -> Value {
		let mut o = null_mut();
		// SAFETY: this call's environment, and values of it.
		unsafe {
			ok((api().create_object)(self.env, &mut o), "an object");
			ok((api().set_named_property)(self.env, o, c"hi".as_ptr(), hi), "a limb");
			ok((api().set_named_property)(self.env, o, c"lo".as_ptr(), self.uint32(lo)), "a limb");
		}
		o
	}

	/// The boolean `b`.
	#[inline]
	pub fn boolean(&self, b: bool) -> Value {
		let mut v = null_mut();
		// SAFETY: this call's environment.
		ok(unsafe { (api().get_boolean)(self.env, b, &mut v) }, "a boolean");
		v
	}

	/// Throw the error of the C function `what`'s failed check, its status `status`.
	#[cold]
	#[inline(never)]
	pub fn failed(&self, status: i32, what: &str) -> Value {
		let why = (status as usize).checked_sub(1).and_then(|i| rt::Status::ALL.get(i)).map(|s| s.message());
		self.throw(&format!("{what}: {}", why.unwrap_or("an unknown status")))
	}

	/// Throw an `Error` of `message`.
	#[cold]
	#[inline(never)]
	pub fn throw(&self, message: &str) -> Value {
		let text = std::ffi::CString::new(message).unwrap_or_default();
		// SAFETY: this call's environment, and a C string.
		ok(unsafe { (api().throw_error)(self.env, null(), text.as_ptr()) }, "a throw");
		THROWN
	}

	/// Argument `n`, a future's id, as its address, freeing its slot: a free's.
	///
	/// # Safety
	/// On JavaScript's thread.
	#[inline]
	pub unsafe fn freed(&self, n: usize) -> *const c_void {
		let id = self.id(n);
		if id == 0 {
			return null();
		}
		// SAFETY: the caller's contract.
		unsafe { forget(slot(id).0) }
	}

	#[inline]
	pub fn undefined(&self) -> Value {
		let mut v = null_mut();
		// SAFETY: this call's environment.
		ok(unsafe { (api().get_undefined)(self.env, &mut v) }, "undefined");
		v
	}

	#[inline]
	pub fn int32(&self, n: i32) -> Value {
		let mut v = null_mut();
		// SAFETY: as above.
		ok(unsafe { (api().create_int32)(self.env, n, &mut v) }, "a number");
		v
	}

	#[inline]
	pub fn uint32(&self, n: u32) -> Value {
		let mut v = null_mut();
		// SAFETY: as above.
		ok(unsafe { (api().create_uint32)(self.env, n, &mut v) }, "a number");
		v
	}

	#[inline]
	pub fn number(&self, n: f64) -> Value {
		let mut v = null_mut();
		// SAFETY: as above.
		ok(unsafe { (api().create_double)(self.env, n, &mut v) }, "a number");
		v
	}

	#[inline]
	pub fn bigint(&self, n: i64) -> Value {
		let mut v = null_mut();
		// SAFETY: as above.
		ok(unsafe { (api().create_bigint_int64)(self.env, n, &mut v) }, "a bigint");
		v
	}

	/// The id of a new slot of the handle `p`.
	///
	/// # Safety
	/// On JavaScript's thread.
	#[inline]
	pub unsafe fn indexed(&self, p: *const c_void) -> Value {
		// SAFETY: the caller's contract.
		self.number(unsafe { index(p) } as f64)
	}

	/// Argument `n`, a typed array: its bytes.
	fn typed(&self, n: usize) -> (*mut u8, usize) {
		let (mut kind, mut len, mut data, mut offset) = (0, 0, null_mut(), 0);
		let mut buffer = null_mut();
		// SAFETY: a value of this call.
		ok(
			unsafe { (api().get_typedarray_info)(self.env, self.argv[n], &mut kind, &mut len, &mut data, &mut buffer, &mut offset) },
			"a typed array",
		);
		let size = match kind {
			0..=2 => 1,
			3 | 4 => 2,
			5..=7 => 4,
			8..=10 => 8,
			// Float16Array.
			11 => 2,
			_ => rt::fail("node", "a typed array of an unknown kind"),
		};
		(data.cast(), len * size)
	}
}

/// The 64 bits of `v`, a value of at most 8 bytes, zero-extended: a function's address, or
/// a struct of one scalar.
#[inline]
pub fn bits<T: Copy>(v: T) -> u64 {
	const { assert!(size_of::<T>() <= 8) };
	let mut out = 0u64;
	// SAFETY: `T` fits, and every byte of it is initialized, as a C ABI value's.
	unsafe { std::ptr::copy_nonoverlapping((&raw const v).cast::<u8>(), (&raw mut out).cast::<u8>(), size_of::<T>()) };
	out
}

/// The value of type `T` of the low bytes of `bits`, as [`bits`] made them.
///
/// # Safety
/// The bytes are a valid `T`.
#[inline]
pub unsafe fn from_bits<T: Copy>(bits: u64) -> T {
	const { assert!(size_of::<T>() <= 8) };
	let mut out = MaybeUninit::<T>::uninit();
	// SAFETY: `T` fits.
	unsafe {
		std::ptr::copy_nonoverlapping((&raw const bits).cast::<u8>(), out.as_mut_ptr().cast::<u8>(), size_of::<T>());
		out.assume_init()
	}
}

// The calls into JavaScript, each run on its thread.

/// What the addon calls of JavaScript, by index: the runtime's, then the glue's.
static IMPORTS: Js<Vec<Ref>> = Js(UnsafeCell::new(Vec::new()));

/// The runtime's imports, before the glue's `EXTERNS`.
pub const RUNTIME_IMPORTS: [&CStr; 9] = [
	c"rust2_wake",
	c"rust2_waker_drop",
	c"rust2_free",
	c"rust2_future_poll",
	c"rust2_reader_read",
	c"rust2_reader_seek",
	c"rust2_idle",
	c"rust2_u64",
	c"rust2_i64",
];

/// The index of `rust2_u64`, before `rust2_i64`'s: `rt.ts`'s `u64Big` and `i64Big`, which
/// make a `bigint` of a 64-bit integer that is no safe one, or throw for what is no integer.
const WIDE: usize = 7;

/// The index of the glue's first import.
pub const FIRST: usize = RUNTIME_IMPORTS.len();

/// JavaScript's environment, which only its thread uses.
static ENV: AtomicPtr<c_void> = AtomicPtr::new(null_mut());

thread_local!(static JS: Cell<bool> = const { Cell::new(false) });

/// Whether this is JavaScript's thread.
#[inline]
fn on_js() -> bool {
	JS.get()
}

/// A call into JavaScript, on its thread: the values it makes, in a handle scope of their
/// own, and the places it takes of the scratch memory, both given back once it returns.
pub struct Out {
	env: Env,
	scope: *mut c_void,
	mark: usize,
}

impl Out {
	/// # Safety
	/// On JavaScript's thread.
	unsafe fn new() -> Self {
		let env = ENV.load(Ordering::Relaxed);
		let mut scope = null_mut();
		// SAFETY: the caller's contract.
		ok(unsafe { (api().open_handle_scope)(env, &mut scope) }, "a handle scope");
		// SAFETY: as above.
		let mark = unsafe { SCRATCH_STATE.get() }.top;
		Self { env, scope, mark }
	}

	fn cx(&self) -> Cx<0> {
		Cx { env: self.env, argv: [] }
	}

	pub fn int32(&self, n: i32) -> Value {
		self.cx().int32(n)
	}

	pub fn uint32(&self, n: u32) -> Value {
		self.cx().uint32(n)
	}

	pub fn number(&self, n: f64) -> Value {
		self.cx().number(n)
	}

	pub fn bigint(&self, n: i64) -> Value {
		self.cx().bigint(n)
	}

	pub fn boolean(&self, b: bool) -> Value {
		self.cx().boolean(b)
	}

	/// `v` as an `rt.u64`.
	pub fn u64v(&self, v: u64) -> Value {
		self.cx().u64v(v)
	}

	/// `v` as an `rt.i64`.
	pub fn i64v(&self, v: i64) -> Value {
		self.cx().i64v(v)
	}

	/// The token `data` is.
	pub fn token(&self, data: *mut c_void) -> Value {
		self.number(data as usize as f64)
	}

	/// The id of a new slot of the handle `p`.
	pub fn handle(&self, p: *const c_void) -> Value {
		// SAFETY: on JavaScript's thread, as an `Out` is.
		self.number(unsafe { index(p) } as f64)
	}

	/// The offset of a place for a `T`, holding a copy of what `src` points to where `copy`,
	/// or 0 for a null `src`.
	///
	/// # Safety
	/// `src` is null or points to a `T`, initialized where `copy`.
	pub unsafe fn place<T>(&mut self, src: *const T, copy: bool) -> usize {
		if src.is_null() {
			return 0;
		}
		// SAFETY: on JavaScript's thread.
		let at = unsafe { take(size_of::<T>().max(1), align_of::<T>()) };
		if copy {
			// SAFETY: the caller's contract, and the place is the scratch memory's.
			unsafe { std::ptr::copy_nonoverlapping(src.cast::<u8>(), (base() + at) as *mut u8, size_of::<T>()) };
		}
		at
	}

	/// The offset of a place for a `T` JavaScript writes.
	pub fn slot<T>(&mut self) -> usize {
		// SAFETY: on JavaScript's thread, as an `Out` is.
		unsafe { take(size_of::<T>().max(1), align_of::<T>()) }
	}

	/// The offset of a place holding a copy of `v`.
	pub fn value<T>(&mut self, v: &T) -> usize {
		// SAFETY: a reference.
		unsafe { self.place(v, true) }
	}

	/// The offset `at`, as JavaScript takes it.
	pub fn offset(&self, at: usize) -> Value {
		self.number(at as f64)
	}

	/// Copy the `T` at `at` back to `dst`, unless null.
	///
	/// # Safety
	/// `dst` is null or points to a `T`, and `at` is a place for one JavaScript wrote.
	pub unsafe fn back<T>(&self, dst: *mut T, at: usize) {
		if !dst.is_null() {
			// SAFETY: the caller's contract.
			unsafe { std::ptr::copy_nonoverlapping((base() + at) as *const u8, dst.cast::<u8>(), size_of::<T>()) };
		}
	}

	/// The `T` JavaScript wrote at `at`.
	///
	/// # Safety
	/// It wrote a valid `T` there.
	pub unsafe fn read<T>(&self, at: usize) -> T {
		// SAFETY: the caller's contract.
		unsafe { ((base() + at) as *const T).read() }
	}

	/// Call import `i` with `args`, aborting on what it throws, as Rust has nowhere to
	/// unwind to: the binding's own `out` aborts on any, so only an abort throws here.
	pub fn invoke(&self, i: usize, args: &[Value]) -> Value {
		match self.attempt(i, args) {
			Ok(result) => result,
			Err(status) => rt::fail("node", &format!("a call into JavaScript failed with status {status}: {}", self.thrown())),
		}
	}

	/// [`Out::invoke`], or the status of a call that threw, or that JavaScript, exiting,
	/// refused.
	fn attempt(&self, i: usize, args: &[Value]) -> Result<Value, Status> {
		// SAFETY: on JavaScript's thread, with the imports `$init` took.
		unsafe {
			let imports = IMPORTS.get();
			let Some(&f) = imports.get(i) else {
				rt::fail("node", "a call into JavaScript before the binding's load gave its imports");
			};
			let mut function = null_mut();
			ok((api().get_reference_value)(self.env, f, &mut function), "an import");
			let this = self.cx().undefined();
			let mut result = null_mut();
			let status = (api().call_function)(self.env, this, function, args.len(), args.as_ptr(), &mut result);
			match status {
				0 => Ok(result),
				status => Err(status),
			}
		}
	}

	/// What the call into JavaScript that failed threw, with its stack where it has one.
	#[cold]
	fn thrown(&self) -> String {
		// SAFETY: on JavaScript's thread, with an exception pending.
		unsafe {
			let mut e = null_mut();
			if (api().get_and_clear_last_exception)(self.env, &mut e) != 0 || e.is_null() {
				return "nothing".to_string();
			}
			let mut stack = null_mut();
			if (api().get_named_property)(self.env, e, c"stack".as_ptr(), &mut stack) == 0 {
				let mut text = null_mut();
				if (api().coerce_to_string)(self.env, stack, &mut text) == 0 {
					e = text;
				}
			}
			let mut text = null_mut();
			if (api().coerce_to_string)(self.env, e, &mut text) != 0 {
				return "what no string shows".to_string();
			}
			let mut len = 0;
			(api().get_value_string_utf8)(self.env, text, null_mut(), 0, &mut len);
			let mut buf = vec![0u8; len + 1];
			(api().get_value_string_utf8)(self.env, text, buf.as_mut_ptr().cast(), len + 1, &mut len);
			String::from_utf8_lossy(&buf[..len]).into_owned()
		}
	}

	/// What JavaScript returned, a number, as an integer.
	pub fn int(&self, v: Value) -> i64 {
		Cx { env: self.env, argv: [v] }.int(0)
	}

	/// What JavaScript returned, a `bigint`, as its 64 bits.
	pub fn big(&self, v: Value) -> i64 {
		Cx { env: self.env, argv: [v] }.big(0)
	}

	/// What JavaScript returned, an `rt.u64`, aborting on what is none, as on what it throws.
	pub fn u64(&self, v: Value) -> u64 {
		match (Cx { env: self.env, argv: [v] }).u64(0) {
			Ok(v) => v,
			Err(()) => rt::fail("node", &format!("a call into JavaScript returned no u64: {}", self.thrown())),
		}
	}

	/// What JavaScript returned, an `rt.i64`, as [`Out::u64`] reads a `u64`.
	pub fn i64(&self, v: Value) -> i64 {
		match (Cx { env: self.env, argv: [v] }).i64(0) {
			Ok(v) => v,
			Err(()) => rt::fail("node", &format!("a call into JavaScript returned no i64: {}", self.thrown())),
		}
	}

	/// What JavaScript returned, a handle's id, as its address: the addon aborts for a closed
	/// one, as Rust takes no error from it.
	pub fn address(&self, v: Value) -> *const c_void {
		// SAFETY: on JavaScript's thread.
		unsafe { address(Cx { env: self.env, argv: [v] }.id(0)) }
	}
}

impl Drop for Out {
	fn drop(&mut self) {
		// SAFETY: on JavaScript's thread, giving back what this took, which nothing holds.
		unsafe {
			SCRATCH_STATE.get().top = self.mark;
			ok((api().close_handle_scope)(self.env, self.scope), "a handle scope");
		}
	}
}

// JavaScript's thread, and the calls into it other threads wait on.

/// What JavaScript's thread runs for another.
enum Task {
	/// A call into JavaScript another thread waits on.
	Call(Arc<Job>),
	/// A call into JavaScript, of a token, that nothing waits on: a wake, a waker's drop,
	/// or the free of what JavaScript implements.
	Token(usize, usize),
	/// Drop what other threads freed of the values JavaScript's thread made.
	Drain,
}

struct Home {
	tasks: Mutex<VecDeque<Task>>,
	ready: Condvar,
	/// Whether the event loop was told of a task it has yet to run.
	told: AtomicBool,
	/// Whether JavaScript refused to run what is queued, as its environment is torn down:
	/// nothing queued then or later runs.
	exited: AtomicBool,
	/// The thread-safe function, null once Node frees it as the environment is torn down.
	tsfn: AtomicPtr<c_void>,
	/// Held across a call of `tsfn` and while it is nulled, so no call reaches it freed.
	telling: Mutex<()>,
	/// How many tasks were ever queued, and taken, each changed under the queue's lock, which
	/// JavaScript's thread reads as it spins.
	pushed: AtomicUsize,
	popped: AtomicUsize,
	/// Whether JavaScript's thread waits on `ready`, set under the queue's lock: else it
	/// spins, and nothing need wake it.
	parked: AtomicBool,
}

static HOME: Home = Home {
	tasks: Mutex::new(VecDeque::new()),
	ready: Condvar::new(),
	told: AtomicBool::new(false),
	exited: AtomicBool::new(false),
	tsfn: AtomicPtr::new(null_mut()),
	telling: Mutex::new(()),
	pushed: AtomicUsize::new(0),
	popped: AtomicUsize::new(0),
	parked: AtomicBool::new(false),
};

/// How long a worker spins for its next call, and JavaScript's thread for its call's end
/// or a task, before parking: a handoff between spinning threads costs a tenth of one
/// through a park, so a run of calls keeps both awake, while an idle one parks within this.
const SPIN: Duration = Duration::from_micros(20);

/// Spin while `f` returns none, until [`SPIN`] passed since it began.
#[inline]
fn spin<T>(mut f: impl FnMut() -> Option<T>) -> Option<T> {
	static PARALLEL: OnceLock<bool> = OnceLock::new();
	if !*PARALLEL.get_or_init(|| std::thread::available_parallelism().is_ok_and(|n| n.get() > 1)) {
		return f();
	}
	let start = Instant::now();
	let mut spins = 0u32;
	loop {
		if let Some(v) = f() {
			return Some(v);
		}
		spins = spins.wrapping_add(1);
		// The clock costs a few dozen spins, so it is read once in 64.
		if spins % 64 == 0 && start.elapsed() >= SPIN {
			return None;
		}
		std::hint::spin_loop();
	}
}

impl Home {
	fn lock(&self) -> MutexGuard<'_, VecDeque<Task>> {
		self.tasks.lock().unwrap_or_else(|e| e.into_inner())
	}

	/// Queue `task` for a wait in a call from JavaScript to run, and tell the event loop to
	/// run it if none does first, returning whether JavaScript is alive to.
	fn push(&self, task: Task) -> bool {
		let parked = {
			let mut tasks = self.lock();
			tasks.push_back(task);
			self.pushed.fetch_add(1, Ordering::SeqCst);
			self.parked.load(Ordering::Relaxed)
		};
		if parked {
			self.ready.notify_all();
		}
		// After the task is queued, so either this sees an exit or the exit's abandon sees
		// the task.
		if self.exited.load(Ordering::SeqCst) {
			self.abandon();
			return false;
		}
		if self.told.swap(true, Ordering::SeqCst) {
			return true;
		}
		let told = {
			let _telling = self.telling.lock().unwrap_or_else(|e| e.into_inner());
			let tsfn = self.tsfn.load(Ordering::Acquire);
			// SAFETY: `on_freed` nulls it, under the same lock, before Node frees it.
			!tsfn.is_null() && unsafe { (api().call_threadsafe_function)(tsfn, null_mut(), 0) } == 0
		};
		if !told {
			self.exit();
		}
		told
	}

	/// JavaScript refused what is queued: abandon it, and all that is queued later.
	fn exit(&self) {
		self.exited.store(true, Ordering::SeqCst);
		self.abandon();
	}

	/// The task first in the queue, if any.
	fn pop(&self) -> Option<Task> {
		let mut tasks = self.lock();
		let task = tasks.pop_front();
		if task.is_some() {
			self.popped.fetch_add(1, Ordering::Release);
		}
		task
	}

	/// Drop each queued task unrun, a call's waiter told it was abandoned.
	fn abandon(&self) {
		let tasks = {
			let mut tasks = self.lock();
			self.popped.fetch_add(tasks.len(), Ordering::Release);
			std::mem::take(&mut *tasks)
		};
		for task in tasks {
			if let Task::Call(job) = task {
				job.finish();
			}
		}
	}

	/// Run `task` on JavaScript's thread.
	///
	/// # Safety
	/// On JavaScript's thread.
	unsafe fn run(&self, task: Task) {
		match task {
			Task::Call(job) => job.run(),
			Task::Token(import, token) => {
				// SAFETY: the caller's contract.
				let out = unsafe { Out::new() };
				let t = out.token(token as *mut c_void);
				out.invoke(import, &[t]);
			}
			Task::Drain => rt::Thread::drain_here(),
		}
	}

	/// Run `f`, which calls into JavaScript, on its thread: at once if called there, or else
	/// as a job it runs while this thread waits.
	fn call<R>(&self, what: &str, f: impl FnOnce(&mut Out) -> R) -> R {
		if on_js() {
			// SAFETY: on JavaScript's thread.
			let mut out = unsafe { Out::new() };
			return f(&mut out);
		}
		let mut result = MaybeUninit::uninit();
		let mut run = ManuallyDrop::new(|| {
			// SAFETY: a job runs on JavaScript's thread.
			let mut out = unsafe { Out::new() };
			result.write(f(&mut out));
		});
		let job = Arc::new(Job {
			// SAFETY: this thread waits below until the job ran, so `run` outlives its use.
			run: Mutex::new(Some(unsafe { Run::new(&mut run) })),
			thread: rt::thread(),
			waiter: std::thread::current(),
			done: Mutex::new(false),
			ran: Condvar::new(),
		});
		if !self.push(Task::Call(job.clone())) {
			rt::fail(what, "the JavaScript it calls into has exited");
		}
		// A home runs meanwhile what the JavaScript it calls asks of the values it holds.
		if !rt::Thread::serve_until(|| *job.done.lock().unwrap_or_else(|e| e.into_inner())) {
			let mut done = job.done.lock().unwrap_or_else(|e| e.into_inner());
			while !*done {
				done = job.ran.wait(done).unwrap_or_else(|e| e.into_inner());
			}
		}
		// A job done with its call still in it was abandoned unrun.
		if job.run.lock().unwrap_or_else(|e| e.into_inner()).is_some() {
			rt::fail(what, "the JavaScript it calls into has exited");
		}
		// SAFETY: the job is done only once it ran.
		unsafe { result.assume_init() }
	}

	/// Lend `run` to a worker, which calls it once, and run each task queued for
	/// JavaScript's thread until it returns. Not generic, so every call shares this code.
	///
	/// # Safety
	/// On JavaScript's thread, and `run` makes only calls through the C ABI.
	#[inline(never)]
	unsafe fn serve_relays(&'static self, run: Run) {
		let done = AtomicUsize::new(0);
		let lent = Lent {
			run,
			done: &raw const done,
			acting: rt::thread(),
		};
		// SAFETY: the caller's contract; this waits below until the worker is done with `lent`.
		unsafe { Worker::lend(&raw const lent) };
		// Once `run` returns, run only what was queued by then, which `done` counts: such as a
		// free its call made, which Rust ran before it returned. What is queued later the
		// event loop was told of, so a steady producer can't hold this thread.
		let queued = || self.pushed.load(Ordering::Acquire) != self.popped.load(Ordering::Acquire);
		loop {
			let woke = spin(|| (done.load(Ordering::Acquire) != 0 || queued()).then_some(()));
			let end = done.load(Ordering::Acquire);
			if end != 0 {
				while self.popped.load(Ordering::Acquire) < end - 1 {
					let Some(task) = self.pop() else { break };
					// SAFETY: the caller's contract.
					unsafe { self.run(task) };
				}
				// SAFETY: on JavaScript's thread, whose nested calls each ended by now.
				unsafe { WORKERS.get().depth -= 1 };
				break;
			}
			if let Some(task) = self.pop() {
				// SAFETY: the caller's contract.
				unsafe { self.run(task) };
				continue;
			}
			if woke.is_none() {
				let mut tasks = self.lock();
				// Against the worker's store of `done` and load of `parked`, so one sees the other's.
				self.parked.store(true, Ordering::SeqCst);
				if done.load(Ordering::SeqCst) == 0 && tasks.is_empty() {
					tasks = self.ready.wait(tasks).unwrap_or_else(|e| e.into_inner());
				}
				self.parked.store(false, Ordering::Relaxed);
				drop(tasks);
			}
		}
	}
}

/// How many tokens JavaScript holds, which `rt.ts` keeps in the scratch memory's first word:
/// what Rust may call into JavaScript through from another thread.
#[inline]
fn relays() -> u32 {
	// SAFETY: the first word of the scratch memory, which only JavaScript's thread writes,
	// read on it.
	unsafe { *(base() as *const u32) }
}

/// Run `f`, a call from JavaScript, at once if JavaScript holds no token, since then nothing
/// it waits on can call into JavaScript. Else run it on a [`Worker`] while JavaScript's
/// thread runs each task queued until `f` returns. The worker acts as this thread does for
/// the handles `f` borrows, as does this one for each job's.
///
/// # Safety
/// A call from JavaScript, and `f` is a call through the C ABI.
#[inline]
pub unsafe fn serve<R>(f: impl FnOnce() -> R) -> R {
	let mut out = MaybeUninit::uninit();
	let mut run = ManuallyDrop::new(|| {
		out.write(f());
	});
	// SAFETY: the caller's contract, and `serve_run` returns once `run` returned.
	unsafe { serve_run(Run::new(&mut run)) };
	// SAFETY: as above.
	unsafe { out.assume_init() }
}

/// [`serve`]'s run of `run`, apart from each call, so each stub holds its call once: inlined
/// into both of its ways to run, each call was there twice, 1.1 MiB of moq's addon.
///
/// # Safety
/// As [`serve`]'s.
#[inline(never)]
unsafe fn serve_run(run: Run) {
	// Only this thread makes a token, so none appears before `run` returns.
	match relays() {
		// SAFETY: the caller's contract.
		0 => unsafe { run.run() },
		// SAFETY: as above.
		_ => unsafe { HOME.serve_relays(run) },
	}
	if rt::Thread::made() {
		drains();
	}
}

/// Give JavaScript's thread a wake that queues a drain, once: a value it made that is not
/// `Send` is then freed on it, though another thread drops it.
#[cold]
fn drains() {
	static WOKEN: AtomicBool = AtomicBool::new(false);
	if !WOKEN.swap(true, Ordering::Relaxed) {
		rt::Thread::wake_here(Arc::new(|| {
			HOME.push(Task::Drain);
		}));
	}
}

/// The workers, one for each depth of the calls JavaScript's thread waits on, which nest:
/// a spawn per call costs about three times a handoff to one.
static WORKERS: Js<Workers> = Js(UnsafeCell::new(Workers {
	all: Vec::new(),
	depth: 0,
}));

struct Workers {
	all: Vec<Arc<Worker>>,
	/// How many calls JavaScript's thread waits on.
	depth: usize,
}

/// A thread that runs a call from JavaScript while JavaScript's thread serves.
struct Worker {
	/// The call lent to it, which its lender keeps alive until the call's done, or null.
	call: AtomicPtr<Lent>,
	/// Whether it waits on `ready`, under `lock`: else it spins, and nothing need wake it.
	parked: AtomicBool,
	lock: Mutex<()>,
	ready: Condvar,
}

/// A call lent to a worker, which acts as `acting` while `run` runs, and tells JavaScript's
/// thread once `run` returned by setting `done` to 1 more than how many tasks were queued.
struct Lent {
	run: Run,
	done: *const AtomicUsize,
	acting: usize,
}

impl Worker {
	/// Run `call` on the worker of the depth of the calls JavaScript's thread waits on, or a
	/// new one.
	///
	/// # Safety
	/// On JavaScript's thread, which waits until `call` is done before it returns.
	unsafe fn lend(call: *const Lent) {
		// SAFETY: the caller's contract.
		let workers = unsafe { WORKERS.get() };
		if workers.depth == workers.all.len() {
			let worker = Arc::new(Worker {
				call: AtomicPtr::new(null_mut()),
				parked: AtomicBool::new(false),
				lock: Mutex::new(()),
				ready: Condvar::new(),
			});
			let parked = worker.clone();
			std::thread::spawn(move || parked.park());
			workers.all.push(worker);
		}
		let worker = &workers.all[workers.depth];
		workers.depth += 1;
		// Against `next`'s store of `parked` and load of `call`, so one of them sees the other's.
		worker.call.store(call.cast_mut(), Ordering::SeqCst);
		if worker.parked.load(Ordering::SeqCst) {
			let _lock = worker.lock.lock().unwrap_or_else(|e| e.into_inner());
			worker.ready.notify_one();
		}
	}

	/// The next call lent, spinning for it a while before parking.
	fn next(&self) -> *const Lent {
		let take = || Some(self.call.swap(null_mut(), Ordering::SeqCst)).filter(|call| !call.is_null());
		if let Some(call) = spin(take) {
			return call;
		}
		let mut lock = self.lock.lock().unwrap_or_else(|e| e.into_inner());
		self.parked.store(true, Ordering::SeqCst);
		loop {
			if let Some(call) = take() {
				self.parked.store(false, Ordering::Relaxed);
				return call;
			}
			lock = self.ready.wait(lock).unwrap_or_else(|e| e.into_inner());
		}
	}

	/// Run each call lent to this worker, idle between them.
	fn park(self: Arc<Self>) {
		loop {
			// SAFETY: its lender waits until `done`, which this sets last.
			let Lent { run, done, acting } = unsafe { self.next().read() };
			ACTING.set(acting);
			// A panic here would leave JavaScript's thread waiting forever, so it aborts, as
			// one through the C ABI would.
			// SAFETY: the lending thread waits until `done`.
			if std::panic::catch_unwind(AssertUnwindSafe(|| unsafe { run.run() })).is_err() {
				rt::fail("node", "a call from JavaScript panicked");
			}
			ACTING.set(0);
			// SAFETY: the lender is alive until this, after which this touches none of it.
			unsafe { (*done).store(HOME.pushed.load(Ordering::SeqCst) + 1, Ordering::SeqCst) };
			if HOME.parked.load(Ordering::SeqCst) {
				// Under the lock its wait holds until it waits, so this wakes it.
				let _tasks = HOME.lock();
				HOME.ready.notify_all();
			}
		}
	}
}

thread_local!(static ACTING: Cell<usize> = const { Cell::new(0) });

/// The thread this one acts as for the handles it borrows, if another: one waiting on it.
pub fn acting() -> Option<usize> {
	Some(ACTING.get()).filter(|&id| id != 0)
}

/// A closure another thread calls once, erased to a function and its data rather than a
/// `dyn FnOnce`, whose vtable would compile an unused shim of every call's closure.
struct Run {
	call: unsafe fn(*mut ()),
	data: *mut (),
}

// SAFETY: the thread that made it waits until it ran.
unsafe impl Send for Run {}

impl Run {
	/// # Safety
	/// `f` outlives the call, which takes it, and is not otherwise called or dropped.
	unsafe fn new<F: FnOnce()>(f: &mut ManuallyDrop<F>) -> Self {
		unsafe fn call<F: FnOnce()>(data: *mut ()) {
			// SAFETY: `new`'s contract.
			unsafe { ManuallyDrop::take(&mut *data.cast::<ManuallyDrop<F>>())() }
		}
		Run {
			call: call::<F>,
			data: std::ptr::from_mut(f).cast(),
		}
	}

	/// # Safety
	/// What made it is still alive, as [`Run::new`] requires.
	unsafe fn run(self) {
		// SAFETY: the caller's contract.
		unsafe { (self.call)(self.data) }
	}
}

/// A call into JavaScript another thread waits on.
struct Job {
	run: Mutex<Option<Run>>,
	/// The waiting thread, which JavaScript's acts as while it runs the call.
	thread: usize,
	/// The waiting thread, which a home's wait parks.
	waiter: std::thread::Thread,
	done: Mutex<bool>,
	ran: Condvar,
}

impl Job {
	fn run(&self) {
		let run = self.run.lock().unwrap_or_else(|e| e.into_inner()).take();
		if let Some(run) = run {
			let was = ACTING.replace(self.thread);
			// SAFETY: the waiting thread keeps it alive until `done`.
			unsafe { run.run() };
			ACTING.set(was);
		}
		self.finish();
	}

	/// Wake the waiting thread: the job ran, or else was abandoned with its call in it.
	fn finish(&self) {
		*self.done.lock().unwrap_or_else(|e| e.into_inner()) = true;
		self.ran.notify_all();
		self.waiter.unpark();
	}
}

/// The thread-safe function's finalizer, as the environment is torn down and before Node frees
/// it: no thread calls it after, and what is queued then or later is abandoned.
unsafe extern "C" fn on_freed(_: Env, _: *mut c_void, _: *mut c_void) {
	{
		let _telling = HOME.telling.lock().unwrap_or_else(|e| e.into_inner());
		HOME.tsfn.store(null_mut(), Ordering::Release);
	}
	HOME.exit();
}

/// The event loop's run of what other threads queued, as the thread-safe function calls it.
unsafe extern "C" fn on_loop(env: Env, _: Value, _: *mut c_void, _: *mut c_void) {
	// A null environment is the environment's teardown: what is queued never runs, so each
	// call waiting on it is told it was abandoned.
	if env.is_null() {
		HOME.exit();
		return;
	}
	HOME.told.store(false, Ordering::Release);
	// SAFETY: on JavaScript's thread, which runs no call from JavaScript meanwhile: what it
	// runs is counted as one, as `rt.ts` counts a call.
	unsafe {
		let out = Out::new();
		let entering = out.int32(1);
		// JavaScript refuses a call while its environment is torn down: what is queued, and
		// all queued later, is abandoned, and a call waiting on it fails rather than wait on.
		if out.attempt(6, &[entering]).is_err() {
			let mut e = null_mut();
			(api().get_and_clear_last_exception)(env, &mut e);
			HOME.exit();
			return;
		}
		// Only the tasks queued by now: each queued after `told` cleared tells the event loop
		// again, so a steady producer can't hold this turn.
		let queued = HOME.lock().len();
		for _ in 0..queued {
			let Some(task) = HOME.pop() else { break };
			HOME.run(task);
		}
		let leaving = out.int32(-1);
		out.invoke(6, &[leaving]);
	}
}

// What every API's values take: the C functions of what JavaScript implements.

unsafe extern "C" fn rust2_wake(data: *mut c_void) {
	token(0, data);
}

unsafe extern "C" fn rust2_waker_drop(data: *mut c_void) {
	token(1, data);
}

unsafe extern "C" fn rust2_free(data: *mut c_void) {
	token(2, data);
}

/// Call import `import` with the token `data` on JavaScript's thread, which nothing waits
/// on: at once there, or else once it runs what this queues.
fn token(import: usize, data: *mut c_void) {
	if on_js() {
		// SAFETY: on JavaScript's thread.
		let out = unsafe { Out::new() };
		let t = out.token(data);
		out.invoke(import, &[t]);
		return;
	}
	// One JavaScript has exited has nothing left to wake or close.
	HOME.push(Task::Token(import, data as usize));
}

unsafe extern "C" fn rust2_future_poll(data: *mut c_void, waker: rt::Waker) -> bool {
	HOME.call("a future's poll", |out| {
		let t = out.token(data);
		let at = out.value(&waker);
		let w = out.offset(at);
		out.int(out.invoke(3, &[t, w])) != 0
	})
}

unsafe extern "C" fn rust2_reader_read(data: *mut c_void, buf: *mut u8, len: usize, n: *mut usize, err: *mut rt::Buf) -> bool {
	HOME.call("a reader's read", |out| {
		let t = out.token(data);
		let b = out.bigint(buf as usize as i64);
		let l = out.number(len as f64);
		// SAFETY: Rust's reader passes places for each.
		let (n_at, err_at) = unsafe { (out.place(n, false), out.place(err, false)) };
		let (np, errp) = (out.offset(n_at), out.offset(err_at));
		let ok = out.int(out.invoke(4, &[t, b, l, np, errp])) != 0;
		// SAFETY: JavaScript wrote the count, or the error where it failed.
		unsafe {
			match ok {
				true => out.back(n, n_at),
				false => out.back(err, err_at),
			}
		}
		ok
	})
}

unsafe extern "C" fn rust2_reader_seek(
	data: *mut c_void,
	whence: std::ffi::c_int,
	offset: i64,
	position: *mut u64,
	err: *mut rt::Buf,
) -> bool {
	HOME.call("a reader's seek", |out| {
		let t = out.token(data);
		let w = out.int32(whence);
		let o = out.bigint(offset);
		// SAFETY: Rust's reader passes places for each.
		let (p_at, err_at) = unsafe { (out.place(position, false), out.place(err, false)) };
		let (pp, errp) = (out.offset(p_at), out.offset(err_at));
		let ok = out.int(out.invoke(5, &[t, w, o, pp, errp])) != 0;
		// SAFETY: JavaScript wrote the position, or the error where it failed.
		unsafe {
			match ok {
				true => out.back(position, p_at),
				false => out.back(err, err_at),
			}
		}
		ok
	})
}

/// The C functions every API's values take, as `rt.ts` reads their addresses.
#[repr(C)]
pub struct Fns {
	wake: unsafe extern "C" fn(*mut c_void),
	waker_drop: unsafe extern "C" fn(*mut c_void),
	free: unsafe extern "C" fn(*mut c_void),
	future_poll: unsafe extern "C" fn(*mut c_void, rt::Waker) -> bool,
	reader_read: unsafe extern "C" fn(*mut c_void, *mut u8, usize, *mut usize, *mut rt::Buf) -> bool,
	reader_seek: unsafe extern "C" fn(*mut c_void, std::ffi::c_int, i64, *mut u64, *mut rt::Buf) -> bool,
}

static FNS: Fns = Fns {
	wake: rust2_wake,
	waker_drop: rust2_waker_drop,
	free: rust2_free,
	future_poll: rust2_future_poll,
	reader_read: rust2_reader_read,
	reader_seek: rust2_reader_seek,
};

/// What a trait's table names for `free`.
pub const FREE: unsafe extern "C" fn(*mut c_void) = rust2_free;

/// Run `f`, a call into JavaScript on its thread: at once there, or else as a job JavaScript's
/// thread runs while this one waits.
pub fn call<R>(what: &str, f: impl FnOnce(&mut Out) -> R) -> R {
	HOME.call(what, f)
}

// The natives `rt.ts` calls.

/// Whether a copy of the binding took the addon's imports.
static INITIALIZED: AtomicBool = AtomicBool::new(false);

/// `$init(imports)`: take the imports, each by its name, and give the scratch memory.
unsafe extern "C" fn native_init(env: Env, info: Info) -> Value {
	// SAFETY: a call from JavaScript, on its thread.
	unsafe {
		let cx = Cx::<1>::new(env, info);
		// Node caches a module's exports, so a second copy of the binding, such as its `.js`
		// beside its `.ts`, reaches the same addon without registering it again: its imports
		// would replace those the first's tokens name.
		// A V8 build with its sandbox, such as Electron's, refuses memory outside its cage: the
		// load fails before it takes anything, so it throws rather than aborts.
		let mut memory = null_mut();
		let status =
			(api().create_external_arraybuffer)(env, base() as *mut c_void, SCRATCH, null(), null_mut(), &mut memory);
		if status != 0 {
			let message = c"the runtime refuses an external ArrayBuffer, as a V8 sandbox does: the addon needs one";
			(api().throw_error)(env, null(), message.as_ptr());
			return null_mut();
		}
		if INITIALIZED.swap(true, Ordering::AcqRel) {
			let message = c"the addon is loaded already, by another copy of its binding: import one copy";
			(api().throw_error)(env, null(), message.as_ptr());
			return null_mut();
		}
		let imports = IMPORTS.get();
		for name in RUNTIME_IMPORTS.iter().chain(EXTERNS.get().copied().unwrap_or_default()) {
			let mut f = null_mut();
			ok((api().get_named_property)(env, cx.argv[0], name.as_ptr(), &mut f), "an import");
			let mut r = null_mut();
			if (api().create_reference)(env, f, 1, &mut r) != 0 {
				rt::fail("node", &format!("the binding's load gave no import {}", name.to_string_lossy()));
			}
			imports.push(r);
		}
		memory
	}
}

/// `$base()`: the scratch memory's address.
unsafe extern "C" fn native_base(env: Env, info: Info) -> Value {
	// SAFETY: a call from JavaScript.
	let cx = unsafe { Cx::<0>::new(env, info) };
	cx.bigint(base() as i64)
}

/// `$fns(at)`: write [`Fns`] at `at`.
unsafe extern "C" fn native_fns(env: Env, info: Info) -> Value {
	// SAFETY: a call from JavaScript, whose place holds `Fns`.
	unsafe {
		let cx = Cx::<1>::new(env, info);
		cx.at(0).cast::<Fns>().copy_from_nonoverlapping(&FNS, 1);
		cx.undefined()
	}
}

/// `$copy(at, ptr, n)`: copy `n` bytes from Rust's `ptr` to `at`.
unsafe extern "C" fn native_copy(env: Env, info: Info) -> Value {
	// SAFETY: a call from JavaScript, which passes what Rust lent, and a place of `n` bytes.
	unsafe {
		let cx = Cx::<3>::new(env, info);
		let n = cx.int(2) as usize;
		if n > 0 {
			std::ptr::copy_nonoverlapping(cx.big(1) as usize as *const u8, cx.at(0), n);
		}
		cx.undefined()
	}
}

/// `$into(array, ptr)`: copy into `array` the bytes at Rust's `ptr`, as many as it holds.
unsafe extern "C" fn native_into(env: Env, info: Info) -> Value {
	// SAFETY: a call from JavaScript, which passes what Rust lent, of at least the array's length.
	unsafe {
		let cx = Cx::<2>::new(env, info);
		let (data, len) = cx.typed(0);
		std::ptr::copy_nonoverlapping(cx.big(1) as usize as *const u8, data, len);
		cx.undefined()
	}
}

/// `$write(ptr, array)`: copy `array` to Rust's `ptr`.
unsafe extern "C" fn native_write(env: Env, info: Info) -> Value {
	// SAFETY: a call from JavaScript, which passes a buffer Rust lent, of at least the array's length.
	unsafe {
		let cx = Cx::<2>::new(env, info);
		let (data, len) = cx.typed(1);
		std::ptr::copy_nonoverlapping(data, cx.big(0) as usize as *mut u8, len);
		cx.undefined()
	}
}

/// `$byte(ptr, i)`: the byte at Rust's `ptr + i`.
unsafe extern "C" fn native_byte(env: Env, info: Info) -> Value {
	// SAFETY: a call from JavaScript, which passes what Rust lent and an index in it.
	unsafe {
		let cx = Cx::<2>::new(env, info);
		let p = (cx.big(0) as usize + cx.int(1) as usize) as *const u8;
		cx.uint32(u32::from(*p))
	}
}

/// `$handle(at)`: the id of a new slot of the handle at `at`.
unsafe extern "C" fn native_handle(env: Env, info: Info) -> Value {
	// SAFETY: a call from JavaScript, whose place holds a handle.
	unsafe {
		let cx = Cx::<1>::new(env, info);
		cx.indexed(*cx.at(0).cast::<*const c_void>())
	}
}

/// `$put(at, id)`: write the handle of `id` at `at`, held until `$release(id)`, returning
/// whether it was open: a closed or freed one writes nothing.
unsafe extern "C" fn native_put(env: Env, info: Info) -> Value {
	// SAFETY: a call from JavaScript, whose place holds a handle.
	unsafe {
		let cx = Cx::<2>::new(env, info);
		let Some((_, p)) = hold(cx.id(1)) else {
			return cx.boolean(false);
		};
		*cx.at(0).cast::<*const c_void>() = p;
		cx.boolean(true)
	}
}

/// `$release(id)`: release the hold `$put` took of `id`.
unsafe extern "C" fn native_release(env: Env, info: Info) -> Value {
	// SAFETY: a call from JavaScript.
	unsafe {
		let cx = Cx::<1>::new(env, info);
		release(slot(cx.id(0)).0);
		cx.undefined()
	}
}

/// `$close(id, free, force)`: close `id`, freeing it with `free`, as [`close`] takes it,
/// once no call holds it. While one does, it closes only where `force`, returning whether it
/// closed.
unsafe extern "C" fn native_close(env: Env, info: Info) -> Value {
	// SAFETY: a call from JavaScript.
	unsafe {
		let cx = Cx::<3>::new(env, info);
		let (i, slot) = slot(cx.id(0));
		let free = cx.int(1) as u32;
		let mut force = false;
		ok((api().get_value_bool)(env, cx.argv[2], &mut force), "a boolean");
		if slot.holds == 0 {
			close(i, free);
		} else if force {
			slot.closing = free;
		} else {
			return cx.boolean(false);
		}
		cx.boolean(true)
	}
}

/// `$external(n)`: tell the garbage collector JavaScript holds `n` bytes more of Rust's,
/// fewer where negative.
unsafe extern "C" fn native_external(env: Env, info: Info) -> Value {
	// SAFETY: a call from JavaScript.
	unsafe {
		let cx = Cx::<1>::new(env, info);
		let mut total = 0;
		ok((api().adjust_external_memory)(env, cx.int(0), &mut total), "external memory");
		cx.undefined()
	}
}

/// `$pends(pending)`: keep the event loop alive while a call into Rust pends, which another
/// thread may wake.
unsafe extern "C" fn native_pends(env: Env, info: Info) -> Value {
	// SAFETY: a call from JavaScript.
	unsafe {
		let cx = Cx::<1>::new(env, info);
		let mut pending = false;
		ok((api().get_value_bool)(env, cx.argv[0], &mut pending), "a boolean");
		let tsfn = HOME.tsfn.load(Ordering::Acquire);
		match pending {
			true => ok((api().ref_threadsafe_function)(env, tsfn), "a reference"),
			false => ok((api().unref_threadsafe_function)(env, tsfn), "a reference"),
		}
		cx.undefined()
	}
}

/// `<space>_call(f, data)`: call the C function `f` with `data`, a waker's wake or drop.
pub unsafe extern "C" fn native_call(env: Env, info: Info) -> Value {
	// SAFETY: a call from JavaScript, of a function Rust gave and its data.
	unsafe {
		let cx = Cx::<2>::new(env, info);
		let f: Option<unsafe extern "C" fn(*mut c_void)> = from_bits(cx.big(0) as u64);
		let data = cx.big(1) as usize as *mut c_void;
		if let Some(f) = f {
			serve(|| f(data));
		}
		cx.undefined()
	}
}

/// `<space>_abort(at, len)`: abort with the message of `len` bytes at `at`.
pub unsafe extern "C" fn native_abort(env: Env, info: Info) -> Value {
	// SAFETY: a call from JavaScript, whose place holds the message.
	unsafe {
		let cx = Cx::<2>::new(env, info);
		let text = std::slice::from_raw_parts(cx.at(0), cx.int(1) as usize);
		rt::fail("node", &String::from_utf8_lossy(text))
	}
}

/// The glue's imports, after the runtime's, by name.
static EXTERNS: OnceLock<&'static [&'static CStr]> = OnceLock::new();

/// Whether an environment registered the addon.
static REGISTERED: AtomicBool = AtomicBool::new(false);

/// Register the addon in `env`: the natives and `calls` on `exports`, and the names of the
/// glue's imports, `externs`. Any later environment, as a worker's, is refused, even
/// once the first has exited.
///
/// # Safety
/// Node-API's module registration, on JavaScript's thread.
pub unsafe fn register(
	env: Env,
	exports: Value,
	calls: &[(&CStr, Callback)],
	externs: &'static [&'static CStr],
	frees: &'static [unsafe fn(*const c_void)],
) -> Value {
	let api = API.get_or_init(resolve);
	if REGISTERED.swap(true, Ordering::AcqRel) {
		// SAFETY: the caller's contract.
		unsafe {
			(api.throw_error)(
				env,
				null(),
				c"the addon is loaded in another environment already: the first to load it keeps it for the process's life".as_ptr(),
			)
		};
		return null_mut();
	}
	JS.set(true);
	ENV.store(env, Ordering::Relaxed);
	let _ = EXTERNS.set(externs);
	let _ = FREES.set(frees);
	BASE.store(reserve(), Ordering::Relaxed);
	// The word the relays' count is in.
	commit(0, CHUNK);
	// SAFETY: on JavaScript's thread, before any call.
	unsafe { SCRATCH_STATE.get().low = CHUNK };
	let natives: [(&CStr, Callback); 14] = [
		(c"$init", native_init),
		(c"$base", native_base),
		(c"$fns", native_fns),
		(c"$commit", native_commit),
		(c"$copy", native_copy),
		(c"$into", native_into),
		(c"$write", native_write),
		(c"$byte", native_byte),
		(c"$handle", native_handle),
		(c"$put", native_put),
		(c"$release", native_release),
		(c"$close", native_close),
		(c"$external", native_external),
		(c"$pends", native_pends),
	];
	// SAFETY: the caller's contract.
	unsafe {
		for (name, f) in natives.iter().chain(calls) {
			let mut function = null_mut();
			ok((api.create_function)(env, name.as_ptr(), usize::MAX, *f, null_mut(), &mut function), "a function");
			ok((api.set_named_property)(env, exports, name.as_ptr(), function), "an export");
		}
		let mut name = null_mut();
		let label = c"rust2";
		ok((api.create_string_utf8)(env, label.as_ptr(), usize::MAX, &mut name), "a name");
		let mut tsfn = null_mut();
		ok(
			(api.create_threadsafe_function)(
				env,
				null_mut(),
				null_mut(),
				name,
				0,
				1,
				null_mut(),
				Some(on_freed),
				null_mut(),
				Some(on_loop),
				&mut tsfn,
			),
			"the thread-safe function",
		);
		ok((api.unref_threadsafe_function)(env, tsfn), "the thread-safe function's reference");
		HOME.tsfn.store(tsfn, Ordering::Release);
	}
	exports
}
