//! Memory from `alloc` and raw pointers read as other types' bytes or as addresses.

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

/// A pointer from a reference the body is given, which JS keeps in no array, read through
/// past it by a constant wrapping offset and one past it, and viewed as a slice of a
/// length the body doesn't fix: each reaches the neighbour Tree Borrows allows it to.
pub fn beside_reference(r: &u32, n: usize) -> u32 {
	let p = std::ptr::from_ref(r);
	unsafe { *p.wrapping_add(1) + *p.add(1) + std::slice::from_raw_parts(p, n).iter().sum::<u32>() }
}

/// A pointer from a reference the body is given, which JS keeps in no array, moved past
/// it.
pub fn past_reference(r: &u32) -> u32 {
	unsafe { *(r as *const u32).add(2) }
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
