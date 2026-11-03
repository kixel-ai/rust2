//! std's calls of a closure that owns a guard: each call logged, and the guard dropped where
//! std drops the closure.

use std::cell::{Cell, LazyCell, RefCell};
use std::ops::ControlFlow;

use crate::log::Log;
use super::Key;
use crate::model::Guard;

/// Run scenario `arg % 12`. The last panics in a `try_fold`, with the log so far.
pub(super) fn called(log: &Log, id: u32, arg: u8) {
	let guard = |n: u32| Guard::new(id + n, log.clone());
	let guards = |from: u32, to: u32| (from..to).map(guard).collect::<Vec<_>>();
	let (c, d) = (guard(20), guard(21));
	match arg % 12 {
		0 => {
			// `try_fold` by `Option`, which breaks by `None` and reads on after, and drops
			// each item it reads.
			let mut it = guards(1, 6).into_iter();
			let n = it.try_fold(0u32, move |n, g| {
				log.note(format!("try {} {}", g.id(), c.id()));
				(g.id() != id + 3).then_some(n + g.id() - id)
			});
			log.note(format!("broke {n:?} {:?}", it.next().map(|g| g.id())));
			let m = it.try_fold(0u32, |n, g| Some(n + g.id() - id));
			log.note(format!("ended {m:?} {}", d.id()));
		}
		1 => {
			// By `Result`, through `map` and `chain`, which drop each half as it ends.
			let mut it = guards(1, 3).into_iter().chain(guards(3, 6)).map(move |g| (g, c.id()));
			let r = it.try_fold(0u32, |n, (g, _)| match g.id() == id + 4 {
				true => Err(g),
				false => Ok(n + g.id() - id),
			});
			log.note(format!("err {:?}", r.map_err(|g| g.id())));
			let r: Result<u32, ()> = it.try_fold(0, |n, (g, _)| Ok(n + g.id() - id));
			log.note(format!("ok {r:?} {}", d.id()));
		}
		2 => {
			// By `ControlFlow`, through `rev` and `filter`, and over none.
			let mut it = guards(1, 6).into_iter().rev().filter(move |g| g.id() != c.id() - 18);
			let r = it.try_fold(0u32, |n, g| match g.id() == id + 3 {
				true => ControlFlow::Break(g),
				false => ControlFlow::Continue(n + g.id() - id),
			});
			let broke = match r {
				ControlFlow::Break(g) => Some(g.id()),
				ControlFlow::Continue(_) => None,
			};
			log.note(format!("break {broke:?}"));
			let none = Vec::<Guard>::new().into_iter().try_fold(0u32, move |n, g| Some(n + g.id() + d.id()));
			log.note(format!("none {none:?}"));
		}
		3 => {
			// `try_for_each` by each type, which drops its closure as it returns.
			let mut it = guards(1, 6).into_iter();
			let r = it.try_for_each(move |g| match g.id() == id + 2 {
				true => Err(g.id() + c.id() - c.id()),
				false => Ok(log.note(format!("each {}", g.id()))),
			});
			log.note(format!("each {r:?}"));
			let r = it.try_for_each(|g| match g.id() == id + 4 {
				true => ControlFlow::Break(g.id()),
				false => ControlFlow::Continue(()),
			});
			log.note(format!("flow {r:?}"));
			let r = it.try_for_each(move |g| (g.id() != d.id()).then_some(()));
			log.note(format!("some {r:?}"));
		}
		4 => {
			// `partition`, which moves each item to a side and drops its closure as it ends.
			let (odd, even): (Vec<Guard>, Vec<Guard>) = guards(1, 7).into_iter().partition(move |g| {
				log.note(format!("side {} {}", g.id(), c.id()));
				g.id() % 2 == 1
			});
			log.note(format!("odd {:?}", odd.iter().map(Guard::id).collect::<Vec<_>>()));
			drop(even);
			let (all, none): (Vec<u32>, Vec<u32>) = (1..4).partition(|_| true);
			log.note(format!("all {all:?} {none:?}"));
			// Through `map`, whose fold drops `partition`'s closure before its own.
			let e = guard(22);
			let (low, high): (Vec<Guard>, Vec<Guard>) = guards(1, 4)
				.into_iter()
				.map(move |g| (g, e.id()).0)
				.partition(move |g| g.id() + d.id() - d.id() < id + 2);
			log.note(format!("sides {} {}", low.len(), high.len()));
		}
		5 => {
			// `repeat_with`, which drops its closure with the iterator, each item as it goes.
			let mut k = 0;
			let made: Vec<Guard> = std::iter::repeat_with(move || {
				k += 1;
				log.note(format!("make {k} {}", c.id()));
				guard(k)
			})
			.take(3)
			.collect();
			log.note(format!("made {}", made.len()));
			let mut it = std::iter::repeat_with(move || d.id());
			log.note(format!("first {:?} {:?} {:?}", it.next(), it.size_hint(), it.by_ref().take(2).size_hint()));
			drop(it);
			log.note("dropped".into());
		}
		6 => {
			// `scan` of a state JS copies, which stops at the first `None` and drops its
			// closure with the iterator.
			let mut it = guards(1, 7).into_iter().scan(0u32, move |sum, g| {
				*sum += g.id() - id;
				log.note(format!("scan {} {sum} {}", g.id(), c.id()));
				(*sum < 10).then_some(*sum)
			});
			let sums: Vec<u32> = it.by_ref().collect();
			log.note(format!("sums {sums:?} {:?}", it.next()));
			drop(it);
			let total: u32 = (1..4).scan(1u32, |n, k| {
				*n *= k;
				Some(*n)
			})
			.sum();
			log.note(format!("total {total}"));
			// Of a state that drops, after the closure, and folded through `flatten`.
			let e = guard(22);
			let n = list![guards(1, 3), guards(3, 6)]
				.into_iter()
				.flatten()
				.scan(guard(23), move |held, g| {
					log.note(format!("held {} {} {}", held.id(), g.id(), d.id()));
					(g.id() != id + 4).then_some(g.id() + e.id() - e.id())
				})
				.fold(0, |n, k| n + k - id);
			log.note(format!("flat {n}"));
		}
		7 => {
			// `sort_by_cached_key`, whose keys drop after the sort, and whose closure drops
			// as it returns.
			let mut v: Vec<Guard> = [3, 1, 4, 5, 2].map(guard).into_iter().collect();
			v.sort_by_cached_key(move |g| {
				log.note(format!("key {} {}", g.id(), c.id()));
				Key::new((g.id() * 7) % 5, log.clone())
			});
			log.note(format!("sorted {:?}", v.iter().map(Guard::id).collect::<Vec<_>>()));
			let mut w = vec![3u32, 1, 2];
			w.sort_by_cached_key(|&k| std::cmp::Reverse(k + d.id()));
			log.note(format!("rev {w:?}"));
		}
		8 => {
			// `LazyCell::new`, which drops its closure as the first `force` calls it, and
			// with the cell if none does.
			let lazy = LazyCell::new(move || {
				log.note(format!("force {}", c.id()));
				c.id() + 1
			});
			log.note("made".into());
			log.note(format!("forced {} {}", *lazy, *lazy));
			let unused = LazyCell::new(move || d.id());
			drop(unused);
			log.note("unused".into());
			// One that `Fn` calls, which drops after `force`'s call through `FnOnce`.
			let e = guard(22);
			let make = move || e.id();
			let lazy = LazyCell::new(make);
			log.note(format!("by Fn {}", *lazy));
		}
		9 => {
			// `Cell::update` and `RefCell::replace_with` of a value JS copies, each of which
			// drops its closure as it returns.
			let cell = Cell::new(1u32);
			cell.update(move |n| {
				log.note(format!("update {n} {}", c.id()));
				n + 1
			});
			log.note(format!("updated {}", cell.get()));
			let cell = RefCell::new(5u32);
			let old = cell.replace_with(|n| {
				log.note(format!("replace {n} {} {}", d.id(), cell.try_borrow().is_err()));
				*n += 1;
				*n * 2
			});
			log.note(format!("replaced {old} {}", cell.borrow()));
			// Each of one that `Fn` calls, which drops after their call through `FnOnce`.
			let (e, f) = (guard(22), guard(23));
			let add = move |n: u32| n + e.id() - e.id() + 1;
			let doubled = move |n: &mut u32| *n * 2 + f.id() - f.id();
			Cell::new(1u32).update(add);
			// Of what JS shares, which the function changes and the cell keeps.
			let pair = Cell::new([1u32, 2]);
			pair.update(|mut p| {
				p[0] += 10;
				log.note(format!("pair {} {}", p[0], pair.get()[0]));
				p
			});
			log.note(format!("paired {:?}", pair.get()));
			log.note(format!("by Fn {}", cell.replace_with(doubled)));
		}
		10 => {
			// Each over none: the closure never runs, and drops as std's does.
			let none = || Vec::<Guard>::new().into_iter();
			let r: Result<u32, u32> = none().try_fold(0, move |n, g| Ok(n + g.id() + c.id()));
			log.note(format!("fold {r:?}"));
			let r: Option<()> = none().try_for_each(move |g| (g.id() != d.id()).then_some(()));
			log.note(format!("each {r:?}"));
			let (a, b): (Vec<Guard>, Vec<Guard>) = none().partition(|g| g.id() > 0);
			log.note(format!("sides {} {}", a.len(), b.len()));
			let scanned: Vec<u32> = none().scan(0u32, |n, g| Some(*n + g.id())).collect();
			let mut empty: Vec<Guard> = Vec::new();
			empty.sort_by_cached_key(Guard::id);
			let mut one = guards(1, 2);
			let e = guard(22);
			one.sort_by_cached_key(move |g| g.id() + e.id());
			log.note(format!("scanned {} {} {}", scanned.len(), empty.len(), one.len()));
		}
		_ => {
			let a = guard(30);
			let _ = guards(1, 6).into_iter().try_fold(0, move |n, g| {
				assert!(
					g.id() != id + 3,
					"try_fold of g{} with g{} after\n{}",
					g.id(),
					a.id(),
					String::from_utf8(log.text()).unwrap()
				);
				Some(n + g.id())
			});
		}
	}
}
