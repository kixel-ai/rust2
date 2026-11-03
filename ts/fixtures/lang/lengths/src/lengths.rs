//! Functions generic over an array's length whose calls take values of types holding it:
//! arrays of bytes, of a type parameter's values, and of arrays, each at a length of none,
//! one, and more. Their writes are judged in the bounds of the body making them, as
//! filling memory nothing points into yet or giving what they replace another object,
//! through what std's iterators give back and what `copied` and `vec![x; n]` make of a
//! `Copy` type whose `Clone` logs and panics.

use std::cell::RefCell;
use std::mem::MaybeUninit;

/// Each case at the length `kind` picks.
pub fn lengths(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	// What a case that panicked left, which a translation outlives, as native Rust's trap does not.
	CLONES.with_borrow_mut(Vec::clear);
	match kind % 4 {
		0 => each::<0>(data, out),
		1 => each::<1>(data, out),
		2 => each::<3>(data, out),
		_ => each::<17>(data, out),
	}
	let byte = |at: usize| u16::from(data.get(at).copied().unwrap_or(0));
	repeated::<u8>(data, 0x41, 0x42, out);
	repeated::<Loud>(data, Loud(byte(1)), Loud(byte(0)), out);
	loud(data, out);
	CLONES.with_borrow_mut(|log| out.extend(log.drain(..)));
}

fn each<const N: usize>(data: &[u8], out: &mut Vec<u8>) {
	let rows: Vec<[u8; N]> = data
		.chunks(N.max(1))
		.map(|c| std::array::from_fn(|i| c.get(i).copied().unwrap_or(i as u8)))
		.collect();
	let wide: Vec<[u32; N]> = rows.iter().map(|r| r.map(|b| u32::from(b) << 8 | 1)).collect();
	joined(data, rows.clone(), out);
	joined(data, vec![[0xa5; N], std::array::from_fn(|i| i as u8)], out);
	retagged(&rows, out);
	recloned(&wide, out);
	filled::<u8, N>(data, std::array::from_fn(|i| i as u8), out);
	filled::<u32, N>(data, std::array::from_fn(|i| 0x0102_0304 * i as u32), out);
	filled::<[u8; N], N>(
		data,
		std::array::from_fn(|i| std::array::from_fn(|j| (i * N + j) as u8)),
		out,
	);
	filled::<Tagged<u32>, N>(
		data,
		std::array::from_fn(|i| Tagged(i as u32 * 0x0101, !(i as u8))),
		out,
	);
	swapped(data, &rows, out);
	swapped(data, &wide, out);
}

/// The bytes of `data` and then of each row, as one run.
fn joined<const N: usize>(data: &[u8], rows: Vec<[u8; N]>, out: &mut Vec<u8>) {
	out.extend(data.iter().chain(rows.iter().flatten()));
}

#[derive(Clone, Copy)]
struct Tagged<T>(T, u8);

/// Each row cloned out of an `Option` kept in a pair and written back whole, after which
/// the pair and the copy each change apart.
fn retagged<const N: usize>(rows: &[[u8; N]], out: &mut Vec<u8>) {
	for (at, &row) in rows.iter().enumerate() {
		let mut pair = (Some(Tagged(row, at as u8)), 0u8);
		#[allow(clippy::clone_on_copy, reason = "the clone `Option`'s `Clone` makes")]
		let mut copy = pair.0.clone();
		pair.0 = copy;
		if let (Some(Tagged(a, _)), Some(Tagged(b, n))) = (&mut pair.0, &mut copy) {
			a.reverse();
			*n ^= 0xff;
			out.push(*n);
			out.extend(a.iter().chain(b.iter()));
		}
	}
}

/// [`retagged`] of a type parameter's values.
fn recloned<T: Copy + Into<u32>, const N: usize>(rows: &[[T; N]], out: &mut Vec<u8>) {
	for (at, &row) in rows.iter().enumerate() {
		let mut pair = (Some(Tagged(row, at as u8)), 0u8);
		#[allow(clippy::clone_on_copy, reason = "the clone `Option`'s `Clone` makes")]
		let mut copy = pair.0.clone();
		pair.0 = copy;
		if let (Some(Tagged(a, _)), Some(Tagged(b, n))) = (&mut pair.0, &mut copy) {
			a.reverse();
			*n ^= 0xff;
			out.push(*n);
			out.extend(a.iter().chain(b.iter()).map(|&x| x.into() as u8));
		}
	}
}

/// Slots of `MaybeUninit` arrays of `T`, each written whole through an iterator while
/// nothing points into them, the last two again from the values a vector's iterator gives
/// beside them, which std drops, then changed in place, a pair of tags through a pointer to
/// it, and read back.
fn filled<T: Copy + Put, const N: usize>(data: &[u8], fill: [T; N], out: &mut Vec<u8>) {
	let mut slots: [MaybeUninit<([T; N], (u8, u8))>; 3] = [const { MaybeUninit::uninit() }; 3];
	for (slot, at) in slots.iter_mut().zip(0u8..) {
		slot.write((fill, (at, !at)));
	}
	let mut picks = Vec::new();
	for &v in &fill {
		picks.push(Some(v));
	}
	for (slot, pick) in slots.iter_mut().skip(1).zip(picks) {
		let row = match pick {
			Some(v) => [v; N],
			None => fill,
		};
		slot.write((row, (N as u8, 0x5a)));
	}
	for &byte in data {
		let slot = unsafe { slots[usize::from(byte) % 3].assume_init_mut() };
		slot.0.rotate_left(usize::from(byte) % N.max(1));
		let tags = &raw mut slot.1;
		unsafe {
			(*tags).0 ^= byte;
			(*tags).1 = (*tags).1.wrapping_add(byte);
			out.extend([(*tags).0, (*tags).1]);
		}
	}
	for slot in &slots {
		let (values, (a, b)) = unsafe { slot.assume_init_ref() };
		values.iter().for_each(|v| v.put(out));
		out.extend([*a, *b]);
	}
}

/// A value's bytes.
trait Put {
	fn put(&self, out: &mut Vec<u8>);
}

impl Put for u8 {
	fn put(&self, out: &mut Vec<u8>) {
		out.push(*self);
	}
}

impl Put for u32 {
	fn put(&self, out: &mut Vec<u8>) {
		out.extend(self.to_le_bytes());
	}
}

impl<const N: usize> Put for [u8; N] {
	fn put(&self, out: &mut Vec<u8>) {
		out.extend_from_slice(self);
	}
}

impl<T: Put> Put for Tagged<T> {
	fn put(&self, out: &mut Vec<u8>) {
		self.0.put(out);
		out.push(self.1);
	}
}

/// Rows of a grid swapped whole, each copy kept apart from the row it was read from as
/// either changes.
fn swapped<T: Copy + Into<u32>, const N: usize>(data: &[u8], rows: &[[T; N]], out: &mut Vec<u8>) {
	let Some((&first, rest)) = rows.split_first() else {
		return;
	};
	let mut grid = [first; 2];
	if let Some(&next) = rest.first() {
		grid[1] = next;
	}
	#[allow(clippy::manual_swap, reason = "the whole writes of each row")]
	for &byte in data {
		let row = grid[0];
		grid[0] = grid[1];
		grid[1] = row;
		if let (Some(x), Some(&y)) = (
			grid[usize::from(byte) % 2].first_mut(),
			rows[usize::from(byte) % rows.len()].last(),
		) {
			*x = y;
		}
		out.extend(row.iter().chain(grid.iter().flatten()).map(|&x| x.into() as u8));
	}
}

thread_local! {
	/// The value of each `Loud` cloned, in order.
	static CLONES: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
}

/// A `Copy` type whose `Clone`, written by hand, logs its value, panics on `0xff`, and
/// counts up, so a clone tells from a copy: `copied` never calls it, and `vec![x; n]` calls
/// it `n - 1` times.
#[derive(Copy)]
struct Loud(u16);

#[allow(clippy::non_canonical_clone_impl, reason = "a `Clone` that differs from the copy")]
impl Clone for Loud {
	fn clone(&self) -> Self {
		CLONES.with_borrow_mut(|log| log.extend(self.0.to_le_bytes()));
		assert_ne!(self.0, 0xff, "a clone of 0xff");
		Self(self.0.wrapping_add(1))
	}
}

impl Put for Loud {
	fn put(&self, out: &mut Vec<u8>) {
		out.extend(self.0.to_le_bytes());
	}
}

/// A value and a pair of tags kept as an object, of no byte held in place.
#[derive(Clone, Copy)]
struct Row<T>(T, (u16, u16));

/// The first row of a vector of `$len` written whole and its tags changed through a pointer
/// to them, the second's changed in place, and each row put.
macro_rules! rewritten {
	($rows:ident, $len:expr, $x:expr, $out:ident) => {
		if $len > 0 {
			$rows[0] = Row($x, (0x3031, 0x3233));
			let tags = &raw mut $rows[0].1;
			unsafe { (*tags).0 ^= 0x1010 }
		}
		if $len > 1 {
			$rows[1].1.1 ^= 0x2020;
		}
		for Row(value, (a, b)) in $rows {
			value.put($out);
			$out.extend(a.to_le_bytes().into_iter().chain(b.to_le_bytes()));
		}
	};
}

/// [`repeated`]'s body, over `$x`'s type.
macro_rules! repeated {
	($data:ident, $x:expr, $y:expr, $out:ident) => {{
		let rows = [Row($y, (0x0102, 0x0304)), Row($x, (0x0506, 0x0708))];
		let mut copies: Vec<_> = rows.iter().copied().collect();
		rewritten!(copies, 2, $x, $out);
		let mut repeats = vec![Row($x, (0x090a, 0x0b0c)); 1 + $data.len() % 3];
		rewritten!(repeats, 1 + $data.len() % 3, $x, $out);
		for Row(value, (a, b)) in &rows {
			value.put($out);
			$out.extend(a.to_le_bytes().into_iter().chain(b.to_le_bytes()));
		}
	}};
}

/// Rows of `T`'s values copied out of an array and repeated by `vec!`, which runs `T`'s
/// `Clone`, each vector then [`rewritten`], every row kept apart from the others and from
/// the array's.
fn repeated<T: Copy + Put>(data: &[u8], x: T, y: T, out: &mut Vec<u8>) {
	repeated!(data, x, y, out);
}

/// [`repeated`] of the crate's own type, in a body generic over none.
fn loud(data: &[u8], out: &mut Vec<u8>) {
	let byte = |at: usize| u16::from(data.get(at).copied().unwrap_or(0));
	repeated!(data, Loud(byte(2)), Loud(byte(0)), out);
}
