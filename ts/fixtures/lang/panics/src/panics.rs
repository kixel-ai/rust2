//! `std::panic` under wasm32's `panic=abort`: `AssertUnwindSafe` made, read and written
//! through `Deref`, `DerefMut`, and its field, called, awaited, printed, defaulted, and
//! converted into, dropping what it holds where Rust drops it; `catch_unwind`, which only
//! ever returns `Ok` or panics on, of closures, functions, and wrapped closures that drop
//! what they own; `panic_any`, whose hook writes a payload of text and `Box<dyn Any>` for any
//! other; `resume_unwind`, which aborts without the hook; and `Debug` of a `dyn Any`.

use std::any::Any;
use std::cell::RefCell;
use std::future::Future;
use std::panic::{self, AssertUnwindSafe};
use std::pin::pin;
use std::task::{Context, Poll, Waker};

/// Logs its byte as it drops.
struct Logged<'a>(u8, &'a RefCell<Vec<u8>>);

impl Drop for Logged<'_> {
	fn drop(&mut self) {
		self.1.borrow_mut().push(self.0);
	}
}

/// A point, which JS copies.
#[derive(Clone, Copy, Debug, Default)]
struct Point {
	x: u8,
	y: u8,
}

fn seven() -> u8 {
	7
}

/// What `f` returns, through `catch_unwind`.
fn caught<R>(f: impl FnOnce() -> R + panic::UnwindSafe) -> R {
	match panic::catch_unwind(f) {
		Ok(r) => r,
		Err(_) => unreachable!("a panic aborts"),
	}
}

/// `AssertUnwindSafe` of values, read and written as what it holds.
pub fn wrapped(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let first = data.first().copied().unwrap_or(0);
	match kind % 6 {
		0 => {
			let mut w = AssertUnwindSafe(data.len());
			*w += 1;
			out.push(*w as u8);
			w.0 = w.0.wrapping_mul(3);
			out.push(w.0 as u8);
			let r = &mut *w;
			*r += usize::from(first);
			out.push(*w as u8);
		}
		1 => {
			let mut v = AssertUnwindSafe(data.to_vec());
			v.push(first);
			v.0.push(9);
			v.retain(|b| b % 2 == 0);
			out.extend_from_slice(&v);
			out.push(v.len() as u8);
		}
		2 => {
			let mut p = AssertUnwindSafe(Point { x: first, y: 2 });
			p.x = p.x.wrapping_add(1);
			p.0.y = p.y.wrapping_mul(first);
			let copy = *p;
			p.x = 0;
			out.extend([copy.x, copy.y, p.x, p.y]);
			let mut nested = AssertUnwindSafe(AssertUnwindSafe(first));
			**nested = nested.wrapping_add(5);
			out.push(nested.0.0);
		}
		3 => {
			let text = crate::harness::text(data);
			let s = format!(
				"{:?}|{:#?}|{:?}|{:5?}|{:?}|{:#?}",
				AssertUnwindSafe(first),
				AssertUnwindSafe(first),
				AssertUnwindSafe(&text),
				AssertUnwindSafe(-1i8),
				AssertUnwindSafe(AssertUnwindSafe(Point { x: first, y: 0 })),
				AssertUnwindSafe((first, "a")),
			);
			out.extend_from_slice(s.as_bytes());
		}
		4 => {
			let d: AssertUnwindSafe<u8> = Default::default();
			let s: AssertUnwindSafe<String> = AssertUnwindSafe::default();
			let p: AssertUnwindSafe<Point> = Default::default();
			let v: AssertUnwindSafe<Vec<u8>> = Default::default();
			let e: AssertUnwindSafe<u8> = first.into();
			let f = AssertUnwindSafe::from(data.to_vec());
			out.extend([d.0, s.len() as u8, p.x, p.y, v.len() as u8, *e, f.len() as u8]);
		}
		_ => {
			// Each drops where Rust drops it: moved into a function, replaced, and at the end.
			let log = RefCell::new(Vec::new());
			let a = AssertUnwindSafe(Logged(1, &log));
			let mut b = AssertUnwindSafe(Logged(2, &log));
			let c = AssertUnwindSafe(AssertUnwindSafe(Logged(3, &log)));
			drop(a);
			*b = Logged(4, &log);
			b.0 = Logged(5, &log);
			if first % 2 == 0 {
				drop(c);
			}
			log.borrow_mut().push(b.0.0);
			drop(b);
			out.extend(log.borrow().iter());
		}
	}
}

/// `AssertUnwindSafe` of functions called, and `catch_unwind` of each kind of function.
pub fn called(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let first = data.first().copied().unwrap_or(0);
	let log = &RefCell::new(Vec::new());
	match kind % 7 {
		0 => {
			let n = AssertUnwindSafe(|| data.len())();
			let s = AssertUnwindSafe(seven)();
			// A closure that writes what it captured, called once through the wrapper.
			let mut count = first;
			AssertUnwindSafe(|| count = count.wrapping_add(3))();
			out.extend([n as u8, s, count]);
		}
		1 => {
			// What the closure owns drops after it returns, as rustc's shim does, or as it moves
			// out.
			let owned = Logged(1, log);
			let kept = Logged(2, log);
			let r = AssertUnwindSafe(move || {
				log.borrow_mut().push(10 + owned.0);
				first
			})();
			log.borrow_mut().push(r);
			let back = AssertUnwindSafe(move || kept)();
			log.borrow_mut().push(20 + back.0);
			drop(back);
			out.extend(log.borrow().iter());
		}
		2 => {
			out.push(caught(|| data.len() as u8));
			out.push(caught(seven));
			out.push(panic::catch_unwind(seven).unwrap());
			let v = caught(|| data.iter().rev().copied().collect::<Vec<u8>>());
			out.extend_from_slice(&v);
			let s = caught(|| format!("{first}"));
			out.extend_from_slice(s.as_bytes());
		}
		3 => {
			// A closure borrowed, and one that writes what it borrows, wrapped.
			let f = || first.wrapping_mul(2);
			out.push(caught(&f));
			let mut sum = 0u8;
			let r = panic::catch_unwind(AssertUnwindSafe(|| {
				for &b in data {
					sum = sum.wrapping_add(b);
				}
				sum
			}));
			out.push(r.unwrap());
			out.push(sum);
		}
		4 => {
			let owned = Logged(1, log);
			let r = panic::catch_unwind(AssertUnwindSafe(move || {
				log.borrow_mut().push(10 + owned.0);
				owned.0
			}));
			log.borrow_mut().push(30 + r.unwrap());
			let boxed: Box<dyn FnOnce() -> u8 + panic::UnwindSafe> = Box::new(move || first.wrapping_add(1));
			log.borrow_mut().push(caught(boxed));
			out.extend(log.borrow().iter());
		}
		5 => {
			let r = panic::catch_unwind(|| data.len() as u8);
			let s = format!("{r:?} {:?} {}", r.is_ok(), r.as_ref().map_or(0, |n| *n));
			out.extend_from_slice(s.as_bytes());
		}
		_ => {
			// A panic inside aborts, so nothing catches it, and what was owned never drops.
			let owned = Logged(1, log);
			let r = panic::catch_unwind(AssertUnwindSafe(|| {
				if first % 3 == 0 {
					panic!("caught {}", owned.0);
				}
				first
			}));
			out.push(r.unwrap());
			drop(owned);
			out.extend(log.borrow().iter());
		}
	}
}

/// A future to poll: ready after `n` polls, waking itself on each that is pending.
struct Countdown(u8);

impl Future for Countdown {
	type Output = u8;

	fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<u8> {
		if self.0 == 0 {
			return Poll::Ready(7);
		}
		self.0 -= 1;
		cx.waker().wake_by_ref();
		Poll::Pending
	}
}

/// `AssertUnwindSafe` of futures, polled by hand and awaited.
pub fn awaited(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let first = data.first().copied().unwrap_or(0);
	let log = RefCell::new(Vec::new());
	let mut cx = Context::from_waker(Waker::noop());
	match kind % 3 {
		0 => {
			let mut f = pin!(AssertUnwindSafe(Countdown(first % 4)));
			let mut polls = 0u8;
			loop {
				polls += 1;
				if let Poll::Ready(v) = f.as_mut().poll(&mut cx) {
					out.extend([v, polls]);
					break;
				}
			}
		}
		1 => {
			let n = first % 3;
			let fut = async move {
				let inner = AssertUnwindSafe(async { Countdown(n).await + 1 }).await;
				AssertUnwindSafe(Countdown(0)).await + inner
			};
			let mut f = pin!(AssertUnwindSafe(fut));
			let mut polls = 0u8;
			while f.as_mut().poll(&mut cx).is_pending() {
				polls += 1;
			}
			out.push(polls);
		}
		_ => {
			// What a future owns drops with it, polled to the end or not.
			let a = Logged(1, &log);
			let b = Logged(2, &log);
			let unpolled = AssertUnwindSafe(async move { a.0 });
			let mut done = pin!(AssertUnwindSafe(async move {
				Countdown(1).await;
				b.0
			}));
			while let Poll::Pending = done.as_mut().poll(&mut cx) {
				log.borrow_mut().push(9);
			}
			if first % 2 == 0 {
				drop(unpolled);
			}
			log.borrow_mut().push(0);
			out.extend(log.borrow().iter());
		}
	}
}

/// Panics with a payload, and `Debug` of a `dyn Any`.
pub fn raised(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let first = data.first().copied().unwrap_or(0);
	let log = RefCell::new(Vec::new());
	match kind % 8 {
		0 => panic::panic_any("a payload of text"),
		1 => panic::panic_any(crate::harness::text(data)),
		2 => panic::panic_any(first),
		3 => panic::panic_any(Logged(first, &RefCell::new(Vec::new())).0),
		4 => panic::resume_unwind(Box::new(first)),
		5 => panic::resume_unwind(Box::new("text")),
		6 => {
			// What a panic would drop never drops, as it aborts.
			let _held = Logged(1, &log);
			if first % 2 == 0 {
				panic::panic_any(Point { x: first, y: 0 });
			}
			out.push(first);
		}
		_ => {
			let boxed: Box<dyn Any + Send> = Box::new(first);
			let any: &dyn Any = &first;
			let synced: Box<dyn Any + Send + Sync> = Box::new(String::new());
			let s = format!("{boxed:?}|{any:#?}|{synced:>20?}|{:?}", AssertUnwindSafe(&*boxed));
			out.extend_from_slice(s.as_bytes());
		}
	}
	out.extend(log.borrow().iter());
}
