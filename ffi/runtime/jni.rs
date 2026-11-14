//! The runtime the Kotlin glue's JNI functions share: the codec values cross in, the
//! thread a call into Kotlin runs on, and the wakers a Kotlin future parks on.
//!
//! A value Kotlin passes is a JNI scalar, a handle as a `long`, its bytes as a `byte[]`,
//! or composite returns constructed by a Kotlin factory over scoped native memory.
//! The codec (also used for parameters) is little-endian, field by field, as the C
//! struct holds it, with a buffer as its length (-1 for none) and its bytes, and an array
//! as its length (-1 for none) and its elements; a record passed or returned directly
//! crosses its scalars as parameters, or through Kotlin's `long[]` slots, apart from the
//! rest. Bytes Rust returns cross copied when short,
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
		// JNI permits a VM without direct-buffer access. Refuse it while loading,
		// before a call can hand over values that its return factory cannot read.
		let mut probe = 0u8;
		let buffer = ((**env).v1_4.NewDirectByteBuffer)(env, (&mut probe as *mut u8).cast(), 1);
		if buffer.is_null() {
			fail(env, "JNI_OnLoad", "the JVM does not support native return buffers");
		}
		((**env).v1_2.DeleteLocalRef)(env, buffer);
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

/// A number Kotlin holds in the primitive array of its width, which crosses JNI whole.
pub trait Prim: Copy {
	/// Copy `len` of `array`'s numbers into `into`.
	///
	/// # Safety
	/// `array` is a live array of this width holding `len`, and `into` has room for them.
	unsafe fn get(env: *mut JNIEnv, array: jarray, len: jsize, into: *mut Self);

	/// A new array of `len` numbers.
	///
	/// # Safety
	/// `env` is this thread's.
	unsafe fn new(env: *mut JNIEnv, len: jsize) -> jarray;

	/// Copy `len` numbers from `from` into `array`.
	///
	/// # Safety
	/// `array` is a live array of this width holding `len`, and `from` holds them.
	unsafe fn set(env: *mut JNIEnv, array: jarray, len: jsize, from: *const Self);
}

macro_rules! prim {
	($($t:ty: $get:ident, $new:ident, $set:ident;)*) => {$(
		impl Prim for $t {
			unsafe fn get(env: *mut JNIEnv, array: jarray, len: jsize, into: *mut Self) {
				// SAFETY: the caller's contract.
				unsafe { ((**env).v1_2.$get)(env, array, 0, len, into.cast()) }
			}

			unsafe fn new(env: *mut JNIEnv, len: jsize) -> jarray {
				// SAFETY: the caller's contract.
				unsafe { ((**env).v1_2.$new)(env, len) }
			}

			unsafe fn set(env: *mut JNIEnv, array: jarray, len: jsize, from: *const Self) {
				// SAFETY: the caller's contract.
				unsafe { ((**env).v1_2.$set)(env, array, 0, len, from.cast()) }
			}
		}
	)*};
}

prim! {
	i8: GetByteArrayRegion, NewByteArray, SetByteArrayRegion;
	u16: GetShortArrayRegion, NewShortArray, SetShortArrayRegion;
	i16: GetShortArrayRegion, NewShortArray, SetShortArrayRegion;
	u32: GetIntArrayRegion, NewIntArray, SetIntArrayRegion;
	i32: GetIntArrayRegion, NewIntArray, SetIntArrayRegion;
	u64: GetLongArrayRegion, NewLongArray, SetLongArrayRegion;
	i64: GetLongArrayRegion, NewLongArray, SetLongArrayRegion;
	f32: GetFloatArrayRegion, NewFloatArray, SetFloatArrayRegion;
	f64: GetDoubleArrayRegion, NewDoubleArray, SetDoubleArrayRegion;
}

/// A primitive array's numbers, copied in one call.
pub fn block<T: Prim>(env: *mut JNIEnv, array: jarray) -> Vec<T> {
	if array.is_null() {
		rt::fail("jni", "a primitive array is null");
	}
	// SAFETY: `array` is a live array of `T`'s width the call was given.
	unsafe {
		let len = ((**env).v1_2.GetArrayLength)(env, array);
		let mut items = Vec::<T>::with_capacity(len as usize);
		T::get(env, array, len, items.as_mut_ptr());
		items.set_len(len as usize);
		items
	}
}

/// `items` as the C array a call takes, which keeps them.
pub fn list<T: Copy>(items: &[T]) -> rt::List<T> {
	rt::List {
		ptr: items.as_ptr(),
		len: items.len(),
		owner: std::ptr::null_mut(),
	}
}

/// A new primitive array of the numbers Rust returned, which it frees once copied.
pub fn new_block<T: Prim>(env: *mut JNIEnv, list: rt::List<T>) -> jarray {
	// SAFETY: an array Rust returned, freed here once copied.
	unsafe {
		let items = list.items("jni");
		let Ok(len) = jsize::try_from(items.len()) else {
			rt::fail("jni", "an array is too long for a Kotlin array")
		};
		let array = T::new(env, len);
		if array.is_null() {
			fail(env, "jni", "a primitive array is out of memory");
		}
		T::set(env, array, len, items.as_ptr());
		list.free("jni", |_| {});
		array
	}
}

/// A `String`'s UTF-8, encoded once into a buffer Rust takes, or none for null.
pub fn text(env: *mut JNIEnv, text: jstring) -> rt::Buf {
	match text.is_null() {
		true => rt::Buf::NONE,
		false => rt::Buf::vec(utf8(env, text)),
	}
}

/// A live `String`'s UTF-8, as Kotlin's `encodeToByteArray` encodes it: a lone surrogate
/// is `?`. JNI's own UTF-8 is modified, which differs for NUL and past the BMP, so the
/// UTF-16 is read a chunk at a time and encoded here.
fn utf8(env: *mut JNIEnv, text: jstring) -> Vec<u8> {
	const CHUNK: usize = 1024;
	// SAFETY: `text` is a live `String` the call was given.
	let len = unsafe { ((**env).v1_2.GetStringLength)(env, text) } as usize;
	let mut out = Vec::with_capacity(len);
	let mut chunk = [0u16; CHUNK];
	let mut at = 0;
	while at < len {
		let n = (len - at).min(CHUNK);
		// SAFETY: `at + n` is within the string, and `chunk` holds `n`.
		unsafe { ((**env).v1_2.GetStringRegion)(env, text, at as jint, n as jint, chunk.as_mut_ptr()) };
		let mut units = &chunk[..n];
		// A pair split across chunks is read whole with the next.
		if at + n < len && (0xD800..0xDC00).contains(&units[n - 1]) {
			units = &units[..n - 1];
		}
		at += units.len();
		for c in char::decode_utf16(units.iter().copied()) {
			match c {
				Ok(c) if c.is_ascii() => out.push(c as u8),
				Ok(c) => out.extend_from_slice(c.encode_utf8(&mut [0; 4]).as_bytes()),
				Err(_) => out.push(b'?'),
			}
		}
	}
	out
}

/// `len` bytes of a live `byte[]` from `start`, copied, failing loud past its end, which
/// Kotlin's `Bytes` checks first.
fn region(env: *mut JNIEnv, array: jbyteArray, start: jint, len: jint) -> Vec<u8> {
	let Ok(n) = usize::try_from(len) else {
		rt::fail("jni", "bytes passed have a negative length")
	};
	let mut bytes = Vec::<u8>::with_capacity(n);
	// SAFETY: `array` is a live `byte[]`; JNI checks the range, throwing past its end.
	unsafe {
		((**env).v1_2.GetByteArrayRegion)(env, array, start, len, bytes.as_mut_ptr().cast());
		if ((**env).v1_2.ExceptionCheck)(env) {
			fail(env, "jni", "bytes passed are past their array's end");
		}
		bytes.set_len(n);
	}
	bytes
}

/// Bytes Kotlin passes as a `Bytes` holds them, for Rust to take: none where `len` is -1,
/// else those from `start` of what a non-zero `owner` holds, shared without a copy, or of
/// the `byte[]` `held`, copied once.
///
/// # Safety
/// A non-zero `owner` came from [`hand`], and `held` keeps it from being freed.
pub unsafe fn passed(env: *mut JNIEnv, held: jobject, owner: jlong, start: jint, len: jint) -> rt::Buf {
	if len == -1 {
		return rt::Buf::NONE;
	}
	if owner != 0 {
		// SAFETY: the caller's contract.
		let bytes = unsafe { range(owner, start, len) };
		let buf = rt::Buf {
			ptr: bytes.as_ptr(),
			len: bytes.len(),
			owner: owner as usize as _,
		};
		// SAFETY: as above.
		return unsafe { buf.share("jni") };
	}
	rt::Buf::vec(region(env, held, start, len))
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

/// Set the first of Kotlin's `long[]` `slots` to `values`: a returned record's scalars,
/// which cross with no `byte[]` to allocate and decode. Slots shorter than `values` throw.
pub fn slots(env: *mut JNIEnv, slots: jlongArray, values: &[i64]) {
	// SAFETY: `env` is this thread's, `slots` a live `long[]` the call was given, and
	// `values` holds `values.len()` longs, which a record's few scalars fit a `jsize`.
	unsafe { ((**env).v1_2.SetLongArrayRegion)(env, slots, 0, values.len() as jsize, values.as_ptr()) }
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
/// one crossing. Its writer holds a bool, true when finished, then the result, or else the
/// future and the waker node made for `signal`, which Kotlin polls and frees.
///
/// # Safety
/// `future` is a live future that `poll` and `free` take, and `signal` a Kotlin `Waker`.
pub unsafe fn start(env: *mut JNIEnv, future: *mut c_void, signal: jobject, poll: Poll, free: unsafe fn(*mut c_void)) -> Writer {
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
		return w;
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
		return w;
	}
	let mut w = Writer::default();
	w.put(false);
	w.put(future);
	w.put(node.cast::<c_void>());
	w
}

/// Poll `future` again with `node`, whose reference Rust holds from the first poll that
/// pended: its result, or null while pending.
///
/// # Safety
/// `future` is a live future that `poll` takes, and Rust holds a reference to `node`.
pub unsafe fn poll(future: *mut c_void, node: *mut Node, poll: Poll) -> Option<Writer> {
	let mut w = Writer::default();
	// SAFETY: the caller's contract.
	match unsafe { poll(future, lend(node), &mut w) } {
		true => Some(w),
		false => None,
	}
}

static POLL: OnceLock<usize> = OnceLock::new();

/// A Kotlin `Pollable` as the future Rust polls, which Rust closes once it drops it.
pub fn future(env: *mut JNIEnv, pollable: jobject) -> rt::ForeignFuture {
	rt::ForeignFuture {
		poll: Some(poll_future),
		free: Some(free_callback),
		data: global(env, pollable),
	}
}

/// Poll a Kotlin `Pollable`, from any thread, handing it Rust's waker as [`returned`]
/// boxes it.
unsafe extern "C" fn poll_future(data: *mut c_void, waker: rt::Waker) -> bool {
	let waker = returned(waker);
	upcall("poll", |env| {
		let id = method(env, &POLL, c"__poll", c"(Ljava/lang/Object;J)Z");
		let args = [jvalue { l: data.cast() }, jvalue { j: waker }];
		// SAFETY: `__poll` takes the `Pollable` and the waker's pointer.
		unsafe { ((**env).v1_2.CallStaticBooleanMethodA)(env, class(), id, args.as_ptr()) }
	})
}

static READ: OnceLock<usize> = OnceLock::new();
static SEEK: OnceLock<usize> = OnceLock::new();
static IO_ERROR: OnceLock<usize> = OnceLock::new();
static ERROR: OnceLock<usize> = OnceLock::new();

/// A Kotlin `SeekableByteChannel`, held by the global reference `channel`, as the reader
/// Rust reads for the call.
pub fn reader(channel: *mut c_void) -> rt::ForeignReader {
	rt::ForeignReader {
		read: Some(read_channel),
		seek: Some(seek_channel),
		data: channel,
	}
}

/// Read a Kotlin channel into `buf`, from any thread: as many bytes as one `read` gives,
/// copied out of the `byte[]` Kotlin fills, so Kotlin never sees Rust's memory.
unsafe extern "C" fn read_channel(data: *mut c_void, buf: *mut u8, len: usize, n: *mut usize, err: *mut rt::Buf) -> bool {
	upcall("SeekableByteChannel.read", |env| {
		let id = method(env, &READ, c"__read", c"(Ljava/lang/Object;I)[B");
		let asked = len.min(jint::MAX as usize);
		let args = [jvalue { l: data.cast() }, jvalue { i: asked as jint }];
		// SAFETY: `__read` takes the channel and how many bytes to read.
		let array = unsafe { ((**env).v1_2.CallStaticObjectMethodA)(env, class(), id, args.as_ptr()) };
		if let Some(error) = caught(env, "SeekableByteChannel.read", &IO_ERROR, c"__io_error") {
			// SAFETY: C's contract: `err` is writable.
			unsafe { err.write(rt::Buf::vec(error)) };
			return false;
		}
		// SAFETY: `__read` returned a `byte[]` of at most `asked` bytes, which `buf` holds.
		unsafe {
			let read = ((**env).v1_2.GetArrayLength)(env, array).clamp(0, asked as jint);
			((**env).v1_2.GetByteArrayRegion)(env, array, 0, read, buf.cast());
			n.write(read as usize);
		}
		true
	})
}

/// Seek a Kotlin channel, from any thread.
unsafe extern "C" fn seek_channel(data: *mut c_void, whence: std::ffi::c_int, offset: i64, position: *mut u64, err: *mut rt::Buf) -> bool {
	upcall("SeekableByteChannel.position", |env| {
		let id = method(env, &SEEK, c"__seek", c"(Ljava/lang/Object;IJ)J");
		let args = [jvalue { l: data.cast() }, jvalue { i: whence }, jvalue { j: offset }];
		// SAFETY: `__seek` takes the channel, where to seek from, and how far.
		let at = unsafe { ((**env).v1_2.CallStaticLongMethodA)(env, class(), id, args.as_ptr()) };
		if let Some(error) = caught(env, "SeekableByteChannel.position", &IO_ERROR, c"__io_error") {
			// SAFETY: C's contract: `err` is writable.
			unsafe { err.write(rt::Buf::vec(error)) };
			return false;
		}
		// SAFETY: C's contract: `position` is writable. Its bits, unsigned, as `__seek` returns
		// a position past `Long.MAX_VALUE`.
		unsafe { position.write(at.cast_unsigned()) };
		true
	})
}

/// The error of the exception a closure threw, cleared, if it is the binding's own
/// exception of a `String`, as a closure returning a `Result` fails.
pub fn thrown(env: *mut JNIEnv, what: &str) -> Option<Vec<u8>> {
	caught(env, what, &ERROR, c"__error")
}

/// The error of the exception pending, cleared, if the Kotlin function `name` takes it
/// as one Rust returns; none if nothing threw. Any other aborts, as [`upcall`] does.
fn caught(env: *mut JNIEnv, what: &str, cache: &OnceLock<usize>, name: &CStr) -> Option<Vec<u8>> {
	// SAFETY: `env` is this thread's, and `thrown` a live reference.
	unsafe {
		let thrown = ((**env).v1_2.ExceptionOccurred)(env);
		if thrown.is_null() {
			return None;
		}
		((**env).v1_2.ExceptionClear)(env);
		let id = method(env, cache, name, c"(Ljava/lang/Throwable;)[B");
		let args = [jvalue { l: thrown }];
		let error = ((**env).v1_2.CallStaticObjectMethodA)(env, class(), id, args.as_ptr());
		if error.is_null() && !((**env).v1_2.ExceptionCheck)(env) {
			((**env).v1_2.Throw)(env, thrown);
		}
		if ((**env).v1_2.ExceptionCheck)(env) {
			fail(env, what, "a call into Kotlin threw");
		}
		bytes(env, error)
	}
}

static FAILED: OnceLock<usize> = OnceLock::new();

/// Raise the error a function returned, encoded in `w`, as the `Native.Failed` Kotlin turns
/// into its exception: a call's success path then returns its value with no `byte[]`.
pub fn raise(env: *mut JNIEnv, w: Writer) {
	let id = method(env, &FAILED, c"__failed", c"([B)Ljava/lang/Throwable;");
	let args = [jvalue { l: w.array(env) }];
	// SAFETY: `env` is this thread's, and `__failed` takes the error's bytes.
	unsafe {
		let failed = ((**env).v1_2.CallStaticObjectMethodA)(env, class(), id, args.as_ptr());
		if failed.is_null() || ((**env).v1_2.ExceptionCheck)(env) {
			fail(env, "jni", "a call into Kotlin threw");
		}
		((**env).v1_2.Throw)(env, failed);
	}
}

/// A Kotlin `Waker` passed by value: Rust holds its only reference.
pub fn waker(env: *mut JNIEnv, waker: jobject) -> rt::Waker {
	lend(node(env, waker))
}

/// A waker a function returned, boxed for Kotlin to hold as a `long`.
pub fn returned(waker: rt::Waker) -> jlong {
	Box::into_raw(Box::new(waker)) as usize as jlong
}

/// Wake a waker [`returned`] boxed.
///
/// # Safety
/// `raw` is one [`returned`] made, not yet dropped.
pub unsafe fn wake_returned(raw: jlong) {
	// SAFETY: the caller's contract.
	unsafe { (*(raw as usize as *const rt::Waker)).wake() };
}

/// Drop a waker [`returned`] boxed.
///
/// # Safety
/// `raw` is one [`returned`] made, dropped only here.
pub unsafe fn drop_returned(raw: jlong) {
	// SAFETY: the caller's contract.
	unsafe { Box::from_raw(raw as usize as *mut rt::Waker).release() };
}

/// Encoded values, read in the order they were written, with the arrays read, which the
/// arrays' views point into until the reader drops.
pub struct Reader<'a> {
	bytes: &'a [u8],
	arrays: Vec<Box<dyn std::any::Any>>,
	/// The call's thread, and the arrays and text the encoding names by place.
	env: *mut JNIEnv,
	refs: jobjectArray,
	/// Whether the value read is one Rust takes, so Rust's own bytes in it, outside an
	/// array, cross as a second owner of them rather than a view.
	take: bool,
}

impl<'a> Reader<'a> {
	/// A reader of a value Rust only reads, as through a pointer, whose encoding names
	/// the objects of `refs` by place.
	pub fn new(env: *mut JNIEnv, bytes: &'a [u8], refs: jobjectArray) -> Self {
		Self {
			bytes,
			arrays: Vec::new(),
			env,
			refs,
			take: false,
		}
	}

	/// A reader of a value passed by value, which Rust takes.
	pub fn taking(env: *mut JNIEnv, bytes: &'a [u8], refs: jobjectArray) -> Self {
		Self {
			take: true,
			..Self::new(env, bytes, refs)
		}
	}

	/// Bytes made once by `make` from the object at `at` of `refs`: taken where the reader
	/// takes, else kept until it drops and read in place.
	fn referred(&mut self, at: i32, make: impl FnOnce(*mut JNIEnv, jobject) -> Vec<u8>) -> rt::Buf {
		let env = self.env;
		if self.refs.is_null() {
			fail(env, "jni", "an encoded value names an object, but none were passed");
		}
		// SAFETY: `refs` is the live `Object[]` the call was given; JNI throws past its end.
		let object = unsafe { ((**env).v1_2.GetObjectArrayElement)(env, self.refs, at) };
		// SAFETY: as above.
		if object.is_null() || unsafe { ((**env).v1_2.ExceptionCheck)(env) } {
			fail(env, "jni", "an encoded value names no object");
		}
		let bytes = make(env, object);
		// SAFETY: a local reference made here.
		unsafe { ((**env).v1_2.DeleteLocalRef)(env, object) };
		if self.take {
			return rt::Buf::vec(bytes);
		}
		let view = rt::Buf::view(&bytes);
		// The vector's heap block, which the view reads, stays put as the box moves.
		self.arrays.push(Box::new(bytes));
		view
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
	/// The native codec buffer, kept between calls and lent only while a return factory runs.
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

	/// Construct a Kotlin value while this writer's native buffer is alive. The generated
	/// factory reads it completely and retains neither the buffer nor its reader.
	pub fn value(self, env: *mut JNIEnv, cache: &OnceLock<usize>, name: &CStr, back: jobject) -> jobject {
		let id = method(env, cache, name, c"(Ljava/nio/ByteBuffer;[Ljava/lang/Object;)Ljava/lang/Object;");
		let len = i32::try_from(self.0.len()).unwrap_or_else(|_| rt::fail("jni", "a value is too large for a ByteBuffer"));
		// SAFETY: this writer owns the allocation through the synchronous factory call.
		// Even an empty Vec has a non-null aligned pointer. Kotlin cannot retain the view.
		unsafe {
			let buffer = ((**env).v1_4.NewDirectByteBuffer)(env, self.0.as_ptr() as *mut c_void, len as jlong);
			if buffer.is_null() {
				fail(env, "jni", "cannot make a return buffer");
			}
			let args = [jvalue { l: buffer }, jvalue { l: back }];
			let result = ((**env).v1_2.CallStaticObjectMethodA)(env, class(), id, args.as_ptr());
			// A declared Rust error is thrown by the factory, after applying writebacks.
			// Leave it pending for the JVM, including a null success such as Option::None.
			((**env).v1_2.DeleteLocalRef)(env, buffer);
			result
		}
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

impl<const N: usize> Wire for [u8; N] {
	fn read(r: &mut Reader<'_>) -> Self {
		r.take(N).try_into().expect("taken")
	}

	fn write(self, w: &mut Writer) {
		w.0.extend_from_slice(&self);
	}
}

impl Wire for bool {
	fn read(r: &mut Reader<'_>) -> Self {
		r.get::<u8>() != 0
	}

	fn write(self, w: &mut Writer) {
		w.put(u8::from(self));
	}
}

/// What crosses as 64 bits, at the target's width: on a 32-bit target, a value past it
/// has no `usize` or `isize` to be.
pub fn narrow<T: TryFrom<W>, W>(wide: W) -> T {
	match T::try_from(wide) {
		Ok(v) => v,
		Err(_) => rt::fail("jni", "a value is too wide for this target's usize or isize"),
	}
}

impl Wire for usize {
	fn read(r: &mut Reader<'_>) -> Self {
		narrow(r.get::<u64>())
	}

	fn write(self, w: &mut Writer) {
		w.put(self as u64);
	}
}

impl Wire for isize {
	fn read(r: &mut Reader<'_>) -> Self {
		narrow(r.get::<i64>())
	}

	fn write(self, w: &mut Writer) {
		w.put(self as i64);
	}
}

impl Wire for *const c_void {
	fn read(r: &mut Reader<'_>) -> Self {
		narrow::<usize, _>(r.get::<u64>()) as Self
	}

	fn write(self, w: &mut Writer) {
		w.put(self as usize as u64);
	}
}

impl Wire for *mut c_void {
	fn read(r: &mut Reader<'_>) -> Self {
		narrow::<usize, _>(r.get::<u64>()) as Self
	}

	fn write(self, w: &mut Writer) {
		w.put(self as usize as u64);
	}
}

/// A returned waker, as [`returned`] boxes it.
impl Wire for rt::Waker {
	fn read(r: &mut Reader<'_>) -> Self {
		// SAFETY: `write` boxed it, and Kotlin read it back once.
		*unsafe { Box::from_raw(narrow::<usize, _>(r.get::<u64>()) as *mut rt::Waker) }
	}

	fn write(self, w: &mut Writer) {
		w.put(returned(self));
	}
}

impl Wire for rt::Buf {
	/// A view of the encoded bytes, or after [`LENT`] of those Rust lent Kotlin, in place,
	/// either of which outlives the call.
	fn read(r: &mut Reader<'_>) -> Self {
		match r.get::<i32>() {
			LENT => {
				let owner = r.get::<i64>();
				let (start, len) = (r.get::<i32>(), r.get::<i32>());
				// SAFETY: an owner Kotlin keeps from being freed until the call returns.
				let bytes = unsafe { range(owner, start, len) };
				let view = rt::Buf::view(bytes);
				match r.take {
					false => view,
					// SAFETY: as above.
					true => unsafe {
						rt::Buf {
							owner: owner as usize as _,
							..view
						}
						.share("jni")
					},
				}
			}
			KOTLIN => {
				let at = r.get::<i32>();
				let (start, len) = (r.get::<i32>(), r.get::<i32>());
				r.referred(at, |env, array| region(env, array, start, len))
			}
			TEXT => {
				let at = r.get::<i32>();
				r.referred(at, utf8)
			}
			len => match usize::try_from(len) {
				Ok(len) => rt::Buf::view(r.take(len)),
				Err(_) => rt::Buf::NONE,
			},
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

/// What precedes the place in a call's objects of Kotlin's `byte[]`, then where its bytes
/// start and how many, where an encoded buffer's length would.
const KOTLIN: i32 = -3;

/// What precedes the place in a call's objects of Kotlin's `String` where an encoded
/// buffer's length would.
const TEXT: i32 = -4;

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
		// Rust only reads an array's items.
		let take = std::mem::replace(&mut r.take, false);
		let items: Box<[T]> = (0..len).map(|_| r.get()).collect();
		r.take = take;
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
