//! Raw pointers to objects that a write of the whole variable, part, or element replaces, and to arrays JS keeps apart.

pub struct Span2 {
	pub start: (u32, u32),
	pub len: u32,
}

/// A pointer to a part kept as an object, which a write of the whole part elsewhere
/// replaces with another object.
pub fn part_replaced(n: u32) -> u32 {
	let mut s = Span2 { start: (n, n), len: n };
	let p = &raw mut s.start;
	s.start = (0, 0);
	unsafe { (*p).0 + s.len }
}

/// A pointer to a variable's object, which an assignment of the whole variable replaces.
pub fn variable_reassigned(n: u32) -> u32 {
	let mut pair = (n, n);
	let p = &raw mut pair;
	pair = (0, 0);
	unsafe { (*p).0 + pair.1 }
}

/// A pointer to a variable's object, which `mem::replace` of the whole variable replaces.
pub fn variable_replaced(n: u32) -> u32 {
	let mut pair = (n, n);
	let p = &raw mut pair;
	let old = std::mem::replace(&mut pair, (0, 0));
	unsafe { (*p).0 + old.1 }
}

/// A pointer to a variable's object, which `mem::swap` replaces.
pub fn variable_swapped(n: u32) -> u32 {
	let mut a = (n, n);
	let mut b = (0, 0);
	let p = &raw mut a;
	std::mem::swap(&mut a, &mut b);
	unsafe { (*p).0 + b.1 }
}

/// A pointer to a variable's object, which a whole write through a `&mut` to it replaces.
pub fn assigned_through(n: u32) -> u32 {
	let mut pair = (n, n);
	let p = &raw mut pair;
	let r = &mut pair;
	*r = (0, 0);
	unsafe { (*p).0 }
}

/// A pointer `NonNull::from` makes of a `&mut` to a variable, which an assignment replaces.
pub fn nonnull_reassigned(n: u32) -> u32 {
	let mut pair = (n, n);
	let p = std::ptr::NonNull::from(&mut pair);
	pair = (0, 0);
	unsafe { p.as_ref().0 + pair.1 }
}

/// A pointer cast from a `&mut` to a variable, which an assignment replaces.
pub fn cast_reassigned(n: u32) -> u32 {
	let mut pair = (n, n);
	let p = &mut pair as *mut (u32, u32);
	pair = (0, 0);
	unsafe { (*p).0 + pair.1 }
}

pub struct Window {
	pub range: (u32, u32),
}

/// A pointer to a part kept as an object, which `mem::replace` of the part elsewhere replaces.
pub fn range_of(w: &mut Window) -> *mut (u32, u32) {
	&raw mut w.range
}

pub fn range_replaced(w: &mut Window) -> (u32, u32) {
	std::mem::replace(&mut w.range, (0, 0))
}

/// A pointer to an element kept as an object.
pub fn element_pointer(n: u32) -> u32 {
	let mut pairs = [(n, n); 2];
	let p = &raw mut pairs[1];
	unsafe { (*p).0 }
}

/// The pointer `UnsafeCell::get` gives to the object its `rs.Cell` holds, which a whole
/// write of the cell elsewhere replaces with another object.
pub fn cell_replaced(n: u32) -> u32 {
	let mut cell = std::cell::UnsafeCell::new((n, n));
	let p = cell.get();
	let old = std::mem::replace(&mut cell, std::cell::UnsafeCell::new((0, 0)));
	unsafe { (*p).0 + old.into_inner().1 }
}

/// The pointer `UnsafeCell::get` gives to a number, the `rs.Cell` itself, which an
/// assignment of the whole variable replaces.
pub fn cell_reassigned(n: u32) -> u32 {
	let mut cell = std::cell::UnsafeCell::new(n);
	let p = cell.get();
	cell = std::cell::UnsafeCell::new(0);
	unsafe { *p + cell.into_inner() }
}

pub struct Hits {
	pub count: std::cell::UnsafeCell<u32>,
}

/// The pointer `UnsafeCell::get` gives to a number, the `rs.Cell` itself, which
/// `mem::take` of the whole variable replaces.
pub fn cell_taken(n: u32) -> u32 {
	let mut cell = std::cell::UnsafeCell::new(n);
	let p = cell.get();
	let old = std::mem::take(&mut cell);
	unsafe { *p + old.into_inner() }
}

/// The pointer `UnsafeCell::get` gives to a number, the `rs.Cell` in a struct, which a
/// write of the whole field replaces.
pub fn cell_field_replaced(n: u32) -> u32 {
	let mut hits = Hits {
		count: std::cell::UnsafeCell::new(n),
	};
	let p = hits.count.get();
	hits.count = std::cell::UnsafeCell::new(0);
	unsafe { *p + *hits.count.get_mut() }
}

pub struct Plain {
	pub at: (u32, u32),
}

fn put<T>(slot: &mut T, value: T) {
	*slot = value;
}

/// A pointer to a part kept as an object, which a generic body reached at its struct
/// replaces with another object.
pub fn put_replaced(n: u32) -> u32 {
	let mut plain = Plain { at: (n, n) };
	let p = &raw mut plain.at;
	put(&mut plain, Plain { at: (0, 0) });
	unsafe { (*p).0 }
}

#[derive(Default)]
pub struct Track {
	pub at: (u32, u32),
}

pub struct Bin<T>(std::cell::UnsafeCell<T>);

pub trait Reset {
	fn reset(&self);
}

impl<T: Default> Reset for Bin<T> {
	fn reset(&self) {
		unsafe { *self.0.get() = T::default() };
	}
}

/// A pointer to a part kept as an object, which a generic body reached only through a trait
/// object replaces with another object.
pub fn object_replaced(n: u32) -> u32 {
	let bin = Bin(std::cell::UnsafeCell::new(Track { at: (n, n) }));
	let p = unsafe { &raw mut (*bin.0.get()).at };
	let object: &dyn Reset = &bin;
	object.reset();
	unsafe { (*p).0 }
}

#[derive(Default)]
pub struct Lane {
	pub at: (u32, u32),
}

fn resetter<T: Default>(slot: *mut T) -> impl FnOnce() {
	move || unsafe { *slot = T::default() }
}

/// A pointer to a part kept as an object, which a generic closure reached at its struct
/// replaces with another object.
pub fn closure_replaced(n: u32) -> u32 {
	let mut lane = Lane { at: (n, n) };
	let whole = &raw mut lane;
	let p = unsafe { &raw mut (*whole).at };
	resetter(whole)();
	unsafe { (*p).0 }
}

pub struct Gap {
	pub range: (u32, u32),
	pub slot: std::mem::MaybeUninit<(u32, u32)>,
}

fn reset_gap(gap: &mut Gap) {
	gap.range = (0, 0);
	gap.slot = std::mem::MaybeUninit::new((0, 0));
}

/// The pointers `NonNull::from` makes of a `&mut` and of a `&` to a part kept as an object,
/// and `into` of a `&mut`, which a write of the whole part elsewhere replaces with another
/// object.
pub fn made_part_replaced(n: u32) -> u32 {
	let mut gap = Gap {
		range: (n, n),
		slot: std::mem::MaybeUninit::new((n, n)),
	};
	let p = std::ptr::NonNull::from(&mut gap.range);
	let q = std::ptr::NonNull::from(&gap.range);
	let r: std::ptr::NonNull<(u32, u32)> = (&mut gap.range).into();
	reset_gap(&mut gap);
	unsafe { p.as_ref().0 + q.as_ref().1 + r.as_ref().0 }
}

/// The pointers `MaybeUninit::as_ptr` and `as_mut_ptr` make to a part kept as an object,
/// which a write of the whole part elsewhere replaces with another object.
pub fn uninit_part_replaced(n: u32) -> u32 {
	let mut gap = Gap {
		range: (n, n),
		slot: std::mem::MaybeUninit::new((n, n)),
	};
	let p = gap.slot.as_ptr();
	let q = gap.slot.as_mut_ptr();
	reset_gap(&mut gap);
	unsafe { (*p).0 + (*q).1 }
}

/// A struct holding a pointer.
pub struct Aimed {
	pub at: *mut (u32, u32),
}

/// A pointer to a variable's object, copied into a struct read after an assignment of
/// the whole variable replaces the object.
pub fn pointer_in_struct(n: u32) -> u32 {
	let mut pair = (n, n);
	let aimed = Aimed { at: &raw mut pair };
	pair = (0, 0);
	unsafe { (*aimed.at).0 + pair.1 }
}

fn handed_back<T>(p: *mut T) -> *mut T {
	p
}

/// A pointer to a variable's object, which a call returns, read after an assignment of
/// the whole variable replaces the object.
pub fn pointer_returned(n: u32) -> u32 {
	let mut pair = (n, n);
	let p = handed_back(&raw mut pair);
	pair = (0, 0);
	unsafe { (*p).0 + pair.1 }
}

/// A pointer to a variable's object, kept in a `Vec` and read after an assignment of the
/// whole variable replaces the object.
pub fn pointer_pushed(n: u32) -> u32 {
	let mut pair = (n, n);
	let mut kept = Vec::new();
	kept.push(&raw mut pair);
	pair = (0, 0);
	unsafe { (*kept[0]).0 + pair.1 }
}

/// A pointer to a variable's object, which a closure captures and reads after an
/// assignment of the whole variable replaces the object.
pub fn pointer_captured(n: u32) -> u32 {
	let mut pair = (n, n);
	let p = &raw mut pair;
	let read = move || unsafe { (*p).0 };
	pair = (0, 0);
	read() + pair.1
}

/// A pointer an earlier iteration made to a variable's object, read after the next
/// iteration's assignment of the whole variable replaces the object.
pub fn pointer_kept_across(data: &[u32]) -> u32 {
	let mut pair = (0, 0);
	let mut last = &raw mut pair;
	let mut sum = 0;
	for &n in data {
		pair = (n, n);
		sum += unsafe { (*last).0 };
		last = &raw mut pair;
	}
	sum
}

/// Pointer operations on arrays of bytes that read the address or other bytes than the
/// array's: copying, offsetting, hashing, and ordering, beside pointers to their first
/// bytes.
pub fn byte_pointer_ops(mut a: [u8; 4], b: [u8; 4]) -> u64 {
	use std::hash::{Hash, Hasher};
	let p: *mut [u8; 4] = &mut a;
	let q: *const [u8; 4] = &b;
	unsafe {
		std::ptr::copy(q, p, 1);
		std::ptr::copy_nonoverlapping(q, p, 1);
		let same = p < q.cast_mut();
		let byte = *p.cast::<u8>() + *(q as *const u8) + (*p.add(0))[0] + (*q.offset(0))[1];
		let slice: *const [u8] = q;
		let mut h = std::collections::hash_map::DefaultHasher::new();
		q.hash(&mut h);
		h.finish() + u64::from(same) + u64::from(byte) + slice.len() as u64
	}
}

/// Pointers to arrays of bytes that are parts of an array kept apart from it: a slice
/// pattern's rest and an array's element.
pub fn byte_pointer_parts(a: [u8; 4], b: [[u8; 2]; 3]) -> bool {
	let [_, ref x @ ..] = a;
	let [_, ref y @ ..] = a;
	std::ptr::eq(x, y) || std::ptr::eq(&b[0], &b[1])
}

/// Pointers to arrays of one, which JS makes of its own for a value, compared.
pub fn array_of_one(n: u32) -> bool {
	std::ptr::eq(std::array::from_ref(&n), std::array::from_ref(&n))
}

/// Pointers to an array of bytes in a variable assigned again while they point there,
/// made of a reference and as a raw borrow.
pub fn byte_pointer_rebound(n: u8) -> u8 {
	let mut a = [n; 4];
	let p: *const [u8; 4] = &a;
	let q = &raw const a;
	a = [n ^ 1; 4];
	unsafe { (*p)[0] + (*q)[1] + a[2] }
}

/// Pointers into arrays whose elements JS keeps as objects: a pointer to one is the
/// object, which knows no array it is in.
pub fn object_elements(v: &mut Vec<(u8, u8)>, a: [(u8, u8); 2]) -> u8 {
	let p = v.as_mut_ptr();
	unsafe {
		let q = p.add(1);
		std::ptr::copy(p, q, 1);
		let s = std::slice::from_raw_parts(p, 2);
		let first = (&raw const a).cast::<(u8, u8)>();
		(*q).0 + s[1].1 + (*first).0 + q.offset_from(p) as u8
	}
}

/// Pointers to values of no bytes, which std's `offset_from` panics on, and a slice of
/// them from one, which has no array to be a view of.
pub fn unit_elements(p: *const ()) -> usize {
	unsafe { p.offset_from(p) as usize + std::slice::from_raw_parts(p, 3).len() }
}

/// A pointer to an element cast to one to an array of elements from it.
pub fn element_to_array(v: &[u32]) -> u32 {
	unsafe { (*(v.as_ptr() as *const [u32; 2]))[1] }
}

/// A struct holding an array.
pub struct Lanes {
	/// The array a pointer points into.
	pub lanes: [u32; 3],
}

/// Pointers to elements of arrays that a later assignment gives another JS array, which a
/// pointer into the first would not see: a variable's, and a struct's field.
pub fn elements_replaced(n: u32, l: &mut Lanes) -> u32 {
	let mut a = [n; 3];
	let p = &raw mut a[1];
	a = [0; 3];
	let q = &raw mut l.lanes[2];
	l.lanes = [1; 3];
	unsafe { *p + *q + a[0] }
}

#[repr(C)]
pub struct Halves {
	pub low: u32,
	pub high: u32,
}

/// Two arrays in one allocation, which JS keeps apart.
#[repr(C)]
pub struct Rows {
	/// The first.
	pub top: [u32; 2],
	/// The second.
	pub bottom: [u32; 2],
}

/// `offset_from` of pointers the body shows JS keeps in different arrays: two made apart
/// of one variable, two of a struct's fields, into two of its array fields, into two rows
/// of a nested array, one into a shared subslice, a copy, against one into what it was
/// taken of, and one into a shared slice of a byte, also a copy, against the byte's.
pub fn pointers_apart(n: u32, h: Halves, r: Rows, byte: u8) -> isize {
	let a = &raw const n;
	let b = &raw const n;
	let grid = [[n; 2]; 2];
	let lanes = [n; 4];
	let part = &lanes[1..3];
	let at = &raw const byte;
	unsafe {
		let bytes = std::slice::from_raw_parts(at, 1);
		a.offset_from(b)
			+ (&raw const h.high).offset_from(&raw const h.low)
			+ r.bottom.as_ptr().offset_from(r.top.as_ptr())
			+ grid[1].as_ptr().offset_from(grid[0].as_ptr())
			+ part.as_ptr().offset_from(lanes.as_ptr())
			+ bytes.as_ptr().offset_from(at)
	}
}

/// `offset_from` between pointers into a `str`'s bytes, which JS encodes anew at each
/// `as_ptr`: a substring's against its text's, and two of one text.
pub fn text_apart(s: &str) -> isize {
	unsafe { s[1..].as_ptr().offset_from(s.as_ptr()) + s.as_ptr().offset_from(s.as_ptr()) }
}

/// A pointer into a substring's bytes moved back before them, into its text's, which JS
/// encodes apart, and read.
pub fn text_before(s: &str) -> u8 {
	unsafe { *s[1..].as_ptr().sub(1) }
}

/// Pointers, slices, and copies the body shows reach out of an array of one or a copied
/// subslice, where Rust's reach the rest of what holds them.
pub fn reached_out(h: Halves, lanes: [u32; 4]) -> u32 {
	let part = &lanes[1..3];
	let mut to = [0u32; 3];
	unsafe {
		std::ptr::copy_nonoverlapping(part.as_ptr(), to.as_mut_ptr(), 3);
		let tail = lanes.get_unchecked(2..);
		*part.as_ptr().sub(1)
			+ std::slice::from_raw_parts(&raw const h.low, 2)[1]
			+ *(&raw const h.high).sub(1)
			+ *tail.as_ptr().offset(-2)
			+ to[0]
	}
}

/// `write_bytes` through a pointer to what JS keeps as a value rather than its bytes, and
/// through one to a byte in no array, past it.
pub fn written_bytes(mut lanes: [u32; 2], mut b: u8) -> u32 {
	unsafe {
		std::ptr::write_bytes(lanes.as_mut_ptr(), 0, 2);
		(&raw mut b).write_bytes(0xff, 2);
	}
	lanes[0] + u32::from(b)
}

/// A byte in a field, which no array of bytes holds.
pub struct Tagged {
	pub tag: u8,
}

/// A slice of one byte in a field, returned: its copy would be written back after no
/// statement of the function.
pub fn byte_slice_returned(t: &mut Tagged) -> &mut [u8] {
	std::slice::from_mut(&mut t.tag)
}

/// A slice of one byte in a field, kept where a parameter points: no variable of the
/// function holds it.
pub fn byte_slice_stored<'a>(t: &'a mut Tagged, out: &mut Vec<&'a mut [u8]>) {
	out.push(std::slice::from_mut(&mut t.tag));
}

/// Slices of one byte in each of the fields, kept together: the one variable holding
/// the copy the call makes can't keep them apart.
pub fn byte_slices_kept(ts: &mut [Tagged]) {
	let mut kept = Vec::new();
	for t in ts.iter_mut() {
		kept.push(std::slice::from_mut(&mut t.tag));
	}
	for s in kept {
		s[0] = 1;
	}
}

/// Whether the byte `s` holds is zero, as a name that borrows none of it.
fn zero_name(s: &mut [u8]) -> &'static str {
	if s[0] == 0 { "zero" } else { "other" }
}

/// What a call given a slice of one byte in a field returns, kept where a parameter
/// points: it could hold the slice whatever its lifetime says, and no variable of the
/// function holds it.
pub fn byte_result_stored(t: &mut Tagged, out: &mut Vec<&'static str>) {
	out.push(zero_name(std::slice::from_mut(&mut t.tag)));
}

/// What calls given slices of one byte in each of the fields return, kept together:
/// each could hold its slice whatever its lifetime says.
pub fn byte_results_kept(ts: &mut [Tagged]) -> usize {
	let mut names = Vec::new();
	for t in ts.iter_mut() {
		names.push(zero_name(std::slice::from_mut(&mut t.tag)));
	}
	names.len()
}

/// A pointer to a variable's object, which `clone_from` of the whole variable replaces.
pub fn variable_cloned_from(n: u32) -> u32 {
	let mut pair = (n, n);
	let p = &raw mut pair;
	pair.clone_from(&(0, 0));
	unsafe { (*p).0 + pair.1 }
}

/// A pointer to a variable's object, which `clone_into` the whole variable replaces.
pub fn variable_cloned_into(n: u32) -> u32 {
	let mut pair = (n, n);
	let p = &raw mut pair;
	(0, 0).clone_into(&mut pair);
	unsafe { (*p).0 + pair.1 }
}

/// A pointer to a variable's object, which a cell of the whole variable replaces.
pub fn variable_celled(n: u32) -> u32 {
	let mut pair = (n, n);
	let p = &raw mut pair;
	std::cell::Cell::from_mut(&mut pair).set((0, 0));
	unsafe { (*p).0 + pair.1 }
}

/// A pointer to a value JS copies, cast to one to `()`, which would be a new identity each
/// borrow.
pub fn erased_place(n: u32) -> bool {
	let p = &raw const n;
	p as *const () == p.cast()
}

/// A pointer to a byte array, cast to one to `()`, which would be a new view each borrow.
pub fn erased_bytes(b: &mut [u8; 4]) -> *mut () {
	(b as *mut [u8; 4]).cast()
}

/// A pointer to a trait object, cast to one to `()`, which loses what it points at's size.
pub fn erased_dyn(d: &dyn std::fmt::Debug) -> *const () {
	d as *const dyn std::fmt::Debug as *const ()
}

/// A struct whose start is another struct.
#[repr(C)]
pub struct Outer {
	head: Inner,
	tail: u8,
}

/// The start of an `Outer`.
pub struct Inner(u16);

/// A pointer to a struct cast to one to `()`, whose address the object of its first field
/// shares, which JS keeps apart.
pub fn erased_start(outer: &Outer) -> bool {
	let whole = &raw const *outer;
	whole as *const () == (&raw const outer.head).cast() && outer.tail == outer.head.0 as u8
}

/// A pointer to a value of no bytes, cast to one to `()`.
pub fn erased_empty() -> *const () {
	let unit = ((), ());
	(&raw const unit).cast()
}

/// Pointers to values JS copies that no `Arc` holds, cast to ones to `()`, which would be
/// a new identity each borrow, beside one to an `Arc`'s value, which is its one place.
pub fn erased_borrows(n: &u32, c: &std::cell::UnsafeCell<u32>, a: &std::sync::Arc<u32>) -> bool {
	let arc = std::ptr::from_ref(&**a).cast::<()>();
	std::ptr::from_ref(n) as *const () == arc && c.get().cast_const().cast() == arc
}

/// A pointer to a value JS copies from elsewhere, cast to one to `()`, which only an `Arc`'s
/// raw pointer would keep the one place of.
pub fn erased_unshown(p: *const u32) -> *const () {
	p.cast()
}

/// A pointer to a box, cast to one to `()`, which JS keeps as what it holds.
pub fn erased_box(b: Box<(u8, u8)>) -> bool {
	let p = &raw const b;
	p as *const () == (&raw const *b).cast()
}

/// An `Arc`'s raw pointer to a slice, which `from_raw` would take back to no `Arc`.
pub fn arc_raw_slice(a: std::sync::Arc<[u32]>) -> *const [u32] {
	std::sync::Arc::into_raw(a)
}

/// An `Rc` taken back from a raw pointer to bytes, which no `Rc` gave.
pub fn rc_raw_bytes(p: *const [u8; 4]) -> u8 {
	unsafe { std::rc::Rc::from_raw(p)[0] }
}

/// An `Arc` taken back from a pointer remade from a reference to a number no `Arc` the body
/// shows holds, which JS copies.
pub fn arc_from_borrow(n: &u32) -> u32 {
	unsafe { *std::sync::Arc::from_raw(std::ptr::from_ref(n)) }
}

/// The greater of two pointers to numbers, which orders by their addresses.
pub fn pointers_ordered(a: &u32, b: &u32) -> bool {
	let (p, q) = (std::ptr::from_ref(a), std::ptr::from_ref(b));
	std::cmp::max(p, q) == p
}

/// A mutable reborrow of a `MaybeUninit` of arrays of arrays through a raw pointer, whose
/// arrays `as_chunks_mut` may have split from one slice with no JS array of their own.
pub fn uninit_rows_mut(rows: &mut [[u8; 2]]) -> u8 {
	let Some(first) = rows.first_chunk_mut::<2>() else {
		return 0;
	};
	let p = std::ptr::from_mut(first).cast::<std::mem::MaybeUninit<[[u8; 2]; 2]>>();
	let wrapped = unsafe { &mut *p };
	let rows = unsafe { wrapped.assume_init_mut() };
	rows[0][0] = 99;
	rows[1][1]
}

/// A mutable reborrow of a `MaybeUninit` of an array through a raw pointer into part of a
/// slice, which no JS array holds alone.
pub fn uninit_part_mut(words: &mut [u32]) -> u32 {
	if words.len() < 3 {
		return 0;
	}
	let p = (&mut words[1..] as *mut [u32]).cast::<std::mem::MaybeUninit<[u32; 2]>>();
	let wrapped = unsafe { &mut *p };
	let pair = unsafe { wrapped.assume_init_mut() };
	pair[0] = 99;
	pair[1]
}
