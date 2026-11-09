//! Memory from `alloc` and raw pointers read as other types' bytes or as addresses.

use std::pin::Pin;

use crate::pointers::Halves;

/// A pointer read as another type's bytes.
pub fn reinterpreted(n: u32) -> u8 {
	unsafe { *(&raw const n).cast::<u8>() }
}

/// A pointer's address, as an integer.
pub fn addressed(n: u32) -> usize {
	(&raw const n).addr()
}

/// Memory from the allocator as bytes, which JS keeps only as values a cast types it as.
pub fn allocated() -> u8 {
	let layout = std::alloc::Layout::new::<u8>();
	unsafe {
		let p = std::alloc::alloc(layout);
		*p = 1;
		let n = *p;
		std::alloc::dealloc(p, layout);
		n
	}
}

/// A whole value written through a pointer from a shared reference to a `Result` of a cell
/// whose only inhabited variant leaves it no tag, all of it in an `UnsafeCell`, which JS
/// can't write into in place.
pub fn infallible_cell(r: &Result<std::cell::Cell<u32>, std::convert::Infallible>) {
	unsafe { std::ptr::from_ref(r).cast_mut().write(Ok(std::cell::Cell::new(19))) }
}

/// A `LazyCell` moved out through a pointer before its initializer, which holds a cell,
/// runs: the closure is state only `rs.LazyCell` knows, which a copy can't reach.
pub fn lazy_moved(cell: std::cell::Cell<u32>) -> u32 {
	let lazy = std::cell::LazyCell::new(move || cell.get());
	let copy = unsafe { std::ptr::read(&lazy) };
	std::mem::forget(lazy);
	*copy
}

/// A pointer from a reference the body is given moved by an offset the body doesn't fix,
/// wrapping or not, and copied from by a count it doesn't fix, each of which may reach
/// past the value JS keeps in no array.
pub fn unfixed_reference(r: &u32, n: usize) -> [u32; 3] {
	let mut out = [0u32; 3];
	unsafe {
		out[0] = *std::ptr::from_ref(r).add(n);
		out[1] = *std::ptr::from_ref(r).wrapping_add(n);
		std::ptr::copy_nonoverlapping(std::ptr::from_ref(r), out.as_mut_ptr(), n);
	}
	out
}

/// A `Peekable` moved out through a pointer with a cell peeked, which only its runtime
/// class holds.
pub fn peeked_moved(cells: Vec<std::cell::Cell<u32>>) -> u32 {
	let mut iter = cells.into_iter().peekable();
	iter.peek();
	let mut copy = unsafe { std::ptr::read(&iter) };
	std::mem::forget(iter);
	copy.next().map_or(0, |cell| cell.get())
}

/// Collections whose JS object keeps its length with its storage, moved out through
/// pointers while the original lives on: by `ptr::read`, `ptr::copy` within a `Vec` as
/// smallvec's `retain` moves its elements, `MaybeUninit::assume_init_read` of a tuple
/// holding one, and `ManuallyDrop::take`. Rust's copy shares the storage and keeps a
/// length of its own, which no JS copy does.
pub fn collection_moved(v: Vec<u32>, mut q: std::collections::VecDeque<u32>, s: String) -> usize {
	let mut v = std::mem::ManuallyDrop::new(v);
	let old_v = unsafe { std::ptr::read(&*v) };
	unsafe { v.set_len(0) };
	let old_q = unsafe { std::ptr::read(&q) };
	q.pop_front();
	std::mem::forget(q);
	let mut words = vec![s.clone(), s.clone()];
	words[1].push('!');
	unsafe {
		std::ptr::copy(words.as_ptr().add(1), words.as_mut_ptr(), 1);
		words.set_len(1);
	}
	let pair = std::mem::MaybeUninit::new((s, 0u8));
	let old_s = unsafe { pair.assume_init_read() };
	let mut taken = std::mem::ManuallyDrop::new(vec![1u32]);
	let old_t = unsafe { std::mem::ManuallyDrop::take(&mut taken) };
	old_v.len() + old_q.len() + words[0].len() + old_s.0.len() + old_t.len()
}

/// Closures read out through a pointer from a shared reference and both called: one that
/// counts in its own capture, one capturing a cell it sets, and one moving out a vector
/// the crate writes into. Rust's copy keeps state of its own, which JS keeps in the one
/// function.
pub fn closure_moved(n: u32, bytes: Vec<u8>) -> u32 {
	fn read<F>(r: &F) -> F {
		unsafe { std::ptr::read(r) }
	}
	let mut m = n;
	let mut count = move || {
		m += 1;
		m
	};
	let mut old = read(&count);
	let cell = std::cell::Cell::new(n);
	let set = move || cell.set(cell.get() + 1);
	let old_set = read(&set);
	set();
	old_set();
	let give = move || bytes;
	let old_give = read(&give);
	let mut first = std::mem::ManuallyDrop::new(give());
	unsafe { first.set_len(0) };
	count() + old() + old_give().len() as u32
}

/// A pointer from a reference the body is given, which JS keeps in no array, read through
/// past it by a constant wrapping offset and one past it, and viewed as a slice of a
/// length the body doesn't fix: each reaches the neighbour Tree Borrows allows it to.
pub fn beside_reference(r: &u32, n: usize) -> u32 {
	let p = std::ptr::from_ref(r);
	unsafe { *p.wrapping_add(1) + *p.add(1) + std::slice::from_raw_parts(p, n).iter().sum::<u32>() }
}

/// `beside_reference`'s read one past, through a pointer cast to one JS keeps the same:
/// to a `MaybeUninit`, by `cast_mut`, and by `cast` to a `ManuallyDrop`.
pub fn recast_reference(r: &u32) -> u32 {
	let p = (&raw const *r) as *const std::mem::MaybeUninit<u32>;
	let q = std::ptr::from_ref(r).cast_mut();
	let m = std::ptr::from_ref(r).cast::<std::mem::ManuallyDrop<u32>>();
	unsafe { (*p.add(1)).assume_init() + q.add(1).read() + *m.wrapping_add(1).read() }
}

/// A pointer from a reference the body is given, which JS keeps in no array, moved past
/// it.
pub fn past_reference(r: &u32) -> u32 {
	unsafe { *(r as *const u32).add(2) }
}

/// `UnsafeCell`s of objects a pointer is the place of, which a whole write through `get`
/// lands in, so a value `replace` reads out before would change too or be a copy, and a
/// `Cell` and an array of a pinned `Box`, whose copy is another allocation.
pub fn cell_of_objects(v: &mut Vec<u8>, s: &mut String, p: &mut Pin<Box<(u8, u8)>>, q: &mut Pin<Box<(u8, u8)>>) -> u8 {
	let v = std::cell::UnsafeCell::from_mut(v);
	let s = std::cell::UnsafeCell::from_mut(s);
	let p = std::cell::UnsafeCell::from_mut(p);
	let first = unsafe {
		v.replace(Vec::new()).len() as u8 ^ s.replace(String::new()).len() as u8 ^ p.replace(Box::pin((3, 4))).0
	};
	let second = std::cell::Cell::from_mut(q).replace(Box::pin((5, 6))).0;
	let [third] = std::mem::replace(std::array::from_mut(q), [Box::pin((7, 8))]);
	first ^ second ^ third.0
}

/// The length of the `Vec` a pointer from the reference given points at, which translates:
/// a call for `cell_pointer_kept` to make between its pointer and the read.
fn vec_pointer(v: &mut Vec<u8>) -> usize {
	let p = &raw const *v;
	unsafe { (*p).len() }
}

/// A pointer to a cell behind a reference, made before a call that `UnsafeCell::raw_get`
/// comes after, which reads it in its place only right after it is made.
pub fn cell_pointer_kept(c: &std::cell::UnsafeCell<u32>, n: u32) -> u32 {
	let x = n.wrapping_mul(3);
	let p: *const std::cell::UnsafeCell<u32> = c;
	let y = vec_pointer(&mut vec![0; x as usize]);
	unsafe { *std::cell::UnsafeCell::raw_get(p) + y as u32 }
}

/// A pointer `UnsafeCell::raw_get` makes of a reference to a cell, read after the variable
/// holding the cell is given another.
pub fn cell_pointer_reassigned(n: u32) -> u32 {
	let mut c = std::cell::UnsafeCell::new(n);
	let p = std::cell::UnsafeCell::raw_get(&c);
	c = std::cell::UnsafeCell::new(n ^ 1);
	let _ = &c;
	unsafe { *p }
}

/// Pointers `UnsafeCell::raw_get` makes of a raw pointer to a cell, of one to either of
/// two, of one to it or else null or a parameter, and of a `MaybeUninit`'s, and `get`
/// through the first, read after the variable holding the cell is given another.
pub fn cell_raw_reassigned(n: u32, given: *const std::cell::UnsafeCell<u32>) -> u32 {
	let mut c = std::cell::UnsafeCell::new(n);
	let d = std::cell::UnsafeCell::new(n ^ 2);
	let to = &raw const c;
	let p = unsafe { std::cell::UnsafeCell::raw_get(to) };
	let either = if n > 3 { &raw const c } else { &raw const d };
	let q = unsafe { std::cell::UnsafeCell::raw_get(either) };
	let r = unsafe { (*to).get() };
	let or_null = if n > 5 { &raw const c } else { std::ptr::null() };
	let s = unsafe { std::cell::UnsafeCell::raw_get(or_null) };
	let or_given = if n > 7 { &raw const c } else { given };
	let t = unsafe { std::cell::UnsafeCell::raw_get(or_given) };
	let mut slot = std::mem::MaybeUninit::new(std::cell::UnsafeCell::new(n ^ 3));
	let u = unsafe { std::cell::UnsafeCell::raw_get(slot.as_ptr()) };
	c = std::cell::UnsafeCell::new(n ^ 1);
	slot = std::mem::MaybeUninit::new(std::cell::UnsafeCell::new(n ^ 4));
	let _ = (&c, &d, &slot);
	unsafe { *p ^ *q ^ *r ^ *s ^ *t ^ *u }
}

/// `UnsafeCell::raw_get` of pointers no known place of a cell makes: into an empty `Vec`'s
/// room for cells of a tuple and of a number, which hold no cell yet, past an array's first
/// cell by arithmetic, and through a parameter.
pub fn cell_raw_unknown(n: u32, given: *const std::cell::UnsafeCell<u32>) -> u32 {
	let mut pairs = Vec::<std::cell::UnsafeCell<(u32, u32)>>::with_capacity(1);
	let mut nums = Vec::<std::cell::UnsafeCell<u32>>::with_capacity(1);
	let cells = [std::cell::UnsafeCell::new(n), std::cell::UnsafeCell::new(n ^ 1)];
	unsafe {
		let empty = std::cell::UnsafeCell::raw_get(pairs.as_mut_ptr()).is_null();
		std::cell::UnsafeCell::raw_get(nums.as_mut_ptr()).write(7);
		nums.set_len(1);
		let next = *std::cell::UnsafeCell::raw_get(cells.as_ptr()).add(1);
		let at = *std::cell::UnsafeCell::raw_get(&raw const (*given));
		u32::from(empty) ^ *nums[0].get() ^ next ^ at
	}
}

/// A `MaybeUninit` of a cell, whose `assume_init_mut` is the place the cell is kept in, lent
/// as the value by `get_mut`.
pub fn uninit_cell_lent(n: u32) -> u32 {
	let mut slot = std::mem::MaybeUninit::new(std::cell::UnsafeCell::new(n));
	unsafe { *slot.assume_init_mut().get_mut() + 1 }
}

/// Whole writes of values holding a cell in a `MaybeUninit` or `ManuallyDrop`, which would
/// put another cell in place of one a pointer `get` gave points into: `UnsafeCell::replace`
/// of a wrapped tuple and of a wrapped cell, `ptr::write` of a tuple holding one,
/// `Cell::set`, and an assignment through a raw pointer.
pub fn wrapped_cells(n: u32) -> u32 {
	use std::cell::{Cell, UnsafeCell};
	use std::mem::{ManuallyDrop, MaybeUninit};
	let tuple = UnsafeCell::new(MaybeUninit::new((UnsafeCell::new(n), n)));
	let manual = UnsafeCell::new(ManuallyDrop::new(UnsafeCell::new(n)));
	let mut pair = (MaybeUninit::new(UnsafeCell::new(n)), n);
	let cell = Cell::new(ManuallyDrop::new(UnsafeCell::new(n)));
	unsafe {
		let old = tuple.replace(MaybeUninit::new((UnsafeCell::new(1), 1)));
		let older = manual.replace(ManuallyDrop::new(UnsafeCell::new(2)));
		std::ptr::write(&raw mut pair, (MaybeUninit::new(UnsafeCell::new(3)), 3));
		cell.set(ManuallyDrop::new(UnsafeCell::new(4)));
		*manual.get() = ManuallyDrop::new(UnsafeCell::new(5));
		old.assume_init().1 ^ ManuallyDrop::into_inner(older).into_inner() ^ pair.1
	}
}

/// `Cell::swap`s of arrays, whose cells may partially overlap.
pub fn cell_of_arrays() -> u16 {
	let (x, y) = (
		std::cell::Cell::new([[1u16, 2]; 2]),
		std::cell::Cell::new([[3u16, 4]; 2]),
	);
	x.swap(&y);
	let (v, w) = (std::cell::Cell::new([5u32, 6]), std::cell::Cell::new([7u32, 8]));
	v.swap(&w);
	x.into_inner()[0][0] ^ v.into_inner()[0] as u16
}

/// Reborrows through a pointer to an array a `MaybeUninit` holds, by `as_mut`, `as_ref`,
/// and `&*p`, shared and mutable, whose reference is the array, which part of an array has
/// none of its own.
pub fn uninit_array_lent(p: *mut std::mem::MaybeUninit<[u32; 2]>) -> u32 {
	let first = unsafe { p.as_mut() }.map_or(0, |m| unsafe { m.assume_init_mut()[0] });
	let second = unsafe { std::ptr::NonNull::new(p).unwrap().as_mut().assume_init_mut()[1] };
	let third = unsafe { p.as_ref() }.map_or(0, |m| unsafe { m.assume_init_ref()[0] });
	let fourth = unsafe { std::ptr::NonNull::new(p).unwrap().as_ref().assume_init_ref()[1] };
	let fifth = unsafe { (&*p).assume_init_ref()[0] };
	let sixth = unsafe { (&mut *p).assume_init_mut()[1] };
	first ^ second ^ third ^ fourth ^ fifth ^ sixth
}

/// `Box::from_raw` of arrays `as_chunks_mut` split from one flat array, which a box of
/// arrays of their own can't share, and of an array of no bytes whose elements are more than
/// structs and tuples, which the translator can't make anew.
pub fn arrays_boxed() -> u32 {
	let flat = Box::into_raw(Box::new([1u32, 2, 3, 4]));
	let (runs, _) = unsafe { (*flat).as_chunks_mut::<2>() };
	let whole: *mut [[u32; 2]; 2] = runs.first_chunk_mut().unwrap();
	let back = unsafe { Box::from_raw(whole) };
	let marks = Box::into_raw(Box::new([Only::One; 2]));
	let marks = unsafe { Box::from_raw(marks) };
	back[0][0] + marks.len() as u32
}

/// An enum of one variant, a value of no bytes.
#[derive(Clone, Copy)]
enum Only {
	One,
}

/// A `get_mut` and an `as_mut_unchecked` of a cell of a pinned `Box`, whose old value
/// `mem::replace` would give back as a copy.
pub fn pinned_lent(c: &mut std::cell::UnsafeCell<Pin<Box<(u8, u8)>>>) -> u8 {
	let old = std::mem::replace(c.get_mut(), Box::pin((1, 2)));
	let older = std::mem::replace(unsafe { c.as_mut_unchecked() }, Box::pin((3, 4)));
	old.0 ^ older.0
}

/// Casts between pointers to bytes and to values outside memory from `alloc`, which read
/// the bytes of one type as another: of an array's bytes to `u32`s, of memory from
/// `alloc` to the crate's struct, which JS keeps as an object, and of a pointer to values
/// to one to their bytes, read rather than freed; and a byte of memory from `alloc` written
/// by `ptr::write`.
pub fn bytes_as_values(bytes: [u8; 4]) -> u32 {
	let layout = std::alloc::Layout::new::<Halves>();
	unsafe {
		std::ptr::write(std::alloc::alloc(layout), 2);
		let words = bytes.as_ptr() as *const u32;
		let halves = std::alloc::alloc(layout) as *mut Halves;
		let n = 7u32;
		let byte = *((&raw const n) as *const u8);
		*words + (*halves).low + u32::from(byte)
	}
}

/// Memory from `alloc` cast to pointers to two types, which reads one's bytes as the
/// other's, also once `realloc` grows it.
pub fn alloc_retyped() -> i32 {
	let layout = std::alloc::Layout::new::<u32>();
	unsafe {
		let raw = std::alloc::alloc(layout);
		raw.cast::<u32>().write(u32::MAX);
		let n = (raw as *mut i32).read();
		let grown = std::alloc::realloc(raw, layout, 8);
		let m = grown.cast::<i32>().read();
		let words = grown.cast::<u32>();
		let again = std::alloc::realloc(words.cast::<u8>(), std::alloc::Layout::array::<u32>(2).unwrap(), 12);
		let k = again.cast::<i32>().read();
		std::alloc::dealloc(again, std::alloc::Layout::array::<u32>(3).unwrap());
		n + m + k
	}
}

/// Memory from `alloc_zeroed`, whose zeroed bytes JS can't type.
pub fn zeroed() -> u32 {
	unsafe { *(std::alloc::alloc_zeroed(std::alloc::Layout::new::<u32>()) as *mut u32) }
}

/// Memory from `alloc` whose byte pointer leaves the body untyped: returned, kept in a
/// field, or given to a function of the crate's own, which JS can't read as bytes.
pub fn alloc_leaked(n: u8) -> *mut u8 {
	let layout = std::alloc::Layout::new::<u32>();
	unsafe {
		let held = Held {
			bytes: std::alloc::alloc(layout),
		};
		fill(std::alloc::alloc(layout), n);
		std::alloc::dealloc(held.bytes, layout);
		std::alloc::alloc(layout)
	}
}

pub struct Held {
	bytes: *mut u8,
}

unsafe fn fill(p: *mut u8, n: u8) {
	unsafe { *p = n };
}

/// `realloc` of memory a pointer to `u32`s points at, whose result is read as `i32`s.
pub fn realloc_retyped(words: *mut u32) -> i32 {
	let layout = std::alloc::Layout::new::<u32>();
	unsafe { std::alloc::realloc(words as *mut u8, layout, 8).cast::<i32>().read() }
}
