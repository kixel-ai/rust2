//! Whole writes through a `&mut` to an object: an `Arc`, whose clones share one, a
//! `Vec`, a map, a set, and the crate's own struct, by `*r = v` and by std's
//! replacements, wherever the reference came from: a `get_mut`, a map lookup, a guard,
//! a parameter, a capture, or an iterator. Each must land in the slot it points at, and
//! drop what it replaced.

use std::collections::{BTreeSet, HashMap, VecDeque};
use std::pin::Pin;
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::{Arc, Mutex, Weak};
use std::task::{Wake, Waker};

/// moq-net's growing buffer: the `Arc` behind a parameter, replaced by a bigger one.
fn reserve(current: &mut Arc<Vec<u8>>, needed: usize) {
	if current.len() >= needed {
		return;
	}
	let mut next = vec![0u8; needed.max(current.len() * 2)];
	next[..current.len()].copy_from_slice(current);
	*current = Arc::new(next);
}

fn repin(p: &mut Pin<Arc<u8>>, to: u8) {
	*p = Pin::new(Arc::new(to));
}

/// What a pointer to a pinned `Arc` reads, through its own `Deref` and then the pin's.
fn through<P: std::ops::Deref<Target = Pin<Arc<u8>>>>(p: P) -> u8 {
	**p
}

/// What a pointer to an `Arc` reads, through its own `Deref` and then the `Arc`'s.
fn through_arc<P: std::ops::Deref<Target = Arc<u8>>>(p: P) -> u8 {
	**p
}

/// What a pointer to a guard reads, through its own `Deref` and then the guard's.
fn through_mutex<P: std::ops::Deref<Target = Mutex<u8>>>(p: P) -> u8 {
	*p.lock().unwrap()
}

/// Writes a byte through a pointer's `DerefMut`.
fn write_through<P: std::ops::DerefMut<Target = u8>>(mut p: P, by: u8) {
	*p ^= by;
}

/// Replaces an `Arc` through a pointer's `DerefMut`.
fn replace_through<P: std::ops::DerefMut<Target = Arc<u8>>>(mut p: P, by: u8) {
	*p = Arc::new(by);
}

/// A pin of a byte, made through a function pointer, that writes the byte.
fn pin_via<'a>(p: &'a mut u8) -> Pin<&'a mut u8> {
	let f: fn(&'a mut u8) -> Pin<&'a mut u8> = Pin::new;
	f(p)
}

fn locked_by(n: u8) -> Mutex<u8> {
	Mutex::new(n.wrapping_add(3))
}

/// The `Arc` a pinned box holds, replaced through `Pin`'s `DerefMut`.
fn rebox(p: &mut Pin<Box<Arc<u8>>>, to: u8) {
	**p = Arc::new(to);
}

/// The crate's own struct, taken apart and put back, as moq-net's `apply` does.
#[derive(Default)]
struct Window {
	start: u8,
	held: Vec<Arc<u8>>,
}

impl Window {
	fn with_start(mut self, start: u8) -> Self {
		self.start = start;
		self
	}
}

fn apply(w: &mut Window, start: u8) {
	*w = std::mem::take(w).with_start(start);
}

/// What moq-net's tracks do: a new value made of the old one's parts, swapped in.
fn park(w: &mut Window, start: u8) -> u8 {
	let parked = Window {
		start,
		held: std::mem::take(&mut w.held),
	};
	std::mem::replace(w, parked).start
}

fn swap_windows(a: &mut Window, b: &mut Window) {
	std::mem::swap(a, b);
}

/// A local's own `Vec` swapped with the one a parameter points at, each kept apart.
fn swap_in(v: &mut Vec<Arc<u8>>, with: &Arc<u8>) -> Vec<Arc<u8>> {
	let mut mine = vec![Arc::clone(with)];
	std::mem::swap(&mut mine, v);
	v.push(Arc::clone(with));
	mine
}

/// A reference a closure copied, which holds the array it points at.
fn replace_captured(a: &mut [u16; 2], to: u16) -> [u16; 2] {
	let mut g = move || std::mem::replace(&mut *a, [to, to.wrapping_add(1)]);
	g()
}

/// A pinned box is what it holds in JS: here a `Vec`, whose length changes.
fn repin_all(v: &mut Pin<Box<Vec<Arc<u8>>>>, with: &Arc<u8>, n: u8) -> usize {
	let old = std::mem::replace(v, Box::pin((0..n % 3).map(|_| Arc::clone(with)).collect()));
	*v = Box::pin((0..n % 4).map(|_| Arc::clone(with)).collect());
	old.len()
}

/// An array of objects, assigned whole through a parameter.
fn reset(a: &mut [Arc<u8>; 2], with: &Arc<u8>) {
	*a = [Arc::clone(with), Arc::new(**with)];
}

fn take_all(v: &mut Vec<Arc<u8>>) -> Vec<Arc<u8>> {
	std::mem::take(v)
}

fn swap_all(a: &mut Vec<Arc<u8>>, b: &mut Vec<Arc<u8>>) {
	std::mem::swap(a, b);
}

fn refill(v: &mut Vec<Arc<u8>>, bytes: &mut Vec<u8>, with: &Arc<u8>, n: u8) {
	*v = (0..n % 4).map(|_| Arc::clone(with)).collect();
	*bytes = vec![n; usize::from(n % 3)];
}

fn requeue(q: &mut VecDeque<u8>, map: &mut HashMap<u8, u8>, n: u8) -> usize {
	let old = std::mem::replace(q, (0..n % 5).collect());
	*map = HashMap::from([(n, n)]);
	old.len()
}

fn relink(w: &mut Weak<u8>, to: &Arc<u8>) {
	*w = Arc::downgrade(to);
}

struct Wakes(AtomicU8);

impl Wake for Wakes {
	fn wake(self: Arc<Self>) {
		self.0.fetch_add(1, Ordering::Relaxed);
	}
}

/// A `Waker`, whose clones may be one object, as `noop`'s are.
fn rewake(w: &mut Waker, to: &Waker) -> Waker {
	std::mem::replace(w, to.clone())
}

/// What `count` says of each `Arc`, and the values `items` hold.
fn counts(out: &mut Vec<u8>, arcs: &[&Arc<u8>], items: &[Arc<u8>]) {
	for a in arcs {
		out.push(Arc::strong_count(a) as u8);
	}
	out.push(items.len() as u8);
	out.extend(items.iter().map(|a| **a));
}

pub fn objects(data: &[u8], out: &mut Vec<u8>) {
	let by = data.first().copied().unwrap_or(5);
	let one = Arc::new(by);
	let two = Arc::new(by.wrapping_add(1));
	// An `Arc` an `Option` lends, from a `get_mut` and an `as_mut`.
	let mut v: Vec<Arc<u8>> = data.iter().map(|_| Arc::clone(&one)).collect();
	for (i, &b) in data.iter().enumerate() {
		if let Some(r) = v.get_mut(usize::from(b) % (data.len() + 1)) {
			*r = Arc::clone(&two);
		}
		if b % 3 == 0
			&& let Some(r) = v.get_mut(i)
		{
			*r = Arc::new(b);
		}
	}
	counts(out, &[&one, &two], &v);
	let mut maybe = Some(Arc::clone(&one));
	if let Some(r) = maybe.as_mut() {
		*r = Arc::clone(&two);
	}
	out.push(maybe.map_or(0xee, |a| *a));
	// Each one `iter_mut` and `last_mut` yield, and one a closure captured.
	for (r, &b) in v.iter_mut().zip(data) {
		if b % 2 == 0 {
			*r = Arc::clone(&one);
		}
	}
	if let Some(last) = v.last_mut() {
		*last = Arc::new(by.wrapping_mul(3));
	}
	let mut kept = Arc::clone(&one);
	let mut replace = |to: &Arc<u8>| kept = Arc::clone(to);
	replace(&two);
	out.push(*kept);
	drop(kept);
	counts(out, &[&one, &two], &v);
	// Behind a parameter, as moq-net's buffer grows, and pinned.
	let mut buffer = Arc::new(data.to_vec());
	let reader = Arc::clone(&buffer);
	for &b in data {
		reserve(&mut buffer, usize::from(b % 16));
	}
	out.extend_from_slice(&[buffer.len() as u8, reader.len() as u8]);
	out.push(Arc::strong_count(&reader) as u8);
	let mut pinned = Pin::new(Arc::new(by));
	repin(&mut pinned, by.wrapping_add(7));
	out.push(*pinned);
	let held = Arc::new(by);
	let mut boxed = Box::pin(Arc::clone(&held));
	rebox(&mut boxed, by.wrapping_add(9));
	out.extend_from_slice(&[**boxed, Arc::strong_count(&held) as u8]);
	// A pinned reference to an `Arc` lends the `Arc`, and a reference to a pinned `Arc`
	// the pin, each through `Deref`.
	let lent = Pin::new(&held);
	out.extend_from_slice(&[Arc::strong_count(&*lent) as u8, **lent, through(&pinned)]);
	out.extend_from_slice(&[through_arc(&held), through_mutex(&locked_by(by))]);
	// A mutable reference to each, read through the shared `Deref`.
	out.extend_from_slice(&[through_arc(&mut Arc::clone(&held)), through(&mut pinned)]);
	out.push(through_mutex(&mut locked_by(by)));
	// Written through `DerefMut` of a pinned or plain mutable reference, and of a guard.
	let mut byte = by;
	*Pin::new(&mut byte) ^= 0x5a;
	write_through(Pin::new(&mut byte), 0x21);
	write_through(&mut byte, 0x0c);
	out.push(byte);
	let mut replaced = Arc::new(by);
	*Pin::new(&mut replaced) = Arc::new(by.wrapping_add(1));
	out.push(*replaced);
	replace_through(Pin::new(&mut replaced), by.wrapping_add(2));
	out.push(*replaced);
	replace_through(&mut replaced, by.wrapping_add(3));
	out.push(*replaced);
	let guarded = Mutex::new(Arc::new(by));
	replace_through(guarded.lock().unwrap(), by.wrapping_add(4));
	out.push(**guarded.lock().unwrap());
	let mut pinned_byte = by;
	*Pin::get_mut(pin_via(&mut pinned_byte)) ^= 0x5a;
	out.push(pinned_byte);
	let mut owned = Arc::clone(&held);
	let lent = Pin::new(&mut owned);
	out.extend_from_slice(&[Arc::strong_count(&*lent) as u8, **lent]);
	// Through a guard, taken and assigned.
	let locked = Mutex::new(v);
	let taken = std::mem::take(&mut *locked.lock().unwrap());
	counts(out, &[&one, &two], &taken);
	*locked.lock().unwrap() = vec![Arc::clone(&two)];
	drop(taken);
	counts(out, &[&one, &two], &locked.lock().unwrap());
	let cell = std::cell::RefCell::new(vec![Arc::clone(&one)]);
	let taken = std::mem::take(&mut *cell.borrow_mut());
	*cell.borrow_mut() = vec![Arc::clone(&two), Arc::clone(&two)];
	counts(out, &[&one, &two], &taken);
	counts(out, &[&one, &two], &cell.borrow());
	// Through a map lookup.
	let mut sets: HashMap<u8, BTreeSet<u8>> = HashMap::new();
	sets.insert(by, data.iter().copied().collect());
	if let Some(set) = sets.get_mut(&by) {
		let old = std::mem::replace(set, BTreeSet::from([by.wrapping_add(1)]));
		out.push(old.len() as u8);
	}
	*sets.entry(by.wrapping_add(1)).or_default() = BTreeSet::from([by]);
	let mut keys: Vec<_> = sets.iter().map(|(&k, s)| (k, s.len() as u8)).collect();
	keys.sort();
	for (k, n) in keys {
		out.extend_from_slice(&[k, n]);
	}
	// The crate's own struct, and `Vec`s behind parameters.
	let mut w = Window {
		start: by,
		held: vec![Arc::clone(&one)],
	};
	let mut other = vec![Arc::clone(&two), Arc::clone(&two)];
	let mut bytes = Vec::new();
	for &b in data {
		match b % 5 {
			0 => apply(&mut w, b),
			1 => out.push(park(&mut w, b)),
			2 => swap_all(&mut w.held, &mut other),
			3 => refill(&mut other, &mut bytes, &one, b),
			4 if b % 2 == 0 => {
				let mine = swap_in(&mut other, &two);
				counts(out, &[&one, &two], &mine);
			}
			_ => drop(take_all(&mut w.held)),
		}
	}
	let mut pinned_all = Box::pin(vec![Arc::clone(&two)]);
	for &b in data {
		out.push(repin_all(&mut pinned_all, &one, b) as u8);
	}
	counts(out, &[&one, &two], &pinned_all);
	drop(pinned_all);
	let mut pair = [Arc::clone(&one), Arc::clone(&two)];
	reset(&mut pair, &two);
	counts(out, &[&one, &two], &pair);
	drop(pair);
	out.push(w.start);
	out.extend_from_slice(&bytes);
	counts(out, &[&one, &two], &w.held);
	counts(out, &[&one, &two], &other);
	// Collections and a `Weak` behind parameters.
	let mut q = VecDeque::from([by]);
	let mut map = HashMap::new();
	for &b in data {
		out.push(requeue(&mut q, &mut map, b) as u8);
	}
	out.extend(q.iter());
	out.extend(map.values());
	let mut weak = Weak::new();
	relink(&mut weak, &two);
	out.push(weak.upgrade().map_or(0xee, |a| *a));
	out.push(Arc::weak_count(&two) as u8);
	// Swapped and replaced through copies of references.
	let mut a = Window::default().with_start(by);
	let mut b = Window {
		start: by.wrapping_add(2),
		held: vec![Arc::clone(&one)],
	};
	let mut array = [u16::from(by); 2];
	for &x in data {
		if x % 2 == 0 {
			swap_windows(&mut a, &mut b);
		}
		let old = replace_captured(&mut array, u16::from(x));
		out.extend_from_slice(&[old[0] as u8, old[1] as u8]);
	}
	out.extend_from_slice(&[a.start, a.held.len() as u8, b.start, b.held.len() as u8]);
	out.extend_from_slice(&[array[0] as u8, array[1] as u8]);
	// A `noop` waker replaced by a counting one, and back.
	let wakes = Arc::new(Wakes(AtomicU8::new(0)));
	let counting = Waker::from(Arc::clone(&wakes));
	let noop = Waker::noop().clone();
	let mut w = noop.clone();
	for &x in data {
		let old = rewake(&mut w, if x % 2 == 0 { &counting } else { &noop });
		old.wake_by_ref();
		w.wake_by_ref();
	}
	drop(w);
	out.push(wakes.0.load(Ordering::Relaxed));
	out.push(Arc::strong_count(&wakes) as u8);
}
