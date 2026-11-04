//! Pointers into parts of objects that a whole write of the part replaces.

use crate::rebound::part_cell;

/// Structs JS keeps as objects, each holding an `rt.Cell`, one for each kind of whole
/// write of a value that holds one, which gives it another cell, while a pointer to the
/// cell, taken through a reference to the struct, may still be read.
pub struct InArray(pub std::cell::UnsafeCell<u16>);

pub struct InTuple(pub std::cell::UnsafeCell<u16>);

pub struct InNested(pub std::cell::UnsafeCell<u16>);

pub struct InPayload(pub std::cell::UnsafeCell<u16>);

pub struct InField(pub std::cell::UnsafeCell<u16>);

pub struct InGeneric(pub std::cell::UnsafeCell<u16>);

pub struct InWritten(pub std::cell::UnsafeCell<u16>);

pub struct Mid<T>(pub u16, pub T);

pub struct Outer<T>(pub Mid<T>);

fn array_part(held: &InArray) -> *mut u16 {
	held.0.get()
}

fn tuple_part(held: &InTuple) -> *mut u16 {
	held.0.get()
}

fn nested_part(held: &InNested) -> *mut u16 {
	held.0.get()
}

fn payload_part(held: &InPayload) -> *mut u16 {
	held.0.get()
}

fn field_part(held: &InField) -> *mut u16 {
	held.0.get()
}

fn generic_part(held: &InGeneric) -> *mut u16 {
	held.0.get()
}

fn written_part(held: &InWritten) -> *mut u16 {
	held.0.get()
}

pub fn cell(n: u16) -> std::cell::UnsafeCell<u16> {
	std::cell::UnsafeCell::new(n)
}

/// A whole array written, then a pointer into its element read.
pub fn array_part_replaced(n: u16) -> u16 {
	let mut whole = [InArray(cell(n)), InArray(cell(n))];
	let p = &raw mut whole;
	let part = array_part(unsafe { &(*p)[1] });
	unsafe { *p = [InArray(cell(0)), InArray(cell(0))] };
	unsafe { *part }
}

/// A whole tuple written, then a pointer into its element read.
pub fn tuple_part_replaced(n: u16) -> u16 {
	let mut whole = (n, InTuple(cell(n)));
	let p = &raw mut whole;
	let part = tuple_part(unsafe { &(*p).1 });
	unsafe { *p = (0, InTuple(cell(0))) };
	unsafe { *part }
}

/// A whole struct written, then a pointer into a struct its field holds read.
pub fn nested_part_replaced(n: u16) -> u16 {
	let mut whole = Outer(Mid(n, InNested(cell(n))));
	let p = &raw mut whole;
	let part = nested_part(unsafe { &(*p).0.1 });
	unsafe { *p = Outer(Mid(0, InNested(cell(0)))) };
	unsafe { *part }
}

/// A whole `Option` written, then a pointer into its payload read.
pub fn payload_part_replaced(n: u16) -> u16 {
	let mut whole = Some(InPayload(cell(n)));
	let p = &raw mut whole;
	let Some(held) = (unsafe { &*p }) else {
		return 0;
	};
	let part = payload_part(held);
	unsafe { *p = Some(InPayload(cell(0))) };
	unsafe { *part }
}

/// A whole field written, then a pointer into a struct it holds read.
pub fn field_part_replaced(n: u16) -> u16 {
	let mut whole = Outer(Mid(n, InField(cell(n))));
	let p = &raw mut whole;
	let part = field_part(unsafe { &(*p).0.1 });
	unsafe { (*p).0 = Mid(0, InField(cell(0))) };
	unsafe { *part }
}

fn set<T>(slot: *mut T, value: T) {
	unsafe { *slot = value }
}

/// A whole struct written by a generic body, then a pointer into a struct it holds read.
pub fn generic_part_replaced(n: u16) -> u16 {
	let mut whole = Outer(Mid(n, InGeneric(cell(n))));
	let p = &raw mut whole;
	let part = generic_part(unsafe { &(*p).0.1 });
	set(p, Outer(Mid(0, InGeneric(cell(0)))));
	unsafe { *part }
}

/// A whole array written by std, then a pointer into its element read.
pub fn written_part_replaced(n: u16) -> u16 {
	let mut whole = [InWritten(cell(n))];
	let p = &raw mut whole;
	let part = written_part(unsafe { &(*p)[0] });
	unsafe { p.write([InWritten(cell(0))]) };
	unsafe { *part }
}

pub struct Replaced {
	pub cell: std::cell::UnsafeCell<u16>,
}

fn replace_part(r: &mut Replaced) {
	drop(std::mem::replace(&mut r.cell, std::cell::UnsafeCell::new(0)));
}

/// A pointer a callee takes through a reference to a cell in a struct, which another
/// function's `mem::replace` gives another object.
pub fn part_mem_replaced(n: u16) -> u16 {
	let mut r = Replaced {
		cell: std::cell::UnsafeCell::new(n),
	};
	let p = part_cell(&r.cell);
	replace_part(&mut r);
	unsafe { *p }
}

pub struct Swapped {
	pub cell: std::cell::UnsafeCell<u16>,
}

fn swap_part(s: &mut Swapped) {
	std::mem::swap(&mut s.cell, &mut std::cell::UnsafeCell::new(0));
}

/// As `part_replaced`, by another function's `mem::swap`.
pub fn part_swapped(n: u16) -> u16 {
	let mut s = Swapped {
		cell: std::cell::UnsafeCell::new(n),
	};
	let p = part_cell(&s.cell);
	swap_part(&mut s);
	unsafe { *p }
}

pub struct Taken {
	pub cell: std::cell::UnsafeCell<u16>,
}

fn take_part(t: &mut Taken) {
	drop(std::mem::take(&mut t.cell));
}

/// As `part_replaced`, by another function's `mem::take`.
pub fn part_taken(n: u16) -> u16 {
	let mut t = Taken {
		cell: std::cell::UnsafeCell::new(n),
	};
	let p = part_cell(&t.cell);
	take_part(&mut t);
	unsafe { *p }
}

pub struct Assigned {
	pub cell: std::cell::UnsafeCell<u16>,
}

fn assign_part(a: &mut Assigned) {
	a.cell = std::cell::UnsafeCell::new(0);
}

/// As `part_replaced`, by another function's assignment of the part through a `&mut` to
/// its holder.
pub fn part_assigned(n: u16) -> u16 {
	let mut a = Assigned {
		cell: std::cell::UnsafeCell::new(n),
	};
	let p = part_cell(&a.cell);
	assign_part(&mut a);
	unsafe { *p }
}

pub struct Deeper {
	pub cell: std::cell::UnsafeCell<u16>,
}

fn deeper_cell(c: &std::cell::UnsafeCell<u16>) -> *mut u16 {
	part_cell(c)
}

fn assign_deeper(d: &mut Deeper) {
	d.cell = std::cell::UnsafeCell::new(0);
}

fn reset_deeper(d: &mut Deeper) {
	assign_deeper(d);
}

/// As `part_assigned`, with the pointer taken and the part assigned each two calls deep.
pub fn part_replaced_deep(n: u16) -> u16 {
	let mut d = Deeper {
		cell: std::cell::UnsafeCell::new(n),
	};
	let p = deeper_cell(&d.cell);
	reset_deeper(&mut d);
	unsafe { *p }
}

pub trait Clear {
	fn clear(&mut self);
}

pub struct Dynamic {
	pub cell: std::cell::UnsafeCell<u16>,
}

impl Clear for Dynamic {
	fn clear(&mut self) {
		self.cell = std::cell::UnsafeCell::new(0);
	}
}

/// As `part_assigned`, through a trait object's method.
pub fn part_replaced_dyn(n: u16) -> u16 {
	let mut d = Dynamic {
		cell: std::cell::UnsafeCell::new(n),
	};
	let p = part_cell(&d.cell);
	let clear: &mut dyn Clear = &mut d;
	clear.clear();
	unsafe { *p }
}

pub struct Closed {
	pub cell: std::cell::UnsafeCell<u16>,
}

/// As `part_assigned`, by a closure, which also takes the pointer.
pub fn part_replaced_closure(n: u16) -> u16 {
	let mut e = Closed {
		cell: std::cell::UnsafeCell::new(n),
	};
	let take = |c: &std::cell::UnsafeCell<u16>| c.get();
	let p = take(&e.cell);
	let reset = |e: &mut Closed| e.cell = std::cell::UnsafeCell::new(0);
	reset(&mut e);
	unsafe { *p }
}

fn replace_element(cells: &mut [std::cell::UnsafeCell<u16>; 2]) {
	cells[1] = std::cell::UnsafeCell::new(0);
}

/// As `part_assigned`, with the cell an array's element, which another function assigns.
pub fn element_replaced(n: u16) -> u16 {
	let mut cells = [std::cell::UnsafeCell::new(n), std::cell::UnsafeCell::new(n)];
	let p = part_cell(&cells[1]);
	replace_element(&mut cells);
	unsafe { *p }
}
