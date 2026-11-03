//! What a B-tree's `collect` and `from` drop of keys whose own `Ord`, `PartialOrd`, and
//! `PartialEq` disagree with each other: std sorts the entries as its stable sort does, a
//! set's by `lt` and a map's by `cmp`, then keeps the last of each run `!=` finds equal,
//! dropping each other one, key then value, as it finds it.

use std::cell::Cell;
use std::collections::{BTreeMap, BTreeSet};

use crate::log::Log;
use crate::model::Guard;

/// A key that logs its drop and each call to its impls: `cmp` orders by `rank`,
/// `partial_cmp` by half of it, and `eq` by it modulo 3. `cmp` panics of a rank of 99.
struct Ranked {
	id: u32,
	rank: u8,
	log: Log,
}

impl Drop for Ranked {
	fn drop(&mut self) {
		self.log.note(format!("k{} drop", self.id));
	}
}

impl PartialEq for Ranked {
	fn eq(&self, other: &Self) -> bool {
		self.log.note(format!("eq {} {}", self.id, other.id));
		self.rank % 3 == other.rank % 3
	}
}

impl Eq for Ranked {}

impl PartialOrd for Ranked {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		self.log.note(format!("partial_cmp {} {}", self.id, other.id));
		(self.rank / 2).partial_cmp(&(other.rank / 2))
	}
}

impl Ord for Ranked {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.log.note(format!("cmp {} {}", self.id, other.id));
		assert!(self.rank != 99 && other.rank != 99, "cmp of k{} with k{}", self.id, other.id);
		self.rank.cmp(&other.rank)
	}
}

/// A key that copies, so a `Cell` of it orders by it, by its own `Ord` modulo 4.
#[derive(Clone, Copy)]
struct Rank(u8);

impl PartialEq for Rank {
	fn eq(&self, other: &Self) -> bool {
		self.0 % 4 == other.0 % 4
	}
}

impl Eq for Rank {}

impl PartialOrd for Rank {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for Rank {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		(self.0 % 4).cmp(&(other.0 % 4))
	}
}

/// Run scenario `arg % 6`. The last panics in the sort, which aborts before anything drops.
pub(super) fn sorted(log: &Log, id: u32, arg: u8) {
	let key = |n: u32, rank: u8| Ranked { id: id + n, rank, log: log.clone() };
	// Each key of `n` and its value, a guard of `n + 100`.
	let entry = |n: u32, rank: u8| (key(n, rank), Guard::new(id + n + 100, log.clone()));
	// Keys of `ranks`, numbered from 1.
	let keys = |ranks: &[u8]| -> Vec<Ranked> { (1..).zip(ranks).map(|(n, &rank)| key(n, rank)).collect() };
	let entries = |ranks: &[u8]| -> Vec<(Ranked, Guard)> { (1..).zip(ranks).map(|(n, &rank)| entry(n, rank)).collect() };
	let set_ids = |s: &BTreeSet<Ranked>| s.iter().map(|k| (k.id - id).to_string()).collect::<Vec<_>>().join(",");
	let map_ids = |m: &BTreeMap<Ranked, Guard>| {
		m.iter()
			.map(|(k, g)| format!("{}:{}", k.id - id, g.id() - id))
			.collect::<Vec<_>>()
			.join(",")
	};
	let short = [3, 1, 3, 2, 1, 5, 4, 0];
	// Past the 20 std's insertion sort takes, so its driftsort runs.
	let long: Vec<u8> = (0..26).map(|n| n * 7 % 9).collect();
	match arg % 6 {
		0 => {
			let set: BTreeSet<Ranked> = keys(&short).into_iter().collect();
			log.note(format!("set {}", set_ids(&set)));
			drop(set);
			let none: BTreeSet<Ranked> = keys(&[]).into_iter().collect();
			let one: BTreeSet<Ranked> = keys(&[7]).into_iter().collect();
			log.note(format!("small {} {}", none.len(), set_ids(&one)));
		}
		1 => {
			let map: BTreeMap<Ranked, Guard> = entries(&short).into_iter().collect();
			log.note(format!("map {}", map_ids(&map)));
		}
		2 => {
			let set = BTreeSet::from([key(1, 4), key(2, 2), key(3, 4), key(4, 1), key(5, 2)]);
			log.note(format!("set {}", set_ids(&set)));
			let map = BTreeMap::from([entry(1, 4), entry(2, 2), entry(3, 4), entry(4, 1), entry(5, 2)]);
			log.note(format!("map {}", map_ids(&map)));
		}
		3 => {
			// Each emptied by `into_iter`, in std's order: dropped whole, a tree of more than one
			// node drops in the runtime's order, not std's node by node.
			let set: BTreeSet<Ranked> = keys(&long).into_iter().collect();
			log.note(format!("set {}", set_ids(&set)));
			set.into_iter().for_each(drop);
			let map: BTreeMap<Ranked, Guard> = entries(&long).into_iter().collect();
			log.note(format!("map {}", map_ids(&map)));
			map.into_iter().for_each(drop);
		}
		4 => {
			// Keys that drop nothing, by a `Cell` of what has its own `Ord`, of values that do.
			let map: BTreeMap<Cell<Rank>, Guard> = [5u8, 1, 9, 2, 6, 3]
				.iter()
				.zip(1..)
				.map(|(&r, n)| (Cell::new(Rank(r)), Guard::new(id + n, log.clone())))
				.collect();
			let kept = map.iter().map(|(k, g)| format!("{}:{}", k.get().0, g.id() - id));
			log.note(format!("cells {}", kept.collect::<Vec<_>>().join(",")));
		}
		_ => {
			let _set: BTreeSet<Ranked> = keys(&[2, 1, 99, 3]).into_iter().collect();
		}
	}
}
