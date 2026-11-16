//! What `web/rt.ts` calls that the C ABI lacks, which the crate's `web` feature builds for
//! wasm32: memory the binding lowers values into, a call through a C function pointer,
//! and the C functions of what JavaScript implements. Each of those is a function the
//! module imports from `rust2`, whose data is a token the binding finds its value by, so
//! JavaScript never puts a function in the module's table.

use std::alloc::Layout;
use std::ffi::c_void;

use crate::rt;

// Each named apart from every C symbol the module links, as libc's `free`, which a
// declaration of the same name would resolve to whatever its import module.
#[link(wasm_import_module = "rust2")]
unsafe extern "C" {
	/// Wake the token a waker JavaScript made holds.
	fn rust2_wake(data: *mut c_void);
	/// Drop a waker JavaScript made.
	fn rust2_waker_drop(data: *mut c_void);
	/// Rust dropped what JavaScript implements and gave it: a trait's implementation, a kept
	/// closure, or a future.
	fn rust2_free(data: *mut c_void);
	/// Poll a future JavaScript implements.
	fn rust2_future_poll(data: *mut c_void, waker: rt::Waker) -> bool;
	/// Read and seek a reader JavaScript implements.
	fn rust2_reader_read(data: *mut c_void, buf: *mut u8, len: usize, n: *mut usize, err: *mut rt::Buf) -> bool;
	fn rust2_reader_seek(
		data: *mut c_void,
		whence: std::ffi::c_int,
		offset: i64,
		position: *mut u64,
		err: *mut rt::Buf,
	) -> bool;
}

/// The imported C functions every API's values take, as `rt.ts` reads their table indices.
#[repr(C)]
pub struct Fns {
	wake: unsafe extern "C" fn(*mut c_void),
	waker_drop: unsafe extern "C" fn(*mut c_void),
	free: unsafe extern "C" fn(*mut c_void),
	future_poll: unsafe extern "C" fn(*mut c_void, rt::Waker) -> bool,
	reader_read: unsafe extern "C" fn(*mut c_void, *mut u8, usize, *mut usize, *mut rt::Buf) -> bool,
	reader_seek: unsafe extern "C" fn(*mut c_void, std::ffi::c_int, i64, *mut u64, *mut rt::Buf) -> bool,
}

pub static FNS: Fns = Fns {
	wake: rust2_wake,
	waker_drop: rust2_waker_drop,
	free: rust2_free,
	future_poll: rust2_future_poll,
	reader_read: rust2_reader_read,
	reader_seek: rust2_reader_seek,
};

/// What a trait's table names for `free`.
pub const FREE: unsafe extern "C" fn(*mut c_void) = rust2_free;

/// `len` bytes aligned to `align`, a power of two, which [`dealloc`] frees.
pub fn alloc(len: usize, align: usize) -> *mut u8 {
	let layout = Layout::from_size_align(len.max(1), align).unwrap_or_else(|_| rt::fail("alloc", "a bad layout"));
	// SAFETY: the layout is not empty.
	let ptr = unsafe { std::alloc::alloc(layout) };
	if ptr.is_null() {
		std::alloc::handle_alloc_error(layout);
	}
	ptr
}

/// Free what [`alloc`] gave for `len` and `align`.
///
/// # Safety
/// `ptr` came from [`alloc`] with these, freed once.
pub unsafe fn dealloc(ptr: *mut u8, len: usize, align: usize) {
	let layout = Layout::from_size_align(len.max(1), align).unwrap_or_else(|_| rt::fail("free", "a bad layout"));
	// SAFETY: the caller's contract.
	unsafe { std::alloc::dealloc(ptr, layout) }
}
