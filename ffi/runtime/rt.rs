//! The runtime every export shares: buffers, handles, lent functions, wakers, and futures
//! across the C ABI. rust2-ffi copies it beside the generated code.
//!
//! A handle borrowed or moved in a way Rust's rules forbid fails the call with a
//! [`Status`], before any Rust code runs, which a binding raises as its language's own
//! error. Any other broken contract across the ABI (a null handle, invalid UTF-8, a future
//! polled after it finished) aborts with a message naming the C function: no binding
//! makes one, and there is no Rust caller to return an error to.

use std::cell::UnsafeCell;
use std::ffi::c_void;
use std::mem::MaybeUninit;
use std::ops::{Deref, DerefMut};
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{self, AtomicBool, AtomicUsize, Ordering};
use std::task::{Context, Wake};

#[cfg(target_os = "android")]
mod android;
mod borrow;
use borrow::{Borrow, EXCLUSIVE, RESERVED, TAKEN, Table};
mod thread;
pub use thread::Thread;
use thread::Dropped;

/// `value`'s `Hash`, through std's hasher with its fixed keys, so a value hashes alike on
/// every call.
pub fn hash<T: std::hash::Hash + ?Sized>(value: &T) -> u64 {
	std::hash::BuildHasher::hash_one(&std::hash::BuildHasherDefault::<std::hash::DefaultHasher>::default(), value)
}

/// Abort with a message naming the C function whose contract was broken.
#[cold]
pub fn fail(what: &str, msg: &str) -> ! {
	eprintln!("{what}: {msg}");
	#[cfg(target_os = "android")]
	android::report(&format!("{what}: {msg}"));
	std::process::abort()
}

/// Why a call's check of a handle failed: a borrow or move Rust's borrow checker would
/// refuse. The call returns before any Rust code runs, so nothing changed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum Status {
	/// The handle's value was moved out, or the handle closed.
	Moved = 1,
	/// Another call or future borrows the value exclusively.
	Exclusive = 2,
	/// Another call or future borrows the value, so it cannot be borrowed exclusively or
	/// moved out.
	Borrowed = 3,
	/// A value that is not `Sync` is borrowed on another thread.
	Thread = 4,
	/// A handle a call lent is borrowed exclusively or moved out.
	Lent = 5,
	/// A handle lent from another outlived it.
	Outlived = 6,
	/// A value that is not `Send` is used on another thread than the one that made it.
	NotSend = 7,
}

impl Status {
	/// Every status, by its number less one.
	pub const ALL: [Self; 7] = [
		Self::Moved,
		Self::Exclusive,
		Self::Borrowed,
		Self::Thread,
		Self::Lent,
		Self::Outlived,
		Self::NotSend,
	];

	/// What broke, as a binding's error says it after the function's name.
	pub fn message(self) -> &'static str {
		self.c_message().to_str().expect("ASCII")
	}

	fn c_message(self) -> &'static std::ffi::CStr {
		match self {
			Self::Moved => c"a handle's value was moved out",
			Self::Exclusive => c"a handle is borrowed exclusively",
			Self::Borrowed => c"a handle is already borrowed",
			Self::Thread => c"a handle is borrowed on another thread",
			Self::Lent => c"a handle a call lent is borrowed exclusively or moved out",
			Self::Outlived => {
				c"a handle lent from another outlived it: the other was borrowed exclusively, moved out, or freed"
			}
			Self::NotSend => c"a value that is not Send is used on another thread than the one that made it",
		}
	}

	/// The message of the status numbered `n`, for `<prefix>_status_message`.
	pub fn text(n: i32) -> &'static std::ffi::CStr {
		Self::of(n).map_or(c"an unknown status", Self::c_message)
	}

	/// The status numbered `n`, as a function wrote it.
	pub fn of(n: i32) -> Option<Self> {
		Self::ALL.get(usize::try_from(n).ok()?.checked_sub(1)?).copied()
	}

	/// Abort, naming the function `what`, where no caller can take the status.
	#[cold]
	pub fn fail(self, what: &str) -> ! {
		fail(what, self.message())
	}
}

/// The value of a check, or an abort naming `what` where no caller can take its status.
pub fn check<T>(checked: Result<T, Status>, what: &str) -> T {
	checked.unwrap_or_else(|status| status.fail(what))
}

/// Fail a C function with `status`: write it through the function's `out`, returning a
/// zeroed value the caller ignores, or abort naming `what` if `out` is null, as a C caller
/// that takes no status passes.
///
/// # Safety
/// `out` is null or points where an `int32_t` goes, and a zeroed `R` is a valid value.
#[cold]
pub unsafe fn failed<R>(out: *mut i32, status: Status, what: &str) -> R {
	if out.is_null() {
		status.fail(what);
	}
	// SAFETY: the caller's contract.
	unsafe {
		out.write(status as i32);
		std::mem::zeroed()
	}
}

/// `s` for as long as the program runs, for a parameter that must be `'static`: each
/// distinct string is leaked once and lent to every later call with equal content, so the
/// strings held grow only with the distinct ones passed.
pub fn intern(s: &str) -> &'static str {
	static INTERNED: std::sync::Mutex<std::collections::BTreeSet<&'static str>> =
		std::sync::Mutex::new(std::collections::BTreeSet::new());
	let mut interned = INTERNED.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
	if let Some(&s) = interned.get(s) {
		return s;
	}
	let s: &'static str = Box::leak(Box::from(s));
	interned.insert(s);
	s
}

/// `b` for as long as the program runs, as [`intern`] keeps a string: keyed by its
/// contents, never by the caller's pointer, which the caller may free or reuse.
pub fn intern_bytes(b: &[u8]) -> &'static [u8] {
	static INTERNED: std::sync::Mutex<std::collections::BTreeSet<&'static [u8]>> =
		std::sync::Mutex::new(std::collections::BTreeSet::new());
	let mut interned = INTERNED.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
	if let Some(&b) = interned.get(b) {
		return b;
	}
	let b: &'static [u8] = Box::leak(Box::from(b));
	interned.insert(b);
	b
}

/// An `Arc` of another crate's type, held erased behind a handle with none of its API, so
/// the export names neither the type nor its crate. Rust takes it back as the `Arc` it
/// gave, and a handle of another such type aborts. A clone clones the `Arc` itself, so
/// Rust counts each handle as an owner.
pub struct Opaque(Box<dyn Held>);

/// An `Arc<T>` of any `T`, a trait object's too, erased.
trait Held: Send + Sync {
	fn any(&self) -> &dyn std::any::Any;
	fn clone_box(&self) -> Box<dyn Held>;
}

impl<T: ?Sized + Send + Sync + 'static> Held for Arc<T> {
	fn any(&self) -> &dyn std::any::Any {
		self
	}

	fn clone_box(&self) -> Box<dyn Held> {
		Box::new(Arc::clone(self))
	}
}

impl Clone for Opaque {
	fn clone(&self) -> Self {
		Self(self.0.clone_box())
	}
}

impl Opaque {
	pub fn new<T: ?Sized + Send + Sync + 'static>(value: Arc<T>) -> Self {
		Self(Box::new(value))
	}

	/// The `Arc` it holds, lent.
	pub fn get<T: ?Sized + Send + Sync + 'static>(&self, what: &str) -> &Arc<T> {
		(self.0.any().downcast_ref())
			.unwrap_or_else(|| fail(what, "an opaque handle holds another type"))
	}

	/// A clone of the `Arc` it holds.
	pub fn arc<T: ?Sized + Send + Sync + 'static>(&self, what: &str) -> Arc<T> {
		Arc::clone(self.get(what))
	}
}

/// An argument a future borrows, kept in place on the heap until the future is dropped.
pub struct Pinned<T>(std::ptr::NonNull<T>);

// It owns its `T`, as a `Box` does.
unsafe impl<T: Send> Send for Pinned<T> {}
unsafe impl<T: Sync> Sync for Pinned<T> {}

impl<T> Pinned<T> {
	pub fn new(value: T) -> Self {
		Self(std::ptr::NonNull::from(Box::leak(Box::new(value))))
	}

	/// The value, lent past this borrow of its holder.
	///
	/// # Safety
	///
	/// Called once, and the borrow ends before the holder drops, as [`Holding`] ensures.
	pub unsafe fn get(&self) -> &'static mut T {
		unsafe { &mut *self.0.as_ptr() }
	}
}

impl<T> Drop for Pinned<T> {
	fn drop(&mut self) {
		drop(unsafe { Box::from_raw(self.0.as_ptr()) });
	}
}

/// A future and the arguments it borrows, which drop after it: fields drop in order.
pub struct Holding<F, H> {
	future: F,
	_held: H,
}

impl<F, H> Holding<F, H> {
	pub fn new(future: F, held: H) -> Self {
		Self { future, _held: held }
	}
}

impl<F: std::future::Future, H> std::future::Future for Holding<F, H> {
	type Output = F::Output;

	fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<F::Output> {
		// The future is pinned with its holder and never moves out; the held values are
		// never pinned.
		unsafe { self.map_unchecked_mut(|holding| &mut holding.future) }.poll(cx)
	}
}

/// Another crate's value, held erased by its handle, which the glue never names.
pub struct Owned(Box<dyn std::any::Any + Send + Sync>);

impl Owned {
	pub fn new<T: Send + Sync + 'static>(value: T) -> Self {
		Self(Box::new(value))
	}

	/// The value it holds, lent.
	pub fn get<T: 'static>(&self, what: &str) -> &T {
		(self.0.downcast_ref()).unwrap_or_else(|| fail(what, "an opaque handle holds another type"))
	}

	/// The value it holds, lent exclusively.
	pub fn get_mut<T: 'static>(&mut self, what: &str) -> &mut T {
		(self.0.downcast_mut()).unwrap_or_else(|| fail(what, "an opaque handle holds another type"))
	}

	/// The value it holds, taken back.
	pub fn take<T: 'static>(self, what: &str) -> T {
		*(self.0.downcast()).unwrap_or_else(|_| fail(what, "an opaque handle holds another type"))
	}
}

/// Bytes across the ABI. Returned, an owned buffer the caller frees once. Passed in by
/// value, one with an owner is Rust's own, which it takes without a copy, and one without
/// is a view it copies or borrows for the call; behind a pointer, as an array's items are,
/// it is only read. A null `ptr` is `None`.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Buf {
	pub ptr: *const u8,
	pub len: usize,
	pub owner: *mut c_void,
}

impl Buf {
	pub const NONE: Self = Self {
		ptr: std::ptr::null(),
		len: 0,
		owner: std::ptr::null_mut(),
	};

	pub fn is_none(&self) -> bool {
		self.ptr.is_null()
	}

	/// The bytes a caller passed in, which it keeps, borrowed no longer than the buffer
	/// is, so nothing kept past the call can hold them.
	///
	/// # Safety
	/// `ptr` points at `len` readable bytes while `self` lives.
	pub unsafe fn bytes(&self, what: &str) -> &[u8] {
		match (self.ptr.is_null(), self.len) {
			(_, 0) => &[],
			(true, _) => fail(what, "a buffer has a length but no pointer"),
			// SAFETY: the caller's contract.
			(false, len) => unsafe { std::slice::from_raw_parts(self.ptr, len) },
		}
	}

	/// The bytes a caller passed in: taken without a copy where `take` and it has an
	/// owner, else copied, leaving the caller's buffer as it was.
	///
	/// # Safety
	/// As [`Self::bytes`], and an owner, where `take`, came from [`Self::owned`] and is
	/// freed by nothing else.
	pub unsafe fn bytes_in(self, what: &str, take: bool) -> bytes::Bytes {
		match take && !self.owner.is_null() {
			// SAFETY: the caller's contract.
			true => unsafe { self.take(what) },
			// SAFETY: the caller's contract.
			false => bytes::Bytes::copy_from_slice(unsafe { self.bytes(what) }),
		}
	}

	/// As [`Self::bytes_in`], as a `Vec`, which reuses an owner's allocation it alone holds.
	///
	/// # Safety
	/// As [`Self::bytes_in`].
	pub unsafe fn vec_in(self, what: &str, take: bool) -> Vec<u8> {
		match take && !self.owner.is_null() {
			// SAFETY: the caller's contract.
			true => Vec::from(unsafe { self.take(what) }),
			// SAFETY: the caller's contract.
			false => unsafe { self.bytes(what) }.to_vec(),
		}
	}

	/// As [`Self::vec_in`], checked to be UTF-8.
	///
	/// # Safety
	/// As [`Self::bytes_in`].
	pub unsafe fn string_in(self, what: &str, take: bool) -> String {
		// SAFETY: the caller's contract.
		match String::from_utf8(unsafe { self.vec_in(what, take) }) {
			Ok(s) => s,
			Err(_) => fail(what, "a string is not UTF-8"),
		}
	}

	/// As [`Self::vec_in`], as a path: its bytes as the OS holds them on Unix and WASI, WTF-8 on
	/// Windows, else UTF-8.
	///
	/// # Safety
	/// As [`Self::bytes_in`].
	pub unsafe fn path_in(self, what: &str, take: bool) -> std::path::PathBuf {
		// SAFETY: the caller's contract.
		let bytes = unsafe { self.vec_in(what, take) };
		#[cfg(any(unix, target_os = "wasi"))]
		{
			use bytes_os::OsStringExt;
			std::path::PathBuf::from(std::ffi::OsString::from_vec(bytes))
		}
		#[cfg(not(any(unix, target_os = "wasi")))]
		match String::from_utf8(bytes) {
			Ok(s) => std::path::PathBuf::from(s),
			Err(err) => decoded(err.as_bytes(), what),
		}
	}

	/// A path a caller passed in for the call alone, as [`Self::hold`] holds its bytes.
	///
	/// # Safety
	/// As [`Self::hold`].
	pub unsafe fn hold_path<'a>(self, what: &str) -> HeldPath<'a> {
		// SAFETY: the caller's contract.
		let held = unsafe { self.hold(what) };
		#[cfg(not(any(unix, target_os = "wasi")))]
		if std::str::from_utf8(held.bytes()).is_err() {
			return HeldPath::Decoded(decoded(held.bytes(), what));
		}
		HeldPath::Held(held)
	}

	/// The bytes a caller passed in for the call alone: taken where it has an owner, which
	/// they free after, else borrowed.
	///
	/// # Safety
	/// As [`Self::bytes_in`] taking.
	pub unsafe fn hold<'a>(self, what: &str) -> Hold<'a> {
		match self.owner.is_null() {
			// SAFETY: the caller's contract; a view lives as long as the call.
			true => Hold::Lent(unsafe { std::slice::from_raw_parts(self.bytes(what).as_ptr(), self.len) }),
			// SAFETY: the caller's contract.
			false => Hold::Owned(unsafe { self.take(what) }),
		}
	}

	/// The bytes an owned buffer holds, taken from its owner: `ptr` and `len` may be any
	/// range of them, as a caller slices a buffer Rust returned.
	///
	/// # Safety
	/// `owner` came from [`Self::owned`], and is freed by nothing else.
	unsafe fn take(self, what: &str) -> bytes::Bytes {
		// SAFETY: the caller's contract.
		let bytes = *unsafe { Box::from_raw(self.owner.cast::<bytes::Bytes>()) };
		let start = (self.ptr as usize).wrapping_sub(bytes.as_ptr() as usize);
		match self.len {
			0 => bytes::Bytes::new(),
			// Whole, as is: a slice of bytes made from a `Vec` would allocate to share them.
			len if start == 0 && len == bytes.len() => bytes,
			_ if start <= bytes.len() && self.len <= bytes.len() - start => bytes.slice(start..start + self.len),
			_ => fail(what, "a buffer's bytes are not its owner's"),
		}
	}

	/// A second owner of an owned buffer's bytes, sharing them: what a caller passes Rust
	/// while it keeps the buffer.
	///
	/// # Safety
	/// `owner` came from [`Self::owned`], and is not yet freed.
	pub unsafe fn share(self, what: &str) -> Self {
		if self.owner.is_null() {
			fail(what, "a buffer has no owner to share");
		}
		// SAFETY: the caller's contract.
		let bytes = unsafe { &*self.owner.cast::<bytes::Bytes>() };
		Self {
			owner: Box::into_raw(Box::new(bytes.clone())).cast(),
			..self
		}
	}

	/// The string a caller passed in, which it keeps.
	///
	/// # Safety
	/// As [`Self::bytes`].
	pub unsafe fn str(&self, what: &str) -> &str {
		// SAFETY: the caller's contract.
		match std::str::from_utf8(unsafe { self.bytes(what) }) {
			Ok(s) => s,
			Err(_) => fail(what, "a string is not UTF-8"),
		}
	}

	/// A view of bytes Rust keeps, for a callback to read during its call.
	pub fn view(bytes: &[u8]) -> Self {
		Self {
			ptr: bytes.as_ptr(),
			len: bytes.len(),
			owner: std::ptr::null_mut(),
		}
	}

	/// Bytes handed to the caller, which frees them.
	pub fn owned(bytes: bytes::Bytes) -> Self {
		let ptr = bytes.as_ptr();
		let len = bytes.len();
		let owner = Box::into_raw(Box::new(bytes)).cast();
		Self { ptr, len, owner }
	}

	pub fn string(string: String) -> Self {
		Self::owned(string.into())
	}

	#[cfg_attr(not(feature = "web"), allow(unused_mut))]
	pub fn vec(mut vec: Vec<u8>) -> Self {
		// The web binding budgets the bytes it holds lent by their length, so their buffer
		// keeps no spare capacity; wasi-libc's allocator shrinks it in place.
		#[cfg(feature = "web")]
		vec.shrink_to_fit();
		Self::owned(vec.into())
	}

	/// A path handed to the caller: its bytes as the OS holds them on Unix and WASI, else as
	/// Rust encodes them: WTF-8 on Windows, which is UTF-8 for any path that has a UTF-8
	/// form.
	pub fn path_buf(path: std::path::PathBuf) -> Self {
		Self::vec(path.into_os_string().into_encoded_bytes())
	}

	/// The buffer `owner` holds, as [`Self::owned`] returned it.
	///
	/// # Safety
	/// `owner` came from [`Self::owned`], and is not yet freed.
	pub unsafe fn of_owner(owner: *mut c_void) -> Self {
		// SAFETY: the caller's contract.
		let bytes = unsafe { &*owner.cast::<bytes::Bytes>() };
		Self {
			ptr: bytes.as_ptr(),
			len: bytes.len(),
			owner,
		}
	}

	/// The string in a buffer C returned Rust, as `<prefix>_buf_copy` made it, which Rust
	/// now owns.
	///
	/// # Safety
	/// `owner` is null or came from [`Self::owned`], and is not yet freed.
	pub unsafe fn take_string(self, what: &str) -> String {
		if self.owner.is_null() {
			fail(what, "a returned buffer is not one buf_copy made");
		}
		// SAFETY: the caller's contract.
		unsafe { self.string_in(what, true) }
	}

	/// Free a buffer Rust returned; a view or `None` frees nothing.
	///
	/// # Safety
	/// `owner` is null or came from [`Self::owned`], and is freed once.
	pub unsafe fn free(self) {
		if !self.owner.is_null() {
			// SAFETY: the caller's contract.
			drop(unsafe { Box::from_raw(self.owner.cast::<bytes::Bytes>()) });
		}
	}
}

/// Bytes passed in for a call: a view borrowed for it, or a buffer Rust took, freed after.
pub enum Hold<'a> {
	Lent(&'a [u8]),
	Owned(bytes::Bytes),
}

impl Hold<'_> {
	pub fn bytes(&self) -> &[u8] {
		match self {
			Self::Lent(bytes) => bytes,
			Self::Owned(bytes) => bytes,
		}
	}

	pub fn str(&self, what: &str) -> &str {
		match std::str::from_utf8(self.bytes()) {
			Ok(s) => s,
			Err(_) => fail(what, "a string is not UTF-8"),
		}
	}
}

/// The `OsStr` extensions of a platform whose paths are any bytes, as Unix's and WASI's are.
#[cfg(unix)]
use std::os::unix::ffi as bytes_os;
#[cfg(target_os = "wasi")]
use std::os::wasi::ffi as bytes_os;

/// A path passed in for a call: the bytes [`Hold`] holds, or on a platform whose paths
/// are not bytes, what they decode to where they are not UTF-8.
pub enum HeldPath<'a> {
	Held(Hold<'a>),
	#[cfg(not(any(unix, target_os = "wasi")))]
	Decoded(std::path::PathBuf),
}

impl HeldPath<'_> {
	pub fn path(&self) -> &std::path::Path {
		match self {
			#[cfg(any(unix, target_os = "wasi"))]
			Self::Held(held) => {
				use bytes_os::OsStrExt;
				std::path::Path::new(std::ffi::OsStr::from_bytes(held.bytes()))
			}
			// UTF-8, or else decoded.
			#[cfg(not(any(unix, target_os = "wasi")))]
			Self::Held(held) => std::path::Path::new(std::str::from_utf8(held.bytes()).unwrap_or_default()),
			#[cfg(not(any(unix, target_os = "wasi")))]
			Self::Decoded(path) => path,
		}
	}
}

/// The path a caller's bytes encode that are not UTF-8: on Windows, WTF-8, as Rust
/// encodes a name that is not UTF-16, any other aborting; elsewhere, none.
#[cfg(not(any(unix, target_os = "wasi")))]
#[cold]
fn decoded(bytes: &[u8], what: &str) -> std::path::PathBuf {
	#[cfg(windows)]
	if let Some(wide) = wtf8(bytes) {
		use std::os::windows::ffi::OsStringExt;
		return std::ffi::OsString::from_wide(&wide).into();
	}
	let _ = bytes;
	fail(what, "a path is not as this platform encodes one")
}

/// The UTF-16 units of WTF-8 `bytes`, if they are: UTF-8 but for a surrogate alone,
/// encoded on its own as UTF-8 would a scalar, but never a lead and a trail together,
/// which WTF-8 encodes as the scalar they make.
pub fn wtf8(bytes: &[u8]) -> Option<Vec<u16>> {
	let mut wide = Vec::with_capacity(bytes.len());
	let (mut rest, mut lead) = (bytes, false);
	loop {
		let (text, after) = match std::str::from_utf8(rest) {
			Ok(text) => (text, &[][..]),
			Err(err) => {
				let (valid, after) = rest.split_at(err.valid_up_to());
				// SAFETY: UTF-8 up to there.
				(unsafe { std::str::from_utf8_unchecked(valid) }, after)
			}
		};
		if !text.is_empty() {
			wide.extend(text.encode_utf16());
			lead = false;
		}
		let [0xED, b @ 0xA0..=0xBF, c @ 0x80..=0xBF, ..] = *after else {
			return after.is_empty().then_some(wide);
		};
		let unit = 0xD000 | u16::from(b & 0x3F) << 6 | u16::from(c & 0x3F);
		let trail = unit >= 0xDC00;
		if trail && lead {
			return None;
		}
		wide.push(unit);
		lead = !trail;
		rest = &after[3..];
	}
}

/// `u128`, as its two halves.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct U128 {
	pub lo: u64,
	pub hi: u64,
}

impl U128 {
	pub fn lift(self) -> u128 {
		u128::from(self.hi) << 64 | u128::from(self.lo)
	}

	pub fn lower(v: u128) -> Self {
		Self {
			lo: v as u64,
			hi: (v >> 64) as u64,
		}
	}
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Duration {
	pub secs: u64,
	pub nanos: u32,
}

impl Duration {
	pub fn lift(self, what: &str) -> std::time::Duration {
		if self.nanos >= 1_000_000_000 {
			fail(what, "a duration has a billion nanoseconds or more");
		}
		std::time::Duration::new(self.secs, self.nanos)
	}

	pub fn lower(duration: std::time::Duration) -> Self {
		Self {
			secs: duration.as_secs(),
			nanos: duration.subsec_nanos(),
		}
	}
}

/// `std::net::Ipv4Addr`, in network order.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Ipv4Addr {
	pub octets: [u8; 4],
}

impl Ipv4Addr {
	pub fn lift(self) -> std::net::Ipv4Addr {
		self.octets.into()
	}

	pub fn lower(ip: std::net::Ipv4Addr) -> Self {
		Self { octets: ip.octets() }
	}
}

/// `std::net::Ipv6Addr`, in network order.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Ipv6Addr {
	pub octets: [u8; 16],
}

impl Ipv6Addr {
	pub fn lift(self) -> std::net::Ipv6Addr {
		self.octets.into()
	}

	pub fn lower(ip: std::net::Ipv6Addr) -> Self {
		Self { octets: ip.octets() }
	}
}

/// An [`IpAddr`]'s address, of the family its `v6` names.
#[repr(C)]
#[derive(Clone, Copy)]
pub union Ip {
	pub v4: Ipv4Addr,
	pub v6: Ipv6Addr,
}

/// `std::net::IpAddr`: `ip.v6` if `v6`, else `ip.v4`.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IpAddr {
	pub v6: bool,
	pub ip: Ip,
}

impl IpAddr {
	pub fn lift(self) -> std::net::IpAddr {
		// SAFETY: `v6` names the member the caller wrote.
		match self.v6 {
			true => unsafe { self.ip.v6 }.lift().into(),
			false => unsafe { self.ip.v4 }.lift().into(),
		}
	}

	pub fn lower(ip: std::net::IpAddr) -> Self {
		// The other family's bytes are zero, not uninitialized, for a caller copying them.
		let mut c = Self {
			v6: ip.is_ipv6(),
			ip: Ip {
				v6: Ipv6Addr { octets: [0; 16] },
			},
		};
		match ip {
			std::net::IpAddr::V4(v4) => c.ip.v4 = Ipv4Addr::lower(v4),
			std::net::IpAddr::V6(v6) => c.ip.v6 = Ipv6Addr::lower(v6),
		}
		c
	}
}

/// `std::net::SocketAddrV4`, its port in the host's order.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SocketAddrV4 {
	pub ip: Ipv4Addr,
	pub port: u16,
}

impl SocketAddrV4 {
	pub fn lift(self) -> std::net::SocketAddrV4 {
		std::net::SocketAddrV4::new(self.ip.lift(), self.port)
	}

	pub fn lower(addr: std::net::SocketAddrV4) -> Self {
		Self {
			ip: Ipv4Addr::lower(*addr.ip()),
			port: addr.port(),
		}
	}
}

/// `std::net::SocketAddrV6`, its port in the host's order.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SocketAddrV6 {
	pub ip: Ipv6Addr,
	pub port: u16,
	pub flowinfo: u32,
	pub scope_id: u32,
}

impl SocketAddrV6 {
	pub fn lift(self) -> std::net::SocketAddrV6 {
		std::net::SocketAddrV6::new(self.ip.lift(), self.port, self.flowinfo, self.scope_id)
	}

	pub fn lower(addr: std::net::SocketAddrV6) -> Self {
		Self {
			ip: Ipv6Addr::lower(*addr.ip()),
			port: addr.port(),
			flowinfo: addr.flowinfo(),
			scope_id: addr.scope_id(),
		}
	}
}

/// A [`SocketAddr`]'s address, of the family its `v6` names.
#[repr(C)]
#[derive(Clone, Copy)]
pub union Socket {
	pub v4: SocketAddrV4,
	pub v6: SocketAddrV6,
}

/// `std::net::SocketAddr`: `addr.v6` if `v6`, else `addr.v4`.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SocketAddr {
	pub v6: bool,
	pub addr: Socket,
}

impl SocketAddr {
	pub fn lift(self) -> std::net::SocketAddr {
		// SAFETY: `v6` names the member the caller wrote.
		match self.v6 {
			true => unsafe { self.addr.v6 }.lift().into(),
			false => unsafe { self.addr.v4 }.lift().into(),
		}
	}

	pub fn lower(addr: std::net::SocketAddr) -> Self {
		// The other family's bytes are zero, not uninitialized, for a caller copying them.
		let mut c = Self {
			v6: addr.is_ipv6(),
			addr: Socket {
				v6: SocketAddrV6::lower(std::net::SocketAddrV6::new(std::net::Ipv6Addr::UNSPECIFIED, 0, 0, 0)),
			},
		};
		match addr {
			std::net::SocketAddr::V4(v4) => c.addr.v4 = SocketAddrV4::lower(v4),
			std::net::SocketAddr::V6(v6) => c.addr.v6 = SocketAddrV6::lower(v6),
		}
		c
	}
}

/// `Vec<T>` across the ABI, as an array of its elements' C values. Passed in, a view Rust
/// copies each element out of and never frees. Returned, an array the caller frees once,
/// with what its first `len` elements hold. A null `ptr` is `None`.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct List<T> {
	pub ptr: *const T,
	pub len: usize,
	pub owner: *mut c_void,
}

impl<T: Copy> List<T> {
	pub const NONE: Self = Self {
		ptr: std::ptr::null(),
		len: 0,
		owner: std::ptr::null_mut(),
	};

	pub fn is_none(&self) -> bool {
		self.ptr.is_null()
	}

	/// The elements a caller passed in, which it keeps.
	///
	/// # Safety
	/// `ptr` points at `len` readable elements while `self` lives.
	pub unsafe fn items(&self, what: &str) -> &[T] {
		match (self.ptr.is_null(), self.len) {
			(_, 0) => &[],
			(true, _) => fail(what, "an array has a length but no pointer"),
			// SAFETY: the caller's contract.
			(false, len) => unsafe { std::slice::from_raw_parts(self.ptr, len) },
		}
	}

	/// Elements handed to the caller, which frees them.
	pub fn owned(items: Vec<T>) -> Self {
		let items = Box::new(items);
		Self {
			ptr: items.as_ptr(),
			len: items.len(),
			owner: Box::into_raw(items).cast(),
		}
	}

	/// Free an array Rust returned, after `each` frees what each of its first `len`
	/// elements holds: a binding that took the elements frees the array with `len` 0.
	///
	/// # Safety
	/// `owner` is null or came from [`Self::owned`], and is freed once.
	pub unsafe fn free(self, what: &str, each: impl Fn(T)) {
		if self.owner.is_null() {
			return;
		}
		// SAFETY: the caller's contract.
		let items = unsafe { Box::from_raw(self.owner.cast::<Vec<T>>()) };
		if !std::ptr::eq(self.ptr, items.as_ptr()) || self.len > items.len() {
			fail(what, "an array is not as Rust returned it");
		}
		items[..self.len].iter().copied().for_each(each);
	}
}

/// `std::ops::Range`.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Range<T> {
	pub start: T,
	pub end: T,
}

impl<T> Range<T> {
	pub fn lift(self) -> std::ops::Range<T> {
		self.start..self.end
	}

	pub fn lower(range: std::ops::Range<T>) -> Self {
		Self {
			start: range.start,
			end: range.end,
		}
	}
}

/// `std::ops::Bound`: its kind, and the value of an included or excluded bound.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Bound<T> {
	pub kind: i32,
	pub value: T,
}

const UNBOUNDED: i32 = 0;
const INCLUDED: i32 = 1;
const EXCLUDED: i32 = 2;

impl<T: Default> Bound<T> {
	pub fn lift(self, what: &str) -> std::ops::Bound<T> {
		match self.kind {
			UNBOUNDED => std::ops::Bound::Unbounded,
			INCLUDED => std::ops::Bound::Included(self.value),
			EXCLUDED => std::ops::Bound::Excluded(self.value),
			_ => fail(what, "a bound's kind is invalid"),
		}
	}

	pub fn lower(bound: std::ops::Bound<T>) -> Self {
		let (kind, value) = match bound {
			std::ops::Bound::Unbounded => (UNBOUNDED, T::default()),
			std::ops::Bound::Included(value) => (INCLUDED, value),
			std::ops::Bound::Excluded(value) => (EXCLUDED, value),
		};
		Self { kind, value }
	}
}

/// `(Bound<T>, Bound<T>)`, the form a `RangeBounds<T>` parameter takes.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Bounds<T> {
	pub start: Bound<T>,
	pub end: Bound<T>,
}

impl<T: Default> Bounds<T> {
	pub fn lift(self, what: &str) -> (std::ops::Bound<T>, std::ops::Bound<T>) {
		(self.start.lift(what), self.end.lift(what))
	}

	pub fn lower((start, end): (std::ops::Bound<T>, std::ops::Bound<T>)) -> Self {
		Self {
			start: Bound::lower(start),
			end: Bound::lower(end),
		}
	}
}

pub fn char(code: u32, what: &str) -> char {
	char::from_u32(code).unwrap_or_else(|| fail(what, "a char is not a Unicode scalar value"))
}

/// Columns of [`TABLE`]: threads borrowing at once before two share one.
const COLUMNS: usize = 64;
/// Rows of [`TABLE`]: handles one thread borrows at once before two share a slot.
const ROWS: usize = 64;

#[repr(align(64))]
struct Slots([AtomicUsize; COLUMNS * ROWS]);

static SLOTS: Slots = Slots([const { AtomicUsize::new(0) }; COLUMNS * ROWS]);
/// Where shared borrows of contended handles go instead of their counts.
static TABLE: Table<'static> = Table::new(&SLOTS.0, ROWS);

/// Wait for another thread to finish a step of a handle's borrow check, which runs no code
/// but the check's: spinning briefly, then yielding in case that thread is descheduled.
fn relax(spins: &mut u32) {
	match *spins < 64 {
		true => std::hint::spin_loop(),
		false => std::thread::yield_now(),
	}
	*spins += 1;
}

/// A [`Ticket`]'s column before a borrow of it asked for one.
const NO_COLUMN: usize = usize::MAX;

/// This thread's column of [`TABLE`]: threads take them in turn, so no two of the first
/// [`COLUMNS`] share one. A thread past its thread locals' teardown takes the first.
fn column() -> usize {
	static NEXT: AtomicUsize = AtomicUsize::new(0);
	thread_local!(static COLUMN: std::cell::Cell<usize> = const { std::cell::Cell::new(usize::MAX) });
	COLUMN
		.try_with(|column| {
			if column.get() == usize::MAX {
				column.set(NEXT.fetch_add(1, Ordering::Relaxed) % COLUMNS);
			}
			column.get()
		})
		.unwrap_or(0)
}

/// A value behind a handle: shared through an `Arc`, and borrowed as Rust borrows it, with
/// the borrow checked at run time by `borrow`, which says how. A value that is not `Sync`
/// is shared on one thread at a time: its state is that thread's id, and `depth` counts its
/// borrows, which only that thread touches.
pub struct Object<T> {
	borrow: Borrow,
	depth: UnsafeCell<usize>,
	/// The value a lent handle points at, where Rust lent it, or null if it owns `value`.
	lent: *mut T,
	value: UnsafeCell<MaybeUninit<T>>,
	/// Bumped each time the value is changed through an exclusive borrow or moved out, or
	/// the handle is freed, so no handle lent from the value outlives any of them.
	epoch: AtomicUsize,
	/// For a handle a call returned lent from another's value, that handle.
	lender: Option<Lender>,
	/// For a value that is not `Send`, the thread it stays on, which alone borrows, moves,
	/// or drops it.
	owner: Option<Arc<Thread>>,
	/// For a value a call returned borrowing another handle's value exclusively, that borrow,
	/// given back only once the value dropped, wherever it drops. Touched only by what
	/// holds the value as [`TAKEN`], or by the last reference's drop.
	loan: UnsafeCell<Option<Loan>>,
	/// Whether `loan` started out holding a borrow, so moving the value out fails but into a
	/// call that ends the value with the borrow: never changes.
	held: bool,
}

/// An exclusive borrow of another handle's value, which a value Rust returned borrowing it
/// holds: given back once the last value sharing it drops, as clones of a borrowing value
/// share their borrow in Rust.
#[derive(Clone)]
pub struct Loan(#[expect(dead_code, reason = "held for its drop")] Arc<dyn Send + Sync>);

impl<T: Send + 'static> ExclusiveArc<T> {
	/// The value, borrowed for as long as the loan it becomes lives.
	///
	/// # Safety
	/// What the returned reference lends outlives neither this nor the [`Loan`] it becomes.
	pub unsafe fn lend<'a>(&mut self) -> &'a mut T {
		self.changed = true;
		// SAFETY: the exclusive borrow is held, for as long as the caller's contract says.
		unsafe { &mut *self.object.value() }
	}

	/// This borrow, held by the value a call returned borrowing it.
	pub fn loan(self) -> Loan {
		Loan(Arc::new(self))
	}
}

/// A value that is not `Send`, held by a handle that keeps it on the thread that made it:
/// every borrow, move, and drop of it checks it is on that thread, or for a drop, queues
/// it there. So a handle to it may cross threads, which the type says to the runtime.
#[repr(transparent)]
pub struct Unsent<T>(T);

// SAFETY: only an `Object` holds one, made by `Object::unsent` and its kin, whose every use
// of it runs on its owner's thread, or for a drop, queues it there.
unsafe impl<T> Send for Unsent<T> {}

impl Unsent<()> {
	/// The name of `U`, an `Unsent`, as the value it holds.
	fn named<U>() -> &'static str {
		let name = std::any::type_name::<U>();
		let inner = name.split_once("Unsent<").and_then(|(_, inner)| inner.strip_suffix('>'));
		inner.unwrap_or(name)
	}
}

impl<T> Unsent<T> {
	/// The value, moved out on its thread.
	pub fn into_inner(self) -> T {
		self.0
	}

	/// A borrowed value, as a borrowed `Unsent` of it.
	fn from_ref(value: &T) -> &Self {
		// SAFETY: `repr(transparent)`.
		unsafe { &*std::ptr::from_ref(value).cast() }
	}

	fn from_mut(value: &mut T) -> &mut Self {
		// SAFETY: `repr(transparent)`.
		unsafe { &mut *std::ptr::from_mut(value).cast() }
	}
}

impl<T: Clone> Clone for Unsent<T> {
	fn clone(&self) -> Self {
		Self(self.0.clone())
	}
}

impl<T: Copy> Copy for Unsent<T> {}

impl<T> Deref for Unsent<T> {
	type Target = T;

	fn deref(&self) -> &T {
		&self.0
	}
}

impl<T> DerefMut for Unsent<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}

/// The handle a returned borrow was lent from, and its epoch then: the borrow lives while
/// the epoch stands, and each use holds the handle shared, so none can start meanwhile. A
/// value that is not `Sync` is held `local`ly, on the using thread alone.
pub struct Lender {
	handle: Arc<dyn Lends>,
	epoch: usize,
	local: bool,
}

/// A handle another is lent from.
trait Lends: Send + Sync {
	/// Borrow the value shared for a use of a handle lent from it, if still at `epoch`, as
	/// the `level`th handle a borrow on the thread in `column` holds.
	fn hold(&self, epoch: usize, local: bool, column: &mut usize, level: u32) -> Result<u64, Status>;
	/// End what [`Self::hold`] took.
	fn unhold(&self, local: bool, column: usize, level: u32, listed: u64);
}

impl<T: Send> Lends for Object<T> {
	fn hold(&self, epoch: usize, local: bool, column: &mut usize, level: u32) -> Result<u64, Status> {
		let listed = match local {
			true => self.acquire_local(column, level)?,
			false => self.acquire(column, level)?,
		};
		if self.epoch.load(Ordering::Acquire) != epoch {
			self.unhold(local, *column, level, listed);
			return Err(Status::Outlived);
		}
		Ok(listed)
	}

	fn unhold(&self, local: bool, column: usize, level: u32, listed: u64) {
		match local {
			true => self.release_local(column, level, listed),
			false => self.release(column, level, listed),
		}
	}
}

/// What a shared borrow of a handle took, to give back: which of the handle and the chain
/// of handles it was lent from went to the table, a bit each from the lowest, below the
/// column of the thread that put them there. One word, so a borrow comes back in registers.
#[derive(Clone, Copy)]
pub struct Ticket(u64);

/// The bits of a [`Ticket`] below its column: past as many nested lenders, which no export
/// makes, the count holds the rest.
const LEVELS: u32 = u64::BITS - COLUMNS.trailing_zeros();

impl Ticket {
	fn new(column: usize, listed: u64) -> Self {
		Self(listed | ((column % COLUMNS) as u64) << LEVELS)
	}

	fn column(self) -> usize {
		(self.0 >> LEVELS) as usize
	}

	fn listed(self) -> u64 {
		self.0 & ((1 << LEVELS) - 1)
	}
}

/// This thread's id for [`Object::borrow_local`]: the address of a thread local, which no
/// other live thread shares and which is neither 0 nor a sentinel. A thread Dart's runtime
/// hands a call to, while the thread it acts for waits, takes that thread's.
pub fn thread() -> usize {
	#[cfg(feature = "dart")]
	if let Some(id) = crate::dart::acting() {
		return id;
	}
	thread_local!(static ID: u8 = const { 0 });
	ID.with(|id| std::ptr::from_ref(id) as usize)
}

// SAFETY: the value moves between threads only by `take` or an exclusive borrow, and is
// shared across threads only by `borrow` and `borrow_arc`, which require `Sync`.
unsafe impl<T: Send> Send for Object<T> {}
// SAFETY: as above.
unsafe impl<T: Send> Sync for Object<T> {}

impl<T> Object<T> {
	/// A new handle owning `value`, which may outlive any borrow, so `value` holds none.
	pub fn new(value: T) -> *const c_void
	where
		T: 'static,
	{
		Self::owning(value, None)
	}

	fn owning(value: T, owner: Option<Arc<Thread>>) -> *const c_void
	where
		T: 'static,
	{
		Arc::into_raw(Arc::new(Self {
			borrow: Borrow::new(0),
			depth: UnsafeCell::new(0),
			lent: std::ptr::null_mut(),
			value: UnsafeCell::new(MaybeUninit::new(value)),
			epoch: AtomicUsize::new(0),
			lender: None,
			owner,
			loan: UnsafeCell::new(None),
			held: false,
		}))
		.cast()
	}

	/// A new handle owning `value`, which holds `loan`, the exclusive borrow of another
	/// handle's value it was returned borrowing: given back once `value` drops, and never
	/// carried out with the value but by a call that ends it, as [`Self::reserve_ending`]
	/// lets one.
	pub fn holding(value: T, loan: Option<Loan>) -> *const c_void
	where
		T: 'static,
	{
		Self::holding_by(value, loan, None)
	}

	fn holding_by(value: T, loan: Option<Loan>, owner: Option<Arc<Thread>>) -> *const c_void
	where
		T: 'static,
	{
		Arc::into_raw(Arc::new(Self {
			borrow: Borrow::new(0),
			depth: UnsafeCell::new(0),
			lent: std::ptr::null_mut(),
			value: UnsafeCell::new(MaybeUninit::new(value)),
			epoch: AtomicUsize::new(0),
			lender: None,
			owner,
			held: loan.is_some(),
			loan: UnsafeCell::new(loan),
		}))
		.cast()
	}

	/// A handle to `value`, which a call returned lent from `lender`'s value: shared, as a
	/// returned `&T` is, and borrowed as its own type is, until that value is borrowed
	/// exclusively, moved out, or freed, after which a use fails. It is never borrowed
	/// exclusively nor moved out itself.
	pub fn lent(value: &T, lender: Lender) -> *const c_void {
		Self::lent_by(value, lender, None)
	}

	fn lent_by(value: &T, lender: Lender, owner: Option<Arc<Thread>>) -> *const c_void {
		Arc::into_raw(Arc::new(Self {
			borrow: Borrow::new(0),
			depth: UnsafeCell::new(0),
			lent: std::ptr::from_ref(value).cast_mut(),
			value: UnsafeCell::new(MaybeUninit::uninit()),
			epoch: AtomicUsize::new(0),
			lender: Some(lender),
			owner,
			loan: UnsafeCell::new(None),
			held: false,
		}))
		.cast()
	}

	/// The handle a call borrowing it shared returns a borrow lent from, as a [`Lender`].
	///
	/// # Safety
	/// `handle` is a live handle of this type.
	pub unsafe fn lender(handle: *const c_void, what: &str) -> Lender
	where
		T: Send + Sync + 'static,
	{
		// SAFETY: the caller's contract.
		unsafe { Self::lender_of(handle, false, what) }
	}

	/// [`Self::lender`] of a value that is not `Sync`, held on one thread at a time.
	///
	/// # Safety
	/// As [`Self::lender`].
	pub unsafe fn lender_local(handle: *const c_void, what: &str) -> Lender
	where
		T: Send + 'static,
	{
		// SAFETY: the caller's contract.
		unsafe { Self::lender_of(handle, true, what) }
	}

	/// # Safety
	/// As [`Self::lender`].
	unsafe fn lender_of(handle: *const c_void, local: bool, what: &str) -> Lender
	where
		T: Send + 'static,
	{
		// SAFETY: the caller's contract.
		let object = unsafe { Self::arc(handle, what) };
		let epoch = object.epoch.load(Ordering::Acquire);
		Lender { handle: object, epoch, local }
	}

	/// The value: its own, or the one it was lent.
	fn value(&self) -> *mut T {
		match self.lent.is_null() {
			true => self.value.get().cast(),
			false => self.lent,
		}
	}

	/// # Safety
	/// `handle` is null or a live handle of this type.
	unsafe fn get<'a>(handle: *const c_void, what: &str) -> &'a Self {
		if handle.is_null() {
			fail(what, "a handle is null");
		}
		// SAFETY: the caller's contract.
		unsafe { &*handle.cast::<Self>() }
	}

	/// Borrow the value shared, as the `level`th handle of a borrow on the thread in
	/// `column`, or [`NO_COLUMN`] until a borrow needs it: the bits of the handles that went
	/// to the table, from `level` on.
	fn acquire(&self, column: &mut usize, level: u32) -> Result<u64, Status> {
		let table = (level < LEVELS).then_some(&TABLE);
		let listed = self.borrow.share(table, || {
			if *column == NO_COLUMN {
				*column = self::column();
			}
			*column
		})?;
		let listed = u64::from(listed) << level.min(LEVELS - 1);
		self.hold_lender(column, level + 1)
			.map(|lenders| listed | lenders)
			.inspect_err(|_| self.borrow.unshare(&TABLE, *column, listed != 0))
	}

	/// End a shared borrow [`Self::acquire`] took.
	fn release(&self, column: usize, level: u32, listed: u64) {
		self.release_lender(column, level + 1, listed);
		self.borrow.unshare(&TABLE, column, level < LEVELS && listed >> level & 1 != 0);
	}

	/// Borrow a value that is not `Sync` shared: any number of times on one thread, as
	/// Rust allows, and on no other thread meanwhile.
	fn acquire_local(&self, column: &mut usize, level: u32) -> Result<u64, Status> {
		self.own()?;
		self.borrow.local(thread())?;
		// SAFETY: only the thread in the state touches `depth`.
		unsafe { *self.depth.get() += 1 };
		self.hold_lender(column, level + 1).inspect_err(|_| self.unlocal())
	}

	/// End a shared borrow [`Self::acquire_local`] took, on its thread.
	fn release_local(&self, column: usize, level: u32, listed: u64) {
		self.release_lender(column, level + 1, listed);
		self.unlocal();
	}

	/// [`Self::acquire`], as a [`Ticket`]: inlined, for a handle no other lent, as far as
	/// taking the count when no borrow contends for it.
	#[inline(always)]
	fn shared(&self) -> Result<Ticket, Status> {
		if self.lender.is_none() && self.borrow.count() {
			return Ok(Ticket::new(NO_COLUMN, 0));
		}
		self.shared_slow()
	}

	#[inline(never)]
	fn shared_slow(&self) -> Result<Ticket, Status> {
		let mut column = NO_COLUMN;
		let listed = self.acquire(&mut column, 0)?;
		Ok(Ticket::new(column, listed))
	}

	/// End a shared borrow [`Self::shared`] took.
	#[inline(always)]
	fn unshared(&self, ticket: Ticket) {
		if self.lender.is_some() {
			self.release_lender(ticket.column(), 1, ticket.listed());
		}
		self.borrow.unshare(&TABLE, ticket.column(), ticket.listed() & 1 != 0);
	}

	/// The [`Ticket`] of a borrow [`Self::acquire_local`] takes.
	fn local(&self) -> Result<Ticket, Status> {
		let mut column = NO_COLUMN;
		let listed = self.acquire_local(&mut column, 0)?;
		Ok(Ticket::new(column, listed))
	}

	/// Count down this thread's borrows, letting go of the value at the last.
	fn unlocal(&self) {
		// SAFETY: this thread is the one in the state, so `depth` is its own.
		let depth = unsafe { &mut *self.depth.get() };
		*depth -= 1;
		if *depth == 0 {
			self.borrow.set(0);
		}
	}

	/// For a handle lent from another, hold that one for a use of this one.
	fn hold_lender(&self, column: &mut usize, level: u32) -> Result<u64, Status> {
		match &self.lender {
			Some(lender) => lender.handle.hold(lender.epoch, lender.local, column, level),
			None => Ok(0),
		}
	}

	fn release_lender(&self, column: usize, level: u32, listed: u64) {
		if let Some(lender) = &self.lender {
			lender.handle.unhold(lender.local, column, level, listed);
		}
	}

	/// Whether this thread may use the value: any may, unless it is not `Send`.
	#[inline(always)]
	fn own(&self) -> Result<(), Status> {
		match &self.owner {
			None => Ok(()),
			Some(owner) => owner.own(),
		}
	}

	/// Take the value from no borrow to `to`, as an exclusive borrow or a move does. A value
	/// holding an exclusive lend moves out only as `ending` lets it, as the call it moves
	/// into then ends it with the lend.
	fn acquire_from_zero(&self, to: usize) -> Result<(), Status> {
		self.acquire_ending(to, false)
	}

	fn acquire_ending(&self, to: usize, ending: bool) -> Result<(), Status> {
		self.own()?;
		let moved = matches!(to, TAKEN | RESERVED);
		if self.lender.is_some() || (moved && (!self.lent.is_null() || (self.held && !ending))) {
			return Err(Status::Lent);
		}
		self.borrow.take(&TABLE, to)
	}
	/// Borrow the value shared for a call.
	///
	/// # Safety
	/// `handle` is a live handle of this type, kept alive through the borrow.
	#[inline(always)]
	pub unsafe fn borrow<'a>(handle: *const c_void, what: &str) -> Result<Shared<'a, T>, Status>
	where
		T: Sync,
	{
		// SAFETY: the caller's contract.
		let object = unsafe { Self::get(handle, what) };
		let ticket = object.shared()?;
		Ok(Shared { object, ticket })
	}

	/// Borrow the value shared for a call, uncounted: no export borrows a value of its type
	/// exclusively, moves it out, or lends it, and it is `Sync`, so no borrow can conflict
	/// with this one, which costs no atomic operation.
	///
	/// # Safety
	/// As [`Self::borrow`], for such a type alone.
	pub unsafe fn peek<'a>(handle: *const c_void, what: &str) -> Peek<'a, T>
	where
		T: Sync,
	{
		// SAFETY: the caller's contract.
		Peek(unsafe { &*Self::get(handle, what).value() })
	}

	/// Borrow a value that is not `Sync` shared for a call: any number of times on one
	/// thread, as Rust allows, and on no other thread meanwhile.
	///
	/// # Safety
	/// As [`Self::borrow`].
	pub unsafe fn borrow_local<'a>(handle: *const c_void, what: &str) -> Result<Local<'a, T>, Status> {
		// SAFETY: the caller's contract.
		let object = unsafe { Self::get(handle, what) };
		let ticket = object.local()?;
		Ok(Local { object, ticket })
	}

	/// Borrow the value exclusively for a call.
	///
	/// # Safety
	/// As [`Self::borrow`].
	pub unsafe fn borrow_mut<'a>(handle: *const c_void, what: &str) -> Result<Exclusive<'a, T>, Status> {
		// SAFETY: the caller's contract.
		let object = unsafe { Self::get(handle, what) };
		object.acquire_from_zero(EXCLUSIVE)?;
		Ok(Exclusive { object, changed: false })
	}

	/// Borrow the value shared for as long as a future runs, keeping the handle alive.
	///
	/// # Safety
	/// `handle` is a live handle of this type.
	pub unsafe fn borrow_arc(handle: *const c_void, what: &str) -> Result<SharedArc<T>, Status>
	where
		T: Sync,
	{
		// SAFETY: the caller's contract.
		let object = unsafe { Self::get(handle, what) };
		let ticket = object.shared()?;
		// SAFETY: as above.
		Ok(SharedArc {
			object: unsafe { Self::arc(handle, what) },
			ticket,
		})
	}

	/// Borrow the value exclusively for as long as a future runs.
	///
	/// # Safety
	/// As [`Self::borrow_arc`].
	pub unsafe fn borrow_mut_arc(handle: *const c_void, what: &str) -> Result<ExclusiveArc<T>, Status> {
		// SAFETY: the caller's contract.
		let object = unsafe { Self::get(handle, what) };
		object.acquire_from_zero(EXCLUSIVE)?;
		// SAFETY: as above.
		Ok(ExclusiveArc {
			object: unsafe { Self::arc(handle, what) },
			changed: false,
		})
	}

	/// # Safety
	/// `handle` is a live handle of this type.
	unsafe fn arc(handle: *const c_void, what: &str) -> Arc<Self> {
		// SAFETY: the caller's contract.
		unsafe { Self::get(handle, what) };
		let ptr = handle.cast::<Self>();
		// SAFETY: a live handle holds a count, so one more keeps it alive.
		unsafe {
			Arc::increment_strong_count(ptr);
			Arc::from_raw(ptr)
		}
	}

	/// Lend `value` to C as a handle while `f` runs, shared as `&T` lends it.
	pub fn lend<R>(value: &T, what: &str, f: impl FnOnce(*const c_void) -> R) -> R
	where
		T: Sync,
	{
		Self::lending(std::ptr::from_ref(value).cast_mut(), 1, 0, None, what, f)
	}

	/// Lend a value that is not `Sync` shared, as [`Self::lend`], on this thread alone.
	pub fn lend_local<R>(value: &T, what: &str, f: impl FnOnce(*const c_void) -> R) -> R {
		Self::lending(std::ptr::from_ref(value).cast_mut(), thread(), 1, None, what, f)
	}

	/// Lend `value` exclusively, as `&mut T` lends it.
	pub fn lend_mut<R>(value: &mut T, what: &str, f: impl FnOnce(*const c_void) -> R) -> R {
		Self::lending(std::ptr::from_mut(value), 0, 0, None, what, f)
	}

	/// Lend the value at `ptr` to C as a handle while `f` runs, holding the borrow `state`
	/// and `depth` describe, if any, so C borrows it no further than Rust lent it. C may
	/// neither move it out, nor free the handle, nor keep a borrow past `f`.
	fn lending<R>(
		ptr: *mut T,
		state: usize,
		depth: usize,
		owner: Option<Arc<Thread>>,
		what: &str,
		f: impl FnOnce(*const c_void) -> R,
	) -> R {
		let object = Arc::new(Self {
			borrow: Borrow::new(state),
			depth: UnsafeCell::new(depth),
			lent: ptr,
			value: UnsafeCell::new(MaybeUninit::uninit()),
			epoch: AtomicUsize::new(0),
			lender: None,
			owner,
			loan: UnsafeCell::new(None),
			held: false,
		});
		// A second count, which a free by C would take, so the handle outlives it.
		let spare = Arc::clone(&object);
		let result = f(Arc::as_ptr(&object).cast());
		match Arc::strong_count(&object) {
			2 if object.borrow.is(&TABLE, state) => {}
			1 => fail(what, "a lent handle was freed"),
			_ => fail(what, "a lent handle is still borrowed"),
		}
		drop(spare);
		result
	}

	/// Whether a call or a future borrows the value, so moving it out would fail. A value
	/// moved out is not, so the move fails as moved.
	///
	/// # Safety
	/// As [`Self::borrow`].
	pub unsafe fn borrowed(handle: *const c_void, what: &str) -> bool {
		// SAFETY: the caller's contract.
		let object = unsafe { Self::get(handle, what) };
		object.borrow.borrowed(&TABLE)
	}

	/// Move the value out, leaving the handle to be freed.
	///
	/// # Safety
	/// As [`Self::borrow`].
	pub unsafe fn take(handle: *const c_void, what: &str) -> Result<T, Status> {
		// SAFETY: the caller's contract.
		unsafe { Self::get(handle, what) }.acquire_from_zero(TAKEN)?;
		// SAFETY: as above, and the value is reserved.
		Ok(unsafe { Self::taken(handle) })
	}

	/// Reserve the value to move out once every other check of the call passed, which
	/// [`Self::taken`] then does: `takes` gives it back if the call fails first.
	///
	/// # Safety
	/// As [`Self::borrow`], kept alive until `takes` commits or drops.
	pub unsafe fn reserve<'a>(handle: *const c_void, takes: &mut Takes<'a>, what: &str) -> Result<(), Status> {
		// SAFETY: the caller's contract.
		let object = unsafe { Self::get(handle, what) };
		object.acquire_from_zero(RESERVED)?;
		// SAFETY: the state outlives `takes`, by the caller's contract; a pointer to it,
		// so the value's type need not.
		takes.push(unsafe { &*::std::ptr::addr_of!(object.borrow) });
		Ok(())
	}

	/// [`Self::reserve`] for a call the value ends in, as its lifetimes name nothing else
	/// the call takes or returns: a value holding an exclusive lend may move out into it,
	/// and [`Self::end`] gives the lend back once the call returns.
	///
	/// # Safety
	/// As [`Self::reserve`].
	pub unsafe fn reserve_ending<'a>(handle: *const c_void, takes: &mut Takes<'a>, what: &str) -> Result<(), Status> {
		// SAFETY: the caller's contract.
		let object = unsafe { Self::get(handle, what) };
		object.acquire_ending(RESERVED, true)?;
		// SAFETY: as in `reserve`.
		takes.push(unsafe { &*::std::ptr::addr_of!(object.borrow) });
		Ok(())
	}

	/// Give back the exclusive lend a value moved out by [`Self::reserve_ending`] held, now
	/// that the call it moved into returned, and the value with it.
	///
	/// # Safety
	/// `handle` is a live handle of this type whose value such a call moved out.
	pub unsafe fn end(handle: *const c_void) {
		// SAFETY: the caller's contract: TAKEN excludes every other use of the loan.
		drop(unsafe { (*(*handle.cast::<Self>()).loan.get()).take() });
	}

	/// Fail a call whose signature would carry a borrow out of the value, or let the value
	/// outlive it, if the value holds an exclusive lend, whose borrow lasts only as long as
	/// the value.
	///
	/// # Safety
	/// `handle` is null or a live handle of this type.
	pub unsafe fn unheld(handle: *const c_void, what: &str) -> Result<(), Status> {
		// SAFETY: the caller's contract; `held` never changes.
		match !handle.is_null() && unsafe { Self::get(handle, what) }.held {
			true => Err(Status::Lent),
			false => Ok(()),
		}
	}

	/// Borrow a `Copy` value shared until `takes` drops, so [`Self::held`] copies it out
	/// once the call's moves commit, and no check after them can fail.
	///
	/// # Safety
	/// As [`Self::reserve`].
	pub unsafe fn hold<'a>(handle: *const c_void, takes: &mut Takes<'a>, what: &str) -> Result<(), Status>
	where
		T: Sync,
	{
		// SAFETY: the caller's contract.
		let object = unsafe { Self::get(handle, what) };
		let ticket = object.shared()?;
		takes.held.push((handle, ticket, Self::unhold));
		Ok(())
	}

	/// [`Self::hold`], for a value that is not `Sync`.
	///
	/// # Safety
	/// As [`Self::reserve`].
	pub unsafe fn hold_local<'a>(handle: *const c_void, takes: &mut Takes<'a>, what: &str) -> Result<(), Status> {
		// SAFETY: the caller's contract.
		let object = unsafe { Self::get(handle, what) };
		let ticket = object.local()?;
		takes.held.push((handle, ticket, Self::unhold_local));
		Ok(())
	}

	/// # Safety
	/// [`Self::hold`] borrowed `handle`'s value, and this ends that borrow.
	unsafe fn unhold(handle: *const c_void, ticket: Ticket) {
		// SAFETY: the caller's contract.
		unsafe { &*handle.cast::<Self>() }.unshared(ticket);
	}

	/// # Safety
	/// [`Self::hold_local`] borrowed `handle`'s value, and this ends that borrow.
	unsafe fn unhold_local(handle: *const c_void, ticket: Ticket) {
		// SAFETY: the caller's contract.
		unsafe { &*handle.cast::<Self>() }.release_local(ticket.column(), 0, ticket.listed());
	}

	/// A copy of the value [`Self::hold`] or [`Self::hold_local`] borrowed.
	///
	/// # Safety
	/// One of them borrowed `handle`'s value into a `Takes` not yet dropped.
	pub unsafe fn held(handle: *const c_void) -> T
	where
		T: Copy,
	{
		// SAFETY: the caller's contract: the borrow excludes every write.
		unsafe { *(*handle.cast::<Self>()).value() }
	}

	/// Move out the value [`Self::take`] took, or [`Self::reserve`] reserved for a call whose
	/// `Takes` committed, leaving the handle to be freed.
	///
	/// # Safety
	/// One of them did so for `handle`'s value, and no other read moves it out.
	pub unsafe fn taken(handle: *const c_void) -> T {
		// SAFETY: the caller's contract.
		let object = unsafe { &*handle.cast::<Self>() };
		object.epoch.fetch_add(1, Ordering::Release);
		// SAFETY: TAKEN excludes every other access, and the value is never read again.
		unsafe { (*object.value.get()).assume_init_read() }
	}

	/// Drop the value now, as a move out would, leaving the handle to be freed: what a
	/// binding's close does, so a later use fails as moved, as Rust refuses a use after a
	/// drop. It fails, changing nothing, while a call or future borrows the value or a call
	/// reserved it to move out, as Rust refuses a drop under a borrow, and does nothing once
	/// the value is gone. A handle a
	/// call returned lent from another's value only stops lending it; one lent to C for a
	/// call fails, as Rust lent it. A value that is not `Send`, closed on another thread than
	/// its own, drops there later, as a free does.
	///
	/// # Safety
	/// As [`Self::borrow`].
	pub unsafe fn close(handle: *const c_void, what: &str) -> Result<(), Status> {
		// SAFETY: the caller's contract.
		let object = unsafe { Self::get(handle, what) };
		let away = object.owner.as_ref().filter(|owner| !owner.is_here());
		if away.is_none() {
			object.own()?;
		}
		if !object.lent.is_null() && object.lender.is_none() {
			return Err(Status::Lent);
		}
		match object.borrow.take(&TABLE, TAKEN) {
			Ok(()) => {}
			Err(Status::Moved) => return Ok(()),
			Err(status) => return Err(status),
		}
		object.epoch.fetch_add(1, Ordering::Release);
		if object.lent.is_null() {
			// SAFETY: TAKEN excludes every other access to the loan.
			let loan = unsafe { (*object.loan.get()).take() };
			match away {
				Some(owner) => {
					// SAFETY: as below; the value moves out to drop on its own thread, before
					// the lend it holds is given back.
					let value = unsafe { (*object.value.get()).assume_init_read() };
					// SAFETY: the value is the owner's.
					unsafe { Dropped::new((value, loan), owner.clone(), Unsent::<()>::named::<T>()) }.send();
				}
				None => {
					// SAFETY: TAKEN excludes every other access, and `Drop` skips a taken value.
					unsafe { (*object.value.get()).assume_init_drop() };
					drop(loan);
				}
			}
		}
		Ok(())
	}

	/// The value of a handle a closure C implements returned Rust, which copies it out and
	/// frees the handle.
	///
	/// # Safety
	/// `handle` is a live handle of this type, which C gives up.
	pub unsafe fn returned(handle: *const c_void, what: &str) -> T
	where
		T: Copy,
	{
		// SAFETY: the caller's contract.
		let value = *check(unsafe { Self::borrow_local(handle, what) }, what);
		// SAFETY: as above; the borrow ended.
		unsafe { Self::free(handle) };
		value
	}

	/// Release the caller's handle.
	///
	/// # Safety
	/// `handle` is null or a live handle of this type, released once.
	pub unsafe fn free(handle: *const c_void) {
		if !handle.is_null() {
			// SAFETY: the caller's contract.
			let object = unsafe { Arc::from_raw(handle.cast::<Self>()) };
			// A future may still hold the value, but no handle lent from it outlives C's.
			object.epoch.fetch_add(1, Ordering::Release);
			drop(object);
		}
	}
}

impl<T> Drop for Object<T> {
	fn drop(&mut self) {
		let mut owner = self.owner.take();
		// Given back only once the value dropped, wherever it drops.
		let loan = self.loan.get_mut().take();
		if self.lent.is_null() && *self.borrow.state.get_mut() != TAKEN {
			let value = self.value.get_mut();
			match owner.take_if(|owner| !owner.is_here()) {
				// A value that is not `Send` drops on its own thread.
				Some(owner) => {
					// SAFETY: as below; the value moves out to drop there, and nothing reads it here again.
					let value = unsafe { value.assume_init_read() };
					// SAFETY: the value is the owner's.
					unsafe { Dropped::new((value, loan), owner, Unsent::<()>::named::<T>()) }.send();
				}
				None => {
					// SAFETY: the last reference is gone, and the value is its own and never taken.
					unsafe { value.assume_init_drop() };
					drop(loan);
				}
			}
		} else {
			drop(loan);
		}
		// A handle whose value dropped already, as one closed or lent, may hold the last
		// record of its home but the home's own: letting go wakes the home to see so.
		if let Some(owner) = owner {
			Thread::let_go(owner);
		}
	}
}

impl<T> Object<Unsent<T>> {
	/// A new handle owning `value`, which is not `Send`, so stays on this thread: a use on
	/// another fails, and a free there drops it here.
	pub fn unsent(value: T) -> *const c_void
	where
		T: 'static,
	{
		Self::owning(Unsent(value), Some(Thread::here()))
	}

	/// [`Self::holding`] of a value that is not `Send`, on this thread.
	pub fn holding_unsent(value: T, loan: Option<Loan>) -> *const c_void
	where
		T: 'static,
	{
		Self::holding_by(Unsent(value), loan, Some(Thread::here()))
	}

	/// [`Self::lent`] of a value that is not `Send`, on this thread.
	pub fn lent_unsent(value: &T, lender: Lender) -> *const c_void {
		Self::lent_by(Unsent::from_ref(value), lender, Some(Thread::here()))
	}

	/// [`Self::lend_local`] of a value that is not `Send`, on this thread.
	pub fn lend_unsent<R>(value: &T, what: &str, f: impl FnOnce(*const c_void) -> R) -> R {
		let ptr = std::ptr::from_ref(Unsent::from_ref(value)).cast_mut();
		Self::lending(ptr, thread(), 1, Some(Thread::here()), what, f)
	}

	/// [`Self::lend_mut`] of a value that is not `Send`, on this thread.
	pub fn lend_mut_unsent<R>(value: &mut T, what: &str, f: impl FnOnce(*const c_void) -> R) -> R {
		Self::lending(std::ptr::from_mut(Unsent::from_mut(value)), 0, 0, Some(Thread::here()), what, f)
	}
}

impl<T> Object<T> {
	/// The thread `handle`'s value stays on, unless it is null: a call borrowing or moving
	/// it runs there if that is a home elsewhere, as the thread that made it.
	///
	/// # Safety
	/// `handle` is null or a live handle of this type, kept alive through the call.
	pub unsafe fn owner<'a>(handle: *const c_void) -> Option<&'a Thread> {
		if handle.is_null() {
			return None;
		}
		// SAFETY: the caller's contract; `owner` never changes.
		let object = unsafe { &*handle.cast::<Self>() };
		object.owner.as_deref()
	}
}

/// The values a call reserved to move out, given back unless it commits: so a call whose
/// later check fails moved nothing. A call's first few allocate nothing. It also holds the
/// `Copy` values the call copies out borrowed until it drops, after they are copied.
#[derive(Default)]
pub struct Takes<'a> {
	first: [Option<&'a Borrow>; 2],
	rest: Vec<&'a Borrow>,
	held: Vec<(*const c_void, Ticket, unsafe fn(*const c_void, Ticket))>,
}

impl<'a> Takes<'a> {
	fn push(&mut self, state: &'a Borrow) {
		match self.first.iter_mut().find(|slot| slot.is_none()) {
			Some(slot) => *slot = Some(state),
			None => self.rest.push(state),
		}
	}

	/// Mark every value reserved moved out, for the call to move out.
	pub fn commit(&mut self) {
		for state in self.first.iter_mut().filter_map(Option::take).chain(self.rest.drain(..)) {
			state.set(TAKEN);
		}
	}
}

impl Drop for Takes<'_> {
	fn drop(&mut self) {
		for state in self.first.iter().flatten().chain(&self.rest) {
			state.set(0);
		}
		for (handle, ticket, unhold) in self.held.drain(..) {
			// SAFETY: `hold` or `hold_local` borrowed it, for this to end.
			unsafe { unhold(handle, ticket) };
		}
	}
}

/// Frees the buffers C handed Rust in arguments a call has not lifted yet, unless the call
/// lifts them: so a call a check refuses leaks none of what it was given.
pub struct Unlifted<F: FnOnce()>(Option<F>);

impl<F: FnOnce()> Unlifted<F> {
	pub fn new(release: F) -> Self {
		Self(Some(release))
	}

	/// The call passed its checks and lifts the arguments, which then own the buffers.
	pub fn lifted(mut self) {
		self.0 = None;
	}
}

impl<F: FnOnce()> Drop for Unlifted<F> {
	fn drop(&mut self) {
		if let Some(release) = self.0.take() {
			release();
		}
	}
}

pub struct Shared<'a, T> {
	object: &'a Object<T>,
	ticket: Ticket,
}

impl<T> Shared<'_, T> {
	/// The exclusive lend the value holds, for a clone of it to share.
	pub fn loan(&self) -> Option<Loan> {
		// SAFETY: the shared borrow excludes the TAKEN that alone changes the hold.
		unsafe { (*self.object.loan.get()).clone() }
	}
}

impl<T> Deref for Shared<'_, T> {
	type Target = T;
	fn deref(&self) -> &T {
		// SAFETY: a shared borrow is held.
		unsafe { &*self.object.value() }
	}
}

impl<T> Drop for Shared<'_, T> {
	fn drop(&mut self) {
		self.object.unshared(self.ticket);
	}
}

/// [`Object::peek`]'s borrow, which ends with nothing to release.
pub struct Peek<'a, T>(&'a T);

impl<T> Deref for Peek<'_, T> {
	type Target = T;
	fn deref(&self) -> &T {
		self.0
	}
}

pub struct Local<'a, T> {
	object: &'a Object<T>,
	ticket: Ticket,
}

impl<T> Local<'_, T> {
	/// The exclusive lend the value holds, for a clone of it to share.
	pub fn loan(&self) -> Option<Loan> {
		// SAFETY: the shared borrow excludes the TAKEN that alone changes the hold.
		unsafe { (*self.object.loan.get()).clone() }
	}
}

impl<T> Deref for Local<'_, T> {
	type Target = T;
	fn deref(&self) -> &T {
		// SAFETY: this thread holds the value shared.
		unsafe { &*self.object.value() }
	}
}

impl<T> Drop for Local<'_, T> {
	fn drop(&mut self) {
		self.object.release_local(self.ticket.column(), 0, self.ticket.listed());
	}
}

/// An exclusive borrow, which once used to change the value ends every handle lent from it.
pub struct Exclusive<'a, T> {
	object: &'a Object<T>,
	changed: bool,
}

impl<T> Deref for Exclusive<'_, T> {
	type Target = T;
	fn deref(&self) -> &T {
		// SAFETY: the exclusive borrow is held.
		unsafe { &*self.object.value() }
	}
}

impl<T> DerefMut for Exclusive<'_, T> {
	fn deref_mut(&mut self) -> &mut T {
		self.changed = true;
		// SAFETY: the exclusive borrow is held.
		unsafe { &mut *self.object.value() }
	}
}

impl<T> Drop for Exclusive<'_, T> {
	fn drop(&mut self) {
		if self.changed {
			self.object.epoch.fetch_add(1, Ordering::Release);
		}
		self.object.borrow.set(0);
	}
}

pub struct SharedArc<T> {
	object: Arc<Object<T>>,
	ticket: Ticket,
}

impl<T> Deref for SharedArc<T> {
	type Target = T;
	fn deref(&self) -> &T {
		// SAFETY: a shared borrow is held.
		unsafe { &*self.object.value() }
	}
}

impl<T> Drop for SharedArc<T> {
	fn drop(&mut self) {
		self.object.unshared(self.ticket);
	}
}

/// [`Exclusive`], for as long as a future runs.
pub struct ExclusiveArc<T> {
	object: Arc<Object<T>>,
	changed: bool,
}

impl<T> Deref for ExclusiveArc<T> {
	type Target = T;
	fn deref(&self) -> &T {
		// SAFETY: the exclusive borrow is held.
		unsafe { &*self.object.value() }
	}
}

impl<T> DerefMut for ExclusiveArc<T> {
	fn deref_mut(&mut self) -> &mut T {
		self.changed = true;
		// SAFETY: the exclusive borrow is held.
		unsafe { &mut *self.object.value() }
	}
}

impl<T> Drop for ExclusiveArc<T> {
	fn drop(&mut self) {
		if self.changed {
			self.object.epoch.fetch_add(1, Ordering::Release);
		}
		self.object.borrow.set(0);
	}
}

/// A foreign waker: `wake` may be called from any thread, any number of times, until
/// `drop` releases `data`. Either function may be null.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Waker {
	pub wake: Option<unsafe extern "C" fn(*mut c_void)>,
	pub drop: Option<unsafe extern "C" fn(*mut c_void)>,
	pub data: *mut c_void,
}

impl Waker {
	/// Whether `other` is this waker again, which a poll reuses rather than wrapping anew.
	fn same(&self, other: &Self) -> bool {
		self.wake.map(|f| f as usize) == other.wake.map(|f| f as usize) && self.data == other.data
	}

	/// The Rust waker around it, which calls `drop` once its last clone drops. One Rust
	/// returned stays wrapped too, so its `data` names it for as long as a future may
	/// compare it.
	pub fn lift(self) -> std::task::Waker {
		match (self.wake, self.drop) {
			// Nothing to wake or release: a ready future's first poll takes no allocation.
			(None, None) => std::task::Waker::noop().clone(),
			_ => std::task::Waker::from(Arc::new(Foreign(self))),
		}
	}

	/// Wake it, as C does.
	///
	/// # Safety
	/// It is not yet dropped.
	pub unsafe fn wake(&self) {
		if let Some(wake) = self.wake {
			// SAFETY: the waker's contract.
			unsafe { wake(self.data) };
		}
	}

	/// Drop it, as C does once.
	///
	/// # Safety
	/// It is not yet dropped, and is never used again.
	pub unsafe fn release(self) {
		if let Some(drop) = self.drop {
			// SAFETY: the waker's contract.
			unsafe { drop(self.data) };
		}
	}

	/// Rust's waker, or a context's, as one C owns: `wake` wakes it, and `drop` releases it.
	pub fn lower(waker: impl Into<Returned>) -> Self {
		Self {
			wake: Some(rust_wake),
			drop: Some(rust_drop),
			data: Box::into_raw(Box::new(waker.into().0)).cast(),
		}
	}
}

/// A waker a function returns: its own, or a context's.
pub struct Returned(std::task::Waker);

impl From<std::task::Waker> for Returned {
	fn from(waker: std::task::Waker) -> Self {
		Self(waker)
	}
}

impl From<Context<'_>> for Returned {
	fn from(cx: Context<'_>) -> Self {
		Self(cx.waker().clone())
	}
}

unsafe extern "C" fn rust_wake(data: *mut c_void) {
	// SAFETY: `Waker::lower` made `data`, which C has not dropped.
	unsafe { &*data.cast::<std::task::Waker>() }.wake_by_ref();
}

unsafe extern "C" fn rust_drop(data: *mut c_void) {
	// SAFETY: `Waker::lower` made `data`, which C drops once.
	drop(unsafe { Box::from_raw(data.cast::<std::task::Waker>()) });
}

/// A future C implements, of `()`: Rust polls it with a waker of its own each time, from
/// any thread, until `poll` returns true, and calls `free` once it drops the future, done
/// or not. `free` may be null.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ForeignFuture {
	pub poll: Option<unsafe extern "C" fn(*mut c_void, Waker) -> bool>,
	pub free: Option<unsafe extern "C" fn(*mut c_void)>,
	pub data: *mut c_void,
}

impl ForeignFuture {
	/// The Rust future around it, which frees it when dropped.
	pub fn lift(self, what: &str) -> Pin<Box<dyn std::future::Future<Output = ()> + Send>> {
		let Some(poll) = self.poll else {
			fail(what, "a future's poll is null");
		};
		Box::pin(Polled {
			poll,
			free: self.free,
			data: self.data,
		})
	}
}

/// A [`ForeignFuture`] Rust holds, whose poll is not null.
struct Polled {
	poll: unsafe extern "C" fn(*mut c_void, Waker) -> bool,
	free: Option<unsafe extern "C" fn(*mut c_void)>,
	data: *mut c_void,
}

// SAFETY: the foreign future's contract: it may be polled and freed from any thread.
unsafe impl Send for Polled {}

impl std::future::Future for Polled {
	type Output = ();

	fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> std::task::Poll<()> {
		// SAFETY: the foreign future's contract; C owns the waker it is given.
		match unsafe { (self.poll)(self.data, Waker::lower(cx.waker().clone())) } {
			true => std::task::Poll::Ready(()),
			false => std::task::Poll::Pending,
		}
	}
}

impl Drop for Polled {
	fn drop(&mut self) {
		if let Some(free) = self.free {
			// SAFETY: the foreign future's contract; Rust drops it once.
			unsafe { free(self.data) };
		}
	}
}

struct Foreign(Waker);

// SAFETY: the waker's contract: `wake` and `drop` may be called from any thread.
unsafe impl Send for Foreign {}
// SAFETY: as above.
unsafe impl Sync for Foreign {}

impl Wake for Foreign {
	fn wake(self: Arc<Self>) {
		self.wake_by_ref();
	}

	fn wake_by_ref(self: &Arc<Self>) {
		if let Some(wake) = self.0.wake {
			// SAFETY: the waker's contract.
			unsafe { wake(self.0.data) };
		}
	}
}

impl Drop for Foreign {
	fn drop(&mut self) {
		if let Some(drop) = self.0.drop {
			// SAFETY: the waker's contract; this is the last reference.
			unsafe { drop(self.0.data) };
		}
	}
}

/// The data of a function C lends for a call, which Rust only passes back to it.
#[derive(Clone, Copy)]
pub struct Data(*mut c_void);

// SAFETY: the lent function's contract: it may be called from any thread.
unsafe impl Send for Data {}
// SAFETY: as above.
unsafe impl Sync for Data {}

impl Data {
	pub fn new(data: *mut c_void) -> Self {
		Self(data)
	}

	/// The pointer, through a method so a closure captures `Data` whole, not its field.
	pub fn get(self) -> *mut c_void {
		self.0
	}
}

/// The data of a function C implements that Rust keeps, which it frees once dropped.
pub struct Kept {
	data: *mut c_void,
	free: Option<unsafe extern "C" fn(*mut c_void)>,
}

// SAFETY: the kept function's contract: it, and its free, may be called from any thread.
unsafe impl Send for Kept {}
// SAFETY: as above.
unsafe impl Sync for Kept {}

impl Kept {
	pub fn new(data: *mut c_void, free: Option<unsafe extern "C" fn(*mut c_void)>) -> Self {
		Self { data, free }
	}

	pub fn get(&self) -> *mut c_void {
		self.data
	}
}

impl Drop for Kept {
	fn drop(&mut self) {
		if let Some(free) = self.free {
			// SAFETY: the kept function's contract; Rust drops it once.
			unsafe { free(self.data) };
		}
	}
}

/// A reader C implements, which Rust reads and seeks during the call it is lent to.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ForeignReader {
	pub read: Option<unsafe extern "C" fn(*mut c_void, *mut u8, usize, *mut usize, *mut Buf) -> bool>,
	pub seek: Option<unsafe extern "C" fn(*mut c_void, std::ffi::c_int, i64, *mut u64, *mut Buf) -> bool>,
	pub data: *mut c_void,
}

impl ForeignReader {
	/// The Rust reader around it.
	pub fn lift(self, what: &'static str) -> Reader {
		let (Some(read), Some(seek)) = (self.read, self.seek) else {
			fail(what, "a reader's function is null");
		};
		Reader {
			read,
			seek,
			data: self.data,
			what,
		}
	}
}

/// A [`ForeignReader`] whose functions are not null.
pub struct Reader {
	read: unsafe extern "C" fn(*mut c_void, *mut u8, usize, *mut usize, *mut Buf) -> bool,
	seek: unsafe extern "C" fn(*mut c_void, std::ffi::c_int, i64, *mut u64, *mut Buf) -> bool,
	data: *mut c_void,
	what: &'static str,
}

// SAFETY: the reader's contract: it may be called from any thread, one call at a time.
unsafe impl Send for Reader {}
// SAFETY: as above; every call takes `&mut self`.
unsafe impl Sync for Reader {}

impl Reader {
	/// The error a failed call wrote, as its message.
	fn error(&self, err: Buf) -> std::io::Error {
		// SAFETY: the reader's contract: a buffer `<prefix>_buf_copy` made.
		std::io::Error::other(unsafe { err.take_string(self.what) })
	}
}

impl std::io::Read for Reader {
	fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
		let mut n = 0;
		let mut err = Buf::NONE;
		// SAFETY: the reader's contract.
		match unsafe { (self.read)(self.data, buf.as_mut_ptr(), buf.len(), &mut n, &mut err) } {
			true if n <= buf.len() => Ok(n),
			true => fail(self.what, "a reader read more than it was asked"),
			false => Err(self.error(err)),
		}
	}
}

impl std::io::Seek for Reader {
	fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
		// C's `whence`, which every platform numbers alike.
		let (whence, offset) = match pos {
			// Its bits, which the reader reads back unsigned, so a start past `i64::MAX`
			// reaches it as Rust's own readers such as `Cursor` take one.
			std::io::SeekFrom::Start(offset) => (0, offset.cast_signed()),
			std::io::SeekFrom::Current(offset) => (1, offset),
			std::io::SeekFrom::End(offset) => (2, offset),
		};
		let mut position = 0;
		let mut err = Buf::NONE;
		// SAFETY: the reader's contract.
		match unsafe { (self.seek)(self.data, whence, offset, &mut position, &mut err) } {
			true => Ok(position),
			false => Err(self.error(err)),
		}
	}
}

pub const PENDING: i32 = 0;
pub const READY: i32 = 1;
pub const ERROR: i32 = 2;

/// A future behind a handle, polled from C: the head of the one allocation holding the
/// future and the function that lowers its result, which [`Future::new`] makes.
#[repr(C)]
pub struct Future {
	/// Poll the allocation, lowering the result through the poll's out pointers once ready.
	poll: unsafe fn(*mut Future, Waker, &[*mut c_void], *mut c_void, &str) -> i32,
	/// Drop the allocation.
	drop: unsafe fn(*mut Future),
	/// The last foreign waker, and the Rust waker around it.
	waker: Option<(Waker, std::task::Waker)>,
	polling: AtomicBool,
	/// For a future that is not `Send`, the thread it stays on, which alone polls or drops it.
	owner: Option<Arc<Thread>>,
}

/// The allocation behind a handle: its head first, so the handle points at both.
#[repr(C)]
struct Typed<F, L> {
	head: Future,
	/// The future, pinned here until it finishes and drops in place.
	future: Option<F>,
	/// Writes the finished future's result through the poll's out pointers, one per value
	/// it returns, and its error pointer, returning the poll. The poll that finishes the
	/// future calls it, so it never crosses threads.
	lower: L,
}

impl Future {
	pub fn new<F, L>(future: F, lower: L) -> *mut Self
	where
		F: std::future::Future + Send + 'static,
		// Send, as any poll may finish the future on another thread.
		L: Fn(F::Output, &[*mut c_void], *mut c_void) -> i32 + Send + 'static,
	{
		Self::make(future, lower, None)
	}

	/// A future that is not `Send`, which stays on this thread: a poll on another fails,
	/// unless this is a home, which the poll then runs on, and a free there drops it here.
	pub fn unsent<F, L>(future: F, lower: L) -> *mut Self
	where
		F: std::future::Future + 'static,
		L: Fn(F::Output, &[*mut c_void], *mut c_void) -> i32 + 'static,
	{
		Self::make(future, lower, Some(Thread::here()))
	}

	fn make<F, L>(future: F, lower: L, owner: Option<Arc<Thread>>) -> *mut Self
	where
		F: std::future::Future + 'static,
		L: Fn(F::Output, &[*mut c_void], *mut c_void) -> i32 + 'static,
	{
		let typed = Box::new(Typed {
			head: Self {
				poll: poll_typed::<F, L>,
				drop: drop_typed::<F, L>,
				waker: None,
				polling: AtomicBool::new(false),
				owner,
			},
			future: Some(future),
			lower,
		});
		Box::into_raw(typed).cast()
	}

	/// Poll it once with `waker`, writing the result through `outs` and `err` when ready. A
	/// future that is not `Send` polled on another thread than its own fails, writing the
	/// status through `status`, or aborting if that is null; one on a home polls there.
	///
	/// # Safety
	/// `future` is a live future handle, each of `outs` and `err` is null or points where
	/// the function's result types go, and `status` is null or points where an `int32_t` goes.
	pub unsafe fn poll(
		future: *mut Self,
		waker: Waker,
		outs: &[*mut c_void],
		err: *mut c_void,
		status: *mut i32,
		what: &str,
	) -> i32 {
		if future.is_null() {
			fail(what, "a future is null");
		}
		// SAFETY: the caller's contract; only the flag and the owner are touched before
		// it is held.
		let head = unsafe { &*future };
		if let Some(owner) = &head.owner {
			if owner.away() {
				// SAFETY: a home, and this thread waits while it polls.
				return unsafe { owner.run(|| Self::poll(future, waker, outs, err, status, what)) };
			}
			if let Err(refused) = owner.own() {
				if head.polling.swap(true, Ordering::Acquire) {
					fail(what, "a future is polled concurrently");
				}
				// A waker it never adopted is Rust's to drop, as a poll drops the one it
				// replaces; the flag keeps the one it holds from changing meanwhile.
				// SAFETY: the flag makes this the only access.
				if !unsafe { &(*future).waker }.as_ref().is_some_and(|(last, _)| last.same(&waker)) {
					// SAFETY: the caller gave it, and nothing else holds it.
					unsafe { waker.release() };
				}
				head.polling.store(false, Ordering::Release);
				// SAFETY: the caller's contract.
				return unsafe { failed(status, refused, what) };
			}
		}
		if head.polling.swap(true, Ordering::Acquire) {
			fail(what, "a future is polled concurrently");
		}
		// SAFETY: the flag makes this the only access, and `new` made the allocation.
		let result = unsafe { (head.poll)(future, waker, outs, err, what) };
		// SAFETY: as above; the allocation outlives the poll.
		unsafe { &*future }.polling.store(false, Ordering::Release);
		result
	}

	/// Make the future's first poll, with no waker: on `READY` or `ERROR` the result is
	/// written, the future freed, and null written through `handle`; on `PENDING` its
	/// handle goes there, for C to poll with a waker and free.
	///
	/// # Safety
	/// `future` is a handle `new` just made, `handle` is null, which aborts, or points
	/// where a handle goes, and each of `outs` and `err` is null or points where the function's result
	/// types go.
	pub unsafe fn start(
		future: *mut Self,
		outs: &[*mut c_void],
		err: *mut c_void,
		handle: *mut *mut Self,
		what: &str,
	) -> i32 {
		if handle.is_null() {
			fail(what, "a future's handle pointer is null");
		}
		let none = Waker {
			wake: None,
			drop: None,
			data: std::ptr::null_mut(),
		};
		// SAFETY: the caller's contract; the future was made on this thread, or one this
		// call runs on.
		let result = unsafe { Self::poll(future, none, outs, err, std::ptr::null_mut(), what) };
		match result {
			// SAFETY: the caller's contract.
			PENDING => unsafe { handle.write(future) },
			// SAFETY: it finished; null tells C there is no handle, and `free` takes it.
			_ => unsafe {
				handle.write(std::ptr::null_mut());
				Self::free(future)
			},
		}
		result
	}

	/// Drop the future, cancelling it if it has not finished.
	///
	/// # Safety
	/// `future` is null or a live future handle, freed once and not being polled.
	pub unsafe fn free(future: *mut Self) {
		if future.is_null() {
			return;
		}
		// SAFETY: the caller's contract.
		let owner = unsafe { &(*future).owner };
		// A future that is not `Send` drops on its own thread.
		if let Some(owner) = owner.as_ref().filter(|owner| !owner.is_here()) {
			// SAFETY: the future is the owner's.
			unsafe { Dropped::new(Freed(future), owner.clone(), "a future") }.send();
			return;
		}
		// SAFETY: the caller's contract.
		unsafe { ((*future).drop)(future) };
	}
}

/// A future freed on another thread than its own, which drops it.
struct Freed(*mut Future);

impl Drop for Freed {
	fn drop(&mut self) {
		// SAFETY: `Future::free`'s contract, on the future's own thread.
		unsafe { ((*self.0).drop)(self.0) };
	}
}

/// [`Future`]'s poll of a `Typed<F, L>`.
///
/// # Safety
/// `future` is a live `Typed<F, L>` that only this call accesses, and each of `outs` and
/// `err` is null or points where the function's result types go.
unsafe fn poll_typed<F: std::future::Future, L: Fn(F::Output, &[*mut c_void], *mut c_void) -> i32>(
	future: *mut Future,
	waker: Waker,
	outs: &[*mut c_void],
	err: *mut c_void,
	what: &str,
) -> i32 {
	// SAFETY: the caller's contract.
	let Typed { head, future, lower } = unsafe { &mut *future.cast::<Typed<F, L>>() };
	let Some(inner) = future.as_mut() else {
		fail(what, "a future is polled after it finished");
	};
	if !head.waker.as_ref().is_some_and(|(last, _)| last.same(&waker)) {
		head.waker = Some((waker, waker.lift()));
	}
	let (_, rust) = head.waker.as_ref().expect("set above");
	let mut cx = Context::from_waker(rust);
	// SAFETY: the allocation never moves, and the future drops in place.
	match unsafe { Pin::new_unchecked(inner) }.poll(&mut cx) {
		std::task::Poll::Pending => PENDING,
		std::task::Poll::Ready(value) => {
			*future = None;
			head.waker = None;
			lower(value, outs, err)
		}
	}
}

/// [`Future`]'s drop of a `Typed<F, L>`.
///
/// # Safety
/// `future` is a live `Typed<F, L>`, dropped once.
unsafe fn drop_typed<F, L>(future: *mut Future) {
	// SAFETY: the caller's contract; `new` boxed it.
	drop(unsafe { Box::from_raw(future.cast::<Typed<F, L>>()) });
}

/// Write `value` through `out` if the caller wants it, or drop it.
///
/// # Safety
/// `out` is null or points where a `T` goes.
pub unsafe fn write<T>(out: *mut c_void, value: impl FnOnce() -> T) {
	if !out.is_null() {
		// SAFETY: the caller's contract.
		unsafe { out.cast::<T>().write(value()) };
	}
}
