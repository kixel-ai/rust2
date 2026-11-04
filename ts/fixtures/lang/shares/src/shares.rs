//! `Rc<[T]>`, `Arc<[T]>`, `Rc<str>`, and `Arc<str>`: made by `From` of a `Vec`, a slice,
//! a `&mut` slice, a `Box`, an array, and a `Cow`, by `collect` and `Default`, read
//! through `Deref`, compared, hashed, and printed as their slice is, cloned and counted,
//! written through `get_mut` and `make_mut`, and downgraded, of bytes, of `u16`s, of
//! values JS copies, and of values that log their clones and drops, empty or not, with
//! what drops dropped when its last owner does; held by a generic type at each of those,
//! looked up as map keys by a borrow of their slice or text, pointed at by `as_ptr`, and
//! unsized from an `Rc` or `Arc` of an array, sharing its count.

use std::borrow::{Borrow, Cow};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::rc::{Rc, Weak};
use std::sync::Arc;

use crate::harness::{put, text};

/// Logs its id as it drops, and as it is cloned, the id with the high bit set: its clone's
/// id is one more.
struct Logged {
	id: u8,
	log: Rc<RefCell<Vec<u8>>>,
}

impl Clone for Logged {
	fn clone(&self) -> Self {
		self.log.borrow_mut().push(0x80 | self.id);
		Self {
			id: self.id.wrapping_add(1),
			log: self.log.clone(),
		}
	}
}

impl Drop for Logged {
	fn drop(&mut self) {
		self.log.borrow_mut().push(self.id);
	}
}

/// A shared slice in a field, beside one that drops after it.
struct Held {
	items: Rc<[Logged]>,
	last: Logged,
}

/// A shared slice of its parameter, made from a `Vec` and an iterator, cloned, and written,
/// as each instance holds it.
struct Shared<T> {
	items: Rc<[T]>,
}

impl<T: Clone> Shared<T> {
	fn new(items: Vec<T>) -> Self {
		Self { items: items.into() }
	}

	fn collected(items: impl Iterator<Item = T>) -> Self {
		Self { items: items.collect() }
	}

	fn first(&self) -> Option<&T> {
		self.items.first()
	}

	/// Its items, made its own, with `item` in place of its last.
	fn set_last(&mut self, item: T) {
		if let Some(last) = Rc::make_mut(&mut self.items).last_mut() {
			*last = item;
		}
	}
}

impl<T> Clone for Shared<T> {
	fn clone(&self) -> Self {
		Self {
			items: Rc::clone(&self.items),
		}
	}
}

/// Puts what was logged since the last flush, after a marker.
fn flush(log: &RefCell<Vec<u8>>, out: &mut Vec<u8>) {
	out.push(0xfd);
	out.extend(log.borrow_mut().drain(..));
}

fn hash<T: Hash + ?Sized>(value: &T) -> u64 {
	let mut h = DefaultHasher::new();
	value.hash(&mut h);
	h.finish()
}

/// Puts `b`'s length and bytes.
fn put_bytes(b: &[u8], out: &mut Vec<u8>) {
	out.push(b.len() as u8);
	out.extend_from_slice(b);
}

/// Puts `b`'s length and each `u16`'s bytes.
fn put_wides(b: &[u16], out: &mut Vec<u8>) {
	out.push(b.len() as u8);
	for x in b {
		out.extend(x.to_le_bytes());
	}
}

/// Puts the ids of `b`.
fn ids(b: &[Logged], out: &mut Vec<u8>) {
	out.push(b.len() as u8);
	out.extend(b.iter().map(|l| l.id));
}

/// Puts each pair's bytes.
fn pairs(b: &[(u8, u16)], out: &mut Vec<u8>) {
	out.push(b.len() as u8);
	for (a, w) in b {
		out.push(*a);
		out.extend(w.to_le_bytes());
	}
}

/// Puts an `Rc`'s strong and weak counts.
fn counts<T: ?Sized>(rc: &Rc<T>, out: &mut Vec<u8>) {
	out.push(Rc::strong_count(rc) as u8);
	out.push(Rc::weak_count(rc) as u8);
}

/// Puts an `Arc`'s strong and weak counts.
fn arc_counts<T: ?Sized>(arc: &Arc<T>, out: &mut Vec<u8>) {
	out.push(Arc::strong_count(arc) as u8);
	out.push(Arc::weak_count(arc) as u8);
}

/// Writes through a `&mut` to bytes another function was handed.
fn scramble(b: &mut [u8]) {
	b.reverse();
	for x in b.iter_mut() {
		*x = x.wrapping_add(1);
	}
	if let Some(first) = b.first_mut() {
		*first ^= 0xff;
	}
}

/// Bytes, each way, read, counted, and written apart from what they came from.
pub fn bytes(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	// As many bytes as `kind`'s high bits pick, none included.
	let data = &data[..usize::from(kind >> 3) % (data.len() + 1)];
	match kind % 4 {
		// Made each way, apart from what each came from.
		0 => {
			let mut source = data.to_vec();
			let from_ref: Rc<[u8]> = Rc::from(&source[..]);
			let from_mut: Arc<[u8]> = Arc::from(&mut source[..]);
			scramble(&mut source);
			put_bytes(&from_ref, out);
			put_bytes(&from_mut, out);
			put_bytes(&source, out);
			let mut spare = Vec::with_capacity(64);
			spare.extend_from_slice(data);
			let from_vec = Rc::<[u8]>::from(spare);
			put_bytes(&from_vec, out);
			let into: Arc<[u8]> = data.to_vec().into();
			put_bytes(&into, out);
			let from_box: Rc<[u8]> = Rc::from(Box::<[u8]>::from(data));
			put_bytes(&from_box, out);
			let array = [data.first().copied().unwrap_or(1), 2, 3];
			let from_array: Arc<[u8]> = Arc::from(array);
			put_bytes(&from_array, out);
			let collected: Rc<[u8]> = data.iter().map(|&x| x ^ 0x55).collect();
			put_bytes(&collected, out);
			let arc_collected: Arc<[u8]> = data.iter().rev().copied().collect();
			put_bytes(&arc_collected, out);
			let empty: Rc<[u8]> = Default::default();
			put_bytes(&empty, out);
			let arc_empty = Arc::<[u8]>::default();
			put_bytes(&arc_empty, out);
			let borrowed: Rc<[u8]> = Rc::from(Cow::Borrowed(data));
			put_bytes(&borrowed, out);
			let owned: Arc<[u8]> = Arc::from(Cow::<[u8]>::Owned(data.to_vec()));
			put_bytes(&owned, out);
			// Of text's bytes, which share its count, as std shares its allocation.
			let s = Rc::<str>::from(text(data));
			let twin = s.clone();
			let of_str: Rc<[u8]> = Rc::from(s);
			counts(&of_str, out);
			let of_twin: Rc<[u8]> = Rc::from(twin);
			out.push(u8::from(Rc::ptr_eq(&of_str, &of_twin)));
			counts(&of_twin, out);
			drop(of_twin);
			counts(&of_str, out);
			put_bytes(&of_str, out);
			// Text written once no bytes share it, and pointers converted to themselves.
			let mut s = Rc::<str>::from(text(data));
			drop(Rc::<[u8]>::from(s.clone()));
			if let Some(m) = Rc::get_mut(&mut s) {
				m.make_ascii_uppercase();
			}
			let same = Rc::<str>::from(s.clone());
			out.push(u8::from(Rc::ptr_eq(&same, &s)));
			put_bytes(&Rc::<[u8]>::from(s), out);
			let n = Arc::new(u32::from(data.len() as u8));
			let same = Arc::<u32>::from(n.clone());
			out.push(u8::from(Arc::ptr_eq(&same, &n)));
			out.extend(same.to_le_bytes());
			let arc_of_str: Arc<[u8]> = Arc::from(Arc::<str>::from(text(data)));
			put_bytes(&arc_of_str, out);
			// Back to what owns its bytes, which grows as any does.
			let mut v = from_vec.to_vec();
			v.push(0xee);
			v.extend_from_slice(&collected);
			put_bytes(&v, out);
			let v = into.to_vec();
			put_bytes(&v, out);
			let b: Box<[u8]> = Box::from(&*from_box);
			put_bytes(&b, out);
		}
		// Compared, ordered, hashed, and printed as their slice is.
		1 => {
			let a: Rc<[u8]> = data.into();
			let half = &data[..data.len() / 2];
			let b: Rc<[u8]> = half.into();
			out.push(u8::from(a == b));
			out.push(u8::from(a != b));
			out.push(u8::from(a < b));
			out.push(a.cmp(&b) as u8);
			out.push(b.partial_cmp(&a).map_or(9, |o| o as u8));
			out.push(u8::from(*a == *data));
			out.push(u8::from(hash(&a) == hash(data)));
			out.extend(hash(&b).to_le_bytes());
			out.push(a.len() as u8);
			out.push(u8::from(a.is_empty()));
			out.push(a.iter().copied().fold(0, u8::wrapping_add));
			out.push(a.first().copied().unwrap_or(0xaa));
			out.push(a.get(1).copied().unwrap_or(0xbb));
			let arc: Arc<[u8]> = Arc::from(data);
			out.push(u8::from(*arc == *a));
			out.push(u8::from(hash(&arc) == hash(&a)));
			put(out, &format!("{a:?}|{b:?}|{arc:?}|{:?}", Rc::<[u8]>::default()));
		}
		// Cloned and counted: one allocation every clone shares, and every `Weak`.
		2 => {
			let a: Rc<[u8]> = Rc::from(data);
			counts(&a, out);
			let b = a.clone();
			counts(&a, out);
			out.push(u8::from(Rc::ptr_eq(&a, &b)));
			let c: Rc<[u8]> = Rc::from(data);
			out.push(u8::from(Rc::ptr_eq(&a, &c)));
			out.push(u8::from(a == c));
			let weak: Weak<[u8]> = Rc::downgrade(&a);
			counts(&b, out);
			drop(a);
			counts(&b, out);
			out.push(u8::from(weak.upgrade().is_some_and(|up| Rc::ptr_eq(&up, &b))));
			out.push(weak.strong_count() as u8);
			drop(b);
			out.push(u8::from(weak.upgrade().is_none()));
			out.push(weak.strong_count() as u8);
			let arc: Arc<[u8]> = Arc::from(data.to_vec());
			let twin = Arc::clone(&arc);
			arc_counts(&arc, out);
			out.push(u8::from(Arc::ptr_eq(&arc, &twin)));
			let arc_weak = Arc::downgrade(&twin);
			arc_counts(&arc, out);
			drop(twin);
			arc_counts(&arc, out);
			drop(arc_weak);
			arc_counts(&arc, out);
			put_bytes(&arc, out);
		}
		// Written through `get_mut`, only when no other owner shares it, and `make_mut`, which
		// copies them first when one does.
		_ => {
			let mut a: Rc<[u8]> = Rc::from(data);
			if let Some(m) = Rc::get_mut(&mut a) {
				scramble(m);
			}
			put_bytes(&a, out);
			let b = a.clone();
			out.push(u8::from(Rc::get_mut(&mut a).is_none()));
			scramble(Rc::make_mut(&mut a));
			out.push(u8::from(Rc::ptr_eq(&a, &b)));
			counts(&a, out);
			counts(&b, out);
			put_bytes(&a, out);
			put_bytes(&b, out);
			// Alone again, `make_mut` writes in place.
			let before = a.clone();
			drop(before);
			Rc::make_mut(&mut a).fill(0x11);
			counts(&a, out);
			put_bytes(&a, out);
			// A `Weak` alone leaves it to `make_mut` to move it out, which the `Weak` no
			// longer sees.
			let weak = Rc::downgrade(&a);
			if let Some(first) = Rc::make_mut(&mut a).first_mut() {
				*first = 0x22;
			}
			out.push(u8::from(weak.upgrade().is_none()));
			put_bytes(&a, out);
			let mut arc: Arc<[u8]> = Arc::from(data);
			let shared = Arc::clone(&arc);
			Arc::make_mut(&mut arc).reverse();
			if let Some(m) = Arc::get_mut(&mut arc) {
				m.iter_mut().for_each(|x| *x ^= 0x0f);
			}
			put_bytes(&arc, out);
			put_bytes(&shared, out);
			arc_counts(&shared, out);
		}
	}
}

/// `u16`s, each way, written, compared, and counted.
pub fn wides(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let data = &data[..usize::from(kind >> 3) % (data.len() + 1)];
	let wide: Vec<u16> = data.iter().map(|&x| u16::from(x) * 257).collect();
	let from_ref: Rc<[u16]> = Rc::from(&wide[..]);
	let from_vec: Arc<[u16]> = Arc::from(wide.clone());
	let from_box: Rc<[u16]> = wide.clone().into_boxed_slice().into();
	let from_array = Rc::<[u16]>::from([1, 2, 0x300]);
	let collected: Rc<[u16]> = wide.iter().map(|&x| x.rotate_left(3)).collect();
	let empty: Arc<[u16]> = Arc::default();
	for w in [&from_ref, &from_box, &from_array, &collected] {
		put_wides(w, out);
	}
	put_wides(&from_vec, out);
	put_wides(&empty, out);
	out.push(u8::from(from_ref == from_box));
	out.push(collected.cmp(&from_ref) as u8);
	out.push(u8::from(hash(&collected) == hash(&collected[..])));
	put(out, &format!("{collected:?}|{from_vec:?}"));
	let mut shared = from_ref.clone();
	counts(&from_ref, out);
	match kind % 2 {
		0 => Rc::make_mut(&mut shared).sort_unstable_by(|a, b| b.cmp(a)),
		_ => {
			if let Some(last) = Rc::make_mut(&mut shared).last_mut() {
				*last = 0xbeef;
			}
		}
	}
	counts(&from_ref, out);
	out.push(u8::from(Rc::ptr_eq(&shared, &from_ref)));
	put_wides(&shared, out);
	put_wides(&from_ref, out);
	let mut alone = collected;
	if let Some(m) = Rc::get_mut(&mut alone) {
		m.iter_mut().for_each(|x| *x ^= 0x0f0f);
	}
	put_wides(&alone, out);
}

/// Values JS copies, held in a shared slice apart from what they came from, and from each
/// clone `make_mut` makes.
pub fn copies(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let data = &data[..usize::from(kind >> 3) % (data.len() + 1)];
	let mut source: Vec<(u8, u16)> = data.iter().map(|&b| (b, u16::from(b) << 4)).collect();
	let from_ref: Rc<[(u8, u16)]> = Rc::from(&source[..]);
	let from_vec: Arc<[(u8, u16)]> = Arc::from(source.clone());
	let p = (data.first().copied().unwrap_or(5), 6u16);
	let mut twins = Rc::<[(u8, u16)]>::from([p, p]);
	for s in source.iter_mut() {
		s.0 ^= 0xff;
	}
	Rc::get_mut(&mut twins).expect("alone")[0].0 ^= 1;
	pairs(&source, out);
	pairs(&from_ref, out);
	pairs(&from_vec, out);
	pairs(&twins, out);
	out.push(p.0);
	let mut shared = from_ref.clone();
	if let Some(first) = Rc::make_mut(&mut shared).first_mut() {
		first.1 = 1;
	}
	pairs(&shared, out);
	pairs(&from_ref, out);
	let quads: Rc<[[u8; 2]]> = data.chunks_exact(2).map(|c| [c[0], c[1]]).collect();
	let mut copy = quads.clone();
	if let Some(q) = Rc::make_mut(&mut copy).first_mut() {
		q[0] = 0x77;
	}
	for q in quads.iter().chain(copy.iter()) {
		out.extend_from_slice(q);
	}
	out.push(u8::from(quads == copy));
}

/// Items that log their clones and drops: each shared slice drops its items in order as its
/// last owner drops, and not before, however it was made.
pub fn drops(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let data = &data[..usize::from(kind >> 3) % (data.len() + 1)];
	let log = Rc::new(RefCell::new(Vec::new()));
	let logged = |id: u8| Logged { id, log: log.clone() };
	let items: Vec<Logged> = data.iter().map(|&b| logged(b & 0x7e)).collect();
	let shared: Rc<[Logged]> = Rc::from(items);
	ids(&shared, out);
	flush(&log, out);
	let twin = shared.clone();
	let weak = Rc::downgrade(&twin);
	flush(&log, out);
	drop(shared);
	flush(&log, out);
	let from_ref: Rc<[Logged]> = Rc::from(&twin[..]);
	flush(&log, out);
	drop(twin);
	flush(&log, out);
	out.push(u8::from(weak.upgrade().is_none()));
	drop(weak);
	flush(&log, out);
	let collected: Arc<[Logged]> = data.iter().map(|&b| logged(b | 1)).collect();
	ids(&collected, out);
	let from_box: Rc<[Logged]> = Rc::from(Box::<[Logged]>::from([logged(0x10), logged(0x12)]));
	let from_array: Arc<[Logged]> = Arc::from([logged(0x14)]);
	let cow: Rc<[Logged]> = Rc::from(Cow::Borrowed(&from_ref[..]));
	flush(&log, out);
	drop(from_array);
	drop(collected);
	flush(&log, out);
	// A clone `make_mut` makes clones every item, and the item it overwrites drops.
	let mut mine = from_box.clone();
	Rc::make_mut(&mut mine)[0] = logged(0x16);
	flush(&log, out);
	ids(&mine, out);
	ids(&from_box, out);
	drop(from_box);
	flush(&log, out);
	if let Some(m) = Rc::get_mut(&mut mine) {
		m[1] = logged(0x18);
	}
	flush(&log, out);
	let held = Held {
		items: mine,
		last: logged(0x20),
	};
	out.push(held.items.len() as u8);
	out.push(held.last.id);
	let kept = match kind % 2 {
		0 => Some(held.items.clone()),
		_ => None,
	};
	drop(held);
	flush(&log, out);
	drop(kept);
	flush(&log, out);
	let empty: Rc<[Logged]> = Rc::default();
	ids(&empty.clone(), out);
	drop(cow);
	drop(from_ref);
	flush(&log, out);
}

/// `Rc<str>` and `Arc<str>`, each way, read, counted, and written.
pub fn strs(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let s = text(data);
	let from_ref: Rc<str> = Rc::from(s.as_str());
	let from_string: Arc<str> = Arc::from(s.clone());
	let from_box: Rc<str> = Rc::from(s.clone().into_boxed_str());
	let mut source = s.clone();
	let from_mut: Arc<str> = Arc::from(source.as_mut_str());
	source.make_ascii_uppercase();
	let borrowed: Rc<str> = Rc::from(Cow::Borrowed(s.as_str()));
	let owned: Arc<str> = Arc::from(Cow::<str>::Owned(s.clone()));
	let into: Rc<str> = s.clone().into();
	let empty: Rc<str> = Rc::default();
	let arc_empty: Arc<str> = Arc::default();
	put(out, &from_ref);
	put(out, &from_string);
	put(out, &from_box);
	put(out, &from_mut);
	put(out, &source);
	put(out, &borrowed);
	put(out, &owned);
	put(out, &into);
	put(out, &empty);
	put(out, &arc_empty);
	out.push(u8::from(from_ref == into));
	out.push(from_ref.cmp(&empty) as u8);
	out.push(u8::from(hash(&from_ref) == hash(s.as_str())));
	out.push(from_ref.len() as u8);
	out.push(from_ref.chars().count() as u8);
	put(out, &format!("{from_ref}|{from_string:?}|{empty:?}"));
	let twin = from_ref.clone();
	counts(&twin, out);
	out.push(u8::from(Rc::ptr_eq(&twin, &from_ref)));
	out.push(u8::from(Rc::ptr_eq(&into, &from_ref)));
	let mut mine = into;
	match kind % 2 {
		0 => {
			if let Some(m) = Rc::get_mut(&mut mine) {
				m.make_ascii_uppercase();
			}
		}
		_ => Rc::make_mut(&mut twin.clone()).make_ascii_lowercase(),
	}
	let mut shared = twin;
	Rc::make_mut(&mut shared).make_ascii_uppercase();
	counts(&shared, out);
	counts(&from_ref, out);
	put(out, &mine);
	put(out, &shared);
	put(out, &from_ref);
	let mut arc = from_string.clone();
	Arc::make_mut(&mut arc).make_ascii_uppercase();
	put(out, &arc);
	put(out, &from_string);
	let back = String::from(&*arc);
	put(out, &back);
	let to: String = from_box.to_string();
	put(out, &to);
}

/// A generic type's shared slice, of bytes, of `u16`s, and of items that log.
pub fn generics(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let data = &data[..usize::from(kind >> 3) % (data.len() + 1)];
	let mut bytes = Shared::new(data.to_vec());
	let copy = bytes.clone();
	bytes.set_last(0xee);
	put_bytes(&bytes.items, out);
	put_bytes(&copy.items, out);
	out.push(bytes.first().copied().unwrap_or(0xaa));
	counts(&copy.items, out);
	let mut wide = Shared::collected(data.iter().map(|&b| u16::from(b) << 8));
	let twin = wide.clone();
	counts(&wide.items, out);
	wide.set_last(0xbeef);
	put_wides(&wide.items, out);
	put_wides(&twin.items, out);
	let log = Rc::new(RefCell::new(Vec::new()));
	let logged = |id: u8| Logged { id, log: log.clone() };
	let mut items = Shared::new(data.iter().map(|&b| logged(b & 0x7e)).collect());
	let kept = items.clone();
	flush(&log, out);
	items.set_last(logged(0x40));
	flush(&log, out);
	ids(&items.items, out);
	out.push(items.first().map_or(0xaa, |l| l.id));
	drop(kept);
	flush(&log, out);
	drop(items);
	flush(&log, out);
}

/// Shared slices and text as keys a borrow of their slice or text looks up, and through
/// `AsRef`, `Borrow`, and `FromIterator::from_iter`.
pub fn keys(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let data = &data[..usize::from(kind >> 3) % (data.len() + 1)];
	let words: Vec<String> = data.chunks(2).map(text).collect();
	let interned: HashSet<Rc<str>> = words.iter().map(|w| Rc::from(w.as_str())).collect();
	out.push(interned.len() as u8);
	for w in &words {
		out.push(u8::from(interned.contains(w.as_str())));
		out.push(interned.get(w.as_str()).map_or(0, |k| Rc::strong_count(k) as u8));
	}
	out.push(u8::from(interned.contains("\u{0}")));
	let mut counted: HashMap<Arc<[u8]>, u8> = HashMap::new();
	for c in data.chunks(3) {
		*counted.entry(Arc::from(c)).or_default() += 1;
	}
	for c in data.chunks(3) {
		out.push(counted.get(c).copied().unwrap_or(0));
	}
	let from_iter = Rc::<[u16]>::from_iter(data.iter().map(|&b| u16::from(b) + 1));
	put_wides(from_iter.as_ref(), out);
	let borrowed: &[u16] = from_iter.borrow();
	put_wides(borrowed, out);
	let text: Arc<str> = Arc::from(words.concat());
	let s: &str = text.as_ref();
	put(out, s);
	let b: &str = text.borrow();
	put(out, b);
	let mut slot: Rc<[u8]> = Rc::from(data);
	let other: Rc<[u8]> = Rc::from(&data[..data.len() / 2]);
	slot.clone_from(&other);
	out.push(u8::from(Rc::ptr_eq(&slot, &other)));
	counts(&other, out);
}

/// What `as_ptr` points at in an `Rc<[u16]>` and an `Arc<[u8]>`, by `kind` alone, shared by
/// another handle, or by a `Weak` alone, read after `make_mut` writes it.
pub fn addresses(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let mut wide: Rc<[u16]> = data.iter().map(|&b| u16::from(b) << 4).collect();
	let mut bytes: Arc<[u8]> = Arc::from(data);
	let other = (kind % 3 == 1).then(|| (Rc::clone(&wide), Arc::clone(&bytes)));
	let weak = (kind % 3 == 2).then(|| (Rc::downgrade(&wide), Arc::downgrade(&bytes)));
	if let Some(first) = Rc::make_mut(&mut wide).first_mut() {
		*first ^= u16::from(kind);
	}
	if let Some(last) = Arc::make_mut(&mut bytes).last_mut() {
		*last ^= kind;
	}
	let (wide, bytes) = unsafe { (&*Rc::as_ptr(&wide), &*Arc::as_ptr(&bytes)) };
	out.extend(wide.iter().flat_map(|w| w.to_le_bytes()));
	put_bytes(bytes, out);
	out.push(u8::from(
		other.is_some() || weak.is_some_and(|(a, b)| a.upgrade().is_none() && b.upgrade().is_none()),
	));
}

/// An `Rc` and an `Arc` of an array, and a `Weak` of one, unsized to their slice while
/// clones of the array's are kept: counted, compared, read, written through `get_mut` and
/// `make_mut`, and dropped by whichever owner is last, of values that log, of `u16`s, and
/// of bytes.
pub fn unsized_arrays(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let log = Rc::new(RefCell::new(Vec::new()));
	let logged = |id: u8| Logged { id, log: log.clone() };
	let at = |i: usize| data.get(i).copied().unwrap_or(i as u8) & 0x7e;
	let array: Rc<[Logged; 2]> = Rc::new([logged(at(0)), logged(at(1))]);
	let kept = (kind & 1 != 0).then(|| array.clone());
	let weak = Rc::downgrade(&array);
	let weak: Weak<[Logged]> = weak;
	let mut shared: Rc<[Logged]> = array;
	out.extend([Rc::strong_count(&shared) as u8, Rc::weak_count(&shared) as u8]);
	out.push(u8::from(
		kept.as_ref()
			.is_some_and(|k| Rc::ptr_eq(&(k.clone() as Rc<[Logged]>), &shared)),
	));
	ids(&shared, out);
	if let Some(items) = Rc::get_mut(&mut shared) {
		items[0] = logged(0x40);
	}
	flush(&log, out);
	Rc::make_mut(&mut shared)[1] = logged(0x42);
	flush(&log, out);
	ids(&shared, out);
	if let Some(k) = &kept {
		ids(&k[..], out);
	}
	let upgraded = weak.upgrade();
	out.push(upgraded.as_ref().map_or(0xff, |u| u.len() as u8));
	drop(upgraded);
	match kind & 2 {
		0 => {
			drop(shared);
			flush(&log, out);
			drop(kept);
		}
		_ => {
			drop(kept);
			flush(&log, out);
			drop(shared);
		}
	}
	flush(&log, out);
	out.push(u8::from(weak.upgrade().is_none()));
	let words: Arc<[u16; 3]> = Arc::new([u16::from(at(2)) << 8 | 1, 2, 3]);
	let twin = words.clone();
	let mut wide: Arc<[u16]> = words;
	out.push(Arc::strong_count(&twin) as u8);
	Arc::make_mut(&mut wide)[2] = 0x700;
	out.extend(wide.iter().chain(twin.iter()).map(|&w| (w ^ w >> 8) as u8));
	let bytes: Arc<[u8; 2]> = Arc::new([at(3), at(4)]);
	let more = bytes.clone();
	let slice: Arc<[u8]> = bytes;
	out.extend(slice.iter().chain(more.iter()));
	out.push(u8::from(Arc::ptr_eq(&slice, &(more as Arc<[u8]>))));
}
