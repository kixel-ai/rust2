//! The runtime the Python glue's extension functions share: the conversions between
//! Python's values and the C ABI's, the handles objects own, and the queue a future's
//! and a waker's wakes reach the event loop through.
//!
//! The glue holds the GIL only while it touches Python: it releases it around each call
//! into the C ABI, so Rust may call Python back from any thread, as a lent closure or a
//! trait method, while the caller waits. A wake never takes the GIL: it queues a token
//! and writes a byte to a socket the event loop reads, and the loop does the rest. A wake
//! on the loop's own thread, during a call, writes nothing: the call schedules the loop's
//! dispatch once it returns.

use std::cell::{Cell, RefCell, UnsafeCell};
use std::ffi::{CStr, c_char, c_int, c_void};
use std::io::{Read, Write};
use std::ptr::{NonNull, null, null_mut};
use std::sync::atomic::{AtomicBool, AtomicI64, AtomicPtr, AtomicU8, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

pub use pyo3_ffi::*;

use crate::rt;

/// A Python exception is set, for the caller to return null with.
pub struct Raised;

pub type R<T> = Result<T, Raised>;

/// A value only the thread holding the GIL touches.
pub struct Gil<T>(UnsafeCell<T>);

// SAFETY: every access holds the GIL, which this abi3 extension's interpreters have.
unsafe impl<T> Sync for Gil<T> {}

impl<T> Gil<T> {
	pub const fn new(value: T) -> Self {
		Self(UnsafeCell::new(value))
	}

	/// The value.
	///
	/// # Safety
	/// The caller holds the GIL, and no other reference to the value is live.
	#[allow(clippy::mut_from_ref, reason = "the GIL makes the access exclusive")]
	pub unsafe fn get(&self) -> &mut T {
		// SAFETY: the caller's contract.
		unsafe { &mut *self.0.get() }
	}
}

/// A strong reference, released when dropped.
pub struct Obj(NonNull<PyObject>);

impl Obj {
	/// A new reference a call returned, or the exception it raised.
	///
	/// # Safety
	/// `ptr` is null with an exception set, or a new reference, and the GIL is held.
	pub unsafe fn new(ptr: *mut PyObject) -> R<Self> {
		NonNull::new(ptr).map(Self).ok_or(Raised)
	}

	/// A new reference to `ptr`, which the caller borrows.
	///
	/// # Safety
	/// `ptr` is a live object, and the GIL is held.
	pub unsafe fn borrowed(ptr: *mut PyObject) -> Self {
		// SAFETY: the caller's contract.
		unsafe { Py_IncRef(ptr) };
		Self(NonNull::new(ptr).expect("a live object"))
	}

	pub fn ptr(&self) -> *mut PyObject {
		self.0.as_ptr()
	}

	/// The reference, for a caller that steals it.
	pub fn into_ptr(self) -> *mut PyObject {
		let ptr = self.ptr();
		std::mem::forget(self);
		ptr
	}
}

impl Drop for Obj {
	fn drop(&mut self) {
		// SAFETY: a strong reference, dropped with the GIL held, as every `Obj` is.
		unsafe { Py_DecRef(self.ptr()) };
	}
}

/// Raise `exc` with `msg`.
pub fn raise<T>(exc: *mut PyObject, msg: &str) -> R<T> {
	// SAFETY: the GIL is held, and `exc` is an exception type.
	unsafe {
		let msg = PyUnicode_FromStringAndSize(msg.as_ptr().cast(), msg.len() as Py_ssize_t);
		if !msg.is_null() {
			PyErr_SetObject(exc, msg);
			Py_DecRef(msg);
		}
	}
	Err(Raised)
}

pub fn type_error<T>(msg: &str) -> R<T> {
	// SAFETY: a static exception type.
	raise(unsafe { PyExc_TypeError }, msg)
}

pub fn value_error<T>(msg: &str) -> R<T> {
	// SAFETY: a static exception type.
	raise(unsafe { PyExc_ValueError }, msg)
}

fn overflow<T>(msg: &str) -> R<T> {
	// SAFETY: a static exception type.
	raise(unsafe { PyExc_OverflowError }, msg)
}

/// Run an extension function's body, returning null once it raised.
pub fn wrap(f: impl FnOnce() -> R<Obj>) -> *mut PyObject {
	match f() {
		Ok(obj) => obj.into_ptr(),
		Err(Raised) => null_mut(),
	}
}

/// The `n` arguments of a fast call of `name`.
///
/// # Safety
/// `args` points at `nargs` live objects.
pub unsafe fn args<'a>(args: *mut *mut PyObject, nargs: Py_ssize_t, n: usize, name: &str) -> R<&'a [*mut PyObject]> {
	if nargs as usize != n {
		return type_error(&format!("{name}() takes {n} arguments ({nargs} given)"));
	}
	match n {
		0 => Ok(&[]),
		// SAFETY: the caller's contract.
		n => Ok(unsafe { std::slice::from_raw_parts(args, n) }),
	}
}

/// The arguments of a call of `name`, a function Python calls directly whose parameters
/// are `names`, in their order after `this`, its `self` if it is a method: `n`
/// positional ones at `a`, then one for each name in `kw`. A wrong call raises as Python
/// raises for a `def` with these parameters.
///
/// # Safety
/// `a` points at `n` live objects, then one for each name in `kw`, a tuple of `str` or
/// null.
pub unsafe fn params<const N: usize>(
	this: Option<*mut PyObject>,
	a: *const *mut PyObject,
	n: Py_ssize_t,
	kw: *mut PyObject,
	names: &[&str],
	name: &str,
) -> R<[*mut PyObject; N]> {
	let mut out = [null_mut(); N];
	let skip = match this {
		Some(this) => {
			out[0] = this;
			1
		}
		None => 0,
	};
	let n = n as usize;
	if n > names.len() {
		let (takes, given) = (names.len() + skip, n + skip);
		let s = if takes == 1 { "" } else { "s" };
		let was = if given == 1 { "was" } else { "were" };
		return type_error(&format!("{name}() takes {takes} positional argument{s} but {given} {was} given"));
	}
	// SAFETY: the caller's contract; `a` may be null when it points at nothing.
	unsafe {
		if n != 0 {
			out[skip..skip + n].copy_from_slice(std::slice::from_raw_parts(a, n));
		}
		if !kw.is_null() {
			for k in 0..PyTuple_Size(kw) {
				let mut len = 0;
				let key = PyUnicode_AsUTF8AndSize(PyTuple_GetItem(kw, k), &mut len);
				if key.is_null() {
					return Err(Raised);
				}
				let key = std::slice::from_raw_parts(key.cast::<u8>(), len as usize);
				let key = String::from_utf8_lossy(key);
				let Some(p) = names.iter().position(|name| *name == key) else {
					return type_error(&format!("{name}() got an unexpected keyword argument '{key}'"));
				};
				if !out[skip + p].is_null() {
					return type_error(&format!("{name}() got multiple values for argument '{key}'"));
				}
				out[skip + p] = *a.add(n + k as usize);
			}
		}
	}
	let missing: Vec<String> = names
		.iter()
		.zip(&out[skip..])
		.filter(|(_, arg)| arg.is_null())
		.map(|(name, _)| format!("'{name}'"))
		.collect();
	if let Some((last, rest)) = missing.split_last() {
		let (s, list) = match rest.len() {
			0 => ("", last.clone()),
			1 => ("s", format!("{} and {last}", rest[0])),
			_ => ("s", format!("{}, and {last}", rest.join(", "))),
		};
		return type_error(&format!(
			"{name}() missing {} required positional argument{s}: {list}",
			missing.len()
		));
	}
	Ok(out)
}

/// Call into the C ABI with the GIL released, so Rust may call Python back from any
/// thread while this one waits, then resume what it woke on this thread's loop.
pub fn call<T>(f: impl FnOnce() -> T) -> T {
	// A call inside another keeps its loop: none starts on a thread one runs on.
	let outer = RUNNING.get();
	if outer == 0 && WAITS.with_borrow(|waits| waits.as_ref().is_some_and(|w| w.load(Ordering::Relaxed) != 0)) {
		let running = running_loop();
		if running == 0 {
			// Until a loop polls or takes here again, which it does once it runs.
			WAITS.take();
		}
		RUNNING.set(running);
	}
	// SAFETY: this thread holds the GIL, and touches no Python object until it retakes it.
	let state = unsafe { PyEval_SaveThread() };
	let result = f();
	// SAFETY: released just above.
	unsafe { PyEval_RestoreThread(state) };
	RUNNING.set(outer);
	if !RESUMES.with_borrow(Vec::is_empty) {
		resume();
	}
	result
}

thread_local! {
	/// The count of what waits on the event loop that last polled or took on this thread,
	/// until a call finds none running: a call looks for the loop running only while
	/// something waits, as only then may it wake something on the loop.
	static WAITS: RefCell<Option<Arc<AtomicUsize>>> = const { RefCell::new(None) };
	/// The address of the event loop running on this thread, during a call, else `0`.
	static RUNNING: Cell<usize> = const { Cell::new(0) };
	/// The queues woken on their loop's thread during the calls it is in.
	static RESUMES: RefCell<Vec<Arc<Queue>>> = const { RefCell::new(Vec::new()) };
}

static GET_RUNNING_LOOP: std::sync::OnceLock<Static> = std::sync::OnceLock::new();

/// The address of the event loop running on this thread, or `0`: none is while an
/// exception is set, which a call must not run Python over.
fn running_loop() -> usize {
	// SAFETY: the GIL is held, and `asyncio._get_running_loop` neither raises nor runs
	// Python code.
	unsafe {
		if !PyErr_Occurred().is_null() {
			return 0;
		}
		let get = GET_RUNNING_LOOP.get().expect("loaded").ptr();
		let Ok(running) = Obj::new(PyObject_CallNoArgs(get)) else {
			PyErr_Clear();
			return 0;
		};
		match is_none(running.ptr()) {
			true => 0,
			false => running.ptr() as usize,
		}
	}
}

/// Schedule the dispatch of each queue a call woke on its loop's thread, as the loop's
/// next callback, or else through its socket as a wake from another thread is.
#[cold]
fn resume() {
	let woken = RESUMES.with_borrow_mut(std::mem::take);
	let resume = ty(T_RESUME);
	// SAFETY: the GIL is held. An exception the caller has set is kept aside while Python
	// runs, and put back after.
	unsafe {
		let (mut kind, mut value, mut trace) = (null_mut(), null_mut(), null_mut());
		PyErr_Fetch(&mut kind, &mut value, &mut trace);
		let running = match Obj::new(PyObject_CallNoArgs(resume)) {
			Ok(queue) => is_type(queue.ptr(), &QUEUE).then(|| queue_of(queue.ptr()).ok()).flatten(),
			Err(Raised) => {
				// Nothing is lost: each queue's socket signals it instead, and an interrupt
				// its Python ran into is raised again at the thread's next chance.
				if PyErr_ExceptionMatches(PyExc_KeyboardInterrupt) != 0 {
					PyErr_Clear();
					PyErr_SetInterrupt();
				} else {
					PyErr_WriteUnraisable(resume);
				}
				None
			}
		};
		for queue in woken {
			if !running.as_ref().is_some_and(|running| Arc::ptr_eq(running, &queue)) {
				queue.signal();
			}
		}
		PyErr_Restore(kind, value, trace);
	}
}

/// Run `f` calling into Python, from whatever thread Rust calls on, holding the GIL; an
/// exception it raises aborts with what it was, since Rust has nowhere to unwind it to.
pub fn upcall<T>(what: &str, f: impl FnOnce() -> R<T>) -> T {
	// SAFETY: any thread may take the GIL.
	let gil = unsafe { PyGILState_Ensure() };
	match f() {
		Ok(value) => {
			// SAFETY: taken above.
			unsafe { PyGILState_Release(gil) };
			value
		}
		Err(Raised) => {
			// SAFETY: the GIL is held, and an exception is set.
			unsafe { PyErr_Print() };
			rt::fail(what, "a call into Python raised")
		}
	}
}

/// A Python string interned once, such as a field's name.
pub struct Key(&'static CStr, AtomicPtr<PyObject>);

impl Key {
	pub const fn new(name: &'static CStr) -> Self {
		Self(name, AtomicPtr::new(null_mut()))
	}

	/// The string, which lives as long as the interpreter.
	pub fn get(&self) -> R<*mut PyObject> {
		let ptr = self.1.load(Ordering::Relaxed);
		if !ptr.is_null() {
			return Ok(ptr);
		}
		// SAFETY: the GIL is held.
		let ptr = unsafe { PyUnicode_InternFromString(self.0.as_ptr()) };
		if ptr.is_null() {
			return Err(Raised);
		}
		self.1.store(ptr, Ordering::Relaxed);
		Ok(ptr)
	}
}

/// What a call borrows from Python until it returns: the objects and copies whose memory
/// it reads, the handles it holds and those it moves out of, and what it
/// gives Rust, taken back if the call never happens.
#[derive(Default)]
pub struct Scope {
	keep: Vec<Obj>,
	arrays: Vec<Box<dyn std::any::Any>>,
	/// The handles held, each with whether the call moves its value out. Inline, as a
	/// call holds few, so holding one allocates nothing.
	held: smallvec::SmallVec<[(Obj, bool); 4]>,
	/// The objects Rust lends a callback, ended once it returns.
	lent: Vec<Obj>,
	/// Python implementations and waker nodes given to Rust, taken back unless the call
	/// happens.
	given: Vec<Obj>,
	nodes: Vec<rt::Waker>,
}

impl Scope {
	/// Keep `obj` until the call returns, returning it borrowed.
	pub fn keep(&mut self, obj: Obj) -> *mut PyObject {
		let ptr = obj.ptr();
		self.keep.push(obj);
		ptr
	}

	/// Note that the call happened: what it moved out of is closed, and what it was given
	/// is Rust's.
	pub fn done(&mut self) {
		for (held, moved) in &self.held {
			if *moved {
				// SAFETY: a handle this scope holds, so live.
				unsafe { close(held.ptr().cast()) };
			}
		}
		while let Some(given) = self.given.pop() {
			given.into_ptr();
		}
		self.nodes.clear();
	}

	/// The handles held, which a future keeps until it ends.
	pub fn take_held(&mut self) -> Vec<Obj> {
		self.held.drain(..).map(|(held, _)| held).collect()
	}
}

// Popping rather than draining, as each call drops a scope whose lists are mostly empty.
impl Drop for Scope {
	fn drop(&mut self) {
		while let Some(waker) = self.nodes.pop() {
			// SAFETY: a reference the call never took.
			unsafe { node_drop(waker.data) };
		}
		while let Some(lent) = self.lent.pop() {
			// SAFETY: an object [`lent`] made.
			unsafe { end(lent.ptr().cast()) };
		}
		while let Some((held, _)) = self.held.pop() {
			// SAFETY: a handle this scope holds.
			unsafe { release(held.ptr().cast()) };
		}
	}
}

/// The objects the generated Python registers, by the index the glue names each with:
/// the runtime's, then the API's classes.
static TYPES: Gil<Vec<Obj>> = Gil::new(Vec::new());

/// The plain enums' members, by class, with their discriminants.
static MEMBERS: Gil<Vec<Option<Vec<(i64, Obj)>>>> = Gil::new(Vec::new());

/// The runtime's entries, at the start of the registry: the awaitable a pending future
/// is, `datetime.timedelta`, and `_resume`, which schedules the running loop's dispatch.
pub const T_FUTURE: usize = 0;
const T_TIMEDELTA: usize = 1;
const T_RESUME: usize = 2;

/// The class registered at `index`.
pub fn ty(index: usize) -> *mut PyObject {
	// SAFETY: the GIL is held, and the module registered its classes when it loaded.
	(unsafe { TYPES.get() })[index].ptr()
}

/// `_register(*classes)`: note the classes the glue makes values of, and set the glue's
/// members on them.
unsafe extern "C" fn register(_: *mut PyObject, args: *mut *mut PyObject, nargs: Py_ssize_t) -> *mut PyObject {
	// SAFETY: Python's call.
	let args = unsafe { std::slice::from_raw_parts(args, nargs as usize) };
	// SAFETY: the GIL is held, and each argument a live class.
	unsafe {
		*TYPES.get() = args.iter().map(|&class| Obj::borrowed(class)).collect();
		*MEMBERS.get() = (0..args.len()).map(|_| None).collect();
	}
	wrap(|| {
		// SAFETY: the GIL is held, and each member's index one `_register` was passed.
		unsafe { install() }?;
		none()
	})
}

/// Set each of the glue's members on its class: a method as a method descriptor, which
/// CPython calls without binding it, as it does a builtin type's.
unsafe fn install() -> R<()> {
	// SAFETY: the GIL is held, and `INSTALL` lives as long as the interpreter.
	unsafe {
		let mut staticmethod = None;
		for member in INSTALL.get().iter_mut() {
			let (class, name, value) = match member {
				Member::Method(class, def) => (*class, def.ml_name, PyDescr_NewMethod(ty(*class).cast(), def)),
				Member::Get(class, def) => (*class, def.name, PyDescr_NewGetSet(ty(*class).cast(), def)),
				Member::Static(class, def) => {
					let f = Obj::new(PyCMethod_New(def, null_mut(), null_mut(), null_mut()));
					if staticmethod.is_none() {
						let builtins = Obj::new(PyImport_ImportModule(c"builtins".as_ptr()))?;
						staticmethod = Some(Obj::new(PyObject_GetAttrString(builtins.ptr(), c"staticmethod".as_ptr()))?);
					}
					let wrap = staticmethod.as_ref().expect("just set").ptr();
					(*class, def.ml_name, call_py(wrap, vec![f]).map_or(null_mut(), Obj::into_ptr))
				}
			};
			let value = Obj::new(value)?;
			if PyObject_SetAttrString(ty(class), name, value.ptr()) != 0 {
				return Err(Raised);
			}
		}
	}
	Ok(())
}

pub fn none() -> R<Obj> {
	// SAFETY: `None` is immortal; a reference to it is still counted on older versions.
	Ok(unsafe { Obj::borrowed(Py_None()) })
}

pub fn is_none(obj: *mut PyObject) -> bool {
	// SAFETY: comparing addresses.
	obj == unsafe { Py_None() }
}

/// Whether `obj` is an instance of the class registered at `index`.
pub fn is(obj: *mut PyObject, index: usize) -> bool {
	// SAFETY: the GIL is held, and `obj` is live.
	unsafe { PyObject_TypeCheck(obj, ty(index).cast()) != 0 }
}

/// A `TypeError` naming the class registered at `index` and what came instead.
fn expected<T>(obj: *mut PyObject, index: usize) -> R<T> {
	expected_named(obj, &type_name(ty(index)))
}

fn expected_named<T>(obj: *mut PyObject, want: &str) -> R<T> {
	// SAFETY: `obj` is live.
	let got = type_name(unsafe { Py_TYPE(obj) }.cast());
	type_error(&format!("expected {want}, got {got}"))
}

/// A class's name, for an error.
fn type_name(class: *mut PyObject) -> String {
	// SAFETY: the GIL is held, and `class` is a type.
	unsafe {
		let Ok(name) = Obj::new(PyType_GetName(class.cast())) else {
			PyErr_Clear();
			return "?".to_string();
		};
		let mut len = 0;
		let ptr = PyUnicode_AsUTF8AndSize(name.ptr(), &mut len);
		if ptr.is_null() {
			PyErr_Clear();
			return "?".to_string();
		}
		String::from_utf8_lossy(std::slice::from_raw_parts(ptr.cast(), len as usize)).into_owned()
	}
}

/// The attribute `key` of `obj`, kept for the call.
pub fn attr(obj: *mut PyObject, key: &Key, s: &mut Scope) -> R<*mut PyObject> {
	// SAFETY: the GIL is held, and `obj` is live.
	let value = unsafe { Obj::new(PyObject_GetAttr(obj, key.get()?)) }?;
	Ok(s.keep(value))
}

pub fn bool_in(obj: *mut PyObject) -> R<bool> {
	// SAFETY: comparing addresses.
	unsafe {
		if obj == Py_True() {
			Ok(true)
		} else if obj == Py_False() {
			Ok(false)
		} else {
			expected_named(obj, "bool")
		}
	}
}

pub fn bool_out(value: bool) -> R<Obj> {
	// SAFETY: the GIL is held.
	unsafe { Obj::new(PyBool_FromLong(value as _)) }
}

/// An integer of Rust's type `T`, from any `int` or object with `__index__` in its range.
pub fn int_in<T: TryFrom<i128>>(obj: *mut PyObject) -> R<T> {
	// SAFETY: the GIL is held, and `obj` is live.
	let value = unsafe {
		let index = Obj::new(PyNumber_Index(obj))?;
		let mut overflow = 0;
		let v = PyLong_AsLongLongAndOverflow(index.ptr(), &mut overflow);
		match overflow {
			0 if v == -1 && !PyErr_Occurred().is_null() => return Err(Raised),
			0 => v as i128,
			// Past `i64`: the `u64`s above it, or out of every range.
			_ => {
				let v = PyLong_AsUnsignedLongLong(index.ptr());
				if v == u64::MAX && !PyErr_Occurred().is_null() {
					PyErr_Clear();
					return overflow_int::<T>();
				}
				v as i128
			}
		}
	};
	T::try_from(value).or_else(|_| overflow_int::<T>())
}

fn overflow_int<T>() -> R<T> {
	let name = std::any::type_name::<T>();
	overflow(&format!("int out of range for Rust's {name}"))
}

pub fn int_out(value: i64) -> R<Obj> {
	// SAFETY: the GIL is held.
	unsafe { Obj::new(PyLong_FromLongLong(value)) }
}

pub fn uint_out(value: u64) -> R<Obj> {
	// SAFETY: the GIL is held.
	unsafe { Obj::new(PyLong_FromUnsignedLongLong(value)) }
}

/// A `u128`, from any `int` or object with `__index__` in its range.
pub fn u128_in(obj: *mut PyObject) -> R<rt::U128> {
	// SAFETY: the GIL is held, and `obj` is live.
	unsafe {
		let index = Obj::new(PyNumber_Index(obj))?;
		let shift = uint_out(64)?;
		let high = Obj::new(PyNumber_Rshift(index.ptr(), shift.ptr()))?;
		// A negative int's high half is negative, which raises here as one past `u64` does.
		let hi = PyLong_AsUnsignedLongLong(high.ptr());
		if hi == u64::MAX && !PyErr_Occurred().is_null() {
			PyErr_Clear();
			return overflow_int::<u128>();
		}
		let lo = PyLong_AsUnsignedLongLongMask(index.ptr());
		if lo == u64::MAX && !PyErr_Occurred().is_null() {
			return Err(Raised);
		}
		Ok(rt::U128 { lo, hi })
	}
}

pub fn u128_out(value: rt::U128) -> R<Obj> {
	let lo = uint_out(value.lo)?;
	if value.hi == 0 {
		return Ok(lo);
	}
	// SAFETY: the GIL is held, and each operand is live.
	unsafe {
		let high = Obj::new(PyNumber_Lshift(uint_out(value.hi)?.ptr(), uint_out(64)?.ptr()))?;
		Obj::new(PyNumber_Or(high.ptr(), lo.ptr()))
	}
}

pub fn float_in(obj: *mut PyObject) -> R<f64> {
	// SAFETY: the GIL is held, and `obj` is live.
	let value = unsafe { PyFloat_AsDouble(obj) };
	// SAFETY: as above.
	if value == -1.0 && !unsafe { PyErr_Occurred() }.is_null() {
		return Err(Raised);
	}
	Ok(value)
}

pub fn float_out(value: f64) -> R<Obj> {
	// SAFETY: the GIL is held.
	unsafe { Obj::new(PyFloat_FromDouble(value)) }
}

/// A `char`, from a `str` of one code point that is a Unicode scalar value.
pub fn char_in(obj: *mut PyObject) -> R<u32> {
	// SAFETY: the GIL is held, and `obj` is live.
	unsafe {
		if PyUnicode_Check(obj) == 0 || PyUnicode_GetLength(obj) != 1 {
			return expected_named(obj, "a str of one character");
		}
		let code = PyUnicode_ReadChar(obj, 0);
		match char::from_u32(code) {
			Some(_) => Ok(code),
			None => value_error("a surrogate is no Rust char"),
		}
	}
}

pub fn char_out(code: u32) -> R<Obj> {
	// SAFETY: the GIL is held.
	unsafe { Obj::new(PyUnicode_FromOrdinal(code as c_int)) }
}

/// A view of a `str`'s UTF-8, which Python caches in the string, so it lives as long as
/// `obj`: the caller keeps `obj` for the call.
pub fn str_in(obj: *mut PyObject) -> R<rt::Buf> {
	// SAFETY: the GIL is held, and `obj` is live.
	unsafe {
		if PyUnicode_Check(obj) == 0 {
			return expected_named(obj, "str");
		}
		let mut len = 0;
		let ptr = PyUnicode_AsUTF8AndSize(obj, &mut len);
		if ptr.is_null() {
			return Err(Raised);
		}
		Ok(rt::Buf {
			ptr: ptr.cast(),
			len: len as usize,
			owner: null_mut(),
		})
	}
}

/// A `str` of a buffer of UTF-8 Rust returned, freeing it.
pub fn str_out(buf: rt::Buf) -> R<Obj> {
	// SAFETY: a buffer Rust returned, freed once copied.
	unsafe {
		let str = Obj::new(PyUnicode_FromStringAndSize(buf.ptr.cast(), buf.len as Py_ssize_t));
		buf.free();
		str
	}
}

/// The bytes of `bytes`, read in place, or of any other object with the buffer protocol,
/// such as a `bytearray` or a `memoryview`, copied: the call runs without the GIL, so
/// another thread could write storage that is not immutable while Rust reads it.
pub fn bytes_in(obj: *mut PyObject, s: &mut Scope) -> R<rt::Buf> {
	// SAFETY: the GIL is held, and `obj` is live.
	unsafe {
		if PyBytes_Check(obj) != 0 {
			let mut ptr: *mut c_char = null_mut();
			let mut len = 0;
			if PyBytes_AsStringAndSize(obj, &mut ptr, &mut len) != 0 {
				return Err(Raised);
			}
			// A `bytes` never changes, and the caller keeps it for the call.
			return Ok(rt::Buf {
				ptr: ptr.cast_const().cast(),
				len: len as usize,
				owner: null_mut(),
			});
		}
		// SAFETY: `Py_buffer` is plain data, which the call fills.
		let mut view: Py_buffer = std::mem::zeroed();
		if PyObject_GetBuffer(obj, &mut view, PyBUF_SIMPLE) != 0 {
			return Err(Raised);
		}
		let copy = match view.len {
			0 => Vec::new(),
			len => std::slice::from_raw_parts(view.buf.cast_const().cast::<u8>(), len as usize).to_vec(),
		};
		PyBuffer_Release(&mut view);
		let buf = rt::Buf {
			ptr: copy.as_ptr(),
			len: copy.len(),
			owner: null_mut(),
		};
		s.arrays.push(Box::new(copy));
		Ok(buf)
	}
}

/// Bytes Rust returned, as a read-only `memoryview` that lends them without a copy until
/// it and every view of it are released.
pub fn bytes_out(buf: rt::Buf) -> R<Obj> {
	// A view Rust lent only for a call is copied into a buffer of its own.
	let buf = match buf.owner.is_null() {
		// SAFETY: a buffer Rust lent for the call.
		true => rt::Buf::owned(bytes::Bytes::copy_from_slice(unsafe { buf.bytes("python") })),
		false => buf,
	};
	// SAFETY: the GIL is held, and the owner type's slots are this module's.
	unsafe {
		let owner = PyType_GenericAlloc(BYTES.get().expect("loaded").ptr().cast(), 0);
		let Ok(owner) = Obj::new(owner) else {
			buf.free();
			return Err(Raised);
		};
		(*owner.ptr().cast::<Bytes>()).buf = buf;
		Obj::new(PyMemoryView_FromObject(owner.ptr()))
	}
}

/// Bytes Rust lends a callback for its call, copied into `bytes`.
pub fn lent_bytes(buf: rt::Buf) -> R<Obj> {
	// SAFETY: the GIL is held, and a buffer Rust lent for the call.
	unsafe {
		let bytes = buf.bytes("python");
		Obj::new(PyBytes_FromStringAndSize(bytes.as_ptr().cast(), bytes.len() as Py_ssize_t))
	}
}

/// A path's bytes, from a `str`, `bytes`, or `os.PathLike`, as `os.fsencode` encodes it,
/// so a name a Unix path holds that is not UTF-8 crosses as it came.
pub fn path_in(obj: *mut PyObject, s: &mut Scope) -> R<rt::Buf> {
	let mut encoded: *mut PyObject = null_mut();
	// SAFETY: the GIL is held, and `obj` is live; the converter writes a new `bytes`.
	unsafe {
		if PyUnicode_FSConverter(obj, (&raw mut encoded).cast()) == 0 {
			return Err(Raised);
		}
		let encoded = s.keep(Obj::new(encoded)?);
		let mut ptr: *mut c_char = null_mut();
		let mut len = 0;
		if PyBytes_AsStringAndSize(encoded, &mut ptr, &mut len) != 0 {
			return Err(Raised);
		}
		Ok(rt::Buf {
			ptr: ptr.cast_const().cast(),
			len: len as usize,
			owner: null_mut(),
		})
	}
}

/// A path Rust returned, as a `str` `os.fsdecode` decodes, freeing it.
pub fn path_out(buf: rt::Buf) -> R<Obj> {
	// SAFETY: a buffer Rust returned, freed once decoded.
	unsafe {
		let path = Obj::new(PyUnicode_DecodeFSDefaultAndSize(buf.ptr.cast(), buf.len as Py_ssize_t));
		buf.free();
		path
	}
}

static DAYS: Key = Key::new(c"days");
static SECONDS: Key = Key::new(c"seconds");
static MICROSECONDS: Key = Key::new(c"microseconds");
static MAX: Key = Key::new(c"max");

/// The days in `timedelta.max`, past which [`duration_out`] gives it.
const MAX_DAYS: u64 = 999_999_999;

/// A `Duration`, from a `datetime.timedelta` that is not negative: `timedelta.max`, which
/// [`duration_out`] gives for any longer one, is `Duration::MAX`.
pub fn duration_in(obj: *mut PyObject) -> R<rt::Duration> {
	if !is(obj, T_TIMEDELTA) {
		return expected(obj, T_TIMEDELTA);
	}
	let mut s = Scope::default();
	let days: u64 = match int_in::<i64>(attr(obj, &DAYS, &mut s)?)? {
		days if days < 0 => return value_error("a negative timedelta has no Rust Duration"),
		days => days as u64,
	};
	let seconds: u64 = int_in(attr(obj, &SECONDS, &mut s)?)?;
	let micros: u32 = int_in(attr(obj, &MICROSECONDS, &mut s)?)?;
	if (days, seconds, micros) == (MAX_DAYS, 86_399, 999_999) {
		return Ok(rt::Duration::lower(std::time::Duration::MAX));
	}
	Ok(rt::Duration {
		secs: days * 86_400 + seconds,
		nanos: micros * 1_000,
	})
}

/// A `datetime.timedelta`, which holds whole microseconds up to 999,999,999 days: Rust's
/// nanoseconds are truncated, and a longer one is `timedelta.max`, as Rust's
/// `Duration::MAX` stands for forever, which [`duration_in`] gives back.
pub fn duration_out(duration: rt::Duration) -> R<Obj> {
	let class = ty(T_TIMEDELTA);
	// SAFETY: the GIL is held.
	unsafe {
		if duration.secs / 86_400 > MAX_DAYS {
			return Obj::new(PyObject_GetAttr(class, MAX.get()?));
		}
		let args = tuple(vec![
			int_out(0),
			uint_out(duration.secs),
			uint_out(u64::from(duration.nanos / 1_000)),
		])?;
		Obj::new(PyObject_CallObject(class, args.ptr()))
	}
}

/// Which addresses a socket address type holds.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Family {
	Any,
	V4,
	V6,
}

/// A socket address, from a tuple as Python's `socket` module writes one: `(host, port)`,
/// or for IPv6, `(host, port, flowinfo, scope_id)`, its host a numeric address.
pub fn addr_in(obj: *mut PyObject, family: Family) -> R<rt::SocketAddr> {
	// SAFETY: the GIL is held, and `obj` is live.
	let len = unsafe {
		if PyTuple_Check(obj) == 0 {
			return expected_named(obj, "a (host, port) tuple");
		}
		PyTuple_Size(obj)
	};
	if len != 2 && len != 4 {
		return value_error("a socket address is (host, port) or (host, port, flowinfo, scope_id)");
	}
	// SAFETY: in range.
	let item = |n| unsafe { PyTuple_GetItem(obj, n) };
	let host = str_in(item(0))?;
	// SAFETY: a view of the host's UTF-8, which the tuple keeps.
	let host = unsafe { host.str("python") };
	let Ok(ip) = host.parse::<std::net::IpAddr>() else {
		return value_error(&format!("{host:?} is no numeric IP address"));
	};
	let port: u16 = int_in(item(1))?;
	let (flowinfo, scope_id) = match len {
		4 => (int_in::<u32>(item(2))?, int_in::<u32>(item(3))?),
		_ => (0, 0),
	};
	let addr = match ip {
		std::net::IpAddr::V4(_) if len == 4 => return value_error("an IPv4 address takes no flowinfo or scope_id"),
		std::net::IpAddr::V4(ip) => std::net::SocketAddr::from((ip, port)),
		std::net::IpAddr::V6(ip) => std::net::SocketAddrV6::new(ip, port, flowinfo, scope_id).into(),
	};
	match (family, addr) {
		(Family::V4, std::net::SocketAddr::V6(_)) => value_error("an IPv6 address has no Rust SocketAddrV4"),
		(Family::V6, std::net::SocketAddr::V4(_)) => value_error("an IPv4 address has no Rust SocketAddrV6"),
		_ => Ok(rt::SocketAddr::lower(addr)),
	}
}

/// A socket address as Python's `socket` module writes one.
pub fn addr_out(addr: rt::SocketAddr) -> R<Obj> {
	let text = |ip: &dyn std::fmt::Display| {
		let text = ip.to_string();
		// SAFETY: the GIL is held.
		unsafe { Obj::new(PyUnicode_FromStringAndSize(text.as_ptr().cast(), text.len() as Py_ssize_t)) }
	};
	match addr.lift("python") {
		std::net::SocketAddr::V4(v4) => tuple(vec![text(v4.ip()), uint_out(v4.port().into())]),
		std::net::SocketAddr::V6(v6) => tuple(vec![
			text(v6.ip()),
			uint_out(v6.port().into()),
			uint_out(v6.flowinfo().into()),
			uint_out(v6.scope_id().into()),
		]),
	}
}

static START: Key = Key::new(c"start");
static STOP: Key = Key::new(c"stop");
static STEP: Key = Key::new(c"step");

/// A `Range`, from a `range` of step 1.
pub fn range_in<T: TryFrom<i128>>(obj: *mut PyObject) -> R<rt::Range<T>> {
	// SAFETY: the GIL is held, and `obj` is live.
	if unsafe { PyObject_TypeCheck(obj, &raw mut PyRange_Type) } == 0 {
		return expected_named(obj, "range");
	}
	let mut s = Scope::default();
	if int_in::<i64>(attr(obj, &STEP, &mut s)?)? != 1 {
		return value_error("a range of a step other than 1 has no Rust Range");
	}
	Ok(rt::Range {
		start: int_in(attr(obj, &START, &mut s)?)?,
		end: int_in(attr(obj, &STOP, &mut s)?)?,
	})
}

pub fn range_out(start: R<Obj>, end: R<Obj>) -> R<Obj> {
	let args = tuple(vec![start, end])?;
	// SAFETY: the GIL is held.
	unsafe { Obj::new(PyObject_CallObject((&raw mut PyRange_Type).cast(), args.ptr())) }
}

/// A tuple of `items`, each made before any error is returned, so each frees what Rust
/// handed over.
pub fn tuple(items: Vec<R<Obj>>) -> R<Obj> {
	// SAFETY: the GIL is held.
	unsafe {
		let tuple = Obj::new(PyTuple_New(items.len() as Py_ssize_t));
		let mut raised = false;
		for (n, item) in items.into_iter().enumerate() {
			match (&tuple, item) {
				(Ok(tuple), Ok(item)) => {
					PyTuple_SetItem(tuple.ptr(), n as Py_ssize_t, item.into_ptr());
				}
				(_, Err(Raised)) => raised = true,
				(Err(Raised), Ok(_)) => {}
			}
		}
		match raised {
			true => Err(Raised),
			false => tuple,
		}
	}
}

/// The item `n` of a tuple of `len`, or a `TypeError`.
pub fn tuple_item(obj: *mut PyObject, n: usize, len: usize) -> R<*mut PyObject> {
	// SAFETY: the GIL is held, and `obj` is live.
	unsafe {
		if PyTuple_Check(obj) == 0 || PyTuple_Size(obj) != len as Py_ssize_t {
			return expected_named(obj, &format!("a tuple of {len}"));
		}
		Ok(PyTuple_GetItem(obj, n as Py_ssize_t))
	}
}

/// A `Vec` of the items any iterable yields, each read by `each`, held for the call.
pub fn list_in<T: Copy + 'static>(
	obj: *mut PyObject,
	s: &mut Scope,
	mut each: impl FnMut(*mut PyObject, &mut Scope) -> R<T>,
) -> R<rt::List<T>> {
	// SAFETY: the GIL is held, and `obj` is live.
	unsafe {
		// A str iterates its characters, which no Rust list of strings means.
		if PyUnicode_Check(obj) != 0 {
			return expected_named(obj, "a list");
		}
		let iter = Obj::new(PyObject_GetIter(obj))?;
		let mut items = Vec::new();
		loop {
			let item = PyIter_Next(iter.ptr());
			if item.is_null() {
				if !PyErr_Occurred().is_null() {
					return Err(Raised);
				}
				break;
			}
			let item = s.keep(Obj::new(item)?);
			items.push(each(item, s)?);
		}
		let items = Box::new(items);
		let list = rt::List {
			ptr: items.as_ptr(),
			len: items.len(),
			owner: null_mut(),
		};
		s.arrays.push(items);
		Ok(list)
	}
}

/// A `list` of the elements of an array Rust returned, each made by `each`, which frees
/// what it holds, before the array is freed.
pub fn list_out<T: Copy + 'static>(list: rt::List<T>, mut each: impl FnMut(T) -> R<Obj>) -> R<Obj> {
	// SAFETY: an array Rust returned, whose elements `each` takes.
	let items: Vec<R<Obj>> = unsafe { list.items("python") }.iter().map(|&item| each(item)).collect();
	// SAFETY: as above, with its elements taken.
	unsafe { rt::List { len: 0, ..list }.free("python", |_| {}) };
	// SAFETY: the GIL is held.
	unsafe {
		let out = Obj::new(PyList_New(items.len() as Py_ssize_t));
		let mut raised = false;
		for (n, item) in items.into_iter().enumerate() {
			match (&out, item) {
				(Ok(out), Ok(item)) => {
					PyList_SetItem(out.ptr(), n as Py_ssize_t, item.into_ptr());
				}
				(_, Err(Raised)) => raised = true,
				(Err(Raised), Ok(_)) => {}
			}
		}
		match raised {
			true => Err(Raised),
			false => out,
		}
	}
}

/// An instance of the class registered at `index` with `fields` set, each made before
/// any error is returned, so each frees what Rust handed over. Made as `object.__new__`
/// makes one and set as `object.__setattr__` sets, so no `__init__` runs.
pub fn record(index: usize, fields: Vec<(&Key, R<Obj>)>) -> R<Obj> {
	let class = ty(index);
	// SAFETY: the GIL is held, and `class` a class of the generated Python.
	unsafe {
		let alloc: allocfunc = std::mem::transmute(PyType_GetSlot(class.cast(), Py_tp_alloc));
		let obj = Obj::new(alloc(class.cast(), 0));
		let mut raised = false;
		for (key, value) in fields {
			match (&obj, value, key.get()) {
				(Ok(obj), Ok(value), Ok(key)) => {
					raised |= PyObject_GenericSetAttr(obj.ptr(), key, value.ptr()) != 0;
				}
				_ => raised = true,
			}
		}
		match raised {
			true => Err(Raised),
			false => obj,
		}
	}
}

/// The field `key` of a record or variant, after checking `obj` is of the class
/// registered at `index`.
pub fn field(obj: *mut PyObject, index: usize, key: &Key, s: &mut Scope) -> R<*mut PyObject> {
	match is(obj, index) {
		true => attr(obj, key, s),
		false => expected(obj, index),
	}
}

/// Check `obj` is of the class registered at `index`.
pub fn check(obj: *mut PyObject, index: usize) -> R<()> {
	match is(obj, index) {
		true => Ok(()),
		false => expected(obj, index),
	}
}

static VALUE: Key = Key::new(c"value");

/// The members of the plain enum registered at `index`, with their discriminants.
fn members(index: usize) -> R<&'static [(i64, Obj)]> {
	// SAFETY: the GIL is held.
	let table = unsafe { MEMBERS.get() };
	if table[index].is_none() {
		let mut members = Vec::new();
		// SAFETY: the GIL is held, and an `Enum` class iterates its members.
		unsafe {
			let iter = Obj::new(PyObject_GetIter(ty(index)))?;
			loop {
				let member = PyIter_Next(iter.ptr());
				if member.is_null() {
					if !PyErr_Occurred().is_null() {
						return Err(Raised);
					}
					break;
				}
				let member = Obj::new(member)?;
				let value = Obj::new(PyObject_GetAttr(member.ptr(), VALUE.get()?))?;
				members.push((int_in::<i64>(value.ptr())?, member));
			}
		}
		table[index] = Some(members);
	}
	Ok(table[index].as_deref().expect("set above"))
}

/// The member of the plain enum registered at `index` whose discriminant is `value`.
pub fn member(index: usize, value: i64) -> R<Obj> {
	match members(index)?.iter().find(|(v, _)| *v == value) {
		// SAFETY: a live member.
		Some((_, member)) => Ok(unsafe { Obj::borrowed(member.ptr()) }),
		None => rt::fail("python", "an enum's discriminant is invalid"),
	}
}

/// The discriminant of `obj`, a member of the plain enum registered at `index`.
pub fn member_in(obj: *mut PyObject, index: usize) -> R<i32> {
	match members(index)?.iter().find(|(_, m)| m.ptr() == obj) {
		Some((value, _)) => Ok(*value as i32),
		None => expected(obj, index),
	}
}

/// Raise the exception class registered at `index`, holding `error`.
pub fn raise_error<T>(index: usize, error: R<Obj>) -> R<T> {
	let args = tuple(vec![error])?;
	// SAFETY: the GIL is held, and the class an exception's.
	unsafe {
		let exc = Obj::new(PyObject_CallObject(ty(index), args.ptr()))?;
		PyErr_SetObject(ty(index), exc.ptr());
	}
	Err(Raised)
}

/// Call `callable` with `args`, each made before any error is returned.
pub fn call_py(callable: *mut PyObject, args: Vec<R<Obj>>) -> R<Obj> {
	let args = tuple(args)?;
	// SAFETY: the GIL is held, and `callable` is live.
	unsafe { Obj::new(PyObject_CallObject(callable, args.ptr())) }
}

/// Call the method `key` of `obj` with `args`.
pub fn call_method(obj: *mut PyObject, key: &Key, args: Vec<R<Obj>>) -> R<Obj> {
	// SAFETY: the GIL is held, and `obj` is live.
	let method = unsafe { Obj::new(PyObject_GetAttr(obj, key.get()?)) }?;
	call_py(method.ptr(), args)
}

/// A Python implementation of a trait, an instance of the abstract class registered at
/// `index`, given to Rust: a reference Rust drops through [`free_callback`], or the scope
/// drops if the call never happens.
pub fn callback(obj: *mut PyObject, index: usize, s: &mut Scope) -> R<*mut c_void> {
	check(obj, index)?;
	// SAFETY: the GIL is held, and `obj` is live.
	let given = unsafe { Obj::borrowed(obj) };
	let ptr = given.ptr();
	s.given.push(given);
	Ok(ptr.cast())
}

/// A callable Rust calls only during the call, as a closure's data: the caller's reference
/// outlives it.
pub fn closure(obj: *mut PyObject) -> R<*mut c_void> {
	// SAFETY: the GIL is held, and `obj` is live.
	match unsafe { PyCallable_Check(obj) } {
		0 => expected_named(obj, "a callable"),
		_ => Ok(obj.cast()),
	}
}

static CLOSE: Key = Key::new(c"close");

/// Let go of a Python implementation of a trait, from any thread: closed first if it has
/// a `close` method.
///
/// # Safety
/// `data` is a reference [`callback`] gave Rust, freed once.
pub unsafe extern "C" fn free_callback(data: *mut c_void) {
	upcall("free", || {
		// SAFETY: the reference Rust held.
		let obj = unsafe { Obj::new(data.cast()) }?;
		// SAFETY: the GIL is held.
		if unsafe { PyObject_HasAttr(obj.ptr(), CLOSE.get()?) } == 1 {
			call_method(obj.ptr(), &CLOSE, Vec::new())?;
		}
		Ok(())
	})
}

/// An object owning a handle Rust returned, or one Rust lends for a call: the instance
/// of a class of the generated Python, which subclasses `Object`.
#[repr(C)]
pub struct Handle {
	ob_base: PyObject,
	raw: *const c_void,
	/// What frees the handle: none for one Rust lends, or once freed.
	free: Option<unsafe extern "C" fn(*const c_void)>,
	/// The calls holding it, each of which it outlives.
	holds: usize,
	closed: bool,
}

/// A new object of the class registered at `index` owning `raw`, which `free` frees.
pub fn object(raw: *const c_void, index: usize, free: unsafe extern "C" fn(*const c_void)) -> R<Obj> {
	handle(raw, index, Some(free))
}

/// [`object`], or `None` for a null handle.
pub fn object_opt(raw: *const c_void, index: usize, free: unsafe extern "C" fn(*const c_void)) -> R<Obj> {
	match raw.is_null() {
		true => none(),
		false => object(raw, index, free),
	}
}

/// An object of the class registered at `index` for a handle Rust lends a callback,
/// ended once the scope `s` of the call ends.
pub fn lent(raw: *const c_void, index: usize, s: &mut Scope) -> R<Obj> {
	let obj = handle(raw, index, None)?;
	// SAFETY: the GIL is held, and `obj` is live.
	s.lent.push(unsafe { Obj::borrowed(obj.ptr()) });
	Ok(obj)
}

fn handle(raw: *const c_void, index: usize, free: Option<unsafe extern "C" fn(*const c_void)>) -> R<Obj> {
	let class = ty(index);
	// SAFETY: the GIL is held, and `class` subclasses `Object` with no fields of its own.
	unsafe {
		let obj = PyType_GenericAlloc(class.cast(), 0);
		let Ok(obj) = Obj::new(obj) else {
			if let Some(free) = free {
				free(raw);
			}
			return Err(Raised);
		};
		let handle = obj.ptr().cast::<Handle>();
		(*handle).raw = raw;
		(*handle).free = free;
		(*handle).holds = 0;
		(*handle).closed = false;
		Ok(obj)
	}
}

/// The handle `obj` owns, an object of the class registered at `index`, held for the
/// call; `moved` when the call moves its value out, after which `obj` is closed.
pub fn hold(obj: *mut PyObject, index: usize, s: &mut Scope, moved: bool) -> R<*const c_void> {
	check(obj, index)?;
	let handle = obj.cast::<Handle>();
	// SAFETY: an object of a class that subclasses `Object`, and the GIL is held.
	unsafe {
		if (*handle).closed {
			return value_error(&format!("{} is closed", type_name(ty(index))));
		}
		(*handle).holds += 1;
		s.held.push((Obj::borrowed(obj), moved));
		Ok((*handle).raw)
	}
}

/// [`hold`], or a null handle for `None`.
pub fn hold_opt(obj: *mut PyObject, index: usize, s: &mut Scope, moved: bool) -> R<*const c_void> {
	match is_none(obj) {
		true => Ok(null()),
		false => hold(obj, index, s, moved),
	}
}

/// Let go of a call's hold, freeing the handle if it was closed meanwhile.
///
/// # Safety
/// `handle` is live, and held.
unsafe fn release(handle: *mut Handle) {
	// SAFETY: the caller's contract.
	unsafe {
		(*handle).holds -= 1;
		if (*handle).closed && (*handle).holds == 0 {
			free_now(handle);
		}
	}
}

/// Close a handle: freed now, or once the last call holding it returns.
///
/// # Safety
/// `handle` is live.
pub unsafe fn close(handle: *mut Handle) {
	// SAFETY: the caller's contract.
	unsafe {
		if (*handle).closed {
			return;
		}
		(*handle).closed = true;
		if (*handle).holds == 0 {
			free_now(handle);
		}
	}
}

/// End a lend, closing its handle, which Rust frees next: a call still holding it would
/// reach the freed handle, so it fails loud, as Rust's own check does once a call borrowed.
///
/// # Safety
/// `handle` is live, and one [`lent`] made.
unsafe fn end(handle: *mut Handle) {
	// SAFETY: the caller's contract, and the GIL is held.
	unsafe {
		if (*handle).holds != 0 {
			let name = type_name(Py_TYPE(handle.cast()).cast());
			rt::fail(&name, "a call still holds it when its lend ends");
		}
		close(handle);
	}
}

/// # Safety
/// `handle` is live and held by no call.
unsafe fn free_now(handle: *mut Handle) {
	// SAFETY: the caller's contract.
	unsafe {
		if let Some(free) = (*handle).free.take() {
			let raw = std::mem::replace(&mut (*handle).raw, null());
			call(|| free(raw));
		}
	}
}

unsafe extern "C" fn handle_dealloc(obj: *mut PyObject) {
	// SAFETY: Python deallocates an `Object` once, holding the GIL; nothing holds it.
	unsafe {
		free_now(obj.cast());
		dealloc(obj);
	}
}

/// Free an instance of a heap type, and the reference to its type it held.
///
/// # Safety
/// `obj` is being deallocated, holding the GIL.
unsafe fn dealloc(obj: *mut PyObject) {
	// SAFETY: the caller's contract.
	unsafe {
		let class = Py_TYPE(obj);
		let free: freefunc = std::mem::transmute(PyType_GetSlot(class, Py_tp_free));
		free(obj.cast());
		Py_DecRef(class.cast());
	}
}

/// The owner of bytes Rust returned, which a `memoryview` reads in place.
#[repr(C)]
struct Bytes {
	ob_base: PyObject,
	buf: rt::Buf,
}

static BYTES: std::sync::OnceLock<Static> = std::sync::OnceLock::new();

/// A reference kept for the interpreter's life.
struct Static(Obj);

// SAFETY: touched only with the GIL held.
unsafe impl Send for Static {}
// SAFETY: as above.
unsafe impl Sync for Static {}

impl Static {
	fn ptr(&self) -> *mut PyObject {
		self.0.ptr()
	}
}

unsafe extern "C" fn bytes_getbuffer(obj: *mut PyObject, view: *mut Py_buffer, flags: c_int) -> c_int {
	// SAFETY: Python asks a live `Bytes` for a view.
	unsafe {
		let buf = (*obj.cast::<Bytes>()).buf;
		PyBuffer_FillInfo(view, obj, buf.ptr.cast_mut().cast(), buf.len as Py_ssize_t, 1, flags)
	}
}

unsafe extern "C" fn bytes_dealloc(obj: *mut PyObject) {
	// SAFETY: Python deallocates a `Bytes` once no view of it is left.
	unsafe {
		(*obj.cast::<Bytes>()).buf.free();
		dealloc(obj);
	}
}

/// What a wake does: queue a token for the event loop, and write a byte it reads, once
/// until it takes them; or, during a call on the thread running the loop, note the queue
/// for the call to resume once it returns.
pub struct Queue {
	tokens: Mutex<Vec<i64>>,
	signaled: AtomicBool,
	/// Whether a call holds the queue to resume, until it is taken.
	woken: AtomicBool,
	/// The address of its event loop, which [`RUNNING`] matches while it runs a call.
	event_loop: usize,
	/// How many of its futures are parked and its `Waker`s alive, which a wake may queue.
	waits: Arc<AtomicUsize>,
	tx: Socket,
	rx: Socket,
}

#[cfg(unix)]
type Socket = std::os::unix::net::UnixStream;
#[cfg(windows)]
type Socket = std::net::TcpStream;

/// A connected pair of sockets that never block.
#[cfg(unix)]
fn socket_pair() -> std::io::Result<(Socket, Socket)> {
	let (tx, rx) = Socket::pair()?;
	tx.set_nonblocking(true)?;
	rx.set_nonblocking(true)?;
	Ok((tx, rx))
}

/// A connected pair of sockets that never block, over loopback TCP, as Python's own
/// `socket.socketpair` makes one on Windows.
#[cfg(windows)]
fn socket_pair() -> std::io::Result<(Socket, Socket)> {
	let listener = std::net::TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, 0))?;
	let tx = Socket::connect(listener.local_addr()?)?;
	let (rx, _) = listener.accept()?;
	tx.set_nodelay(true)?;
	tx.set_nonblocking(true)?;
	rx.set_nonblocking(true)?;
	Ok((tx, rx))
}

impl Queue {
	fn push(self: &Arc<Self>, token: i64) {
		self.tokens.lock().unwrap_or_else(|e| e.into_inner()).push(token);
		// During a call on the thread running the loop, which runs nothing until it returns.
		if RUNNING.get() == self.event_loop {
			if !self.woken.swap(true, Ordering::SeqCst) {
				RESUMES.with_borrow_mut(|resumes| resumes.push(self.clone()));
			}
			return;
		}
		self.signal();
	}

	/// Note this thread as running the queue's loop, so its calls look for the loop while
	/// something on the queue waits.
	fn here(&self) {
		WAITS.with_borrow_mut(|waits| {
			if !waits.as_ref().is_some_and(|w| Arc::ptr_eq(w, &self.waits)) {
				*waits = Some(self.waits.clone());
			}
		});
	}

	fn signal(&self) {
		if !self.signaled.swap(true, Ordering::SeqCst) {
			// A full socket already signals.
			let _ = (&self.tx).write(&[1]);
		}
	}

	/// The tokens queued, each once, in the order first queued: the socket drained if
	/// `read`, as the loop does once it signals, then each signal reset, then the queue
	/// taken, so a token queued after is signaled again.
	fn take(&self, read: bool) -> Vec<i64> {
		self.here();
		if read {
			let mut sink = [0u8; 64];
			while matches!((&self.rx).read(&mut sink), Ok(n) if n > 0) {}
			self.signaled.store(false, Ordering::SeqCst);
		}
		self.woken.store(false, Ordering::SeqCst);
		let tokens = std::mem::take(&mut *self.tokens.lock().unwrap_or_else(|e| e.into_inner()));
		let mut seen = std::collections::HashSet::new();
		tokens.into_iter().filter(|t| seen.insert(*t)).collect()
	}
}

/// What a waker Rust holds wakes: a future polled on an event loop, or a Python
/// `Waker`. A token names it to the loop; a `Waker`'s negative token says it is gone.
pub struct Node {
	state: AtomicU8,
	token: i64,
	queue: Arc<Queue>,
	/// Whether it is a `Waker`'s, whose every wake reaches the loop.
	waker: bool,
}

const IDLE: u8 = 0;
const POLLING: u8 = 1;
const PARKED: u8 = 2;
const WOKEN: u8 = 3;

static TOKENS: AtomicI64 = AtomicI64::new(1);

impl Node {
	fn new(queue: Arc<Queue>, waker: bool) -> Arc<Self> {
		if waker {
			queue.waits.fetch_add(1, Ordering::Relaxed);
		}
		Arc::new(Self {
			state: AtomicU8::new(IDLE),
			token: TOKENS.fetch_add(1, Ordering::Relaxed),
			queue,
			waker,
		})
	}

	/// Move to `state`, returning the state it left: a node that leaves `PARKED` no
	/// longer waits.
	fn enter(&self, state: u8) -> u8 {
		let left = self.state.swap(state, Ordering::AcqRel);
		if left == PARKED {
			self.queue.waits.fetch_sub(1, Ordering::Relaxed);
		}
		left
	}

	/// Park after a poll that pended, unless woken while it polled.
	fn park(&self) -> bool {
		// Counted first, so a wake that unparks it at once never takes the count below zero.
		self.queue.waits.fetch_add(1, Ordering::Relaxed);
		let parked = self
			.state
			.compare_exchange(POLLING, PARKED, Ordering::AcqRel, Ordering::Acquire)
			.is_ok();
		if !parked {
			self.queue.waits.fetch_sub(1, Ordering::Relaxed);
		}
		parked
	}

	/// A C waker holding a new reference to the node.
	fn give(self: &Arc<Self>) -> rt::Waker {
		rt::Waker {
			wake: Some(node_wake),
			drop: Some(node_drop),
			data: Arc::into_raw(self.clone()).cast_mut().cast(),
		}
	}

	/// The C waker [`Self::give`] gave, again: the C ABI reuses the one it holds.
	fn lend(self: &Arc<Self>) -> rt::Waker {
		rt::Waker {
			wake: Some(node_wake),
			drop: Some(node_drop),
			data: Arc::as_ptr(self).cast_mut().cast(),
		}
	}
}

impl Drop for Node {
	fn drop(&mut self) {
		if self.waker {
			self.queue.waits.fetch_sub(1, Ordering::Relaxed);
			self.queue.push(-self.token);
		}
	}
}

/// A wake, from any thread: a future's is latched while it polls, and queued once parked.
unsafe extern "C" fn node_wake(data: *mut c_void) {
	// SAFETY: Rust holds a reference to the node while it may wake it.
	let node = unsafe { &*data.cast::<Node>() };
	match node.waker {
		true => node.queue.push(node.token),
		false => {
			if node.enter(WOKEN) == PARKED {
				node.queue.push(node.token);
			}
		}
	}
}

unsafe extern "C" fn node_drop(data: *mut c_void) {
	// SAFETY: a reference `give` made, dropped once.
	drop(unsafe { Arc::from_raw(data.cast::<Node>()) });
}

/// `Queue(loop)`: the queue of one event loop, with its socket's file descriptor to read.
#[repr(C)]
struct QueueObj {
	ob_base: PyObject,
	queue: Option<Arc<Queue>>,
}

unsafe extern "C" fn queue_new(class: *mut PyTypeObject, args: *mut PyObject, _: *mut PyObject) -> *mut PyObject {
	wrap(|| {
		let event_loop = tuple_item(args, 0, 1)? as usize;
		let (tx, rx) = match socket_pair() {
			Ok(pair) => pair,
			// SAFETY: a static exception type.
			Err(err) => return raise(unsafe { PyExc_OSError }, &err.to_string()),
		};
		// SAFETY: the GIL is held, and `class` is `Queue`.
		unsafe {
			let obj = Obj::new(PyType_GenericAlloc(class, 0))?;
			let queue = Queue {
				tokens: Mutex::new(Vec::new()),
				signaled: AtomicBool::new(false),
				woken: AtomicBool::new(false),
				event_loop,
				waits: Arc::new(AtomicUsize::new(0)),
				tx,
				rx,
			};
			std::ptr::write(&raw mut (*obj.ptr().cast::<QueueObj>()).queue, Some(Arc::new(queue)));
			Ok(obj)
		}
	})
}

unsafe extern "C" fn queue_dealloc(obj: *mut PyObject) {
	// SAFETY: Python deallocates a `Queue` once.
	unsafe {
		drop((*obj.cast::<QueueObj>()).queue.take());
		dealloc(obj);
	}
}

/// The queue a `Queue` holds.
fn queue_of(obj: *mut PyObject) -> R<Arc<Queue>> {
	if !is_type(obj, &QUEUE) {
		return expected_named(obj, "Queue");
	}
	// SAFETY: a `Queue`, which holds its queue until deallocated.
	Ok(unsafe { (*obj.cast::<QueueObj>()).queue.clone() }.expect("a live queue"))
}

unsafe extern "C" fn queue_fileno(obj: *mut PyObject, _: *mut PyObject) -> *mut PyObject {
	// SAFETY: Python calls it on a `Queue`.
	let queue = unsafe { (*obj.cast::<QueueObj>()).queue.as_ref() }.expect("a live queue");
	#[cfg(unix)]
	let fd = std::os::fd::AsRawFd::as_raw_fd(&queue.rx) as i64;
	#[cfg(windows)]
	let fd = std::os::windows::io::AsRawSocket::as_raw_socket(&queue.rx) as i64;
	int_out(fd).map_or(null_mut(), Obj::into_ptr)
}

/// `take(read)`: the tokens queued, the socket drained first if `read`.
unsafe extern "C" fn queue_take(obj: *mut PyObject, read: *mut PyObject) -> *mut PyObject {
	// SAFETY: Python calls it on a `Queue`.
	let queue = unsafe { (*obj.cast::<QueueObj>()).queue.as_ref() }.expect("a live queue");
	// SAFETY: comparing addresses.
	let tokens = queue.take(read == unsafe { Py_True() });
	wrap(|| {
		let list = rt::List::owned(tokens);
		list_out(list, int_out)
	})
}

/// `signaled()`: whether a wake wrote a byte the loop has yet to take. The socket alone
/// cannot say, since a proactor loop's pending read takes the byte as it lands.
unsafe extern "C" fn queue_signaled(obj: *mut PyObject, _: *mut PyObject) -> *mut PyObject {
	// SAFETY: Python calls it on a `Queue`.
	let queue = unsafe { (*obj.cast::<QueueObj>()).queue.as_ref() }.expect("a live queue");
	bool_out(queue.signaled.load(Ordering::SeqCst)).map_or(null_mut(), Obj::into_ptr)
}

/// `Node`: a Python `Waker`'s node, made on its loop's queue.
#[repr(C)]
struct NodeObj {
	ob_base: PyObject,
	node: Option<Arc<Node>>,
}

unsafe extern "C" fn node_new(class: *mut PyTypeObject, args: *mut PyObject, _: *mut PyObject) -> *mut PyObject {
	wrap(|| {
		let queue = queue_of(tuple_item(args, 0, 1)?)?;
		// SAFETY: the GIL is held, and `class` is `Node`.
		unsafe {
			let obj = Obj::new(PyType_GenericAlloc(class, 0))?;
			std::ptr::write(&raw mut (*obj.ptr().cast::<NodeObj>()).node, Some(Node::new(queue, true)));
			Ok(obj)
		}
	})
}

unsafe extern "C" fn node_obj_dealloc(obj: *mut PyObject) {
	// SAFETY: Python deallocates a `Node` once.
	unsafe {
		drop((*obj.cast::<NodeObj>()).node.take());
		dealloc(obj);
	}
}

unsafe extern "C" fn node_token(obj: *mut PyObject, _: *mut PyObject) -> *mut PyObject {
	// SAFETY: Python calls it on a `Node`.
	let node = unsafe { (*obj.cast::<NodeObj>()).node.as_ref() }.expect("a live node");
	int_out(node.token).map_or(null_mut(), Obj::into_ptr)
}

static NODE_ATTR: Key = Key::new(c"_node");

/// A Python `Waker` passed by value: Rust holds a new reference to its node.
pub fn waker_in(obj: *mut PyObject, s: &mut Scope) -> R<rt::Waker> {
	let node = attr(obj, &NODE_ATTR, s)?;
	if !is_type(node, &NODE) {
		return expected_named(obj, "Waker");
	}
	// SAFETY: a `Node`, which holds its node until deallocated.
	let node = unsafe { (*node.cast::<NodeObj>()).node.clone() }.expect("a live node");
	let waker = node.give();
	s.nodes.push(waker);
	Ok(waker)
}

/// Poll a future once with a waker, returning its value once it finishes.
pub type Poll = unsafe fn(*mut c_void, rt::Waker) -> R<Option<Obj>>;

/// `Future`: a C ABI future its first poll left pending, which `__await__` polls on its
/// loop's queue until it finishes, and frees once it does or is closed, cancelling it.
#[repr(C)]
pub struct Future {
	ob_base: PyObject,
	raw: *mut c_void,
	poll: Option<Poll>,
	free: Option<unsafe fn(*mut c_void)>,
	node: Option<Arc<Node>>,
	token: i64,
	/// Whether an `await` took it, after which another raises, as awaiting a coroutine
	/// twice does.
	claimed: bool,
	/// Whether a poll runs, without the GIL, so a close from another thread waits for it.
	polling: bool,
	/// Whether it was closed while it polled, which the poll then does.
	closing: bool,
	/// The handles the call holds until the future ends.
	held: Vec<Obj>,
}

static PENDING: std::sync::OnceLock<Static> = std::sync::OnceLock::new();

/// A future the call `raw` made, polled at once unless `eager`, since a function that is
/// not `async` ran at the call and is polled when awaited: its value if ready, or else the
/// awaitable that polls it again, holding what the call held.
///
/// # Safety
/// `raw` is a live future that `poll` and `free` take.
pub unsafe fn start(raw: *mut c_void, poll: Poll, free: unsafe fn(*mut c_void), s: &mut Scope, eager: bool) -> R<Obj> {
	if !eager {
		let none = rt::Waker {
			wake: None,
			drop: None,
			data: null_mut(),
		};
		// SAFETY: the caller's contract.
		match unsafe { poll(raw, none) } {
			Ok(None) => {}
			done => {
				// SAFETY: as above, and finished or failed.
				call(|| unsafe { free(raw) });
				return done.map(|value| value.expect("ready"));
			}
		}
	}
	// SAFETY: the GIL is held, and the registered class subclasses `Future`.
	unsafe {
		let obj = PyType_GenericAlloc(ty(T_FUTURE).cast(), 0);
		let Ok(obj) = Obj::new(obj) else {
			call(|| free(raw));
			return Err(Raised);
		};
		// The allocation is zeroed, which is `None` for each option.
		let future = obj.ptr().cast::<Future>();
		(*future).raw = raw;
		(*future).poll = Some(poll);
		(*future).free = Some(free);
		(*future).token = TOKENS.fetch_add(1, Ordering::Relaxed);
		std::ptr::write(&raw mut (*future).held, s.take_held());
		Ok(obj)
	}
}

/// Free a future's C future, cancelling it unless it finished, and let go of its holds:
/// once its poll returns, if one runs.
///
/// # Safety
/// `future` is live.
unsafe fn future_close(future: *mut Future) {
	// SAFETY: the caller's contract.
	unsafe {
		if (*future).polling {
			(*future).closing = true;
			return;
		}
		// Ended before the free releases the GIL, as Python may run during it.
		(*future).poll = None;
		if let Some(free) = (*future).free.take() {
			let raw = std::mem::replace(&mut (*future).raw, null_mut());
			call(|| free(raw));
		}
		// Woken for good: a wake Rust makes after queues nothing.
		if let Some(node) = (*future).node.take() {
			node.enter(WOKEN);
		}
		for held in std::mem::take(&mut (*future).held) {
			release(held.ptr().cast());
		}
	}
}

/// `poll(queue)`: poll on the queue's loop, again at once if woken while polling: the
/// value, or `PENDING` once parked, after which a wake queues the future's token.
unsafe extern "C" fn future_poll(obj: *mut PyObject, args: *mut *mut PyObject, nargs: Py_ssize_t) -> *mut PyObject {
	let future = obj.cast::<Future>();
	wrap(|| {
		// SAFETY: Python's call.
		let args = unsafe { self::args(args, nargs, 1, "poll") }?;
		// SAFETY: Python calls it on a `Future`, holding the GIL. Each access goes through
		// the pointer, as a thread holding the GIL may close it while the poll runs without.
		unsafe {
			let Some(poll) = (*future).poll else {
				return value_error("the future ended");
			};
			if (*future).polling {
				return raise(PyExc_RuntimeError, "a Rust future is polled already");
			}
			let node = match &(*future).node {
				Some(node) => node.clone(),
				None => {
					let node = Arc::new(Node {
						state: AtomicU8::new(IDLE),
						token: (*future).token,
						queue: queue_of(args[0])?,
						waker: false,
					});
					(*future).node = Some(node.clone());
					node
				}
			};
			node.queue.here();
			loop {
				let first = node.enter(POLLING) == IDLE;
				// The first poll with the node gives the C ABI a reference, which it keeps
				// while it reuses the waker.
				let waker = match first {
					true => node.give(),
					false => node.lend(),
				};
				(*future).polling = true;
				let done = poll((*future).raw, waker);
				(*future).polling = false;
				if std::mem::take(&mut (*future).closing) {
					future_close(future);
					// What the poll returned is dropped with the future.
					return value_error("the future was closed while it polled");
				}
				if let Some(done) = done.transpose() {
					future_close(future);
					return done;
				}
				if node.park() {
					return Ok(Obj::borrowed(PENDING.get().expect("loaded").ptr()));
				}
				// Woken while it polled: the wake is for this poll's registration, so poll again.
			}
		}
	})
}

/// `claim()`: take the future for one `await`, returning the token its wakes queue.
unsafe extern "C" fn future_claim(obj: *mut PyObject, _: *mut PyObject) -> *mut PyObject {
	// SAFETY: Python calls it on a `Future`, holding the GIL.
	let future = unsafe { &mut *obj.cast::<Future>() };
	wrap(|| match std::mem::replace(&mut future.claimed, true) {
		// SAFETY: a static exception type.
		true => raise(unsafe { PyExc_RuntimeError }, "a Rust future is awaited already"),
		false => int_out(future.token),
	})
}

unsafe extern "C" fn future_close_py(obj: *mut PyObject, _: *mut PyObject) -> *mut PyObject {
	// SAFETY: Python calls it on a `Future`, holding the GIL.
	unsafe { future_close(obj.cast()) };
	none().map_or(null_mut(), Obj::into_ptr)
}

unsafe extern "C" fn future_dealloc(obj: *mut PyObject) {
	// SAFETY: Python deallocates a `Future` once.
	unsafe {
		future_close(obj.cast());
		std::ptr::drop_in_place(&raw mut (*obj.cast::<Future>()).held);
		dealloc(obj);
	}
}

/// Whether `obj` is exactly the runtime's type in `slot`.
fn is_type(obj: *mut PyObject, slot: &std::sync::OnceLock<Static>) -> bool {
	// SAFETY: `obj` is live.
	slot.get().is_some_and(|t| unsafe { Py_TYPE(obj) }.cast() == t.ptr())
}

static OBJECT: std::sync::OnceLock<Static> = std::sync::OnceLock::new();
static FUTURE: std::sync::OnceLock<Static> = std::sync::OnceLock::new();
static QUEUE: std::sync::OnceLock<Static> = std::sync::OnceLock::new();
static NODE: std::sync::OnceLock<Static> = std::sync::OnceLock::new();

/// A method table entry.
pub const fn method(name: &'static CStr, f: PyCFunctionFast) -> PyMethodDef {
	PyMethodDef {
		ml_name: name.as_ptr(),
		ml_meth: PyMethodDefPointer { PyCFunctionFast: f },
		ml_flags: METH_FASTCALL,
		ml_doc: null(),
	}
}

const fn noargs(name: &'static CStr, f: PyCFunction) -> PyMethodDef {
	PyMethodDef {
		ml_name: name.as_ptr(),
		ml_meth: PyMethodDefPointer { PyCFunction: f },
		ml_flags: METH_NOARGS,
		ml_doc: null(),
	}
}

const fn onearg(name: &'static CStr, f: PyCFunction) -> PyMethodDef {
	PyMethodDef {
		ml_name: name.as_ptr(),
		ml_meth: PyMethodDefPointer { PyCFunction: f },
		ml_flags: METH_O,
		ml_doc: null(),
	}
}

pub const END: PyMethodDef = PyMethodDef::zeroed();

/// A member the glue defines natively, which `_register` sets on the class registered
/// at its index, so Python calls into the extension with no frame of its own between.
pub enum Member {
	/// A method taking `self`.
	Method(usize, PyMethodDef),
	/// A static method.
	Static(usize, PyMethodDef),
	/// A read-only property.
	Get(usize, PyGetSetDef),
}

/// A method or function Python calls directly, by position or keyword, whose `doc`
/// starts with its signature, as `inspect.signature` reads it.
pub const fn function(name: &'static CStr, doc: &'static CStr, f: PyCFunctionFastWithKeywords) -> PyMethodDef {
	PyMethodDef {
		ml_name: name.as_ptr(),
		ml_meth: PyMethodDefPointer {
			PyCFunctionFastWithKeywords: f,
		},
		ml_flags: METH_FASTCALL | METH_KEYWORDS,
		ml_doc: doc.as_ptr(),
	}
}

/// A read-only property.
pub const fn property(name: &'static CStr, doc: Option<&'static CStr>, f: getter) -> PyGetSetDef {
	PyGetSetDef {
		name: name.as_ptr(),
		get: Some(f),
		set: None,
		doc: match doc {
			Some(doc) => doc.as_ptr(),
			None => null(),
		},
		closure: null_mut(),
	}
}

/// The glue's members, which `_register` sets on their classes.
static INSTALL: Gil<&'static mut [Member]> = Gil::new(&mut []);

/// A table of methods Python reads for the interpreter's life.
pub struct Methods<const N: usize>(pub UnsafeCell<[PyMethodDef; N]>);

// SAFETY: Python reads it, holding the GIL.
unsafe impl<const N: usize> Sync for Methods<N> {}

impl<const N: usize> Methods<N> {
	pub fn ptr(&self) -> *mut PyMethodDef {
		self.0.get().cast()
	}
}

static FUTURE_METHODS: Methods<4> = Methods(UnsafeCell::new([
	method(c"poll", future_poll),
	noargs(c"claim", future_claim),
	noargs(c"close", future_close_py),
	END,
]));

static QUEUE_METHODS: Methods<4> = Methods(UnsafeCell::new([
	noargs(c"fileno", queue_fileno),
	onearg(c"take", queue_take),
	noargs(c"signaled", queue_signaled),
	END,
]));

static NODE_METHODS: Methods<2> = Methods(UnsafeCell::new([noargs(c"token", node_token), END]));

static REGISTER: Methods<2> = Methods(UnsafeCell::new([method(c"_register", register), END]));

/// Make a type from `slots` and add it to `module` by its name, noting it in `cell`.
///
/// # Safety
/// The GIL is held, and `module` is the module being made.
unsafe fn add_type(
	module: *mut PyObject,
	cell: &std::sync::OnceLock<Static>,
	name: &CStr,
	size: usize,
	flags: u64,
	slots: &mut [PyType_Slot],
) -> R<()> {
	let qualified = format!("{}.{}\0", module_name(module), name.to_str().expect("ASCII"));
	// Python keeps the name for the type's life.
	let qualified: &'static str = Box::leak(qualified.into_boxed_str());
	let mut spec = PyType_Spec {
		name: qualified.as_ptr().cast(),
		basicsize: size as c_int,
		itemsize: 0,
		flags: (Py_TPFLAGS_DEFAULT as u64 | flags) as _,
		slots: slots.as_mut_ptr(),
	};
	// SAFETY: the caller's contract, and `spec` a valid spec.
	unsafe {
		let class = Obj::new(PyType_FromSpec(&mut spec))?;
		if PyModule_AddObjectRef(module, name.as_ptr(), class.ptr()) != 0 {
			return Err(Raised);
		}
		let _ = cell.set(Static(class));
	}
	Ok(())
}

fn module_name(module: *mut PyObject) -> String {
	// SAFETY: the GIL is held, and `module` a module.
	unsafe {
		let name = PyModule_GetName(module);
		CStr::from_ptr(name).to_string_lossy().into_owned()
	}
}

/// Make the extension module `def` names, with the runtime's types and functions beside
/// the glue's, whose `members` `_register` sets on their classes.
///
/// # Safety
/// Python calls the module's `PyInit_` with the GIL held, once per interpreter.
pub unsafe fn module(def: *mut PyModuleDef, members: &'static mut [Member]) -> *mut PyObject {
	// SAFETY: the caller's contract.
	unsafe {
		*INSTALL.get() = members;
		let Ok(module) = Obj::new(PyModule_Create(def)) else {
			return null_mut();
		};
		match runtime(module.ptr()) {
			Ok(()) => module.into_ptr(),
			Err(Raised) => null_mut(),
		}
	}
}

unsafe fn runtime(module: *mut PyObject) -> R<()> {
	// SAFETY: the GIL is held, and `module` the module being made.
	unsafe {
		if PyModule_AddFunctions(module, REGISTER.ptr()) != 0 {
			return Err(Raised);
		}
		let base = (Py_TPFLAGS_BASETYPE | Py_TPFLAGS_DISALLOW_INSTANTIATION) as u64;
		add_type(module, &OBJECT, c"Object", size_of::<Handle>(), base, &mut [
			slot(Py_tp_dealloc, handle_dealloc as *mut c_void),
			slot(
				Py_tp_doc,
				c"An object owning a Rust value's handle, freed once the last reference to it goes.".as_ptr().cast_mut().cast(),
			),
			slot(0, null_mut()),
		])?;
		add_type(module, &FUTURE, c"Future", size_of::<Future>(), base, &mut [
			slot(Py_tp_dealloc, future_dealloc as *mut c_void),
			slot(Py_tp_methods, FUTURE_METHODS.ptr().cast()),
			slot(0, null_mut()),
		])?;
		add_type(module, &QUEUE, c"Queue", size_of::<QueueObj>(), 0, &mut [
			slot(Py_tp_new, queue_new as *mut c_void),
			slot(Py_tp_dealloc, queue_dealloc as *mut c_void),
			slot(Py_tp_methods, QUEUE_METHODS.ptr().cast()),
			slot(0, null_mut()),
		])?;
		add_type(module, &NODE, c"Node", size_of::<NodeObj>(), 0, &mut [
			slot(Py_tp_new, node_new as *mut c_void),
			slot(Py_tp_dealloc, node_obj_dealloc as *mut c_void),
			slot(Py_tp_methods, NODE_METHODS.ptr().cast()),
			slot(0, null_mut()),
		])?;
		let bytes_slots = &mut [
			slot(Py_bf_getbuffer, bytes_getbuffer as *mut c_void),
			slot(Py_tp_dealloc, bytes_dealloc as *mut c_void),
			slot(0, null_mut()),
		];
		add_type(module, &BYTES, c"Bytes", size_of::<Bytes>(), Py_TPFLAGS_DISALLOW_INSTANTIATION as u64, bytes_slots)?;
		// A plain `object()`, which nothing else is.
		let pending = Obj::new(PyObject_CallObject((&raw mut PyBaseObject_Type).cast(), null_mut()))?;
		if PyModule_AddObjectRef(module, c"PENDING".as_ptr(), pending.ptr()) != 0 {
			return Err(Raised);
		}
		let _ = PENDING.set(Static(pending));
		let asyncio = Obj::new(PyImport_ImportModule(c"asyncio".as_ptr()))?;
		let get = Obj::new(PyObject_GetAttrString(asyncio.ptr(), c"_get_running_loop".as_ptr()))?;
		let _ = GET_RUNNING_LOOP.set(Static(get));
	}
	Ok(())
}

const fn slot(slot: c_int, pfunc: *mut c_void) -> PyType_Slot {
	PyType_Slot { slot, pfunc }
}
