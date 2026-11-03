//! Zero-sized values, which MIR never assigns and reads as constants: unit structs, empty
//! arrays and arrays of zero-sized elements, structs of zero-sized fields, enums of one
//! variant, `PhantomData`, closures that capture nothing, and std's iterators made of
//! them, as results, as values, in collections, and as they drop; and std's impls of its
//! own, as `#[derive]` calls them on a field, and of `()` as a key and a collected item;
//! parameters whose every read lowers to nothing; and writes through a `&mut` or a
//! pointer to one, which write nothing.

use std::cell::UnsafeCell;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::iter::Empty;
use std::marker::{PhantomData, PhantomPinned};
use std::mem::MaybeUninit;
use std::ops::RangeFull;
use std::sync::atomic::{AtomicU32, Ordering::Relaxed};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct Unit;

/// Of zero-sized fields, so zero-sized too.
#[derive(Clone, Copy)]
struct Pair(Unit, PhantomData<u8>);

/// Of one variant, so zero-sized.
#[derive(Clone, Copy, Debug, PartialEq)]
enum One {
	Only,
}

/// A closure it holds, zero-sized when the closure captures nothing.
struct Holds<F>(F);

/// What dropped, four bits each, the latest lowest.
static LOG: AtomicU32 = AtomicU32::new(0);

fn log(tag: u32) {
	LOG.store(LOG.load(Relaxed) << 4 | tag, Relaxed);
}

struct Loud;

impl Drop for Loud {
	fn drop(&mut self) {
		log(1);
	}
}

struct Quiet;

impl Drop for Quiet {
	fn drop(&mut self) {
		log(2);
	}
}

/// Drops itself, then its fields in order.
struct Both(Loud, Quiet);

impl Drop for Both {
	fn drop(&mut self) {
		log(3);
	}
}

fn unit() -> Unit {
	Unit
}

fn pair() -> Pair {
	Pair(Unit, PhantomData)
}

fn none<T>() -> [T; 0] {
	[]
}

fn units() -> [Unit; 3] {
	[Unit; 3]
}

fn one() -> One {
	One::Only
}

fn tuple() -> ((), Unit, [u8; 0]) {
	((), Unit, [])
}

fn phantom<T>() -> PhantomData<T> {
	PhantomData
}

fn adder() -> impl Fn(u8) -> u8 {
	|b| b.wrapping_add(1)
}

fn loud() -> Loud {
	Loud
}

fn made<T: Default>() -> T {
	T::default()
}

/// The case `kind` picks, on `data`; each puts what it read.
pub fn zeros(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	LOG.store(0, Relaxed);
	let n = data.len();
	match kind % 7 {
		// Derived and written impls whose results nothing assigns.
		0 => {
			let u = Unit;
			let c = u.clone();
			let p = pair().clone();
			let q: Unit = made();
			let _: PhantomData<u8> = phantom();
			out.push(u8::from(c == u) | u8::from(p.0 == q) << 1);
			out.extend(format!("{c:?} {:?}", One::Only.clone()).bytes());
		}
		// Closures that return them, read as items.
		1 => {
			out.push(data.iter().map(|_| [0u8; 0]).count() as u8);
			out.push(data.iter().map(|_| Unit).collect::<Vec<Unit>>().len() as u8);
			out.push(data.iter().map(|_| [Unit; 3]).flatten().count() as u8);
			out.push(
				data.iter()
					.map(|_| pair())
					.filter(|p| p.0 == Pair(Unit, PhantomData).0)
					.count() as u8,
			);
			let f = |_: u8| One::Only;
			out.push(u8::from(f(1) == one()));
		}
		2 => {
			let s = format!(
				"{:?} {:?} {:?} {:?} {:?}",
				unit(),
				none::<String>(),
				units(),
				one(),
				tuple()
			);
			out.extend(s.bytes());
		}
		// Closures that capture nothing as values: returned, moved, and held in a field.
		3 => {
			let f = adder();
			let g = adder();
			let h = g;
			let held = Holds(|b: u8| b ^ 0x5a);
			out.extend(data.iter().map(|&b| f(b)));
			out.extend(data.iter().copied().map(h));
			out.extend(data.iter().copied().map(adder()));
			out.extend(data.iter().map(|&b| (held.0)(b)));
		}
		// Drops, in order: of locals, a result, a `Vec`'s items, a struct's fields after it,
		// a moved value, and an array's items, but not of what's forgotten.
		4 => {
			{
				let _a = Loud;
				let _b = Quiet;
			}
			drop(loud());
			let v: Vec<Loud> = data.iter().map(|_| Loud).collect();
			out.push(v.len() as u8);
			drop(v);
			drop(Both(Loud, Quiet));
			std::mem::forget(Loud);
			let x = Quiet;
			let y = x;
			drop(y);
			drop([Quiet, Quiet]);
			out.extend(LOG.load(Relaxed).to_be_bytes());
		}
		5 => {
			let mut v: Vec<()> = Vec::with_capacity(n);
			v.extend(data.iter().map(|_| ()));
			out.push(vec![Unit; n].len() as u8);
			out.push(v.len() as u8);
		}
		// std's iterators of zero-sized fields, beside one that captures.
		_ => {
			out.push(std::iter::from_fn(|| None::<u8>).count() as u8);
			out.push(std::iter::from_fn(|| Some(Unit)).take(n).count() as u8);
			let mut left = n;
			out.extend(std::iter::from_fn(move || {
				left = left.checked_sub(1)?;
				Some(left as u8)
			}));
			let empty = std::iter::empty::<u8>().map(|b| b + 1);
			out.push(empty.size_hint().0 as u8);
			out.push(empty.count() as u8);
			out.push(std::iter::empty::<&u8>().copied().rev().filter(|_| true).count() as u8);
		}
	}
}

/// Each derivable impl, on fields of std's zero-sized types beside one that isn't.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Tagged<T> {
	n: u8,
	of: PhantomData<T>,
	pin: PhantomPinned,
	unit: (),
}

/// `PhantomData` of several types, whose `Debug` names each.
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Names(
	PhantomData<u8>,
	PhantomData<String>,
	PhantomData<(i64, bool)>,
	PhantomData<&'static str>,
	PhantomData<[u16; 3]>,
	PhantomData<Vec<Option<char>>>,
	PhantomData<fn(u8) -> u8>,
	PhantomData<Unit>,
	PhantomData<Tagged<()>>,
	PhantomData<()>,
);

/// `..` and `Empty`, whose impls are std's own, of what each has.
#[derive(Clone, Debug, Default)]
struct Rest(RangeFull, Empty<u8>);

/// `DefaultHasher::new`'s hash of `value`.
fn hash<T: Hash + ?Sized>(value: &T) -> u64 {
	let mut h = DefaultHasher::new();
	value.hash(&mut h);
	h.finish()
}

/// [`hash`] of `value`, little-endian.
fn digest<T: Hash + ?Sized>(value: &T, out: &mut Vec<u8>) {
	out.extend_from_slice(&hash(value).to_le_bytes());
}

/// The impls `#[derive]` calls on `PhantomData`, `PhantomPinned`, and `()` fields.
pub fn impls(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let tags: Vec<Tagged<String>> = data
		.iter()
		.map(|&n| Tagged {
			n,
			..Default::default()
		})
		.collect();
	match kind % 4 {
		0 => {
			let t = tags.first().cloned().unwrap_or_default();
			let copied = Tagged::<u8> {
				n: t.n,
				..Default::default()
			};
			let again = copied;
			out.extend(format!("{t:?} {copied:?} {again:#?} {:#?}", Names::default()).bytes());
			let rest = Rest::default().clone();
			out.extend(format!("{rest:?} {rest:#?} {:>9?}|{:<9?}|", PhantomPinned, PhantomData::<u8>).bytes());
			out.push(rest.1.count() as u8);
		}
		// Order, beside the hashes it must agree with.
		1 => {
			let mut sorted = tags.clone();
			sorted.sort();
			out.extend(sorted.iter().map(|t| t.n));
			// Equal by `Ord` exactly when equal by `PartialEq` and by hash.
			for pair in tags.windows(2) {
				let (a, b) = (&pair[0], &pair[1]);
				out.push(a.cmp(b) as u8 | u8::from(a == b) << 2 | u8::from(a.partial_cmp(b) == Some(a.cmp(b))) << 3);
				out.push(u8::from(a.cmp(b).is_eq() == (hash(a) == hash(b))));
			}
			out.push(u8::from(PhantomData::<u8> == PhantomData) | u8::from(PhantomPinned == PhantomPinned) << 1);
			out.push(PhantomData::<String>.cmp(&PhantomData) as u8);
			out.push(().cmp(&()) as u8);
		}
		2 => {
			for t in &tags {
				digest(t, out);
				digest(&t.n, out);
			}
			digest(&(), out);
			digest(&PhantomData::<u64>, out);
			digest(&PhantomPinned, out);
			digest(&.., out);
			digest(&Names::default(), out);
		}
		_ => {
			let names = Names::default().clone();
			out.push(u8::from(names == Names::default()));
			out.push(u8::from(names.cmp(&Names::default()).is_eq()));
			out.push(u8::from(tags.iter().all(|t| t.clone() == *t)));
			out.push(u8::from(Rest::default().0 == ..));
		}
	}
}

/// `Some(())` if every byte of `data` is below 0x80.
fn small(data: &[u8]) -> Option<()> {
	data.iter().map(|&b| (b < 0x80).then_some(())).collect()
}

/// `Ok(())`, or the first byte of `data` that is 0xff.
fn checked(data: &[u8]) -> Result<(), u8> {
	data.iter().map(|&b| if b == 0xff { Err(b) } else { Ok(()) }).collect()
}

/// `()` as a hash and B-tree key and as `collect::<Option<_>>`'s item.
pub fn keys(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	match kind % 3 {
		0 => {
			let mut hash = HashSet::new();
			let mut tree = BTreeSet::new();
			for &b in data {
				if b & 1 == 0 {
					out.push(u8::from(hash.insert(())) | u8::from(tree.insert(())) << 1);
				} else {
					out.push(u8::from(hash.remove(&())) | u8::from(tree.remove(&())) << 1);
				}
				out.push(hash.len() as u8 | (tree.len() as u8) << 4);
			}
			out.push(u8::from(hash.contains(&())) | u8::from(tree.contains(&())) << 1);
			out.push(u8::from(tree.first() == tree.last()));
		}
		1 => {
			let mut map = HashMap::new();
			let mut tree = BTreeMap::new();
			for &b in data {
				if b & 3 == 3 {
					out.extend(map.remove(&()).map(|v: u8| [1, v]).unwrap_or([0, 0]));
					out.extend(tree.remove(&()).map(|v: u8| [1, v]).unwrap_or([0, 0]));
				} else {
					out.extend(map.insert((), b).map(|v| [1, v]).unwrap_or([0, 0]));
					out.extend(tree.insert((), b).map(|v| [1, v]).unwrap_or([0, 0]));
				}
				out.push(map.len() as u8 | (tree.len() as u8) << 4);
				out.push(map.get(&()).copied().unwrap_or(0));
			}
		}
		// `None` midway stops the collection there.
		_ => {
			let units: Option<Vec<()>> = data.iter().map(|&b| (b != 0).then_some(())).collect();
			out.extend(units.map(|v| [1, v.len() as u8]).unwrap_or([0, 0]));
			let count = data
				.iter()
				.map(|&b| (b != 0).then_some(()))
				.inspect(|_| out.push(9))
				.collect::<Option<Vec<()>>>();
			out.push(u8::from(count.is_some()));
			out.push(u8::from(small(data).is_some()));
			out.push(match checked(data) {
				Ok(()) => 0,
				Err(b) => b,
			});
		}
	}
}

/// A one-time cell as crossbeam-epoch's is, whose drop drops its value only once set:
/// nothing, of a value that drops nothing.
struct Lock<T> {
	set: bool,
	value: UnsafeCell<MaybeUninit<T>>,
}

impl<T> Lock<T> {
	fn new(value: Option<T>) -> Self {
		let set = value.is_some();
		let value = match value {
			Some(v) => MaybeUninit::new(v),
			None => MaybeUninit::uninit(),
		};
		Lock {
			set,
			value: UnsafeCell::new(value),
		}
	}
}

impl<T> Drop for Lock<T> {
	fn drop(&mut self) {
		if self.set {
			unsafe { (*self.value.get()).assume_init_drop() }
		}
	}
}

/// Whether `p` equals another `PhantomData`, which it always does.
fn phantom_eq<T>(p: PhantomData<T>, n: u8) -> u8 {
	if p == PhantomData { n } else { 0 }
}

/// Whether `u` is `()`, which it always is.
fn unit_eq(u: &(), n: u8) -> u8 {
	if *u == () { n } else { 0 }
}

/// `n` if `p`, reassigned, equals another `PhantomData`, which it always does.
#[allow(unused_assignments)]
fn rewritten(mut p: PhantomData<u8>, n: u8) -> u8 {
	p = PhantomData;
	if p == PhantomData { n } else { 0 }
}

/// How many items `xs` hold if `value` is `()`, which it always is, beside a function
/// std's `flatten` makes, whose own parameter is `value` too.
fn flattened(value: &(), xs: &[Vec<u8>]) -> u8 {
	if *value == () {
		xs.iter().flatten().count() as u8
	} else {
		0
	}
}

/// Functions, closures, and locals whose every read lowers to nothing: a drop of what
/// drops nothing, and a comparison of what holds nothing.
pub fn unread(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	LOG.store(0, Relaxed);
	let n = data.len() as u8;
	match kind % 3 {
		0 => {
			drop(Lock::new(data.first().map(|&b| u32::from(b))));
			drop(Lock::<u32>::new(None));
			drop(Lock::new(data.first().map(|_| Loud)));
			drop(Lock::<Loud>::new(None));
			out.extend(LOG.load(Relaxed).to_be_bytes());
		}
		1 => {
			let units: Vec<()> = data.iter().map(|_| ()).collect();
			out.push(units.iter().filter(|u| **u == ()).count() as u8);
			out.push(phantom_eq(PhantomData::<String>, n));
			out.push(unit_eq(&(), n));
			out.push(rewritten(PhantomData, n));
			out.push(flattened(&(), &[data.to_vec(), data.to_vec()]));
		}
		_ => {
			let lock = Lock::new(data.first().map(|&b| u32::from(b)));
			let p = PhantomData::<u8>;
			let mut q = PhantomData::<u8>;
			q = if n > 1 { q } else { PhantomData };
			let same = |r: PhantomData<u8>| r == p;
			out.push(u8::from(same(q)) | u8::from(p == q) << 1);
			drop(lock);
			out.extend(LOG.load(Relaxed).to_be_bytes());
		}
	}
}

/// A fresh `&mut ()`, after logging `tag`.
fn leaked(tag: u32) -> &'static mut () {
	log(tag);
	Box::leak(Box::new(()))
}

/// A fresh `*mut ()`, after logging `tag`.
fn raw(tag: u32) -> *mut () {
	log(tag);
	Box::into_raw(Box::new(()))
}

/// What `T::default` makes, after logging 5.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Defaulted;

impl Default for Defaulted {
	fn default() -> Self {
		log(5);
		Defaulted
	}
}

/// Writes through a `&mut` or a pointer to a value of no bytes, which have nothing to
/// read or write but still run their operands.
pub fn writes(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	LOG.store(0, Relaxed);
	let n = data.len() as u8;
	match kind % 5 {
		// `mem`'s, of `()`, through what a call makes and through locals.
		0 => {
			let _ = std::mem::replace(leaked(1), ());
			let mut u = ();
			let old = std::mem::replace(&mut u, ());
			out.push(u8::from(old == u));
			std::mem::swap(leaked(2), &mut ());
			std::mem::swap(leaked(3), leaked(4));
			std::mem::take(leaked(6));
			let t: () = std::mem::take(&mut u);
			out.push(u8::from(t == u) | n << 1);
		}
		// Of `PhantomData` and unit structs, one whose `Default` has an effect.
		1 => {
			let mut p = PhantomData::<u8>;
			let q = std::mem::replace(&mut p, PhantomData);
			std::mem::swap(&mut p, &mut PhantomData);
			out.push(u8::from(p == q));
			let mut a = Unit;
			let mut b = unit();
			std::mem::swap(&mut a, &mut b);
			std::mem::take(&mut a);
			let t = std::mem::take(&mut b);
			out.push(u8::from(a == t) | n << 1);
			let mut c = Defaulted;
			let d = std::mem::take(&mut c);
			let _ = std::mem::replace(&mut c, Defaulted);
			out.push(u8::from(c == d));
		}
		// Through raw pointers.
		2 => unsafe {
			let p = raw(1);
			p.write(());
			let r: () = p.read();
			let _ = p.replace(r);
			std::ptr::swap(p, raw(2));
			std::ptr::swap(raw(3), std::ptr::dangling_mut());
			raw(4).write(());
			let mut s = Unit;
			let q: *mut Unit = &mut s;
			std::ptr::write(q, Unit);
			let old = std::ptr::replace(q, unit());
			std::ptr::swap(q, &mut Unit);
			out.push(u8::from(std::ptr::read(q) == old) | n << 1);
		},
		// Through `Cell`s and slices, whose bounds still panic.
		3 => {
			let c = std::cell::Cell::new(());
			let _ = c.replace(());
			let t: () = c.take();
			c.set(t);
			let units = &mut [(); 3];
			units.swap(0, usize::from(n));
			let mut v = vec![Unit; 2];
			v.swap(0, 1);
			out.push(units.len() as u8 | (v.len() as u8) << 4);
			let p = std::cell::Cell::new(PhantomData::<u8>);
			let _ = p.replace(PhantomData);
			out.push(u8::from(p.take() == PhantomData));
		}
		// `Option<()>`, which has a byte, and replaces of others whose result nothing reads.
		_ => {
			let mut o = (n > 0).then_some(());
			let old = o.replace(());
			out.push(u8::from(old.is_some()) | u8::from(o.is_some()) << 1);
			let taken = o.take();
			let _ = o.take();
			out.push(u8::from(taken.is_some()) | u8::from(o.is_some()) << 1);
			let mut x = n;
			let _ = std::mem::replace(&mut x, 1);
			let _ = std::mem::take(&mut x);
			let mut y = Some(n);
			let _ = y.take();
			let _ = y.replace(n);
			out.push(x | y.unwrap_or(0) << 4);
		}
	}
	out.extend(LOG.load(Relaxed).to_be_bytes());
}
