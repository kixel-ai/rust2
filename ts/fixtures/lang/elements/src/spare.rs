//! A `Vec`'s spare capacity, the room past its length: written through `as_mut_ptr` moved
//! past the length, by element, by `ptr::copy`, and through a slice of the pointers, and
//! through the slots `spare_capacity_mut` lends, then taken in by `set_len`; shortened by
//! `set_len` and read back past its length, as std's `Drain` and `retain` read theirs, then
//! lengthened over what it gave up; and slots never written, copied as they are. Of numbers,
//! bytes, `String`s, `Option`s, values that count their drops, and values JS keeps as
//! objects, by a generic function too.

use std::mem::MaybeUninit;
use std::ptr;
use std::slice;
use std::sync::atomic::{AtomicU8, Ordering};

/// What a case's vector holds, made of a byte and put as bytes.
trait Item: Clone {
	fn of(n: u8) -> Self;
	fn put(&self, out: &mut Vec<u8>);
}

impl Item for u32 {
	fn of(n: u8) -> Self {
		u32::from(n) * 0x0101
	}

	fn put(&self, out: &mut Vec<u8>) {
		out.extend_from_slice(&self.to_le_bytes());
	}
}

impl Item for String {
	fn of(n: u8) -> Self {
		format!("{n:x}")
	}

	fn put(&self, out: &mut Vec<u8>) {
		out.extend_from_slice(self.as_bytes());
		out.push(0xff);
	}
}

/// Bytes, whose length JS keeps apart from them, as Rust does, so a pointer moves one exactly.
impl Item for Vec<u8> {
	fn of(n: u8) -> Self {
		vec![n; usize::from(n % 3)]
	}

	fn put(&self, out: &mut Vec<u8>) {
		out.extend_from_slice(self);
		out.push(0xff);
	}
}

impl Item for Option<u16> {
	fn of(n: u8) -> Self {
		(n % 3 != 0).then_some(u16::from(n) << 4)
	}

	fn put(&self, out: &mut Vec<u8>) {
		out.extend(self.map_or([0xee, 0xee], u16::to_le_bytes));
	}
}

/// How many `Noted`s dropped since the case began.
static DROPS: AtomicU8 = AtomicU8::new(0);

/// A value that counts its drops, so what `set_len` gives up shows it drops nothing. JS
/// keeps it as an object, so no pointer into a `Vec` of them is modeled.
#[derive(Clone)]
struct Noted(u8);

impl Drop for Noted {
	fn drop(&mut self) {
		DROPS.fetch_add(1, Ordering::Relaxed);
	}
}

impl Item for Noted {
	fn of(n: u8) -> Self {
		Noted(n)
	}

	fn put(&self, out: &mut Vec<u8>) {
		out.push(self.0);
	}
}

/// A struct JS keeps as an object.
#[derive(Clone)]
struct Pair {
	a: u8,
	b: String,
}

impl Item for Pair {
	fn of(n: u8) -> Self {
		Pair { a: n, b: n.to_string() }
	}

	fn put(&self, out: &mut Vec<u8>) {
		out.push(self.a);
		self.b.put(out);
	}
}

/// A tuple JS keeps as an object.
impl Item for (u8, u32) {
	fn of(n: u8) -> Self {
		(n, u32::from(n) + 1)
	}

	fn put(&self, out: &mut Vec<u8>) {
		out.push(self.0);
		self.1.put(out);
	}
}

/// The bytes a case reads its steps from.
struct Ops<'a>(&'a [u8]);

impl Ops<'_> {
	fn byte(&mut self) -> u8 {
		let Some((&b, rest)) = self.0.split_first() else {
			return 0;
		};
		self.0 = rest;
		b
	}
}

/// The length, capacity, and room of `v`, how many `Noted`s dropped, and its items.
fn state<T: Item>(v: &mut Vec<T>, out: &mut Vec<u8>) {
	for n in [v.len(), v.capacity(), v.spare_capacity_mut().len()] {
		out.extend_from_slice(&(n as u32).to_le_bytes());
	}
	out.push(DROPS.load(Ordering::Relaxed));
	for x in v.iter() {
		x.put(out);
	}
}

/// A vector of values JS copies, written past its length through pointers, by element, by
/// `ptr::copy_nonoverlapping`, and through a slice made of them, and taken in by
/// `set_len`, of the `Vec` itself or of the one it moved to; an insert that moves the tail into the room, as std's does; a removal of a
/// run that shortens it by `set_len` first and reads the run past its length, as std's
/// `Drain` does; a filter that shortens it to nothing and writes back what it keeps, as
/// std's `retain` does; grown, cut, and refilled.
fn pointers<T: Item>(data: &[u8], out: &mut Vec<u8>) {
	let mut ops = Ops(data);
	let mut v: Vec<T> = Vec::with_capacity(usize::from(ops.byte() % 12));
	while !ops.0.is_empty() {
		let (op, n) = (ops.byte(), ops.byte());
		let (len, room) = (v.len(), v.capacity() - v.len());
		let k = usize::from(n) % (room + 1);
		match op % 8 {
			0 => unsafe {
				let end = v.as_mut_ptr().add(len);
				for i in 0..k {
					end.add(i).write(T::of(n.wrapping_add(i as u8)));
				}
				// The room's raw slice cast to an array, read and written through.
				if k >= 2 {
					let pair = &mut *(ptr::slice_from_raw_parts_mut(end, k) as *mut [T; 2]);
					pair.swap(0, 1);
					pair[0].put(out);
				}
				// Moved to another binding and back, which keeps its storage, read through
				// the pointer from before the move.
				let mut moved = std::mem::take(&mut v);
				for x in slice::from_raw_parts(end.sub(len), len + k) {
					x.put(out);
				}
				moved.set_len(len + k);
				v = moved;
			},
			1 => unsafe {
				let src: Vec<T> = (0..k).map(|i| T::of(n ^ i as u8)).collect();
				let src = std::mem::ManuallyDrop::new(src);
				ptr::copy_nonoverlapping(src.as_ptr(), v.as_mut_ptr().add(len), k);
				v.set_len(len + k);
			},
			2 => unsafe {
				let part = slice::from_raw_parts_mut(v.as_mut_ptr().add(len).cast::<MaybeUninit<T>>(), k);
				for (i, slot) in part.iter_mut().enumerate().rev() {
					slot.write(T::of(n.wrapping_sub(i as u8)));
				}
				v.set_len(len + k);
			},
			3 => {
				v.reserve(1);
				let at = usize::from(n) % (len + 1);
				unsafe {
					let p = v.as_mut_ptr().add(at);
					ptr::copy(p, p.add(1), len - at);
					p.write(T::of(n));
					v.set_len(len + 1);
				}
			}
			4 if len > 0 => {
				let at = usize::from(n) % len;
				let end = at + usize::from(n >> 4) % (len - at + 1);
				unsafe {
					v.set_len(at);
					let p = v.as_mut_ptr();
					for i in at..end {
						ptr::read(p.add(i)).put(out);
					}
					ptr::copy(p.add(end), p.add(at), len - end);
					v.set_len(len - (end - at));
				}
			}
			5 => unsafe {
				v.set_len(0);
				let p = v.as_mut_ptr();
				let mut kept = 0;
				for i in 0..len {
					let x = ptr::read(p.add(i));
					if (i + usize::from(n)) % 3 != 0 {
						p.add(kept).write(x);
						kept += 1;
					}
				}
				v.set_len(kept);
			},
			6 => v.reserve(usize::from(n % 16)),
			_ => v.truncate(usize::from(n) % (len + 1)),
		}
		state(&mut v, out);
	}
}

/// A vector of any items written through the slots `spare_capacity_mut` lends: by
/// `write`, by assigning one whole, by `write_clone_of_slice` and through the slice it
/// gives back, and by `fill_with`, then taken in by `set_len`; one written and read back before
/// any `set_len`, and written over by a second call, then taken in by the `Vec` it was
/// swapped into; grown and cut.
fn slots<T: Item>(data: &[u8], out: &mut Vec<u8>) {
	let mut ops = Ops(data);
	let mut v: Vec<T> = Vec::with_capacity(usize::from(ops.byte() % 12));
	while !ops.0.is_empty() {
		let (op, n) = (ops.byte(), ops.byte());
		let len = v.len();
		let spare = v.spare_capacity_mut();
		let k = usize::from(n) % (spare.len() + 1);
		match op % 6 {
			0 => {
				for (i, slot) in spare[..k].iter_mut().enumerate() {
					slot.write(T::of(n ^ i as u8));
				}
				unsafe { v.set_len(len + k) };
			}
			1 => {
				for (i, slot) in spare[..k].iter_mut().enumerate().rev() {
					*slot = MaybeUninit::new(T::of(n.wrapping_add(i as u8)));
				}
				unsafe { v.set_len(len + k) };
			}
			2 => {
				let src: Vec<T> = (0..k).map(|i| T::of(n.wrapping_mul(i as u8))).collect();
				spare[..k].write_clone_of_slice(&src).reverse();
				unsafe { v.set_len(len + k) };
			}
			3 => {
				spare[..k].fill_with(|| MaybeUninit::new(T::of(n)));
				unsafe { v.set_len(len + k) };
			}
			4 => {
				if let Some(slot) = spare.first_mut() {
					slot.write(T::of(!n));
					unsafe { slot.assume_init_ref() }.put(out);
					unsafe { slot.assume_init_drop() };
				}
				if let Some(slot) = v.spare_capacity_mut().first_mut() {
					slot.write(T::of(n));
					// Swapped out and back, which keeps its storage.
					let mut swapped = Vec::new();
					std::mem::swap(&mut v, &mut swapped);
					unsafe { swapped.set_len(len + 1) };
					v = swapped;
				}
			}
			_ if n % 2 == 0 => v.reserve(usize::from(n % 16)),
			_ => v.truncate(usize::from(n) % (len + 1)),
		}
		state(&mut v, out);
	}
}

/// Slots never written, copied as they are: within the room, by element and by
/// `ptr::copy`, and into a vector of `MaybeUninit`s lengthened over them by `set_len`,
/// each written before any read.
fn unwritten(data: &[u8], out: &mut Vec<u8>) {
	let mut v: Vec<u32> = Vec::with_capacity(data.len() + 4);
	let spare = v.spare_capacity_mut();
	spare[1] = spare[0];
	unsafe { ptr::copy(spare.as_ptr(), spare.as_mut_ptr().add(2), 2) };
	for (slot, &b) in spare.iter_mut().zip(data) {
		slot.write(u32::from(b) + 1);
	}
	unsafe { v.set_len(data.len()) };
	state(&mut v, out);

	let mut held: Vec<MaybeUninit<u16>> = Vec::with_capacity(data.len());
	unsafe { held.set_len(data.len()) };
	held.reverse();
	for (i, slot) in held.iter_mut().enumerate() {
		slot.write(u16::from(data[i]) << 1);
	}
	out.extend(held.iter().flat_map(|x| unsafe { x.assume_init() }.to_le_bytes()));
}

/// Each kind of item through the pointers and slots of its spare capacity.
pub fn spares(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	DROPS.store(0, Ordering::Relaxed);
	match kind % 10 {
		0 => pointers::<u32>(data, out),
		// A `String`'s JS object keeps its length with its text, so a pointer can't move one.
		1 => pointers::<Vec<u8>>(data, out),
		2 => pointers::<Option<u16>>(data, out),
		3 => slots::<u32>(data, out),
		4 => slots::<String>(data, out),
		5 => slots::<Option<u16>>(data, out),
		6 => slots::<Noted>(data, out),
		7 => slots::<Pair>(data, out),
		8 => slots::<(u8, u32)>(data, out),
		_ => unwritten(data, out),
	}
	out.push(DROPS.load(Ordering::Relaxed));
}
