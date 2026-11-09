//! The crate's own collections, which std's adaptors reach by their `FromIterator` and
//! `IntoIterator` impls: `collect`, and `collect` into a `Result` or an `Option` of one;
//! `flat_map` and `flatten` over them, by value and by reference, and over one whose
//! iterator is the crate's own; and `chain`, `zip`, and `Vec::extend` of one. Each item
//! logs as it is made, read, and dropped, so the log shows what each reads, how far, and
//! when what it left drops.

use std::cell::RefCell;

use crate::harness::put;

type Log = RefCell<Vec<String>>;

/// Logs its byte as it drops.
struct Item<'a> {
	v: u8,
	log: &'a Log,
}

impl Drop for Item<'_> {
	fn drop(&mut self) {
		self.log.borrow_mut().push(format!("drop {}", self.v));
	}
}

/// The crate's own collection, which reads its items only up to the first 0, dropping
/// that one and leaving the rest in the iterator, which drops as `from_iter` returns.
struct Bag<'a> {
	items: Vec<Item<'a>>,
}

impl<'a> FromIterator<Item<'a>> for Bag<'a> {
	fn from_iter<I: IntoIterator<Item = Item<'a>>>(iter: I) -> Self {
		let mut iter = iter.into_iter();
		let mut items = Vec::new();
		for item in iter.by_ref() {
			if item.v == 0 {
				break;
			}
			items.push(item);
		}
		if let Some(item) = items.first() {
			item.log.borrow_mut().push(format!("bag of {}", items.len()));
		}
		Bag { items }
	}
}

impl<'a> IntoIterator for Bag<'a> {
	type Item = Item<'a>;
	type IntoIter = std::vec::IntoIter<Item<'a>>;

	fn into_iter(self) -> Self::IntoIter {
		self.items.into_iter()
	}
}

impl<'a, 'b> IntoIterator for &'b Bag<'a> {
	type Item = &'b Item<'a>;
	type IntoIter = std::slice::Iter<'b, Item<'a>>;

	fn into_iter(self) -> Self::IntoIter {
		self.items.iter()
	}
}

/// The crate's own collection of any type, which keeps its items in reverse and counts
/// them, so a `collect` that skipped it would differ.
struct Stack<T> {
	items: Vec<T>,
	count: usize,
}

impl<T> FromIterator<T> for Stack<T> {
	fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
		let mut items: Vec<T> = iter.into_iter().collect();
		items.reverse();
		let count = items.len();
		Stack { items, count }
	}
}

/// The crate's own collection of the first three reads of its iterator, which reads on past
/// a `None`, as nothing stops it from doing.
struct Tries<T>(Vec<Option<T>>);

impl<T> FromIterator<T> for Tries<T> {
	fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
		let mut iter = iter.into_iter();
		Tries((0..3).map(|_| iter.next()).collect())
	}
}

/// What counts down from `n`, by the crate's own iterator.
struct Count<'a> {
	n: u8,
	log: &'a Log,
}

impl<'a> IntoIterator for Count<'a> {
	type Item = u8;
	type IntoIter = Down<'a>;

	fn into_iter(self) -> Down<'a> {
		self.log.borrow_mut().push(format!("count {}", self.n));
		Down {
			n: self.n,
			log: self.log,
		}
	}
}

/// The crate's own iterator, which logs each read and its drop.
struct Down<'a> {
	n: u8,
	log: &'a Log,
}

impl Iterator for Down<'_> {
	type Item = u8;

	fn next(&mut self) -> Option<u8> {
		self.log.borrow_mut().push(format!("next {}", self.n));
		self.n = self.n.checked_sub(1)?;
		Some(self.n)
	}
}

impl Drop for Down<'_> {
	fn drop(&mut self) {
		self.log.borrow_mut().push(format!("down dropped at {}", self.n));
	}
}

/// What `values` reads, each at most `limit`, and the log.
fn write<'a>(out: &mut Vec<u8>, log: &Log, values: impl IntoIterator<Item = &'a Item<'a>>) {
	let values: Vec<u8> = values.into_iter().map(|item| item.v).collect();
	put(out, &format!("{values:?}"));
	put(out, &log.borrow_mut().drain(..).collect::<Vec<_>>().join(", "));
}

/// What makes an `Item` of each byte, logging it, and how far a case reads.
macro_rules! setup {
	($log:ident, $item:ident, $k:ident = $kind:ident >> $shift:literal) => {
		let $log = Log::default();
		let $item = |v: u8| {
			$log.borrow_mut().push(format!("make {v}"));
			Item { v, log: &$log }
		};
		let $k = usize::from($kind >> $shift);
	};
}

/// `collect` into the crate's collections, which read their source lazily up to its first
/// 0, and into a `Result` or an `Option` of one, which stops at its first `Err` or `None`.
pub fn collects(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	setup!(log, item, k = kind >> 3);
	match kind % 8 {
		0 => {
			let bag: Bag = data.iter().map(|&v| item(v)).collect();
			write(out, &log, &bag);
		}
		1 => {
			let bag = data
				.iter()
				.rev()
				.filter(|&&v| v % 3 != 1)
				.map(|&v| item(v))
				.collect::<Bag>();
			write(out, &log, bag.items.iter().take(k));
		}
		2 => {
			let items: Vec<Item> = data.iter().map(|&v| item(v)).collect();
			let bag: Bag = items.into_iter().skip(k).collect();
			write(out, &log, &bag);
		}
		3 => {
			let bag: Result<Bag, u8> = data
				.iter()
				.map(|&v| match v {
					0xf0.. => Err(v),
					_ => Ok(item(v)),
				})
				.collect();
			match bag {
				Ok(bag) => write(out, &log, &bag),
				Err(e) => put(out, &format!("err {e}")),
			}
		}
		4 => {
			let bag: Option<Bag> = data.iter().map(|&v| (v < 0xf0).then(|| item(v))).collect();
			let len = bag.as_ref().map(|bag| bag.items.len());
			put(out, &format!("{len:?}"));
			write(out, &log, bag.iter().flatten());
		}
		// The crate's own generic collection.
		5 => {
			let stack: Stack<u16> = data.iter().map(|&v| u16::from(v) * 3).collect();
			put(out, &format!("{:?} {}", stack.items, stack.count));
			let stack: Stack<Item> = data.iter().take(k).map(|&v| item(v)).collect();
			write(out, &log, &stack.items);
		}
		// Of the crate's own iterator, by value and borrowed.
		6 => {
			let n = data.first().map_or(0, |v| v % 9);
			let stack: Stack<u8> = Down { n, log: &log }.collect();
			put(out, &format!("{:?} {}", stack.items, stack.count));
			let mut down = Down { n, log: &log };
			let stack: Stack<u8> = down.by_ref().take(k).collect();
			put(out, &format!("{:?} {} {:?}", stack.items, stack.count, down.next()));
		}
		// Into a `Result` or an `Option` of one that reads on past an `Err` or a `None`, which
		// keeps the last `Err`, dropping the one it replaces.
		_ => {
			let tries: Result<Tries<u8>, Item> = data
				.iter()
				.map(|&v| if v % 3 == 0 { Err(item(v)) } else { Ok(v) })
				.collect();
			match tries {
				Ok(Tries(values)) => put(out, &format!("{values:?}")),
				Err(e) => put(out, &format!("err {}", e.v)),
			}
			let tries: Option<Tries<Item>> = data.iter().map(|&v| (v % 3 != 0).then(|| item(v))).collect();
			let values = tries.map(|Tries(values)| {
				values
					.iter()
					.map(|item| item.as_ref().map(|item| item.v))
					.collect::<Vec<_>>()
			});
			put(out, &format!("{values:?}"));
		}
	}
	write(out, &log, []);
}

/// `flat_map` and `flatten` over the crate's collections, by value and by reference, and
/// over one whose iterator is the crate's own, read only so far.
pub fn flattens(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	setup!(log, item, k = kind >> 3);
	match kind % 8 {
		// Read up to `k`, then from the back.
		0 => {
			let mut it = data
				.chunks(3)
				.flat_map(|chunk| chunk.iter().map(|&v| item(v)).collect::<Bag>());
			let front: Vec<Item> = it.by_ref().take(k).collect();
			let back = it.next_back().map(|item| item.v);
			log.borrow_mut().push(format!("back {back:?}"));
			drop(it);
			write(out, &log, &front);
		}
		1 => {
			let bags: Vec<Bag> = data
				.chunks(2)
				.map(|chunk| chunk.iter().map(|&v| item(v)).collect())
				.collect();
			write(out, &log, bags.iter().flatten().skip(k));
			let found = bags.into_iter().flatten().find(|item| usize::from(item.v) == k);
			write(out, &log, found.as_ref());
		}
		2 => {
			let bags: Vec<Bag> = data
				.chunks(2)
				.map(|chunk| chunk.iter().map(|&v| item(v)).collect())
				.collect();
			write(out, &log, (0..bags.len()).rev().flat_map(|i| &bags[i]).step_by(k + 1));
		}
		// The crate's own iterator, which drops where std's adaptor ends.
		3 => {
			let counts: Vec<u8> = data
				.iter()
				.flat_map(|&v| Count { n: v % 5, log: &log })
				.take(k)
				.collect();
			put(out, &format!("{counts:?}"));
		}
		4 => {
			let counts = data.iter().map(|&v| Count { n: v % 4, log: &log });
			let sum = counts.flatten().fold(0u32, |sum, v| sum + u32::from(v));
			put(out, &format!("{sum}"));
		}
		_ => {}
	}
	write(out, &log, []);
}

/// `chain`, `zip`, and `extend` of the crate's collections.
pub fn chains(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	setup!(log, item, k = kind >> 1);
	match kind % 2 {
		0 => {
			let bag: Bag = data.iter().take(k).map(|&v| item(v)).collect();
			let other: Bag = data.iter().skip(k).map(|&v| item(v)).collect();
			write(out, &log, bag.items.iter().chain(&other));
			let mut items: Vec<Item> = bag.into_iter().chain(other).collect();
			let more: Bag = data.iter().rev().map(|&v| item(v)).collect();
			items.extend(more);
			write(out, &log, &items);
		}
		_ => {
			let bag: Bag = data.iter().map(|&v| item(v)).collect();
			let pairs: Vec<(u8, u8)> = data
				.iter()
				.zip(Count { n: 4, log: &log })
				.map(|(&a, b)| (a, b))
				.collect();
			put(out, &format!("{pairs:?}"));
			let zipped: Vec<u8> = (0..k).zip(bag).map(|(_, item)| item.v).collect();
			put(out, &format!("{zipped:?}"));
		}
	}
	write(out, &log, []);
}
