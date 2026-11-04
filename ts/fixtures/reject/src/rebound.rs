//! Pointers a callee makes, which the caller's write of the whole variable replaces.

fn cell_of(c: &std::cell::UnsafeCell<u64>) -> *mut u64 {
	c.get()
}

/// The pointer `UnsafeCell::get` gives a callee, the `rs.Cell` itself, which the caller's
/// assignment of the variable replaces.
pub fn cell_rebound(n: u64) -> u64 {
	let mut cell = std::cell::UnsafeCell::new(n);
	let p = cell_of(&cell);
	cell = std::cell::UnsafeCell::new(0);
	unsafe { *p + cell.into_inner() }
}

pub struct Target(pub u32);

/// A pointer through a reference that `mem::replace` points at another variable, which an
/// assignment of that variable replaces.
pub fn retargeted(n: u32) -> u32 {
	let mut a = Target(n);
	let mut b = Target(n);
	let mut r = &mut a;
	let _ = std::mem::replace(&mut r, &mut b);
	let p = &raw mut *r;
	b = Target(0);
	unsafe { (*p).0 + b.0 + a.0 }
}

/// A pointer through a reference that `mem::swap` points at another variable, which an
/// assignment of that variable replaces.
pub fn swap_retargeted(n: u32) -> u32 {
	let mut a = (n, n);
	let mut b = (n, n);
	let mut r = &mut a;
	let mut s = &mut b;
	std::mem::swap(&mut r, &mut s);
	let p = &raw mut *r;
	b = (0, 0);
	unsafe { (*p).0 + b.0 + a.0 }
}

/// A pointer through a reference a loop's `mem::swap`s may point at either variable, one
/// of which an assignment then replaces.
pub fn swapped_in_loop(n: u32) -> u32 {
	let mut a = (n, n);
	let mut b = (n, 1);
	let mut r = &mut a;
	let mut s = &mut b;
	for _ in 0..n {
		std::mem::swap(&mut r, &mut s);
	}
	let p = &raw mut *r;
	a = (0, 0);
	unsafe { (*p).0 + a.0 }
}

pub struct Behind(pub std::cell::Cell<u32>);

fn read_behind(b: &Behind) -> *const Behind {
	let p = &raw const *b;
	let held = (p,);
	unsafe { std::ptr::read(&raw const held).0 }
}

/// A pointer a callee reads back through a pointer to where it put it, which the read
/// holds, and returns, to a cell the caller's assignment replaces.
pub fn read_behind_rebound(n: u32) -> u32 {
	let mut b = Behind(std::cell::Cell::new(n));
	let p = read_behind(&b);
	b = Behind(std::cell::Cell::new(0));
	unsafe { (*p).0.get() + b.0.get() }
}

pub struct Unboxed(pub std::cell::Cell<u32>);

fn unbox(u: &Unboxed) -> *const Unboxed {
	*Box::leak(Box::new(&raw const *u))
}

/// A pointer a callee reads back out of a box it put it in, and returns, to a cell the
/// caller's assignment replaces.
pub fn unboxed_rebound(n: u32) -> u32 {
	let mut u = Unboxed(std::cell::Cell::new(n));
	let p = unbox(&u);
	u = Unboxed(std::cell::Cell::new(0));
	unsafe { (*p).0.get() + u.0.get() }
}

pub struct Deep {
	pub mid: DeepMid,
}

pub struct DeepMid(pub u8, pub std::cell::UnsafeCell<u32>);

fn deep_of(d: &Deep) -> *mut u32 {
	d.mid.1.get()
}

/// A pointer a callee makes into a cell two structs deep, which the caller's assignment
/// of the outer one replaces.
pub fn deep_cell_rebound(n: u32) -> u32 {
	let mut d = Deep {
		mid: DeepMid(1, std::cell::UnsafeCell::new(n)),
	};
	let p = deep_of(&d);
	d = Deep {
		mid: DeepMid(2, std::cell::UnsafeCell::new(0)),
	};
	unsafe { *p + d.mid.0 as u32 }
}

pub enum Slot {
	Empty,
	Full(std::cell::Cell<u32>),
}

pub struct Slotted(pub Slot);

fn slotted_of(s: &Slotted) -> *const Slotted {
	&raw const *s
}

/// A pointer a callee makes to a struct whose only cell is in an enum's payload, which
/// the caller's assignment replaces.
pub fn payload_cell_rebound(n: u32) -> u32 {
	let mut s = Slotted(Slot::Full(std::cell::Cell::new(n)));
	let p = slotted_of(&s);
	s = Slotted(Slot::Empty);
	let rest = matches!(s.0, Slot::Empty) as u32;
	match unsafe { &(*p).0 } {
		Slot::Full(c) => c.get() + rest,
		Slot::Empty => rest,
	}
}

pub struct Shared(pub std::cell::RefCell<usize>);

fn shared_of(s: &Shared) -> *const Shared {
	s
}

/// A pointer a callee makes to a `RefCell`'s holder, which the caller's assignment
/// replaces. A `usize` leaves no padding, which is not in the cell.
pub fn ref_cell_rebound(n: usize) -> usize {
	let mut s = Shared(std::cell::RefCell::new(n));
	let p = shared_of(&s);
	s = Shared(std::cell::RefCell::new(0));
	unsafe { *(*p).0.borrow() + *s.0.borrow() }
}

pub struct Duo(pub u32, pub u32);

pub struct Inside(pub std::cell::UnsafeCell<Duo>);

fn inside_of(i: &Inside) -> *mut Duo {
	i.0.get()
}

/// A pointer a callee makes through a cell to the object it holds, which the caller's
/// assignment of the cell's holder replaces.
pub fn inside_cell_rebound(n: u32) -> u32 {
	let mut i = Inside(std::cell::UnsafeCell::new(Duo(n, n)));
	let p = inside_of(&i);
	i = Inside(std::cell::UnsafeCell::new(Duo(0, 0)));
	unsafe { (*p).0 + (*i.0.get()).1 }
}

fn inside_field_of(i: &Inside) -> *mut u32 {
	unsafe { &raw mut (*i.0.get()).1 }
}

/// A pointer a callee makes through a cell to a field of the object it holds, which the
/// caller's assignment of the cell's holder replaces.
pub fn inside_field_rebound(n: u32) -> u32 {
	let mut i = Inside(std::cell::UnsafeCell::new(Duo(n, n)));
	let p = inside_field_of(&i);
	i = Inside(std::cell::UnsafeCell::new(Duo(0, 0)));
	unsafe { *p + (*i.0.get()).0 }
}

pub struct Compared(pub u32);

fn compared_of(c: &Compared) -> *const Compared {
	c
}

/// A pointer a callee makes to a struct with no cell, compared after the caller's
/// assignment, which Rust defines as its address and JS as the object it replaced.
pub fn compared_rebound(n: u32) -> bool {
	let mut c = Compared(n);
	let p = compared_of(&c);
	c = Compared(n + 1);
	p == compared_of(&c) && c.0 > 0
}

pub struct Enclosed(pub u32);

pub struct Enclosing(pub Enclosed);

fn enclosed_of(e: &Enclosing) -> *const Enclosed {
	&e.0
}

/// A pointer a callee makes to a part of a struct with no cell, compared after the
/// caller's assignment of the whole.
pub fn enclosed_compared_rebound(n: u32) -> bool {
	let mut e = Enclosing(Enclosed(n));
	let p = enclosed_of(&e);
	e = Enclosing(Enclosed(n + 1));
	p == enclosed_of(&e) && (e.0).0 > 0
}

pub struct Chosen(pub u32);

fn choose<'a>(a: &'a mut Chosen, b: &'a mut Chosen, first: bool) -> &'a mut Chosen {
	if first { a } else { b }
}

/// A pointer made of a reference a callee picks between two variables, compared after
/// the caller's assignment of one of them.
pub fn chosen_compared_rebound(n: u32, first: bool) -> bool {
	let mut a = Chosen(n);
	let mut b = Chosen(n + 1);
	let r = choose(&mut a, &mut b, first);
	let p = &raw const *r;
	a = Chosen(n + 2);
	p == &raw const a && a.0 > b.0
}

fn bytes_of(b: &[u8; 4]) -> *const [u8; 4] {
	b
}

/// A pointer a callee makes to an array of bytes, compared after the caller's assignment.
pub fn bytes_compared_rebound(n: u8) -> bool {
	let mut b = [n; 4];
	let p = bytes_of(&b);
	b = [n ^ 1; 4];
	std::ptr::eq(p, bytes_of(&b)) && b[0] > 0
}

pub struct Wrapped<T>(pub u8, pub T);

fn wrapped_of<T>(w: &Wrapped<T>) -> *const Wrapped<T> {
	w
}

/// A pointer a generic callee makes, instantiated with a cell, which the caller's
/// assignment replaces.
pub fn generic_cell_rebound(n: u32) -> u32 {
	let mut w = Wrapped(1, std::cell::Cell::new(n));
	let p = wrapped_of(&w);
	w = Wrapped(2, std::cell::Cell::new(0));
	unsafe { (*p).1.get() + w.0 as u32 }
}

pub fn part_cell(c: &std::cell::UnsafeCell<u16>) -> *mut u16 {
	c.get()
}

pub struct Parking<T> {
	pub task: T,
	pub park: Parked,
}

pub struct Parked(pub std::cell::Cell<i16>);

fn parked_of<A>(a: &A) -> *const A {
	a
}

/// A pointer a callee generic over what it points at makes into a cell, which the
/// caller's assignment replaces, in a body that the census lowers over its own type
/// parameter: the callee's instance for the cell's holder takes it.
pub fn generic_park_rebound<T>(mut w: Parking<T>, next: Parking<T>) -> i16 {
	let p = parked_of(&w.park);
	w = next;
	unsafe { (*p).0.get() + w.park.0.get() }
}
