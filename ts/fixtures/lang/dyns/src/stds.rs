//! Trait objects of std's traits but those `dyns` covers: a `dyn Hasher` of std's
//! `DefaultHasher` or the crate's own, written by each of its methods and by `Hash`, and a
//! `dyn fmt::Write` of a `String`, a `Formatter`, or the crate's own writer, by `write!` and
//! each of its methods, borrowed and boxed. Each call runs the impl of the value's own type,
//! or std's provided method where the impl keeps it, and a box drops what it holds.

use std::cell::RefCell;
use std::fmt::{self, Write};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::rc::Rc;

use crate::harness::put;

/// FNV-1a over what it is written, which logs each `write_u8` it writes itself, and keeps
/// std's other provided methods.
struct Fnv(u64, Rc<RefCell<Vec<u8>>>);

impl Hasher for Fnv {
	fn write(&mut self, bytes: &[u8]) {
		for &b in bytes {
			self.0 = (self.0 ^ u64::from(b)).wrapping_mul(0x100_0000_01b3);
		}
	}
	fn write_u8(&mut self, b: u8) {
		self.1.borrow_mut().push(b);
		self.write(&[b]);
	}
	fn finish(&self) -> u64 {
		self.0
	}
}

impl Drop for Fnv {
	fn drop(&mut self) {
		self.1.borrow_mut().push(0xfe);
	}
}

/// A number, which JS copies, so `&mut self` reaches it through a place.
struct Sum(u32);

impl Hasher for Sum {
	fn write(&mut self, bytes: &[u8]) {
		for &b in bytes {
			self.0 = self.0.rotate_left(5) ^ u32::from(b);
		}
	}
	fn write_u32(&mut self, x: u32) {
		self.0 = self.0.wrapping_add(x);
	}
	fn finish(&self) -> u64 {
		u64::from(self.0)
	}
}

/// A key of the crate's own, which hashes its fields into whichever hasher it is given.
#[derive(Hash)]
struct Key {
	id: u16,
	name: String,
	tags: Vec<u8>,
}

/// Hashes its value into a `dyn Hasher`, as crates do to hash a trait object.
trait DynHash {
	fn dyn_hash(&self, h: &mut dyn Hasher);
}

impl<T: Hash> DynHash for T {
	fn dyn_hash(&self, mut h: &mut dyn Hasher) {
		self.hash(&mut h);
	}
}

/// Each of `Hasher`'s methods, then `Hash` through the `&mut dyn Hasher` and objects of
/// `DynHash`, then `finish`.
fn hash_into(h: &mut dyn Hasher, data: &[u8], keys: &[&dyn DynHash]) -> u64 {
	h.write(data);
	for &b in data {
		h.write_u8(b);
	}
	h.write_u16(data.len() as u16);
	h.write_u32(0x1234_5678);
	h.write_u64(u64::MAX - 1);
	h.write_u128(1 << 100);
	h.write_usize(7);
	h.write_i8(-1);
	h.write_i16(-2);
	h.write_i32(-3);
	h.write_i64(-4);
	h.write_i128(-5);
	h.write_isize(-6);
	let mut h = h;
	data.hash(&mut h);
	"str".hash(&mut h);
	(data.len(), 'x').hash(&mut h);
	for key in keys {
		key.dyn_hash(h);
	}
	h.finish()
}

/// A writer of the crate's own, upper-casing what it is written, which fails once it holds
/// `limit` bytes, logs each char `write_char` writes itself, and logs as it drops.
struct Upper {
	s: String,
	limit: usize,
	log: Rc<RefCell<Vec<u8>>>,
}

impl Write for Upper {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		if self.s.len() + s.len() > self.limit {
			return Err(fmt::Error);
		}
		self.s.push_str(&s.to_uppercase());
		Ok(())
	}
	fn write_char(&mut self, c: char) -> fmt::Result {
		self.log.borrow_mut().push(c as u8);
		self.write_str(c.encode_utf8(&mut [0; 4]))
	}
}

impl Drop for Upper {
	fn drop(&mut self) {
		self.log.borrow_mut().push(0xfd);
	}
}

/// A writer that counts what it is written and keeps std's `write_char`.
struct Counter(usize);

impl Write for Counter {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		self.0 += s.chars().count();
		Ok(())
	}
}

/// Each of `fmt::Write`'s methods through the object, a placeholder too.
fn render(w: &mut dyn Write, n: u8, s: &str) -> fmt::Result {
	write!(w, "[{n}")?;
	w.write_char('é')?;
	w.write_str(s)?;
	write!(w, "{s:>5}|{n:#x}]")?;
	write!(w, "!")
}

/// Over a writer that may be a trait object, as a type parameter instantiates it.
fn twice<W: Write + ?Sized>(w: &mut W, s: &str) -> fmt::Result {
	w.write_str(s)?;
	w.write_str(s)
}

/// Writes itself through a `dyn Write` of its `Formatter`, whose options it ignores.
struct Rendered(u8);

impl fmt::Display for Rendered {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		render(f, self.0, "f")
	}
}

pub fn stds(data: &[u8], out: &mut Vec<u8>) {
	let [a, b, rest @ ..] = data else {
		return;
	};
	let (a, b) = (*a, *b);
	let log = Rc::new(RefCell::new(Vec::new()));
	let key = Key {
		id: u16::from(a) << 8 | u16::from(b),
		name: String::from_utf8_lossy(rest).into_owned(),
		tags: rest.to_vec(),
	};
	let keys: [&dyn DynHash; 3] = [&key, &a, &"k"];

	// Into std's hasher, the crate's own, and one JS copies, borrowed and boxed.
	let mut std = DefaultHasher::new();
	let mut fnv = Fnv(0xcbf2_9ce4_8422_2325, log.clone());
	let mut sum = Sum(u32::from(b));
	put(
		out,
		&format!(
			"{} {} {} {} {:?} ",
			hash_into(&mut std, rest, &keys),
			hash_into(&mut fnv, rest, &keys),
			hash_into(&mut sum, rest, &keys),
			sum.0,
			log.borrow(),
		),
	);
	let mut boxed: Box<dyn Hasher> = match a % 3 {
		0 => Box::new(DefaultHasher::new()),
		1 => Box::new(Fnv(u64::from(b), log.clone())),
		_ => Box::new(Sum(u32::from(a))),
	};
	boxed.write_u8(b);
	let mut native = DefaultHasher::new();
	key.dyn_hash(&mut native);
	// Into a `&mut` and a box of std's hasher and the crate's own, which std's impls forward.
	(a, &key.name).hash(&mut &mut native);
	key.hash(&mut boxed);
	let mut sum = Sum(u32::from(a));
	key.hash(&mut &mut sum);
	put(out, &format!("{} {} {} ", boxed.finish(), native.finish(), sum.0));
	drop(boxed);
	put(out, &format!("{:?} ", log.borrow()));
	drop(fnv);
	put(out, &format!("{:?} ", log.borrow()));
	log.borrow_mut().clear();

	// Into a `String`, a `Formatter`, and the crate's own writers, borrowed and boxed.
	let text = String::from_utf8_lossy(rest).into_owned();
	let mut s = String::from("s:");
	let written = render(&mut s, a, &text);
	let mut upper = Upper {
		s: String::new(),
		limit: usize::from(b % 32),
		log: log.clone(),
	};
	let failed = render(&mut upper, a, &text);
	let mut counter = Counter(0);
	let counted = render(&mut counter, b, &text);
	let w: &mut dyn Write = &mut s;
	let again = twice(w, "+");
	// Through std's impl for a `&mut` writer.
	let mut inner = &mut s;
	let w: &mut dyn Write = &mut inner;
	let again = again.and(render(w, b, "&"));
	put(
		out,
		&format!(
			"{s} {written:?} {} {failed:?} {} {counted:?} {again:?} {:?} ",
			upper.s,
			counter.0,
			log.borrow()
		),
	);
	put(out, &format!("{} {:>12} ", Rendered(a), Rendered(b)));
	let mut boxed: Box<dyn Write> = match b % 2 {
		0 => Box::new(Counter(usize::from(a))),
		_ => Box::new(Upper {
			s: String::from("x"),
			limit: 64,
			log: log.clone(),
		}),
	};
	let written = render(&mut *boxed, b, "b");
	drop(boxed);
	put(out, &format!("{written:?} {:?} ", log.borrow()));
	drop(upper);
	put(out, &format!("{:?}", log.borrow()));
}
