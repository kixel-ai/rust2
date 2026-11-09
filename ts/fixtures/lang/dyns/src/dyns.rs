//! Trait objects of the crate's own traits, behind `&`, `&mut`, and `Box`: each call runs
//! the impl of the value's own type, a struct, an enum, or a number, changing it in place
//! through `&mut self` wherever it is, a provided method, a supertrait's, `Debug` among
//! them, `self: Box<Self>`, and an upcast to the supertrait. Behind an `Arc`, the object
//! shares the count of the `Arc` it was made from. A `dyn Any` downcasts to the type it
//! was made of, borrowed, boxed, or shared, and upcast from the crate's own trait. A `dyn Fn*` of a closure, a function, or a
//! function pointer calls it, changing what it captured, borrowed, boxed, or shared. An
//! `Rc`, a `Weak`, a `Pin`, and an `Rc` or an `Arc` made from a box hold one too. A
//! `dyn Iterator`, `dyn DoubleEndedIterator`, `dyn ExactSizeIterator`, or
//! `dyn FusedIterator` of std's iterators or the crate's own, borrowed, boxed, or shared,
//! reads by its vtable's methods and folds by `next`, dropping what it owns in std's order.
//! A box of an iterator reads by what std's box forwards and folds by `next` too.
//! A struct whose last field is a trait object, a `RefCell`, a `Mutex`, an `RwLock`, a
//! `Cell`, an `UnsafeCell`, or the crate's own, shares its fields, borrows, and lock with the
//! value it was made from, a future there polled in place and dropped once. A
//! `ManuallyDrop` of one drops only by its own `drop`, and a raw pointer unsized to one, or
//! to a struct of one, reads and writes what it points at, drops it as the box made of it,
//! or stays null. An object of a generic trait, or of one with an associated type, calls
//! the impl at its arguments, through a supertrait at others too, held in a generic struct
//! too, and drops what its methods give out where Rust drops it.

use std::any::Any;
use std::cell::{Cell, RefCell, UnsafeCell};
use std::collections::BTreeMap;
use std::fmt::{self, Debug};
use std::future::Future;
use std::iter::FusedIterator;
use std::mem::ManuallyDrop;
use std::pin::Pin;
use std::rc::{self, Rc};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex, RwLock, Weak};
use std::task::{Context, Poll, Waker};

use crate::harness::put;

pub(crate) trait Shape {
	fn area(&self) -> u32;
	fn grow(&mut self, by: u8);
	/// Provided, and overridden by one impl.
	fn sides(&self) -> u8 {
		4
	}
	fn into_area(self: Box<Self>) -> u32 {
		self.area().wrapping_add(1)
	}
	/// Over early-bound lifetimes, which its signature takes as arguments too.
	fn larger<'a: 'b, 'b>(&'a self, other: &'b u32) -> u32 {
		self.area().max(*other)
	}
	/// Writes through a `&mut` to a number, which is an `rs.Place`.
	fn tally(&self, sum: &mut u32) {
		*sum = sum.wrapping_add(self.area());
	}
}

trait Named: Shape + Debug {
	fn label(&self) -> String;
}

#[derive(Debug)]
struct Square {
	side: u16,
}

impl Shape for Square {
	fn area(&self) -> u32 {
		u32::from(self.side) * u32::from(self.side)
	}
	fn grow(&mut self, by: u8) {
		self.side = self.side.wrapping_add(u16::from(by));
	}
}

impl Named for Square {
	fn label(&self) -> String {
		format!("square {}", self.side)
	}
}

/// An enum, which JS copies, so `&mut self` reaches it through a place.
#[derive(Debug, Clone, Copy)]
enum Tri {
	Flat,
	Tall(u8),
}

impl Shape for Tri {
	fn area(&self) -> u32 {
		match self {
			Tri::Flat => 0,
			Tri::Tall(h) => u32::from(*h) * 3,
		}
	}
	fn grow(&mut self, by: u8) {
		*self = match *self {
			Tri::Flat if by % 2 == 0 => Tri::Flat,
			Tri::Flat => Tri::Tall(by),
			Tri::Tall(h) => Tri::Tall(h.wrapping_add(by)),
		};
	}
	fn sides(&self) -> u8 {
		3
	}
	fn into_area(self: Box<Self>) -> u32 {
		self.area().wrapping_mul(2)
	}
}

impl Named for Tri {
	fn label(&self) -> String {
		format!("tri {self:?}")
	}
}

/// A number, which JS copies too.
impl Shape for u32 {
	fn area(&self) -> u32 {
		*self
	}
	fn grow(&mut self, by: u8) {
		*self = self.wrapping_mul(u32::from(by));
	}
}

/// Writes a `&dyn Debug` by calling its `fmt`, not through a placeholder.
struct Shown<'a>(&'a dyn Debug);

impl Debug for Shown<'_> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.0.fmt(f)
	}
}

fn total(shapes: &[&dyn Shape]) -> u32 {
	shapes.iter().fold(0, |sum, s| sum.wrapping_add(s.area()))
}

fn grow_twice(s: &mut dyn Shape, by: u8) {
	s.grow(by);
	s.grow(by.wrapping_add(1));
}

/// Over a type parameter that the trait object instantiates.
fn sides_of<T: Shape + ?Sized>(s: &T) -> u8 {
	s.sides()
}

fn upcast(n: &dyn Named) -> &dyn Shape {
	n
}

fn make(kind: u8, n: u8) -> Box<dyn Named> {
	match kind % 2 {
		0 => Box::new(Square { side: u16::from(n) }),
		_ => Box::new(if n % 3 == 0 { Tri::Flat } else { Tri::Tall(n) }),
	}
}

pub fn dyns(data: &[u8], out: &mut Vec<u8>) {
	let [a, b, c, rest @ ..] = data else {
		return;
	};
	let (a, b, c) = (*a, *b, *c);

	// Borrowed, of each type.
	let mut sq = Square { side: u16::from(a) };
	let mut tri = if b % 2 == 0 { Tri::Flat } else { Tri::Tall(b) };
	let mut n = u32::from(c);
	put(out, &format!("{} ", total(&[&sq, &tri, &n])));

	// Changed in place through `&mut dyn`, a local each.
	grow_twice(&mut sq, b);
	grow_twice(&mut tri, c);
	grow_twice(&mut n, a);
	put(out, &format!("{} {tri:?} {n} ", sq.side));
	put(out, &format!("{} {} ", sides_of::<dyn Shape>(&tri), sides_of(&sq)));

	// Owned, changed through the box, then consumed by `self: Box<Self>`.
	let mut shapes: Vec<Box<dyn Named>> = Vec::new();
	for (i, &x) in rest.iter().enumerate() {
		shapes.push(make(x.wrapping_add(i as u8), x));
	}
	for i in 0..shapes.len() {
		let s = &mut shapes[i];
		if i % 2 == 0 {
			s.grow(a);
		}
		put(
			out,
			&format!("{}:{}:{}:{:?} ", s.label(), s.area(), sides_of(upcast(&**s)), s),
		);
	}
	let mut sum = 0u32;
	for s in &shapes {
		s.tally(&mut sum);
	}
	put(
		out,
		&format!("{sum} {:?} {} ", Shown(&tri), shapes[0].larger(&u32::from(b))),
	);
	for s in shapes {
		sum = sum.wrapping_add(s.into_area());
	}
	let boxed: Box<dyn Shape> = Box::new(n);
	put(out, &format!("{sum} {}", boxed.into_area()));
}

/// Counts its calls, which every `Arc` of it sees.
struct Hits(AtomicU32);

impl Shape for Hits {
	fn area(&self) -> u32 {
		self.0.fetch_add(1, Ordering::Relaxed)
	}
	fn grow(&mut self, by: u8) {
		*self.0.get_mut() = self.0.get_mut().wrapping_add(u32::from(by));
	}
}

fn counts<T: ?Sized>(arcs: &[&Arc<T>]) -> String {
	let counts: Vec<String> = arcs.iter().map(|a| Arc::strong_count(a).to_string()).collect();
	counts.join(",")
}

pub fn shared(data: &[u8], out: &mut Vec<u8>) {
	let [a, b, rest @ ..] = data else {
		return;
	};

	// Two objects of one `Arc`, and an upcast of each: one count, which either side's
	// clones and drops change, and one value.
	let sq = Arc::new(Square { side: u16::from(*a) });
	let named: Arc<dyn Named> = sq.clone();
	let again: Arc<dyn Named> = Arc::<Square>::clone(&sq);
	let shape: Arc<dyn Shape> = named.clone();
	let up: Arc<dyn Shape> = again.clone();
	put(out, &format!("{} ", counts(&[&sq])));
	put(
		out,
		&format!("{} {} {} {:?} ", named.label(), shape.area(), sides_of(&*up), &*named),
	);
	let (weak, other) = (Arc::downgrade(&shape), Arc::downgrade(&up));
	put(
		out,
		&format!(
			"{} {} {} ",
			Arc::ptr_eq(&named, &again),
			Arc::ptr_eq(&shape, &up),
			Weak::ptr_eq(&weak, &other)
		),
	);
	drop(sq);
	drop(named);
	put(out, &format!("{} {} ", counts(&[&again]), Weak::strong_count(&weak)));
	let upgraded = weak.upgrade().map(|s| s.area());
	drop(again);
	put(out, &format!("{} ", counts(&[&up])));
	drop(up);
	put(
		out,
		&format!("{} {} {:?} ", counts(&[&shape]), weak.upgrade().is_some(), upgraded),
	);
	drop(shape);
	put(
		out,
		&format!("{} {} ", weak.upgrade().is_none(), Weak::strong_count(&other)),
	);

	// Calls through every `Arc` reach the one value.
	let hits = Arc::new(Hits(AtomicU32::new(u32::from(*b))));
	let objects: Vec<Arc<dyn Shape>> = (0..3).map(|_| hits.clone() as Arc<dyn Shape>).collect();
	let sum = objects.iter().fold(0u32, |sum, s| sum.wrapping_add(s.area()));
	put(
		out,
		&format!("{sum} {} {} ", hits.0.load(Ordering::Relaxed), counts(&[&hits])),
	);
	drop(objects);
	put(out, &format!("{} ", counts(&[&hits])));

	// Of a number and an enum, which JS copies, and two `Arc`s that are not the same.
	let n: Arc<dyn Shape> = Arc::new(u32::from(*b));
	let m: Arc<dyn Shape> = Arc::new(u32::from(*b));
	put(out, &format!("{} {} ", n.area(), Arc::ptr_eq(&n, &m)));
	let tris: Vec<Arc<dyn Named>> = rest
		.iter()
		.map(|&x| match x % 3 {
			0 => Arc::new(Tri::Flat) as Arc<dyn Named>,
			1 => Arc::new(Tri::Tall(x)),
			_ => Arc::new(Square { side: u16::from(x) }),
		})
		.collect();
	let kept: Vec<Arc<dyn Named>> = tris.iter().step_by(2).cloned().collect();
	for t in &tris {
		put(out, &format!("{}:{}:{} ", t.label(), t.area(), Arc::strong_count(t)));
	}
	drop(tris);
	let mut total = 0u32;
	for k in &kept {
		k.tally(&mut total);
	}
	put(out, &format!("{total} {}", counts(&kept.iter().collect::<Vec<_>>())));
}

/// A value that knows its own type, as an object of a trait whose `as_any` gives it.
trait Erased {
	fn as_any(&self) -> &dyn Any;
	fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: Any> Erased for T {
	fn as_any(&self) -> &dyn Any {
		self
	}
	fn as_any_mut(&mut self) -> &mut dyn Any {
		self
	}
}

/// A trait over `Any`, whose objects upcast to `dyn Any`.
trait Keyed: Any {
	fn key(&self) -> u32;
}

impl Keyed for u32 {
	fn key(&self) -> u32 {
		*self
	}
}

impl Keyed for Square {
	fn key(&self) -> u32 {
		u32::from(self.side)
	}
}

/// What `a` holds, by the type it downcasts to.
fn kind(a: &dyn Any) -> String {
	if let Some(sq) = a.downcast_ref::<Square>() {
		format!("square {}", sq.side)
	} else if let Some(n) = a.downcast_ref::<u32>() {
		format!("u32 {n}")
	} else if let Some(t) = a.downcast_ref::<Tri>() {
		format!("tri {t:?}")
	} else if a.is::<u8>() {
		"u8".into()
	} else {
		"other".into()
	}
}

/// Bumps the number or enum `a` holds where it is.
fn bump(a: &mut dyn Any, by: u8) -> bool {
	if let Some(n) = a.downcast_mut::<u32>() {
		*n = n.wrapping_add(u32::from(by));
		true
	} else if let Some(t) = a.downcast_mut::<Tri>() {
		t.grow(by);
		true
	} else if let Some(sq) = a.downcast_mut::<Square>() {
		sq.grow(by);
		true
	} else {
		false
	}
}

/// The same, through a generic `downcast_ref`, as a type parameter instantiates it.
fn is_a<T: Any>(e: &dyn Erased) -> bool {
	e.as_any().downcast_ref::<T>().is_some()
}

pub fn anys(data: &[u8], out: &mut Vec<u8>) {
	let [a, b, rest @ ..] = data else {
		return;
	};
	let (a, b) = (*a, *b);

	// Borrowed, of each type, and of one no downcast names.
	let mut sq = Square { side: u16::from(a) };
	let mut tri = if b % 2 == 0 { Tri::Flat } else { Tri::Tall(b) };
	let mut n = u32::from(b);
	let anys: [&dyn Any; 5] = [&sq, &tri, &n, &a, &"str"];
	for x in anys {
		put(out, &format!("{} ", kind(x)));
	}

	// Changed in place through `&mut dyn Any`, which JS copies for a number and an enum.
	let hit = (bump(&mut sq, a), bump(&mut tri, b), bump(&mut n, a), bump(&mut 0u8, a));
	put(out, &format!("{} {tri:?} {n} {hit:?} ", sq.side));

	// Through a trait whose methods give the value as `dyn Any`.
	let erased: [&dyn Erased; 3] = [&sq, &tri, &n];
	for e in erased {
		put(
			out,
			&format!("{}{}{} ", is_a::<Square>(e), is_a::<u32>(e), kind(e.as_any())),
		);
	}
	let mut m = u32::from(a);
	let e: &mut dyn Erased = &mut m;
	if let Some(x) = e.as_any_mut().downcast_mut::<u32>() {
		*x = x.wrapping_mul(3);
	}
	put(out, &format!("{m} "));

	// Of types that can be `undefined`, whose `None` is `rs.None`.
	let unit: &dyn Any = &();
	put(
		out,
		&format!("{:?} ", (unit.downcast_ref::<()>(), unit.is::<Option<u8>>())),
	);
	let (mut some, mut none) = (Some(a), None::<u8>);
	let odd: [&mut dyn Any; 3] = [&mut some, &mut none, &mut n];
	for x in odd {
		let found = (x.downcast_ref::<()>(), x.downcast_ref::<Option<u8>>().copied());
		put(out, &format!("{found:?}"));
		if let Some(o) = x.downcast_mut::<Option<u8>>() {
			*o = o.map_or(Some(b), |_| None);
		}
		put(out, &format!("{} ", x.downcast_mut::<()>().is_some()));
	}
	put(out, &format!("{some:?} {none:?} "));

	// Owned: a box downcasts to its value, or back to itself.
	let mut boxes: Vec<Box<dyn Any>> = Vec::new();
	for (i, &x) in rest.iter().enumerate() {
		boxes.push(match x % 4 {
			0 => Box::new(x) as Box<dyn Any>,
			1 => Box::new(u32::from(x)),
			2 => Box::new(Tri::Tall(x)),
			_ => Box::new(Square { side: i as u16 }),
		});
	}
	let mut kept = Vec::new();
	for (i, boxed) in boxes.into_iter().enumerate() {
		match boxed.downcast::<u32>() {
			Ok(n) => put(out, &format!("n{} ", *n)),
			Err(other) if i % 2 == 0 => kept.push(other),
			Err(other) => match other.downcast::<u8>() {
				Ok(byte) => put(out, &format!("u8 {byte} ")),
				Err(other) => put(out, &format!("{} ", kind(&*other))),
			},
		}
	}
	for k in kept.iter_mut().rev() {
		put(out, &format!("{}:{} ", kind(&**k), bump(&mut **k, a)));
	}

	// Shared: one count across the clones, and `Arc::downcast` gives back the `Arc` it
	// was made of.
	let hold: Arc<dyn Any + Send + Sync> = Arc::new((a, b));
	let held = [hold.clone(), hold.clone()];
	put(
		out,
		&format!("{} {:?} ", Arc::strong_count(&hold), hold.downcast_ref::<(u8, u8)>()),
	);
	drop(held);
	put(out, &format!("{} ", Arc::strong_count(&hold)));
	let typed = match hold.downcast::<u8>() {
		Ok(_) => unreachable!(),
		Err(hold) => hold.downcast::<(u8, u8)>().map(|t| (Arc::strong_count(&t), t.0)).ok(),
	};
	put(out, &format!("{typed:?} "));

	// Upcast from the crate's trait over `Any`: the downcast finds the value's own type,
	// borrowed, changed in place, boxed, shared, and weak.
	let mut n = u32::from(a);
	let keyed: [&dyn Keyed; 2] = [&sq, &n];
	for k in keyed {
		let any: &dyn Any = k;
		put(out, &format!("{}{} ", k.key(), kind(any)));
	}
	let k: &mut dyn Keyed = &mut n;
	let any: &mut dyn Any = k;
	put(out, &format!("{} ", bump(any, b)));
	put(out, &format!("{n} "));
	let boxed: Box<dyn Keyed> = Box::new(Square { side: u16::from(b) });
	let any: Box<dyn Any> = boxed;
	put(out, &format!("{:?} ", any.downcast::<Square>().map(|sq| sq.side).ok()));
	let shared: Rc<dyn Keyed> = Rc::new(n);
	let weak: rc::Weak<dyn Keyed> = Rc::downgrade(&shared);
	let weak: rc::Weak<dyn Any> = weak;
	let any: Rc<dyn Any> = shared;
	put(
		out,
		&format!(
			"{} {:?} ",
			weak.upgrade().is_some_and(|a| a.is::<u32>()),
			any.downcast_ref::<u32>()
		),
	);
	let typed = any.downcast::<u32>().map(|n| (Rc::strong_count(&n), *n)).ok();
	put(out, &format!("{typed:?} {}", weak.upgrade().is_some()));
}

/// A boxed callback kept in a struct, as kio keeps its waiters' polls.
struct Hook {
	f: Box<dyn FnMut(u8) -> u32>,
}

fn twice(f: &dyn Fn(u8) -> u8, x: u8) -> u8 {
	f(f(x))
}

fn bump_all(f: &mut dyn FnMut(u8), xs: &[u8]) {
	for &x in xs {
		f(x);
	}
}

/// Over a lifetime the call picks, as `form_urlencoded`'s encoder override is.
fn first_word(f: &dyn for<'a> Fn(&'a str) -> &'a str, s: &str) -> usize {
	f(s).len()
}

fn adder(by: u8) -> Box<dyn Fn(u8) -> u8> {
	Box::new(move |x| x.wrapping_add(by))
}

fn halve(x: u8) -> u8 {
	x / 2
}

fn flip(x: u8) -> u8 {
	x ^ 0x5a
}

pub fn fns(data: &[u8], out: &mut Vec<u8>) {
	let [a, b, rest @ ..] = data else {
		return;
	};
	let (a, b) = (*a, *b);

	// Borrowed: a closure, a function item, and a function pointer.
	let mul = |x: u8| x.wrapping_mul(b);
	let ptr: fn(u8) -> u8 = if b % 2 == 0 { halve } else { flip };
	put(
		out,
		&format!("{} {} {} ", twice(&mul, a), twice(&halve, a), twice(&ptr, a)),
	);
	put(
		out,
		&format!("{} ", first_word(&|s: &str| s.split(' ').next().unwrap_or(""), "ab cd")),
	);

	// A `&mut dyn FnMut` changes what its closure captured.
	let mut sum = 0u32;
	let mut seen = Vec::new();
	bump_all(
		&mut |x| {
			sum = sum.wrapping_add(u32::from(x));
			seen.push(x);
		},
		rest,
	);
	put(out, &format!("{sum} {seen:?} "));

	// Owned, of each kind, called in turn; the counter keeps its state between calls.
	let mut count = 0u8;
	let mut fs: Vec<Box<dyn Fn(u8) -> u8>> = vec![adder(a), Box::new(halve), Box::new(mul), Box::new(ptr)];
	fs.push(Box::new(move |x| twice(&halve, x).wrapping_add(a)));
	let turn = usize::from(b) % fs.len();
	fs.swap(0, turn);
	let mut hook = Hook {
		f: Box::new(move |x| {
			count = count.wrapping_add(x);
			u32::from(count) * 3
		}),
	};
	for (i, f) in fs.iter().enumerate() {
		let y = f(rest.get(i).copied().unwrap_or(b));
		put(out, &format!("{y}:{} ", (hook.f)(y)));
	}
	hook.f = Box::new(|x| u32::from(x));
	put(out, &format!("{} ", (hook.f)(a)));

	// Consumed once: a closure that moves out what it captured, and an `Fn` upcast to `FnOnce`.
	let list = vec![a];
	let take: Box<dyn FnOnce(u8) -> Vec<u8>> = Box::new(move |x| {
		let mut list = list;
		list.push(x);
		list
	});
	put(out, &format!("{:?} ", take(b)));
	let up: Box<dyn FnOnce(u8) -> u8> = fs.pop().unwrap();
	put(out, &format!("{} ", up(a)));

	// Shared: every clone calls the one closure.
	let shared: Arc<dyn Fn(u8) -> u8 + Send + Sync> = Arc::new(move |x| x ^ b);
	let other = shared.clone();
	put(
		out,
		&format!("{} {} {}", shared(a), other(b), Arc::strong_count(&other)),
	);
}

thread_local! {
	static DROPS: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
}

/// Puts what dropped since the last call.
fn dropped(out: &mut Vec<u8>) {
	let tags = DROPS.with_borrow_mut(|d| std::mem::take(d));
	put(out, &format!("{tags:?} "));
}

/// Logs its tag as it drops.
struct Tagged(u8);

impl Drop for Tagged {
	fn drop(&mut self) {
		DROPS.with_borrow_mut(|d| d.push(self.0));
	}
}

/// The crate's own iterator, counting down, which logs 200 as it drops.
struct Countdown(u8);

impl Iterator for Countdown {
	type Item = u8;

	fn next(&mut self) -> Option<u8> {
		self.0 = self.0.checked_sub(1)?;
		Some(self.0)
	}
}

impl Drop for Countdown {
	fn drop(&mut self) {
		DROPS.with_borrow_mut(|d| d.push(200));
	}
}

/// Each of the vtable's methods, then std's adaptors over the borrow, then by value.
fn read(it: &mut dyn Iterator<Item = u8>, out: &mut Vec<u8>) {
	let hint = it.size_hint();
	let first = it.next();
	let third = it.nth(1);
	let pair: Vec<u8> = (&mut *it).step_by(2).take(2).collect();
	let rest: u32 = it.map(u32::from).sum();
	put(out, &format!("{hint:?} {first:?} {third:?} {pair:?} {rest} "));
}

/// The first value at or past `from` that `f` answers, from either end, as moq's model
/// scans its groups.
fn scan<'a>(map: &'a BTreeMap<u64, u8>, from: u64, rev: bool, mut f: impl FnMut(&'a u8) -> Option<u8>) -> Option<u8> {
	let mut scan = |range: &mut dyn DoubleEndedIterator<Item = (&'a u64, &'a u8)>| match rev {
		true => range.rev().find_map(|(_, v)| f(v)),
		false => range.map(|(_, v)| v).find_map(&mut f),
	};
	scan(&mut map.range(from..))
}

/// The length on either side of an item, and an upcast that reads the rest.
fn lens(it: &mut dyn ExactSizeIterator<Item = &u8>) -> String {
	let before = it.len();
	let first = it.next().copied();
	let after = it.len();
	let up: &mut dyn Iterator<Item = &u8> = it;
	let back = up.last().copied();
	format!("{before} {first:?} {after} {back:?}")
}

fn tagged(tags: &[u8]) -> Vec<Tagged> {
	tags.iter().map(|&t| Tagged(t)).collect()
}

/// Lends its bytes as parts, as moq's transports lend a stream's chunks.
trait Chunks {
	fn chunks(&self) -> Box<dyn Iterator<Item = &[u8]> + '_>;
}

struct Lease(Vec<u8>, usize);

impl Chunks for Lease {
	fn chunks(&self) -> Box<dyn Iterator<Item = &[u8]> + '_> {
		Box::new(self.0.chunks(self.1))
	}
}

pub fn iters(data: &[u8], out: &mut Vec<u8>) {
	let [a, b, rest @ ..] = data else {
		return;
	};
	let (a, b) = (*a, *b);

	// Borrowed: std's iterators, a range, and the crate's own, which the borrow never drops.
	read(&mut rest.iter().copied(), out);
	read(&mut (a..b), out);
	let mut down = Countdown(a % 16);
	read(&mut down, out);
	dropped(out);
	drop(down);
	dropped(out);
	let map: BTreeMap<u64, u8> = rest.iter().enumerate().map(|(i, &x)| (i as u64, x)).collect();
	let odd = |v: &u8| (v % 2 == 1).then_some(*v);
	put(
		out,
		&format!(
			"{:?} {:?} ",
			scan(&map, u64::from(a % 8), false, odd),
			scan(&map, u64::from(b % 8), true, odd)
		),
	);
	put(out, &format!("{} ", lens(&mut rest.iter())));

	// Boxed: what it owns drops as it reads, then the rest as the box drops, in std's order.
	let (front, back) = rest.split_at(usize::from(a) % (rest.len() + 1));
	let mut boxed: Box<dyn DoubleEndedIterator<Item = Tagged>> = Box::new(tagged(front).into_iter());
	let ends = (boxed.next().map(|t| t.0), boxed.next_back().map(|t| t.0));
	let nth = boxed.nth(1).map(|t| t.0);
	dropped(out);
	put(out, &format!("{ends:?} {nth:?} "));
	drop(boxed);
	dropped(out);
	// A chain's own `last` keeps the first half's last until the end; through the box, std's
	// default reads by `next` instead.
	let chained: Box<dyn Iterator<Item = Tagged>> = Box::new(tagged(front).into_iter().chain(tagged(back)));
	let last = chained.last().map(|t| t.0);
	dropped(out);
	put(out, &format!("{last:?} "));
	let counted: Box<dyn Iterator<Item = Tagged>> = Box::new(tagged(back).into_iter().chain(tagged(front)));
	put(out, &format!("{} ", counted.count()));
	dropped(out);
	let own: Box<dyn Iterator<Item = u8>> = Box::new(Countdown(b % 16));
	let mut sum = 0u32;
	for x in own {
		sum = sum.wrapping_add(u32::from(x));
	}
	put(out, &format!("{sum} "));
	dropped(out);
	// A box of the crate's own, which std's `Iterator` for `Box<I>` forwards to: boxed again,
	// borrowed, read itself, and by an adaptor; and a box of a box and a box of a trait
	// object, borrowed.
	let nested: Box<dyn Iterator<Item = u8>> = Box::new(Box::new(Countdown(a % 8)));
	let mut inner = Box::new(Countdown(b % 8));
	let lent: &mut dyn Iterator<Item = u8> = &mut inner;
	let first = lent.next();
	let second = inner.next();
	let mut erased: Box<dyn Iterator<Item = u8>> = Box::new(a % 4..b % 8);
	let reborrowed: &mut dyn Iterator<Item = u8> = &mut erased;
	let third = reborrowed.next();
	let mut doubled = Box::new(Box::new(Countdown(a % 8)));
	let deep: &mut dyn Iterator<Item = u8> = &mut doubled;
	let fourth = deep.next();
	let mapped: Vec<u8> = Box::new(Countdown(b % 8)).map(|x| x ^ 1).collect();
	put(
		out,
		&format!(
			"{:?} {first:?} {second:?} {third:?} {fourth:?} {mapped:?} {} {} ",
			nested.collect::<Vec<_>>(),
			erased.count(),
			doubled.count()
		),
	);
	dropped(out);
	drop(inner);
	dropped(out);
	// Shared boxes, and boxes and references in any order, each of which std forwards through.
	let (own, range) = (Box::new(Countdown(a % 8)), Box::new(a % 4..b % 8));
	let seen: [&dyn Iterator<Item = u8>; 2] = [&own, &range];
	let hints = seen.map(|it| it.size_hint());
	let mut held = Box::new(Countdown(b % 8));
	let mut lent = &mut held;
	let mut layered: Box<&mut &mut Box<Countdown>> = Box::new(&mut lent);
	let through: &mut dyn Iterator<Item = u8> = &mut layered;
	let fifth = through.next();
	let sixth = layered.next();
	drop(layered);
	put(out, &format!("{hints:?} {fifth:?} {sixth:?} {} ", held.count()));
	dropped(out);
	drop(own);
	dropped(out);
	// A box is no `TrustedLen` iterator, though what it holds is, so std collects it by `next`.
	let seen = RefCell::new(Vec::new());
	let zipped = || {
		let seen = &seen;
		(0..a % 4)
			.chain(2..2)
			.map(move |x| {
				seen.borrow_mut().push(x);
				x
			})
			.zip(0..b % 4)
	};
	let owned: Vec<(u8, u8)> = Box::new(zipped()).collect();
	let mut boxed = Box::new(zipped());
	let lent: Vec<(u8, u8)> = (&mut boxed).collect();
	put(out, &format!("{owned:?} {lent:?} {:?} ", seen.borrow()));

	// Returned through a trait object of the crate's own trait.
	let split: &dyn Chunks = &Lease(rest.to_vec(), usize::from(b % 4) + 1);
	let lens: Vec<usize> = split.chunks().map(<[u8]>::len).collect();
	put(out, &format!("{lens:?} {} ", split.chunks().count()));

	// Fused: through the object, a `fuse` keeps an iterator that resumes at its end, and the
	// crate's own fused iterator, done.
	let mut fused = Resuming(a % 4).fuse();
	put(out, &format!("{} ", drain(&mut fused)));
	let mut resuming = Resuming(b % 4);
	let up: &mut dyn Iterator<Item = u8> = &mut resuming;
	put(
		out,
		&format!("{:?} ", (up.next(), up.next(), up.next(), up.next(), up.next())),
	);
	let mut boxed: Box<dyn FusedIterator<Item = u8>> = Box::new(Countdown(a % 8));
	put(out, &format!("{} ", drain(&mut *boxed)));
	drop(boxed);
	dropped(out);

	// Shared: the vtable's `&self` methods through any `Arc`, `next` only through the one
	// `get_mut` lends, and the items left drop with the last `Arc`.
	let concrete = Arc::new(tagged(front).into_iter());
	let mut shared: Arc<dyn Iterator<Item = Tagged>> = concrete;
	let other = shared.clone();
	let weak = Arc::downgrade(&other);
	put(
		out,
		&format!("{:?} {} ", other.size_hint(), Arc::get_mut(&mut shared).is_none()),
	);
	drop(other);
	drop(weak);
	let taken = Arc::get_mut(&mut shared).and_then(|it| it.next()).map(|t| t.0);
	dropped(out);
	put(out, &format!("{taken:?} "));
	drop(shared);
	dropped(out);
	let exact: Rc<dyn ExactSizeIterator<Item = u8>> = Rc::new(back.to_vec().into_iter());
	let mut up: Rc<dyn Iterator<Item = u8>> = exact.clone();
	put(
		out,
		&format!("{} {:?} ", exact.len(), Rc::get_mut(&mut up).map(|it| it.next())),
	);
	drop(exact);
	let next = Rc::get_mut(&mut up).and_then(|it| it.next());
	put(out, &format!("{next:?} {:?} ", up.size_hint()));
	let boxed: Box<dyn Iterator<Item = u8>> = Box::new(Countdown(b % 8));
	let mut from_box: Arc<dyn Iterator<Item = u8>> = Arc::from(boxed);
	let first = Arc::get_mut(&mut from_box).and_then(|it| it.next());
	put(out, &format!("{first:?} "));
	drop(from_box);
	dropped(out);
	let counted = Rc::new(Countdown(a % 8));
	let weak: rc::Weak<Countdown> = Rc::downgrade(&counted);
	let weak: rc::Weak<dyn Iterator<Item = u8>> = weak;
	put(out, &format!("{:?} ", weak.upgrade().map(|it| it.size_hint())));
	drop(counted);
	put(out, &format!("{} ", weak.upgrade().is_none()));
	dropped(out);
	let mut pinned: Pin<Box<dyn Iterator<Item = u8> + Unpin>> = Box::pin(Countdown(b % 8));
	put(out, &format!("{:?}", pinned.as_mut().next()));
	drop(pinned);
	dropped(out);
}

/// Counts down to its end, then starts over, as a fused iterator never does.
struct Resuming(u8);

impl Iterator for Resuming {
	type Item = u8;

	fn next(&mut self) -> Option<u8> {
		match self.0.checked_sub(1) {
			Some(n) => {
				self.0 = n;
				Some(n)
			}
			None => {
				self.0 = 2;
				None
			}
		}
	}
}

impl FusedIterator for Countdown {}

/// What `it` yields up to its end and twice past it.
fn drain(it: &mut dyn FusedIterator<Item = u8>) -> String {
	let items: Vec<u8> = (&mut *it).collect();
	format!("{items:?} {:?} {:?}", it.next(), it.next())
}

/// A shape that notes its id in a log as it drops.
struct Noisy(u8, Rc<RefCell<Vec<u8>>>);

impl Shape for Noisy {
	fn area(&self) -> u32 {
		u32::from(self.0)
	}
	fn grow(&mut self, by: u8) {
		self.0 = self.0.wrapping_add(by);
	}
}

impl Drop for Noisy {
	fn drop(&mut self) {
		self.1.borrow_mut().push(self.0);
	}
}

/// A shape holding a `Weak` of itself as a trait object, made inside `Rc::new_cyclic`.
struct Looped {
	side: u8,
	me: rc::Weak<dyn Shape>,
}

impl Shape for Looped {
	fn area(&self) -> u32 {
		u32::from(self.side) + 1
	}
	fn grow(&mut self, by: u8) {
		self.side = self.side.wrapping_add(by);
	}
}

/// Trait objects behind the rest of the pointers std coerces to one: an `Rc`, a `Weak` of
/// an `Rc` or an `Arc`, a `Pin`, and an `Rc` or an `Arc` made from a box of one. Counts,
/// upgrades, `downcast`, and what drops when.
pub fn pointers(data: &[u8], out: &mut Vec<u8>) {
	let [a, b, rest @ ..] = data else {
		return;
	};
	let log = Rc::new(RefCell::new(Vec::new()));

	// An `Rc` of each, an upcast, and a `Weak` unsized from one of the value's own type.
	let sq = Rc::new(Square { side: u16::from(*a) });
	let weak: rc::Weak<Square> = Rc::downgrade(&sq);
	let weak: rc::Weak<dyn Shape> = weak;
	let named: Rc<dyn Named> = sq.clone();
	let shape: Rc<dyn Shape> = named.clone();
	let other = Rc::downgrade(&shape);
	put(
		out,
		&format!(
			"{} {} {} {} {} {} ",
			named.label(),
			shape.area(),
			Rc::strong_count(&sq),
			rc::Weak::strong_count(&weak),
			rc::Weak::weak_count(&weak),
			rc::Weak::ptr_eq(&weak, &other),
		),
	);
	let again = weak.clone();
	drop(sq);
	drop(named);
	put(
		out,
		&format!("{:?} {} ", again.upgrade().map(|s| s.area()), Rc::strong_count(&shape)),
	);
	drop(shape);
	put(
		out,
		&format!(
			"{} {} {} ",
			weak.upgrade().is_none(),
			again.upgrade().is_none(),
			rc::Weak::strong_count(&weak)
		),
	);

	// Of an `Arc`, whose value drops with the last strong one while a `Weak` stays.
	let noisy = Arc::new(Noisy(*b, log.clone()));
	let weak: Weak<Noisy> = Arc::downgrade(&noisy);
	let weak: Weak<dyn Shape> = weak;
	put(out, &format!("{:?} ", weak.upgrade().map(|s| s.area())));
	drop(noisy);
	put(out, &format!("{:?} {:?} ", weak.upgrade().is_none(), log.borrow()));

	// One coerced inside `new_cyclic`, before the value it points at is there.
	let looped = Rc::new_cyclic(|me: &rc::Weak<Looped>| Looped {
		side: *a,
		me: me.clone(),
	});
	put(
		out,
		&format!(
			"{:?} {} {} ",
			looped.me.upgrade().map(|s| s.area()),
			Rc::strong_count(&looped),
			Rc::weak_count(&looped),
		),
	);

	// Pinned: through a box, which drops what it holds, and through a `&mut`.
	{
		let pinned: Pin<Box<dyn Shape>> = Box::pin(Noisy(a.wrapping_add(1), log.clone()));
		let mut n = u32::from(*b);
		let borrowed: Pin<&mut u32> = Pin::new(&mut n);
		let borrowed: Pin<&mut dyn Shape> = borrowed;
		put(
			out,
			&format!("{} {} {} ", pinned.area(), pinned.sides(), borrowed.area()),
		);
	}
	put(out, &format!("{:?} ", log.borrow()));

	// From a box of one: an `Rc` or an `Arc` that drops it with the last clone.
	let boxes: Vec<Box<dyn Shape>> = rest
		.iter()
		.map(|&x| match x % 3 {
			0 => Box::new(Noisy(x, log.clone())) as Box<dyn Shape>,
			1 => Box::new(u32::from(x)),
			_ => Box::new(Tri::Tall(x)),
		})
		.collect();
	let mut rcs: Vec<Rc<dyn Shape>> = Vec::new();
	let mut arcs: Vec<Arc<dyn Shape>> = Vec::new();
	for (i, boxed) in boxes.into_iter().enumerate() {
		match i % 2 {
			0 => rcs.push(Rc::from(boxed)),
			_ => arcs.push(boxed.into()),
		}
	}
	let kept: Vec<Rc<dyn Shape>> = rcs.iter().step_by(2).cloned().collect();
	let areas: Vec<u32> = rcs
		.iter()
		.map(|s| s.area())
		.chain(arcs.iter().map(|s| s.area()))
		.collect();
	put(out, &format!("{areas:?} "));
	log.borrow_mut().clear();
	drop(arcs);
	put(out, &format!("{:?} ", log.borrow()));
	drop(rcs);
	put(out, &format!("{:?} ", log.borrow()));
	drop(kept);
	put(out, &format!("{:?} ", log.borrow()));

	// `Rc::downcast`, of one made from the value's own `Rc`, and of one made from a box,
	// which shares its count with what it gives back.
	let any: Rc<dyn Any> = Rc::new(Square { side: u16::from(*b) });
	let wrong = any.clone().downcast::<Tri>().is_err();
	match any.downcast::<Square>() {
		Ok(sq) => put(out, &format!("{wrong} {} {} ", sq.side, Rc::strong_count(&sq))),
		Err(_) => put(out, "none "),
	}
	let boxed: Box<dyn Any> = Box::new(Noisy(b.wrapping_add(1), log.clone()));
	let any: Rc<dyn Any> = Rc::from(boxed);
	let held = any.clone();
	// Two downcasts of one, and each erased again, are still the one allocation.
	if let (Ok(x), Ok(y)) = (any.clone().downcast::<Noisy>(), held.clone().downcast::<Noisy>()) {
		let (ex, ey): (Rc<dyn Any>, Rc<dyn Any>) = (x.clone(), y.clone());
		put(
			out,
			&format!(
				"{} {} {} {} ",
				Rc::ptr_eq(&x, &y),
				Rc::ptr_eq(&ex, &ey),
				Rc::ptr_eq(&ex, &held),
				Rc::strong_count(&x),
			),
		);
	}
	log.borrow_mut().clear();
	match any.downcast::<Noisy>() {
		Ok(n) => {
			put(out, &format!("{} {} ", n.0, Rc::strong_count(&n)));
			drop(held);
			put(out, &format!("{} {:?} ", Rc::strong_count(&n), log.borrow()));
		}
		Err(_) => put(out, "none "),
	}
	put(out, &format!("{:?} ", log.borrow()));
	let boxed: Box<dyn Any + Send + Sync> = Box::new(u32::from(*a));
	let shared: Arc<dyn Any + Send + Sync> = boxed.into();
	put(out, &format!("{:?} ", shared.downcast::<u32>().ok().map(|n| *n)));

	// Functions, shared by an `Rc` and from a box.
	let f: Rc<dyn Fn(u8) -> u8> = Rc::new(move |x| x.wrapping_add(*a));
	let boxed: Box<dyn Fn(u8) -> u8> = Box::new(move |x| x ^ *b);
	let g: Arc<dyn Fn(u8) -> u8> = Arc::from(boxed);
	put(out, &format!("{} {}", f(*b), g(*a)));
}

/// A tag of `t`, below 0x80, which logs it with the high bit set as it is made.
fn made(t: u8) -> Tagged {
	DROPS.with_borrow_mut(|d| d.push(t | 0x80));
	Tagged(t)
}

/// A skip of a zip std reads by index, of items that log as they are made and dropped, the
/// left side's below 0x40 and the right's above: its `next` skips through the zip's `nth`,
/// which makes and drops each skipped pair a side at a time, where its folds make the pair
/// whole.
fn skipped(data: &[u8]) -> impl DoubleEndedIterator<Item = (Tagged, Tagged)> + ExactSizeIterator + '_ {
	let (n, rest) = data.split_first().map_or((0, data), |(n, rest)| (*n, rest));
	let (left, right) = rest.split_at(usize::from(n) % (rest.len() + 1));
	let left = left.iter().map(|&t| made(t & 0x3f));
	left.zip(right.iter().map(|&t| made(t & 0x3f | 0x40)))
		.skip(usize::from(n % 3))
}

fn tags(pair: Option<(Tagged, Tagged)>) -> Option<(u8, u8)> {
	pair.map(|p| (p.0.0, p.1.0))
}

/// A box of an iterator, owned, behind a `&mut`, of a `&mut`, of a box, and as a trait
/// object: std's `Iterator` for `Box<I>` forwards `next`, `size_hint`, `nth`, `last`, and
/// the double-ended and exact-size methods to `I`, and reads by `next` for the rest, its
/// `advance_by` and folds included.
pub fn boxes(data: &[u8], out: &mut Vec<u8>) {
	let it = || Box::new(skipped(data));

	// What it forwards.
	let mut boxed = it();
	let hint = (boxed.size_hint(), boxed.len());
	let ends = (tags(boxed.next()), tags(boxed.next_back()));
	let nths = (tags(boxed.nth(1)), tags(boxed.nth_back(0)));
	dropped(out);
	put(out, &format!("{hint:?} {ends:?} {nths:?} "));
	drop(boxed);
	dropped(out);
	put(out, &format!("{:?} ", tags(it().last())));
	dropped(out);
	put(out, &format!("{:?} ", tags(Box::new(it()).last())));
	dropped(out);

	// What it reads by `next`.
	put(out, &format!("{} ", it().count()));
	dropped(out);
	let sum = it().fold(0u32, |n, (x, y)| n.wrapping_mul(7).wrapping_add(u32::from(x.0 ^ y.0)));
	dropped(out);
	put(out, &format!("{sum} "));
	it().for_each(|(x, y)| put(out, &format!("{} {} ", x.0, y.0)));
	dropped(out);
	let found = it().position(|(x, _)| x.0 % 2 == 1);
	dropped(out);
	put(out, &format!("{found:?} "));
	let found = tags(it().find(|(_, y)| y.0 % 3 == 0));
	dropped(out);
	put(out, &format!("{found:?} {} ", it().any(|(x, _)| x.0 > 8)));
	dropped(out);
	put(out, &format!("{:?} ", tags(it().max_by_key(|p| p.1.0))));
	dropped(out);
	let all: Vec<(u8, u8)> = it().collect::<Vec<_>>().into_iter().map(|p| (p.0.0, p.1.0)).collect();
	dropped(out);
	put(out, &format!("{all:?} "));
	let back = it()
		.rev()
		.fold(0u32, |n, (x, y)| n.wrapping_mul(5).wrapping_add(u32::from(x.0 ^ y.0)));
	dropped(out);
	put(
		out,
		&format!(
			"{back} {} ",
			it().map(|(x, y)| u32::from(x.0) + u32::from(y.0)).sum::<u32>()
		),
	);
	dropped(out);
	put(out, &format!("{} {} ", it().skip(1).count(), it().step_by(2).count()));
	dropped(out);
	// `Chain::nth` advances its first half by `advance_by`, which the box reads by `next`.
	put(out, &format!("{:?} ", tags(it().chain(None).nth(1))));
	dropped(out);

	// Behind a `&mut`, which forwards to the box, and so reads by `next` too, `last` included.
	let mut boxed = it();
	let last = tags((&mut boxed).last());
	dropped(out);
	put(out, &format!("{last:?} "));
	let mut boxed = it();
	let nth = tags((&mut boxed).chain(None).nth(1));
	dropped(out);
	let n = (&mut boxed).count();
	dropped(out);
	put(out, &format!("{nth:?} {n} "));
	let mut boxed = it();
	let first = tags(boxed.next());
	let sum = (&mut boxed).fold(0u32, |n, (x, y)| n.wrapping_add(u32::from(x.0 ^ y.0)));
	dropped(out);
	put(out, &format!("{first:?} {sum} "));
	drop(boxed);
	dropped(out);

	// Of items that never drop: `last` is still what the box holds, whose zip folds no more
	// pairs than its size hint, where `next` reads its left side once more.
	let n = data.first().map_or(0, |n| n % 4);
	let seen = RefCell::new(Vec::new());
	let logged = || {
		let seen = &seen;
		let left = (0..n).chain(n..n + 2).map(move |x| {
			seen.borrow_mut().push(x);
			x
		});
		Box::new(left.zip(0..n % 3))
	};
	let last = logged().last();
	let mut boxed = logged();
	put(out, &format!("{last:?} {:?} {:?} ", (&mut boxed).last(), seen.borrow()));

	// A box of a `&mut`, whose `last` is the borrow's, which folds what it borrows.
	let mut held = skipped(data);
	put(out, &format!("{:?} ", tags(Box::new(&mut held).last())));
	dropped(out);
	let mut held = skipped(data);
	put(out, &format!("{} ", Box::new(&mut held).count()));
	dropped(out);

	// A box of a trait object, which reads by `next` where the object reads by its vtable.
	let erased = || -> Box<dyn Iterator<Item = (Tagged, Tagged)> + '_> { Box::new(skipped(data)) };
	put(out, &format!("{:?} ", tags(erased().chain(None).nth(1))));
	dropped(out);
	put(out, &format!("{:?} {} ", tags(erased().last()), erased().count()));
	dropped(out);
	let mut boxed = erased();
	let lent: &mut dyn Iterator<Item = (Tagged, Tagged)> = &mut *boxed;
	put(out, &format!("{:?} ", tags(lent.chain(None).nth(1))));
	dropped(out);
	drop(boxed);
	dropped(out);
}

/// A struct whose last field unsizes, as a `RefCell`'s and a `Mutex`'s does: the fields
/// before it stay where they are, and drop before it.
struct Tail<T: ?Sized> {
	count: u8,
	head: Noisy,
	value: T,
}

/// A struct with a `Drop` impl of its own, which runs before its fields drop.
struct Guarded<T: ?Sized> {
	log: Rc<RefCell<Vec<u8>>>,
	value: T,
}

impl<T: ?Sized> Drop for Guarded<T> {
	fn drop(&mut self) {
		self.log.borrow_mut().push(0xee);
	}
}

/// An enum, which JS copies, holding a value that drops.
enum Held {
	Empty,
	Full(Noisy),
}

impl Shape for Held {
	fn area(&self) -> u32 {
		match self {
			Held::Empty => 0,
			Held::Full(n) => n.area(),
		}
	}
	fn grow(&mut self, by: u8) {
		match self {
			Held::Empty => {}
			Held::Full(n) => n.grow(by),
		}
	}
}

/// Through a `&mut` to a struct whose last field is a trait object.
fn bump_tail(t: &mut Tail<dyn Shape>, by: u8) -> u32 {
	t.count = t.count.wrapping_add(by);
	t.head.grow(by);
	t.value.grow(by);
	t.value.area()
}

pub fn tails(data: &[u8], out: &mut Vec<u8>) {
	let [a, b, c, rest @ ..] = data else {
		return;
	};
	let (a, b, c) = (*a, *b, *c);
	let log = Rc::new(RefCell::new(Vec::new()));

	// Each `Rc<RefCell<dyn Shape>>` shares its cell, and the cell's borrows, with the
	// value's own `Rc`.
	let sq = Rc::new(RefCell::new(Square { side: u16::from(a) }));
	let n = Rc::new(RefCell::new(u32::from(b)));
	let tri = Rc::new(RefCell::new(Tri::Tall(c)));
	let shapes: Vec<Rc<RefCell<dyn Shape>>> = vec![
		sq.clone(),
		n.clone(),
		tri.clone(),
		Rc::new(RefCell::new(Noisy(a ^ b, log.clone()))),
	];
	for (i, s) in shapes.iter().enumerate() {
		s.borrow_mut().grow(c.wrapping_add(i as u8));
	}
	let areas: Vec<u32> = shapes.iter().map(|s| s.borrow().area()).collect();
	put(
		out,
		&format!("{areas:?} {} {} {:?} ", sq.borrow().side, n.borrow(), tri.borrow()),
	);
	{
		let held = n.borrow_mut();
		let shared = shapes[0].borrow();
		put(
			out,
			&format!(
				"{} {} {} ",
				shapes[1].try_borrow().is_err(),
				sq.try_borrow_mut().is_err(),
				shapes[0].try_borrow().map(|s| s.sides()).unwrap_or(0),
			),
		);
		drop(held);
		drop(shared);
		let r = shapes[2].borrow();
		put(out, &format!("{} {} ", tri.try_borrow_mut().is_err(), r.sides()));
	}
	put(
		out,
		&format!("{} {} ", Rc::strong_count(&sq), Rc::strong_count(&shapes[1])),
	);

	// A `dyn Named`, written through its `Debug`, and upcast inside the cell.
	let named: Rc<RefCell<dyn Named>> = Rc::new(RefCell::new(Tri::Flat));
	named.borrow_mut().grow(b);
	let upcast: Rc<RefCell<dyn Shape>> = named.clone();
	upcast.borrow_mut().grow(a);
	put(
		out,
		&format!("{:?} {} {} ", named, named.borrow().label(), upcast.borrow().area()),
	);
	{
		let _held = named.borrow_mut();
		put(out, &format!("{named:?} "));
	}

	// Dropped by the last `Rc` of either kind.
	let noisy = Rc::new(RefCell::new(Noisy(c, log.clone())));
	let shape: Rc<RefCell<dyn Shape>> = noisy.clone();
	let weak: rc::Weak<RefCell<dyn Shape>> = Rc::downgrade(&shape);
	drop(noisy);
	put(
		out,
		&format!("{:?} {:?} ", log.borrow(), weak.upgrade().map(|s| s.borrow().area())),
	);
	drop(shape);
	put(out, &format!("{:?} {} ", log.borrow(), weak.upgrade().is_none()));
	drop(shapes);
	put(out, &format!("{:?} ", log.borrow()));
	log.borrow_mut().clear();

	// An `Arc<Mutex<dyn Shape>>` and an `Arc<RwLock<dyn Shape>>` lock the value's own.
	let m = Arc::new(Mutex::new(Tri::Flat));
	let locked: Arc<Mutex<dyn Shape>> = m.clone();
	locked.lock().unwrap().grow(b);
	{
		let _guard = m.lock().unwrap();
		put(out, &format!("{} ", locked.try_lock().is_err()));
	}
	let weak: Weak<Mutex<dyn Shape>> = Arc::downgrade(&locked);
	let area = weak.upgrade().map(|l| l.lock().unwrap().area());
	put(
		out,
		&format!("{:?} {area:?} {} ", m.lock().unwrap(), Arc::strong_count(&m)),
	);
	let rw = Arc::new(RwLock::new(u32::from(c)));
	let lock: Arc<RwLock<dyn Shape>> = rw.clone();
	lock.write().unwrap().grow(a);
	{
		let _read = rw.read().unwrap();
		put(
			out,
			&format!("{} {} ", lock.try_write().is_err(), lock.read().unwrap().area()),
		);
	}
	put(out, &format!("{} ", rw.read().unwrap()));

	// The crate's own struct, borrowed, changed in place, nested in a cell, and boxed.
	let mut t = Tail {
		count: a,
		head: Noisy(b, log.clone()),
		value: u32::from(c),
	};
	let area = bump_tail(&mut t, b);
	let shared: &Tail<dyn Shape> = &t;
	put(
		out,
		&format!(
			"{area} {} {} {} {} ",
			shared.count,
			shared.head.0,
			shared.value.area(),
			t.value
		),
	);
	let cell = Rc::new(RefCell::new(Tail {
		count: b,
		head: Noisy(a, log.clone()),
		value: Square { side: u16::from(b) },
	}));
	let nested: Rc<RefCell<Tail<dyn Shape>>> = cell.clone();
	let area = bump_tail(&mut nested.borrow_mut(), c);
	put(
		out,
		&format!("{area} {} {} ", cell.borrow().count, cell.borrow().value.side),
	);
	drop(cell);
	drop(t);
	put(out, &format!("{:?} ", log.borrow()));
	drop(nested);
	let boxed: Box<Tail<dyn Shape>> = Box::new(Tail {
		count: c,
		head: Noisy(c, log.clone()),
		value: Noisy(c.wrapping_add(1), log.clone()),
	});
	put(out, &format!("{} ", boxed.value.area()));
	drop(boxed);
	put(out, &format!("{:?} ", log.borrow()));

	// Pinned, and a `Weak` unsized from one of the value's own type.
	let pinned: Pin<Box<Tail<dyn Shape>>> = Box::pin(Tail {
		count: a,
		head: Noisy(a, log.clone()),
		value: Tri::Tall(b),
	});
	let own = Rc::new(RefCell::new(u32::from(a)));
	let weak: rc::Weak<RefCell<u32>> = Rc::downgrade(&own);
	let weak: rc::Weak<RefCell<dyn Shape>> = weak;
	if let Some(s) = weak.upgrade() {
		s.borrow_mut().grow(c);
	}
	put(
		out,
		&format!(
			"{} {} {} {} ",
			pinned.count,
			pinned.value.area(),
			own.borrow(),
			rc::Weak::strong_count(&weak)
		),
	);
	drop(pinned);
	put(out, &format!("{:?} ", log.borrow()));
	let boxed: Box<RefCell<dyn Shape>> = Box::new(RefCell::new(Noisy(b, log.clone())));
	let shared: Rc<RefCell<dyn Shape>> = Rc::from(boxed);
	let again = shared.clone();
	again.borrow_mut().grow(a);
	drop(shared);
	put(out, &format!("{} {:?} ", again.borrow().area(), log.borrow()));
	drop(again);
	put(out, &format!("{:?} ", log.borrow()));

	// A struct whose own `Drop` runs first, then its last field's, of each kind of value.
	let guarded: Vec<Box<Guarded<dyn Shape>>> = vec![
		Box::new(Guarded {
			log: log.clone(),
			value: Held::Full(Noisy(a, log.clone())),
		}),
		Box::new(Guarded {
			log: log.clone(),
			value: Noisy(b, log.clone()),
		}),
		Box::new(Guarded {
			log: log.clone(),
			value: Held::Empty,
		}),
	];
	let cell: Rc<RefCell<Guarded<dyn Shape>>> = Rc::new(RefCell::new(Guarded {
		log: log.clone(),
		value: Held::Full(Noisy(c, log.clone())),
	}));
	cell.borrow_mut().value.grow(a);
	let areas: Vec<u32> = guarded.iter().map(|g| g.value.area()).collect();
	put(out, &format!("{areas:?} {} ", cell.borrow().value.area()));
	log.borrow_mut().clear();
	drop(guarded);
	put(out, &format!("{:?} ", log.borrow()));
	drop(cell);
	put(out, &format!("{:?} ", log.borrow()));

	// Two made of one value are one pointer, and two of two values two.
	{
		let t = Tail {
			count: a,
			head: Noisy(a, log.clone()),
			value: u32::from(b),
		};
		let apart = Tail {
			count: a,
			head: Noisy(b, log.clone()),
			value: u32::from(b),
		};
		let (p, q, o): (&Tail<dyn Shape>, &Tail<dyn Shape>, &Tail<dyn Shape>) = (&t, &t, &apart);
		let cell = Rc::new(RefCell::new(u32::from(c)));
		let (r, s): (Rc<RefCell<dyn Shape>>, Rc<RefCell<dyn Shape>>) = (cell.clone(), cell.clone());
		let other: Rc<RefCell<dyn Shape>> = Rc::new(RefCell::new(u32::from(c)));
		let (u, v) = (Rc::downgrade(&r), Rc::downgrade(&s));
		put(
			out,
			&format!(
				"{} {} {} {} {} ",
				std::ptr::eq(p, q),
				std::ptr::eq(p, o),
				Rc::ptr_eq(&r, &s),
				Rc::ptr_eq(&r, &other),
				rc::Weak::ptr_eq(&u, &v),
			),
		);
	}
	log.borrow_mut().clear();

	// Of std's traits: a function, a writer of `Debug`, a `dyn Any`, and an iterator.
	let mut sum = 0u32;
	{
		let f: Rc<RefCell<dyn FnMut(u8)>> = Rc::new(RefCell::new(|x: u8| sum = sum.wrapping_add(u32::from(x))));
		for &x in rest {
			(f.borrow_mut())(x);
		}
	}
	let shown: &RefCell<dyn Debug> = &RefCell::new(Tri::Tall(c));
	let any: Box<Tail<dyn Any>> = Box::new(Tail {
		count: b,
		head: Noisy(b, log.clone()),
		value: u32::from(c),
	});
	let it: Arc<Mutex<dyn Iterator<Item = u8>>> = Arc::new(Mutex::new(rest.iter().copied().rev()));
	let next = it.lock().unwrap().next();
	put(
		out,
		&format!(
			"{sum} {shown:?} {:?} {} {next:?} {:?} ",
			any.value.downcast_ref::<u32>(),
			any.value.is::<u8>(),
			it.lock().unwrap().next(),
		),
	);

	// A borrow or a lock that conflicts, through the trait object, panics as it does natively.
	match rest.first().map(|x| x % 4) {
		Some(0) => {
			let _shared = n.borrow();
			let dynamic: Rc<RefCell<dyn Shape>> = n.clone();
			dynamic.borrow_mut().grow(1);
		}
		Some(1) => {
			let dynamic: Rc<RefCell<dyn Shape>> = n.clone();
			let _held = dynamic.borrow_mut();
			put(out, &format!("{}", n.borrow()));
		}
		Some(2) => {
			let _guard = m.lock().unwrap();
			put(out, &format!("{}", locked.lock().unwrap().area()));
		}
		_ => put(out, &format!("{}", n.borrow())),
	}
}

/// A future of the crate's own, which JS keeps as an object: pending until it has counted
/// down, waking its waker each time, and noting its count as it drops.
struct Ticks {
	left: u8,
	polls: u8,
	log: Rc<RefCell<Vec<u8>>>,
}

impl Future for Ticks {
	type Output = u8;

	fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<u8> {
		self.polls = self.polls.wrapping_add(1);
		match self.left.checked_sub(1) {
			Some(left) => {
				self.left = left;
				cx.waker().wake_by_ref();
				Poll::Pending
			}
			None => Poll::Ready(self.polls),
		}
	}
}

impl Drop for Ticks {
	fn drop(&mut self) {
		self.log.borrow_mut().push(0xf0 | self.polls);
	}
}

/// Polls `f` until it is ready, at most `max` times: how many it took and its output.
fn poll_tail(f: &Mutex<dyn Future<Output = u8> + Unpin>, max: u8) -> (u8, Option<u8>) {
	let mut cx = Context::from_waker(Waker::noop());
	for i in 0..max {
		if let Poll::Ready(v) = Pin::new(&mut *f.lock().unwrap()).poll(&mut cx) {
			return (i + 1, Some(v));
		}
	}
	(max, None)
}

/// Through a raw pointer to a trait object, as `&mut` to it would.
unsafe fn grow_raw(p: *mut dyn Shape, by: u8) -> u32 {
	unsafe {
		(*p).grow(by);
		(*p).area()
	}
}

pub fn rests(data: &[u8], out: &mut Vec<u8>) {
	let [a, b, c, ..] = data else {
		return;
	};
	let (a, b, c) = (*a, *b, *c);
	let log = Rc::new(RefCell::new(Vec::new()));

	// A `Cell`, changed in place through `get_mut` and read through `as_ptr`, of each kind
	// of value, and shared, dropping its value with the last `Rc`.
	let mut n = Cell::new(u32::from(a));
	let mut sq = Cell::new(Square { side: u16::from(b) });
	let mut tri = Cell::new(Tri::Tall(c));
	{
		let cells: [&mut Cell<dyn Shape>; 3] = [&mut n, &mut sq, &mut tri];
		for (i, cell) in cells.into_iter().enumerate() {
			cell.get_mut().grow(c.wrapping_add(i as u8));
		}
	}
	let shared: &Cell<dyn Shape> = &tri;
	let area = unsafe { (*shared.as_ptr()).area() };
	put(
		out,
		&format!("{} {} {:?} {area} ", n.get(), sq.get_mut().side, tri.get()),
	);
	let noisy = Rc::new(Cell::new(Noisy(a, log.clone())));
	let cell: Rc<Cell<dyn Shape>> = noisy.clone();
	let area = unsafe { grow_raw(cell.as_ptr(), b) };
	drop(noisy);
	put(out, &format!("{area} {:?} ", log.borrow()));
	drop(cell);
	put(out, &format!("{:?} ", log.borrow()));
	log.borrow_mut().clear();

	// An `UnsafeCell`, written through `get` in the value's own cell, and boxed.
	let u = Rc::new(UnsafeCell::new(Tri::Flat));
	let shape: Rc<UnsafeCell<dyn Shape>> = u.clone();
	unsafe { (*shape.get()).grow(a) };
	let area = unsafe { grow_raw(shape.get(), b) };
	let mut boxed: Box<UnsafeCell<dyn Shape>> = Box::new(UnsafeCell::new(Noisy(c, log.clone())));
	boxed.get_mut().grow(a);
	put(
		out,
		&format!("{area} {:?} {} ", unsafe { *u.get() }, unsafe { (*boxed.get()).area() }),
	);
	drop(boxed);
	put(out, &format!("{:?} ", log.borrow()));
	log.borrow_mut().clear();

	// A `ManuallyDrop`, changed in place, whose box drops nothing and whose `drop` drops
	// the value once.
	let mut kept = ManuallyDrop::new(Noisy(a, log.clone()));
	let mut held = ManuallyDrop::new(Tri::Tall(b));
	{
		let m: &mut ManuallyDrop<dyn Shape> = &mut kept;
		m.grow(c);
		let t: &mut ManuallyDrop<dyn Shape> = &mut held;
		t.grow(a);
		put(out, &format!("{} {} ", m.area(), t.area()));
	}
	let boxed: Box<ManuallyDrop<dyn Shape>> = Box::new(ManuallyDrop::new(Noisy(b, log.clone())));
	put(out, &format!("{} {} {:?} ", boxed.area(), kept.0, *held));
	drop(boxed);
	put(out, &format!("{:?} ", log.borrow()));
	{
		let m: &mut ManuallyDrop<dyn Shape> = &mut kept;
		unsafe { ManuallyDrop::drop(m) };
	}
	put(out, &format!("{:?} ", log.borrow()));
	log.borrow_mut().clear();

	// A future in a lock, polled through a pin of the guard: each poll advances the one
	// future, which drops once, with the last `Arc`.
	let ticks = Arc::new(Mutex::new(Ticks {
		left: a % 4,
		polls: 0,
		log: log.clone(),
	}));
	let future: Arc<Mutex<dyn Future<Output = u8> + Unpin>> = ticks.clone();
	let first = poll_tail(&future, b % 3);
	let polls = ticks.lock().unwrap().polls;
	let rest = poll_tail(&future, 8);
	drop(ticks);
	put(out, &format!("{first:?} {polls} {rest:?} {:?} ", log.borrow()));
	drop(future);
	put(out, &format!("{:?} ", log.borrow()));
	log.borrow_mut().clear();
	// An `async` block in a cell, polled through a pin of its borrow, which drops what it
	// owns once, with the last `Rc`.
	let owned = Noisy(c, log.clone());
	let block = Rc::new(RefCell::new(async move { owned.0 }));
	let erased: Rc<RefCell<dyn Future<Output = u8>>> = block.clone();
	let mut cx = Context::from_waker(Waker::noop());
	let ready = {
		let mut borrowed = erased.borrow_mut();
		match unsafe { Pin::new_unchecked(&mut *borrowed) }.poll(&mut cx) {
			Poll::Ready(v) => Some(v),
			Poll::Pending => None,
		}
	};
	drop(block);
	put(out, &format!("{ready:?} {:?} ", log.borrow()));
	drop(erased);
	put(out, &format!("{:?} ", log.borrow()));
	log.borrow_mut().clear();

	// A raw pointer unsized to a trait object, and to a struct of one, writing the original.
	let mut side = Square { side: u16::from(a) };
	let mut num = u32::from(b);
	let mut t = Tail {
		count: c,
		head: Noisy(c, log.clone()),
		value: Tri::Tall(a),
	};
	let (p, q): (*mut dyn Shape, *mut dyn Shape) = (&raw mut side, &raw mut num);
	let r: *const dyn Shape = &raw const side;
	let tail: *mut Tail<dyn Shape> = &raw mut t;
	let areas = unsafe { (grow_raw(p, c), grow_raw(q, a), (*r).area()) };
	let area = unsafe {
		(*tail).count = (*tail).count.wrapping_add(a);
		grow_raw(&raw mut (*tail).value, b)
	};
	put(
		out,
		&format!("{areas:?} {area} {} {num} {} {:?} ", side.side, t.count, t.value),
	);
	drop(t);
	put(out, &format!("{:?} ", log.borrow()));
	// To a cell of one, read and written in place.
	let mut sq = Cell::new(Square { side: u16::from(c) });
	let mut tri = RefCell::new(Tri::Tall(a));
	let p: *mut Cell<dyn Shape> = &raw mut sq;
	let q: *mut RefCell<dyn Shape> = &raw mut tri;
	let areas = unsafe {
		(*p).get_mut().grow(b);
		(*q).borrow_mut().grow(c);
		((*p).get_mut().area(), (*q).borrow().area())
	};
	put(out, &format!("{areas:?} {} {:?} ", sq.get_mut().side, tri.borrow()));
	// Owned again by `Box::from_raw`, which drops what it points at once.
	let (ticks, noisy, tail) = (
		Box::into_raw(Box::new(Ticks {
			left: b % 3,
			polls: 0,
			log: log.clone(),
		})),
		Box::into_raw(Box::new(Noisy(a, log.clone()))),
		Box::into_raw(Box::new(Tail {
			count: b,
			head: Noisy(b, log.clone()),
			value: Noisy(c, log.clone()),
		})),
	);
	let (ticks, noisy, tail): (*mut dyn Future<Output = u8>, *mut dyn Shape, *mut Tail<dyn Shape>) =
		(ticks, noisy, tail);
	let mut cx = Context::from_waker(Waker::noop());
	let polled = unsafe { Pin::new_unchecked(&mut *ticks) }.poll(&mut cx).is_ready();
	unsafe {
		(*noisy).grow(c);
		drop(Box::from_raw(ticks));
		drop(Box::from_raw(noisy));
		drop(Box::from_raw(tail));
	}
	put(out, &format!("{polled} {:?} ", log.borrow()));
	log.borrow_mut().clear();
	// An iterator of the crate's own in a cell and in a lock, read through each and
	// dropped once, with the last owner.
	let mut cell: Box<Cell<dyn Iterator<Item = u8>>> = Box::new(Cell::new(Countdown(a % 8)));
	let first = cell.get_mut().next();
	let locked: Arc<Mutex<dyn Iterator<Item = u8>>> = Arc::new(Mutex::new(Countdown(b % 8)));
	let second = locked.lock().unwrap().next();
	put(out, &format!("{first:?} {second:?} "));
	drop(cell);
	dropped(out);
	drop(locked);
	dropped(out);
	// To a cell not there yet, read only once it is.
	let mut slot = std::mem::MaybeUninit::<Cell<u32>>::uninit();
	let thin = slot.as_mut_ptr();
	let wide: *mut Cell<dyn Shape> = thin;
	let null = wide.is_null();
	let area = unsafe {
		thin.write(Cell::new(u32::from(a)));
		(*wide).get_mut().grow(b);
		(*wide).get_mut().area()
	};
	put(out, &format!("{null} {area} {} ", unsafe { slot.assume_init() }.get()));
	// Of std's traits: a `Debug` written, and a null pointer, which stays null.
	let shown: *const dyn Debug = &raw const side;
	let null: *const dyn Shape = std::ptr::null::<Square>();
	put(out, &format!("{:?} {}", unsafe { &*shown }, null.is_null()));
}

/// A generic trait.
trait Scaled<T> {
	fn scale(&self, by: T) -> T;
	/// Writes through a `&mut` to the argument, an `rs.Place` for a number.
	fn bump(&self, at: &mut T);
	/// Takes and returns references to the argument, which the trait's type has in each
	/// form a reference may be.
	fn pick<'a>(&self, a: &'a T, b: &'a T) -> &'a T;
	/// Provided, taking a function of a reference to the argument, which the trait's type
	/// takes in each form a reference may be.
	fn apply(&self, f: fn(&T) -> T, x: T) -> T {
		f(&self.scale(x))
	}
}

/// A trait whose method returns an object of it at another argument, and so on without end.
trait Nest<T> {
	fn next(&self) -> Option<&dyn Nest<Vec<T>>>;
	fn depth(&self) -> u8;
}

struct Leaf(u8);

impl<T> Nest<T> for Leaf {
	fn next(&self) -> Option<&dyn Nest<Vec<T>>> {
		None
	}
	fn depth(&self) -> u8 {
		self.0
	}
}

fn doubled(x: &u8) -> u8 {
	x.wrapping_mul(2)
}

/// A generic trait over its supertrait at another argument.
trait Paired<T>: Scaled<Vec<T>> {
	fn pair(&self, x: T) -> (T, T);
}

/// A trait with an associated type.
trait Source {
	type Item;
	fn take(&mut self) -> Option<Self::Item>;
	fn left(&self) -> usize;
}

/// A trait whose supertrait binds an associated type.
trait Counted: Source<Item = u32> {
	fn total(&self) -> u32;
}

struct By(u8);

impl Scaled<u8> for By {
	fn scale(&self, by: u8) -> u8 {
		self.0.wrapping_mul(by)
	}
	fn bump(&self, at: &mut u8) {
		*at = at.wrapping_add(self.0);
	}
	fn pick<'a>(&self, a: &'a u8, b: &'a u8) -> &'a u8 {
		if self.0 % 2 == 0 { a } else { b }
	}
}

/// A number, which JS copies.
impl Scaled<u16> for u16 {
	fn scale(&self, by: u16) -> u16 {
		self.wrapping_mul(by).wrapping_add(1)
	}
	fn bump(&self, at: &mut u16) {
		*at = at.wrapping_sub(*self);
	}
	fn pick<'a>(&self, a: &'a u16, b: &'a u16) -> &'a u16 {
		if a > b { a } else { b }
	}
}

impl Scaled<String> for Tri {
	fn scale(&self, by: String) -> String {
		format!("{by}{self:?}")
	}
	fn bump(&self, at: &mut String) {
		at.push_str(&self.area().to_string());
	}
	fn pick<'a>(&self, a: &'a String, b: &'a String) -> &'a String {
		if a.len() >= b.len() { a } else { b }
	}
}

/// Bytes, which JS keeps apart from other vectors.
struct Many(u8);

impl Scaled<Vec<u8>> for Many {
	fn scale(&self, by: Vec<u8>) -> Vec<u8> {
		by.iter().map(|x| x.wrapping_add(self.0)).collect()
	}
	fn bump(&self, at: &mut Vec<u8>) {
		at.push(self.0);
	}
	fn pick<'a>(&self, a: &'a Vec<u8>, b: &'a Vec<u8>) -> &'a Vec<u8> {
		if a.len() < b.len() { a } else { b }
	}
}

impl Paired<u8> for Many {
	fn pair(&self, x: u8) -> (u8, u8) {
		(x, x.wrapping_add(self.0))
	}
}

/// Owns what it gives out, each dropped where Rust drops it.
struct Drained(Vec<Noisy>);

impl Source for Drained {
	type Item = Noisy;
	fn take(&mut self) -> Option<Noisy> {
		self.0.pop()
	}
	fn left(&self) -> usize {
		self.0.len()
	}
}

struct Steps(u32, u32);

impl Source for Steps {
	type Item = u32;
	fn take(&mut self) -> Option<u32> {
		self.0.checked_sub(1).map(|n| {
			self.0 = n;
			self.1 = self.1.wrapping_add(n);
			n
		})
	}
	fn left(&self) -> usize {
		self.0 as usize
	}
}

impl Counted for Steps {
	fn total(&self) -> u32 {
		self.1
	}
}

/// A copied value, which `&mut self` reaches through a place, giving strings.
impl Source for u8 {
	type Item = String;
	fn take(&mut self) -> Option<String> {
		*self = self.checked_sub(1)?;
		Some(format!("<{self}>"))
	}
	fn left(&self) -> usize {
		usize::from(*self)
	}
}

fn taken<I: Debug>(s: &mut dyn Source<Item = I>) -> String {
	let mut taken = Vec::new();
	while let Some(x) = s.take() {
		taken.push(format!("{x:?}"));
	}
	taken.join(",")
}

impl Debug for Noisy {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "N{}", self.0)
	}
}

/// A generic trait whose method returns its argument, `()` too.
trait Task<T> {
	fn run(&mut self) -> T;
}

impl Task<()> for Steps {
	fn run(&mut self) {
		self.1 = self.1.wrapping_mul(3);
	}
}

impl Task<u32> for Square {
	fn run(&mut self) -> u32 {
		self.side = self.side.wrapping_add(1);
		self.area()
	}
}

/// A generic struct of trait objects of its parameter, which its type has as its own.
struct Over<T> {
	scale: Box<dyn Scaled<T>>,
	source: Box<dyn Source<Item = T>>,
}

impl<T: Clone> Over<T> {
	fn run(&mut self) -> Option<T> {
		let next = self.source.take()?;
		let other = self.scale.scale(next.clone());
		Some(self.scale.pick(&next, &other).clone())
	}
}

pub fn generics(data: &[u8], out: &mut Vec<u8>) {
	let [a, b, c, rest @ ..] = data else {
		return;
	};
	let (a, b, c) = (*a, *b, *c);
	let log = Rc::new(RefCell::new(Vec::new()));

	// Objects of a generic trait at three arguments, each of another type.
	let by = By(a);
	let narrow: &dyn Scaled<u8> = &by;
	let wide: Box<dyn Scaled<u16> + Send> = Box::new(u16::from(b) * 7);
	let tri = if c % 2 == 0 { Tri::Flat } else { Tri::Tall(c) };
	let named: &dyn Scaled<String> = &tri;
	let (mut n, mut w, mut s) = (b, u16::from(c) << 8, String::from("s"));
	narrow.bump(&mut n);
	wide.bump(&mut w);
	named.bump(&mut s);
	put(
		out,
		&format!(
			"{} {} {} {n} {w} {s} ",
			narrow.scale(b),
			wide.scale(u16::from(c) * 300),
			named.scale(format!("{a}"))
		),
	);
	let (x, y) = (String::from("ab"), format!("{c}"));
	put(
		out,
		&format!(
			"{} {} {} ",
			narrow.pick(&b, &c),
			wide.pick(&u16::from(a), &w),
			named.pick(&x, &y)
		),
	);

	// A subtrait at one argument, its supertrait at another, through the subtrait and
	// upcast to the supertrait.
	let many = Many(b);
	let paired: &dyn Paired<u8> = &many;
	let up: &dyn Scaled<Vec<u8>> = paired;
	let mut v = rest.to_vec();
	paired.bump(&mut v);
	let picked = up.pick(&v, &[a].to_vec()).clone();
	put(
		out,
		&format!("{:?} {:?} {v:?} {picked:?} ", paired.pair(c), up.scale(rest.to_vec())),
	);
	// Shared, upcast through the `Arc`, whose count both sides share.
	let shared: Arc<dyn Paired<u8>> = Arc::new(Many(c));
	let up: Arc<dyn Scaled<Vec<u8>>> = shared.clone();
	let mut v = vec![a];
	up.bump(&mut v);
	put(out, &format!("{:?} {v:?} {} ", shared.pair(b), Arc::strong_count(&up)));
	let weak: Weak<dyn Paired<u8>> = Arc::downgrade(&shared);
	let weak: Weak<dyn Scaled<Vec<u8>>> = weak;
	drop(up);
	let scaled = weak.upgrade().map(|s| s.scale(vec![a, b]));
	drop(shared);
	put(out, &format!("{scaled:?} {} ", weak.upgrade().is_none()));
	let nest: &dyn Nest<u8> = &Leaf(c);
	put(
		out,
		&format!(
			"{} {} {} ",
			nest.next().is_none(),
			nest.depth(),
			narrow.apply(doubled, c)
		),
	);

	// Associated types: owned items each dropped as Rust drops them, numbers, and strings
	// from a copied value changed in place.
	let items = rest.iter().map(|&x| Noisy(x, log.clone())).collect();
	let mut owned: Box<dyn Source<Item = Noisy>> = Box::new(Drained(items));
	let first = owned.take();
	put(out, &format!("{} {first:?} {:?} ", owned.left(), log.borrow()));
	drop(first);
	put(out, &format!("{:?} ", log.borrow()));
	if a % 2 == 0 {
		owned.take();
	}
	drop(owned);
	put(out, &format!("{:?} ", log.borrow()));
	let mut steps = Steps(u32::from(a % 6), u32::from(b));
	let counted: &mut dyn Counted = &mut steps;
	let next = counted.take();
	put(out, &format!("{next:?} {} {} ", counted.left(), counted.total()));
	put(out, &format!("{} {} ", taken(counted), steps.1));
	let mut left = c % 5;
	put(out, &format!("{} {left} ", taken(&mut left)));

	// A unit return, and a number's.
	let mut tasks: Vec<Box<dyn Task<()>>> = vec![Box::new(Steps(1, u32::from(a)))];
	let mut square: Box<dyn Task<u32>> = Box::new(Square { side: u16::from(c) });
	tasks[0].run();
	let ran = tasks.iter_mut().map(|t| t.run()).count();
	put(out, &format!("{ran} {} ", square.run()));

	// Held in a generic struct, whose type names the objects' arguments.
	let mut over = Over {
		scale: Box::new(tri),
		source: Box::new(a % 4),
	};
	let mut runs = Vec::new();
	while let Some(x) = over.run() {
		runs.push(x);
	}
	put(out, &format!("{runs:?} {}", over.source.left()));
}

/// Items that can already be `undefined`, whose `None` is `rs.None`: a countdown's
/// remainders, `None` where it skips one, and its ticks.
struct Skips(u8);

impl Source for Skips {
	type Item = Option<u8>;
	fn take(&mut self) -> Option<Option<u8>> {
		self.0 = self.0.checked_sub(1)?;
		Some((self.0 % 3 != 0).then_some(self.0))
	}
	fn left(&self) -> usize {
		usize::from(self.0)
	}
}

struct Beats(u8);

impl Source for Beats {
	type Item = ();
	fn take(&mut self) -> Option<()> {
		self.0 = self.0.checked_sub(1)?;
		Some(())
	}
	fn left(&self) -> usize {
		usize::from(self.0)
	}
}

/// A generic trait returning an `Option` of its parameter, which an object's type has as
/// an `rs.OptionOf`, at a parameter that can be `undefined`.
trait Peek<T> {
	fn peek(&self) -> Option<T>;
}

impl Peek<()> for u8 {
	fn peek(&self) -> Option<()> {
		(*self % 2 == 0).then_some(())
	}
}

impl Peek<Option<u8>> for u16 {
	fn peek(&self) -> Option<Option<u8>> {
		match *self % 3 {
			0 => None,
			1 => Some(None),
			_ => Some(Some(*self as u8)),
		}
	}
}

/// Objects whose methods return an `Option` of what can already be `undefined`: of an
/// associated type and of a generic trait's parameter.
pub fn options(data: &[u8], out: &mut Vec<u8>) {
	let [a, b, ..] = data else {
		return;
	};
	let mut skips: Box<dyn Source<Item = Option<u8>>> = Box::new(Skips(*a % 8));
	let first = skips.take();
	put(out, &format!("{first:?} {} {} ", skips.left(), taken(&mut *skips)));
	let mut beats = Beats(*b % 5);
	let ticked: &mut dyn Source<Item = ()> = &mut beats;
	put(out, &format!("{:?} {} ", ticked.take(), taken(ticked)));
	for x in data {
		let unit: &dyn Peek<()> = x;
		let wide = u16::from(*x);
		let nested: &dyn Peek<Option<u8>> = &wide;
		put(out, &format!("{:?} {:?} ", unit.peek(), nested.peek()));
	}
}
