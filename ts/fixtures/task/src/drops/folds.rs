//! What std's by-value consumers drop of an iterator whose adaptors hold closures that
//! drop beside items that drop: `fold` drops each adaptor's closure as its own fold ends,
//! before the source it wraps, where dropping the adaptor drops it after.

use std::collections::{BTreeMap, VecDeque};

use crate::log::Log;
use crate::model::Guard;

/// Run scenario `arg % 42`. Items are guards 1 to 7; what closures own, 10 and up.
pub(super) fn folded(log: &Log, id: u32, arg: u8) {
	let guard = |n: u32| Guard::new(id + n, log.clone());
	let guards = |from: u32, to: u32| (from..to).map(guard).collect::<Vec<_>>();
	let (c, d) = (guard(10), guard(11));
	match arg % 42 {
		0 => {
			let n = guards(1, 5).into_iter().map(move |g| g.id() + c.id()).count();
			let sum: u32 = guards(5, 8).into_iter().map(move |g| g.id() - id + d.id() - id).sum();
			log.note(format!("count {n} {sum}"));
		}
		1 => {
			// An accumulator that drops.
			let kept = guards(1, 7).into_iter().filter(move |g| g.id() % 2 == c.id() % 2).fold(
				None,
				|kept: Option<Guard>, g| {
					log.note(format!("fold {} {:?}", g.id(), kept.as_ref().map(|k| k.id())));
					if g.id() % 4 == 0 { kept } else { Some(g) }
				},
			);
			log.note(format!("fold {:?} {}", kept.map(|k| k.id()), d.id()));
		}
		2 => {
			guards(1, 7)
				.into_iter()
				.filter_map(move |g| (g.id() % 3 != c.id() % 3).then_some(g))
				.for_each(|g| log.note(format!("each {}", g.id())));
			log.note(format!("each {}", d.id()));
		}
		3 => {
			let last = guards(1, 5)
				.into_iter()
				.inspect(move |g| {
					log.note(format!("inspect {} {}", g.id(), c.id()));
				})
				.last();
			log.note(format!("last {:?} {}", last.map(|g| g.id()), d.id()));
		}
		4 => {
			let best = guards(1, 6)
				.into_iter()
				.map(move |g| {
					log.note(format!("map {} {}", g.id(), c.id()));
					g
				})
				.max_by_key(|g| g.id() % 3);
			let least = guards(6, 8).into_iter().min_by_key(|g| g.id() + d.id());
			log.note(format!("best {:?} {:?}", best.map(|g| g.id()), least.map(|g| g.id())));
		}
		5 => {
			let max = guards(1, 4).into_iter().map(move |g| g.id() % c.id()).max();
			let product: u32 = guards(4, 7)
				.into_iter()
				.map(move |g| g.id() - id + d.id() - d.id())
				.product();
			log.note(format!("max {max:?} {product}"));
		}
		6 => {
			// In place of the `Vec` it reads, and not.
			let same: Vec<Guard> = guards(1, 4)
				.into_iter()
				.map(move |g| {
					log.note(format!("map {} {}", g.id(), c.id()));
					g
				})
				.collect();
			let wider: Vec<(Guard, u64)> = guards(4, 7).into_iter().map(move |g| (g, u64::from(d.id()))).collect();
			log.note(format!("collected {} {}", same.len(), wider.len()));
		}
		7 => {
			let queue: VecDeque<Guard> = guards(1, 4)
				.into_iter()
				.map(move |g| {
					log.note(format!("map {} {}", g.id(), c.id()));
					g
				})
				.collect();
			let tree: BTreeMap<u32, Guard> = guards(4, 7).into_iter().map(move |g| (g.id() + d.id(), g)).collect();
			log.note(format!("collected {} {}", queue.len(), tree.len()));
		}
		8 => {
			let mut v = guards(1, 2);
			v.extend(guards(2, 4).into_iter().map(move |g| {
				log.note(format!("map {} {}", g.id(), c.id()));
				g
			}));
			let mut tree = BTreeMap::new();
			tree.extend(guards(4, 7).into_iter().map(move |g| (g.id() + d.id(), g)));
			log.note(format!("extended {} {}", v.len(), tree.len()));
		}
		9 => {
			// Two closures that drop, and nothing else.
			let n = [1u32, 2, 3, 4, 5]
				.iter()
				.map(move |&n| n + c.id())
				.filter(move |n| n % 2 == d.id() % 2)
				.count();
			log.note(format!("count {n}"));
		}
		10 => {
			// Two closures beside items: a `filter`'s drops before what it folds into.
			let n: u32 = guards(1, 5)
				.into_iter()
				.filter(move |g| g.id() != c.id())
				.map(move |g| g.id() - id + d.id() - d.id())
				.sum();
			let (e, f) = (guard(12), guard(13));
			let m = guards(5, 8)
				.into_iter()
				.filter_map(move |g| (g.id() != e.id()).then_some(g))
				.inspect(move |g| log.note(format!("inspect {} {}", g.id(), f.id())))
				.count();
			log.note(format!("two {n} {m}"));
		}
		11 => {
			// Through a chain, which folds each half in turn.
			let n: u32 = guards(1, 3)
				.into_iter()
				.chain(guards(3, 5))
				.map(move |g| g.id() - id + c.id() - id)
				.sum();
			let m = guards(5, 7)
				.into_iter()
				.map(move |g| g.id() + d.id())
				.chain([1u32, 2])
				.count();
			log.note(format!("chain {n} {m}"));
		}
		12 => {
			let n = guards(1, 4)
				.into_iter()
				.chain(guards(4, 7).into_iter().filter(move |g| g.id() != c.id()))
				.fold(0, |n, g| n + g.id() - id);
			let m = guards(7, 8).into_iter().chain(Vec::new()).last();
			log.note(format!("chain {n} {:?} {}", m.map(|g| g.id()), d.id()));
		}
		13 => {
			// Through what reads past items or reorders them.
			let n = guards(1, 8)
				.into_iter()
				.map(move |g| {
					log.note(format!("map {} {}", g.id(), c.id()));
					g
				})
				.rev()
				.skip(1)
				.step_by(2)
				.take(2)
				.fold(0, |n, g| n + g.id() - id);
			log.note(format!("read {n} {}", d.id()));
		}
		14 => {
			let n: usize = guards(1, 4)
				.into_iter()
				.enumerate()
				.map(move |(i, _)| i + c.id() as usize)
				.sum();
			let last = guards(4, 7)
				.into_iter()
				.map(move |g| {
					log.note(format!("map {} {}", g.id(), d.id()));
					g
				})
				.enumerate()
				.last();
			log.note(format!("enumerate {n} {:?}", last.map(|(i, g)| (i, g.id()))));
		}
		15 => {
			// A peeked item, and a fuse.
			let mut peek = guards(1, 5)
				.into_iter()
				.map(move |g| {
					log.note(format!("map {} {}", g.id(), c.id()));
					g
				})
				.peekable();
			log.note(format!("peek {:?}", peek.peek().map(|g| g.id())));
			let n = peek.fold(0, |n, g| n + g.id() - id);
			let m = guards(5, 8)
				.into_iter()
				.fuse()
				.filter(move |g| g.id() != d.id())
				.fold(0, |n, g| n + g.id());
			log.note(format!("peek {n} {m}"));
		}
		16 => {
			// A `skip_while` folds what it wraps, and drops its closure after.
			let n: u32 = guards(1, 5)
				.into_iter()
				.skip_while(move |g| g.id() < c.id() - 8)
				.map(|g| g.id() - id)
				.sum();
			let m: u32 = guards(5, 8)
				.into_iter()
				.skip_while(move |g| g.id() < d.id() - 5)
				.take_while(|g| g.id() < id + 7)
				.map(|g| g.id() - id)
				.sum();
			log.note(format!("skip {n} {m}"));
		}
		17 => {
			// What std folds by `try_fold`, which leaves its closure to the adaptor.
			let n: u32 = guards(1, 5)
				.into_iter()
				.map_while(move |g| (g.id() != c.id() - 7).then(|| g.id() - id))
				.sum();
			let m = guards(5, 8)
				.into_iter()
				.take_while(move |g| g.id() != d.id() - 4)
				.count();
			log.note(format!("while {n} {m}"));
		}
		18 => {
			// What a `flat_map` folds: its closure, and each iterator it makes.
			let n = [1u32, 3, 5]
				.into_iter()
				.flat_map(move |n| {
					let v = list![guard(n), guard(n + 1)];
					log.note(format!("flat {n} {}", c.id()));
					v
				})
				.count();
			log.note(format!("flat {n} {}", d.id()));
		}
		19 => {
			let mut flat = guards(1, 4)
				.into_iter()
				.map(move |g| list![g, guard(c.id() - id + 1)])
				.flatten();
			log.note(format!("front {:?}", flat.next().map(|g| g.id())));
			let n: u32 = flat.map(|g| g.id() - id).sum();
			let m = [4u32, 6]
				.into_iter()
				.flat_map(move |n| list![guard(n), guard(d.id() - id + n)])
				.last();
			log.note(format!("flat {n} {:?}", m.map(|g| g.id())));
		}
		20 => {
			// Lazy sources, which hold their closure and what it made.
			let mut held = guards(1, 5);
			let n = std::iter::from_fn(move || {
				let g = held.pop()?;
				Some(g.id() + c.id())
			})
			.count();
			let m = std::iter::successors(Some(guard(5)), move |g| {
				(g.id() < id + 7).then(|| Guard::new(g.id() + 1 + d.id() - d.id(), log.clone()))
			})
			.fold(0, |n, g| n + g.id() - id);
			log.note(format!("lazy {n} {m}"));
		}
		21 => {
			// A consumer over the adaptor that holds the closure.
			let n = guards(1, 5).into_iter().map(move |g| g.id() + c.id()).skip(1).count();
			let m = guards(5, 8).into_iter().take(2).count();
			log.note(format!("over {n} {m}"));
		}
		22 => {
			// Back to front.
			let n = guards(1, 5)
				.into_iter()
				.map(move |g| {
					log.note(format!("map {} {}", g.id(), c.id()));
					g.id() - id
				})
				.rev()
				.fold(0, |n, k| n * 10 + k);
			let m = guards(5, 6)
				.into_iter()
				.chain(guards(6, 8))
				.filter(move |g| g.id() != d.id() - 5)
				.rev()
				.count();
			log.note(format!("rev {n} {m}"));
		}
		23 => {
			let n = guards(1, 6)
				.into_iter()
				.map(move |g| {
					log.note(format!("map {} {}", g.id(), c.id()));
					g
				})
				.take(3)
				.rev()
				.fold(0, |n, g| n * 10 + g.id() - id);
			let m: u32 = guards(6, 8).into_iter().map(move |g| g.id() - id).take(0).rev().sum();
			log.note(format!("take {n} {m} {}", d.id()));
		}
		24 => {
			let n: u32 = guards(1, 7)
				.into_iter()
				.map(move |g| {
					log.note(format!("map {} {}", g.id(), c.id()));
					g
				})
				.step_by(2)
				.rev()
				.map(|g| g.id() - id)
				.sum();
			let m = guards(7, 8)
				.into_iter()
				.filter(move |g| g.id() != d.id())
				.step_by(3)
				.count();
			log.note(format!("step {n} {m}"));
		}
		25 => {
			let mut peek = guards(1, 5)
				.into_iter()
				.map(move |g| {
					log.note(format!("map {} {}", g.id(), c.id()));
					g
				})
				.peekable();
			log.note(format!("peek {:?}", peek.peek().map(|g| g.id())));
			let n = peek.rev().fold(0, |n, g| n * 10 + g.id() - id);
			let mut ended = guards(5, 6).into_iter().peekable();
			log.note(format!("peek {:?}", ended.next().map(|g| g.id())));
			log.note(format!("peek {:?}", ended.peek().map(|g| g.id())));
			log.note(format!("peek {n} {} {}", ended.rev().count(), d.id()));
		}
		26 => {
			let mut flat = guards(1, 4)
				.into_iter()
				.map(move |g| list![g, guard(c.id() - id + 1)])
				.flatten();
			log.note(format!("back {:?}", flat.next_back().map(|g| g.id())));
			let n = flat.rev().fold(0, |n, g| n * 100 + g.id() - id);
			let m = guards(4, 8).into_iter().enumerate().rev().last();
			log.note(format!("flat {n} {:?} {}", m.map(|(i, g)| (i, g.id())), d.id()));
		}
		27 => {
			let kept = guards(1, 5)
				.into_iter()
				.filter(move |g| g.id() != c.id() - 8)
				.rev()
				.fold(
					None,
					|kept: Option<Guard>, g| if g.id() % 2 == 0 { Some(g) } else { kept },
				);
			let n: u32 = guards(5, 8)
				.into_iter()
				.filter_map(move |g| (g.id() != d.id() - 5).then_some(g))
				.rev()
				.map(|g| g.id() - id)
				.sum();
			log.note(format!("rev {:?} {n}", kept.map(|g| g.id())));
		}
		28 => {
			let n = guards(1, 5)
				.into_iter()
				.inspect(move |g| {
					log.note(format!("inspect {} {}", g.id(), c.id()));
				})
				.rev()
				.count();
			let m: u32 = guards(5, 8)
				.into_iter()
				.fuse()
				.rev()
				.map(|g| g.id() - id + d.id() - d.id())
				.sum();
			log.note(format!("rev {n} {m}"));
		}
		29 => {
			// Two closures beside items, back to front.
			let n = guards(1, 4)
				.into_iter()
				.chain(guards(4, 7).into_iter().filter(move |g| g.id() != d.id() - 6))
				.rev()
				.fold(0, |n, g| n * 10 + g.id() - id);
			log.note(format!("rev {n} {}", c.id()));
		}
		31 => options(log, id),
		32 => reducers(log, id),
		33 => closures(log, id),
		34..42 => zips(log, id, arg - 34),
		_ => {
			let n: u32 = guards(1, 6)
				.into_iter()
				.map(move |g| {
					log.note(format!("map {} {}", g.id(), c.id()));
					g
				})
				.skip(2)
				.rev()
				.map(|g| g.id() - id)
				.sum();
			let m = guards(6, 8)
				.into_iter()
				.filter(move |g| g.id() != d.id())
				.skip(1)
				.fold(0, |n, g| n + g.id() - id);
			log.note(format!("skip {n} {m}"));
		}
	}
}

/// Run scenario `arg % 18` of what folds back to front and through what wraps a closure
/// beside another, as `folded` does, of `rfold` and `try_rfold` themselves, and of a
/// crate's iterator with folds of its own. The last panics in a `try_rfold`, with what
/// dropped before.
pub(super) fn rfolded(log: &Log, id: u32, arg: u8) {
	let guard = |n: u32| Guard::new(id + n, log.clone());
	let guards = |from: u32, to: u32| (from..to).map(guard).collect::<Vec<_>>();
	let (c, d, e) = (guard(10), guard(11), guard(12));
	let pile = |from: u32, to: u32| Pile {
		held: guards(from, to),
		log: log.clone(),
	};
	match arg % 18 {
		0 => {
			let n: u32 = guards(1, 5)
				.into_iter()
				.map(move |g| {
					log.note(format!("map {} {}", g.id(), c.id()));
					g
				})
				.rev()
				.map(move |g| g.id() - id + d.id() - d.id())
				.sum();
			log.note(format!("map {n} {}", e.id()));
		}
		1 => {
			let n: u32 = guards(1, 5)
				.into_iter()
				.filter(move |g| g.id() != c.id())
				.rev()
				.map(move |g| g.id() - id + d.id() - d.id())
				.sum();
			let f = guard(13);
			let m = guards(5, 8)
				.into_iter()
				.filter_map(move |g| (g.id() != e.id()).then_some(g))
				.rev()
				.map(move |g| g.id() + f.id())
				.count();
			log.note(format!("filter {n} {m}"));
		}
		2 => {
			let n = guards(1, 4)
				.into_iter()
				.inspect(move |g| {
					log.note(format!("inspect {} {}", g.id(), c.id()));
				})
				.map(move |g| g.id() + d.id())
				.count();
			let f = guard(13);
			let m = guards(4, 8)
				.into_iter()
				.inspect(move |g| {
					log.note(format!("inspect {} {}", g.id(), e.id()));
				})
				.rev()
				.map(move |g| g.id() + f.id())
				.count();
			log.note(format!("inspect {n} {m}"));
		}
		3 => {
			let last = guards(1, 5)
				.into_iter()
				.map(move |g| {
					log.note(format!("map {} {}", g.id(), c.id()));
					g
				})
				.enumerate()
				.rev()
				.last();
			log.note(format!(
				"enumerate {:?} {} {}",
				last.map(|(i, g)| (i, g.id())),
				d.id(),
				e.id()
			));
		}
		4 => {
			let n = guards(1, 5)
				.into_iter()
				.map(move |g| g.id() + c.id())
				.rev()
				.rev()
				.count();
			let m = guards(5, 8)
				.into_iter()
				.map(move |g| g.id() + d.id())
				.skip(1)
				.fold(0, |n, k| n + k);
			log.note(format!("twice {n} {m} {}", e.id()));
		}
		5 => {
			// An accumulator that drops, folded through `step_by`.
			let kept = guards(1, 5)
				.into_iter()
				.map(move |g| {
					log.note(format!("map {} {}", g.id(), c.id()));
					g
				})
				.step_by(2)
				.map(move |g| (g, d.id()))
				.fold(
					Some(guard(20)),
					|kept, (g, _)| if g.id() % 2 == 1 { Some(g) } else { kept },
				);
			log.note(format!("step {:?} {}", kept.map(|g| g.id()), e.id()));
		}
		6 => {
			let kept = guards(1, 6)
				.into_iter()
				.map(move |g| {
					log.note(format!("map {} {}", g.id(), c.id()));
					g
				})
				.step_by(2)
				.rev()
				.map(move |g| (g, d.id()))
				.fold(
					Some(guard(20)),
					|kept, (g, _)| if g.id() % 2 == 1 { Some(g) } else { kept },
				);
			log.note(format!("step {:?} {}", kept.map(|g| g.id()), e.id()));
		}
		7 => {
			let n: u32 = guards(1, 5)
				.into_iter()
				.map(move |g| g.id() - id + c.id() - c.id())
				.fuse()
				.sum();
			let m = guards(5, 8)
				.into_iter()
				.map(move |g| g.id() + d.id())
				.fuse()
				.rev()
				.count();
			log.note(format!("fuse {n} {m} {}", e.id()));
		}
		8 => {
			// What is left between the front and back iterators, folded.
			let mut flat = guards(1, 4)
				.into_iter()
				.map(move |g| {
					log.note(format!("map {} {}", g.id(), c.id()));
					list![guard(g.id() - id + 3), g]
				})
				.flatten();
			log.note(format!("ends {:?}", flat.next().map(|g| g.id())));
			log.note(format!("ends {:?}", flat.next_back().map(|g| g.id())));
			let n = flat.map(move |g| g.id() + d.id()).count();
			log.note(format!("flat {n} {}", e.id()));
		}
		9 => {
			// `rfold` through a closure that owns a guard, of an accumulator that drops.
			let kept = guards(1, 6)
				.into_iter()
				.map(move |g| {
					log.note(format!("map {} {}", g.id(), c.id()));
					g
				})
				.rfold(Some(guard(20)), move |kept, g| {
					log.note(format!("rfold {} {}", g.id(), d.id()));
					if g.id() % 2 == 1 { Some(g) } else { kept }
				});
			// Of an iterator that drops nothing, by a closure that does.
			let n = [1u32, 2, 3].iter().rfold(0, move |n, &k| n * 10 + k + e.id() - e.id());
			log.note(format!("rfold {:?} {n}", kept.map(|g| g.id())));
		}
		10 => {
			// `try_rfold` breaks by `None`, dropping its closure as it returns, before what the
			// iterator still holds, which drops with it.
			let mut held = guards(1, 6).into_iter();
			let sum = held.try_rfold(0, move |n, g| {
				log.note(format!("try {} {}", g.id(), c.id()));
				(g.id() != id + 3).then(|| n + g.id() - id)
			});
			log.note(format!("tried {sum:?} {}", held.len()));
			let rest = held.try_rfold(0, move |n, g| Some(n + g.id() + d.id() - d.id()));
			log.note(format!("rest {rest:?} {}", e.id()));
		}
		11 => {
			// By `Err`, through a `map` whose closure it borrows, which drops with the map.
			let mut mapped = guards(1, 6).into_iter().map(move |g| {
				log.note(format!("map {} {}", g.id(), c.id()));
				g
			});
			let r: Result<u32, Guard> =
				mapped.try_rfold(0, move |n, g| if g.id() == id + 2 { Err(g) } else { Ok(n + g.id() + d.id() - d.id()) });
			log.note(format!("tried {:?}", r.map_err(|g| g.id())));
			drop(mapped);
			log.note(format!("dropped {}", e.id()));
		}
		12 => {
			use std::ops::ControlFlow;
			// By `Break`, through a chain, which drops each half it ends as it goes.
			let mut chained = guards(1, 3).into_iter().chain(guards(3, 6)).rev();
			let r = chained.try_rfold(0, move |n, g| match g.id() == id + 4 {
				true => ControlFlow::Break(g),
				false => ControlFlow::Continue(n + g.id() + c.id() - c.id()),
			});
			let broke = match r {
				ControlFlow::Break(g) => Some(g.id()),
				ControlFlow::Continue(_) => None,
			};
			log.note(format!("broke {broke:?}"));
			let n = chained.try_rfold(0, |n, g| ControlFlow::<(), u32>::Continue(n + g.id() - id));
			log.note(format!("rest {n:?} {} {}", d.id(), e.id()));
		}
		13 => {
			// Of nothing: each closure drops before the iterator, or as `try_rfold` returns.
			let none = guards(1, 1).into_iter().rfold(guard(20), move |acc, _| {
				log.note(format!("never {}", c.id()));
				acc
			});
			let mut empty = guards(1, 1).into_iter().map(move |g| g.id() + d.id());
			let tried = empty.try_rfold(0u32, move |n, k| {
				log.note(format!("never {}", e.id()));
				n.checked_add(k)
			});
			log.note(format!("empty {} {tried:?}", none.id()));
		}
		14 => {
			// A crate's own `fold` and `count`, which std's `map` and `for_each` call, drop
			// what they hold, and the closure std gives the fold, where their bodies do.
			let n = pile(1, 5)
				.map(move |g| {
					log.note(format!("map {} {}", g.id(), c.id()));
					g.id() - id
				})
				.fold(0, |n, k| n * 10 + k);
			log.note(format!("fold {n}"));
			pile(5, 8).for_each(move |g| log.note(format!("each {} {}", g.id(), d.id())));
			let m = pile(1, 4).map(move |g| g.id() + e.id()).count();
			log.note(format!("count {m} {}", pile(4, 6).count()));
		}
		15 => {
			// Its own `last`, `rfold` through `rev`, and `nth` through `skip`.
			let last = pile(1, 4).last();
			let back = pile(4, 7).rev().last();
			let skipped = pile(7, 9).skip(1).last();
			log.note(format!("last {:?} {:?} {:?}", last.map(|g| g.id()), back.map(|g| g.id()), skipped.map(|g| g.id())));
			let kept = pile(1, 6).rev().map(move |g| (g, c.id())).fold(None, |_, (g, _)| Some(g));
			log.note(format!("rev {:?} {} {}", kept.map(|g| g.id()), d.id(), e.id()));
		}
		16 => {
			// Its own `nth_back` through `rev`, `count` through `chain`, and its `fold`, not
			// called through `&mut`.
			let mut held = pile(1, 6);
			log.note(format!("nth {:?}", held.by_ref().rev().nth(1).map(|g| g.id())));
			let n = (&mut held).map(move |g| g.id() + c.id() - c.id()).fold(0, |n, k| n + k);
			log.note(format!("borrowed {n} {}", held.held.len()));
			let m = pile(6, 8).chain(guards(8, 10)).count();
			log.note(format!("chain {m} {} {}", d.id(), e.id()));
		}
		_ => {
			let a = guard(20);
			guards(1, 6).into_iter().try_rfold(0, move |n, g| {
				assert!(g.id() != id + 3, "try_rfold of g{} with g{} after\n{}", g.id(), a.id(), String::from_utf8(log.text()).unwrap());
				Some(n + g.id())
			});
		}
	}
}

/// Guards popped from the back of a `Vec`, with its own `nth`, `nth_back`, folds, `count`,
/// and `last`, each of which notes it was called. Its folds stop short of the last guard
/// and drop their closure before they note they folded, and the rest drop with it.
struct Pile {
	held: Vec<Guard>,
	log: Log,
}

impl Iterator for Pile {
	type Item = Guard;

	fn next(&mut self) -> Option<Guard> {
		self.held.pop()
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		(self.held.len(), Some(self.held.len()))
	}

	fn nth(&mut self, n: usize) -> Option<Guard> {
		self.log.note(format!("pile nth {n}"));
		for _ in 0..n {
			drop(self.held.pop()?);
		}
		self.held.pop()
	}

	fn fold<B, F: FnMut(B, Guard) -> B>(mut self, init: B, mut f: F) -> B {
		self.log.note("pile fold".into());
		let mut acc = init;
		while self.held.len() > 1 {
			acc = f(acc, self.held.pop().unwrap());
		}
		drop(f);
		self.log.note("pile folded".into());
		acc
	}

	fn count(self) -> usize {
		self.log.note(format!("pile count {}", self.held.len()));
		self.held.len()
	}

	fn last(mut self) -> Option<Guard> {
		self.log.note("pile last".into());
		(!self.held.is_empty()).then(|| self.held.remove(0))
	}
}

impl DoubleEndedIterator for Pile {
	fn next_back(&mut self) -> Option<Guard> {
		(!self.held.is_empty()).then(|| self.held.remove(0))
	}

	fn nth_back(&mut self, n: usize) -> Option<Guard> {
		self.log.note(format!("pile nth_back {n}"));
		for _ in 0..n {
			drop(self.next_back()?);
		}
		self.next_back()
	}

	fn rfold<B, F: FnMut(B, Guard) -> B>(mut self, init: B, mut f: F) -> B {
		self.log.note("pile rfold".into());
		let mut acc = init;
		while self.held.len() > 1 {
			acc = f(acc, self.held.remove(0));
		}
		drop(f);
		self.log.note("pile rfolded".into());
		acc
	}
}

/// `last` and `min` of `Option`s, by what drops: a `None` item apart from no items.
fn options(log: &Log, id: u32) {
	let guard = |n: u32| Guard::new(id + n, log.clone());
	let guards = |from: u32, to: u32| (from..to).map(guard).collect::<Vec<_>>();
	let (c, d, e) = (guard(10), guard(11), guard(12));
	let odd = guards(1, 4)
		.into_iter()
		.map(move |g| (g.id() % 2 == c.id() % 2).then_some(g.id()))
		.last();
	let none = guards(4, 4)
		.into_iter()
		.map(move |g| (g.id() != d.id()).then_some(g.id()))
		.last();
	let least = guards(4, 4)
		.into_iter()
		.map(move |g| (g.id() != e.id()).then_some(g.id()))
		.min();
	log.note(format!("options {odd:?} {none:?} {least:?}"));
}

/// `max_by`, `min_by`, and `reduce` of items that drop, by what drops when: each loser as
/// it loses, and the iterator with the closure it holds once it ends, at once if empty.
fn reducers(log: &Log, id: u32) {
	let guard = |n: u32| Guard::new(id + n, log.clone());
	let guards = |from: u32, to: u32| (from..to).map(guard).collect::<Vec<_>>();
	let (c, d) = (guard(10), guard(11));
	let most = guards(1, 6)
		.into_iter()
		.map(move |g| {
			log.note(format!("map {} {}", g.id(), c.id()));
			g
		})
		.max_by(|a, b| (a.id() % 2).cmp(&(b.id() % 2)));
	let least = guards(1, 6).into_iter().min_by(|a, b| (a.id() % 3).cmp(&(b.id() % 3)));
	let kept = guards(1, 4).into_iter().reduce(|a, b| {
		log.note(format!("reduce {} {}", a.id(), b.id()));
		if b.id() % 2 == 0 { b } else { a }
	});
	let none = guards(4, 4)
		.into_iter()
		.map(move |g| g.id() + d.id())
		.reduce(|a, b| a + b);
	let ids = |g: Option<Guard>| g.map(|g| g.id());
	log.note(format!(
		"reducers {:?} {:?} {:?} {none:?}",
		ids(most),
		ids(least),
		ids(kept)
	));
	// A comparator that owns a guard, dropped as `min_by` returns.
	let (held, e) = (guards(1, 3), guard(12));
	let first = held
		.iter()
		.min_by(move |a, b| (a.id() + e.id()).cmp(&(b.id() + e.id())));
	log.note(format!("first {:?}", first.map(Guard::id)));
}

/// Closures that own a guard and return an `Option<Option<Guard>>`, whose `Some(None)` is
/// an item: each drops what it rejects as it returns, and with its adaptor, after the
/// source it reads.
fn closures(log: &Log, id: u32) {
	let guard = |n: u32| Guard::new(id + n, log.clone());
	let guards = |from: u32, to: u32| (from..to).map(guard).collect::<Vec<_>>();
	let (c, d, e, f, h) = (guard(10), guard(11), guard(12), guard(13), guard(14));
	// `None` for 0 mod 3, `Some(None)` for 1, and else `Some(Some(g))`.
	let pick = |g: Guard| match (g.id() - id) % 3 {
		0 => None,
		1 => Some(None),
		_ => Some(Some(g)),
	};
	let ids = |o: Option<Guard>| o.map(|g| g.id());
	let kept: Vec<Option<u32>> = guards(1, 7)
		.into_iter()
		.filter_map(move |g| {
			log.note(format!("filter_map {} {}", g.id(), c.id()));
			pick(g)
		})
		.map(ids)
		.collect();
	log.note(format!("kept {kept:?}"));
	// Through `fold`, which a source that drops takes.
	let folded = guards(1, 7)
		.into_iter()
		.filter_map(pick)
		.fold(0, |n, o| n * 10 + o.map_or(9, |g| g.id() - id));
	log.note(format!("folded {folded}"));
	let mut whiles = guards(4, 8).into_iter().map_while(move |g| {
		log.note(format!("map_while {} {}", g.id(), d.id()));
		pick(g)
	});
	let first = whiles.next().map(ids);
	let second = whiles.next().map(ids);
	drop(whiles);
	log.note(format!("whiles {first:?} {second:?}"));
	let found = guards(3, 7).into_iter().find_map(move |g| {
		log.note(format!("find_map {} {}", g.id(), e.id()));
		pick(g)
	});
	log.note(format!("found {:?}", found.map(ids)));
	let mut held = guards(1, 6);
	let mut made = std::iter::from_fn(move || {
		let g = held.pop()?;
		log.note(format!("from_fn {} {}", g.id(), f.id()));
		Some((g.id() % 2 == 0).then_some(g))
	});
	let first = made.next().map(ids);
	let second = made.next().map(ids);
	drop(made);
	log.note(format!("made {first:?} {second:?}"));
	// From a `Some(None)` seed, holding the guard it yields next as it drops.
	let mut walked = std::iter::successors(Some(None), move |prev: &Option<Guard>| {
		let next = prev.as_ref().map_or(5, |g| g.id() - id + 1);
		log.note(format!("successors {next} {}", h.id()));
		(next < 8).then(|| Some(Guard::new(id + next, log.clone())))
	});
	let first = walked.next().map(ids);
	let second = walked.next().map(ids);
	drop(walked);
	log.note(format!("walked {first:?} {second:?}"));
	let n = std::iter::successors(Some(None), |prev: &Option<Guard>| {
		let next = prev.as_ref().map_or(5, |g| g.id() - id + 1);
		(next < 8).then(|| Some(Guard::new(id + next, log.clone())))
	})
	.count();
	log.note(format!("walked {n}"));
}

/// Scenario `arg` of a zip of items that drop, each side made by a closure where std reads
/// it by index: by index, `next` makes no item past the shorter side and `next_back` makes
/// and drops the longer side's extra first; a zip of two `TrustedLen` iterators, or two only
/// `TrustedRandomAccessNoCoerce`, folds no pair past its size hint, and drops the rest with
/// itself, where `try_fold` and `next` read the first side once more and drop that item.
fn zips(log: &Log, id: u32, arg: u8) {
	let guard = |n: u32| Guard::new(id + n, log.clone());
	let guards = |from: u32, to: u32| (from..to).map(guard).collect::<Vec<_>>();
	// A guard made by a side's closure, which says so, where std reads by index.
	let made = |n: u32| {
		log.note(format!("make {n}"));
		guard(n)
	};
	let ids = |p: Option<(Guard, Guard)>| p.map(|(a, b)| (a.id() - id, b.id() - id));
	match arg {
		0 => {
			let n = (1..4).map(made).zip((4..6).map(made)).count();
			let nth = ids((1..4).map(made).zip((4..7).map(made)).nth(1));
			let past = ids((1..3).map(made).zip((3..6).map(made)).nth(3));
			log.note(format!("index {n} {nth:?} {past:?}"));
		}
		1 => {
			let mut longer = (1..5).map(made).zip((5..7).map(made));
			let back = ids(longer.next_back());
			let front = ids(longer.next());
			let mut shorter = (1..3).map(made).zip((3..7).map(made));
			let next = ids(shorter.next());
			let rest = shorter.rev().map(|(a, b)| a.id() + b.id() - 2 * id).collect::<Vec<_>>();
			log.note(format!("back {back:?} {front:?} {next:?} {rest:?}"));
		}
		2 => {
			let n = guards(1, 4).into_iter().zip(guards(4, 6)).count();
			let any = guards(1, 4).into_iter().zip(guards(4, 6)).any(|_| false);
			let last = ids(guards(1, 3).into_iter().zip(guards(3, 6)).last());
			guards(1, 4)
				.into_iter()
				.zip(guards(4, 6))
				.for_each(|(a, b)| log.note(format!("each {} {}", a.id() - id, b.id() - id)));
			log.note(format!("len {n} {any} {last:?}"));
		}
		3 => {
			let zip = || {
				vec![1, 2, 3]
					.into_iter()
					.map(made)
					.zip(vec![4, 5].into_iter().map(made))
			};
			let n = zip().count();
			let at = zip().position(|_| false);
			let sum: u32 = zip().map(|(a, b)| a.id() + b.id() - 2 * id).sum();
			let skipped = vec![1, 2, 3, 4]
				.into_iter()
				.skip(1)
				.map(made)
				.zip((5..7).map(made))
				.count();
			// Collected in place by index, then dropped whole: `c`'s closure, then `d`'s.
			let (c, d) = (guard(10), guard(11));
			let kept: Vec<u32> = vec![1, 2, 3]
				.into_iter()
				.map(move |x| x + c.id() - id)
				.map(move |x| x + d.id() - id)
				.collect();
			log.note(format!("no coerce {n} {at:?} {sum} {skipped} {kept:?}"));
		}
		4 => {
			let v = guards(1, 4).into_iter().zip((4..6).map(guard)).collect::<Vec<_>>();
			log.note(format!("collect {}", v.len()));
			drop(v);
			let found = ids(guards(1, 3)
				.into_iter()
				.zip((3..6).map(guard))
				.find(|(a, _)| a.id() == id + 9));
			log.note(format!("find {found:?}"));
		}
		5 => {
			let nested = || guards(1, 4).into_iter().zip(guards(4, 6)).zip((6..9).map(guard));
			let n = nested().count();
			let found = nested().find(|((a, _), _)| a.id() == id + 9).is_some();
			log.note(format!("nested {n} {found}"));
		}
		6 => {
			// Neither side `TrustedLen`: `fold` reads by `next`, past the shorter side.
			let n = guards(1, 4)
				.into_iter()
				.filter(|g| g.id() != id)
				.zip(guards(4, 6))
				.count();
			let m = guards(1, 3)
				.into_iter()
				.zip(guards(3, 6).into_iter().filter(|g| g.id() != id))
				.count();
			log.note(format!("neither {n} {m}"));
		}
		_ => {
			let v = [1, 2, 3];
			let n = v.iter().map(|&n| made(n)).zip(0..2).count();
			let back = v.iter().map(|&n| made(n)).zip(0..2).rev().count();
			let empty = v.iter().map(|&n| made(n)).zip(0..0).count() + (0..0).zip(v.iter().map(|&n| made(n))).count();
			log.note(format!("trimmed {n} {back} {empty}"));
		}
	}
}
