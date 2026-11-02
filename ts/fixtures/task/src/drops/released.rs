//! Where std drops a closure that owns a guard given to a consumer that takes the iterator
//! by value, or to a map's entry: as the fold ends, before the iterator and what it still
//! holds, or as it finds the iterator empty, and uncalled where the entry has no use for it.

use std::collections::{BTreeMap, HashMap};

use crate::log::Log;
use crate::model::Guard;

/// The guards of `ids`, in order.
fn guards(log: &Log, id: u32, ids: std::ops::Range<u32>) -> Vec<Guard> {
	ids.map(|n| Guard::new(id + n, log.clone())).collect()
}

/// Each of a map's entries given a closure that owns a guard: `or_insert_with` of an
/// occupied key and of a vacant one, by a closure that moves its guard in and by one that
/// `Fn` calls, which the call drops after; `or_insert_with_key`; and `and_modify` of each.
macro_rules! entries {
	($log:expr, $id:expr, $map:expr, $kind:literal) => {{
		let (log, id) = ($log, $id);
		let guard = |n: u32| Guard::new(id + n, log.clone());
		let mut map = $map;
		map.insert(1u32, guard(1));
		let (a, b) = (guard(2), guard(3));
		log.note(format!(
			"{} occupied {}",
			$kind,
			map.entry(1).or_insert_with(move || a).id()
		));
		log.note(format!(
			"{} vacant {}",
			$kind,
			map.entry(2).or_insert_with(move || b).id()
		));
		let c = guard(4);
		let made = move || Guard::new(c.id() + 10, log.clone());
		log.note(format!("{} made {}", $kind, map.entry(3).or_insert_with(made).id()));
		let d = guard(5);
		let keyed = move |k: &u32| Guard::new(d.id() + k, log.clone());
		log.note(format!(
			"{} keyed {}",
			$kind,
			map.entry(4).or_insert_with_key(keyed).id()
		));
		let (e, f) = (guard(6), guard(7));
		map.entry(1)
			.and_modify(move |g| g.note(&format!("modified {}", e.id())));
		let added = map
			.entry(5)
			.and_modify(move |g| g.note(&format!("{}", f.id())))
			.or_insert(guard(8));
		log.note(format!("{} added {}", $kind, added.id()));
		// Out one at a time, so a hash map's order of its own leaves nothing to differ in.
		for k in 1..6 {
			drop(map.remove(&k));
		}
	}};
}

/// Run scenario `arg % 7`. The last panics in a fold, which aborts before anything else
/// drops.
pub(super) fn released(log: &Log, id: u32, arg: u8) {
	let guard = |n: u32| Guard::new(id + n, log.clone());
	match arg % 7 {
		0 => {
			// The closure drops as the fold ends, before what a `take` holds back.
			let a = guard(10);
			let best = guards(log, id, 1..5).into_iter().take(3).reduce(move |x, y| {
				y.note(&format!("reduce {}", a.id()));
				if x.id() < y.id() { y } else { x }
			});
			log.note(format!("reduced {:?}", best.map(|g| g.id())));
			// Empty: it drops before the iterator.
			let b = guard(11);
			let none = guards(log, id, 5..7).into_iter().take(0).reduce(move |x, _| {
				b.note("never");
				x
			});
			log.note(format!("none {}", none.is_none()));
			// Over an iterator that drops nothing of its own, and lent, which its owner drops.
			let c = guard(12);
			let made = (13..16)
				.map(guard)
				.reduce(move |x, y| if y.id() == c.id() + 2 { y } else { x });
			log.note(format!("made {:?}", made.map(|g| g.id())));
			let d = guard(16);
			let mut first = move |x: Guard, _: Guard| {
				d.note("first");
				x
			};
			let kept = guards(log, id, 17..20).into_iter().reduce(&mut first);
			log.note(format!("lent {:?}", kept.map(|g| g.id())));
			drop(first);
		}
		1 => {
			// Each loser drops as it loses; the comparator as the fold ends, or before an empty
			// iterator.
			let a = guard(10);
			let max = guards(log, id, 1..5)
				.into_iter()
				.take(3)
				.max_by(move |x, y| (x.id() / 2 + a.id()).cmp(&(y.id() / 2 + a.id())));
			log.note(format!("max {:?}", max.map(|g| g.id())));
			let b = guard(11);
			let min = guards(log, id, 5..8)
				.into_iter()
				.min_by(move |x, y| (x.id() / 2 + b.id()).cmp(&(y.id() / 2 + b.id())));
			log.note(format!("min {:?}", min.map(|g| g.id())));
			let c = guard(12);
			let none = guards(log, id, 13..15)
				.into_iter()
				.take(0)
				.max_by(move |x, y| x.id().cmp(&(y.id() + c.id())));
			log.note(format!("none {}", none.is_none()));
		}
		2 => {
			// std maps the iterator by the key's closure, which drops as the fold ends, but after
			// an empty iterator.
			let a = guard(10);
			let max = guards(log, id, 1..5)
				.into_iter()
				.take(3)
				.max_by_key(move |g| g.id() / 2 + a.id());
			log.note(format!("max {:?}", max.map(|g| g.id())));
			let b = guard(11);
			let min = guards(log, id, 5..8)
				.into_iter()
				.min_by_key(move |g| g.id() / 2 + b.id());
			log.note(format!("min {:?}", min.map(|g| g.id())));
			let c = guard(12);
			let none = guards(log, id, 13..15)
				.into_iter()
				.take(0)
				.min_by_key(move |g| g.id() + c.id());
			log.note(format!("none {}", none.is_none()));
			let d = guard(15);
			let mut key = move |g: &Guard| g.id() + d.id();
			let lent = guards(log, id, 16..18).into_iter().max_by_key(&mut key);
			log.note(format!("lent {:?}", lent.map(|g| g.id())));
			drop(key);
		}
		3 => {
			// std maps the iterator by the closure, which drops after it, and after the last
			// item read.
			let a = guard(10);
			let sorted = guards(log, id, 1..4).into_iter().is_sorted_by_key(move |g| {
				g.note(&format!("key {}", a.id()));
				g.id()
			});
			log.note(format!("sorted {sorted}"));
			let b = guard(11);
			let unsorted = guards(log, id, 4..8)
				.into_iter()
				.is_sorted_by_key(move |g| (g.id() + b.id()) % 3);
			log.note(format!("unsorted {unsorted}"));
			let c = guard(12);
			let empty = guards(log, id, 13..15)
				.into_iter()
				.take(0)
				.is_sorted_by_key(move |g| g.id() + c.id());
			log.note(format!("empty {empty}"));
			let d = guard(15);
			let plain = [3u32, 1].iter().is_sorted_by_key(move |&n| n + d.id());
			log.note(format!("plain {plain}"));
		}
		4 => entries!(log, id, HashMap::new(), "hash"),
		5 => entries!(log, id, BTreeMap::new(), "btree"),
		_ => {
			// A key's closure that panics on the second item.
			let a = guard(10);
			guards(log, id, 1..4).into_iter().max_by_key(move |g| {
				assert!(g.id() != id + 2, "key of g{} with g{}", g.id(), a.id());
				g.id()
			});
		}
	}
}
