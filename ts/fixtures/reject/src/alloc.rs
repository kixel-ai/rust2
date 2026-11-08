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

/// A pointer to a `Vec`, whose JS array is its heap, not where Rust keeps the `Vec`.
pub fn vec_pointer(v: &mut Vec<u8>) -> usize {
	let p = &raw const *v;
	unsafe { (*p).len() }
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
