//! Trait objects of the crate's own traits, behind `&`, `&mut`, and `Box`: each call runs
//! the impl of the value's own type, a struct, an enum, or a number, changing it in place
//! through `&mut self` wherever it is, a provided method, a supertrait's, `Debug` among
//! them, `self: Box<Self>`, and an upcast to the supertrait. Behind an `Arc`, the object
//! shares the count of the `Arc` it was made from. A `dyn Any` downcasts to the type it
//! was made of, borrowed, boxed, or shared. A `dyn Fn*` of a closure, a function, or a
//! function pointer calls it, changing what it captured, borrowed, boxed, or shared. An
//! `Rc`, a `Weak`, a `Pin`, and an `Rc` or an `Arc` made from a box hold one too.

use std::any::Any;
use std::cell::RefCell;
use std::fmt::{self, Debug};
use std::pin::Pin;
use std::rc::{self, Rc};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Weak};

use crate::fmt::put;

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
	/// Writes through a `&mut` to a number, which is an `rt.Place`.
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

	// Of types that can be `undefined`, whose `None` is `rt.None`.
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
	put(out, &format!("{typed:?}"));
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
