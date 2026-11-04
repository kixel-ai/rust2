//! Iterators held across an `.await`, as a `for` loop's is: a `Vec`'s, a slice's, a
//! range, a deque's, a B-tree's and a hash set's, a `Drain`, the crate's own, and adaptors
//! over them, each dropped after every number of polls up to its end, which drops what
//! each still holds.

use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet, HashSet, VecDeque};
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll, Waker};

/// The iterators `kind` picks, each over at most 16 bytes, dropped after each number of
/// polls until one is ready, with what each drop logs.
pub fn awaits(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let data = &data[..data.len().min(16)];
	let log = Rc::new(RefCell::new(Vec::new()));
	let items = || data.iter().map(|&b| Noted(b, log.clone()));
	match kind % 11 {
		0 => run(|| each(items().collect::<Vec<_>>()), &log, out),
		1 => run(|| each(items().collect::<VecDeque<_>>()), &log, out),
		// An iterator, as a whole B-tree drops in another order than std's.
		2 => run(|| each(items().collect::<BTreeSet<_>>().into_iter()), &log, out),
		3 => run(
			|| each(items().map(|l| (l.0, l)).collect::<BTreeMap<_, _>>().into_values()),
			&log,
			out,
		),
		4 => run(|| each(Countdown(data.len() as u8, log.clone())), &log, out),
		5 => run(
			|| each(items().collect::<Vec<_>>().into_iter().rev().skip(1).step_by(2)),
			&log,
			out,
		),
		6 => run(
			|| {
				let kept = items().collect::<Vec<_>>().into_iter().filter(|l| l.0 % 3 != 0);
				each(kept.chain(items().collect::<VecDeque<_>>()))
			},
			&log,
			out,
		),
		7 => run(|| drain(items().collect(), data.len() / 2), &log, out),
		8 => run(|| inline(data.to_vec()), &log, out),
		9 => run(|| borrowed(items().collect()), &log, out),
		_ => run(|| hashed(data.iter().copied().collect()), &log, out),
	}
}

/// A byte that writes itself into the log as it drops.
struct Noted(u8, Rc<RefCell<Vec<u8>>>);

impl Drop for Noted {
	fn drop(&mut self) {
		self.1.borrow_mut().push(self.0);
	}
}

impl PartialEq for Noted {
	fn eq(&self, other: &Self) -> bool {
		self.0 == other.0
	}
}

impl Eq for Noted {}

impl PartialOrd for Noted {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for Noted {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.0.cmp(&other.0)
	}
}

/// The crate's own iterator, counting down to zero, which logs as it drops.
struct Countdown(u8, Rc<RefCell<Vec<u8>>>);

impl Iterator for Countdown {
	type Item = Noted;

	fn next(&mut self) -> Option<Noted> {
		self.0 = self.0.checked_sub(1)?;
		Some(Noted(self.0, self.1.clone()))
	}
}

impl Drop for Countdown {
	fn drop(&mut self) {
		self.1.borrow_mut().push(0xc0 | self.0);
	}
}

/// Pending on its first poll, which wakes its task, and ready on the next.
struct Yield(bool);

impl Future for Yield {
	type Output = ();

	fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
		if self.0 {
			return Poll::Ready(());
		}
		self.0 = true;
		cx.waker().wake_by_ref();
		Poll::Pending
	}
}

/// Each future `make` makes, polled one more time than the last until one is ready, then
/// dropped: what it gives, or that it is still pending, and what dropping it logs.
fn run<F: Future<Output = u8>>(make: impl Fn() -> F, log: &RefCell<Vec<u8>>, out: &mut Vec<u8>) {
	let mut cx = Context::from_waker(Waker::noop());
	for polls in 0.. {
		let mut f = Box::pin(make());
		let ready = (0..polls).find_map(|_| match f.as_mut().poll(&mut cx) {
			Poll::Ready(v) => Some(v),
			Poll::Pending => None,
		});
		log.borrow_mut().push(0xfe);
		drop(f);
		out.push(ready.unwrap_or(0xee));
		out.append(&mut log.take());
		if ready.is_some() {
			return;
		}
	}
}

/// Each item of `items`, held across an `.await` with the iterator.
async fn each(items: impl IntoIterator<Item = Noted>) -> u8 {
	let mut sum = 0u8;
	for item in items {
		Yield(false).await;
		sum = sum.wrapping_add(item.0);
	}
	sum
}

/// A `Drain` of `v` from `from` on, held across an `.await`, then what is left of `v`.
async fn drain(mut v: Vec<Noted>, from: usize) -> u8 {
	let mut sum = 0u8;
	for item in v.drain(from..) {
		Yield(false).await;
		sum = sum.wrapping_add(item.0);
	}
	sum.wrapping_add(v.len() as u8)
}

/// `for n in v` in an `async` block, and a range, which step in place outside one.
fn inline(v: Vec<u8>) -> impl Future<Output = u8> {
	async move {
		let mut sum = 0u8;
		let len = v.len();
		for n in v {
			sum = sum.wrapping_add(std::future::ready(n).await);
			Yield(false).await;
		}
		for i in 0..len {
			Yield(false).await;
			sum = sum.wrapping_mul(3).wrapping_add(i as u8);
		}
		sum
	}
}

/// A `Vec` iterated by reference, shared and then mutable, across an `.await`.
async fn borrowed(mut v: Vec<Noted>) -> u8 {
	let mut sum = 0u8;
	for item in &mut v {
		Yield(false).await;
		item.0 = item.0.wrapping_add(1);
	}
	for item in &v {
		Yield(false).await;
		sum = sum.wrapping_add(item.0);
	}
	for item in v.iter().rev() {
		Yield(false).await;
		sum = sum.wrapping_mul(7).wrapping_add(item.0);
	}
	sum
}

/// A hash set's items, whose order is the hasher's, so only what no order changes is
/// written.
async fn hashed(set: HashSet<u8>) -> u8 {
	let (mut sum, mut count) = (0u8, 0u8);
	for n in set {
		Yield(false).await;
		sum = sum.wrapping_add(n);
		count += 1;
	}
	sum ^ count
}
