//! The api fixture's glue, driven through its `extern "C"` functions under Miri, which
//! cannot call foreign code: `api_miri` in tests/fixtures.rs mounts this file as a test
//! module of a copy of the export. Miri fails on a leak, a use after free, a handle lent
//! past its call, or a race between a wake and a poll.

use super::*;
use std::ffi::c_void;
use std::sync::atomic::{AtomicUsize, Ordering};

fn view(s: &str) -> rt::Buf {
	rt::Buf {
		ptr: s.as_ptr(),
		len: s.len(),
		owner: std::ptr::null_mut(),
	}
}

/// A returned buffer's bytes, freeing it.
fn take(buf: rt::Buf) -> String {
	// SAFETY: Rust returned `buf`, and it is freed once, below.
	let text = String::from_utf8(unsafe { std::slice::from_raw_parts(buf.ptr, buf.len) }.to_vec()).unwrap();
	unsafe { api_buf_free(buf) };
	text
}

/// A foreign waker counting its wakes and drops in the counters `data` names.
#[derive(Default)]
struct Counts {
	wakes: AtomicUsize,
	drops: AtomicUsize,
}

unsafe extern "C" fn wake(data: *mut c_void) {
	unsafe { &*data.cast::<Counts>() }.wakes.fetch_add(1, Ordering::SeqCst);
}

unsafe extern "C" fn drop_waker(data: *mut c_void) {
	unsafe { &*data.cast::<Counts>() }.drops.fetch_add(1, Ordering::SeqCst);
}

fn waker(counts: &Counts) -> rt::Waker {
	rt::Waker {
		wake: Some(wake),
		drop: Some(drop_waker),
		data: std::ptr::from_ref(counts).cast_mut().cast(),
	}
}

/// A future the foreign side implements: ready on its `ready`th poll, holding the waker
/// of the last.
struct Foreign {
	ready: usize,
	polls: usize,
	freed: bool,
	waker: Option<rt::Waker>,
}

impl Foreign {
	fn new(ready: usize) -> Self {
		Self {
			ready,
			polls: 0,
			freed: false,
			waker: None,
		}
	}

	fn release(&mut self) {
		if let Some(waker) = self.waker.take() {
			unsafe { waker.release() };
		}
	}

	fn future(&mut self) -> rt::ForeignFuture {
		rt::ForeignFuture {
			poll: Some(foreign_poll),
			free: Some(foreign_free),
			data: std::ptr::from_mut(self).cast(),
		}
	}
}

unsafe extern "C" fn foreign_poll(data: *mut c_void, waker: rt::Waker) -> bool {
	let f = unsafe { &mut *data.cast::<Foreign>() };
	f.release();
	f.polls += 1;
	if f.polls >= f.ready {
		unsafe { waker.release() };
		return true;
	}
	f.waker = Some(waker);
	false
}

unsafe extern "C" fn foreign_free(data: *mut c_void) {
	let f = unsafe { &mut *data.cast::<Foreign>() };
	f.release();
	f.freed = true;
}

#[test]
fn handles() {
	unsafe {
		let counter = api_Counter_new(view("c"), 5);
		assert_eq!(api_Counter_get(counter), 5);
		let mut out = 0;
		assert!(api_Counter_add(counter, 2, &mut out, std::ptr::null_mut()));
		assert_eq!(api_Counter_get(counter), 7);
		assert_eq!(take(api_Counter_name(counter)), "c");
		let other = api_Counter_new(view("d"), 3);
		assert_eq!(api_Counter_plus(counter, other), 10);
		api_Counter_free(other);
		// Moving the value out leaves the handle for the caller to free.
		assert_eq!(api_Counter_into_sum(counter, 1), 8);
		api_Counter_free(counter);
	}
}

#[test]
fn buffers() {
	unsafe {
		assert_eq!(take(api_greet(view("you"), 1)), "hello you, a Circle");
		let copy = api_buf_copy(view("copied"));
		assert_eq!(take(copy), "copied");
		let empty = api_buf_copy(view(""));
		assert_eq!(take(empty), "");
	}
}

#[test]
fn channel() {
	let counts = Counts::default();
	// One waker, copied: Miri may give each reification of `wake` its own address, so a
	// waker made anew per poll would never match the one a future keeps.
	let w = waker(&counts);
	unsafe {
		let channel = api_Channel_new();
		let recv = api_Channel_recv(channel);
		let mut payload = rt::Buf::NONE;
		assert_eq!(api_Channel_recv_poll(recv, w, &mut payload), rt::PENDING);
		api_Channel_send(channel, view("hi"));
		assert_eq!(counts.wakes.load(Ordering::SeqCst), 1);
		assert_eq!(api_Channel_recv_poll(recv, w, &mut payload), rt::READY);
		assert_eq!(take(payload), "hi");
		api_Channel_recv_future_free(recv);

		// A send from another thread wakes the poll's waker there.
		let recv = api_Channel_recv(channel);
		assert_eq!(api_Channel_recv_poll(recv, w, &mut payload), rt::PENDING);
		api_Channel_send_from_thread(channel, view("far"));
		assert_eq!(counts.wakes.load(Ordering::SeqCst), 2);
		assert_eq!(api_Channel_recv_poll(recv, w, &mut payload), rt::READY);
		assert_eq!(take(payload), "far");
		api_Channel_recv_future_free(recv);

		// Cancelled while pending, it frees what it held, its waker included.
		let recv = api_Channel_recv(channel);
		assert_eq!(api_Channel_recv_poll(recv, w, &mut payload), rt::PENDING);
		api_Channel_recv_future_free(recv);
		api_Channel_free(channel);
	}
	// A repeated waker is kept rather than lifted anew: one drop per future.
	assert_eq!(counts.drops.load(Ordering::SeqCst), 3);
}

#[test]
fn foreign_futures() {
	let counts = Counts::default();
	unsafe {
		let mut out = 0u32;
		let mut now = Foreign::new(1);
		let after = api_after(now.future(), 4);
		assert_eq!(api_after_poll(after, waker(&counts), &mut out), rt::READY);
		assert_eq!(out, 8);
		api_after_future_free(after);
		assert!(now.freed);

		let mut later = Foreign::new(2);
		let after = api_after(later.future(), 5);
		assert_eq!(api_after_poll(after, waker(&counts), &mut out), rt::PENDING);
		later.waker.unwrap().wake();
		assert_eq!(counts.wakes.load(Ordering::SeqCst), 1);
		assert_eq!(api_after_poll(after, waker(&counts), &mut out), rt::READY);
		assert_eq!(out, 10);
		api_after_future_free(after);
		assert!(later.freed && later.waker.is_none());

		// Dropped before it finishes, a poll frees the future it owns.
		let mut dropped = Foreign::new(usize::MAX);
		let polled = api_Waiter_poll_future(dropped.future());
		assert_eq!(api_Waiter_poll_future_poll(polled, waker(&counts)), rt::PENDING);
		api_Waiter_poll_future_future_free(polled);
		assert!(dropped.freed && dropped.waker.is_none());
	}
}
