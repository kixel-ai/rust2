//! std's own futures and what a poll gives: `future::ready`'s, polled, awaited, taken by
//! `into_inner`, cloned, and dropped with its value or after giving it, and polled twice;
//! `future::pending`'s, never ready however often polled; `poll_fn`'s; a `Poll` written,
//! made `From` a value, and hashed; and a waker cloned over another by `clone_from`.

use std::cell::RefCell;
use std::collections::HashSet;
use std::fmt::Debug;
use std::future::{Future, IntoFuture, Pending, Ready, pending, poll_fn, ready};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::pin::{Pin, pin};
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::{AtomicU8, Ordering};
use std::task::{Context, Poll, Wake, Waker};

/// The futures and polls `kind` picks.
pub fn futures(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	match kind % 8 {
		0 => readies(data, out),
		1 => dropped(data, out),
		2 => repolled(data, out),
		3 => reread(data, out),
		4 => pendings(data, out),
		5 => polls(data, out),
		6 => hashes(data, out),
		_ => wakers(data, out),
	}
}

/// A byte that writes itself into the log as it drops.
struct Logged(u8, Rc<RefCell<Vec<u8>>>);

impl Drop for Logged {
	fn drop(&mut self) {
		self.1.borrow_mut().push(self.0);
	}
}

/// What one poll of `f` gives, with a waker that does nothing.
fn poll_once<F: Future + ?Sized>(f: Pin<&mut F>) -> Poll<F::Output> {
	f.poll(&mut Context::from_waker(Waker::noop()))
}

/// `value` written by `Debug`, plain and with `#`, apart by a byte no UTF-8 holds.
fn debug(out: &mut Vec<u8>, value: &impl Debug) {
	crate::harness::put(out, &format!("{value:?}"));
	crate::harness::put(out, &format!("{value:#?}"));
}

/// The crate's own `IntoFuture`, whose future is std's `Ready`.
struct Later(u8);

impl IntoFuture for Later {
	type Output = u8;
	type IntoFuture = Ready<u8>;

	fn into_future(self) -> Ready<u8> {
		ready(self.0.wrapping_add(1))
	}
}

/// Each byte through a `Ready`, by a poll, `into_inner`, an `.await`, and a clone, with
/// each written before and after it gives its value.
fn readies(data: &[u8], out: &mut Vec<u8>) {
	for &b in data {
		let mut r = ready(b);
		debug(out, &r);
		let copy = r.clone();
		match poll_once(Pin::new(&mut r)) {
			Poll::Ready(v) => out.push(v),
			Poll::Pending => out.push(0xee),
		}
		// Taken, which a clone of it keeps.
		debug(out, &r);
		debug(out, &r.clone());
		out.push(copy.into_inner());
		let awaited = pin!(async move { ready(b ^ 0x5a).await.wrapping_add(Later(b).await) });
		match poll_once(awaited) {
			Poll::Ready(v) => out.push(v),
			Poll::Pending => out.push(0xee),
		}
	}
	// Of values JS keeps as `undefined`, nests in an `Option`, and holds as text.
	debug(out, &ready(()));
	debug(out, &ready(data.first().copied()));
	debug(out, &ready(None::<Option<u8>>));
	let mut text = ready(crate::harness::text(data));
	debug(out, &text);
	if let Poll::Ready(s) = poll_once(Pin::new(&mut text)) {
		out.extend(s.bytes());
	}
	debug(out, &text);
	// A `Ready` behind a `dyn Future`, which polls it in place.
	let mut boxed: Pin<Box<dyn Future<Output = usize>>> = Box::pin(ready(data.len()));
	if let Poll::Ready(n) = poll_once(boxed.as_mut()) {
		out.push(n as u8);
	}
}

/// A `Ready`'s value drops with it unless a poll or `into_inner` gave it away, in order with
/// the values around it.
fn dropped(data: &[u8], out: &mut Vec<u8>) {
	let log = Rc::new(RefCell::new(Vec::new()));
	for (i, &b) in data.iter().enumerate() {
		let before = Logged(b, log.clone());
		let mut r = ready(Logged(b ^ 0x80, log.clone()));
		let after = Logged(b ^ 0x40, log.clone());
		match i % 6 {
			// Never polled: it drops its value in reverse order of declaration.
			0 => {}
			1 => drop(r),
			2 => {
				if let Poll::Ready(v) = poll_once(Pin::new(&mut r)) {
					log.borrow_mut().push(0xf0);
					drop(v);
				}
				log.borrow_mut().push(0xf1);
			}
			3 => {
				let v = r.into_inner();
				drop(after);
				log.borrow_mut().push(v.0);
			}
			4 => {
				let clone = ready(Rc::new(Logged(b ^ 0x20, log.clone())));
				let copy = clone.clone();
				drop(clone);
				let count = Rc::strong_count(&copy.into_inner());
				log.borrow_mut().push(count as u8);
			}
			// Moved into an `async` block, which awaits it.
			_ => {
				let awaited = pin!(async move { r.await.0 });
				if let Poll::Ready(v) = poll_once(awaited) {
					log.borrow_mut().push(v);
				}
			}
		}
		drop(before);
	}
	out.extend(log.borrow().iter());
}

/// A `Ready` polled after giving its value panics, as std's does, as does `into_inner`.
fn repolled(data: &[u8], out: &mut Vec<u8>) {
	let mut r = ready(data.len() as u8);
	if let Poll::Ready(v) = poll_once(Pin::new(&mut r)) {
		out.push(v);
	}
	if data.first().is_some_and(|b| b % 2 == 0) {
		out.push(poll_once(Pin::new(&mut r)).is_ready().into());
	}
	out.push(r.into_inner());
}

/// `into_inner` of a `Ready` a poll already took.
fn reread(data: &[u8], out: &mut Vec<u8>) {
	let mut r = ready(data.to_vec());
	if data.len() % 2 == 0 {
		let _ = poll_once(Pin::new(&mut r));
	}
	out.extend(r.into_inner());
}

/// `pending()` is never ready, however often polled, through a box and cloned, and holds
/// nothing to drop; `poll_fn`'s future writes only its name.
fn pendings(data: &[u8], out: &mut Vec<u8>) {
	let log = Rc::new(RefCell::new(Vec::new()));
	let mut never: Pending<Logged> = pending();
	let mut boxed = Box::pin(pending::<u8>());
	for &b in data {
		let held = Logged(b, log.clone());
		out.push(poll_once(Pin::new(&mut never)).is_pending().into());
		out.push(poll_once(boxed.as_mut()).is_pending().into());
		let mut copy = never.clone();
		out.push(poll_once(Pin::new(&mut copy)).is_pending().into());
		drop(held);
	}
	debug(out, &never);
	debug(out, &never.clone());
	let n = data.len();
	debug(out, &poll_fn(move |_| Poll::Ready(n)));
	drop(never);
	out.extend(log.borrow().iter());
}

/// `Poll`s written, plain, with `#`, and through a width, which reaches what each holds,
/// and made `From` what they hold.
fn polls(data: &[u8], out: &mut Vec<u8>) {
	for &b in data {
		let p = Poll::from(b);
		let q: Poll<u8> = if b % 3 == 0 { Poll::Pending } else { b.into() };
		debug(out, &p);
		debug(out, &q);
		crate::harness::put(out, &format!("{q:>4?}|{p:<#5x?}|{p:08?}"));
		debug(out, &Poll::Ready(Some(b)).map(|o| o.filter(|v| v % 2 == 0)));
		let r: Poll<Result<u8, String>> = match b % 3 {
			0 => Poll::Pending,
			1 => Poll::Ready(Ok(b)),
			_ => Poll::Ready(Err(b.to_string())),
		};
		debug(out, &r);
		debug(out, &nested(b));
		debug(out, &Poll::from(()));
		debug(out, &Poll::from(crate::harness::text(data)));
		debug(out, &Poll::from([b, b ^ 1]));
	}
}

/// A `Poll` of a `Poll`, whose own `Pending` is apart from the one it holds.
fn nested(b: u8) -> Poll<Poll<u8>> {
	match b % 3 {
		0 => Poll::Pending,
		1 => Poll::Ready(Poll::Pending),
		_ => Poll::from(Poll::Ready(b)),
	}
}

/// What hashing a `Poll` writes: its variant, then what it holds, as std's derived `Hash`
/// writes them.
fn hashes(data: &[u8], out: &mut Vec<u8>) {
	let polls: Vec<Poll<u8>> = data
		.iter()
		.map(|&b| if b % 4 == 0 { Poll::Pending } else { Poll::Ready(b) })
		.collect();
	for p in &polls {
		let mut h = DefaultHasher::new();
		p.hash(&mut h);
		out.extend(h.finish().to_be_bytes());
	}
	let mut h = DefaultHasher::new();
	polls.hash(&mut h);
	Poll::Ready(Some(data.len())).hash(&mut h);
	Poll::<Option<String>>::Pending.hash(&mut h);
	for &b in data {
		nested(b).hash(&mut h);
	}
	out.extend(h.finish().to_be_bytes());
	// As a set's keys, `Pending` among them.
	let set: HashSet<Poll<u8>> = polls.iter().copied().collect();
	let nests: HashSet<Poll<Poll<u8>>> = data.iter().map(|&b| nested(b)).collect();
	out.push(set.len() as u8);
	out.push(nests.len() as u8);
	out.push(set.contains(&Poll::Pending).into());
	out.push(nests.contains(&Poll::Ready(Poll::Pending)).into());
	out.extend(data.iter().map(|&b| u8::from(set.contains(&Poll::Ready(b)))));
}

/// Counts the wakes through it.
struct Count(AtomicU8);

impl Wake for Count {
	fn wake(self: Arc<Self>) {
		self.0.fetch_add(1, Ordering::Relaxed);
	}
}

/// A waker a struct holds.
struct Held {
	waker: Waker,
}

/// A waker cloned over another by `clone_from` takes the source's place, unless both would
/// wake the same task, when std keeps the one it had: one in a variable and one in a field.
fn wakers(data: &[u8], out: &mut Vec<u8>) {
	let a = Arc::new(Count(AtomicU8::new(0)));
	let b = Arc::new(Count(AtomicU8::new(0)));
	let mut w = Waker::from(a.clone());
	let mut held = Held {
		waker: Waker::noop().clone(),
	};
	for &byte in data {
		let source = match byte % 3 {
			0 => Waker::from(a.clone()),
			1 => Waker::from(b.clone()),
			_ => Waker::noop().clone(),
		};
		w.clone_from(&source);
		held.waker.clone_from(&w);
		out.extend([Arc::strong_count(&a) as u8, Arc::strong_count(&b) as u8]);
		out.push(w.will_wake(&source).into());
		w.wake_by_ref();
		held.waker.wake_by_ref();
		drop(source);
		out.extend([Arc::strong_count(&a) as u8, Arc::strong_count(&b) as u8]);
	}
	drop(w);
	drop(held);
	out.extend([a.0.load(Ordering::Relaxed), b.0.load(Ordering::Relaxed)]);
	out.extend([Arc::strong_count(&a) as u8, Arc::strong_count(&b) as u8]);
}
