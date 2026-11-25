//! Futures C polls and the wakers it polls them with: a future is one allocation behind a
//! handle, polled one call at a time from any thread, and a waker C gives is wrapped once
//! and dropped once its last clone drops, on whichever thread that is.
//!
//! The runtime compiles this beside `rt.rs`, and `ffi/tests/future.rs` under loom, each
//! with its own `Arc`, `AtomicBool`, `Ordering`, and threads.

use std::ffi::c_void;
use std::pin::Pin;
use std::task::{Context, Wake};

use super::thread::{Dropped, Thread};
use super::{Arc, AtomicBool, Ordering, fail, failed};

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
	/// Whether `other` is this waker again, which a poll reuses rather than wrapping anew:
	/// one that differs in any function is another, whose `drop` must run.
	fn same(&self, other: &Self) -> bool {
		self.wake.map(|f| f as usize) == other.wake.map(|f| f as usize)
			&& self.drop.map(|f| f as usize) == other.drop.map(|f| f as usize)
			&& self.data == other.data
	}

	/// The Rust waker around it, which calls `drop` once its last clone drops. One Rust
	/// returned stays wrapped too, so its `data` names it for as long as a future may
	/// compare it.
	pub fn lift(self) -> std::task::Waker {
		match (self.wake, self.drop) {
			// Nothing to wake or release: a ready future's first poll takes no allocation.
			(None, None) => std::task::Waker::noop().clone(),
			_ => std::task::Waker::from(std::sync::Arc::new(Foreign(self))),
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

struct Foreign(Waker);

// SAFETY: the waker's contract: `wake` and `drop` may be called from any thread.
unsafe impl Send for Foreign {}
// SAFETY: as above.
unsafe impl Sync for Foreign {}

impl Wake for Foreign {
	fn wake(self: std::sync::Arc<Self>) {
		self.wake_by_ref();
	}

	fn wake_by_ref(self: &std::sync::Arc<Self>) {
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
		// SAFETY: the caller's contract. Only the flag and the owner are borrowed, each
		// through its own field, as another poll may hold the rest while this one is refused.
		let (polling, owner) = unsafe { (&(*future).polling, &(*future).owner) };
		if let Some(owner) = owner {
			if owner.away() {
				// SAFETY: a home, and this thread waits while it polls.
				return unsafe { owner.run(|| Self::poll(future, waker, outs, err, status, what)) };
			}
			if let Err(refused) = owner.own() {
				if polling.swap(true, Ordering::Acquire) {
					fail(what, "a future is polled concurrently");
				}
				// A waker it never adopted is Rust's to drop, as a poll drops the one it
				// replaces; the flag keeps the one it holds from changing meanwhile.
				// SAFETY: the flag makes this the only access.
				if !unsafe { &(*future).waker }
					.as_ref()
					.is_some_and(|(last, _)| last.same(&waker))
				{
					// SAFETY: the caller gave it, and nothing else holds it.
					unsafe { waker.release() };
				}
				polling.store(false, Ordering::Release);
				// SAFETY: the caller's contract.
				return unsafe { failed(status, refused, what) };
			}
		}
		if polling.swap(true, Ordering::Acquire) {
			fail(what, "a future is polled concurrently");
		}
		// Cleared however the poll ends, so a future a poll unwound out of, as a Kotlin
		// closure's failed check unwinds, can still be freed.
		let polled = Polling(polling);
		// SAFETY: the flag makes this the only access, and `new` made the allocation.
		let result = unsafe { ((*future).poll)(future, waker, outs, err, what) };
		drop(polled);
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
		// A first poll that unwinds frees the future, which has no handle yet to free it by.
		let unwinding = Unwinding(future);
		// SAFETY: the caller's contract; the future was made on this thread, or one this
		// call runs on.
		let result = unsafe { Self::poll(future, none, outs, err, std::ptr::null_mut(), what) };
		std::mem::forget(unwinding);
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
			// Node's addon keeps such a future on a home only because a call may run on a
			// worker, so a free from JavaScript waits for the drop, as it would on its own
			// thread: what the future borrowed is free again once `cancel()` returns.
			#[cfg(feature = "node")]
			if owner.away() && crate::node::from_js() {
				let owner = owner.clone();
				// SAFETY: a home, and this thread waits while it drops the future.
				return unsafe { owner.run(|| unsafe { ((*future).drop)(future) }) };
			}
			// SAFETY: the future is the owner's.
			unsafe { Dropped::new(Freed(future), owner.clone(), "a future") }.send();
			return;
		}
		// SAFETY: the caller's contract.
		unsafe { ((*future).drop)(future) };
	}
}

/// A future's polling flag, cleared once dropped.
struct Polling<'a>(&'a AtomicBool);

impl Drop for Polling<'_> {
	fn drop(&mut self) {
		self.0.store(false, Ordering::Release);
	}
}

/// A future its first poll is unwinding out of, freed once dropped.
struct Unwinding(*mut Future);

impl Drop for Unwinding {
	fn drop(&mut self) {
		// SAFETY: `Future::start`'s contract, and C never got the handle.
		unsafe { Future::free(self.0) };
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
	let typed = future.cast::<Typed<F, L>>();
	// SAFETY: the caller's contract. Only the fields the polling flag guards are borrowed,
	// not the flag or the owner, which a concurrent poll reads before it is refused.
	let (last, future, lower) = unsafe { (&mut (*typed).head.waker, &mut (*typed).future, &(*typed).lower) };
	let Some(inner) = future.as_mut() else {
		fail(what, "a future is polled after it finished");
	};
	if !last.as_ref().is_some_and(|(last, _)| last.same(&waker)) {
		*last = Some((waker, waker.lift()));
	}
	let (_, rust) = last.as_ref().expect("set above");
	let mut cx = Context::from_waker(rust);
	// SAFETY: the allocation never moves, and the future drops in place.
	match unsafe { Pin::new_unchecked(inner) }.poll(&mut cx) {
		std::task::Poll::Pending => PENDING,
		std::task::Poll::Ready(value) => {
			*future = None;
			*last = None;
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
