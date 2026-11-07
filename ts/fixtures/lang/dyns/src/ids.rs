//! `TypeId`s, of `TypeId::of`, of a value's `type_id`, and of a trait object's, which is
//! its value's type's: borrowed, changed in place, boxed, shared, and upcast from the
//! crate's own trait over `Any`, but for a box or an `Rc`'s own `type_id`, which is the
//! pointer's type's. Each is equal to another only for the same type, in a struct's
//! derives, a hash map, and a `BTreeSet` too. Its value is the crate's metadata's, so each
//! check compares within a run: `Debug` writes `TypeId(0x…)` of its 128-bit hash, `Ord`
//! orders by that hash's pointer-sized words, lowest first, and `Hash` writes one `u64`,
//! the hash's high half.

use std::any::{Any, TypeId};
use std::collections::{BTreeSet, HashMap, HashSet};
use std::fmt::{self, Write};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use crate::harness::put;

/// A value of a type of its own, which only its `TypeId` tells from a `u8`.
#[derive(Debug, Clone, Copy)]
struct Byte(u8);

/// An enum, which JS copies, so a `&mut dyn Any` of one is of a class of its place.
enum Mood {
	Calm,
	Loud,
}

/// A trait over `Any`, whose objects give their value's `type_id`.
trait Marked: Any {
	fn tag(&self) -> u8;
}

impl Marked for u8 {
	fn tag(&self) -> u8 {
		*self
	}
}

impl Marked for Byte {
	fn tag(&self) -> u8 {
		self.0
	}
}

/// A `TypeId` in a struct, whose derives compare, order, and hash it as std's.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct WithId {
	id: TypeId,
	n: u8,
}

/// Logs each call it is given, and how many bytes `write` is.
#[derive(Default)]
struct Log(Vec<String>);

impl Hasher for Log {
	fn write(&mut self, bytes: &[u8]) {
		self.0.push(format!("write {}", bytes.len()));
	}
	fn write_u8(&mut self, x: u8) {
		self.0.push(format!("u8 {x}"));
	}
	fn write_u64(&mut self, x: u64) {
		self.0.push(format!("u64 {x:x}"));
	}
	fn finish(&self) -> u64 {
		0
	}
}

/// Takes at most `cap` bytes, logging each `write_str` it takes, and fails the call that
/// would pass `cap`.
struct Capped {
	log: Vec<String>,
	len: usize,
	calls: usize,
	cap: usize,
}

impl Write for Capped {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		self.calls += 1;
		if self.len + s.len() > self.cap {
			return Err(fmt::Error);
		}
		self.len += s.len();
		self.log.push(s.to_string());
		Ok(())
	}
}

/// `id`'s `Debug`, written into a `Capped` of `cap` bytes.
fn capped(id: TypeId, cap: usize) -> (fmt::Result, Capped) {
	let mut w = Capped {
		log: Vec::new(),
		len: 0,
		calls: 0,
		cap,
	};
	let result = write!(w, "{id:?}");
	(result, w)
}

/// A `TypeId` the compiler evaluates.
const U8: TypeId = TypeId::of::<u8>();

/// A reference to one, which the compiler places in memory.
const U8_REF: &TypeId = &TypeId::of::<u8>();

/// `T`'s `TypeId`, through a type parameter.
fn id_of<T: ?Sized + 'static>() -> TypeId {
	TypeId::of::<T>()
}

/// The `TypeId` of what `x` is, through a type parameter that may be a trait object.
fn id_of_val<T: ?Sized + 'static>(_: &T) -> TypeId {
	TypeId::of::<T>()
}

/// The 128-bit hash `Debug` writes, if it writes `TypeId(0x…)` of 32 hex digits.
fn hash_of(id: TypeId) -> Option<u128> {
	let s = format!("{id:?}");
	let digits = s.strip_prefix("TypeId(0x")?.strip_suffix(')')?;
	match digits.len() == 32 && digits.bytes().all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase()) {
		true => u128::from_str_radix(digits, 16).ok(),
		false => None,
	}
}

/// The pointer-sized words of `id`'s hash as it lays them out, lowest first, which its
/// derived `Ord` compares.
fn words(id: TypeId) -> Vec<u128> {
	let hash = hash_of(id).unwrap_or_default();
	let bits = usize::BITS;
	let mask = u128::MAX >> (128 - bits);
	(0..128 / bits).map(|i| (hash >> (i * bits)) & mask).collect()
}

pub fn ids(data: &[u8], out: &mut Vec<u8>) {
	let [a, b, rest @ ..] = data else {
		return;
	};
	let (a, b) = (*a, *b);

	// A type's own, through a type parameter, of a value, and of a trait object of a value.
	let mut n = u32::from(a);
	let mut mood = if b % 2 == 0 { Mood::Calm } else { Mood::Loud };
	let byte = Byte(b);
	let ids = [
		TypeId::of::<u8>(),
		TypeId::of::<u32>(),
		TypeId::of::<Byte>(),
		TypeId::of::<Mood>(),
		TypeId::of::<&'static str>(),
		TypeId::of::<str>(),
		TypeId::of::<()>(),
		TypeId::of::<Option<u8>>(),
		TypeId::of::<[u8]>(),
		TypeId::of::<dyn Any>(),
		TypeId::of::<Box<dyn Any>>(),
		TypeId::of::<dyn Marked>(),
		TypeId::of::<fn(u8) -> u8>(),
	];
	let found = [
		id_of::<u8>(),
		U8,
		*U8_REF,
		const { TypeId::of::<Byte>() },
		a.type_id(),
		n.type_id(),
		byte.type_id(),
		id_of_val("str"),
		id_of_val(&mood),
		id_of_val::<dyn Any>(&a),
		id_of::<Option<u8>>(),
	];
	for id in found {
		put(out, &format!("{:?} ", ids.iter().position(|i| *i == id)));
	}
	put(out, &format!("{:?} ", ids.iter().position(|i| i == U8_REF)));

	// A trait object's is its value's, borrowed, changed in place, which JS copies for a
	// number and an enum, boxed, shared, and upcast, but a box's and an `Rc`'s own are theirs.
	let objects: [&dyn Any; 5] = [&a, &n, &byte, &mood, &"str"];
	for x in objects {
		put(out, &format!("{:?} ", ids.iter().position(|i| *i == x.type_id())));
	}
	let places: [&mut dyn Any; 2] = [&mut n, &mut mood];
	for x in places {
		put(out, &format!("{:?} ", ids.iter().position(|i| *i == (*x).type_id())));
	}
	let marked: [&dyn Marked; 2] = [&a, &byte];
	for t in marked {
		let any: &dyn Any = t;
		put(
			out,
			&format!(
				"{} {:?} {} {} ",
				t.tag(),
				ids.iter().position(|i| *i == t.type_id()),
				t.type_id() == any.type_id(),
				id_of_val(t) == TypeId::of::<dyn Marked>(),
			),
		);
	}
	let mut boxes: Vec<Box<dyn Any>> = Vec::new();
	for &x in rest {
		boxes.push(match x % 3 {
			0 => Box::new(x),
			1 => Box::new(Byte(x)),
			_ => Box::new(Mood::Loud),
		});
	}
	for x in &boxes {
		let inner = (**x).type_id();
		put(
			out,
			&format!(
				"{:?} {} ",
				ids.iter().position(|i| *i == inner),
				x.type_id() == TypeId::of::<Box<dyn Any>>()
			),
		);
	}
	let shared: Rc<dyn Any> = Rc::new(byte);
	let boxed: Box<dyn Marked> = Box::new(a);
	put(
		out,
		&format!(
			"{} {} {} ",
			(*shared).type_id() == TypeId::of::<Byte>(),
			shared.type_id() == TypeId::of::<Rc<dyn Any>>(),
			(*boxed).type_id() == TypeId::of::<u8>(),
		),
	);

	// Equal only for the same type, ordered by its hash's words lowest first, and hashed as
	// one `u64`, its hash's high half, written alone.
	let pairs = || {
		ids.iter()
			.enumerate()
			.flat_map(|(i, &x)| ids.iter().enumerate().map(move |(j, &y)| (i, x, j, y)))
	};
	put(
		out,
		&format!(
			"{} {} {} {} ",
			pairs().all(|(i, x, j, y)| (x == y) == (i == j) && (x != y) == (i != j)),
			pairs().all(|(_, x, _, y)| x.cmp(&y) == words(x).cmp(&words(y))),
			pairs().all(|(_, x, _, y)| x.partial_cmp(&y) == Some(words(x).cmp(&words(y)))),
			pairs().all(|(_, x, _, y)| (x < y) == (words(x) < words(y)) && (x >= y) == (words(x) >= words(y))),
		),
	);
	let (x, y) = (ids[usize::from(a) % ids.len()], ids[usize::from(b) % ids.len()]);
	put(out, &format!("{} ", ids.iter().all(|&id| hash_of(id).is_some())));
	// `Debug` writes as std's `write!` does: `TypeId(`, `0x`, a `write_char` of each leading
	// zero, the digits, and `)`, stopping at the first write the writer fails.
	let (result, w) = capped(x, usize::from(a % 10));
	put(out, &format!("{result:?} {} {:?} ", w.calls, w.log));
	let debug = format!("{x:?}");
	let zeros = debug[9..].bytes().take_while(|&b| b == b'0').count().min(31);
	let (result, w) = capped(x, usize::MAX);
	let (digits, n) = (&w.log[2..w.log.len() - 1], w.log.len());
	put(
		out,
		&format!(
			"{result:?} {:?} {:?} {} {} ",
			&w.log[..2],
			w.log.last(),
			n == 4 + zeros && digits[..zeros].iter().all(|d| d == "0"),
			digits.concat() == debug[9..41],
		),
	);
	let (result, w) = capped(x, debug.len() - 1);
	put(out, &format!("{result:?} {} ", w.log.concat() + ")" == debug));
	let mut log = Log::default();
	x.hash(&mut log);
	let high = hash_of(x).map(|h| format!("u64 {:x}", h >> 64));
	put(out, &format!("{} {} ", log.0.len(), log.0.first() == high.as_ref()));
	put(
		out,
		&format!("{} ", x.max(y) == if words(x) < words(y) { y } else { x }),
	);

	// Sorted, in sets and maps, and in a struct's derives.
	let mut sorted = ids.to_vec();
	sorted.sort();
	let ordered = sorted.windows(2).all(|w| words(w[0]) < words(w[1]));
	let set: BTreeSet<TypeId> = ids.iter().chain(&found).copied().collect();
	let hashed: HashSet<TypeId> = ids.iter().chain(&found).copied().collect();
	put(out, &format!("{ordered} {} {} ", set.len(), hashed.len()));
	let names: HashMap<TypeId, &str> = [
		(TypeId::of::<u8>(), "u8"),
		(TypeId::of::<Byte>(), "byte"),
		(TypeId::of::<Mood>(), "mood"),
	]
	.into_iter()
	.collect();
	for x in &boxes {
		put(out, &format!("{:?} ", names.get(&(**x).type_id())));
	}
	let keys = [WithId { id: x, n: a }, WithId { id: y, n: b }, WithId { id: x, n: b }];
	let by_id = |k: &WithId, l: &WithId| words(k.id).cmp(&words(l.id)).then(k.n.cmp(&l.n));
	let order: Vec<(bool, bool)> = keys
		.iter()
		.flat_map(|k| keys.iter().map(move |l| (k, l)))
		.map(|(k, l)| (k == l, k.cmp(l) == by_id(k, l)))
		.collect();
	put(out, &format!("{order:?} "));
	let mut log = Log::default();
	keys[0].hash(&mut log);
	put(out, &format!("{} {:?}", log.0.len(), log.0.get(1)));
}
