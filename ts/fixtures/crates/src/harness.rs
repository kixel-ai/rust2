//! The entry point both sides of the differential test run.

use std::sync::{Arc, Mutex};

use crates_dep::{Countdown, Guard, Point, Slots, Tally, Weigh};

/// The package's own type, which the dependency's generic function ranks.
struct Item(u8);

impl Weigh for Item {
	fn weight(&self) -> u32 {
		self.0 as u32 % 7
	}
}

impl Tally for Item {
	fn add(&mut self, by: u8) {
		self.0 ^= by;
	}
	fn count(&self) -> u32 {
		u32::from(self.0)
	}
}

pub fn run(input: &[u8]) -> Vec<u8> {
	let mut out = Vec::new();
	// Only the dependency writes into a `Point`, so the copy must not share it.
	if let [a, b, ..] = input {
		let p = Point {
			x: *a as i32,
			y: *b as i32,
		};
		let mut q = p;
		q.shift(1, -1);
		out.extend([p.x as u8, p.y as u8, q.x as u8, q.y as u8]);
	}
	let items: Vec<Item> = input.iter().map(|&byte| Item(byte)).collect();
	if let Some(Item(byte)) = crates_dep::heaviest(&items) {
		out.push(*byte);
	}
	// The dependency's drops, where native Rust runs them.
	let log = Arc::new(Mutex::new(Vec::new()));
	{
		let guards: Vec<Guard> = input.iter().take(3).map(|&id| Guard::new(id, &log)).collect();
		let held = Guard::new(0xff, &log);
		drop(guards);
		let _late = Guard::new(0xfe, &log);
		drop(held);
	}
	out.extend(log.lock().unwrap().iter());
	// Trait objects of the dependency's trait, of either crate's types, which drop what
	// they hold where native Rust does.
	if let [a, b, c, ..] = input {
		let mut item = Item(*a);
		let mut n = u32::from(*b);
		let mut guard = Guard::new(*c, &log);
		let sum = crates_dep::tally(&mut [&mut item, &mut n, &mut guard], *a);
		out.extend(sum.to_le_bytes());
		out.extend([item.0, n as u8]);
		let mut boxed: Vec<Box<dyn Tally>> = vec![Box::new(guard), Box::new(item), Box::new(n)];
		boxed.swap(0, 2);
		boxed.truncate(2);
		for t in &mut boxed {
			t.add(*c);
			out.push(t.count() as u8);
		}
		drop(boxed);
		out.extend(log.lock().unwrap().iter());
	}
	for n in Countdown::new(input.len().min(4) as u8) {
		out.push(n);
	}
	// The dependency's iterator, from the back and then from either end until they meet.
	out.extend(Countdown::new(input.len().min(5) as u8).rev());
	let mut both = Countdown::new(input.len().min(6) as u8);
	while let Some(n) = both.next_back() {
		out.push(n);
		out.extend(both.next());
	}
	// Only the binding writes into a `[i16; 2]`, so the copy must not share it.
	let kept: [i16; 2] = [input.len() as i16, 7];
	let mut filled = kept;
	Slots::with_base(kept[0]).fill(&mut filled);
	out.extend([kept[0] as u8, kept[1] as u8, filled[0] as u8, filled[1] as u8]);
	out.push(crates_dep::first(input));
	out.push(crates_dep::last(input));
	out
}
