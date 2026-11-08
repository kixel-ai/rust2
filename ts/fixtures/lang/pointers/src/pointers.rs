//! Raw pointers: a circular list of boxed nodes as `hashlink`'s `LinkedHashMap` keeps one,
//! linked through a union and holding a `MaybeUninit` entry, with a free list of nodes to
//! reuse; pointers to numbers, to fields, and to whole values, read, written, swapped,
//! and compared; pointers to the parts of a struct kept as objects; the pointers
//! `UnsafeCell::get` gives; slots of fresh memory written whole before any pointer
//! into them is taken, by index or through std's iterators over them; pointers into
//! nodes that an `AtomicPtr` and memory hold pointers to; pointers to arrays, compared
//! where they point, and to parts of slices as arrays, written through; a
//! pointer into a cell that other functions write whole through a `&mut` to it;
//! pointers a generic type holds to its parameter, at each form a pointer takes; the
//! raw pointers of an `Arc` and an `Rc`, counted and taken back; and unions over a type
//! parameter, each instance laid out at its own offsets.

use std::cell::{Cell, OnceCell, RefCell, UnsafeCell};
use std::mem::{self, MaybeUninit};
use std::ops;
use std::ptr::{self, NonNull};
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::Ordering::SeqCst;
use std::sync::atomic::{AtomicBool, AtomicPtr};
use std::task::{RawWaker, RawWakerVTable, Waker};

struct Links<V> {
	next: NonNull<Node<V>>,
	prev: NonNull<Node<V>>,
}

impl<V> Clone for Links<V> {
	fn clone(&self) -> Self {
		*self
	}
}

impl<V> Copy for Links<V> {}

struct Free<V> {
	next: Option<NonNull<Node<V>>>,
}

impl<V> Clone for Free<V> {
	fn clone(&self) -> Self {
		*self
	}
}

impl<V> Copy for Free<V> {}

/// A node's links while it is in the list, or while it is free.
union Link<V> {
	value: Links<V>,
	free: Free<V>,
}

struct Node<V> {
	entry: MaybeUninit<(u8, V)>,
	link: Link<V>,
}

/// Entries in the order they were pushed, but for those moved to the front.
struct List<V> {
	/// The guard: `next` is the first entry and `prev` the last, or itself if none.
	guard: Option<NonNull<Node<V>>>,
	free: Option<NonNull<Node<V>>>,
	len: usize,
}

impl<V> List<V> {
	fn new() -> Self {
		List {
			guard: None,
			free: None,
			len: 0,
		}
	}

	fn guard(&mut self) -> NonNull<Node<V>> {
		if let Some(guard) = self.guard {
			return guard;
		}
		let mut guard = unsafe { NonNull::new_unchecked(Box::into_raw(Box::new(Node::new()))) };
		unsafe {
			guard.as_mut().link.value = Links {
				next: guard,
				prev: guard,
			}
		};
		self.guard = Some(guard);
		guard
	}

	fn alloc(&mut self) -> NonNull<Node<V>> {
		match self.free {
			Some(free) => {
				self.free = unsafe { free.as_ref().link.free.next };
				free
			}
			None => NonNull::from(Box::leak(Box::new(Node::new()))),
		}
	}

	fn push_back(&mut self, key: u8, value: V) {
		let guard = self.guard();
		let mut node = self.alloc();
		unsafe {
			node.as_mut().entry.as_mut_ptr().write((key, value));
			attach_before(node, guard);
		}
		self.len += 1;
	}

	fn find(&self, key: u8) -> Option<NonNull<Node<V>>> {
		let guard = self.guard?;
		let mut at = unsafe { guard.as_ref().link.value.next };
		while at != guard {
			unsafe {
				if (*at.as_ptr()).entry.assume_init_ref().0 == key {
					return Some(at);
				}
				at = at.as_ref().link.value.next;
			}
		}
		None
	}

	fn remove(&mut self, key: u8) -> Option<(u8, V)> {
		let mut node = self.find(key)?;
		unsafe {
			detach(node);
			let entry = node.as_mut().entry.as_ptr().read();
			node.as_mut().link.free = Free { next: self.free };
			self.free = Some(node);
			self.len -= 1;
			Some(entry)
		}
	}

	fn to_front(&mut self, key: u8) -> bool {
		let Some(node) = self.find(key) else {
			return false;
		};
		unsafe {
			let Some(guard) = self.guard else { return false };
			detach(node);
			attach_before(node, guard.as_ref().link.value.next);
		}
		true
	}

	fn get_mut(&mut self, key: u8) -> Option<&mut V> {
		let node = self.find(key)?;
		unsafe { Some(&mut (*(*node.as_ptr()).entry.as_mut_ptr()).1) }
	}
}

impl List<Vec<u8>> {
	fn each(&self, out: &mut Vec<u8>) {
		let Some(guard) = self.guard else {
			return;
		};
		let mut at = unsafe { guard.as_ref().link.value.next };
		while at != guard {
			let (key, value) = unsafe { at.as_ref().entry.assume_init_ref() };
			out.push(*key);
			out.extend_from_slice(value);
			at = unsafe { at.as_ref().link.value.next };
		}
	}
}

impl<V> Node<V> {
	fn new() -> Self {
		Node {
			entry: MaybeUninit::uninit(),
			link: Link {
				value: Links {
					next: NonNull::dangling(),
					prev: NonNull::dangling(),
				},
			},
		}
	}
}

/// Put `node` in the list just before `at`.
unsafe fn attach_before<V>(mut node: NonNull<Node<V>>, mut at: NonNull<Node<V>>) {
	unsafe {
		node.as_mut().link.value = Links {
			prev: at.as_ref().link.value.prev,
			next: at,
		};
		at.as_mut().link.value.prev = node;
		(*node.as_ref().link.value.prev.as_ptr()).link.value.next = node;
	}
}

unsafe fn detach<V>(mut node: NonNull<Node<V>>) {
	unsafe {
		node.as_mut().link.value.prev.as_mut().link.value.next = node.as_ref().link.value.next;
		node.as_mut().link.value.next.as_mut().link.value.prev = node.as_ref().link.value.prev;
	}
}

impl<V> Drop for List<V> {
	fn drop(&mut self) {
		if let Some(guard) = self.guard {
			unsafe {
				let mut at = guard.as_ref().link.value.next;
				while at != guard {
					let next = at.as_ref().link.value.next;
					let mut node = Box::from_raw(at.as_ptr());
					node.entry.assume_init_drop();
					at = next;
				}
				drop(Box::from_raw(guard.as_ptr()));
			}
		}
		while let Some(free) = self.free {
			unsafe {
				self.free = free.as_ref().link.free.next;
				drop(Box::from_raw(free.as_ptr()));
			}
		}
	}
}

/// Steps on a list of the operation and key each pair of bytes picks, putting what each
/// finds and then the list in order.
pub fn lists(data: &[u8], out: &mut Vec<u8>) {
	let mut list = List::<Vec<u8>>::new();
	let mut data = data;
	while let [op, key, rest @ ..] = data {
		data = rest;
		let (op, key) = (*op, *key % 8);
		match op % 4 {
			0 => list.push_back(key, [&b""[..], b"v", b"vv"][usize::from(op % 3)].to_vec()),
			1 => match list.remove(key) {
				Some((k, v)) => {
					out.push(k);
					out.extend_from_slice(&v);
				}
				None => out.push(0xff),
			},
			2 => out.push(list.to_front(key) as u8),
			_ => match list.get_mut(key) {
				Some(value) => value.push(b'w'),
				None => out.push(0xfe),
			},
		}
	}
	out.push(list.len as u8);
	list.each(out);
}

/// Steps on a list of shared counts, putting how many hold the one each push clones
/// while the list lives, and after it drops.
pub fn shared(data: &[u8], out: &mut Vec<u8>) {
	let held = Arc::new(0u8);
	{
		let mut list = List::<Arc<u8>>::new();
		let mut data = data;
		while let [op, key, rest @ ..] = data {
			data = rest;
			let (op, key) = (*op, *key % 4);
			match op % 3 {
				0 => list.push_back(key, Arc::clone(&held)),
				1 => out.push(list.remove(key).map_or(0xff, |(_, v)| *v)),
				_ => out.push(list.to_front(key) as u8),
			}
			out.push(Arc::strong_count(&held) as u8);
		}
	}
	out.push(Arc::strong_count(&held) as u8);
}

#[derive(Clone, Copy)]
struct Pair {
	a: u32,
	b: u32,
}

/// Pointers to numbers, to a field, to a whole struct, and to bytes: read, written,
/// swapped, replaced with the old value read or not, compared, and null.
pub fn places(data: &[u8], out: &mut Vec<u8>) {
	let mut x = 0u32;
	let mut y = 1u32;
	let mut pair = Pair { a: 2, b: 3 };
	let px = &raw mut x;
	let py = &raw mut y;
	let pa = &raw mut pair.a;
	let pp = &raw mut pair;
	let mut held: *const u32 = ptr::null();
	let mut bytes = [0u8; 2];
	let pb = &raw mut bytes;
	for &byte in data {
		let v = u32::from(byte);
		unsafe {
			match byte % 12 {
				0 => *px += v,
				1 => py.write(v),
				2 => ptr::swap(px, py),
				3 => out.push(ptr::replace(pa, v) as u8),
				4 => (*pp).b = v,
				5 => {
					let old = pp.read();
					pp.write(Pair { a: old.b, b: old.a });
				}
				6 => held = if byte & 1 == 0 { px } else { pa },
				7 => out.push(held.is_null() as u8 | ((pp == &raw mut pair) as u8) << 1),
				9 => {
					let _ = pa.replace(v);
				}
				10 => {
					let _ = pp.replace(Pair { a: v, b: v + 1 });
				}
				11 => {
					let _ = pb.replace([byte, !byte]);
				}
				_ => out.push(if held.is_null() { 0xff } else { *held as u8 }),
			}
		}
	}
	out.extend_from_slice(&[x as u8, y as u8, pair.a as u8, pair.b as u8, bytes[0], bytes[1]]);
}

/// Unchecked indexing, a boxed number behind a pointer, a `MaybeUninit` number, and a
/// value dropped in place.
pub fn unchecked(data: &[u8], out: &mut Vec<u8>) {
	let mut counts = [0u32; 4];
	let boxed = Box::into_raw(Box::new(0u32));
	let mut slot = MaybeUninit::<u32>::uninit();
	slot.write(7);
	let held = Arc::new(0u8);
	let mut dropped = MaybeUninit::new(Arc::clone(&held));
	for &byte in data {
		let at = usize::from(byte % 4);
		unsafe {
			*counts.get_unchecked_mut(at) += 1;
			*boxed += u32::from(byte);
			*slot.as_mut_ptr() ^= u32::from(byte);
			out.push(*counts.get_unchecked(at) as u8);
		}
	}
	unsafe {
		out.push(*Box::from_raw(boxed) as u8);
		out.push(slot.assume_init_read() as u8);
		out.push(Arc::strong_count(&held) as u8);
		ptr::drop_in_place(dropped.as_mut_ptr());
	}
	out.push(Arc::strong_count(&held) as u8);
}

#[derive(Clone, Copy)]
struct Inner {
	a: u32,
	b: u32,
}

struct Outer {
	inner: Inner,
	pair: (u32, u32),
	n: u32,
}

/// Pointers to the parts of a struct kept as objects: initialized one by one from nothing,
/// written through the struct's pointer and their own, and read out before a write.
pub fn nested(data: &[u8], out: &mut Vec<u8>) {
	let mut slot = MaybeUninit::<Outer>::uninit();
	let p = slot.as_mut_ptr();
	let (inner, pair) = unsafe { (&raw mut (*p).inner, &raw mut (*p).pair) };
	unsafe {
		inner.write(Inner { a: 1, b: 2 });
		pair.write((3, 4));
		(&raw mut (*p).n).write(5);
	}
	for &byte in data {
		let v = u32::from(byte);
		unsafe {
			match byte % 6 {
				0 => (*p).inner.a = v,
				1 => inner.write(Inner { a: v, b: (*inner).a }),
				2 => (*pair).1 += v,
				3 => {
					let old = p.read();
					(*inner).b = v;
					out.push(old.inner.b as u8);
				}
				4 => {
					let old = pair.read();
					(*p).pair.0 = v;
					out.push(old.0 as u8);
				}
				_ => out.push((*inner).a as u8 ^ (*pair).0 as u8),
			}
		}
	}
	let outer = unsafe { slot.assume_init() };
	let Outer { inner, pair, n } = outer;
	out.extend_from_slice(&[inner.a as u8, inner.b as u8, pair.0 as u8, pair.1 as u8, n as u8]);
}

#[derive(Clone, Copy)]
union Halves {
	a: [u32; 2],
	b: [u32; 2],
}

/// Copies of a `MaybeUninit` and of a union holding arrays, each changed after, and values
/// read out through pointers before what they point at changes.
pub fn copies(data: &[u8], out: &mut Vec<u8>) {
	for &byte in data {
		let v = u32::from(byte);
		let mut x = MaybeUninit::new([v; 4]);
		let y = x;
		x.write([v ^ 1; 4]);
		let h = Halves { a: [v, v + 1] };
		let mut g = h;
		unsafe { g.b[0] = v + 2 };
		let mut held = Some(Pair { a: v, b: 0 });
		let p = &raw mut held;
		let old = unsafe { p.read() };
		if let Some(pair) = unsafe { &mut *p } {
			pair.a = v + 3;
		}
		let (before, after) = snapshot(v);
		unsafe {
			out.extend_from_slice(&[
				y.assume_init()[0] as u8,
				x.assume_init()[0] as u8,
				h.a[0] as u8,
				g.a[0] as u8,
				old.map_or(0, |o| o.a as u8),
				held.map_or(0, |o| o.a as u8),
				before,
				after,
			]);
		}
	}
}

/// What a pointer to a tuple holding an array reads, and what the tuple holds after.
fn snapshot(v: u32) -> (u8, u8) {
	let mut nest = ([v, 1u32], 0u8);
	let q = &raw mut nest;
	let before = unsafe { q.read() };
	nest.0[0] = v + 4;
	(before.0[0] as u8, nest.0[0] as u8)
}

/// Null and dangling pointers to values JS keeps as `undefined`: an `Option` read through
/// `as_ref`, and `()` written through a dangling pointer; and values of no bytes read
/// through dangling pointers.
pub fn sentinels(data: &[u8], out: &mut Vec<u8>) {
	for &byte in data {
		let some = Some(u32::from(byte)).filter(|v| v % 2 == 0);
		let p = match byte % 3 {
			0 => ptr::null(),
			_ => &raw const some,
		};
		let unit = NonNull::<()>::dangling().as_ptr();
		unsafe {
			unit.write(());
			*unit = ();
			out.push(match p.as_ref() {
				None => 0,
				Some(None) => 1,
				Some(Some(v)) => 2 + (*v % 100) as u8,
			});
			out.push(p.as_ref().is_none() as u8);
			let units = NonNull::<[(); 2]>::dangling();
			out.push(units.as_ptr().read().into_iter().count() as u8);
			out.push(units.as_ref().len() as u8);
		}
	}
}

/// A struct JS keeps as an object, which Rust copies. It has no `Default`, which
/// `generics::reset` would count as replaced whole.
#[derive(Clone, Copy)]
struct Duo(u8, u8);

/// A struct JS keeps as an object, owning a count.
struct Held {
	n: u8,
	count: Rc<()>,
}

/// crossbeam-epoch's own wrapper over `UnsafeCell`, which lends its pointer to a closure.
struct Wrapped<T>(UnsafeCell<T>);

impl<T> Wrapped<T> {
	const fn new(value: T) -> Self {
		Self(UnsafeCell::new(value))
	}

	fn with<R>(&self, f: impl FnOnce(*const T) -> R) -> R {
		f(self.0.get())
	}

	fn with_mut<R>(&self, f: impl FnOnce(*mut T) -> R) -> R {
		f(self.0.get())
	}
}

/// `UnsafeCell`s of a number, a string, a struct Rust copies, and one that owns a count: written
/// through `get` and `get_mut`, read through another `get` and before a write, assigned
/// through `get`, which drops what each held, replaced whole, and their values taken out.
pub fn cells(data: &[u8], out: &mut Vec<u8>) {
	let count = Rc::new(());
	let mut num = UnsafeCell::new(u32::MAX - 1);
	let mut pair = UnsafeCell::new(Duo(0, 0xff));
	let mut text = UnsafeCell::new(String::new());
	let mut held = Wrapped::new(Held {
		n: 0,
		count: Rc::clone(&count),
	});
	for &op in data {
		let n = op / 8;
		match op % 8 {
			// A write through one pointer, seen through another, and a value read out before it.
			0 => {
				let (a, b) = (&num, &num);
				let before = unsafe { *b.get() };
				unsafe { *a.get() = (*b.get()).wrapping_add(u32::from(n) << 26) };
				out.extend(before.to_le_bytes());
				out.extend(unsafe { *b.get() }.to_le_bytes());
			}
			1 => {
				*num.get_mut() ^= u32::from(n);
				out.extend(num.get_mut().to_le_bytes());
			}
			// A part written, then the whole, with a copy read out before each.
			2 => {
				let before = unsafe { *pair.get() };
				unsafe { (*pair.get()).0 = n };
				let between = unsafe { *pair.get() };
				let p = pair.get();
				unsafe { *p = Duo((*p).1, (*p).0) };
				pair.get_mut().1 ^= 0x80;
				out.extend([
					before.0,
					before.1,
					between.0,
					between.1,
					pair.get_mut().0,
					pair.get_mut().1,
				]);
			}
			3 => {
				unsafe { (*text.get()).push(char::from(b'a' + n % 26)) };
				text.get_mut().push('!');
				out.extend(unsafe { &*text.get() }.as_bytes());
			}
			// Assigned through the pointer, which drops what it held, or written over, which
			// leaks it.
			4 => {
				let fresh = Held {
					n,
					count: Rc::clone(&count),
				};
				match n % 2 {
					0 => held.with_mut(|p| unsafe { *p = fresh }),
					_ => held.with_mut(|p| unsafe { ptr::write(p, fresh) }),
				}
				out.extend([held.with(|p| unsafe { (*p).n }), Rc::strong_count(&count) as u8]);
			}
			5 => {
				held.with_mut(|p| unsafe { (*p).n = (*p).n.wrapping_add(n) });
				held.0.get_mut().n ^= 1;
				out.push(held.with(|p| unsafe { (*p).n }));
			}
			// Each of a number and a string replaced whole, its old value taken out, while
			// the pointers `get` gave in other arms are dead. A cell of an object can't be,
			// as a pointer `get` gave would keep the old object.
			6 => {
				let old = std::mem::replace(&mut num, UnsafeCell::new(u32::from(n)));
				out.extend(old.into_inner().to_le_bytes());
				let old = std::mem::replace(&mut text, UnsafeCell::new(n.to_string()));
				out.extend(old.into_inner().as_bytes());
				*pair.get_mut() = Duo(n, !n);
			}
			// One made, written through `get`, and its value taken out and dropped.
			_ => {
				let made = Wrapped::new(Held {
					n,
					count: Rc::clone(&count),
				});
				made.with_mut(|p| unsafe { (*p).n ^= 0x55 });
				out.push(Rc::strong_count(&count) as u8);
				let made = made.0.into_inner();
				out.push(made.n);
				drop(made);
				out.push(Rc::strong_count(&count) as u8);
			}
		}
	}
	out.extend(num.into_inner().to_le_bytes());
	let Duo(a, b) = pair.into_inner();
	out.extend([a, b]);
	out.extend(text.into_inner().as_bytes());
	drop(held);
	out.push(Rc::strong_count(&count) as u8);
}

fn renew_num(cell: &mut UnsafeCell<u32>, value: u32) -> u32 {
	std::mem::replace(cell, UnsafeCell::new(value)).into_inner()
}

/// Pointers that keep pointing where Rust's do across `mem::replace`, `mem::swap`,
/// `mem::take`, and `*r = v`: one taken after its variable is replaced, one whose number
/// parts are, ones a helper writes through a `&mut` into the object there, and one taken
/// through two helpers and read while the variable is borrowed, which is assigned after;
/// one to a part a `&mut` given as `mem::replace`'s value borrows, which it leaves be;
/// and one through a reference `mem::replace` pointed elsewhere first.
pub fn rebound(data: &[u8], out: &mut Vec<u8>) {
	for &n in data {
		match n % 7 {
			0 => out.extend(replaced_before(n)),
			6 => out.extend(retargeted(n)),
			5 => out.extend(lent_as_value(n)),
			1 => out.extend(parts_replaced(n)),
			2 => out.extend(written_in_place(n)),
			3 => out.extend(cell_written_in_place(n)),
			_ => out.extend(borrowed_through(n)),
		}
	}
}

fn replaced_before(n: u8) -> [u8; 2] {
	let mut pair = (n, !n);
	let old = std::mem::replace(&mut pair, (n ^ 1, n));
	let p = &raw mut pair;
	unsafe { (*p).1 ^= old.0 };
	[pair.0, pair.1]
}

struct Span {
	range: (u8, u8),
}

fn lent_as_value(n: u8) -> [u8; 4] {
	let mut span = Span { range: (n, !n) };
	let mut other = (0, 0);
	let mut r = &mut other;
	let p = &raw mut span.range;
	let old = std::mem::replace(&mut r, &mut span.range);
	old.0 = n ^ 1;
	r.0 ^= 3;
	unsafe { [(*p).0, (*p).1, other.0, span.range.0] }
}

fn retargeted(n: u8) -> [u8; 2] {
	let mut a = (n, !n);
	let mut b = (n ^ 5, n);
	let mut r = &mut a;
	let _ = std::mem::replace(&mut r, &mut b);
	let p = &raw mut *r;
	a = (0, 0);
	unsafe { [(*p).0 ^ a.0, (*p).1] }
}

fn parts_replaced(n: u8) -> [u8; 4] {
	let mut pair = (n, !n);
	let p = &raw mut pair;
	let old = std::mem::replace(&mut pair.0, n.wrapping_add(1));
	std::mem::swap(&mut pair.0, &mut pair.1);
	let taken = std::mem::take(&mut pair.1);
	unsafe { [(*p).0, (*p).1, old, taken] }
}

fn written_in_place(n: u8) -> [u8; 6] {
	let mut a = Duo(n, 1);
	let mut b = Duo(2, n);
	let p = &raw mut a;
	let old = renew(&mut a, Duo(n ^ 3, n ^ 4));
	let between = unsafe { *p };
	exchange(&mut a, &mut b);
	let swapped = unsafe { *p };
	assign(&mut a, Duo(old.1, b.0));
	unsafe { [between.0, between.1, swapped.0, swapped.1, (*p).0, (*p).1] }
}

fn cell_written_in_place(n: u8) -> [u8; 4] {
	let mut cell = UnsafeCell::new(u32::from(n) << 9);
	let p = cell.get();
	let old = renew_num(&mut cell, u32::from(n) + 7);
	let between = unsafe { *p };
	let taken = clear(&mut cell);
	[
		old as u8,
		(old >> 9) as u8,
		between as u8,
		taken as u8 ^ unsafe { *p } as u8,
	]
}

fn borrowed_through(n: u8) -> [u8; 3] {
	let mut d = Duo(n, !n);
	let first = flipped(&mut d);
	d = Duo(d.1, d.0);
	[first, d.0, d.1]
}

fn renew(at: &mut Duo, value: Duo) -> Duo {
	std::mem::replace(at, value)
}

fn exchange(a: &mut Duo, b: &mut Duo) {
	std::mem::swap(a, b)
}

fn assign(at: &mut Duo, value: Duo) {
	*at = value;
}

fn clear(cell: &mut UnsafeCell<u32>) -> u32 {
	std::mem::take(cell).into_inner()
}

/// The first of `d`, flipped through the pointer `pointer_of` gives.
fn flipped(d: &mut Duo) -> u8 {
	let p = pointer_of(d);
	unsafe {
		(*p).0 ^= 0x5a;
		(*p).0
	}
}

fn pointer_of(d: &mut Duo) -> *mut Duo {
	&raw mut *d
}

/// A struct JS keeps as an object, holding bytes.
struct Log {
	bytes: Vec<u8>,
}

/// Pointers to variables JS keeps as objects, each dead before its variable is assigned
/// again, so it reads the object the variable held while it was made: one made in each
/// iteration of a loop that assigns the variable after, one an arm makes where another
/// arm replaces its variable, and ones nested loops make.
pub fn lapsed(data: &[u8], out: &mut Vec<u8>) {
	let mut pair = Duo(0, 0xff);
	let mut log = Log { bytes: Vec::new() };
	let mut window = [0u8; 3];
	for &n in data {
		let p = &raw mut pair;
		unsafe { (*p).0 = (*p).0.wrapping_add(n) };
		out.extend(unsafe { [(*p).0, (*p).1] });
		pair = Duo(pair.1 ^ n, pair.0);
		match n % 4 {
			0 => {
				let q = &raw mut log;
				unsafe { (*q).bytes.push(n) };
				out.extend(unsafe { &(*q).bytes });
			}
			1 => {
				let old = std::mem::replace(&mut log, Log { bytes: vec![n, !n] });
				out.extend(old.bytes);
				let old = std::mem::replace(&mut pair, Duo(n, n ^ 0x0f));
				out.extend([old.0, old.1]);
			}
			// The outer loop's pointer, read in the inner one and after it, and the inner
			// one's own, whose variable each inner iteration assigns.
			2 => {
				let mut duo = Duo(n, 0);
				let outer = &raw mut duo;
				for i in 0..n % 5 {
					let inner = &raw mut window;
					unsafe {
						(*inner)[usize::from(i % 3)] ^= n;
						(*outer).1 = (*outer).1.wrapping_add((*inner)[0]);
					}
					window = [window[2], window[0], window[1].wrapping_add(i)];
				}
				out.extend(unsafe { [(*outer).0, (*outer).1] });
				duo = Duo(duo.1, duo.0);
				out.extend([duo.0, duo.1]);
			}
			_ => {
				let p = &raw mut window;
				let copy = p;
				out.push(unsafe { (*copy)[0] });
				window = [n; 3];
			}
		}
	}
	out.extend([pair.0, pair.1]);
	out.extend(log.bytes);
	out.extend(window);
}

/// Structs JS keeps as objects, which Rust copies, and which only the generic bodies below
/// write into, each reached at one of them: by a call, through a trait object, and through
/// a closure. Each copy of one is copied in JS too, as are none of the types they aren't
/// reached at, such as `nested`'s, whose pointers translate.
#[derive(Clone, Copy, Default)]
struct ByCall(u8, u8);

#[derive(Clone, Copy, Default)]
struct ByObject(u8, u8);

#[derive(Clone, Copy, Default)]
struct ByClosure(u8, u8);

fn put<T>(slot: &mut T, value: T) {
	*slot = value;
}

/// A value of a `T` in a cell, which `reset` gives `T`'s default.
struct Bin<T>(UnsafeCell<T>);

trait Reset {
	fn reset(&self);
}

impl<T: Default> Reset for Bin<T> {
	fn reset(&self) {
		unsafe { *self.0.get() = T::default() };
	}
}

fn resetter<T: Default>(slot: *mut T) -> impl FnOnce() {
	move || unsafe { *slot = T::default() }
}

/// Each byte a copy of one of them, the original written whole after by its generic body,
/// and both put. Each is a function of its own, as a loop assigns its variables again.
pub fn writers(data: &[u8], out: &mut Vec<u8>) {
	for &byte in data {
		let n = byte / 3;
		out.extend(match byte % 3 {
			0 => by_call(n),
			1 => by_object(n),
			_ => by_closure(n),
		});
	}
}

fn by_call(n: u8) -> [u8; 4] {
	let mut a = ByCall(n, 1);
	let b = a;
	put(&mut a, ByCall(2, !n));
	[a.0, a.1, b.0, b.1]
}

fn by_object(n: u8) -> [u8; 4] {
	let bin = Bin(UnsafeCell::new(ByObject(n, 1)));
	let before = unsafe { *bin.0.get() };
	let object: &dyn Reset = &bin;
	object.reset();
	let after = bin.0.into_inner();
	[after.0, after.1, before.0, before.1]
}

fn by_closure(n: u8) -> [u8; 4] {
	let mut a = ByClosure(n, 1);
	let b = a;
	resetter(&raw mut a)();
	[a.0, a.1, b.0, b.1]
}

struct Gauge {
	range: (u8, u8),
	slot: MaybeUninit<(u8, u8)>,
}

/// Each part of a `Gauge` written into, never given another object.
fn nudge(g: &mut Gauge, n: u8) {
	g.range.0 ^= n;
	unsafe { (*g.slot.as_mut_ptr()).1 ^= n };
}

/// Pointers std makes to parts kept as objects, which a helper writes into but never gives
/// another object: `NonNull::from` of a `&mut` and of a `&`, `into` of a `&mut`, and
/// `MaybeUninit::as_mut_ptr` and `as_ptr`. Each is a function of its own, as a loop assigns
/// its variables again.
pub fn made(data: &[u8], out: &mut Vec<u8>) {
	for &n in data {
		out.extend(made_parts(n));
	}
}

fn made_parts(n: u8) -> [u8; 7] {
	let mut g = Gauge {
		range: (n, !n),
		slot: MaybeUninit::new((!n, n)),
	};
	let p = NonNull::from(&mut g.range);
	let q = g.slot.as_mut_ptr();
	nudge(&mut g, n ^ 0x5a);
	unsafe { (*q).0 = p.as_ref().1 };
	let r = NonNull::from(&g.range);
	let s = g.slot.as_ptr();
	let t: NonNull<(u8, u8)> = (&mut g.range).into();
	nudge(&mut g, n);
	unsafe {
		[
			p.as_ref().0,
			p.as_ref().1,
			r.as_ref().0,
			t.as_ref().1,
			(*s).0,
			(*s).1,
			g.slot.assume_init().0,
		]
	}
}

/// A cell read and written only through the pointer it lends a closure, as kio's
/// `sync::UnsafeCell` is.
struct Lent<T>(UnsafeCell<T>);

impl<T> Lent<T> {
	const fn new(value: T) -> Self {
		Self(UnsafeCell::new(value))
	}

	fn with<R>(&self, f: impl FnOnce(*const T) -> R) -> R {
		f(self.0.get())
	}

	fn with_mut<R>(&self, f: impl FnOnce(*mut T) -> R) -> R {
		f(self.0.get())
	}
}

type Entry = Lent<MaybeUninit<(u8, u8)>>;

/// A run of slots as kio's log keeps one: each filled whole with an empty cell while
/// nothing points into the fresh allocation yet, then written and read through the
/// pointer its cell lends.
struct Segment {
	filled: usize,
	slots: Box<[MaybeUninit<Entry>; 4]>,
}

impl Segment {
	fn new() -> Self {
		let mut slots = Box::new([const { MaybeUninit::uninit() }; 4]);
		for at in 0..4 {
			slots[at].write(Lent::new(MaybeUninit::uninit()));
		}
		Segment { filled: 0, slots }
	}

	/// As [`Self::new`], with kio's loop written with an iterator.
	fn iterated() -> Self {
		let mut slots = Box::new([const { MaybeUninit::uninit() }; 4]);
		for slot in slots.iter_mut() {
			slot.write(Lent::new(MaybeUninit::uninit()));
		}
		Segment { filled: 0, slots }
	}

	fn slot(&self, at: usize) -> &Entry {
		unsafe { self.slots[at].assume_init_ref() }
	}
}

/// Entries pushed into segments of slots that each fill whole first, read back, and
/// changed in place through the pointer each slot's cell lends.
pub fn segments(data: &[u8], out: &mut Vec<u8>) {
	let mut log: Vec<Segment> = Vec::new();
	let mut len = 0;
	for &byte in data {
		match byte % 4 {
			0 => {
				if len % 8 == 0 {
					log.push(Segment::new());
				} else if len % 4 == 0 {
					log.push(Segment::iterated());
				}
				let segment = &mut log[len / 4];
				segment
					.slot(segment.filled)
					.with_mut(|p| unsafe { (*p).write((byte, !byte)) });
				segment.filled += 1;
				len += 1;
			}
			1 if len > 0 => {
				let at = usize::from(byte) % len;
				let (a, b) = log[at / 4].slot(at % 4).with(|p| unsafe { *(*p).assume_init_ref() });
				out.extend([a, b]);
			}
			2 if len > 0 => {
				let at = usize::from(byte) % len;
				log[at / 4]
					.slot(at % 4)
					.with_mut(|p| unsafe { (*(*p).as_mut_ptr()).1 ^= byte });
			}
			_ => out.push(log.iter().map(|s| s.filled as u8).sum()),
		}
	}
}

/// A struct JS keeps as an object, with a part it keeps as one too.
struct Run {
	span: (u8, u8),
	n: u8,
}

/// A `MaybeUninit` array and a `MaybeUninit` variable, each slot written whole before any
/// pointer into it is taken, then changed through pointers to a part of each and read back.
pub fn uninit_runs(data: &[u8], out: &mut Vec<u8>) {
	let mut runs: [MaybeUninit<Run>; 3] = [const { MaybeUninit::uninit() }; 3];
	for at in 0..3 {
		let n = data.get(at).copied().unwrap_or(0);
		runs[at].write(Run {
			span: (n, !n),
			n: at as u8,
		});
	}
	let mut last = MaybeUninit::<Run>::uninit();
	unsafe {
		last.as_mut_ptr().write(Run {
			span: (data.len() as u8, 0),
			n: 3,
		})
	};
	let tail = unsafe { &raw mut (*last.as_mut_ptr()).span };
	for &byte in data {
		let run = unsafe { runs[usize::from(byte) % 3].assume_init_mut() };
		let span = &raw mut run.span;
		unsafe {
			(*span).0 ^= byte;
			(*span).1 = (*span).1.wrapping_add(run.n);
			(*tail).1 = (*tail).1.wrapping_add((*span).0);
			out.push((*span).0);
		}
	}
	for run in &runs {
		let run = unsafe { run.assume_init_ref() };
		out.extend([run.span.0, run.span.1, run.n]);
	}
	let last = unsafe { last.assume_init() };
	out.extend([last.span.0, last.span.1, last.n]);
}

/// `MaybeUninit` slots each written whole through std's iterators over them before any
/// pointer into them is taken: an array's `iter_mut`, a boxed slice's `&mut`, its
/// `chunks_exact_mut`, and a `zip` with an index range; then changed through pointers to
/// a part of each and read back.
pub fn iterated_runs(data: &[u8], out: &mut Vec<u8>) {
	let byte = |at: usize| data.get(at).copied().unwrap_or(0);
	let mut runs: [MaybeUninit<Run>; 3] = [const { MaybeUninit::uninit() }; 3];
	for slot in runs.iter_mut() {
		slot.write(Run {
			span: (byte(0), !byte(0)),
			n: 0,
		});
	}
	let mut boxed: Box<[MaybeUninit<Run>]> = Box::new([const { MaybeUninit::uninit() }; 4]);
	for slot in &mut boxed {
		slot.write(Run {
			span: (byte(1), 1),
			n: 1,
		});
	}
	for (chunk, at) in boxed.chunks_exact_mut(2).zip(0u8..) {
		for slot in chunk {
			slot.write(Run {
				span: (byte(2), at),
				n: at.wrapping_add(byte(3)),
			});
		}
	}
	for (slot, at) in runs.iter_mut().zip(0..) {
		slot.write(Run {
			span: (byte(at), at as u8),
			n: at as u8,
		});
	}
	for &byte in data {
		let run = unsafe { runs[usize::from(byte) % 3].assume_init_mut() };
		let span = &raw mut run.span;
		unsafe {
			(*span).0 ^= byte;
			(*span).1 = (*span).1.wrapping_add(run.n);
			out.push((*span).0);
		}
		let run = unsafe { boxed[usize::from(byte) % 4].assume_init_mut() };
		let span = &raw mut run.span;
		unsafe {
			(*span).1 ^= byte;
			out.push((*span).1);
		}
	}
	for run in runs.iter().chain(boxed.iter()) {
		let run = unsafe { run.assume_init_ref() };
		out.extend([run.span.0, run.span.1, run.n]);
	}
}

/// A struct JS keeps as an object, holding an `rs.Cell`.
struct Stable(UnsafeCell<u8>);

fn first_set<T>(pair: &mut (T, Stable), value: T) {
	pair.0 = value;
}

fn stable_cell(stable: &Stable) -> *mut u8 {
	stable.0.get()
}

/// A pointer to a cell beside a field a generic body writes whole, which leaves the cell's
/// struct be.
pub fn beside_written(data: &[u8], out: &mut Vec<u8>) {
	let mut pair = (0u16, Stable(UnsafeCell::new(0)));
	let cell = stable_cell(&pair.1);
	for &byte in data {
		first_set(&mut pair, u16::from(byte));
		unsafe { *cell = (*cell).wrapping_add(byte) };
		out.push(unsafe { *cell } ^ pair.0 as u8);
	}
}

/// A node a list links through an `AtomicPtr`, holding a cell it lends a pointer into, as
/// wasmtimer's timer nodes hold futures' `AtomicWaker`.
struct Waiter {
	next: AtomicPtr<Waiter>,
	enqueued: AtomicBool,
	slot: UnsafeCell<(u8, u8)>,
}

fn waiter(n: u8) -> *mut Waiter {
	Box::into_raw(Box::new(Waiter {
		next: AtomicPtr::new(ptr::null_mut()),
		enqueued: AtomicBool::new(false),
		slot: UnsafeCell::new((n, 0)),
	}))
}

fn slot_of(waiter: &Waiter) -> *mut (u8, u8) {
	waiter.slot.get()
}

/// Waiters pushed onto an atomic head and popped through `get_mut`, as wasmtimer's
/// `ArcList` keeps them, with the one popped last kept in memory too.
struct Waiters {
	head: AtomicPtr<Waiter>,
	last: *mut Waiter,
}

impl Waiters {
	fn push(&self, waiter: *mut Waiter) {
		let node = unsafe { &*waiter };
		if node.enqueued.swap(true, SeqCst) {
			return;
		}
		let mut head = self.head.load(SeqCst);
		loop {
			node.next.store(head, SeqCst);
			match self.head.compare_exchange(head, waiter, SeqCst, SeqCst) {
				Ok(_) => break,
				Err(now) => head = now,
			}
		}
	}

	fn pop(&mut self) -> bool {
		let head = *self.head.get_mut();
		if head.is_null() {
			return false;
		}
		*self.head.get_mut() = unsafe { (*head).next.load(SeqCst) };
		unsafe { (*head).enqueued.store(false, SeqCst) };
		self.last = head;
		true
	}
}

/// Waiters linked and unlinked, each changed and read through the pointer into its cell
/// taken when it was made, which nothing writing a waiter whole leaves stale.
pub fn waiters(data: &[u8], out: &mut Vec<u8>) {
	let nodes = [waiter(0), waiter(1), waiter(2), waiter(3)];
	let slots = unsafe {
		[
			slot_of(&*nodes[0]),
			slot_of(&*nodes[1]),
			slot_of(&*nodes[2]),
			slot_of(&*nodes[3]),
		]
	};
	let mut list = Waiters {
		head: AtomicPtr::new(ptr::null_mut()),
		last: ptr::null_mut(),
	};
	for &byte in data {
		let at = usize::from(byte >> 2) % 4;
		match byte % 4 {
			0 => list.push(nodes[at]),
			1 => out.push(u8::from(list.pop())),
			2 if !list.last.is_null() => unsafe {
				let slot = slot_of(&*list.last);
				(*slot).1 = (*slot).1.wrapping_add(byte);
			},
			_ => {
				let (a, b) = unsafe { *slots[at] };
				out.extend([a, b]);
			}
		}
	}
	let mut at = list.head.load(SeqCst);
	while !at.is_null() {
		let (a, b) = unsafe { *slot_of(&*at) };
		out.extend([a, b]);
		at = unsafe { (*at).next.load(SeqCst) };
	}
	for node in 0..4 {
		drop(unsafe { Box::from_raw(nodes[node]) });
	}
}

/// Pointers callees take through references, which keep pointing where Rust's do: one
/// read before its variable is assigned again, ones kept where liveness can't follow them
/// for a variable each iteration of a loop declares, one through a reference `mem::swap`
/// points at another variable, whose first is assigned after, and ones a callee gives back
/// only as a reference, through a borrow a closure keeps.
pub fn taken(data: &[u8], out: &mut Vec<u8>) {
	for &n in data {
		match n % 4 {
			0 => out.extend(read_before(n)),
			1 => per_iteration(n, out),
			2 => out.extend(swapped_away(n)),
			_ => out.extend(viewed(n)),
		}
	}
}

fn read_before(n: u8) -> [u8; 3] {
	let mut d = Duo(n, !n);
	let q = pointer_of(&mut d);
	let seen = unsafe {
		(*q).0 ^= 0x33;
		(*q).0
	};
	d = Duo(d.1, d.0);
	[seen, d.0, d.1]
}

fn keep(d: &mut Duo, kept: &mut Vec<*mut Duo>) {
	kept.push(&raw mut *d);
}

fn per_iteration(n: u8, out: &mut Vec<u8>) {
	let mut kept = Vec::new();
	for i in 0..n % 4 {
		let mut d = Duo(n ^ i, i);
		keep(&mut d, &mut kept);
		unsafe { (*kept[0]).1 ^= n };
		out.extend([d.0, d.1]);
		kept.clear();
	}
}

fn swapped_away(n: u8) -> [u8; 4] {
	let mut a = (n, 1);
	let mut b = (2, n);
	let mut r = &mut a;
	let mut s = &mut b;
	std::mem::swap(&mut r, &mut s);
	let p = &raw mut *r;
	a = (a.1, a.0);
	unsafe { (*p).0 ^= 7 };
	[a.0, a.1, b.0, b.1]
}

struct Viewed(u8, u8);

fn first_of(v: &mut Viewed) -> &mut u8 {
	unsafe { &mut (*(&raw mut *v)).0 }
}

fn viewed(n: u8) -> [u8; 2] {
	let mut v = Viewed(n, 3);
	let mut bump = |k| *first_of(&mut v) ^= k;
	bump(5);
	bump(n);
	v = Viewed(v.1, v.0);
	[v.0, v.1]
}

/// Pointers to arrays of bytes, made of the array and of references to it and to parts of
/// it: compared where they point, alone and held in other values, read, written whole and
/// by index, swapped, and replaced, as `ptr`, `NonNull`, `Box`, `MaybeUninit`,
/// `UnsafeCell`, and `AtomicPtr` make and take them. Pointers to one array are equal however they were made, and to arrays of equal
/// bytes, or to parts at other offsets, are not.
pub fn bytes(data: &[u8], out: &mut Vec<u8>) {
	for &n in data {
		let mut a = [n, n ^ 1, n ^ 2, n ^ 3];
		let b = a;
		let from_ref: *const [u8; 4] = &a;
		let from_raw = &raw const a;
		out.extend([
			u8::from(ptr::eq(from_ref, &a)),
			u8::from(ptr::eq(from_ref, from_raw)),
			u8::from(ptr::eq(&a, &b)),
			u8::from(from_ref == from_raw),
			u8::from(from_ref != &raw const b),
			u8::from(ptr::eq(from_ref, ptr::null())),
			u8::from(same_bytes(&a, &a)),
			u8::from(same_bytes(&a, &b)),
		]);
		out.extend(held(n));
		let chunks = parts(&mut a);
		out.extend(chunks);
		match n % 4 {
			0 => out.extend(moved(n)),
			1 => out.extend(wrapped(n)),
			2 => out.extend(in_vec(n)),
			_ => out.extend(exchanged(n)),
		}
		out.extend(a);
	}
}

fn same_bytes(p: *const [u8; 4], q: &[u8; 4]) -> bool {
	ptr::eq(p, q)
}

#[derive(PartialEq)]
struct Pointing(*const [u8; 4]);

/// Pointers compared in a tuple, an array, a vector, an `Option`, and a derived
/// `PartialEq`.
fn held(n: u8) -> [u8; 6] {
	let (a, b) = ([n; 4], [n; 4]);
	let (p, q, r): (*const [u8; 4], *const [u8; 4], *const [u8; 4]) = (&a, &a, &b);
	[
		u8::from((p, 1) == (q, 1)),
		u8::from([p] == [r]),
		u8::from(vec![r, p].contains(&q)),
		u8::from(Some(p) == Some(q)),
		u8::from(Some(p) == Some(r)),
		u8::from(Pointing(p) == Pointing(q)),
	]
}

/// Pointers to parts of an array at each offset, compared, and one written through.
fn parts(a: &mut [u8; 4]) -> [u8; 5] {
	let first: *const [u8; 2] = a.first_chunk().unwrap();
	let last: *const [u8; 2] = a.last_chunk().unwrap();
	let again: *const [u8; 2] = a[..3].first_chunk().unwrap();
	let (_, middle) = a.split_first_chunk::<1>().unwrap();
	let middle: *const [u8; 2] = middle.first_chunk().unwrap();
	let written: *mut [u8; 2] = a.last_chunk_mut().unwrap();
	unsafe { (*written)[0] ^= 0x80 };
	[
		u8::from(ptr::eq(first, last)),
		u8::from(ptr::eq(first, again)),
		u8::from(ptr::eq(first, middle)),
		u8::from(ptr::eq(last, written)),
		unsafe { (*last)[0] },
	]
}

/// `ptr::read`, `write`, `swap`, and `replace`, a whole write and a write by index
/// through a pointer, and a reference made again of one.
fn moved(n: u8) -> [u8; 16] {
	let mut a = [n; 4];
	let mut b = [1, 2, 3, n];
	let p = &raw mut a;
	let q: *mut [u8; 4] = &mut b;
	let (read, old, through) = unsafe {
		let read = p.read();
		ptr::swap(p, q);
		ptr::swap(p, p);
		let old = ptr::replace(q, [n ^ 0x0f; 4]);
		(*p)[0] = old[1];
		q.write([read[0], 7, 8, 9]);
		*p = [(*p)[0], (*p)[1], (*q)[1], n];
		let r: &mut [u8; 4] = &mut *q;
		r[3] ^= 0x40;
		(read, old, [r[0], r[1], (*p)[3]])
	};
	[
		read[0], read[1], old[0], old[2], a[0], a[1], a[2], a[3], b[0], b[1], b[2], b[3], through[0], through[1],
		through[2], n,
	]
}

/// Pointers `NonNull`, `Box`, `MaybeUninit`, and `UnsafeCell` make, and the references
/// made again of them.
fn wrapped(n: u8) -> [u8; 14] {
	let mut a = [n, 1, 2, 3];
	let mut non_null = NonNull::from(&mut a);
	let shared = NonNull::from(&a);
	let same = non_null == shared;
	unsafe {
		non_null.as_mut()[1] = n ^ 0x55;
		let viewed = shared.as_ref()[1];
		let as_ref = non_null.as_ptr().as_ref().unwrap()[0];
		let boxed = Box::into_raw(Box::new([n; 3]));
		(*boxed)[2] = 9;
		let boxed = *Box::from_raw(boxed);
		let mut uninit = MaybeUninit::<[u8; 2]>::uninit();
		let whole: *mut MaybeUninit<[u8; 2]> = &mut uninit;
		let slot = (*whole).as_mut_ptr();
		(*whole).write([n, n]);
		(*slot)[1] ^= 0x22;
		let aliased = ptr::eq(whole.cast::<[u8; 2]>(), slot);
		let uninit = uninit.assume_init();
		let cell = UnsafeCell::new([n; 2]);
		(*cell.get())[0] = 4;
		let cell = cell.into_inner();
		[
			u8::from(same),
			u8::from(aliased),
			viewed,
			as_ref,
			boxed[0],
			boxed[2],
			uninit[0],
			uninit[1],
			cell[0],
			cell[1],
			a[0],
			a[1],
			a[2],
			a[3],
		]
	}
}

/// Pointers to parts of a vector's bytes, compared and written through.
fn in_vec(n: u8) -> [u8; 5] {
	let mut v = vec![n; 6];
	let p: *mut [u8; 2] = v.first_chunk_mut().unwrap();
	let q: *const [u8; 2] = v.first_chunk().unwrap();
	let r: *const [u8; 2] = v.last_chunk().unwrap();
	unsafe {
		(*p)[1] = 9;
		p.write([(*r)[0] ^ 1, (*q)[1]]);
	}
	[u8::from(ptr::eq(p, q)), u8::from(ptr::eq(q, r)), v[0], v[1], v[5]]
}

/// An `AtomicPtr` to an array, exchanged only for a pointer to it.
fn exchanged(n: u8) -> [u8; 3] {
	let mut a = [n; 2];
	let mut b = [n ^ 1; 2];
	let at = AtomicPtr::new(&raw mut a);
	let missed = at.compare_exchange(&raw mut b, &raw mut b, SeqCst, SeqCst).is_err();
	let hit = at.compare_exchange(&mut a, &mut b, SeqCst, SeqCst).is_ok();
	unsafe { (*at.load(SeqCst))[0] = 3 };
	[u8::from(missed), u8::from(hit), b[0]]
}

/// Values std's reads load through pointers a closure makes of references, which hold
/// none of those pointers: each read out over a type parameter, as smallvec's `Drain`
/// reads its items, and kept past its array's next value.
pub fn loaded(data: &[u8], out: &mut Vec<u8>) {
	for &n in data {
		let mut pair = [Loaded(n, 1), Loaded(2, !n)];
		let first = next_read(&mut pair.iter()).unwrap_or(Loaded(0, 0));
		pair = [Loaded(first.1, n ^ 0x5a), Loaded(first.0, n)];
		out.extend([first.0, first.1, pair[0].0, pair[0].1, pair[1].0]);
	}
}

/// A struct JS keeps as an object that only [`loaded`] reads out.
struct Loaded(u8, u8);

fn next_read<T>(items: &mut std::slice::Iter<'_, T>) -> Option<T> {
	items.next().map(|r| unsafe { std::ptr::read(r) })
}

/// Pointers callees keep, through references, to values with no cell in them, which their
/// variables are assigned again past while they are kept: each made again before it is
/// read, as zlib's stream takes its window each round, by a callee over a type parameter,
/// into fields of one generic type: one into an array and one into a struct.
pub fn reattached(data: &[u8], out: &mut Vec<u8>) {
	let mut window = Window {
		at: std::ptr::null_mut(),
	};
	let mut buf = [0u16; 4];
	let mut slot = Window {
		at: std::ptr::null_mut(),
	};
	let mut pair = Reattached(0, 0);
	for &n in data {
		attach(&mut window.at, &mut buf);
		unsafe { (*window.at)[usize::from(n % 4)] ^= u16::from(n) << 7 };
		out.extend(buf.map(|b| (b >> 4) as u8));
		buf = [u16::from(n); 4];
		attach(&mut slot.at, &mut pair);
		unsafe { (*slot.at).1 = (*slot.at).0.wrapping_add(n) };
		out.extend([pair.0, pair.1]);
		pair = Reattached(pair.1, n);
	}
	out.extend(buf.map(|b| b as u8));
}

/// A struct JS keeps as an object that only [`reattached`] points into.
struct Reattached(u8, u8);

struct Window<T> {
	at: *mut T,
}

fn attach<T>(at: &mut *mut T, to: &mut T) {
	*at = to;
}

/// A pointer a callee takes through a reference to a cell in a struct, which other
/// functions then write whole through a `&mut` to the cell alone: by assignment, by
/// `mem::replace`, and by `mem::swap`. JS writes each into the cell there, which the
/// pointer sees. Another struct's cell, which no pointer is taken into, is given another
/// cell by `mem::replace` through a `&mut` to the struct.
pub fn part_written(data: &[u8], out: &mut Vec<u8>) {
	let mut refilled = Refilled {
		cell: UnsafeCell::new(0),
	};
	for &n in data {
		out.push(refill(&mut refilled, n));
		let mut w = Written {
			cell: UnsafeCell::new(n),
			tag: !n,
		};
		let p = written_cell(&w.cell);
		let seen = unsafe { *p };
		overwrite(&mut w.cell, n ^ 0x11);
		let after_write = unsafe { *p };
		let old = replace_cell(&mut w.cell, n ^ 0x22);
		let after_replace = unsafe { *p };
		swap_cell(&mut w.cell, n ^ 0x33);
		out.extend([
			seen,
			after_write,
			old,
			after_replace,
			unsafe { *p },
			w.cell.into_inner(),
			w.tag,
		]);
	}
}

/// A struct JS keeps as an object that only [`part_written`] points into.
struct Written {
	cell: UnsafeCell<u8>,
	tag: u8,
}

fn written_cell(c: &UnsafeCell<u8>) -> *mut u8 {
	c.get()
}

fn overwrite(c: &mut UnsafeCell<u8>, n: u8) {
	*c = UnsafeCell::new(n);
}

fn replace_cell(c: &mut UnsafeCell<u8>, n: u8) -> u8 {
	std::mem::replace(c, UnsafeCell::new(n)).into_inner()
}

/// A struct JS keeps as an object, whose cell only [`refill`] replaces.
struct Refilled {
	cell: UnsafeCell<u8>,
}

fn refill(r: &mut Refilled, n: u8) -> u8 {
	std::mem::replace(&mut r.cell, UnsafeCell::new(n)).into_inner()
}

fn swap_cell(c: &mut UnsafeCell<u8>, n: u8) {
	std::mem::swap(c, &mut UnsafeCell::new(n));
}

/// An [`Enclosed`] of pointers to `$to`, a local of the body making them.
macro_rules! aim {
	($to:expr, $n:expr) => {{
		let at: *mut _ = &raw mut $to;
		let held = NonNull::new(at).unwrap();
		Enclosed {
			inner: Aimed {
				at,
				seen: at,
				held,
				maybe: if $n % 2 == 0 { Some(held) } else { None },
				tagged: (at, $n),
				to: if $n % 3 == 0 { To::At(at) } else { To::Nowhere },
				read: read_through,
			},
		}
	}};
}

/// Whether pointers that JS keeps as the pointee, or as where its bytes are, compare as
/// Rust's do: equal however they were made, and apart from another's. Written out for
/// each type, as comparing pointers to a type parameter would compare every type's.
macro_rules! compared {
	($to:expr, $other:expr) => {{
		let other: *mut _ = &raw mut $other;
		let nest = aim!($to, 0u8);
		let a = &nest.inner;
		let none = aim!($to, 1u8).inner.maybe;
		let same = [
			a.at as *const _ == a.seen,
			a.held == NonNull::new(a.at).unwrap(),
			matches!(a.maybe, Some(p) if p == a.held),
			a.tagged.0 == a.at,
			matches!(a.to, To::At(p) if p == a.at),
		];
		let apart = [
			a.at == other,
			matches!(a.maybe, Some(p) if p.as_ptr() == other),
			ptr::eq(a.seen, other),
		];
		// Options of them, `None` and a null pointer included, by `==` and `!=`.
		let null = ptr::null();
		let optional = [
			a.maybe == Some(a.held),
			a.maybe != Some(a.held),
			a.maybe == NonNull::new(other),
			a.maybe == none,
			none == None,
			Some(a.seen) == Some(a.at.cast_const()),
			Some(a.seen) != Some(null),
			Some(null) == Some(ptr::null()),
		];
		let bits = |b: &[bool]| b.iter().fold(0u8, |acc, &b| acc << 1 | u8::from(b));
		[bits(&same), bits(&apart), bits(&optional)]
	}};
}

/// Reads and writes `$to` through an [`Enclosed`] of pointers to it, an `AtomicPtr` to it,
/// and references to it, then reads it.
macro_rules! aimed_at {
	($to:ident, $n:expr, $out:expr) => {{
		let nest = aim!($to, $n);
		through(&nest, $n, $out);
		swapped_in(&raw mut $to, $n, $out);
		lend(&mut $to, $n, $out);
		$out.push($to.get());
	}};
}

/// Pointers a generic type holds to its type parameter, in each shape a field can hold
/// one, instantiated where JS keeps the pointee as an object, a number it copies, an
/// enum it copies, the bytes of a `[u8; N]`, a cell, an array, and another generic
/// type's instance: read and written through, and compared where JS can compare them.
/// An `AtomicPtr` to the parameter and references to it are held beside them.
pub fn aimed(data: &[u8], out: &mut Vec<u8>) {
	for &n in data {
		let mut pair = Paired(n, !n);
		aimed_at!(pair, n, out);
		let mut num = n;
		aimed_at!(num, n, out);
		let mut signed = if n % 2 == 0 { Leaning::Pos(n) } else { Leaning::Neg(n) };
		aimed_at!(signed, n, out);
		let mut bytes = [n; 4];
		aimed_at!(bytes, n, out);
		let mut cell = Cell::new(n);
		aimed_at!(cell, n, out);
		let mut wide = [u16::from(n) << 4; 2];
		aimed_at!(wide, n, out);
		let mut wrap = Housed(n);
		aimed_at!(wrap, n, out);
		let mut other = Paired(!n, n);
		let mut other_bytes = [!n; 4];
		let mut other_wrap = Housed(!n);
		out.extend(compared!(pair, other));
		out.extend(compared!(bytes, other_bytes));
		out.extend(compared!(wrap, other_wrap));
	}
}

/// A pointee that reads and writes as a byte, whatever JS keeps it as.
trait Aim {
	fn made(n: u8) -> Self;
	fn get(&self) -> u8;
	fn put(&mut self, n: u8);
}

/// A struct JS keeps as an object, which pointers to are.
struct Paired(u8, u8);

impl Aim for Paired {
	fn made(n: u8) -> Self {
		Paired(n, n ^ 0x0f)
	}
	fn get(&self) -> u8 {
		self.0 ^ self.1
	}
	fn put(&mut self, n: u8) {
		self.1 = self.0.wrapping_add(n);
	}
}

impl Aim for u8 {
	fn made(n: u8) -> Self {
		n ^ 0x3c
	}
	fn get(&self) -> u8 {
		*self
	}
	fn put(&mut self, n: u8) {
		*self = self.wrapping_mul(3).wrapping_add(n);
	}
}

/// An enum, an object JS copies, so pointers to it are places, as pointers to a number are.
enum Leaning {
	Pos(u8),
	Neg(u8),
}

impl Aim for Leaning {
	fn made(n: u8) -> Self {
		Leaning::Neg(n ^ 0x55)
	}
	fn get(&self) -> u8 {
		match self {
			Leaning::Pos(n) => *n,
			Leaning::Neg(n) => !*n,
		}
	}
	fn put(&mut self, n: u8) {
		*self = match self {
			Leaning::Pos(m) => Leaning::Neg(m.wrapping_add(n)),
			Leaning::Neg(m) => Leaning::Pos(m.wrapping_sub(n)),
		};
	}
}

impl Aim for [u8; 4] {
	fn made(n: u8) -> Self {
		[n, !n, n ^ 1, n ^ 2]
	}
	fn get(&self) -> u8 {
		self[0] ^ self[3]
	}
	fn put(&mut self, n: u8) {
		self[usize::from(n % 4)] = self[0].wrapping_add(n);
	}
}

impl Aim for Cell<u8> {
	fn made(n: u8) -> Self {
		Cell::new(n ^ 0x77)
	}
	fn get(&self) -> u8 {
		Cell::get(self)
	}
	fn put(&mut self, n: u8) {
		self.set(self.get().rotate_left(1) ^ n);
	}
}

impl Aim for [u16; 2] {
	fn made(n: u8) -> Self {
		[u16::from(n) << 4, u16::from(!n) << 4]
	}
	fn get(&self) -> u8 {
		((self[0] ^ self[1]) >> 4) as u8
	}
	fn put(&mut self, n: u8) {
		self[usize::from(n % 2)] ^= u16::from(n) << 4;
	}
}

/// An instance of another generic type.
struct Housed<T>(T);

impl Aim for Housed<u8> {
	fn made(n: u8) -> Self {
		Housed(n ^ 0x99)
	}
	fn get(&self) -> u8 {
		self.0
	}
	fn put(&mut self, n: u8) {
		self.0 = self.0.wrapping_add(n) ^ 0x10;
	}
}

/// Pointers to `T` in each shape a field holds one, copied as Rust copies them.
struct Aimed<T> {
	at: *mut T,
	seen: *const T,
	held: NonNull<T>,
	maybe: Option<NonNull<T>>,
	tagged: (*mut T, u8),
	to: To<T>,
	/// A function taking one, which JS takes as the instance's own pointer.
	read: fn(*const T) -> u8,
}

impl<T> Clone for Aimed<T> {
	fn clone(&self) -> Self {
		*self
	}
}

impl<T> Copy for Aimed<T> {}

enum To<T> {
	At(*mut T),
	Nowhere,
}

impl<T> Clone for To<T> {
	fn clone(&self) -> Self {
		*self
	}
}

impl<T> Copy for To<T> {}

/// A generic type holding another's instance.
struct Enclosed<T> {
	inner: Aimed<T>,
}

/// References to `T`.
struct Borrows<'a, T> {
	to: &'a mut T,
	seen: &'a T,
}

/// An `AtomicPtr` to `T`.
struct Swapped<T> {
	at: AtomicPtr<T>,
}

/// Reads and writes through each of `nest`'s pointers, and through those of a copy of
/// them, whose own tag changes apart from theirs.
fn through<T: Aim>(nest: &Enclosed<T>, n: u8, out: &mut Vec<u8>) {
	let a = &nest.inner;
	let mut copy = *a;
	copy.tagged.1 ^= 0x40;
	let (at, tag) = tagged(copy);
	out.extend([a.tagged.1, tag]);
	unsafe {
		(*at).put(tag);
		out.push((a.read)(a.seen));
		(*a.at).put(n);
		out.push((*a.held.as_ptr()).get());
		if let Some(p) = a.maybe {
			(*p.as_ptr()).put(n ^ 3);
		}
		(*a.tagged.0).put(n.wrapping_add(a.tagged.1));
		if let To::At(p) = a.to {
			(*p).put(!n);
		}
		out.push(u8::from(a.maybe.is_some()) << 1 | u8::from(matches!(a.to, To::At(_))));
		if n % 5 == 0 {
			a.at.write(T::made(n));
		}
		out.push((*a.seen).get());
	}
}

/// What `at` points at, read as a byte.
fn read_through<T: Aim>(at: *const T) -> u8 {
	unsafe { (*at).get() }
}

/// `a`'s tagged pointer and tag, taken apart.
fn tagged<T>(a: Aimed<T>) -> (*mut T, u8) {
	let Aimed { tagged: (at, tag), .. } = a;
	(at, tag)
}

/// Reads and writes through an `AtomicPtr` to what `to` points at, swapped in for null.
fn swapped_in<T: Aim>(to: *mut T, n: u8, out: &mut Vec<u8>) {
	let shared: Swapped<T> = Swapped {
		at: AtomicPtr::new(ptr::null_mut()),
	};
	out.push(u8::from(shared.at.load(SeqCst).is_null()));
	shared.at.store(to, SeqCst);
	unsafe {
		let at = shared.at.load(SeqCst);
		(*at).put(n);
		out.push((*at).get());
	}
	out.push(u8::from(shared.at.swap(ptr::null_mut(), SeqCst).is_null()));
}

/// Reads and writes `to` through references to it and to another `T`.
fn lend<T: Aim>(to: &mut T, n: u8, out: &mut Vec<u8>) {
	let made = T::made(n);
	let lent = Borrows { to, seen: &made };
	lent.to.put(n >> 1);
	out.extend([lent.to.get(), lent.seen.get()]);
}

/// A struct nothing writes into, holding a pair nothing writes into either.
#[derive(Clone, Copy)]
struct Spot {
	at: (u16, u8),
	tag: u8,
}

/// A value of each that holds no copy of its own but is cloned by its parts.
#[derive(Clone)]
struct Named {
	spot: Spot,
	name: String,
}

const ORIGIN: (u16, u8) = (0, 1);

/// Whether pointers to copies of one value nothing writes into are equal: of a pair, a
/// struct, the pair it holds, and bytes, each allocated twice by `Arc`, `Rc`, `Box`, a
/// `Vec`, an `Option`, a constant, a clone, std's copies of each element of a run, a
/// `Cell`'s `get`, and `ptr::read`, and moved out of one and allocated again, by hand or
/// by `make_mut`, of a slice of bytes too, as `ptr::eq`, `addr_eq`, and `as_ptr` compare
/// them; a clone of an `Arc` or `Rc` shares its value.
pub fn distinct(data: &[u8], out: &mut Vec<u8>) {
	for chunk in data.chunks(3) {
		let &[a, b, c] = chunk else {
			continue;
		};
		let pair = (u16::from(a) << 8 | u16::from(b), c);
		let spot = Spot { at: pair, tag: a ^ c };
		let bytes = [a, b, c];
		let (x, y) = (Arc::new(pair), Arc::new(pair));
		let same = Arc::clone(&x);
		let (p, q) = (Rc::new(spot), Rc::new(spot));
		let shared = Rc::clone(&p);
		let (m, n) = (Box::new(pair), Box::new(pair));
		let (k, l) = (Arc::new(bytes), Rc::new(bytes));
		let list = vec![pair; 2];
		let mut pushed = Vec::new();
		pushed.push(spot);
		pushed.push(spot);
		let held = [Some(pair), Some(pair)];
		let (origin, other) = (Box::new(ORIGIN), Box::new(ORIGIN));
		let named = Named {
			spot,
			name: String::from("a"),
		};
		let renamed = named.clone();
		out.extend([
			u8::from(ptr::eq(&*x, &*y)),
			u8::from(ptr::eq(&*x, &*same)),
			u8::from(Arc::as_ptr(&x) == Arc::as_ptr(&y)),
			u8::from(ptr::addr_eq(Arc::as_ptr(&x), Arc::as_ptr(&same))),
			u8::from(ptr::eq(&*x, &pair)),
			u8::from(ptr::eq(&*p, &*q)),
			u8::from(ptr::eq(&*p, &*shared)),
			u8::from(ptr::eq(&p.at, &q.at)),
			u8::from(ptr::eq(&p.at, &spot.at)),
			u8::from(ptr::addr_eq(Rc::as_ptr(&p), Rc::as_ptr(&shared))),
			u8::from(ptr::eq(&*m, &*n)),
			u8::from(ptr::eq(&*m, &*x)),
			u8::from(ptr::addr_eq(Arc::as_ptr(&k), Rc::as_ptr(&l))),
			u8::from(ptr::eq(&*k, &bytes)),
			u8::from(ptr::eq(&list[0], &list[1])),
			u8::from(ptr::eq(&pushed[0], &pushed[1])),
			u8::from(ptr::eq(&pushed[0].at, &spot.at)),
			u8::from(ptr::eq(held[0].as_ref().unwrap(), held[1].as_ref().unwrap())),
			u8::from(ptr::eq(&*origin, &*other)),
			u8::from(ptr::eq(&named.spot, &renamed.spot)),
			u8::from(ptr::eq(&named.spot.at, &renamed.spot.at)),
		]);
		// Moved out and allocated again.
		let moved = *m;
		let again = Box::new(moved);
		let swapped = Box::new(*n);
		let alone = Arc::into_inner(x).unwrap_or(ORIGIN);
		let unwrapped = Arc::new(alone);
		let inner = Rc::into_inner(q).map_or(spot, |s| s);
		let rewrapped = Rc::new(inner);
		out.extend([
			u8::from(ptr::eq(&*again, &*m)),
			u8::from(ptr::eq(&*again, &moved)),
			u8::from(ptr::eq(&*n, &*again)),
			u8::from(ptr::eq(&*swapped, &*n)),
			u8::from(ptr::eq(&*unwrapped, &*y)),
			u8::from(ptr::eq(&*unwrapped, &*same)),
			u8::from(ptr::addr_eq(Arc::as_ptr(&unwrapped), Arc::as_ptr(&y))),
			u8::from(ptr::eq(&*rewrapped, &*p)),
			u8::from(ptr::eq(&rewrapped.at, &p.at)),
			u8::from(Rc::strong_count(&p) as u8),
			swapped.1 ^ unwrapped.1 ^ rewrapped.tag ^ named.name.len() as u8,
		]);
		// Copied element by element by std.
		let repeated = [pair; 2].to_vec();
		let copied: Vec<(u16, u8)> = list.iter().copied().collect();
		let cloned: Vec<Spot> = pushed.iter().cloned().collect();
		let mut extended = list.to_vec();
		extended.extend_from_slice(&list);
		extended.push(pair);
		extended.push(pair);
		extended.extend(list.iter());
		let opted = held[0].as_ref().copied();
		let cell = Cell::new(pair);
		let (got, again_got) = (cell.get(), cell.get());
		let (read, reread) = unsafe { (ptr::read(&pair), ptr::read(&*m)) };
		out.extend([
			u8::from(ptr::eq(&repeated[0], &repeated[1])),
			u8::from(ptr::eq(&copied[0], &list[0])),
			u8::from(ptr::eq(&cloned[1].at, &pushed[1].at)),
			u8::from(ptr::eq(&extended[0], &list[0])),
			u8::from(ptr::eq(&extended[2], &list[0])),
			u8::from(ptr::eq(&extended[4], &extended[5])),
			u8::from(ptr::eq(&extended[6], &list[0])),
			u8::from(ptr::eq(opted.as_ref().unwrap(), held[0].as_ref().unwrap())),
			u8::from(ptr::eq(&got, &again_got)),
			u8::from(ptr::eq(&read, &pair)),
			u8::from(ptr::eq(&reread, &*m)),
		]);
		// Moved to an allocation of its own by `make_mut` while a `Weak` alone shares it,
		// or cloned into one while another handle does.
		let mut maybe = Rc::new(Some(pair));
		let mut spotted = Arc::new(spot);
		let weak = (c & 1 != 0).then(|| (Rc::downgrade(&maybe), Arc::downgrade(&spotted)));
		let strong = (c & 2 != 0).then(|| (Rc::clone(&maybe), Arc::clone(&spotted)));
		let mut run: Arc<[u8]> = Arc::from(vec![a, b, c]);
		let run_weak = (c & 4 != 0).then(|| Arc::downgrade(&run));
		Arc::make_mut(&mut run)[0] ^= c;
		out.extend_from_slice(&run);
		drop(run_weak);
		let was: *const (u16, u8) = maybe.as_ref().as_ref().unwrap();
		let was_at: *const (u16, u8) = &spotted.at;
		let was_spot = Arc::as_ptr(&spotted);
		let now: *const (u16, u8) = Rc::make_mut(&mut maybe).as_ref().unwrap();
		let now_at: *const (u16, u8) = &Arc::make_mut(&mut spotted).at;
		out.extend([
			u8::from(was == now),
			u8::from(was_at == now_at),
			u8::from(ptr::eq(was_spot, &*spotted)),
			u8::from(weak.is_some_and(|(r, a)| r.upgrade().is_some() || a.upgrade().is_some())),
			u8::from(strong.is_some_and(|(r, a)| {
				ptr::eq(r.as_ref().as_ref().unwrap(), maybe.as_ref().as_ref().unwrap()) || ptr::eq(&a.at, &spotted.at)
			})),
		]);
	}
}

/// What an `Arc` or `Rc` holds through its raw pointer, which notes its drop in the
/// `Cell` beside it.
struct Counted<'a> {
	id: u8,
	dropped: &'a Cell<u8>,
}

impl Drop for Counted<'_> {
	fn drop(&mut self) {
		self.dropped.set(self.dropped.get().wrapping_add(self.id));
	}
}

/// What two `Arc`s hold copies of, which nothing compares pointers to.
#[derive(Clone, Copy)]
struct Twin(u8, u8);

/// `Arc`s and `Rc`s of a number, a struct, `()`, and a value that drops, through the
/// pointers `into_raw` and `as_ptr` give: counted up and down by `increment_strong_count`
/// and `decrement_strong_count`, the value dropping with the last, taken back by
/// `from_raw`, cast to `*const ()` and back first, and compared, as `strong_count` reads.
pub fn raw_arcs(data: &[u8], out: &mut Vec<u8>) {
	let dropped = Cell::new(0);
	let num = Arc::new(7u32);
	let duo = Rc::new(Duo(1, 2));
	let unit = Arc::new(());
	let other = Arc::new(());
	let held = Rc::new(Counted {
		id: 3,
		dropped: &dropped,
	});
	let num_at = Arc::as_ptr(&num);
	let (num_raw, duo_raw) = (Arc::into_raw(Arc::clone(&num)), Rc::into_raw(Rc::clone(&duo)));
	let (unit_raw, other_raw) = (Arc::into_raw(Arc::clone(&unit)), Arc::into_raw(other));
	let held_raw = Rc::into_raw(held);
	out.extend([ptr::eq(duo_raw, &*duo), unit_raw == other_raw].map(u8::from));
	out.extend(
		[
			unit_raw == Arc::as_ptr(&unit),
			duo_raw.cast::<()>() == Rc::as_ptr(&duo).cast(),
		]
		.map(u8::from),
	);
	for &byte in data {
		match byte % 6 {
			// Moved off and back, which is the same address.
			0 => unsafe { Arc::increment_strong_count(num_raw.add(1).sub(1)) },
			// Never the count `num` and `num_raw` hold.
			1 if Arc::strong_count(&num) > 2 => {
				let back = usize::from(byte >> 3);
				unsafe { Arc::decrement_strong_count(num_at.wrapping_add(back).wrapping_sub(back)) }
			}
			2 => unsafe { Rc::increment_strong_count(duo_raw) },
			// Through `*const ()`, as a waker's data is.
			3 if dropped.get() == 0 => unsafe { Rc::increment_strong_count(held_raw.cast::<()>().cast::<Counted>()) },
			4 if dropped.get() == 0 => unsafe { Rc::decrement_strong_count(held_raw as *const () as *const Counted) },
			1 | 3 | 4 => {}
			_ => {
				let taken = unsafe { Rc::from_raw(duo_raw) };
				out.push(Rc::strong_count(&taken) as u8);
				mem::forget(taken);
			}
		}
		// Each count as a borrowed `Arc` reads it, until the value drops.
		if dropped.get() == 0 {
			let held = mem::ManuallyDrop::new(unsafe { Rc::from_raw(held_raw) });
			out.extend([Rc::strong_count(&held) as u8, held.id]);
		}
		out.extend([
			Arc::strong_count(&num) as u8,
			Rc::strong_count(&duo) as u8,
			dropped.get(),
		]);
	}
	// Taken back, and dropped, as many times as they were counted.
	let num_back = unsafe { Arc::from_raw(num_raw) };
	let duo_back = unsafe { Rc::from_raw(duo_raw) };
	let unit_back = unsafe { Arc::from_raw(unit_raw.cast::<()>()) };
	out.extend(
		[
			Arc::ptr_eq(&num_back, &num),
			Rc::ptr_eq(&duo_back, &duo),
			Arc::ptr_eq(&unit_back, &unit),
		]
		.map(u8::from),
	);
	out.extend([*num_back as u8, duo_back.1, Arc::strong_count(&unit) as u8]);
	drop((num_back, duo_back, unit_back));
	out.extend([
		Arc::strong_count(&num) as u8,
		Rc::strong_count(&duo) as u8,
		Arc::strong_count(&unit) as u8,
	]);
	drop(unsafe { Arc::from_raw(other_raw) });
	// Two `Arc`s of copies of one value, which are two allocations though no pointers to a
	// `Twin` compare.
	let twin = Twin(data.len() as u8, 1);
	let (a, b) = (Arc::new(twin), Rc::new(twin));
	let (c, d) = (Arc::new(twin), Rc::new(twin));
	let (p, q) = (Arc::into_raw(Arc::clone(&a)), Rc::into_raw(Rc::clone(&b)));
	let (r, s) = (Arc::into_raw(Arc::clone(&c)), Rc::into_raw(Rc::clone(&d)));
	unsafe {
		Arc::increment_strong_count(p);
		Rc::increment_strong_count(q);
	}
	out.extend([a.as_ref(), &b, &c, &d].map(|t| t.0 ^ t.1));
	out.extend(
		[
			Arc::strong_count(&a),
			Rc::strong_count(&b),
			Arc::strong_count(&c),
			Rc::strong_count(&d),
		]
		.map(|n| n as u8),
	);
	unsafe {
		drop((Arc::from_raw(p), Arc::from_raw(p), Rc::from_raw(q), Rc::from_raw(q)));
		drop((Arc::from_raw(r), Rc::from_raw(s)));
	}
	while dropped.get() == 0 {
		unsafe { Rc::decrement_strong_count(held_raw) };
		out.push(dropped.get());
	}
	out.push(dropped.get());
}

/// A waker's data, an `Arc<AtomicBool>`'s raw pointer, which waking sets.
unsafe fn flag_clone(data: *const ()) -> RawWaker {
	unsafe { Arc::increment_strong_count(data.cast::<AtomicBool>()) };
	RawWaker::new(data, &FLAG)
}

unsafe fn flag_wake(data: *const ()) {
	unsafe { Arc::from_raw(data as *const AtomicBool) }.store(true, SeqCst);
}

unsafe fn flag_wake_by_ref(data: *const ()) {
	unsafe { &*data.cast::<AtomicBool>() }.store(true, SeqCst);
}

unsafe fn flag_drop(data: *const ()) {
	unsafe { Arc::decrement_strong_count(data as *const AtomicBool) };
}

static FLAG: RawWakerVTable = RawWakerVTable::new(flag_clone, flag_wake, flag_wake_by_ref, flag_drop);

/// A waker that sets `flag`, holding a count of it.
fn flag_waker(flag: &Arc<AtomicBool>) -> Waker {
	let data = Arc::into_raw(Arc::clone(flag)) as *const ();
	unsafe { Waker::from_raw(RawWaker::new(data, &FLAG)) }
}

/// A null pointer to a number, as a constant.
const NULL_NUM: *const u32 = ptr::null();

/// The raw pointers of `Arc`s and `Rc`s of values JS copies: a number's remade from a
/// shared reference to its value, counted up and down, cloned, and taken back through it,
/// and an `Arc<AtomicBool>`'s cast to `*const ()` and back, as a waker's data, which its
/// vtable counts, wakes, and drops. Each compares as the pointer `as_ptr` gives, and a
/// null one, made here or a constant, as null, through casts to and from `MaybeUninit`.
pub fn raw_places(data: &[u8], out: &mut Vec<u8>) {
	let num = Arc::new(data.len() as u32);
	let rc = Rc::new(9u16);
	let flag = Arc::new(AtomicBool::new(false));
	let mut wakers = vec![flag_waker(&flag)];
	for &byte in data {
		let num_at = ptr::from_ref(&*num);
		let rc_at = &*rc as *const u16;
		match byte % 8 {
			0 => unsafe { Arc::increment_strong_count(num_at) },
			1 if Arc::strong_count(&num) > 1 => unsafe { Arc::decrement_strong_count(num_at) },
			2 => unsafe { Rc::increment_strong_count(rc_at) },
			3 if Rc::strong_count(&rc) > 1 => drop(unsafe { Rc::from_raw(rc_at) }),
			// A clone of the `Arc` `from_raw` takes back, which the count holds after.
			4 => {
				let taken = mem::ManuallyDrop::new(unsafe { Arc::from_raw(NonNull::from(&*num).as_ptr()) });
				mem::forget(Arc::clone(&taken));
			}
			5 => wakers.push(wakers[usize::from(byte) % wakers.len()].clone()),
			6 if wakers.len() > 1 => drop(wakers.swap_remove(usize::from(byte) % wakers.len())),
			_ => {
				flag.store(false, SeqCst);
				match byte & 8 {
					0 => wakers[0].wake_by_ref(),
					_ => wakers.pop().unwrap().wake(),
				}
				if wakers.is_empty() {
					wakers.push(flag_waker(&flag));
				}
			}
		}
		let taken = mem::ManuallyDrop::new(unsafe { Arc::from_raw(ptr::from_ref(&*num)) });
		out.extend([
			Arc::strong_count(&num) as u8,
			u8::from(Arc::ptr_eq(&taken, &num)),
			Rc::strong_count(&rc) as u8,
			Arc::strong_count(&flag) as u8,
			u8::from(flag.load(SeqCst)),
		]);
		// Erased, the pointers compare as `as_ptr`'s, which is the value's one place.
		out.extend(
			[
				num_at as *const () == Arc::as_ptr(&num).cast(),
				rc_at.cast::<()>() == Rc::as_ptr(&rc) as *const (),
				Arc::as_ptr(&flag).cast::<()>() == wakers[0].data(),
				ptr::from_ref(&*flag).cast::<()>() == wakers[0].data(),
				ptr::null::<u32>() as *const () == ptr::null(),
				NULL_NUM.cast::<()>() == ptr::null(),
				ptr::null::<MaybeUninit<u32>>().cast::<u32>().cast::<()>() == ptr::null(),
				Arc::as_ptr(&num).cast::<MaybeUninit<u32>>().cast::<()>() == Arc::as_ptr(&num).cast(),
			]
			.map(u8::from),
		);
	}
	drop(wakers);
	out.push(Arc::strong_count(&flag) as u8);
	while Arc::strong_count(&num) > 1 {
		unsafe { Arc::decrement_strong_count(Arc::as_ptr(&num)) };
	}
	while Rc::strong_count(&rc) > 1 {
		unsafe { Rc::decrement_strong_count(&*rc) };
	}
	out.extend([Arc::strong_count(&num) as u8, Rc::strong_count(&rc) as u8, *rc as u8]);
	// A pointer remade from a reference borrowed through another reference, which the body
	// points elsewhere after: still at the value it borrowed.
	let (first, second) = (Shelf(Arc::new(1)), Shelf(Arc::new(2)));
	let mut shelf = &first;
	let value = &*shelf.0;
	if data.first().is_some_and(|b| b % 2 == 0) {
		shelf = &second;
	}
	let p = ptr::from_ref(value);
	out.extend([unsafe { *p } as u8, *shelf.0 as u8]);
	// And one a `&mut` to it points elsewhere.
	let mut shelf = &first;
	let value = &*shelf.0;
	if data.len() % 3 == 0 {
		let _ = mem::replace(&mut shelf, &second);
	}
	let p = ptr::from_ref(value);
	out.extend([unsafe { *p } as u8, *shelf.0 as u8]);
}

/// What holds an `Arc` a reference to it borrows through.
struct Shelf(Arc<u32>);

/// Pointers to arrays of other values than bytes, made of the array, of references to it
/// and its parts, by `first_chunk`, `split_first_chunk`, a subslice pattern, and
/// `array::from_mut`: compared where they point, and written through.
pub fn arrays(data: &[u8], out: &mut Vec<u8>) {
	for &n in data {
		let w = u32::from(n) * 257;
		let mut a = [w, w ^ 1, w ^ 2, w ^ 3];
		let b = a;
		let p: *const [u32; 4] = &a;
		let s: &[u32] = &a;
		let first: *const [u32; 2] = s.first_chunk().unwrap();
		let again: *const [u32; 2] = a[..3].first_chunk().unwrap();
		let (_, rest) = a.split_first_chunk::<1>().unwrap();
		let middle: *const [u32; 2] = rest.first_chunk().unwrap();
		out.extend([
			u8::from(ptr::eq(p, &a)),
			u8::from(ptr::eq(p, &b)),
			u8::from(ptr::eq(first, again)),
			u8::from(ptr::eq(first, middle)),
			u8::from(ptr::eq(p, ptr::null())),
		]);
		let chunk = a.first_chunk_mut::<2>().unwrap();
		chunk[1] ^= 0x8000;
		let made: *const [u32; 2] = chunk;
		out.push(u8::from(ptr::eq(made, first)) | u8::from(ptr::eq(made, middle)) << 1);
		if let [x, .., y] = &mut a {
			*x ^= 1;
			mem::swap(x, y);
		}
		let one = std::array::from_mut(&mut a[2]);
		one[0] ^= 0x10;
		let whole: &mut [u32; 4] = &mut a;
		let r: *const [u32; 4] = whole;
		out.push(u8::from(ptr::eq(r, p)));
		out.extend(a.iter().map(|&x| (x ^ x >> 8) as u8));
		// A read through a pointer of arrays of arrays, a value of its own.
		let nested = [[w, 1], [2, 3]];
		let mut read = unsafe { ptr::read(&nested) };
		read[0][0] ^= 0xff;
		out.extend([nested[0][0] as u8, read[0][0] as u8]);
		out.extend(runs(n));
		out.extend(lent(n));
	}
}

/// Pointers to arrays in a `Vec`, compared where they point.
fn runs(n: u8) -> [u8; 4] {
	let mut v = vec![
		[n, n ^ 1].map(|b| char::from(b).to_string()),
		[String::new(), String::from("x")],
	];
	let first: *const [String; 2] = &v[0];
	let last: *const [String; 2] = v.last().unwrap();
	v[1][0].push('y');
	[
		u8::from(ptr::eq(first, &v[0])),
		u8::from(ptr::eq(first, last)),
		u8::from(ptr::eq(last, &v[1])),
		v[1][0].len() as u8,
	]
}

/// Pointers to arrays each way the language and std lend one, the runs of a shared
/// slice's `as_chunks` and `as_rchunks` included, compared where they point.
fn lent(n: u8) -> Vec<u8> {
	let w = u32::from(n);
	let a = [w, w + 1, w + 2, w + 3];
	let v = vec![w, w + 1, w + 2, w + 3];
	let mut o = Vec::new();
	let (chunks, _) = a.as_chunks::<2>();
	o.push(u8::from(ptr::eq(
		chunks.first().unwrap(),
		a.first_chunk::<2>().unwrap(),
	)));
	let (_, back) = a.as_rchunks::<2>();
	o.push(u8::from(ptr::eq(back.last().unwrap(), a.last_chunk::<2>().unwrap())));
	let (vc, _) = v.as_chunks::<2>();
	o.push(u8::from(ptr::eq(
		vc.iter().nth(1).unwrap(),
		v.last_chunk::<2>().unwrap(),
	)));
	o.push(u8::from(ptr::eq(
		a[1..3].first_chunk::<2>().unwrap(),
		a[1..].first_chunk::<2>().unwrap(),
	)));
	o.push(u8::from(ptr::eq(
		v[1..3].first_chunk::<2>().unwrap(),
		v[1..].first_chunk::<2>().unwrap(),
	)));
	o.push(u8::from(ptr::eq(
		v.windows(2).nth(1).unwrap().first_chunk::<2>().unwrap(),
		v[1..].first_chunk::<2>().unwrap(),
	)));
	o.push(u8::from(ptr::eq(
		v.chunks(2).nth(1).unwrap().first_chunk::<2>().unwrap(),
		v.last_chunk::<2>().unwrap(),
	)));
	o.push(u8::from(ptr::eq(
		a.chunks_exact(2).next().unwrap().first_chunk::<2>().unwrap(),
		a.first_chunk::<2>().unwrap(),
	)));
	let (l, _) = a.split_at(2);
	o.push(u8::from(ptr::eq(
		l.first_chunk::<2>().unwrap(),
		a.first_chunk::<2>().unwrap(),
	)));
	let pairs = vec![[w, 1], [2, 3]];
	o.push(u8::from(ptr::eq(pairs.iter().next().unwrap(), pairs.first().unwrap())));
	let boxed = Box::new([w, 2]);
	o.push(u8::from(ptr::eq(&*boxed, &*boxed)));
	let opt = Some([w, 2]);
	o.push(u8::from(ptr::eq(opt.as_ref().unwrap(), opt.as_ref().unwrap())));
	let rc = Rc::new([w, 2]);
	let rc2 = Rc::clone(&rc);
	o.push(u8::from(ptr::eq(&*rc, &*rc2)));
	let cell = std::cell::RefCell::new([w, 2]);
	let p1: *const [u32; 2] = &*cell.borrow();
	let p2: *const [u32; 2] = &*cell.borrow();
	o.push(u8::from(ptr::eq(p1, p2)));
	let mut map = std::collections::HashMap::new();
	map.insert(1u8, [w, 2]);
	o.push(u8::from(ptr::eq(map.get(&1).unwrap(), map.get(&1).unwrap())));
	let st = Holder { arr: [w, 5] };
	o.push(u8::from(ptr::eq(&st.arr, &st.arr)));
	let held = Arc::new([w, 2]);
	let un: Arc<[u32]> = held.clone();
	o.push(u8::from(ptr::eq(un.first_chunk::<2>().unwrap(), &*held)));
	let nested = [[w, 1], [2, 3]];
	o.push(u8::from(ptr::eq(nested.last().unwrap(), nested.iter().nth(1).unwrap())));
	o
}

/// What holds an array in a field.
struct Holder {
	arr: [u32; 2],
}

/// What smallvec's `Drain` is: the elements of a slice nothing will drop again, each read
/// out through the reference its iterator lends.
struct Drained<'a, T> {
	iter: std::slice::Iter<'a, T>,
}

impl<T> Iterator for Drained<'_, T> {
	type Item = T;

	fn next(&mut self) -> Option<T> {
		self.iter.next().map(|r| unsafe { ptr::read(r) })
	}
}

impl<T> DoubleEndedIterator for Drained<'_, T> {
	fn next_back(&mut self) -> Option<T> {
		self.iter.next_back().map(|r| unsafe { ptr::read(r) })
	}
}

/// `v`'s elements, read out from both ends as `bits` picks, which nothing drops in `v`.
fn drained<T>(v: Vec<T>, bits: u8) -> Vec<T> {
	let v = mem::ManuallyDrop::new(v);
	let mut iter = Drained { iter: v.iter() };
	let mut out = Vec::new();
	let mut i = 0;
	loop {
		let next = match bits >> (i % 8) & 1 {
			0 => iter.next(),
			_ => iter.next_back(),
		};
		let Some(next) = next else { break };
		out.push(next);
		i += 1;
	}
	// Its elements moved out, its room is freed without dropping them again.
	let mut v = mem::ManuallyDrop::into_inner(v);
	unsafe { v.set_len(0) };
	out
}

/// What smallvec's `retain` is: each element lent as a `&mut`, which a pointer from it then
/// drops or moves to the front.
fn retained<T>(v: &mut Vec<T>, mut keep: impl FnMut(&mut T) -> bool) {
	let len = v.len();
	let p = v.as_mut_ptr();
	let mut write = 0;
	for read in 0..len {
		let cur = unsafe { &mut *p.add(read) };
		if !keep(cur) {
			unsafe { ptr::drop_in_place(cur) };
		} else {
			unsafe { ptr::copy(cur, p.add(write), 1) };
			write += 1;
		}
	}
	// What is past the length, dropped or moved, which nothing drops again.
	unsafe { v.set_len(write) };
}

/// The length of the `Vec` a pointer from a reference the body is given points at.
fn vec_len(v: &Vec<u8>) -> usize {
	let p = &raw const *v;
	unsafe { (*p).len() }
}

/// `with` written whole through a pointer from a `&mut` the body is given, made each way
/// std makes one, which its caller's `Vec` holds after.
fn refilled(v: &mut Vec<u8>, with: Vec<u8>, how: u8) {
	let p = match how % 4 {
		0 => v as *mut Vec<u8>,
		1 => ptr::from_mut(v),
		2 => NonNull::from(v).as_ptr(),
		_ => Into::<NonNull<Vec<u8>>>::into(v).as_ptr(),
	};
	unsafe { *p = with };
	unsafe { (*p).push(how) };
}

/// What the `Vec` a pointer from a `&mut` the body is given points at held, moved out
/// each way std moves one out as `with` is written whole in its place, and then swapped
/// with `w` through pointers from `&mut`s to each.
fn replaced(v: &mut Vec<u8>, w: &mut Vec<u8>, with: Vec<u8>, how: u8) -> Vec<u8> {
	let p = ptr::from_mut(v);
	let old = match how % 3 {
		0 => unsafe { ptr::replace(p, with) },
		1 => {
			let old = unsafe { ptr::read(p) };
			unsafe { ptr::write(p, with) };
			old
		}
		_ => {
			let mut slot = MaybeUninit::<Vec<u8>>::uninit();
			unsafe { ptr::copy_nonoverlapping(p, slot.as_mut_ptr(), 1) };
			unsafe { ptr::write(p, with) };
			unsafe { slot.assume_init() }
		}
	};
	unsafe { ptr::swap(ptr::from_mut(v), ptr::from_mut(w)) };
	old
}

/// The value a pointer from a shared reference the body is given points at, made each way
/// std makes one.
fn behind(r: &u32, how: u8) -> u32 {
	let p = match how % 3 {
		0 => r as *const u32,
		1 => ptr::from_ref(r),
		_ => NonNull::from(r).as_ptr(),
	};
	unsafe { ptr::read(p) }
}

/// Whether the `Option` a pointer from a shared reference the body is given points at
/// holds a cell, and the number in it, read through the pointer made each way std makes
/// one.
fn cell_behind(r: &Option<Cell<u32>>, how: u8) -> u32 {
	let p = match how % 2 {
		0 => &raw const *r,
		_ => ptr::from_ref(r),
	};
	match unsafe { &*p } {
		Some(cell) => cell.get() + 1,
		None => 0,
	}
}

/// Raw pointers from references: smallvec's `Drain` and `retain`, over a number, library
/// types, and an `Option`; pointers from a reference the body borrowed, which move within
/// the array it borrowed from; from references the body is given, to a number, to an
/// `Option` of a cell, and to a `Vec`, read, written, moved out of, and swapped whole; and
/// from references to an `Arc`'s value, measured against each other.
pub fn behind_refs(data: &[u8], out: &mut Vec<u8>) {
	let bits = data.first().copied().unwrap_or(0);
	let nums: Vec<u32> = data.iter().map(|&b| u32::from(b) * 3).collect();
	out.extend(drained(nums.clone(), bits).iter().map(|&n| n as u8));
	let runs: Vec<Vec<u8>> = data.chunks(3).map(<[u8]>::to_vec).collect();
	for r in drained(runs.clone(), bits) {
		out.push(r.len() as u8);
		out.extend(r);
	}
	let opts: Vec<Option<u16>> = data.iter().map(|&b| (b % 3 != 0).then_some(u16::from(b))).collect();
	for o in drained(opts, bits) {
		out.extend(o.map_or([0xff, 0xff], u16::to_be_bytes));
	}

	let mut nums = nums;
	retained(&mut nums, |n| {
		*n += 1;
		*n % 2 == 0
	});
	out.extend(nums.iter().map(|&n| n as u8));
	let mut runs = runs;
	retained(&mut runs, |r| {
		r.push(7);
		r[0] % 2 == 0
	});
	out.extend(runs.iter().flatten());
	// Each element it drops, dropped once.
	let dropped = Cell::new(0);
	let mut counted: Vec<Rc<Counted>> = data
		.iter()
		.map(|&id| Rc::new(Counted { id, dropped: &dropped }))
		.collect();
	retained(&mut counted, |c| c.id % 2 == 0);
	out.push(dropped.get());
	out.extend(counted.iter().map(|c| c.id));
	drop(counted);
	out.push(dropped.get());
	// A `Weak` holds only its pointer to what it shares, which a read out copies as is, with
	// one written whole in place.
	let arc = Arc::new(u32::from(bits));
	let weaks: Vec<std::sync::Weak<u32>> = data
		.iter()
		.map(|&b| match b % 2 {
			0 => Arc::downgrade(&arc),
			_ => std::sync::Weak::new(),
		})
		.collect();
	for w in drained(weaks.clone(), bits) {
		out.push(w.upgrade().map_or(0xff, |n| *n as u8));
	}
	let mut weaks = weaks;
	let mut i = 0u8;
	retained(&mut weaks, |w| {
		i += 1;
		if i % 3 == 0 {
			*w = Arc::downgrade(&arc);
		}
		i % 2 == 0
	});
	out.extend(weaks.iter().map(|w| w.upgrade().map_or(0xff, |n| *n as u8)));
	out.push(Arc::weak_count(&arc) as u8);
	drop(weaks);
	out.push(Arc::weak_count(&arc) as u8);

	// From a reference the body borrowed, where the element is.
	let mut nums: Vec<u32> = data.iter().map(|&b| u32::from(b)).collect();
	if let [_, _, ..] = nums[..] {
		let first = &nums[0] as *const u32;
		let last = &nums[nums.len() - 1] as *const u32;
		out.push(unsafe { last.offset_from(first) } as u8);
		for i in 0..nums.len() {
			out.push(unsafe { *first.add(i) } as u8);
		}
		let second = ptr::from_ref(&nums[1]);
		out.push(unsafe { *second.sub(1) } as u8);
		let m = &mut nums[1] as *mut u32;
		unsafe { *m.sub(1) = 9 };
		out.push(nums[0] as u8);
	}

	let mut v = data.to_vec();
	out.push(vec_len(&v) as u8);
	refilled(&mut v, data.iter().rev().copied().collect(), bits);
	out.extend(&v);
	for how in 0..3 {
		let mut w: Vec<u8> = data.iter().map(|b| b ^ how).collect();
		let old = replaced(&mut v, &mut w, vec![bits; usize::from(how) + 1], how);
		out.extend(&old);
		out.extend(&v);
		out.extend(&w);
		out.push(0xff);
	}
	for (i, &b) in data.iter().enumerate() {
		out.push(behind(&u32::from(b), i as u8) as u8);
		let cell = (b % 3 != 0).then(|| Cell::new(u32::from(b)));
		out.push(cell_behind(&cell, b) as u8);
	}
	// Pointers from references to an `Arc`'s value, which is its one place.
	let arc = Arc::new(u32::from(bits));
	let p = ptr::from_ref(&*arc);
	let q = ptr::from_ref(&*arc);
	out.push(unsafe { p.offset_from(q) } as u8);
	out.push(unsafe { *q } as u8);
	let n: NonNull<u32> = (&*arc).into();
	out.push(unsafe { n.as_ptr().offset_from(p) } as u8);
	// A conversion of the crate's own, which runs as written.
	let halves = Halves32 {
		first: u32::from(bits),
		second: u32::from(bits) ^ 0x55,
	};
	let second: NonNull<u32> = (&halves).into();
	out.push(unsafe { *second.as_ptr() } as u8);
	// A pointer to a cell from a shared reference to it, read and written in place by
	// `UnsafeCell::raw_get` after a call between.
	let cell = UnsafeCell::new(u32::from(bits));
	let shared = &cell;
	let at_cell: *const UnsafeCell<u32> = shared;
	let len = vec_len(&v);
	unsafe { *UnsafeCell::raw_get(at_cell) += len as u32 };
	out.push(unsafe { *cell.get() } as u8);
	// A pointer one past a variable, stored whole through a pointer to a pointer, and one
	// moved off a value of no bytes and borrowed: valid, as neither reads past its value.
	let lone = u32::from(bits);
	let at = &raw const lone;
	let mut end = ptr::null::<u32>();
	unsafe { ptr::write(&raw mut end, at.add(1)) };
	out.push(unsafe { end.offset_from(at) } as u8);
	let none = Nothing;
	let past = unsafe { (&raw const none).add(1) };
	out.push(nothing_tag(unsafe { &*past }, bits));
}

/// A value of no bytes, whose every pointer is where it was.
struct Nothing;

/// `bits`, given beside a `Nothing` borrowed through a pointer moved off one.
#[inline(never)]
fn nothing_tag(nothing: &Nothing, bits: u8) -> u8 {
	let Nothing = nothing;
	bits ^ 0x3c
}

/// Two numbers, the second of which a pointer from a reference to them points at.
struct Halves32 {
	first: u32,
	second: u32,
}

impl From<&Halves32> for NonNull<u32> {
	fn from(halves: &Halves32) -> Self {
		let _ = halves.first;
		NonNull::from(&halves.second)
	}
}

/// `with` written whole into the `Vec` `r` is, in place, dropping nothing, as a body that
/// can't see where it is kept writes it.
#[inline(never)]
fn refill_in(r: &mut Vec<u8>, with: Vec<u8>) {
	unsafe { ptr::write(r, with) };
}

/// What the `Vec` `v` held, moved out through a pointer made each way `how` picks: from a
/// shared reborrow, cast to one to a `MaybeUninit` or a `ManuallyDrop` of it, or swapped
/// through such a cast with another; then `with` is written whole into `v` in place.
fn moved_vec(v: &mut Vec<u8>, with: Vec<u8>, how: u8) -> Vec<u8> {
	let old = match how % 4 {
		0 => unsafe { ptr::read(&*v as *const Vec<u8>) },
		1 => unsafe { ptr::read(ptr::from_mut(v).cast::<MaybeUninit<Vec<u8>>>()).assume_init() },
		2 => mem::ManuallyDrop::into_inner(unsafe { ptr::read(ptr::from_mut(v).cast::<mem::ManuallyDrop<Vec<u8>>>()) }),
		_ => {
			let mut other = MaybeUninit::new(Vec::new());
			unsafe { ptr::swap(ptr::from_mut(v).cast::<MaybeUninit<Vec<u8>>>(), &mut other) };
			unsafe { other.assume_init() }
		}
	};
	refill_in(v, with);
	old
}

/// A `Vec` with a tag, which a pointer to one moves out whole.
struct TaggedVec {
	v: Vec<u8>,
	tag: u8,
}

/// What an `Option`, a tuple, and a struct each holding a `Vec` held, moved out through
/// pointers from `&mut`s to them: read, or copied, replaced, and swapped, as `how` picks;
/// then `with` is written whole into each `Vec` they hold, in place.
fn moved_nested(
	o: &mut Option<Vec<u8>>,
	t: &mut (Vec<u8>, u8),
	s: &mut TaggedVec,
	with: &[u8],
	how: u8,
) -> (Option<Vec<u8>>, (Vec<u8>, u8), TaggedVec) {
	let old = match how % 2 {
		0 => unsafe { (ptr::read(o), ptr::read(t), ptr::read(s)) },
		_ => unsafe {
			let mut slot = MaybeUninit::uninit();
			ptr::copy_nonoverlapping(ptr::from_mut(o), slot.as_mut_ptr(), 1);
			let old_t = ptr::replace(ptr::from_mut(t), (Vec::new(), how));
			let mut old_s = TaggedVec {
				v: Vec::new(),
				tag: how,
			};
			ptr::swap(ptr::from_mut(s), &mut old_s);
			(slot.assume_init(), old_t, old_s)
		},
	};
	if let Some(v) = o {
		refill_in(v, with.to_vec());
	}
	refill_in(&mut t.0, with.to_vec());
	refill_in(&mut s.v, with.to_vec());
	old
}

/// The `Option` of a cell a shared reference points at, moved out through a pointer from
/// it, whose cell is then set.
fn cell_moved(r: &Option<Cell<u32>>) -> Option<Cell<u32>> {
	let old = unsafe { ptr::read(r) };
	if let Some(cell) = &old {
		cell.set(cell.get() + 5);
	}
	old
}

/// The `RefCell` and the `OnceCell` of a cell that shared references point at, moved out
/// through pointers from them, whose cells, held inline, are then set.
fn inline_moved(r: &RefCell<Cell<u32>>, o: &OnceCell<Cell<u32>>) -> (RefCell<Cell<u32>>, OnceCell<Cell<u32>>) {
	let old = unsafe { (ptr::read(r), ptr::read(o)) };
	let cell = old.0.borrow();
	cell.set(cell.get() + 9);
	drop(cell);
	if let Some(cell) = old.1.get() {
		cell.set(cell.get() + 9);
	}
	old
}

/// A cell written whole through a pointer from the shared reference given, which may be
/// the cell `Cell::from_mut` makes of another place.
#[inline(never)]
fn cell_rewritten(r: &Cell<u32>, n: u32) {
	unsafe { ptr::from_ref(r).cast_mut().write(Cell::new(n)) }
}

/// A `HashMap` dropped in place and written whole through a pointer from the `&mut` given,
/// which holds what the new one does, its capacity included.
#[inline(never)]
fn map_rewritten(r: &mut std::collections::HashMap<u32, u32>) {
	let p = ptr::from_mut(r);
	unsafe {
		ptr::drop_in_place(p);
		ptr::write(p, std::collections::HashMap::new());
	}
}

/// A cell of a `RefCell`, which an uninitialized shell holds nothing in.
struct HeldCell {
	c: UnsafeCell<RefCell<u32>>,
}

/// What the uninitialized `HeldCell` a shared reference points at holds, moved out through
/// a pointer from it.
#[inline(never)]
fn uninit_moved(r: &MaybeUninit<HeldCell>) -> MaybeUninit<HeldCell> {
	unsafe { ptr::read(r) }
}

/// A range of cells and a cell `Cell::from_mut` made of a variable's place, moved out
/// through pointers from shared references to them, whose cells are then set.
fn ends_moved(r: &ops::Range<Cell<u32>>, c: &Cell<u32>) -> (ops::Range<Cell<u32>>, Cell<u32>) {
	let old = unsafe { (ptr::read(r), ptr::read(c)) };
	old.0.start.set(old.0.start.get() + 9);
	old.0.end.set(old.0.end.get() + 9);
	old.1.set(old.1.get() + 9);
	old
}

/// Values holding a library object, moved out through pointers made each way, at any
/// depth, and then written in place: what was moved out keeps what it held.
pub fn moved_out(data: &[u8], out: &mut Vec<u8>) {
	let with = |n: u8| -> Vec<u8> { data.iter().map(|b| b.wrapping_add(n)).collect() };
	for how in 0..4 {
		let mut v = data.to_vec();
		let old = moved_vec(&mut v, with(how + 1), how);
		out.extend(&old);
		out.push(0xfe);
		out.extend(&v);
		out.push(0xff);
	}
	for how in 0..2 {
		let mut o = data.first().map(|_| data.to_vec());
		let mut t = (data.to_vec(), how);
		let mut s = TaggedVec {
			v: data.to_vec(),
			tag: how,
		};
		let (old_o, old_t, old_s) = moved_nested(&mut o, &mut t, &mut s, &with(9), how);
		for v in [
			old_o.as_ref(),
			Some(&old_t.0),
			Some(&old_s.v),
			o.as_ref(),
			Some(&t.0),
			Some(&s.v),
		]
		.into_iter()
		.flatten()
		{
			out.extend(v);
			out.push(0xfe);
		}
		out.extend([old_t.1, old_s.tag, t.1, s.tag]);
	}
	for &b in data {
		let cell = (b % 3 != 0).then(|| Cell::new(u32::from(b)));
		let old = cell_moved(&cell);
		for c in [&cell, &old] {
			out.push(c.as_ref().map_or(0, Cell::get) as u8);
		}
		let r = RefCell::new(Cell::new(u32::from(b)));
		let o = OnceCell::new();
		if b % 2 == 0 {
			let _ = o.set(Cell::new(u32::from(b) + 1));
		}
		let (old_r, old_o) = inline_moved(&r, &o);
		for r in [&r, &old_r] {
			out.push(r.borrow().get() as u8);
		}
		for o in [&o, &old_o] {
			out.push(o.get().map_or(0, Cell::get) as u8);
		}
		let r = Cell::new(u32::from(b))..Cell::new(u32::from(b) + 1);
		let mut x = u32::from(b) + 2;
		let (old_r, old_c) = ends_moved(&r, Cell::from_mut(&mut x));
		for r in [&r, &old_r] {
			out.extend([r.start.get() as u8, r.end.get() as u8]);
		}
		out.extend([old_c.get() as u8, x as u8]);
	}
	let mut map = std::collections::HashMap::<u32, u32>::with_capacity(data.len() * 8);
	map.extend(data.iter().map(|&b| (u32::from(b), u32::from(b))));
	map_rewritten(&mut map);
	out.extend([map.len() as u8, map.capacity() as u8]);
	for &b in data {
		let mut x = u32::from(b);
		cell_rewritten(Cell::from_mut(&mut x), u32::from(b) + 1);
		let cell = Cell::new(u32::from(b));
		cell_rewritten(&cell, u32::from(b) + 2);
		out.extend([x as u8, cell.get() as u8]);
	}
	let mut held = Box::<HeldCell>::new_uninit();
	let mut old = uninit_moved(&held);
	for (i, h) in [&mut *held, &mut old].into_iter().enumerate() {
		h.write(HeldCell {
			c: UnsafeCell::new(RefCell::new(u32::from(data.first().copied().unwrap_or(0)) + i as u32)),
		});
	}
	for h in [&mut *held, &mut old] {
		out.push(*unsafe { h.assume_init_mut() }.c.get_mut().get_mut() as u8);
	}
}

/// Pointers to slices cast to ones to arrays of their elements, of bytes and of numbers:
/// read whole, borrowed, and written through, at every offset; compared with the arrays
/// std's safe casts give; cast past where the slice ends and read only where its elements
/// are; and cast from null, dangling, and empty ones, which point at nothing.
pub fn array_casts(data: &[u8], out: &mut Vec<u8>) {
	let mut bytes = data.to_vec();
	for at in 0..bytes.len() {
		let rest = &bytes[at..];
		// Cast first, then read only where the slice reaches, as Rust allows.
		let pair = rest as *const [u8] as *const [u8; 2];
		if rest.len() >= 2 {
			out.extend(unsafe { *pair });
			let borrowed = unsafe { &*pair };
			out.push(u8::from(ptr::eq(borrowed, rest.first_chunk::<2>().unwrap())));
			out.push(u8::from(ptr::eq(pair, rest.first_chunk::<2>().unwrap())));
		}
		let lent = NonNull::from(&bytes[at..]).cast::<[u8; 1]>();
		out.push(unsafe { lent.as_ref() }[0]);
	}
	if bytes.len() >= 3 {
		let tail = bytes.len() - 3;
		let quad = unsafe { &mut *(&mut bytes[tail..] as *mut [u8] as *mut [u8; 3]) };
		quad[0] ^= quad[2];
		quad.reverse();
		let whole = unsafe { &mut *(&raw mut bytes[..]).cast::<[u8; 3]>() };
		whole[1] = whole[1].wrapping_add(1);
	}
	out.extend(&bytes);
	let words: Vec<u32> = data.iter().map(|&b| u32::from(b) * 65_793).collect();
	let mut copy = words.clone();
	for at in 0..words.len().saturating_sub(1) {
		let three = &words[at..] as *const [u32] as *const [u32; 3];
		if words.len() - at >= 3 {
			let read = unsafe { *three };
			out.extend(read.iter().flat_map(|w| w.to_le_bytes()));
			out.push(u8::from(ptr::eq(three, words[at..].first_chunk::<3>().unwrap())));
		}
		let first = unsafe { &mut *(&mut copy[at..] as *mut [u32] as *mut [u32; 2]) };
		first.swap(0, 1);
		first[0] ^= 0x0101;
	}
	out.extend(copy.iter().flat_map(|w| w.to_le_bytes()));
	let mut pairs: Vec<[u8; 2]> = data.iter().map(|&b| [b, !b]).collect();
	if pairs.len() >= 3 {
		let two = unsafe { &mut *(&mut pairs[1..] as *mut [[u8; 2]] as *mut [[u8; 2]; 2]) };
		two[0][1] ^= two[1][0];
		two.swap(0, 1);
		let read = unsafe { *(&pairs[..] as *const [[u8; 2]] as *const [[u8; 2]; 3]) };
		out.extend(read.iter().flatten());
	}
	out.extend(pairs.iter().flatten());
	let empty: &[u8] = &bytes[bytes.len()..];
	let none = empty as *const [u8] as *const [u8; 0];
	out.push(unsafe { *none }.len() as u8);
	// Values in no array, an empty slice of a byte and a whole one of a number.
	let byte = data.first().copied().unwrap_or(7);
	let lone = ptr::slice_from_raw_parts(&raw const byte, 0).cast::<[u8; 0]>();
	out.push(u8::from(lone.is_null()));
	out.push(unsafe { &*(lone as *const [u8]) }.len() as u8);
	let word = u32::from(byte) * 3;
	let one = ptr::slice_from_raw_parts(&raw const word, 1) as *const [u32; 1];
	out.extend(unsafe { *one }[0].to_le_bytes());
	let null = ptr::slice_from_raw_parts(ptr::null::<u8>(), 4) as *const [u8; 4];
	let dangling = NonNull::<[u32]>::slice_from_raw_parts(NonNull::dangling(), 0).cast::<[u32; 2]>();
	out.extend([
		u8::from(null.is_null()),
		u8::from(ptr::eq(dangling.as_ptr(), NonNull::dangling().as_ptr())),
	]);
}

/// A value beside a byte, which each instance of [`Either`] lays out at its own offsets.
#[derive(Clone, Copy)]
struct Tagged<T> {
	value: T,
	tag: u8,
}

/// A union over a type parameter, whose fields overlap where each instance puts them.
#[derive(Clone, Copy)]
union Either<T: Copy> {
	pair: (T, u8),
	tagged: Tagged<T>,
	value: T,
}

/// Each instance of a union over a type parameter: written as one field, in part through
/// another and through a reference into it, and whole as a third, read as each, and copied
/// before it changes.
fn either<T: Copy>(v: T, w: T, tag: u8) -> [(T, u8); 3] {
	let mut e = Either { pair: (v, tag) };
	let copy = e;
	unsafe {
		e.tagged.tag = tag.wrapping_add(1);
		let seen = e.pair;
		e.value = w;
		flip(&mut e.tagged.tag);
		[seen, (e.tagged.value, e.pair.1), copy.pair]
	}
}

fn flip(tag: &mut u8) {
	*tag ^= 0x80;
}

/// Unions over a type parameter, at a byte's instance and a `u32`'s.
pub fn generic_unions(data: &[u8], out: &mut Vec<u8>) {
	for (i, &byte) in data.iter().enumerate() {
		let wide = u32::from(byte) << 16 | i as u32;
		for (v, tag) in either(wide, !wide, byte) {
			out.extend_from_slice(&v.to_le_bytes());
			out.push(tag);
		}
		for (v, tag) in either(byte, byte.wrapping_mul(3), i as u8) {
			out.extend([v, tag]);
		}
	}
}

/// A struct holding cells, reached through a raw pointer to it.
struct Slots {
	num: UnsafeCell<u32>,
	held: UnsafeCell<Held>,
	pair: UnsafeCell<(u8, u16)>,
}

/// What `UnsafeCell::raw_get` gives of a reference to a cell of a number, a struct Rust
/// copies, one JS keeps as an object, a tuple, and arrays of bytes and of numbers; of a
/// pointer to a variable's cell, to a field through a pointer to its struct, and from
/// `MaybeUninit::as_ptr`; cells `UnsafeCell::from_mut` makes of a variable, an object,
/// an element, and an array, written through `get`; `UnsafeCell::replace` of each, which a
/// pointer taken before sees, giving back what it held undropped; what `as_ref_unchecked`
/// and `as_mut_unchecked` lend; and the allocations of boxes and pinned boxes replaced and
/// swapped through pointers, `UnsafeCell`s, one `from_mut` makes, and `Cell`s.
pub fn raw_cells(data: &[u8], out: &mut Vec<u8>) {
	let count = Rc::new(());
	let num = UnsafeCell::new(7u32);
	let duo = UnsafeCell::new(Duo(1, 2));
	let held = UnsafeCell::new(Held {
		n: 3,
		count: Rc::clone(&count),
	});
	let pair = UnsafeCell::new((4u8, 5u16));
	let bytes = UnsafeCell::new([6u8, 7]);
	let wide = UnsafeCell::new([8u16, 9]);
	let slots = Slots {
		num: UnsafeCell::new(10),
		held: UnsafeCell::new(Held {
			n: 11,
			count: Rc::clone(&count),
		}),
		pair: UnsafeCell::new((12, 13)),
	};
	let slot = MaybeUninit::new(UnsafeCell::new(14u32));
	let mut local = 15u32;
	let mut whole = Held {
		n: 16,
		count: Rc::clone(&count),
	};
	let mut elems = [17u16, 18, 19];
	for &op in data {
		let n = op / 8;
		match op % 8 {
			// Through references, each read back through `get`.
			0 => unsafe {
				*UnsafeCell::raw_get(&num) ^= u32::from(n) << 20;
				*UnsafeCell::raw_get(&num as *const _) ^= u32::from(n);
				(*UnsafeCell::raw_get(&duo)).1 ^= n;
				(*UnsafeCell::raw_get(&held)).n ^= n;
				(*UnsafeCell::raw_get(&pair)).1 ^= u16::from(n) << 8;
				(*UnsafeCell::raw_get(&bytes))[usize::from(n % 2)] ^= n;
				(*UnsafeCell::raw_get(&wide))[usize::from(n % 2)] ^= u16::from(n) << 4;
				out.extend((*num.get()).to_le_bytes());
				out.extend([(*duo.get()).1, (*held.get()).n]);
				out.extend((*pair.get()).1.to_le_bytes());
				out.extend(*bytes.get());
				out.extend((*wide.get())[1].to_le_bytes());
			},
			// Through raw pointers to where each cell is kept.
			1 => unsafe {
				let p = &raw const num;
				*UnsafeCell::raw_get(p) = (*UnsafeCell::raw_get(p)).wrapping_mul(u32::from(n) | 1);
				let s = &raw const slots;
				*UnsafeCell::raw_get(&raw const (*s).num) += u32::from(n);
				(*UnsafeCell::raw_get(&raw const (*s).held)).n ^= n;
				(*UnsafeCell::raw_get(&raw const (*s).pair)).0 ^= n;
				*UnsafeCell::raw_get(slot.as_ptr()) ^= u32::from(n);
				out.extend((*num.get()).to_le_bytes());
				out.extend((*slots.num.get()).to_le_bytes());
				out.extend([(*slots.held.get()).n, (*slots.pair.get()).0]);
				out.extend((*slot.assume_init_ref().get()).to_le_bytes());
			},
			// Cells of what a `&mut` borrows, written through `get` and seen by the borrowed.
			2 => unsafe {
				let c = UnsafeCell::from_mut(&mut local);
				*c.get() = (*c.get()).rotate_left(u32::from(n));
				let h = UnsafeCell::from_mut(&mut whole);
				(*h.get()).n ^= n;
				let e = UnsafeCell::from_mut(&mut elems[usize::from(n % 3)]);
				*e.get() ^= u16::from(n);
				out.extend(local.to_le_bytes());
				out.push(whole.n);
				out.extend(elems.iter().flat_map(|e| e.to_le_bytes()));
			},
			// Each replaced, what it held given back, a pointer taken before seeing the new.
			3 => unsafe {
				let p = num.get();
				let old = num.replace(u32::from(n) << 3);
				out.extend(old.to_le_bytes());
				out.extend((*p).to_le_bytes());
				let p = held.get();
				let old = held.replace(Held {
					n,
					count: Rc::clone(&count),
				});
				out.extend([old.n, (*p).n, Rc::strong_count(&count) as u8]);
				drop(old);
				out.push(Rc::strong_count(&count) as u8);
				let p = pair.get();
				let old = pair.replace((n, u16::from(n) * 3));
				out.extend([old.0, (*p).0]);
				let p = bytes.get();
				let old = bytes.replace([n, !n]);
				out.extend(old);
				out.extend(*p);
				let old = duo.replace(Duo(n, n ^ 1));
				out.extend([old.0, old.1, (*duo.get()).0]);
			},
			// Through cells `from_mut` makes, each replaced.
			4 => unsafe {
				let old = UnsafeCell::from_mut(&mut local).replace(u32::from(n));
				out.extend(old.to_le_bytes());
				let h = UnsafeCell::from_mut(&mut whole);
				let p = h.get();
				let old = h.replace(Held {
					n: !n,
					count: Rc::clone(&count),
				});
				out.extend([old.n, (*p).n, whole.n, Rc::strong_count(&count) as u8]);
				let old = UnsafeCell::from_mut(&mut elems[usize::from(n % 3)]).replace(u16::from(n));
				out.extend(old.to_le_bytes());
				out.extend(local.to_le_bytes());
			},
			// What the unchecked references lend, and a cell of an array a `&mut` borrows.
			5 => unsafe {
				*num.as_mut_unchecked() ^= u32::from(n);
				let r = num.as_mut_unchecked();
				*r = r.rotate_left(1);
				held.as_mut_unchecked().n ^= n;
				pair.as_mut_unchecked().1 ^= u16::from(n);
				bytes.as_mut_unchecked()[1] ^= n;
				*wide.as_mut_unchecked() = [u16::from(n), 1];
				out.extend(num.as_ref_unchecked().to_le_bytes());
				out.extend([held.as_ref_unchecked().n, duo.as_ref_unchecked().0]);
				out.extend(pair.as_ref_unchecked().1.to_le_bytes());
				out.extend(bytes.as_ref_unchecked());
				out.extend(wide.as_ref_unchecked()[0].to_le_bytes());
				let mut arr = [n, 1, 2];
				let a = UnsafeCell::from_mut(&mut arr);
				(*a.get())[usize::from(n % 3)] ^= 0x40;
				let old = a.replace([3, n, 4]);
				out.extend(old);
				out.extend(arr);
			},
			// Boxes replaced and swapped through pointers and cells, each allocation moving
			// with its box.
			6 => unsafe {
				let held = |n| Held {
					n,
					count: Rc::clone(&count),
				};
				let mut a = Box::new(held(n));
				let at_a: *const Held = &*a;
				let old = ptr::replace(&mut a, Box::new(held(!n)));
				let mut other = Box::new(held(n ^ 1));
				let (at_new, at_other): (*const Held, *const Held) = (&*a, &*other);
				ptr::swap(&mut a, &mut other);
				out.extend([u8::from(ptr::eq(at_a, &*old)), u8::from(ptr::eq(at_new, &*other))]);
				out.extend([u8::from(ptr::eq(at_other, &*a)), old.n, a.n, other.n]);
				let pinned = UnsafeCell::new(Box::pin((n, 1u16)));
				let at_pinned: *const (u8, u16) = &**pinned.get();
				let was = pinned.replace(Box::pin((n ^ 2, 2)));
				let (x, y) = (Cell::new(Box::pin((n, 3u16))), Cell::new(Box::pin((!n, 4u16))));
				let at_x: *const (u8, u16) = &**x.as_ptr();
				x.swap(&y);
				out.extend([u8::from(ptr::eq(at_x, &**y.as_ptr())), x.into_inner().0]);
				out.extend([u8::from(ptr::eq(at_pinned, &*was)), was.0, pinned.into_inner().0]);
				let cell = UnsafeCell::new(Box::new(held(n ^ 3)));
				let at_cell: *const Held = &**cell.get();
				let was = cell.replace(Box::new(held(n ^ 4)));
				let mut boxed = Box::new(held(n ^ 7));
				let at_boxed: *const Held = &*boxed;
				let lent = UnsafeCell::from_mut(&mut boxed).replace(Box::new(held(n)));
				out.extend([u8::from(ptr::eq(at_boxed, &*lent)), lent.n, boxed.n]);
				let (x, y) = (Cell::new(Box::new(held(n ^ 5))), Cell::new(Box::new(held(n ^ 6))));
				let at_x: *const Held = &**x.as_ptr();
				x.swap(&y);
				out.extend([u8::from(ptr::eq(at_cell, &*was)), was.n, (*cell.get()).n]);
				out.extend([u8::from(ptr::eq(at_x, &**y.as_ptr())), Rc::strong_count(&count) as u8]);
			},
			_ => out.push(n),
		}
	}
	out.extend(num.into_inner().to_le_bytes());
	out.extend([held.into_inner().n, Rc::strong_count(&count) as u8]);
	out.extend(slots.num.into_inner().to_le_bytes());
	drop(slots.held);
	out.extend([whole.n, Rc::strong_count(&count) as u8]);
	out.extend(elems.iter().flat_map(|e| e.to_le_bytes()));
}

thread_local! {
	/// The cell `Reentrant`'s `default` and `eq` write.
	static REENTERED: Cell<Reentrant> = const { Cell::new(Reentrant(1, 1)) };
}

/// A struct Rust copies whose `default` and `eq` write the cell holding it.
#[derive(Clone, Copy)]
struct Reentrant(u8, u8);

impl Default for Reentrant {
	fn default() -> Self {
		REENTERED.with(|c| c.set(Reentrant(7, 7)));
		Reentrant(0, 0)
	}
}

impl PartialEq for Reentrant {
	fn eq(&self, other: &Self) -> bool {
		REENTERED.with(|c| c.set(Reentrant(9, 9)));
		self.0 == other.0
	}
}

/// What `Cell`s of a tuple, a struct Rust copies, one that owns a count, and arrays of bytes
/// and of numbers, and cells `Cell::from_mut` makes of a tuple and an array, hold after
/// `replace`, `set`, `swap` (but of the arrays), `take`, and `update`, each read back
/// through a pointer `as_ptr` gave before, and what each gives back and drops; and `take`
/// and `==` of a cell its value's `default` and `eq` write.
pub fn cell_writes(data: &[u8], out: &mut Vec<u8>) {
	let count = Rc::new(());
	let held = |n: u8| Held {
		n,
		count: Rc::clone(&count),
	};
	let pair = Cell::new((1u8, 2u16));
	let duo = Cell::new(Duo(3, 4));
	let owner = Cell::new(held(5));
	let bytes = Cell::new([6u8, 7]);
	let wide = Cell::new([8u16, 9]);
	let (to_pair, to_duo, to_owner) = (pair.as_ptr(), duo.as_ptr(), owner.as_ptr());
	let (to_bytes, to_wide) = (bytes.as_ptr(), wide.as_ptr());
	let mut lent = (10u8, 11u16);
	let mut lent_bytes = [12u8, 13];
	for &op in data {
		let n = op / 8;
		match op % 8 {
			0 => {
				let old = pair.replace((n, u16::from(n) << 8));
				out.extend([old.0, duo.replace(Duo(n, !n)).1, owner.replace(held(n)).n]);
				out.extend(bytes.replace([n, 1]));
				out.extend(wide.replace([u16::from(n), 2])[1].to_le_bytes());
			}
			1 => {
				pair.set((n ^ 1, 3));
				duo.set(Duo(n ^ 2, 4));
				owner.set(held(n ^ 3));
				bytes.set([n ^ 4, 5]);
				wide.set([6, u16::from(n) << 4]);
			}
			2 => {
				pair.swap(&Cell::new((n, 7)));
				let other = Cell::new(Duo(8, n));
				duo.swap(&other);
				out.extend([other.get().0, other.get().1]);
				let other = Cell::new(held(n ^ 5));
				owner.swap(&other);
				out.push(other.into_inner().n);
			}
			3 => {
				out.push(pair.take().0);
				out.extend(bytes.take());
				out.extend(wide.take()[0].to_le_bytes());
			}
			4 => {
				pair.update(|(a, b)| (a.wrapping_add(n), b ^ 1));
				duo.update(|Duo(a, b)| Duo(b, a ^ n));
				bytes.update(|[a, b]| [b, a.wrapping_mul(n)]);
				wide.update(|[a, b]| [b.rotate_left(u32::from(n)), a]);
			}
			6 => REENTERED.with(|c| {
				c.set(Reentrant(n, 1));
				let taken = c.take();
				out.extend([taken.0, c.get().0]);
				c.set(Reentrant(n, 1));
				let same = *c == Cell::new(Reentrant(n, 2));
				out.extend([u8::from(same), c.get().0]);
			}),
			5 => unsafe {
				let c = Cell::from_mut(&mut lent);
				let p = c.as_ptr();
				c.set((n, 1));
				let old = c.replace((n ^ 1, 2));
				c.swap(&Cell::new((n ^ 2, 3)));
				c.update(|(a, b)| (a ^ 4, b));
				out.extend([old.0, (*p).0, c.take().0]);
				let c = Cell::from_mut(&mut lent_bytes);
				let p = c.as_ptr();
				let old = c.replace([n, 5]);
				c.update(|[a, b]| [b, a]);
				out.extend(old);
				out.extend(*p);
				out.extend([lent.0, lent_bytes[0]]);
			},
			_ => out.push(n),
		}
		unsafe {
			out.extend([(*to_pair).0, (*to_duo).0, (*to_duo).1, (*to_owner).n]);
			out.extend((*to_pair).1.to_le_bytes());
			out.extend(*to_bytes);
			out.extend((*to_wide)[1].to_le_bytes());
		}
		out.push(Rc::strong_count(&count) as u8);
	}
	out.extend([pair.get().0, duo.get().1, owner.into_inner().n, bytes.get()[1]]);
	out.push(Rc::strong_count(&count) as u8);
}

/// A struct holding a cell beside a number.
struct Celled {
	c: UnsafeCell<u32>,
	n: u8,
}

/// Whole writes of values holding cells into a cell, a tuple, and a struct, each read back
/// through a pointer saved into an inner cell before: `UnsafeCell::replace`, `ptr::replace`,
/// `ptr::write`, `ptr::swap`, `Cell`'s writes, and assignments through a raw pointer; a
/// value `ptr::read` copies out before one, and before a write into a part holding no
/// cell; `ptr::write`s into memory not written yet; and pointers `UnsafeCell::raw_get`
/// makes into such memory before the write.
pub fn nested_cells(data: &[u8], out: &mut Vec<u8>) {
	let nest = UnsafeCell::new(UnsafeCell::new(1u32));
	let deep = UnsafeCell::new(UnsafeCell::new(UnsafeCell::new(2u32)));
	let pair = UnsafeCell::new((UnsafeCell::new(3u32), 4u8));
	let strukt = UnsafeCell::new(Celled {
		c: UnsafeCell::new(5),
		n: 6,
	});
	let obj = UnsafeCell::new(UnsafeCell::new((7u8, 8u8)));
	let cell = Cell::new((UnsafeCell::new(9u32), 10u8));
	let wrapped = Cell::new(UnsafeCell::new(11u32));
	unsafe {
		let to_wrapped = (*wrapped.as_ptr()).get();
		let to_nest = (*nest.get()).get();
		let to_deep = (*(*deep.get()).get()).get();
		let to_pair = (*pair.get()).0.get();
		let to_strukt = (*strukt.get()).c.get();
		let to_obj = (*obj.get()).get();
		let to_cell = (*cell.as_ptr()).0.get();
		for &op in data {
			let n = op / 8;
			match op % 8 {
				0 => {
					let old = nest.replace(UnsafeCell::new(u32::from(n)));
					out.push(old.into_inner() as u8);
					let old = deep.replace(UnsafeCell::new(UnsafeCell::new(u32::from(n) + 1)));
					out.push(old.into_inner().into_inner() as u8);
					let old = pair.replace((UnsafeCell::new(u32::from(n) + 2), n));
					out.extend([old.0.into_inner() as u8, old.1]);
					let old = strukt.replace(Celled {
						c: UnsafeCell::new(u32::from(n) + 3),
						n,
					});
					out.extend([old.c.into_inner() as u8, old.n]);
					let old = obj.replace(UnsafeCell::new((n, n ^ 1)));
					out.push(old.into_inner().1);
				}
				1 => {
					let old = ptr::replace(nest.get(), UnsafeCell::new(u32::from(n) ^ 1));
					out.push(old.into_inner() as u8);
					let old = ptr::replace(pair.get(), (UnsafeCell::new(u32::from(n) ^ 2), n));
					out.push(old.0.into_inner() as u8);
				}
				2 => {
					ptr::write(nest.get(), UnsafeCell::new(u32::from(n) ^ 3));
					ptr::write(
						strukt.get(),
						Celled {
							c: UnsafeCell::new(u32::from(n) ^ 4),
							n,
						},
					);
				}
				3 => {
					let other = UnsafeCell::new(UnsafeCell::new(u32::from(n) ^ 5));
					ptr::swap(nest.get(), other.get());
					out.push(other.into_inner().into_inner() as u8);
					let mut other = Celled {
						c: UnsafeCell::new(u32::from(n) ^ 12),
						n: !n,
					};
					ptr::swap(strukt.get(), &raw mut other);
					out.extend([other.c.into_inner() as u8, other.n]);
				}
				4 => {
					let old = cell.replace((UnsafeCell::new(u32::from(n) ^ 6), n));
					out.push(old.0.into_inner() as u8);
					cell.set((UnsafeCell::new(u32::from(n) ^ 7), n));
				}
				5 => {
					*to_nest = u32::from(n) * 3;
					*to_pair = u32::from(n) * 5;
					let kept = ptr::read(nest.get());
					ptr::write(nest.get(), UnsafeCell::new(u32::from(n) + 7));
					out.push(kept.into_inner() as u8);
				}
				6 => {
					let old = wrapped.replace(UnsafeCell::new(u32::from(n) ^ 8));
					out.push(old.into_inner() as u8);
					wrapped.set(UnsafeCell::new(u32::from(n) ^ 9));
					let mut fresh = MaybeUninit::<UnsafeCell<u32>>::uninit();
					ptr::write(fresh.as_mut_ptr(), UnsafeCell::new(u32::from(n) ^ 10));
					let mut parts = MaybeUninit::<(UnsafeCell<u32>, u8)>::uninit();
					ptr::write(parts.as_mut_ptr(), (UnsafeCell::new(u32::from(n) ^ 11), n));
					out.extend([
						*fresh.assume_init_ref().get() as u8,
						*parts.assume_init_ref().0.get() as u8,
					]);
					let mut slot = MaybeUninit::<UnsafeCell<u32>>::uninit();
					let mut pair_slot = MaybeUninit::<UnsafeCell<(u8, u8)>>::uninit();
					let to_slot = slot.as_mut_ptr();
					let to_pair_slot = pair_slot.as_mut_ptr();
					let (early, early_pair) = (UnsafeCell::raw_get(to_slot), UnsafeCell::raw_get(to_pair_slot));
					ptr::write(to_slot, UnsafeCell::new(u32::from(n) ^ 17));
					ptr::write(to_pair_slot, UnsafeCell::new((n, !n)));
					out.extend([*early as u8, (*early_pair).1]);
					let mut uninit = MaybeUninit::new(UnsafeCell::new(u32::from(n)));
					let read_out = uninit.assume_init_read();
					ptr::write(uninit.as_mut_ptr(), UnsafeCell::new(u32::from(n) ^ 19));
					out.extend([read_out.into_inner() as u8, *uninit.assume_init_ref().get() as u8]);
					let src = UnsafeCell::new((UnsafeCell::new(u32::from(n)), (2u32, 3u32)));
					let copy = ptr::read(src.get());
					(*src.get()).1.0 = 9;
					out.extend([copy.1.0 as u8, (*src.get()).1.0 as u8]);
				}
				_ => {
					*nest.get() = UnsafeCell::new(u32::from(n) ^ 13);
					*deep.get() = UnsafeCell::new(UnsafeCell::new(u32::from(n) ^ 14));
				}
			}
			out.extend([*to_nest as u8, *to_deep as u8, *to_pair as u8, *to_strukt as u8]);
			out.extend([(*to_obj).0, (*to_obj).1, *to_cell as u8, *to_wrapped as u8]);
			out.extend([(*nest.get()).get().read() as u8, (*pair.get()).1, (*strukt.get()).n]);
		}
	}
}

/// Bytes whose `&raw` the body takes, a parameter's included: raw, mutable, and shared
/// slices of each, and pointers to arrays cast from them by each route, are views of
/// where the byte is, so a write through one reaches it and `offset_from` measures them.
pub fn lone_bytes(data: &[u8], out: &mut Vec<u8>) {
	for &d in data {
		let mut b = d;
		let p = &raw mut b;
		let raw = ptr::slice_from_raw_parts_mut(p, 1);
		unsafe { (*raw)[0] ^= 0x5a };
		out.push(b);
		let s = unsafe { std::slice::from_raw_parts_mut(&raw mut b, 1) };
		s[0] = s[0].wrapping_add(1);
		out.push(b);
		b = b.rotate_left(3);
		let shared = unsafe { std::slice::from_raw_parts(&raw const b, 1) };
		out.push(shared[0]);
		out.push(unsafe { shared.as_ptr().offset_from(&raw const b) } as u8);
		// To an array, by a cast, `cast`, and from an empty slice of it, whose pointer still
		// reaches the byte.
		let one = ptr::slice_from_raw_parts_mut(&raw mut b, 1) as *mut [u8; 1];
		unsafe { (*one)[0] = (*one)[0].wrapping_mul(5) };
		let none = ptr::slice_from_raw_parts(&raw const b, 0).cast::<[u8; 1]>();
		out.push(unsafe { (*none)[0] });
		let empty = unsafe { std::slice::from_raw_parts(&raw const b, 0) };
		let lent = unsafe { *(&raw const *empty).cast::<[u8; 1]>() };
		let from = unsafe { *NonNull::from(empty).cast::<[u8; 1]>().as_ptr() };
		out.extend([lent[0], from[0]]);
		let nn = NonNull::new(&raw mut b).unwrap();
		let made = NonNull::slice_from_raw_parts(nn, 1).cast::<[u8; 1]>();
		unsafe { (*made.as_ptr())[0] ^= 0xff };
		// A `&mut` to it and a closure's are where its pointers are.
		let r = &mut b;
		*r = r.wrapping_add(7);
		let mut bump = |by: u8| b = b.wrapping_add(by);
		bump(d);
		out.push(b);
		out.push(tripled(b));
		// Through a pointer cast from a `&mut`, and a `MaybeUninit` byte's.
		let mut c = d;
		let cast = unsafe { std::slice::from_raw_parts_mut(&mut c as *mut u8, 1) };
		cast[0] = cast[0].wrapping_sub(9);
		let p: *const u8 = &c;
		out.push(unsafe { std::slice::from_raw_parts(p, 1)[0] });
		let mut m = MaybeUninit::<u8>::uninit();
		unsafe { std::slice::from_raw_parts_mut(&raw mut m, 1)[0] = MaybeUninit::new(c) };
		out.push(unsafe { m.assume_init() });
		// Through a reference redirected by a `&mut` to it, at the byte it now points at.
		let (mut e, mut f) = (d, d ^ 1);
		let mut r = &mut e;
		redirect(&mut r, &mut f);
		let p = &raw mut *r;
		unsafe { *p = 9 };
		out.extend([e, f]);
	}
}

/// `r` pointed at `to`.
fn redirect<'a>(r: &mut &'a mut u8, to: &'a mut u8) {
	*r = to;
}

/// `b` tripled through a mutable slice of the parameter.
fn tripled(mut b: u8) -> u8 {
	let s = unsafe { std::slice::from_raw_parts_mut(&raw mut b, 1) };
	s[0] = s[0].wrapping_mul(3);
	b
}

/// A raw pointer to a `[T; N]` and to a `[[T; N]; M]` that is part of a slice, lent back as
/// a reference by `as_mut` and `NonNull::as_mut`, and the first written whole by
/// `ptr::replace` and `ptr::swap` and taken back as a box by `Box::from_raw`; and arrays a
/// `MaybeUninit` holds, lent by `as_ref` of bytes and by a cell's `get_mut` and
/// `as_mut_unchecked`.
pub fn array_parts(data: &[u8], out: &mut Vec<u8>) {
	let n = data.first().map_or(0, |&b| u32::from(b));
	let mut words: Vec<u32> = data.iter().map(|&b| u32::from(b)).collect();
	words.resize(words.len().max(6), 9);
	let a: &mut [u32; 2] = words[1..].first_chunk_mut().unwrap();
	let p: *mut [u32; 2] = a;
	if let Some(a) = unsafe { p.as_mut() } {
		a[0] = n;
	}
	let old = unsafe { ptr::replace(p, [n + 1, n + 2]) };
	out.extend(old.iter().map(|&w| w as u8));
	let q: *mut [u32; 2] = words[3..].first_chunk_mut().unwrap();
	let p: *mut [u32; 2] = words[1..].first_chunk_mut().unwrap();
	unsafe { ptr::swap(p, q) };
	let mut nn = NonNull::from(words[2..].first_chunk_mut::<2>().unwrap());
	unsafe { nn.as_mut()[1] = n + 3 };
	let boxed = Box::new([n, n + 1, n + 2]);
	let raw = Box::into_raw(boxed);
	unsafe { (*raw)[1] = 7 };
	let back = unsafe { Box::from_raw(raw) };
	out.extend(back.iter().map(|&w| w as u8));
	// Of arrays of no bytes, whose box owns no allocation, so any pointer to one makes one.
	let units: Box<[()]> = Box::new([(); 3]);
	let raw = Box::into_raw(units);
	let fewer = unsafe { Box::from_raw(raw as *mut [(); 2]) };
	let more = unsafe { Box::from_raw(raw as *mut [(); 5]) };
	let none: *mut [u32; 0] = words[2..].first_chunk_mut().unwrap();
	let none = unsafe { Box::from_raw(none) };
	out.extend([fewer.len(), more.iter().count(), none.len()].map(|n| n as u8));
	out.extend(words.iter().map(|&w| w as u8));
	// Of arrays of arrays.
	let mut rows: Vec<[u32; 2]> = words.chunks(2).map(|c| [c[0], c[c.len() - 1]]).collect();
	let p: *mut [[u32; 2]; 2] = rows[1..].first_chunk_mut().unwrap();
	if let Some(r) = unsafe { p.as_mut() } {
		r[1][0] = n + 4;
	}
	let mut nn = NonNull::new(p).unwrap();
	unsafe { nn.as_mut()[0][1] = n + 5 };
	out.extend(rows.iter().flatten().map(|&w| w as u8));
	// Of arrays a `MaybeUninit` holds, whose reference is the array.
	let bytes = MaybeUninit::new([n as u8, 5]);
	let p: *const MaybeUninit<[u8; 2]> = &bytes;
	out.extend(unsafe { p.as_ref() }.map_or([0; 2], |m| unsafe { m.assume_init() }));
	let mut cell = Cell::new(MaybeUninit::new([n as u8, 2]));
	unsafe { cell.get_mut().assume_init_mut()[1] = 9 };
	out.extend(unsafe { cell.into_inner().assume_init() });
	let mut uc = UnsafeCell::new(MaybeUninit::new([n, 3]));
	unsafe { uc.get_mut().assume_init_mut()[0] = n + 8 };
	unsafe { uc.as_mut_unchecked().assume_init_mut()[1] = n + 9 };
	out.extend(unsafe { uc.into_inner().assume_init() }.map(|w| w as u8));
	// Lazy cells in arrays a `Cell` replaces, moved out safely and forced.
	let (seven, eight): (fn() -> u32, fn() -> u32) = (|| 7, || 8);
	let mut lazies = [[std::cell::LazyCell::new(seven)]];
	let old = Cell::from_mut(&mut lazies).replace([[std::cell::LazyCell::new(eight)]]);
	out.extend([*old[0][0] as u8, *lazies[0][0] as u8, n as u8]);
}
