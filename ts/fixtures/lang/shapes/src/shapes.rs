//! Control flow, patterns, aggregates, closures, and traits.

use std::convert::Infallible;
use std::ops::ControlFlow;
use std::sync::{Arc, Mutex};
use std::task::Poll;

use crate::fmt::Pair;

/// A `Copy` struct, which Rust copies where JS would share.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Point {
	pub x: i32,
	pub y: i32,
}

impl Point {
	pub fn new(x: i32, y: i32) -> Self {
		Self { x, y }
	}

	/// Changes `self`, which a caller's copy must not see.
	pub fn shift(mut self, by: i32) -> Self {
		self.x += by;
		self
	}

	pub fn bump(&mut self) {
		self.y += 1;
	}
}

/// A tuple struct.
#[derive(Clone, Copy)]
pub struct Meters(pub u64);

/// A tuple struct that is not `Copy`, so struct update syntax reads its base in place.
#[derive(Debug)]
pub struct Moved(pub u8, pub u16);

/// An enum with data, and one without.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shape {
	Dot,
	Line(Point, Point),
	Rect { corner: Point, width: u16, height: u16 },
}

/// Discriminants a cast truncates, or reads with another sign.
#[derive(Clone, Copy)]
#[repr(u16)]
pub enum Wide {
	Low = 1,
	High = 300,
	Top = u16::MAX,
}

#[derive(Clone, Copy)]
#[repr(i8)]
pub enum Signed {
	Neg = -1,
	Min = i8::MIN,
	Pos = 100,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Dir {
	North = 1,
	East = 2,
	South = 4,
	West = 8,
}

pub trait Area {
	const SIDES: u8;

	fn area(&self) -> u64;

	/// A default method, monomorphized for each type.
	fn describe(&self, out: &mut Vec<u8>) {
		out.push(Self::SIDES);
		out.extend_from_slice(&self.area().to_be_bytes());
	}
}

impl Area for Shape {
	const SIDES: u8 = 4;

	fn area(&self) -> u64 {
		match *self {
			Shape::Dot => 0,
			Shape::Line(..) => 0,
			Shape::Rect { width, height, .. } => width as u64 * height as u64,
		}
	}
}

impl Area for Meters {
	const SIDES: u8 = 0;

	fn area(&self) -> u64 {
		self.0
	}

	fn describe(&self, out: &mut Vec<u8>) {
		out.push(0xaa);
		out.extend_from_slice(&self.0.to_be_bytes());
	}
}

fn total<T: Area>(items: &[T], out: &mut Vec<u8>) -> u64 {
	let mut sum = 0u64;
	for item in items {
		item.describe(out);
		sum = sum.saturating_add(item.area());
	}
	sum
}

impl From<Dir> for u8 {
	fn from(dir: Dir) -> u8 {
		dir as u8
	}
}

fn dir(byte: u8) -> Option<Dir> {
	match byte % 5 {
		0 => Some(Dir::North),
		1 => Some(Dir::East),
		2 => Some(Dir::South),
		3 => Some(Dir::West),
		_ => None,
	}
}

fn shape(data: &[u8]) -> Shape {
	let Some((&kind, rest)) = data.split_first() else {
		return Shape::Dot;
	};
	let at = |i: usize| rest.get(i).copied().unwrap_or(0) as i32 - 128;
	match kind % 3 {
		0 => Shape::Dot,
		1 => Shape::Line(Point::new(at(0), at(1)), Point::new(at(2), at(3))),
		_ => Shape::Rect {
			corner: Point::new(at(0), at(1)),
			width: rest.get(2).copied().unwrap_or(0) as u16 * 300,
			height: rest.get(3).copied().unwrap_or(0) as u16,
		},
	}
}

/// `Copy` aggregates: a copy changes apart from its original.
pub fn copies(data: &[u8], out: &mut Vec<u8>) {
	let p = Point::new(data.first().copied().unwrap_or(0) as i32, 7);
	let mut q = p;
	q.x += 100;
	q.bump();
	let r = p.shift(5);
	let mut pair = (p, 3u8);
	let saved = pair;
	pair.0.y = -1;
	pair.1 += 1;
	let mut grid = [p; 3];
	grid[1].x = 9;
	let row = grid;
	grid[2] = q;
	for v in [
		p.x, p.y, q.x, q.y, r.x, saved.0.y, pair.0.y, row[1].x, row[2].x, grid[2].x,
	] {
		out.extend_from_slice(&v.to_be_bytes());
	}
	out.push(saved.1);
	out.push(pair.1);
	out.push((p == saved.0) as u8);

	let mut a = Point::new(1, 2);
	let mut b = Point::new(3, 4);
	std::mem::swap(&mut a, &mut b);
	let old = std::mem::replace(&mut a, Point::new(5, 6));
	out.extend_from_slice(&[a.x as u8, b.x as u8, old.x as u8]);

	// A box is its contents: a write through it, or through a borrow of it, changes them.
	let mut boxed = Box::new(Moved(data.len() as u8, 0));
	boxed.1 += 1;
	let held = &mut boxed;
	held.1 += 2;
	out.extend_from_slice(&[boxed.0]);
	out.extend_from_slice(&boxed.1.to_be_bytes());

	// A tuple keeps the values it was made of, though the variable they came from changes.
	let mut n = data.len() as u32;
	let both = (n, n);
	n = 9;
	out.extend_from_slice(&(both.0 + both.1 + n).to_be_bytes());

	// So does a tuple of references, though the variables they came from are assigned
	// others: a swap in a loop.
	let (mut s, mut t) = ("ab", "cde");
	let (mut x, mut y) = (&p, &q);
	let (mut i, mut j) = (data.len() as u8, 1u8);
	for _ in 0..data.len() % 3 {
		(s, t) = (t, s);
		(x, y) = (y, x);
		(i, j) = (j, i);
		out.extend_from_slice(s.as_bytes());
		out.push(x.x as u8 ^ i);
	}
	out.push(t.len() as u8 ^ y.y as u8 ^ j);

	// A value computed from a call keeps that call before a later one, though it is read
	// after.
	// A conversion reads its bytes before they change.
	let mut word = [0, 0, 0, data.len() as u8];
	let converted = u32::from_be_bytes(word);
	word[0] = 9;
	out.extend_from_slice(&(converted + word[0] as u32).to_be_bytes());

	// An argument copied keeps its value, though the callee changes its source through
	// another.
	let mut cell = [data.len() as u8];
	out.push(write_then_read(cell, &mut cell));

	// A result returning an argument is its own copy, though the source changes after.
	let mut source = [data.len() as u8];
	let returned = same(source);
	source[0] = 9;
	out.extend_from_slice(&[returned[0], source[0]]);
	out.push(copy_then_write((data.len() as u8, 0)));

	let marked = mark(out, 1) == 0;
	mark(out, 2);
	out.push(marked as u8);

	let mut opt = Some(Meters(9));
	let taken = opt.take();
	out.push(opt.is_none() as u8);
	out.extend_from_slice(&taken.map_or(0, |m| m.0).to_be_bytes());

	// A `Copy` payload leaves a `Result` as a copy.
	let res: Result<Point, u8> = Ok(p);
	let mut first = res.unwrap();
	first.x = 2;
	let fail: Result<u8, Point> = Err(q);
	let mut err = fail.unwrap_err();
	err.y = 3;
	out.extend_from_slice(&[
		first.x as u8,
		res.unwrap().x as u8,
		err.y as u8,
		fail.unwrap_err().y as u8,
	]);

	// An enum's and a `Poll`'s payload changes in place apart from a copy's.
	let mut shape = shape(data);
	let before = shape;
	match &mut shape {
		Shape::Dot => {}
		Shape::Line(from, _) => from.x = 77,
		Shape::Rect { corner, width, .. } => {
			corner.y = 5;
			*width = 6;
		}
	}
	for shape in [before, shape] {
		out.extend_from_slice(&match shape {
			Shape::Dot => [0; 3],
			Shape::Line(from, _) => [1, from.x as u8, from.y as u8],
			Shape::Rect { corner, width, .. } => [corner.x as u8, corner.y as u8, width as u8],
		});
	}
	let mut poll = match data.len() % 2 {
		0 => Poll::Pending,
		_ => Poll::Ready([data.len() as u8]),
	};
	let polled = poll;
	if let Poll::Ready(ref mut bytes) = poll {
		bytes[0] = 9;
	}
	for poll in [polled, poll] {
		out.push(match poll {
			Poll::Ready(bytes) => bytes[0],
			Poll::Pending => 0xff,
		});
	}
	shares(data, out);
	unwritten(data, out);
}

/// A `Copy` struct this module never writes into: only `generics::advance` does.
#[derive(Clone, Copy)]
pub struct Mark {
	pub at: u8,
}

/// What holds a `Mark`.
pub struct Track {
	pub mark: Mark,
}

impl Track {
	fn mark(&self) -> Mark {
		self.mark
	}
}

/// A `Copy` struct written into only behind a type parameter, by `generics::reset`.
#[derive(Clone, Copy, Default)]
struct Gauge {
	level: u8,
}

/// A `Copy` struct written into only through a box.
#[derive(Clone, Copy)]
struct Level(u8);

/// `Copy` aggregates written into only in another module, behind a type parameter, or
/// through a box. Each copy keeps its value.
fn unwritten(data: &[u8], out: &mut Vec<u8>) {
	let n = data.len() as u8;
	let mut track = Track { mark: Mark { at: n } };
	let seen = track.mark();
	crate::generics::advance(&mut track);
	out.extend_from_slice(&[seen.at, track.mark.at]);

	let first = Gauge { level: n | 1 };
	let mut gauges = [first; 2];
	crate::generics::reset(&mut gauges);
	out.extend_from_slice(&[first.level, gauges[0].level, gauges[1].level]);

	let start = Level(n);
	let mut boxed = Box::new(start);
	*boxed = Level(!n);
	for level in [start, *boxed] {
		out.push(level.0);
	}
}

/// A `Copy` struct holding a shared reference to a value that changes through it.
#[derive(Clone, Copy)]
struct Lens<'a> {
	slot: &'a Mutex<u8>,
	n: u8,
}

/// Adds one to what `slot` holds.
fn bump(slot: &Mutex<u8>) {
	let mut held = slot.lock().unwrap();
	*held = held.wrapping_add(1);
}

/// `Copy` aggregates holding shared references: a copy has its own values, and shares
/// what its references point at with the original's.
fn shares(data: &[u8], out: &mut Vec<u8>) {
	let a = data.first().copied().unwrap_or(0);
	let slot = Mutex::new(a);
	let other = Mutex::new(0);

	let pair = (&slot, a);
	let mut copy = pair;
	copy.1 = copy.1.wrapping_add(1);
	bump(copy.0);
	bump(pair.0);
	out.extend_from_slice(&[pair.1, copy.1]);

	let refs = [&slot, &other];
	let mut copy = refs;
	bump(copy[0]);
	copy[1] = &slot;
	bump(copy[1]);
	bump(refs[1]);

	let held = Some((&slot, a));
	let mut copy = held;
	if let Some((held, ref mut n)) = copy {
		bump(held);
		*n = n.wrapping_add(2);
	}
	for held in [held, copy] {
		out.push(held.map_or(0, |(_, n)| n));
	}

	let res: Result<(&Mutex<u8>, u8), u8> = if a % 2 == 0 { Ok((&slot, a)) } else { Err(a) };
	let mut copy = res;
	if let Ok((held, n)) = &mut copy {
		bump(held);
		*n = n.wrapping_add(3);
	}
	for r in [res, copy] {
		out.push(match r {
			Ok((_, n)) | Err(n) => n,
		});
	}

	let lens = Lens { slot: &slot, n: a };
	let mut copy = lens;
	copy.n = copy.n.wrapping_add(4);
	bump(copy.slot);
	out.extend_from_slice(&[lens.n, copy.n]);
	out.push(lens_bumped(lens));

	// Copied out of a slice and a collection, each still points at the original.
	for lens in [lens; 2].to_vec() {
		bump(lens.slot);
	}
	let mut lenses = Vec::new();
	lenses.push(lens);
	lenses.extend([lens].iter());
	for lens in lenses.iter().copied() {
		bump(lens.slot);
	}

	let arc = Arc::new(a);
	let flow: ControlFlow<&Arc<u8>, [u8; 1]> = match a % 3 {
		0 => ControlFlow::Continue([a]),
		_ => ControlFlow::Break(&arc),
	};
	out.push(same_arc(flow, &arc));

	out.push(*slot.lock().unwrap());
	out.push(*other.lock().unwrap());

	// An argument copied into what a call returns, or read after the call changes its
	// source through another argument, keeps its value.
	let mut source = Held {
		at: Point::new(a as i32, 0),
	};
	let kept = hold(source.at);
	source.at.x = 9;
	let read = moved_then_read(source.at, &mut source);
	let by = moved_by(source.at, Mover { held: &mut source });
	let part = part_moved_then_read(source, &mut source.at);
	out.extend_from_slice(&[kept.at.x as u8, read as u8, by as u8, part as u8, source.at.x as u8]);
	let mut bytes = [a, 1];
	out.push(unsized_copy(&mut bytes));
	out.push(bytes[0]);
	let at = data.get(1).map_or(0, |&b| b as usize % 3);
	out.push(write_at(&mut bytes, at, a));
	out.extend_from_slice(&bytes);
	let mut points = vec![Point::new(a as i32, 0); 2];
	moved_at(&mut points, at);
	out.extend_from_slice(&[points[0].x as u8, points[1].x as u8, points[1].y as u8]);
}

/// `live[at]`, written with `b`, then changed through its first byte, and through a
/// reborrow that may move past it: an index past the end panics as native Rust does.
#[inline(never)]
fn write_at(live: &mut [u8], at: usize, b: u8) -> u8 {
	live[at] = b;
	live[0] ^= live[at].wrapping_add(1);
	// Reassigned, so its bounds checks read its length apart from `rt.idx`.
	let mut rest = &mut *live;
	if b % 2 == 0 {
		rest = &mut rest[1..];
	}
	rest[0] = rest[0].wrapping_add(3);
	live[at]
}

/// `live[at]` moved right by one, and its neighbor by its new `x`: an index past the end
/// panics as native Rust does.
#[inline(never)]
fn moved_at(live: &mut [Point], at: usize) {
	live[at].x += 1;
	live[1].y = live[at].x;
}

/// A `Copy` struct holding another by value.
#[derive(Clone, Copy)]
struct Held {
	at: Point,
}

/// `at`, held.
#[inline(never)]
fn hold(at: Point) -> Held {
	Held { at }
}

/// `copy.x`, read after `held` moves right by one, which a copy of its point doesn't see.
#[inline(never)]
fn moved_then_read(copy: Point, held: &mut Held) -> i32 {
	held.at.x += 1;
	copy.x
}

/// `copy.at.x`, read after `at` moves right by one, which a copy of what holds it doesn't
/// see.
#[inline(never)]
fn part_moved_then_read(copy: Held, at: &mut Point) -> i32 {
	at.x += 1;
	copy.at.x
}

/// `copy[0]`, read after `live`, its source unsized, changes.
#[inline(never)]
fn slice_changed_then_read(copy: [u8; 2], live: &mut [u8]) -> u8 {
	live.copy_from_slice(&[9, 9]);
	copy[0]
}

/// `src[0]` as it was, read after a call changes it through a slice.
fn unsized_copy(src: &mut [u8; 2]) -> u8 {
	slice_changed_then_read(*src, src)
}

/// A mutable borrow of a `Held`, held by value.
struct Mover<'a> {
	held: &'a mut Held,
}

/// `copy.x`, read after `by` moves what it holds right by one.
#[inline(never)]
fn moved_by(copy: Point, by: Mover) -> i32 {
	by.held.at.y += 1;
	by.held.at.x += 1;
	copy.x
}

/// A `Lens` changed in place, which changes its slot too.
fn lens_bumped(mut lens: Lens) -> u8 {
	lens.n = lens.n.wrapping_add(5);
	bump(lens.slot);
	lens.n
}

/// Whether `flow` breaks with `arc` itself, after changing its own bytes in place.
fn same_arc(mut flow: ControlFlow<&Arc<u8>, [u8; 1]>, arc: &Arc<u8>) -> u8 {
	match flow {
		ControlFlow::Continue(ref mut bytes) => {
			bytes[0] = bytes[0].wrapping_add(1);
			bytes[0]
		}
		ControlFlow::Break(p) => Arc::ptr_eq(p, arc) as u8,
	}
}

/// `a`'s byte, after writing `b`'s.
#[inline(never)]
fn write_then_read(a: [u8; 1], b: &mut [u8; 1]) -> u8 {
	b[0] = 9;
	a[0]
}

/// A copy of `a` read after `a` changes in place, which it doesn't see.
fn copy_then_write(mut a: (u8, u8)) -> u8 {
	let b = a;
	a.0 = 9;
	b.0.wrapping_add(a.0)
}

/// `a` itself.
#[inline(never)]
fn same(a: [u8; 1]) -> [u8; 1] {
	a
}

/// Logs `b`, returning how much is logged.
#[inline(never)]
fn mark(out: &mut Vec<u8>, b: u8) -> usize {
	out.push(b);
	out.len()
}

/// Aggregates written with their fields out of declaration order evaluate them as
/// written: each field logs as it runs.
pub fn order(data: &[u8], out: &mut Vec<u8>) {
	let at = |i: usize| data.get(i).copied().unwrap_or(0);
	let rect = Shape::Rect {
		height: mark(out, at(0)) as u16,
		width: mark(out, at(1)) as u16,
		corner: Point {
			y: mark(out, 2) as i32,
			x: mark(out, 3) as i32,
		},
	};
	let line = Shape::Line {
		1: Point::new(mark(out, 4) as i32, 0),
		0: Point::new(mark(out, 5) as i32, 0),
	};
	let pair = Pair {
		1: mark(out, 6) as i64,
		0: mark(out, 7) as u8,
	};
	// A local read before a later field changes it.
	let mut n = [at(2)];
	let read = Point {
		y: n[0] as i32,
		x: increment(&mut n) as i32,
	};
	// The base is read after the fields, which change it.
	let mut base = Moved(at(4), 0);
	let updated = Moved {
		1: reset(&mut base),
		..base
	};
	// Only the branch taken runs its fields.
	let taken = match at(3) % 2 {
		0 => Shape::Rect {
			width: mark(out, 8) as u16,
			height: 1,
			corner: Point::new(0, mark(out, 9) as i32),
		},
		_ => Shape::Dot,
	};
	let range = std::ops::Range {
		end: mark(out, 10),
		start: mark(out, 11) / 2,
	};
	let mut sum = 0;
	for i in (std::ops::Range {
		end: mark(out, 12),
		start: mark(out, 13) - 4,
	}) {
		sum += i;
	}
	let len = data.len();
	// The sliced value comes before the range's fields.
	let slice = &logged(data, out)[std::ops::Range {
		end: mark(out, 14).min(len),
		start: mark(out, 15).min(len) / 2,
	}];
	let text = format!(
		"{rect:?} {line:?} {pair:?} {read:?} {updated:?} {taken:?} {} {} {sum} {slice:?}",
		range.start, range.end
	);
	out.extend_from_slice(text.as_bytes());
	// A tuple of borrows read by its fields, as `format_args!` reads its arguments: each
	// borrowed value is evaluated once, where the tuple is made.
	let (marked, len) = (&mark(out, 16), &out.len());
	out.push(*marked as u8);
	out.push(*len as u8);
	// A tuple read by one field: the others still run, where the tuple is made.
	let (only, _) = (&mark(out, 17), &out.len());
	let (first, _) = (&mark(out, 18), &mark(out, 19));
	out.push(*only as u8);
	out.push(*first as u8);
	indexed(data, out);
}

/// Indexing, calls, and assignments whose later operands need statements of their own
/// evaluate what comes before those first: each logs as it runs, or changes what an
/// earlier operand read.
fn indexed(data: &[u8], out: &mut Vec<u8>) {
	let at = |i: usize| data.get(i).copied().unwrap_or(0);
	// The vector, then the index.
	let first = listed(out, at(0))[{
		out.push(0xe1);
		1
	}];
	// The slice, the range, then `dest`.
	let mut bytes = [at(1), at(2), at(3), 4];
	logged_mut(&mut bytes, out).copy_within(
		{
			out.push(0xe3);
			1
		}..3,
		{
			out.push(0xe4);
			0
		},
	);
	logged_mut(&mut bytes, out).copy_within(2.., {
		out.push(0xe5);
		1
	});
	// Without an end, the slice is read once for its length too.
	logged_mut(&mut bytes, out).copy_within(3.., 0);
	// A slice, then its index.
	let second = logged(&bytes, out)[{
		out.push(0xe6);
		1
	}];
	// A function pointer is read before its argument, which changes it.
	let mut f: fn(u8) -> u8 = halve;
	let called = f({
		f = twice;
		at(4)
	});
	// A compound assignment's value is read before its place's index, which changes it.
	let mut n = at(5);
	let mut slots = [0u8; 2];
	slots[{
		n = n.wrapping_add(1);
		0
	}] += n;
	// A value read before a call that changes it, used after the call's result.
	let mut m = [at(6)];
	let before = m[0];
	let after = increment(&mut m).wrapping_sub(before);
	out.extend_from_slice(&[first, second, called, f(1), slots[0], n, after]);
	out.extend_from_slice(&bytes);
	// An array of byte slices unsized to a slice of them.
	let (head, tail) = data.split_at(data.len() / 2);
	ends(&[head, tail], out);
	// Of two operands that can panic, the one that runs first panics first.
	if at(7) % 4 == 0 {
		let (x, y, z) = (at(8), at(9), at(10));
		let quotient = x / y;
		let sum = z + 200;
		out.push(sum.wrapping_add(quotient));
	}
}

/// Each part's length and first byte.
#[inline(never)]
fn ends(parts: &[&[u8]], out: &mut Vec<u8>) {
	for part in parts {
		out.extend_from_slice(&[part.len() as u8, part.first().copied().unwrap_or(0)]);
	}
}

/// A vector of `b`, logging that it was made.
#[inline(never)]
fn listed(out: &mut Vec<u8>, b: u8) -> Vec<u8> {
	out.push(0xe0);
	[b, b ^ 1].to_vec()
}

/// `s`, logging that it was read.
#[inline(never)]
fn logged_mut<'a>(s: &'a mut [u8], out: &mut Vec<u8>) -> &'a mut [u8] {
	out.push(0xe2);
	s
}

#[inline(never)]
fn halve(b: u8) -> u8 {
	b / 2
}

#[inline(never)]
fn twice(b: u8) -> u8 {
	b.wrapping_add(b)
}

/// `data`, logging that it was read.
#[inline(never)]
fn logged<'a>(data: &'a [u8], out: &mut Vec<u8>) -> &'a [u8] {
	out.push(0xee);
	data
}

/// `n`'s byte, incremented.
#[inline(never)]
fn increment(n: &mut [u8; 1]) -> u8 {
	n[0] = n[0].wrapping_add(1);
	n[0]
}

/// Changes `m`'s first field.
#[inline(never)]
fn reset(m: &mut Moved) -> u16 {
	m.0 = 9;
	1
}

/// Fieldless enums cast to every width, past their discriminant's range and sign.
pub fn discriminants(data: &[u8], out: &mut Vec<u8>) {
	for &b in data {
		let wide = match b % 3 {
			0 => Wide::Low,
			1 => Wide::High,
			_ => Wide::Top,
		};
		let signed = match b % 3 {
			0 => Signed::Neg,
			1 => Signed::Min,
			_ => Signed::Pos,
		};
		out.push(wide as u8);
		out.push(wide as i8 as u8);
		out.extend_from_slice(&(wide as i16).to_be_bytes());
		out.extend_from_slice(&(wide as u64).to_be_bytes());
		out.push(signed as u8);
		out.extend_from_slice(&(signed as u16).to_be_bytes());
		out.extend_from_slice(&(signed as u32).to_be_bytes());
		out.extend_from_slice(&(signed as i64).to_be_bytes());
		out.extend_from_slice(&(signed as u128).to_be_bytes());
	}
}

/// Loops: labeled, `while let`, ranges, values out of `break`, loops that end a function,
/// and values of type `!`.
pub fn loops(data: &[u8], out: &mut Vec<u8>) {
	let mut stack: Vec<u8> = data.to_vec();
	let mut popped = 0u32;
	while let Some(top) = stack.pop() {
		if top == 0 {
			continue;
		}
		popped += top as u32;
		if popped > 1000 {
			break;
		}
	}
	out.extend_from_slice(&popped.to_be_bytes());

	let mut found = None;
	'outer: for i in 0..data.len() {
		for j in (i + 1)..data.len() {
			if data[i] == data[j] {
				found = Some((i, j));
				break 'outer;
			}
			if data[j] == 0 {
				continue 'outer;
			}
		}
	}
	match found {
		Some((i, j)) => out.extend_from_slice(&[i as u8, j as u8]),
		None => out.push(0xff),
	}

	let mut n = data.len() as u64;
	let steps = loop {
		if n <= 1 {
			break 0u8;
		}
		n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
		if n > 100 {
			break 1;
		}
	};
	out.push(steps);

	let mut count = 0u8;
	for _ in 0..=3u8 {
		count += 1;
	}
	let mut i = 10i8;
	while i > -3 {
		i -= 4;
		count += 1;
	}
	out.push(count);

	// Ranges stepped by hand: signed across zero, up to the type's end, and an item read
	// after the range changed.
	let lo = -((data.len() % 4) as i32);
	let mut r = lo..lo + 3;
	let first = r.next();
	r.end = r.start;
	let second = r.next();
	if let Some(x) = first {
		out.extend_from_slice(&x.to_be_bytes());
	}
	if let Some(x) = second {
		out.extend_from_slice(&x.to_be_bytes());
	}
	for k in lo..2 {
		out.push(k as u8);
	}
	for k in 124 - lo as i8..i8::MAX {
		out.push(k as u8);
	}

	out.push(first_repeat(data));
	out.extend_from_slice(&tail_break(data).to_be_bytes());
	out.push(arm_loop(data));
	out.extend_from_slice(&(position(data, &0) as u32).to_be_bytes());
	let settle = |mut x: u8| -> u8 {
		loop {
			if x < 10 {
				return x;
			}
			x /= 3;
		}
	};
	out.push(settle(data.len() as u8 * 7));
	drain(data, out);
	out.push(returned_loop(data));
	branch_loops(data, out);
	diverging(data, out);
}

/// Values of type `!` where a value goes, triggered by the first two bytes so most
/// inputs run every case to its end.
fn diverging(data: &[u8], out: &mut Vec<u8>) {
	let head = &data[..data.len().min(2)];
	let b = head.first().copied().unwrap_or(0);
	out.push(returned_never(b, data.len()));
	out.push(broken_never(head));
	let check = |b: u8| -> u8 {
		if b == 0xfd {
			#[allow(unreachable_code, clippy::needless_return, reason = "a `return` of a `!` value")]
			return panic!("closure {b}");
		}
		b / 3
	};
	out.push(check(b));
	out.push(guarded_return(head));
	let found = bound_never(head, out);
	out.push(found);
	out.push(closure_never(b));
	generic_never(b, data.len());
}

/// Calls that never return, through a closure, a function pointer, and a closure a
/// function made: a statement, a bound value, and the tail.
fn closure_never(b: u8) -> u8 {
	let fail = |b: u8| -> ! { panic!("closure failed on {b:#x}") };
	let ptr: fn(u8) -> ! = bail;
	let made = made_never();
	if b == 0xf5 {
		fail(b);
	}
	let x: u8 = if b == 0xf4 { ptr(b) } else { b / 5 };
	match b {
		0xf3 => fail(b),
		0xfa => made(b),
		_ => x,
	}
}

/// A closure that never returns, made by a function.
fn made_never() -> impl Fn(u8) -> ! {
	|b| panic!("made {b}")
}

/// Generic functions instantiated at `!`, which edition 2024 infers for a value that
/// never exists: given one, and given an `Option` of none, which then calls one.
#[allow(unreachable_code, clippy::diverging_sub_expression, reason = "`!` arguments")]
fn generic_never(b: u8, len: usize) {
	fn pass<T>(x: T) -> T {
		x
	}
	fn unwrap_or_bail<T>(o: Option<T>, b: u8) -> T {
		match o {
			Some(x) => pass(x),
			None => bail(b),
		}
	}
	if b == 0xf1 {
		pass(bail(b));
	}
	if b == 0xf6 {
		pass(panic!("passed {b}"));
	}
	if b == 0xf9 {
		let o = if len > 1 { None } else { Some(panic!("some {b}")) };
		unwrap_or_bail(o, b);
	}
}

/// A function that never returns.
fn bail(b: u8) -> ! {
	panic!("failed on {b:#x}")
}

/// `return`, and `break` out of the function's labeled block, with values that never exist.
#[allow(unreachable_code, clippy::needless_return, reason = "`return` of `!` values")]
fn returned_never(b: u8, len: usize) -> u8 {
	match b {
		0xf7 => return panic!("returned {b}"),
		0xf8 => return if len % 2 == 0 { panic!("then {b}") } else { bail(b) },
		_ => {}
	}
	'value: {
		if b == 0xfe {
			break 'value bail(b);
		}
		b ^ 0x11
	}
}

/// A `return` of a `match` that guards, whose arms fall through to the next.
#[allow(clippy::needless_return, reason = "the `return` is not the function's tail")]
fn guarded_return(data: &[u8]) -> u8 {
	if !data.is_empty() {
		return match data {
			[a, b] if a > b => a - b,
			[a, ..] if *a > 0x80 => 1,
			_ => 2,
		};
	}
	3
}

/// A loop ending the function, left by `break` with values that never exist.
#[allow(unreachable_code, reason = "`break` with `!` values")]
fn broken_never(data: &[u8]) -> u8 {
	let mut i = 0;
	loop {
		if i >= data.len() {
			return 0xcc;
		}
		match data[i] {
			0xfb => break panic!("broken at {i}"),
			0xfc => break bail(data[i]),
			0 => return i as u8,
			_ => i += 1,
		}
	}
}

/// Values that never exist where a value is needed: bound, and passed. A loop that
/// only `return` leaves gives the function's result.
#[allow(unreachable_code, clippy::diverging_sub_expression, reason = "`!` values")]
fn bound_never(data: &[u8], out: &mut Vec<u8>) -> u8 {
	let mut i = 0;
	let found: u8 = loop {
		if i >= data.len() {
			return 0xbb;
		}
		match data[i] {
			0xef => {
				let x: u8 = panic!("bound at {i}");
				out.push(x);
			}
			0xf0 => out.push(bail(data[i])),
			0 => out.push(return i as u8),
			0x80 => out.push(loop {
				i += 1;
				if i >= data.len() || data[i] < 0x80 {
					return i as u8;
				}
			}),
			_ => {}
		}
		i += 1;
	};
	found
}

/// Loops that `break` ending a guarded `match` arm and an `if let` chain branch.
fn branch_loops(data: &[u8], out: &mut Vec<u8>) {
	let mut i = 0;
	while i + 1 < data.len() {
		let w = &data[i..i + 2];
		i += 2;
		match w {
			[a, b] if a > b => {
				let mut n = *a;
				loop {
					if n <= *b {
						break;
					}
					n -= 1;
					out.push(n);
				}
			}
			[a, ..] => out.push(*a),
			_ => {}
		}
		if let [a, b] = w
			&& a % 3 == b % 3
		{
			let mut n = *a;
			loop {
				n /= 2;
				out.push(n);
				if n < 4 {
					break;
				}
			}
		} else {
			out.push(0xaa);
		}
	}
}

/// A loop that only `return` leaves, returned itself, around loops that `break`.
#[allow(unreachable_code, clippy::needless_return)]
fn returned_loop(data: &[u8]) -> u8 {
	let mut i = 0;
	return loop {
		if i >= data.len() {
			return 0xdd;
		}
		for &b in &data[i..] {
			if b == 0 {
				break;
			}
			if b > 0xf0 {
				return b ^ i as u8;
			}
		}
		i += 1;
	};
}

/// A loop that ends the function, left only by `return` from nested scopes.
fn first_repeat(data: &[u8]) -> u8 {
	let mut i = 0;
	'scan: loop {
		if i >= data.len() {
			return 0xff;
		}
		let mut j = i + 1;
		while j < data.len() {
			if data[j] == 0 {
				i = j + 1;
				continue 'scan;
			}
			match data[i] == data[j] {
				true => return data[j] ^ i as u8,
				false => {}
			}
			j += 1;
		}
		i += 1;
	}
}

/// A loop that ends the function, left by `break` with a value from nested scopes.
fn tail_break(data: &[u8]) -> u16 {
	let mut i = 0;
	'outer: loop {
		if i >= data.len() {
			break 0x100;
		}
		let b = data[i];
		i += 1;
		match b % 4 {
			0 => continue,
			1 => {
				for &c in &data[i..] {
					if c == b {
						break 'outer c as u16 * 2;
					}
				}
			}
			2 if b > 100 => break b as u16 + i as u16,
			_ => {
				let mut n = b;
				loop {
					if n < 10 {
						break;
					}
					n /= 3;
					if n == 7 {
						break 'outer 7;
					}
				}
			}
		}
	}
}

/// A loop that only `return` leaves, in a `match` arm that needs a value.
fn arm_loop(data: &[u8]) -> u8 {
	let mut k = 0;
	let start = match data.first() {
		Some(&b) if b >= 0x80 => b,
		_ => loop {
			if k >= data.len() {
				return 0xee;
			}
			if data[k] >= 0x80 {
				return data[k] - 0x80;
			}
			k += 1;
		},
	};
	start / 2
}

/// A generic function ending in a loop.
fn position<T: PartialEq>(items: &[T], want: &T) -> usize {
	let mut i = 0;
	loop {
		if i == items.len() || items[i] == *want {
			return i;
		}
		i += 1;
	}
}

/// A unit function ending in a loop.
fn drain(data: &[u8], out: &mut Vec<u8>) {
	let mut i = 0;
	loop {
		if i >= data.len() {
			return;
		}
		out.push(data[i] ^ 0x5a);
		i += 2;
	}
}

/// Patterns: slices, ranges, guards that bind, `@` bindings, and let chains.
pub fn patterns(data: &[u8], out: &mut Vec<u8>) {
	match data {
		[] => out.push(0),
		[one] => out.push(*one),
		[first, .., last] if first == last => out.push(2),
		[first, second, rest @ ..] => {
			out.push(*first ^ *second);
			out.push(rest.len() as u8);
		}
	}
	for &b in &data[..data.len().min(4)] {
		let class = match b {
			0 => 0,
			1..=9 => 1,
			10 | 20 | 30 => 2,
			n @ 100..200 if n % 2 == 0 => 3,
			n if n >= 250 => 4,
			_ => 5,
		};
		out.push(class);
	}
	if let [a, b, ..] = data
		&& let Some(d) = dir(*a)
		&& *b > 10
	{
		out.push(u8::from(d));
	} else {
		out.push(0xee);
	}
	let shape = shape(data);
	let area = match shape {
		Shape::Rect { width: 0, .. } | Shape::Dot => 0,
		Shape::Rect {
			corner: Point { x, .. },
			width,
			..
		} if x > 0 => width as i64 * x as i64,
		Shape::Rect { width, height, .. } => -(width as i64 * height as i64),
		Shape::Line(a, b) => (a.x - b.x) as i64 * (a.y - b.y) as i64,
	};
	out.extend_from_slice(&area.to_be_bytes());
	// One binding from either side of an or-pattern.
	let either: Result<u8, u8> = if data.len() % 2 == 0 {
		Ok(data.len() as u8)
	} else {
		Err(0xa0)
	};
	match either {
		Ok(v) | Err(v) => out.push(v),
	}
	// An empty repeat, which MIR never assigns since it holds nothing.
	let none = [data.len() as u8; 0];
	out.push(none.len() as u8 + none.iter().sum::<u8>());
	// A rest pattern borrowing bytes changes them where they are.
	let mut bytes = [data.len() as u8, 1, 2];
	let [_, tail @ ..] = &mut bytes;
	tail[0] = 9;
	out.extend_from_slice(&bytes);
	let total = total(&[shape, Shape::Dot], out) + total(&[Meters(3), Meters(u64::MAX)], out);
	out.extend_from_slice(&total.to_be_bytes());

	// An `Err` of `Void` never exists, so its arm matches nothing, or is left out.
	let parsed: Result<u8, Void> = Ok(data.len() as u8);
	let a = match parsed {
		Ok(v) => v,
		Err(e) => absurd((e, 1)),
	};
	let b = match parsed {
		Ok(v) => v,
		Err(e) => match e {},
	};
	let c = match parsed {
		Ok(v) => v,
	};
	out.extend_from_slice(&[a, b, c, early(a)]);
	out.extend_from_slice(format!("{parsed:?}").as_bytes());
	payloads(data, out);
	known(data, out);
	rematched(data, out);
}

/// `n`'s result: `Ok` for an even `n`.
#[inline(never)]
fn parity(n: u8) -> Result<u8, u8> {
	if n % 2 == 0 { Ok(n) } else { Err(n) }
}

/// A struct holding an enum, changed through a borrow of its field.
struct Slot {
	res: Result<u8, u8>,
}

/// Payloads changed through a borrow, which changes the enum holding them where it is
/// and none of its copies.
fn payloads(data: &[u8], out: &mut Vec<u8>) {
	let n = data.len() as u8;
	let mut res = parity(n);
	let before = res;
	if let Err(e) = &mut res {
		*e = e.wrapping_add(1);
	}
	if let Ok(v) = &mut res {
		*v ^= 0x55;
	}
	let mut err: Result<u8, u8> = Err(n);
	if let Err(e) = &mut err {
		*e = e.wrapping_add(3);
	}
	let mut owned: Result<u8, Vec<u8>> = Ok(n);
	if let Ok(v) = &mut owned {
		*v = v.wrapping_add(2);
	}
	let mut slot = Slot {
		res: parity(n.wrapping_add(1)),
	};
	if let Err(e) = &mut slot.res {
		*e = e.wrapping_sub(4);
	}
	let mut flow: ControlFlow<u8, u8> = ControlFlow::Break(n);
	if let ControlFlow::Break(b) = &mut flow {
		*b = b.wrapping_add(9);
	}
	let mut opt = Some(n);
	if let Some(v) = &mut opt {
		*v = v.wrapping_add(7);
	}
	let mut poll = Poll::Ready(n);
	if let Poll::Ready(v) = &mut poll {
		*v = v.wrapping_sub(1);
	}
	let mut nested: Poll<Option<Result<u8, u8>>> = Poll::Ready(Some(parity(n.wrapping_add(2))));
	if let Poll::Ready(Some(Err(v))) = &mut nested {
		*v = v.wrapping_add(6);
	}
	let mut caught = parity(n.wrapping_add(3));
	let mut bump = || {
		if let Err(e) = &mut caught {
			*e = e.wrapping_add(5);
		}
	};
	bump();
	bump();
	let mut replaced = parity(n.wrapping_add(4));
	if let Ok(v) = &mut replaced {
		out.push(std::mem::replace(v, 0x42));
	}
	let mut named: Result<String, u8> = Ok(String::new());
	if let Ok(s) = &mut named {
		s.push('r');
	}
	out.push(match named {
		Ok(s) => s.len() as u8,
		Err(e) => e,
	});
	let mut taken: Result<Option<u8>, u8> = Ok(n.checked_sub(1));
	if let Ok(o) = &mut taken {
		out.push(o.take().unwrap_or(0x43));
	}
	if let Poll::Ready(Some(r)) = nested {
		out.push(match r {
			Ok(v) => v,
			Err(e) => e ^ 0x80,
		});
	}
	out.push(match taken {
		Ok(o) => o.unwrap_or(0x44),
		Err(e) => e,
	});
	for r in [before, res, err, slot.res, caught, replaced] {
		out.extend_from_slice(&match r {
			Ok(v) => [0, v],
			Err(e) => [1, e],
		});
	}
	out.push(match owned {
		Ok(v) => v,
		Err(e) => e.len() as u8,
	});
	out.extend_from_slice(&[
		match flow {
			ControlFlow::Break(b) => b,
			ControlFlow::Continue(c) => c,
		},
		opt.unwrap_or(0),
		match poll {
			Poll::Ready(v) => v,
			Poll::Pending => 0,
		},
	]);
}

/// Enums matched where the variant they were made with is in sight.
fn known(data: &[u8], out: &mut Vec<u8>) {
	let n = data.len() as u8;
	let ready: Poll<u8> = Poll::Ready(n);
	let copied = ready;
	let idle: Poll<Point> = Poll::Pending;
	let none: Option<Point> = None;
	let some = Some(Point::new(n as i32, 1));
	out.extend_from_slice(&[
		match ready {
			Poll::Pending => 0,
			Poll::Ready(v) => v,
		},
		match copied {
			Poll::Pending => 0,
			Poll::Ready(v) => v.wrapping_add(1),
		},
		match idle {
			Poll::Pending => 0xff,
			Poll::Ready(p) => p.x as u8,
		},
		match none {
			Some(p) => p.y as u8,
			None => 0xfe,
		},
		match some {
			Some(p) => p.x as u8,
			None => 0xfd,
		},
	]);
	let dot = Shape::Dot;
	let line = Shape::Line(Point::new(n as i32, 2), Point::new(3, 4));
	out.push(match dot {
		Shape::Dot => 1,
		Shape::Line(from, _) => from.x as u8,
		Shape::Rect { width, .. } => width as u8,
	});
	out.push(match line {
		Shape::Dot => 1,
		Shape::Line(from, _) => from.x as u8,
		Shape::Rect { width, .. } => width as u8,
	});
	let east = Dir::East;
	out.push(match east {
		Dir::North => 1,
		Dir::East => 2,
		_ => 3,
	});
	let equal = std::cmp::Ordering::Equal;
	out.push(match equal {
		std::cmp::Ordering::Less => 1,
		std::cmp::Ordering::Equal => 2,
		std::cmp::Ordering::Greater => 3,
	});
	// A closure tests what it captured.
	let later = Poll::Ready(n);
	let none_later: Option<Point> = None;
	let read = || {
		let a = match later {
			Poll::Pending => 0,
			Poll::Ready(v) => v,
		};
		a ^ none_later.map_or(0x10, |p| p.x as u8)
	};
	out.push(read());
	let moved = move || match none_later {
		Some(p) => p.y as u8,
		None => 0x20,
	};
	out.push(moved());
}

/// Enums matched again where an enclosing match decided their variant.
fn rematched(data: &[u8], out: &mut Vec<u8>) {
	let s = shape(data);
	out.push(match s {
		Shape::Dot => match s {
			Shape::Line(..) => 1,
			_ => 2,
		},
		Shape::Line(from, _) => match s {
			Shape::Line(_, to) => (from.x ^ to.y) as u8,
			Shape::Dot | Shape::Rect { .. } => 3,
		},
		Shape::Rect { .. } => 4,
	});
	out.push(match s {
		Shape::Dot => {
			let copy = s;
			match copy {
				Shape::Line(..) => 5,
				_ => 6,
			}
		}
		_ => 7,
	});
	// An arm that matches what the others left, and a borrow matched as the place.
	out.push(match s {
		Shape::Dot => 8,
		_ => match &s {
			Shape::Dot => 9,
			Shape::Line(..) => 10,
			Shape::Rect { width, .. } => *width as u8,
		},
	});
	let d = dir(data.len() as u8);
	out.push(match d {
		Some(Dir::North) => match d {
			Some(Dir::East) => 11,
			Some(_) => 12,
			None => 13,
		},
		Some(d) => match d {
			Dir::North => 14,
			_ => d as u8,
		},
		None => 15,
	});
	let order = (data.len() as u8).cmp(&2);
	out.push(match order {
		std::cmp::Ordering::Less => match order {
			std::cmp::Ordering::Equal => 16,
			_ => 17,
		},
		_ => 0,
	});
	// A library call's test narrows what it reads, as a match would.
	if order == std::cmp::Ordering::Less {
		out.push(match order {
			std::cmp::Ordering::Equal => 31,
			_ => 32,
		});
	}
	let r: Result<u8, u8> = if data.len() > 1 { Ok(1) } else { Err(2) };
	if r.is_ok() {
		out.push(match r {
			Ok(v) => v,
			Err(e) => e,
		});
	}
	// So does what the place was assigned, before such a test.
	let mut assigned = std::cmp::Ordering::Less;
	if data.len() > 3 {
		assigned = std::cmp::Ordering::Greater;
	}
	if assigned == std::cmp::Ordering::Equal {
		out.push(34);
	}
	let flow = step(0, data.len() as u8);
	if flow.is_break() {
		out.push(match flow {
			ControlFlow::Continue(n) => n as u8,
			ControlFlow::Break(b) => b.unwrap_or(33),
		});
	}
	// A map's entry is one value for either variant, which TypeScript narrows nothing by.
	let mut counts = std::collections::HashMap::new();
	if data.len() % 2 == 0 {
		counts.insert(data.len() as u8, 35);
	}
	let entry = Some(counts.entry(data.len() as u8));
	if entry.is_some() {
		out.push(match entry {
			Some(std::collections::hash_map::Entry::Occupied(e)) => *e.get(),
			Some(std::collections::hash_map::Entry::Vacant(e)) => *e.insert(36),
			None => 37,
		});
	}
	// What changes the place between the matches undecides it.
	let mut m = s;
	out.push(match m {
		Shape::Dot => {
			m = shape(&data[data.len().min(1)..]);
			match m {
				Shape::Line(..) => 18,
				_ => 19,
			}
		}
		_ => 20,
	});
	let mut held = Some(m);
	out.push(match held {
		Some(_) => {
			let taken = held.take();
			match held {
				Some(_) => 21,
				None => taken.map_or(22, |t| t.area() as u8),
			}
		}
		None => 22,
	});
	// What a loop knows on entry is what every way into it knows.
	let mut cur = s;
	for i in 0..3u8 {
		cur = match cur {
			Shape::Dot => Shape::Line(Point::new(i as i32, 0), Point::new(0, 0)),
			Shape::Line(from, _) => match cur {
				Shape::Line(_, to) => Shape::Rect {
					corner: from,
					width: to.x as u16 + 1,
					height: 2,
				},
				_ => Shape::Dot,
			},
			Shape::Rect { width, .. } if width > 1 => Shape::Dot,
			Shape::Rect { .. } => cur,
		};
		out.push(cur.area() as u8);
	}
	let mut machine = Machine { state: s, steps: 0 };
	for _ in 0..4 {
		out.push(machine.run(data.len() as u8));
	}
	let mut filled = Filled {
		shape: dir(data.len() as u8).map(|_| s),
	};
	for _ in 0..3 {
		out.push(filled.run());
	}
	// A copy keeps what was known of its original after the original changes.
	let mut original = shape(data);
	if let Shape::Dot = original {
		let copy = original;
		original = shape(&[1]);
		out.push(match copy {
			Shape::Line(..) => 23,
			_ => 24,
		});
		out.push(match original {
			Shape::Line(..) => 25,
			_ => 26,
		});
	}
	// An `if` or a guard between the matches keeps what the outer one decided.
	out.push(match s {
		Shape::Line(from, _) => {
			let lift = if data.len() > 2 { 1 } else { 2 };
			match s {
				Shape::Line(_, to) => (from.y ^ to.x) as u8 ^ lift,
				_ => 48,
			}
		}
		_ => 49,
	});
	out.push(match s {
		Shape::Dot if data.len() > 1 => match s {
			Shape::Line(..) => 50,
			_ => 51,
		},
		Shape::Rect { width, .. } if width > 300 => match s {
			Shape::Rect { height, .. } => height as u8,
			_ => 52,
		},
		Shape::Dot => match s {
			Shape::Dot => 53,
			_ => 54,
		},
		_ => 55,
	});
	// A loop's test keeps only what every way around it knows.
	let mut w = s;
	if let Shape::Dot = w {
		let mut i = 0;
		while i < data.len().min(3) {
			out.push(match w {
				Shape::Dot => {
					w = shape(&[1]);
					56
				}
				Shape::Line(..) => 57,
				_ => 58,
			});
			i += 1;
		}
	}
	reseated(data, out);
	decided_by_bool(data, out);
}

/// A match a `bool` already decided: `matches!`, and `==` of a derived `PartialEq`.
fn decided_by_bool(data: &[u8], out: &mut Vec<u8>) {
	let s = shape(data);
	if matches!(s, Shape::Dot) {
		out.push(match s {
			Shape::Line(..) => 60,
			_ => 61,
		});
	}
	if matches!(s, Shape::Line(..) | Shape::Rect { .. }) {
		out.push(match s {
			Shape::Line(from, _) => from.x as u8,
			Shape::Rect { width, .. } => width as u8,
			Shape::Dot => 62,
		});
	}
	if !matches!(s, Shape::Line(..)) {
		out.push(match s {
			Shape::Line(_, to) => to.y as u8,
			_ => 63,
		});
	}
	// A `bool` that is `false` on a way in that knows nothing of the place.
	let wide = data.len() > 2 && matches!(s, Shape::Rect { .. });
	out.push(match wide {
		true => match s {
			Shape::Rect { height, .. } => height as u8,
			_ => 64,
		},
		false => match s {
			Shape::Rect { .. } => 65,
			_ => 66,
		},
	});
	if s == Shape::Dot {
		out.push(match s {
			Shape::Line(..) => 67,
			_ => 68,
		});
	}
	if s != Shape::Dot {
		out.push(match s {
			Shape::Dot => 69,
			_ => 70,
		});
	}
	// Another `Line` may hold other points, so `!=` rules out no variant with fields.
	let line = Shape::Line(Point::new(0, 0), Point::new(0, 0));
	if s == line {
		out.push(match s {
			Shape::Line(_, to) => to.x as u8 ^ 71,
			_ => 72,
		});
	}
	if s != line {
		out.push(match s {
			Shape::Line(..) => 73,
			_ => 74,
		});
	}
	let t = shape(&data[data.len().min(1)..]);
	if let Shape::Rect { .. } = t
		&& s == t
	{
		out.push(match s {
			Shape::Rect { width, .. } => width as u8 ^ 75,
			_ => 76,
		});
	}
	// What changes the place or the `bool` after the test undecides the match.
	let mut m = s;
	let dot = matches!(m, Shape::Dot);
	m = t;
	if dot {
		out.push(match m {
			Shape::Line(..) => 77,
			_ => 78,
		});
	}
	let mut dot = matches!(s, Shape::Dot);
	if data.len() > 3 {
		dot = !dot;
	}
	if dot {
		out.push(match s {
			Shape::Line(..) => 79,
			_ => 80,
		});
	}
	// So does a write through a pointer to the `bool`.
	let mut seen = matches!(s, Shape::Dot);
	let at = &raw mut seen;
	match seen {
		true => {
			// SAFETY: `at` points at `seen`, which nothing else borrows.
			unsafe { *at = data.len() > 4 };
			out.push(match seen {
				true => 83,
				false => 84,
			});
		}
		false => out.push(85),
	}
	// A `bool` carried around a loop, set from what the place was before it changed.
	let mut cur = s;
	let mut was_dot = false;
	for i in 0..3u8 {
		if was_dot {
			out.push(match cur {
				Shape::Dot => 81,
				_ => 82,
			});
		}
		was_dot = matches!(cur, Shape::Dot);
		cur = shape(&[i, i]);
	}
}

/// A reference pointed elsewhere without an assignment to it: through a `&mut` to it,
/// a reborrow of that, a closure, and a call.
fn reseated(data: &[u8], out: &mut Vec<u8>) {
	let (a, b) = (shape(data), shape(&[1]));
	let mut r = &a;
	out.push(match *r {
		Shape::Dot => {
			let rr = &mut r;
			*rr = &b;
			match *r {
				Shape::Line(..) => 27,
				_ => 28,
			}
		}
		_ => 29,
	});
	let mut r = &a;
	out.push(match *r {
		Shape::Dot => {
			let rr = &mut r;
			let again = &mut *rr;
			*again = &b;
			match *r {
				Shape::Line(..) => 30,
				_ => 31,
			}
		}
		_ => 32,
	});
	let mut r = &a;
	out.push(match *r {
		Shape::Dot => {
			let mut point = || r = &b;
			point();
			match *r {
				Shape::Line(..) => 36,
				_ => 37,
			}
		}
		_ => 38,
	});
	let mut r = &a;
	let mut other = &b;
	out.push(match *r {
		Shape::Dot => {
			std::mem::swap(&mut r, &mut other);
			match *r {
				Shape::Line(..) => 39,
				_ => 40,
			}
		}
		_ => 41,
	});
	let mut r = &a;
	for _ in 0..2 {
		out.push(match *r {
			Shape::Dot => {
				let rr = &mut r;
				*rr = &b;
				match *r {
					Shape::Line(..) => 42,
					_ => 43,
				}
			}
			_ => 44,
		});
	}
	out.push(rebound(&a, &b));
}

/// A parameter pointed elsewhere, after a copy of it that still points where it did.
fn rebound<'a>(mut r: &'a Shape, b: &'a Shape) -> u8 {
	let kept = r;
	let rr = &mut r;
	*rr = b;
	match *r {
		Shape::Line(..) => match *kept {
			Shape::Line(..) => 45,
			_ => 46,
		},
		_ => 47,
	}
}

/// A state a method matches, changing it between its matches.
struct Machine {
	state: Shape,
	steps: u8,
}

impl Machine {
	fn step(&mut self) {
		self.steps += 1;
		self.state = match self.state {
			Shape::Dot => Shape::Line(Point::new(1, 2), Point::new(3, 4)),
			Shape::Line(from, _) => Shape::Rect {
				corner: from,
				width: 5,
				height: 6,
			},
			Shape::Rect { .. } => Shape::Dot,
		};
	}

	/// Reads the state, which a shared borrow can't change.
	#[inline(never)]
	fn peek(&self) -> u8 {
		self.steps ^ self.state.area() as u8
	}

	#[inline(never)]
	fn log(&mut self, v: u8) -> u8 {
		self.steps = self.steps.wrapping_add(v);
		self.steps
	}

	fn run(&mut self, n: u8) -> u8 {
		match self.state {
			Shape::Dot => {
				self.step();
				match self.state {
					Shape::Line(_, to) => to.x as u8 ^ self.steps ^ n,
					_ => 27,
				}
			}
			Shape::Line(..) => {
				let seen = self.peek();
				match &self.state {
					Shape::Line(_, to) => to.y as u8 ^ seen,
					_ => 28,
				}
			}
			Shape::Rect { width, .. } => {
				// A two-phase borrow of `self`, which the match reads past.
				let logged = self.log(match self.state {
					Shape::Rect { height, .. } => height as u8,
					_ => 29,
				});
				self.step();
				match self.state {
					Shape::Rect { .. } => 30,
					_ => width as u8 ^ logged,
				}
			}
		}
	}
}

/// An `Option` a method matches, filling it between its matches.
struct Filled {
	shape: Option<Shape>,
}

impl Filled {
	#[inline(never)]
	fn fill(&mut self) {
		self.shape = match self.shape {
			None => Some(Shape::Line(Point::new(5, 6), Point::new(7, 8))),
			Some(Shape::Dot) => Some(Shape::Line(Point::new(1, 2), Point::new(3, 4))),
			Some(_) => None,
		};
	}

	fn run(&mut self) -> u8 {
		match self.shape {
			None => {
				self.fill();
				match self.shape {
					Some(Shape::Line(_, to)) => to.y as u8,
					Some(_) => 40,
					None => 41,
				}
			}
			Some(Shape::Dot) => {
				self.fill();
				match self.shape {
					Some(Shape::Line(from, _)) => from.x as u8,
					_ => 42,
				}
			}
			Some(_) => {
				self.fill();
				43
			}
		}
	}
}

/// An enum without variants: no value of it exists.
#[derive(Clone, Copy, Debug)]
pub enum Void {}

/// A value holding a `Void` can't exist either.
fn absurd(v: (Void, u8)) -> u8 {
	match v {}
}

/// A `match` without arms on a scrutinee that diverges, not on an empty type.
#[allow(unreachable_code)]
fn early(n: u8) -> u8 {
	match return n.wrapping_add(1) {}
}

/// A struct whose fields' other variants can't exist.
struct Parts {
	left: Result<u8, Infallible>,
	right: Option<Result<u16, Void>>,
}

/// An enum whose other variant holds a value that can't exist.
enum Partial {
	Real(u8),
	#[allow(dead_code)]
	Never(Void),
}

/// A `Result` that can't fail, which counts its calls so their order shows.
fn counted(calls: &mut u8, b: u8) -> Result<u8, Infallible> {
	*calls = calls.wrapping_add(1);
	Ok(b.wrapping_mul(*calls))
}

/// Patterns on parts of a place whose other variants can't exist, which test nothing
/// there, while what they read still runs in order.
pub fn uninhabited(data: &[u8], out: &mut Vec<u8>) {
	let a = data.first().copied().unwrap_or(0);
	let b = data.get(1).copied().unwrap_or(0);
	let mut calls = 0;
	let (Ok(x), Ok(y)) = (counted(&mut calls, a), counted(&mut calls, b));
	out.extend_from_slice(&[x, y, calls]);
	let [Ok(x), Ok(y)] = [counted(&mut calls, b), counted(&mut calls, a)];
	out.extend_from_slice(&[x, y, calls]);
	// A call's result read only to find its one variant.
	let Ok(_) = counted(&mut calls, a);
	out.push(calls);
	let (Ok(_),) = (counted(&mut calls, a),);
	out.push(calls);
	let (Ok(_), (Ok(_),)) = (counted(&mut calls, a), (counted(&mut calls, b),));
	out.push(calls);
	let pair = (counted(&mut calls, a), Partial::Real(b));
	let (Ok(x), Partial::Real(y)) = pair;
	out.extend_from_slice(&[x, y]);

	let held = Parts {
		left: Ok(a),
		right: (b % 2 == 0).then_some(Ok(u16::from(b) << 4)),
	};
	let Parts { left: Ok(l), right } = held;
	match right {
		Some(Ok(r)) => out.extend_from_slice(&r.to_be_bytes()),
		None => out.push(l),
	}
	match (Partial::Real(a), held.left) {
		(Partial::Real(p), Ok(q)) => out.push(p ^ q),
	}

	// The other part can fail.
	if let (Ok(x), Some(&y)) = (counted(&mut calls, a), data.get(2)) {
		out.extend_from_slice(&[x, y]);
	} else {
		out.push(calls);
	}
	let (Ok(x), Some(&y)) = (counted(&mut calls, b), data.get(3)) else {
		out.push(calls);
		return;
	};
	out.extend_from_slice(&[x, y, calls]);
}

/// Closures capturing by reference and by move, and functions as values.
pub fn closures(data: &[u8], out: &mut Vec<u8>) {
	let mut sum = 0u32;
	let mut add = |v: u8| sum += v as u32;
	for &b in data {
		add(b);
	}
	out.extend_from_slice(&sum.to_be_bytes());

	let base = data.len() as u32;
	let scale = move |v: u32| v * 3 + base;
	out.extend_from_slice(&apply(scale, sum).to_be_bytes());
	// A `move` closure copies what it captured, which a later change to the original misses.
	let mut n = base;
	let copied = move || n + 1;
	n += 10;
	out.extend_from_slice(&(copied() + n).to_be_bytes());

	// A closure writing a whole array through a reference it captured changes the caller's.
	let mut whole = [data.len() as u8];
	overwrite(&mut whole);
	out.push(whole[0]);

	let mut parsed: Vec<Option<Dir>> = Vec::new();
	for &b in data {
		parsed.push(dir(b));
	}
	let mut tags = Vec::new();
	for d in &parsed {
		tags.push(d.map(u8::from).unwrap_or(0));
	}
	out.extend_from_slice(&tags);
}

/// Replaces `a` whole from a closure.
fn overwrite(a: &mut [u8; 1]) {
	let mut write = || *a = [9];
	write();
}

fn apply<F: Fn(u32) -> u32>(f: F, v: u32) -> u32 {
	f(v)
}

/// Options, results, `?`, and strings.
pub fn library(data: &[u8], out: &mut Vec<u8>) {
	let checked = |a: u8, b: u8| -> Option<u8> {
		let sum = a.checked_add(b)?;
		let doubled = sum.checked_mul(2)?;
		Some(doubled - 1)
	};
	let a = data.first().copied().unwrap_or(0);
	let b = data.get(1).copied().unwrap_or(0);
	out.push(checked(a, b).unwrap_or(0xff));

	let parsed: Result<u8, &str> = match a {
		0 => Err("zero"),
		n => Ok(n),
	};
	let mapped = parsed.map(|n| n / 2).map_err(|e| e.len() as u8);
	match mapped {
		Ok(n) => out.extend_from_slice(&[0, n]),
		Err(n) => out.extend_from_slice(&[1, n]),
	}

	// `?` in a function that returns a `Poll`, converting the error.
	let ready = |a: u8| -> std::task::Poll<Result<u8, u8>> {
		let half: Result<u8, Dir> = match dir(a) {
			Some(d) if a % 2 == 1 => Err(d),
			_ => Ok(a / 2),
		};
		let n = half?;
		std::task::Poll::Ready(Ok(n))
	};
	match ready(b) {
		std::task::Poll::Ready(Ok(n)) => out.extend_from_slice(&[2, n]),
		std::task::Poll::Ready(Err(e)) => out.extend_from_slice(&[3, e]),
		std::task::Poll::Pending => out.push(4),
	}

	match String::from_utf8(data.to_vec()) {
		Ok(s) => {
			out.push(1);
			out.extend_from_slice(&(s.len() as u32).to_be_bytes());
			out.extend_from_slice(s.as_bytes());
		}
		Err(_) => out.push(0),
	}

	// `then` calls its closure only when true, a function as well as a closure.
	let mut calls = 0u8;
	for &x in data.iter().take(6) {
		let half = (x % 2 == 0).then(|| {
			calls += 1;
			x / 2
		});
		out.extend_from_slice(&[half.unwrap_or(0xee), calls]);
		let empty = (x > 0x80).then(Vec::<u8>::new);
		out.push(empty.map_or(2, |v| v.len() as u8));
		let flag = x % 3 == 0;
		out.push(u8::from(flag) + i8::from(flag) as u8 + usize::from(flag) as u8);
		out.extend_from_slice(&u64::from(flag).to_be_bytes());
		out.extend_from_slice(&u128::from(flag).to_be_bytes());
		out.extend_from_slice(&i64::from(flag).to_be_bytes());
	}

	// `ok_or` of a local TypeScript sees is `undefined`.
	let none: Option<u8> = None;
	match none.ok_or(a) {
		Ok(n) => out.extend_from_slice(&[5, n]),
		Err(e) => out.extend_from_slice(&[6, e]),
	}
	const NONE: Option<u8> = None;
	for result in [None.ok_or(b), NONE.ok_or(a)] {
		match result {
			Ok(n) => out.extend_from_slice(&[5, n]),
			Err(e) => out.extend_from_slice(&[6, e]),
		}
	}
}

/// A result with nothing set yet.
fn unset() -> Result<Option<u8>, u8> {
	Ok(None)
}

/// Steps a total by `x`: breaks with what it can't take, or continues with the new total.
fn step(total: u16, x: u8) -> ControlFlow<Option<u8>, u16> {
	match total.checked_add(u16::from(x) * 0x40) {
		_ if x == 0 => ControlFlow::Break(None),
		Some(n) if n > 0x2000 => ControlFlow::Break(Some(x)),
		Some(n) => ControlFlow::Continue(n),
		None => ControlFlow::Break(Some(0xff)),
	}
}

/// A step that may be pending, as a polled read is.
fn polled(x: u8) -> Poll<Result<ControlFlow<()>, u8>> {
	match x % 4 {
		0 => Poll::Pending,
		1 => Poll::Ready(Err(x)),
		2 => Poll::Ready(Ok(ControlFlow::Break(()))),
		_ => Poll::Ready(Ok(ControlFlow::Continue(()))),
	}
}

/// `ControlFlow`s made by hand rather than by `?`, and a value replaced through a
/// `MutexGuard`.
pub fn flows(data: &[u8], out: &mut Vec<u8>) {
	let mut total = 0u16;
	for &x in data {
		let flow = step(total, x);
		out.extend_from_slice(&[flow.is_break() as u8, flow.is_continue() as u8]);
		match flow {
			ControlFlow::Continue(n) => total = n,
			ControlFlow::Break(Some(b)) => out.push(b),
			ControlFlow::Break(None) => out.push(0xfe),
		}
		match polled(x) {
			Poll::Pending => out.push(0),
			Poll::Ready(Err(e)) => out.push(e),
			Poll::Ready(Ok(flow)) => out.push(2 + flow.is_break() as u8),
		}
	}
	out.extend_from_slice(&total.to_be_bytes());

	// The two shapes `?` branches on, made by hand.
	let a = data.first().copied().unwrap_or(0);
	let result: ControlFlow<Result<Infallible, u8>, u16> = match a % 3 {
		0 => ControlFlow::Break(Err(a)),
		_ => ControlFlow::Continue(u16::from(a) + 0x100),
	};
	out.push(result.is_continue() as u8);
	match result {
		ControlFlow::Continue(n) => out.extend_from_slice(&n.to_be_bytes()),
		ControlFlow::Break(Err(e)) => out.push(e),
	}
	let option: ControlFlow<Option<Infallible>, u8> = match a % 5 {
		0 => ControlFlow::Break(None),
		_ => ControlFlow::Continue(a / 5),
	};
	out.push(option.is_break() as u8);
	match option {
		ControlFlow::Continue(n) => out.push(n),
		ControlFlow::Break(None) => out.push(0xfd),
	}

	// A copy holds its own copy of what it holds.
	let original: ControlFlow<(), [u8; 2]> = ControlFlow::Continue([a, 1]);
	let mut copied = original;
	if let ControlFlow::Continue(ref mut v) = copied {
		v[0] = v[0].wrapping_add(1);
	}
	for flow in [original, copied] {
		if let ControlFlow::Continue(v) = flow {
			out.extend_from_slice(&v);
		}
	}

	// Every `Ok(None)` is one shared object: a store through a `ref mut` to its payload
	// makes the result again, so the next stays `None`.
	let mut set = unset();
	if let Ok(ref mut n) = set {
		*n = Some(a);
	}
	for result in [set, unset()] {
		out.push(match result {
			Ok(Some(n)) => n,
			Ok(None) => 0xfe,
			Err(e) => e,
		});
	}

	let slot = Mutex::new(None);
	for &x in data {
		let mut held = slot.lock().unwrap();
		if !matches!(*held, Some(h) if h >= x) {
			*held = Some(x);
		}
	}
	out.push(slot.lock().unwrap().unwrap_or(0xfc));
}
