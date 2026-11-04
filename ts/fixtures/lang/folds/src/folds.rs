//! An inclusive range borrowed by `by_ref` and read through each of std's adaptors and
//! consumers, after which it is printed: its bounds, whether it is empty, and its `Debug`,
//! which says whether it is exhausted. Where std's adaptor folds the range, as `try_fold`
//! and `try_rfold` step it, `start` stops at `end`, exhausted; where it reads by `next`,
//! `start` passes `end`, and only wrapping exhausts it.

use std::fmt::Debug;
use std::ops::RangeInclusive;

use crate::harness::put;

/// `r`'s state after `v`, what read it returned.
fn state<T: Debug + PartialOrd>(out: &mut Vec<u8>, v: impl Debug, r: &RangeInclusive<T>) {
	put(
		out,
		&format!("{v:?} {:?} {:?} {} {r:?}", r.start(), r.end(), r.is_empty()),
	);
}

/// Each read of `$lo..=$hi`, a fresh range of each, stopping early at `$k`, the item
/// `$n` past the start, if any, of `$t`s.
macro_rules! folds {
	($out:ident, $t:ty, $lo:expr, $hi:expr, $n:expr) => {{
		let (lo, hi, n): ($t, $t, usize) = ($lo, $hi, $n);
		let k = (lo..=hi).nth(n);
		let is_k = |x: &$t| Some(*x) == k;
		let below = |x: &$t| match k {
			Some(k) => *x < k,
			None => true,
		};
		macro_rules! read {
			(|$r:ident| $e:expr) => {{
				let mut $r = lo..=hi;
				let v = $e;
				state($out, v, &$r);
			}};
		}
		// `for` reads by `next`, as does a `break` out of it.
		read!(|r| for _ in r.by_ref() {});
		read!(|r| {
			for x in r.by_ref() {
				if is_k(&x) {
					break;
				}
			}
		});
		// The range's own folds, read from the back too.
		read!(|r| r.rfind(is_k));
		read!(|r| r.by_ref().rfind(|_| false));
		// `map` and `inspect` fold their iterator, and so do the `fold`s through them.
		read!(|r| r.by_ref().map(|x| x).count());
		read!(|r| r.by_ref().map(|x| x).last());
		read!(|r| r.by_ref().map(|x| x).position(|x| is_k(&x)));
		read!(|r| r.by_ref().map(|x| x).rfind(is_k));
		read!(|r| r.by_ref().map(|x| x).fold(0usize, |n, _| n + 1));
		// `nth` and `nth_back` advance it by `advance_by`, which folds.
		read!(|r| r.by_ref().map(|x| x).nth(n));
		read!(|r| r.by_ref().map(|x| x).nth_back(n));
		read!(|r| r.by_ref().map(|x| x).rev().max());
		read!(|r| r.by_ref().map(|x| x).collect::<Vec<_>>().len());
		read!(|r| r.by_ref().inspect(|_| {}).count());
		read!(|r| r.by_ref().inspect(|_| {}).any(|x| is_k(&x)));
		// Not `TrustedLen`, so `collect` reads it by `next`.
		read!(|r| r.by_ref().inspect(|_| {}).collect::<Vec<_>>().len());
		// `filter` and `filter_map` fold it in `next` too, through `find` and `find_map`.
		read!(|r| r.by_ref().filter(|x| !is_k(x)).count());
		read!(|r| r.by_ref().filter(is_k).next());
		read!(|r| r.by_ref().filter(is_k).next_back());
		read!(|r| r.by_ref().filter(|_| true).collect::<Vec<_>>().len());
		read!(|r| r.by_ref().filter_map(|x| (!is_k(&x)).then_some(x)).count());
		read!(|r| r.by_ref().filter_map(|x| is_k(&x).then_some(x)).next());
		read!(|r| r.by_ref().filter_map(|x| is_k(&x).then_some(x)).next_back());
		// `enumerate` counts as it folds.
		read!(|r| r.by_ref().enumerate().count());
		read!(|r| r.by_ref().enumerate().position(|(_, x)| is_k(&x)));
		read!(|r| r.by_ref().enumerate().last());
		// `chain` folds each in turn.
		read!(|r| r.by_ref().chain(lo..=lo).count());
		read!(|r| (hi..=hi).chain(r.by_ref()).last());
		read!(|r| r.by_ref().chain(hi..=hi).any(|x| is_k(&x)));
		read!(|r| r.by_ref().chain(lo..=lo).rev().last());
		// `rev` folds it from the back.
		read!(|r| r.by_ref().rev().count());
		read!(|r| r.by_ref().rev().position(|x| is_k(&x)));
		read!(|r| r.by_ref().rev().min());
		read!(|r| r.by_ref().rev().rev().last());
		// `take` folds only up to its limit, and so does its `count`, by `advance_by`.
		read!(|r| r.by_ref().take(n).last());
		read!(|r| r.by_ref().take(n + 1).max());
		read!(|r| r.by_ref().take(n).count());
		read!(|r| r.by_ref().take(n).map(|x| x).count());
		read!(|r| r.by_ref().take(usize::MAX).min());
		read!(|r| r.by_ref().take(n).enumerate().count());
		// `skip` passes what it skips by `nth`, one short where it then folds.
		read!(|r| r.by_ref().skip(n).next());
		read!(|r| r.by_ref().skip(n).count());
		read!(|r| r.by_ref().skip(n).last());
		read!(|r| r.by_ref().skip(n).max());
		read!(|r| r.by_ref().skip(n).collect::<Vec<_>>().len());
		// `skip_while` finds its first by `find`; `take_while` and `map_while` fold.
		read!(|r| r.by_ref().skip_while(below).next());
		read!(|r| r.by_ref().skip_while(below).count());
		read!(|r| r.by_ref().take_while(below).count());
		read!(|r| r.by_ref().take_while(|_| true).last());
		read!(|r| r.by_ref().take_while(below).collect::<Vec<_>>().len());
		read!(|r| r.by_ref().map_while(|x| below(&x).then_some(x)).count());
		read!(|r| r.by_ref().map_while(|x| Some(x)).last());
		read!(|r| r.by_ref().map_while(|x| Some(x)).collect::<Vec<_>>().len());
		// `fuse` and `peekable` fold their iterator, `peekable` after what it peeked.
		read!(|r| r.by_ref().fuse().count());
		read!(|r| r.by_ref().fuse().rev().any(|x| is_k(&x)));
		read!(|r| {
			let mut p = r.by_ref().peekable();
			let peeked = p.peek().copied();
			(peeked, p.count())
		});
		read!(|r| {
			let mut p = r.by_ref().peekable();
			let peeked = p.peek().copied();
			(peeked, p.rev().last())
		});
		read!(|r| {
			let mut p = r.by_ref().peekable();
			let peeked = p.peek().copied();
			(peeked, p.position(|x| is_k(&x)))
		});
		read!(|r| r.by_ref().peekable().last());
		// `for` over a `&mut` to a `Peekable`, which then peeks on from where it stopped.
		read!(|r| {
			let mut p = r.by_ref().peekable();
			for x in p.by_ref() {
				if is_k(&x) {
					break;
				}
			}
			p.peek().copied()
		});
		// `flat_map` and `flatten` fold their source, and each iterator it makes.
		read!(|r| r.by_ref().flat_map(|x| Some(x)).count());
		read!(|r| r.by_ref().flat_map(|x| Some(x)).next());
		read!(|r| r.by_ref().map(|x| lo..=x).flatten().count());
		read!(|r| r.by_ref().flat_map(|x| x..=hi).position(|x| is_k(&x)));
		read!(|r| r.by_ref().flat_map(|x| [x, x]).rev().last());
		// `step_by` and `zip` read it by `nth` and `next`, as std's do, but a `zip` of two
		// `TrustedLen` iterators folds only as many pairs as its size hint's upper bound.
		read!(|r| r.by_ref().step_by(n + 1).count());
		read!(|r| r.by_ref().zip(0..n).any(|_| false));
		read!(|r| r.by_ref().zip(0..n).count());
		read!(|r| r.by_ref().zip(0..n).last());
		read!(|r| r.by_ref().zip(0..n).map(|(x, _)| x).collect::<Vec<_>>().len());
		read!(|r| r.by_ref().zip(0..n).position(|_| false));
		// Collected as `Result`s, read by `try_fold` until the first error.
		read!(|r| r.by_ref().map(|x| Ok::<$t, ()>(x)).collect::<Result<Vec<_>, _>>().map(|v| v.len()));
		read!(|r| {
			let items = r.by_ref().map(|x| if is_k(&x) { None } else { Some(x) });
			items.collect::<Option<Vec<_>>>().map(|v| v.len())
		});
		// `extend` folds a `TrustedLen` iterator and reads others by `next`.
		read!(|r| {
			let mut v = Vec::new();
			v.extend(r.by_ref().map(|x| x));
			v.len()
		});
		read!(|r| {
			let mut v = Vec::new();
			v.extend(r.by_ref().take_while(|_| true));
			v.len()
		});
	}};
}

/// What only a range of a type with an `ExactSizeIterator` can do: `take` and `skip`
/// from the back, `rposition`, and `enumerate` from the back.
macro_rules! exact {
	($out:ident, $lo:expr, $hi:expr, $n:expr) => {{
		let (lo, hi, n): (u8, u8, usize) = ($lo, $hi, $n);
		let k = (lo..=hi).nth(n);
		let is_k = |x: u8| Some(x) == k;
		macro_rules! read {
			(|$r:ident| $e:expr) => {{
				let mut $r = lo..=hi;
				let v = $e;
				state($out, v, &$r);
			}};
		}
		read!(|r| r.rposition(is_k));
		read!(|r| r.by_ref().rposition(|_| false));
		read!(|r| r.by_ref().take(n).rev().last());
		read!(|r| r.by_ref().take(n).rposition(is_k));
		read!(|r| r.by_ref().skip(n).rev().last());
		read!(|r| r.by_ref().skip(n).rposition(is_k));
		read!(|r| r.by_ref().enumerate().rev().last());
		read!(|r| r.by_ref().enumerate().rposition(|(_, x)| is_k(x)));
	}};
}

/// Each read of empty, single, and full ranges of the integer type `kind` picks, and at
/// its limits.
pub fn folds(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let at = |i: usize| data.get(i).copied().unwrap_or(0);
	let (a, b, n) = (at(0), at(1), at(2) as usize % 6);
	match kind % 2 {
		0 => {
			for (lo, hi) in [
				(a, b),
				(a, a),
				(b, a),
				(u8::MAX - a % 4, u8::MAX),
				(u8::MAX, u8::MAX),
				(0, a % 4),
			] {
				folds!(out, u8, lo, hi, n);
				exact!(out, lo, hi, n);
			}
		}
		_ => {
			let (a, b) = (a as i64 - 128, b as i64 - 128);
			let low = i64::MIN + (at(0) % 4) as i64;
			let high = i64::MAX - (at(1) % 4) as i64;
			for (lo, hi) in [
				(a, b),
				(a, a),
				(b, a),
				(i64::MIN, low),
				(i64::MIN, i64::MIN),
				(high, i64::MAX),
			] {
				folds!(out, i64, lo, hi, n);
			}
		}
	}
}
