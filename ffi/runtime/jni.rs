//! The runtime the Kotlin glue's JNI functions share: the codec values cross in, the
//! thread a call into Kotlin runs on, and the wakers a Kotlin future parks on.
//!
//! A value Kotlin passes is a JNI scalar, a handle as a `long`, its bytes as a `byte[]`,
//! or anything else encoded into a `byte[]`: little-endian, field by field, as the C
//! struct holds it, with a buffer as its length (-1 for none) and its bytes, and an array
//! as its length (-1 for none) and its elements. Bytes Rust returns cross copied when short,
//! and otherwise as their owner, which a Kotlin `Bytes` reads through a call each until the
//! GC collects it.

use std::cell::Cell;
use std::ffi::{CStr, c_void};
use std::sync::OnceLock;
use std::sync::atomic::{AtomicUsize, Ordering};

pub use jni_sys::*;

use crate::rt;

/// The JVM, and the global reference to the class holding the native methods and the
/// Kotlin functions Rust calls, set when the library loads.
static VM: OnceLock<usize> = OnceLock::new();
static CLASS: OnceLock<usize> = OnceLock::new();

/// Note the JVM and the class `name`, from `JNI_OnLoad`.
///
/// # Safety
/// `vm` is the JVM loading the library, on the thread loading it.
pub unsafe fn load(vm: *mut JavaVM, name: &CStr) -> jint {
	let _ = VM.set(vm as usize);
	let env = env();
	// SAFETY: `env` is this thread's; the class loader is the one loading the library,
	// which a thread Rust attaches later cannot reach.
	unsafe {
		let class = ((**env).v1_2.FindClass)(env, name.as_ptr());
		if class.is_null() {
			fail(env, "JNI_OnLoad", "the native class is missing");
		}
		let _ = CLASS.set(((**env).v1_2.NewGlobalRef)(env, class) as usize);
	}
	JNI_VERSION_1_6
}

/// The class holding the Kotlin functions Rust calls.
pub fn class() -> jclass {
	*CLASS.get().expect("the library is loaded") as jclass
}

thread_local! {
	/// Whether Rust attached this thread, so it detaches when the thread exits.
	static ATTACHED: Attached = const { Attached(Cell::new(false)) };
}

struct Attached(Cell<bool>);

impl Drop for Attached {
	fn drop(&mut self) {
		if self.0.get() {
			let vm = *VM.get().expect("attached") as *mut JavaVM;
			// SAFETY: this thread attached itself, and nothing on it calls Kotlin now.
			unsafe { ((**vm).v1_4.DetachCurrentThread)(vm) };
		}
	}
}

/// This thread's JNI environment, attaching it as a daemon until it exits if the JVM
/// does not know it, as a thread Rust spawned.
pub fn env() -> *mut JNIEnv {
	let vm = *VM.get().expect("the library is loaded") as *mut JavaVM;
	let mut env: *mut c_void = std::ptr::null_mut();
	// SAFETY: `vm` is the JVM, which outlives every thread calling into it.
	unsafe {
		if ((**vm).v1_4.GetEnv)(vm, &mut env, JNI_VERSION_1_6) == JNI_OK {
			return env.cast();
		}
		if ((**vm).v1_4.AttachCurrentThreadAsDaemon)(vm, &mut env, std::ptr::null_mut()) != JNI_OK {
			rt::fail("jni", "a thread cannot attach to the JVM");
		}
	}
	ATTACHED.with(|attached| attached.0.set(true));
	env.cast()
}

/// Abort for `what`, printing the Kotlin exception pending, if any.
pub fn fail(env: *mut JNIEnv, what: &str, msg: &str) -> ! {
	// SAFETY: `env` is this thread's.
	unsafe {
		if ((**env).v1_2.ExceptionCheck)(env) {
			((**env).v1_2.ExceptionDescribe)(env);
		}
	}
	rt::fail(what, msg)
}

/// Run `f` calling into Kotlin on this thread, freeing the local references it makes, and
/// abort if what it calls throws: Rust has nowhere to unwind it to.
pub fn upcall<R>(what: &str, f: impl FnOnce(*mut JNIEnv) -> R) -> R {
	let env = env();
	// SAFETY: `env` is this thread's.
	unsafe {
		if ((**env).v1_2.PushLocalFrame)(env, 8) != JNI_OK {
			fail(env, what, "a call into Kotlin is out of memory");
		}
		let result = f(env);
		if ((**env).v1_2.ExceptionCheck)(env) {
			fail(env, what, "a call into Kotlin threw");
		}
		((**env).v1_2.PopLocalFrame)(env, std::ptr::null_mut());
		result
	}
}

/// The static method `name` of the native class, whose type is `sig`, cached in `cache`.
pub fn method(env: *mut JNIEnv, cache: &OnceLock<usize>, name: &CStr, sig: &CStr) -> jmethodID {
	*cache.get_or_init(|| {
		// SAFETY: `env` is this thread's, and the class is a global reference.
		let id = unsafe { ((**env).v1_2.GetStaticMethodID)(env, class(), name.as_ptr(), sig.as_ptr()) };
		if id.is_null() {
			fail(env, "jni", "a Kotlin function the library calls is missing");
		}
		id as usize
	}) as jmethodID
}

/// A `byte[]`'s bytes, copied, or none for null.
pub fn bytes(env: *mut JNIEnv, array: jbyteArray) -> Option<Vec<u8>> {
	if array.is_null() {
		return None;
	}
	// SAFETY: `array` is a live `byte[]` the call was given.
	unsafe {
		let len = ((**env).v1_2.GetArrayLength)(env, array);
		let mut bytes = Vec::<u8>::with_capacity(len as usize);
		((**env).v1_2.GetByteArrayRegion)(env, array, 0, len, bytes.as_mut_ptr().cast());
		bytes.set_len(len as usize);
		Some(bytes)
	}
}

/// A view of `bytes`, or a null view for none.
pub fn view(bytes: &Option<Vec<u8>>) -> rt::Buf {
	match bytes {
		Some(bytes) => rt::Buf::view(bytes),
		None => rt::Buf::NONE,
	}
}

/// A new `byte[]` holding `bytes`.
pub fn array(env: *mut JNIEnv, bytes: &[u8]) -> jbyteArray {
	let Ok(len) = jsize::try_from(bytes.len()) else {
		rt::fail("jni", "a buffer is too long for a byte[]")
	};
	// SAFETY: `env` is this thread's, and `bytes` holds `len` bytes.
	unsafe {
		let array = ((**env).v1_2.NewByteArray)(env, len);
		if array.is_null() {
			fail(env, "jni", "a byte[] is out of memory");
		}
		((**env).v1_2.SetByteArrayRegion)(env, array, 0, len, bytes.as_ptr().cast());
		array
	}
}

/// The longest bytes Rust returns that cross copied, freed at once: a lent buffer costs a
/// cleaner registration, and a call per read, which copying up to about this many bytes
/// undercuts.
const COPIED: usize = 1024;

/// The most bytes lent to Kotlin at once, freed or not. The GC sees none of them, so a
/// program making little garbage of its own would never collect the `Bytes` holding them;
/// past this, bytes cross copied into Kotlin's heap, whose garbage the GC does see.
const LENT_MAX: usize = 64 << 20;

/// The bytes lent to Kotlin and not yet freed.
static LENT_BYTES: AtomicUsize = AtomicUsize::new(0);

/// Whether `len` bytes cross lent rather than copied: longer than [`COPIED`], and within
/// [`LENT_MAX`], which they then count against until freed. A call racing another past
/// the limit may copy where it could have lent, never the reverse.
fn lends(len: usize) -> bool {
	if len <= COPIED {
		return false;
	}
	if LENT_BYTES.fetch_add(len, Ordering::Relaxed) + len <= LENT_MAX {
		return true;
	}
	LENT_BYTES.fetch_sub(len, Ordering::Relaxed);
	false
}

/// A buffer Rust returned or lent, as Kotlin's `Native.owned` takes it: null for none, a
/// `byte[]` copy, freeing the buffer, or where it [`lends`] a `long[]` holding the owner it
/// lends them through and their length.
///
/// # Safety
/// `buf` is a buffer Rust returned or lent, freed by nothing else.
pub unsafe fn hand(env: *mut JNIEnv, buf: rt::Buf) -> jobject {
	if buf.is_none() {
		return std::ptr::null_mut();
	}
	if !lends(buf.len) {
		// SAFETY: the caller's contract.
		return unsafe {
			let array = array(env, buf.bytes("jni"));
			buf.free();
			array
		};
	}
	let len = lent_len(&buf);
	// SAFETY: the caller's contract.
	let lent = [unsafe { self::owner(buf) }, len.into()];
	// SAFETY: `env` is this thread's.
	unsafe {
		let array = ((**env).v1_2.NewLongArray)(env, 2);
		if array.is_null() {
			fail(env, "jni", "a long[] is out of memory");
		}
		((**env).v1_2.SetLongArrayRegion)(env, array, 0, 2, lent.as_ptr());
		array
	}
}

/// The length of a buffer handed to Kotlin, whose `Bytes` counts in an `Int`.
fn lent_len(buf: &rt::Buf) -> i32 {
	match i32::try_from(buf.len) {
		Ok(len) => len,
		Err(_) => rt::fail("jni", "a buffer is too long for Bytes"),
	}
}

/// The owner of a buffer Rust returned, handed to Kotlin uncopied: a view, which Rust lent
/// only for a call, is copied into an owned buffer.
///
/// # Safety
/// `buf` is a buffer Rust returned or lent, not none, and freed by nothing else.
unsafe fn owner(buf: rt::Buf) -> jlong {
	let buf = match buf.owner.is_null() {
		// SAFETY: the caller's contract.
		true => rt::Buf::owned(bytes::Bytes::copy_from_slice(unsafe { buf.bytes("jni") })),
		false => buf,
	};
	buf.owner as usize as jlong
}

/// The bytes `owner` holds from `start`, `len` long, failing loud past their end, which
/// Kotlin checks first.
///
/// # Safety
/// `owner` came from [`hand`], and is not yet freed.
unsafe fn range<'a>(owner: jlong, start: jint, len: jint) -> &'a [u8] {
	// SAFETY: the caller's contract; the owner holds the bytes until freed.
	let bytes = unsafe {
		let buf = rt::Buf::of_owner(owner as usize as _);
		std::slice::from_raw_parts(buf.ptr, buf.len)
	};
	match (usize::try_from(start), usize::try_from(len)) {
		(Ok(start), Ok(len)) if start <= bytes.len() && len <= bytes.len() - start => &bytes[start..start + len],
		_ => rt::fail("jni", "a read is past a lent buffer's end"),
	}
}

/// The byte at `index` of those `owner` holds.
///
/// # Safety
/// As [`range`].
pub unsafe fn byte_at(owner: jlong, index: jint) -> jbyte {
	// SAFETY: the caller's contract.
	unsafe { range(owner, index, 1)[0] as jbyte }
}

/// Copy the bytes `owner` holds from `start`, `len` long, into `array` at `at`.
///
/// # Safety
/// As [`range`], and `array` is a live `byte[]`.
pub unsafe fn copy(env: *mut JNIEnv, owner: jlong, start: jint, len: jint, array: jbyteArray, at: jint) {
	// SAFETY: the caller's contract; JNI throws if `array` is too short.
	unsafe {
		let bytes = range(owner, start, len);
		((**env).v1_2.SetByteArrayRegion)(env, array, at, len, bytes.as_ptr().cast());
	}
}

/// Free the bytes `owner` holds.
///
/// # Safety
/// `owner` came from [`hand`], and is freed once.
pub unsafe fn free_bytes(owner: jlong) {
	// SAFETY: the caller's contract.
	let buf = unsafe { rt::Buf::of_owner(owner as usize as _) };
	let len = buf.len;
	// SAFETY: the caller's contract.
	unsafe { buf.free() };
	LENT_BYTES.fetch_sub(len, Ordering::Relaxed);
}

/// A buffer Rust returned as a `byte[]`, or null for none, freeing it.
///
/// # Safety
/// `buf` is a buffer Rust returned, freed by nothing else.
pub unsafe fn owned(env: *mut JNIEnv, buf: rt::Buf) -> jbyteArray {
	if buf.is_none() {
		return std::ptr::null_mut();
	}
	// SAFETY: the caller's contract.
	unsafe {
		let array = array(env, buf.bytes("jni"));
		buf.free();
		array
	}
}

/// A buffer Rust lent for a call as a `byte[]`, or null for none.
///
/// # Safety
/// `buf` is a buffer Rust lent for the call.
pub unsafe fn lent(env: *mut JNIEnv, buf: rt::Buf) -> jbyteArray {
	match buf.is_none() {
		true => std::ptr::null_mut(),
		// SAFETY: the caller's contract.
		false => array(env, unsafe { buf.bytes("jni") }),
	}
}

/// A global reference to a Kotlin value, which Rust may hold on any thread.
pub fn global(env: *mut JNIEnv, value: jobject) -> *mut c_void {
	if value.is_null() {
		rt::fail("jni", "a Kotlin value is null");
	}
	// SAFETY: `env` is this thread's, and `value` a live reference.
	unsafe { ((**env).v1_2.NewGlobalRef)(env, value).cast() }
}

/// Delete a global reference [`global`] made, from any thread.
///
/// # Safety
/// `data` is a global reference, deleted once.
pub unsafe extern "C" fn free_global(data: *mut c_void) {
	let env = env();
	// SAFETY: the caller's contract.
	unsafe { ((**env).v1_2.DeleteGlobalRef)(env, data.cast()) };
}

static FREE: OnceLock<usize> = OnceLock::new();

/// Let go of a Kotlin implementation of a trait, from any thread: closed first if it is
/// `AutoCloseable`.
///
/// # Safety
/// `data` is a global reference, deleted once.
pub unsafe extern "C" fn free_callback(data: *mut c_void) {
	upcall("free", |env| {
		let id = method(env, &FREE, c"__free", c"(Ljava/lang/Object;)V");
		let args = [jvalue { l: data.cast() }];
		// SAFETY: `__free` takes the implementation.
		unsafe { ((**env).v1_2.CallStaticVoidMethodA)(env, class(), id, args.as_ptr()) };
	});
	// SAFETY: the caller's contract.
	unsafe { free_global(data) };
}

/// A global reference for the length of a call.
pub struct Global(pub *mut c_void);

impl Global {
	pub fn new(env: *mut JNIEnv, value: jobject) -> Self {
		Self(global(env, value))
	}
}

impl Drop for Global {
	fn drop(&mut self) {
		// SAFETY: made by `new`, and dropped once.
		unsafe { free_global(self.0) };
	}
}

/// A Kotlin `Waker`, counted: Rust holds one reference per waker it was given, and Kotlin
/// one while it polls a future with it.
pub struct Node {
	waker: *mut c_void,
	refs: AtomicUsize,
}

/// A node holding one reference, for a `Waker`.
pub fn node(env: *mut JNIEnv, waker: jobject) -> *mut Node {
	Box::into_raw(Box::new(Node {
		waker: global(env, waker),
		refs: AtomicUsize::new(1),
	}))
}

/// Drop a reference to `node`, freeing it with the last.
///
/// # Safety
/// The caller holds a reference to `node`.
pub unsafe extern "C" fn unref(node: *mut c_void) {
	let node = node.cast::<Node>();
	// SAFETY: the caller's contract.
	if unsafe { &*node }.refs.fetch_sub(1, Ordering::AcqRel) == 1 {
		// SAFETY: that was the last reference.
		let node = unsafe { Box::from_raw(node) };
		// SAFETY: the node made the global reference.
		unsafe { free_global(node.waker) };
	}
}

static WAKE: OnceLock<usize> = OnceLock::new();

unsafe extern "C" fn wake(node: *mut c_void) {
	// SAFETY: Rust holds a reference to the node while it may wake it.
	let waker = unsafe { &*node.cast::<Node>() }.waker;
	upcall("wake", |env| {
		let id = method(env, &WAKE, c"__wake", c"(Ljava/lang/Object;)V");
		let args = [jvalue { l: waker.cast() }];
		// SAFETY: `wake` takes the waker.
		unsafe { ((**env).v1_2.CallStaticVoidMethodA)(env, class(), id, args.as_ptr()) };
	});
}

/// `node` as a waker that borrows its reference: a poll given it again reuses it rather
/// than dropping it.
pub fn lend(node: *mut Node) -> rt::Waker {
	rt::Waker {
		wake: Some(wake),
		drop: Some(unref),
		data: node.cast(),
	}
}

/// `node` as a waker holding a new reference, which Rust drops.
///
/// # Safety
/// The caller holds a reference to `node`.
pub unsafe fn give(node: *mut Node) -> rt::Waker {
	// SAFETY: the caller's contract.
	unsafe { &*node }.refs.fetch_add(1, Ordering::Relaxed);
	lend(node)
}

/// Poll a future with a waker, writing its result into the writer if it finished.
pub type Poll = unsafe fn(*mut c_void, rt::Waker, &mut Writer) -> bool;

/// Poll the future just made, and free it if it finished: the call and its first poll are
/// one crossing. Its result is a bool, true when finished, then the result, or else the
/// future and the waker node made for `signal`, which Kotlin polls and frees.
///
/// # Safety
/// `future` is a live future that `poll` and `free` take, and `signal` a Kotlin `Waker`.
pub unsafe fn start(env: *mut JNIEnv, future: *mut c_void, signal: jobject, poll: Poll, free: unsafe fn(*mut c_void)) -> jbyteArray {
	let mut w = Writer::default();
	w.put(true);
	// Most futures are ready at once, so the first poll takes no waker: a node costs a
	// global reference, and two crossings to free it.
	let none = rt::Waker {
		wake: None,
		drop: None,
		data: std::ptr::null_mut(),
	};
	// SAFETY: the caller's contract.
	if unsafe { poll(future, none, &mut w) } {
		// SAFETY: as above, and it finished.
		unsafe { free(future) };
		return w.array(env);
	}
	// Pending, so polled again with the waker it parks on, which replaces the first.
	let node = node(env, signal);
	// SAFETY: as above, and `node` holds a reference for this.
	if unsafe { poll(future, give(node), &mut w) } {
		// SAFETY: as above.
		unsafe {
			free(future);
			unref(node.cast());
		}
		return w.array(env);
	}
	let mut w = Writer::default();
	w.put(false);
	w.put(future);
	w.put(node.cast::<c_void>());
	w.array(env)
}

/// Poll `future` again with `node`, whose reference Rust holds from the first poll that
/// pended: its result, or null while pending.
///
/// # Safety
/// `future` is a live future that `poll` takes, and Rust holds a reference to `node`.
pub unsafe fn poll(env: *mut JNIEnv, future: *mut c_void, node: *mut Node, poll: Poll) -> jbyteArray {
	let mut w = Writer::default();
	// SAFETY: the caller's contract.
	match unsafe { poll(future, lend(node), &mut w) } {
		true => w.array(env),
		false => std::ptr::null_mut(),
	}
}

/// A Kotlin `Waker` passed by value: Rust holds its only reference.
pub fn waker(env: *mut JNIEnv, waker: jobject) -> rt::Waker {
	lend(node(env, waker))
}

/// Encoded values, read in the order they were written, with the arrays read, which the
/// arrays' views point into until the reader drops.
pub struct Reader<'a> {
	bytes: &'a [u8],
	arrays: Vec<Box<dyn std::any::Any>>,
}

impl<'a> Reader<'a> {
	pub fn new(bytes: &'a [u8]) -> Self {
		Self { bytes, arrays: Vec::new() }
	}

	fn take(&mut self, n: usize) -> &'a [u8] {
		if self.bytes.len() < n {
			rt::fail("jni", "an encoded value is truncated");
		}
		let (head, rest) = self.bytes.split_at(n);
		self.bytes = rest;
		head
	}

	pub fn get<T: Wire>(&mut self) -> T {
		T::read(self)
	}
}

thread_local! {
	/// The buffer a writer encodes into, kept between calls so a result allocates nothing
	/// but its `byte[]`.
	static SCRATCH: Cell<Vec<u8>> = const { Cell::new(Vec::new()) };
}

/// Values encoded into a `byte[]` for Kotlin.
pub struct Writer(Vec<u8>);

impl Default for Writer {
	fn default() -> Self {
		Self(SCRATCH.try_with(Cell::take).unwrap_or_default())
	}
}

impl Drop for Writer {
	fn drop(&mut self) {
		let mut buf = std::mem::take(&mut self.0);
		// A large one is let go, so no thread keeps the biggest value it ever returned.
		if buf.capacity() <= 4096 {
			buf.clear();
			let _ = SCRATCH.try_with(|scratch| scratch.set(buf));
		}
	}
}

impl Writer {
	pub fn put<T: Wire>(&mut self, value: T) {
		value.write(self);
	}

	pub fn array(self, env: *mut JNIEnv) -> jbyteArray {
		array(env, &self.0)
	}
}

/// A C value as it crosses encoded: Kotlin writes one Rust reads, and Rust writes one it
/// owns, freeing what it holds once copied.
pub trait Wire: Sized {
	fn read(r: &mut Reader<'_>) -> Self;
	fn write(self, w: &mut Writer);
}

macro_rules! scalar {
	($($t:ty),*) => {$(
		impl Wire for $t {
			fn read(r: &mut Reader<'_>) -> Self {
				Self::from_le_bytes(r.take(size_of::<Self>()).try_into().expect("taken"))
			}

			fn write(self, w: &mut Writer) {
				w.0.extend_from_slice(&self.to_le_bytes());
			}
		}
	)*};
}

scalar!(u8, u16, u32, u64, i8, i16, i32, i64, f32, f64);

impl Wire for bool {
	fn read(r: &mut Reader<'_>) -> Self {
		r.get::<u8>() != 0
	}

	fn write(self, w: &mut Writer) {
		w.put(u8::from(self));
	}
}

impl Wire for usize {
	fn read(r: &mut Reader<'_>) -> Self {
		r.get::<u64>() as usize
	}

	fn write(self, w: &mut Writer) {
		w.put(self as u64);
	}
}

impl Wire for isize {
	fn read(r: &mut Reader<'_>) -> Self {
		r.get::<i64>() as isize
	}

	fn write(self, w: &mut Writer) {
		w.put(self as i64);
	}
}

impl Wire for *const c_void {
	fn read(r: &mut Reader<'_>) -> Self {
		r.get::<u64>() as usize as Self
	}

	fn write(self, w: &mut Writer) {
		w.put(self as usize as u64);
	}
}

impl Wire for *mut c_void {
	fn read(r: &mut Reader<'_>) -> Self {
		r.get::<u64>() as usize as Self
	}

	fn write(self, w: &mut Writer) {
		w.put(self as usize as u64);
	}
}

impl Wire for rt::Buf {
	/// A view of the encoded bytes, which outlive the call.
	fn read(r: &mut Reader<'_>) -> Self {
		match usize::try_from(r.get::<i32>()) {
			Ok(len) => rt::Buf::view(r.take(len)),
			Err(_) => rt::Buf::NONE,
		}
	}

	fn write(self, w: &mut Writer) {
		if self.is_none() {
			w.put(-1i32);
			return;
		}
		// SAFETY: a buffer Rust returned, freed here once copied.
		let bytes = unsafe { self.bytes("jni") };
		let Ok(len) = i32::try_from(bytes.len()) else {
			rt::fail("jni", "a buffer is too long for a byte[]")
		};
		w.put(len);
		w.0.extend_from_slice(bytes);
		// SAFETY: as above.
		unsafe { self.free() };
	}
}

/// Bytes Rust returns, or an array of them, each crossing as [`hand`] hands it over.
pub struct Owned<T>(pub T);

/// What precedes a lent buffer's owner where an encoded one's length would.
const LENT: i32 = -2;

impl Wire for Owned<rt::Buf> {
	fn read(_: &mut Reader<'_>) -> Self {
		rt::fail("jni", "Kotlin passes bytes, never their owner")
	}

	// None as -1, bytes it [`lends`] as [`LENT`], their owner, and their length, and others
	// as their length and a copy.
	fn write(self, w: &mut Writer) {
		let buf = self.0;
		if buf.is_none() {
			w.put(-1i32);
			return;
		}
		let len = lent_len(&buf);
		if lends(buf.len) {
			w.put(LENT);
			// SAFETY: a buffer Rust returned or lent, handed over here.
			w.put(unsafe { owner(buf) });
			w.put(len);
		} else {
			w.put(len);
			// SAFETY: a buffer Rust returned or lent, freed here once copied.
			unsafe {
				w.0.extend_from_slice(buf.bytes("jni"));
				buf.free();
			}
		}
	}
}

impl<T: Copy + 'static> Wire for Owned<rt::List<T>>
where
	Owned<T>: Wire,
{
	fn read(_: &mut Reader<'_>) -> Self {
		rt::fail("jni", "Kotlin passes bytes, never their owner")
	}

	fn write(self, w: &mut Writer) {
		let list = self.0;
		if list.is_none() {
			w.put(-1i32);
			return;
		}
		let Ok(len) = i32::try_from(list.len) else {
			rt::fail("jni", "an array is too long for a List")
		};
		w.put(len);
		// SAFETY: an array Rust returned, whose elements move into the writer, so it is
		// freed without them.
		unsafe {
			list.items("jni").iter().for_each(|&item| w.put(Owned(item)));
			rt::List { len: 0, ..list }.free("jni", |_| {});
		}
	}
}

impl Wire for rt::U128 {
	fn read(r: &mut Reader<'_>) -> Self {
		Self { lo: r.get(), hi: r.get() }
	}

	fn write(self, w: &mut Writer) {
		w.put(self.lo);
		w.put(self.hi);
	}
}

impl Wire for rt::Duration {
	fn read(r: &mut Reader<'_>) -> Self {
		Self {
			secs: r.get(),
			nanos: r.get(),
		}
	}

	fn write(self, w: &mut Writer) {
		w.put(self.secs);
		w.put(self.nanos);
	}
}

impl Wire for rt::SocketAddr {
	fn read(r: &mut Reader<'_>) -> Self {
		Self {
			ip: std::array::from_fn(|_| r.get()),
			port: r.get(),
			v6: r.get(),
			flowinfo: r.get(),
			scope_id: r.get(),
		}
	}

	fn write(self, w: &mut Writer) {
		self.ip.into_iter().for_each(|b| w.put(b));
		w.put(self.port);
		w.put(self.v6);
		w.put(self.flowinfo);
		w.put(self.scope_id);
	}
}

impl<T: Wire + Copy + 'static> Wire for rt::List<T> {
	/// A view of the elements, as their length, or -1 for none, then each element, which
	/// the reader keeps.
	fn read(r: &mut Reader<'_>) -> Self {
		let Ok(len) = usize::try_from(r.get::<i32>()) else {
			return Self::NONE;
		};
		let items: Box<[T]> = (0..len).map(|_| r.get()).collect();
		let list = Self {
			ptr: items.as_ptr(),
			len,
			owner: std::ptr::null_mut(),
		};
		r.arrays.push(Box::new(items));
		list
	}

	fn write(self, w: &mut Writer) {
		if self.is_none() {
			w.put(-1i32);
			return;
		}
		let Ok(len) = i32::try_from(self.len) else {
			rt::fail("jni", "an array is too long for a List")
		};
		w.put(len);
		// SAFETY: an array Rust returned, whose elements move into the writer, so it is
		// freed without them.
		unsafe {
			self.items("jni").iter().for_each(|&item| w.put(item));
			Self { len: 0, ..self }.free("jni", |_| {});
		}
	}
}

impl<T: Wire> Wire for rt::Range<T> {
	fn read(r: &mut Reader<'_>) -> Self {
		Self {
			start: r.get(),
			end: r.get(),
		}
	}

	fn write(self, w: &mut Writer) {
		w.put(self.start);
		w.put(self.end);
	}
}

impl<T: Wire> Wire for rt::Bound<T> {
	fn read(r: &mut Reader<'_>) -> Self {
		Self {
			kind: r.get(),
			value: r.get(),
		}
	}

	fn write(self, w: &mut Writer) {
		w.put(self.kind);
		w.put(self.value);
	}
}

impl<T: Wire> Wire for rt::Bounds<T> {
	fn read(r: &mut Reader<'_>) -> Self {
		Self {
			start: r.get(),
			end: r.get(),
		}
	}

	fn write(self, w: &mut Writer) {
		w.put(self.start);
		w.put(self.end);
	}
}
