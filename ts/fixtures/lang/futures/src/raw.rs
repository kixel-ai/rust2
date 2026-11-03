//! Wakers built by hand from a `RawWaker` and its vtable, each clone, wake, and drop calling
//! the vtable's function as std's does, which each logs: through `Waker::from_raw` and
//! `Waker::new`, cloned over another by `clone_from`, compared by `will_wake`, dropped
//! unpolled or forgotten, and polling a future to its end in an executor of its own; vtables
//! compared by `==`; and `RawWaker::from` an `Arc` of a `Wake` impl.

use std::cell::{Cell, RefCell};
use std::future::Future;
use std::mem::{self, ManuallyDrop};
use std::pin::{Pin, pin};
use std::ptr::{self, NonNull};
use std::sync::Arc;
use std::sync::atomic::{AtomicU8, Ordering};
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Wake, Waker};

/// The wakers `kind` picks.
pub fn raw(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	match kind % 5 {
		0 => calls(data, out),
		1 => clones(data, out),
		2 => executor(data, out),
		3 => vtables(data, out),
		_ => arcs(data, out),
	}
	out.extend(LOG.take());
}

thread_local! {
	/// Each vtable call: its function, its vtable, and whether its data is null.
	static LOG: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
	/// Whether the executor's waker woke since its last poll.
	static WOKEN: Cell<bool> = const { Cell::new(false) };
}

/// Log a call of function `call` of vtable `TABLE` on `data`.
fn note<const TABLE: u8>(call: u8, data: *const ()) {
	LOG.with_borrow_mut(|log| log.extend([call, TABLE, data.is_null().into()]));
}

unsafe fn on_clone<const TABLE: u8>(data: *const ()) -> RawWaker {
	note::<TABLE>(b'c', data);
	RawWaker::new(data, &A)
}

/// `B`'s clone, which clones into a waker of vtable `A`.
unsafe fn clone_into_a(data: *const ()) -> RawWaker {
	note::<b'b'>(b'c', data);
	RawWaker::new(data, &A)
}

unsafe fn on_wake<const TABLE: u8>(data: *const ()) {
	note::<TABLE>(b'w', data);
}

unsafe fn on_wake_by_ref<const TABLE: u8>(data: *const ()) {
	note::<TABLE>(b'r', data);
}

unsafe fn on_drop<const TABLE: u8>(data: *const ()) {
	note::<TABLE>(b'd', data);
}

static A: RawWakerVTable = RawWakerVTable::new(
	on_clone::<b'a'>,
	on_wake::<b'a'>,
	on_wake_by_ref::<b'a'>,
	on_drop::<b'a'>,
);
static B: RawWakerVTable = RawWakerVTable::new(clone_into_a, on_wake::<b'b'>, on_wake_by_ref::<b'b'>, on_drop::<b'b'>);
/// `A`'s functions in a static of its own, which `==` finds equal and `will_wake` apart.
static A_AGAIN: RawWakerVTable = RawWakerVTable::new(
	on_clone::<b'a'>,
	on_wake::<b'a'>,
	on_wake_by_ref::<b'a'>,
	on_drop::<b'a'>,
);

/// Data that is null, or dangling, which no other data is.
fn data_of(byte: u8) -> *const () {
	match byte % 2 {
		0 => ptr::null(),
		_ => NonNull::<()>::dangling().as_ptr(),
	}
}

/// A waker of vtable `A`, `B`, or `A_AGAIN`, as `byte` picks, made through `from_raw` or
/// `new`.
fn waker(byte: u8) -> Waker {
	let vtable = match byte % 3 {
		0 => &A,
		1 => &B,
		_ => &A_AGAIN,
	};
	let data = data_of(byte / 3);
	match byte / 6 % 2 {
		0 => unsafe { Waker::from_raw(RawWaker::new(data, vtable)) },
		_ => unsafe { Waker::new(data, vtable) },
	}
}

/// Each byte a waker, cloned, woken by reference and by value, dropped, forgotten, or never
/// used, and those it made dropped in reverse.
fn calls(data: &[u8], out: &mut Vec<u8>) {
	let mut kept = Vec::new();
	for &byte in data {
		let w = waker(byte);
		out.push(w.data().is_null().into());
		match byte / 12 % 6 {
			0 => w.wake_by_ref(),
			1 => w.wake(),
			2 => kept.push(w.clone()),
			3 => mem::forget(w),
			4 => {
				let clone = w.clone();
				clone.wake();
				w.wake_by_ref();
			}
			// Dropped unused.
			_ => {}
		}
	}
	while let Some(w) = kept.pop() {
		w.wake_by_ref();
	}
}

/// Wakers cloned over others by `clone_from`, which keeps one that would wake the same
/// task and else drops it after cloning the source; and `will_wake` of each pair.
fn clones(data: &[u8], out: &mut Vec<u8>) {
	let mut held = waker(0);
	let mut maybe: Option<Waker> = None;
	for pair in data.chunks(2) {
		let source = waker(pair[0]);
		let other = waker(pair.get(1).copied().unwrap_or(0));
		out.push(source.will_wake(&other).into());
		out.push(held.will_wake(&source).into());
		held.clone_from(&source);
		maybe.clone_from(&Some(other.clone()));
		out.push(held.will_wake(&source).into());
		// A `B` waker's clone is an `A` one, which would wake another task.
		out.push(source.will_wake(&source.clone()).into());
		LOG.with_borrow_mut(|log| log.push(b'|'));
	}
}

/// A future that is pending `self.0` times, waking its task by a clone of the waker it
/// holds until the next poll, or by reference, before each.
struct Yields(u8, Option<Waker>);

impl Future for Yields {
	type Output = u8;

	fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<u8> {
		if let Some(held) = self.1.take() {
			held.wake();
		}
		if self.0 == 0 {
			return Poll::Ready(0xaa);
		}
		self.0 -= 1;
		match self.0 % 3 {
			0 => cx.waker().wake_by_ref(),
			1 => self.1 = Some(cx.waker().clone()),
			_ => {
				let w = cx.waker().clone();
				w.wake_by_ref();
			}
		}
		Poll::Pending
	}
}

unsafe fn exec_clone(data: *const ()) -> RawWaker {
	note::<b'e'>(b'c', data);
	RawWaker::new(data, &EXEC)
}

unsafe fn exec_wake(data: *const ()) {
	note::<b'e'>(b'w', data);
	WOKEN.set(true);
}

unsafe fn exec_wake_by_ref(data: *const ()) {
	note::<b'e'>(b'r', data);
	WOKEN.set(true);
}

static EXEC: RawWakerVTable = RawWakerVTable::new(exec_clone, exec_wake, exec_wake_by_ref, on_drop::<b'e'>);

/// Poll `f` until it is ready, again only once its task wakes.
fn block_on<F: Future>(f: F) -> Option<F::Output> {
	let waker = unsafe { Waker::from_raw(RawWaker::new(ptr::null(), &EXEC)) };
	let mut cx = Context::from_waker(&waker);
	let mut f = pin!(f);
	loop {
		WOKEN.set(false);
		if let Poll::Ready(v) = f.as_mut().poll(&mut cx) {
			return Some(v);
		}
		if !WOKEN.get() {
			return None;
		}
	}
}

/// An `async` block awaiting a `Yields` of each byte, polled to its end by `block_on`.
fn executor(data: &[u8], out: &mut Vec<u8>) {
	let counts: Vec<u8> = data.iter().map(|b| b % 5).collect();
	let done = block_on(async {
		let mut sum = 0u8;
		for &n in counts.iter() {
			sum = sum.wrapping_add(Yields(n, None).await).wrapping_add(n);
		}
		sum
	});
	out.extend(done);
	// A future that never wakes its task stops the executor.
	out.push(block_on(std::future::pending::<u8>()).is_none().into());
}

/// `==` of vtables compares their functions, not where they are, as `will_wake` does.
fn vtables(data: &[u8], out: &mut Vec<u8>) {
	let tables = [&A, &B, &A_AGAIN, &EXEC];
	for &byte in data {
		let (l, r) = (tables[usize::from(byte % 4)], tables[usize::from(byte / 4 % 4)]);
		out.push((l == r).into());
		out.push((*l != *r).into());
		let (wl, wr) = (waker(byte % 3), waker(byte / 4 % 3));
		out.push((*wl.vtable() == *wr.vtable()).into());
		out.push(wl.will_wake(&wr).into());
		out.push((RawWaker::new(data_of(byte), l) == RawWaker::new(data_of(byte >> 1), r)).into());
		mem::forget((wl, wr));
	}
	let noop = Waker::noop();
	out.push((*noop.vtable() == *Waker::noop().vtable()).into());
	out.push((*noop.vtable() == A).into());
	out.push(noop.data().is_null().into());
	// A waker of another's parts, borrowed, which wakes as it does.
	let borrowed = ManuallyDrop::new(unsafe { Waker::new(noop.data(), noop.vtable()) });
	borrowed.wake_by_ref();
	out.push(borrowed.will_wake(noop).into());
}

/// Counts the wakes through it.
struct Counter(AtomicU8);

impl Wake for Counter {
	fn wake(self: Arc<Self>) {
		self.0.fetch_add(1, Ordering::Relaxed);
	}
}

/// `RawWaker::from` an `Arc` of a `Wake` impl, whose vtable is the one `Waker::from` gives,
/// and which counts the `Arc` as a waker does.
fn arcs(data: &[u8], out: &mut Vec<u8>) {
	let count = Arc::new(Counter(AtomicU8::new(0)));
	let from = Waker::from(count.clone());
	let raw = RawWaker::from(count.clone());
	let other = RawWaker::from(Arc::new(Counter(AtomicU8::new(0))));
	out.push((raw == RawWaker::new(from.data(), from.vtable())).into());
	out.push((raw == other).into());
	let w = unsafe { Waker::from_raw(raw) };
	out.push(w.will_wake(&from).into());
	out.push((*w.vtable() == *from.vtable()).into());
	out.push((*w.vtable() == A).into());
	mem::drop(unsafe { Waker::from_raw(other) });
	out.push(Arc::strong_count(&count) as u8);
	for &byte in data {
		match byte % 4 {
			0 => w.wake_by_ref(),
			1 => w.clone().wake(),
			2 => mem::drop(w.clone()),
			// Borrowed from its parts, which wakes the same `Arc` without counting it.
			_ => ManuallyDrop::new(unsafe { Waker::new(w.data(), w.vtable()) }).wake_by_ref(),
		}
		out.push(Arc::strong_count(&count) as u8);
	}
	mem::drop((w, from));
	out.extend([count.0.load(Ordering::Relaxed), Arc::strong_count(&count) as u8]);
}
