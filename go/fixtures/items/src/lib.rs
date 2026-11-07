//! Items whose Go names or identity take care: fields and variants whose escapes could
//! meet, constant memory, which every use of shares, a constant, a fresh value at each
//! use, a static, one place every use shares, and the `pub` API, which Go callers reach by
//! the paths the crate exports it at.

/// A `pub` type in a private module, exported by its re-export with its `pub` methods.
mod counter {
	pub struct Counter {
		pub count: u8,
	}

	impl Counter {
		/// `CountMax`, a Go constant.
		pub const MAX: u8 = u8::MAX;
		/// `CountStart()`, a fresh value at each call, which bumping one leaves the next
		/// alone.
		pub const START: Counter = Counter { count: 1 };

		/// `CountNew`.
		pub fn new(count: u8) -> Self {
			Self { count }
		}

		/// `CountBump`.
		pub fn bump(&mut self) -> u8 {
			self.count = self.count.wrapping_add(1);
			self.hidden()
		}

		/// Private, so no Go caller reaches it.
		fn hidden(&self) -> u8 {
			self.count
		}
	}
}

/// Re-exported renamed and again under `tallies`, so Go names it by the shorter path:
/// `Count`, not `TalliesCounter`.
pub use counter::Counter as Count;

pub mod tallies {
	pub use crate::counter::Counter;

	/// `TalliesTotal`, by its module path.
	pub struct Total {
		pub sum: u8,
	}

	/// `TalliesSum`.
	pub fn sum(a: &crate::Count, b: &crate::Count) -> Total {
		Total {
			sum: a.count.wrapping_add(b.count),
		}
	}
}

mod gauge {
	pub struct Gauge(pub u8);

	impl Gauge {
		/// `MeterNew`.
		pub fn new(v: u8) -> Self {
			Self(v)
		}
	}
}

/// The only path to `Gauge`, so Go names it `Meter`.
pub type Meter = gauge::Gauge;

mod value {
	pub struct Value<T>(pub T);
}

/// A Go alias of a specialized type: `type Byte = Value_2[uint8]`, since the alias below
/// takes `Value`.
pub type Byte = value::Value<u8>;

/// An alias whose name a type it does not name would otherwise take first.
pub type Value = u16;

/// A Go alias of a type with no declaration of its own: `type Level = uint8`.
pub type Level = u8;

/// A Go alias of a generic type: `type Pair[T any] = tuple2[T, T]`.
pub type Pair<T> = (T, T);

/// `Unwrap`, through the aliases.
pub fn unwrap(b: Byte) -> Pair<Level> {
	(b.0, b.0.wrapping_add(1))
}

/// `MaxLen`, a Go constant.
pub const MAX_LEN: u8 = 200;
/// `Floor`, a Go constant at the edge of its type.
pub const FLOOR: i64 = i64::MIN;
/// `Enabled`.
pub const ENABLED: bool = true;
/// `Letter`, a Go rune.
pub const LETTER: char = 'é';
/// `Greeting`, a Go string, which escapes what Go's literal would end at.
pub const GREETING: &str = "hi \"go\"\n\\é";
/// `Origin()`, a fresh value at each call.
pub const ORIGIN: tallies::Total = tallies::Total { sum: 7 };
/// `Hits`, one variable, which translated code reads what Go callers write to.
pub static HITS: Count = Count { count: 3 };
/// `Table`, a slice of a static's own memory.
pub static TABLE: &[u16] = &[1, 2, 300];
/// `Name`.
pub static NAME: &str = "items";
/// `Bytes`, whose address every use shares.
pub static BYTES: [u8; 3] = *b"abc";
/// `Letters`, which the slices below point into.
pub static LETTERS: [u8; 4] = *b"wxyz";
/// `Tail`, a slice of part of another static, which Go callers see a write to it through.
pub static TAIL: &[u8] = LETTERS.split_at(1).1;
/// `Text`, a Go string from a static's bytes.
pub const TEXT: &str = match std::str::from_utf8(&LETTERS) {
	Ok(text) => text,
	Err(_) => panic!(),
};
/// A slice of all of a static, from a constant.
const ALL: &[u8] = &LETTERS;
/// A constant pointing at a static points at the static.
const HITS_AT: &Count = &HITS;

mod hidden {
	pub struct Shape {
		pub sides: u8,
	}

	pub struct Edge(pub u8);
}

/// Of a type no path exports, whose Go name the function after it takes.
pub static TRIANGLE: hidden::Shape = hidden::Shape { sides: 3 };

/// `Shape`, which the static's type above does not take.
pub fn shape() -> u8 {
	TRIANGLE.sides
}

/// `Edge`, which the static's type below does not take.
pub const EDGE: u8 = 2;

pub static EDGES: hidden::Edge = hidden::Edge(5);

/// Two statics each pointing at the other, which Go initializes after declaring both.
struct Link {
	next: &'static Link,
	value: u8,
}

static RING_A: Link = Link {
	next: &RING_B,
	value: 1,
};
static RING_B: Link = Link {
	next: &RING_A,
	value: 2,
};

/// Another crate's module, whose `pub` items Go callers reach as this crate's.
pub use items_dep::shapes;

/// Fields Go would name alike if escaping were not one to one, a `pub` one exported.
#[allow(non_snake_case, reason = "the names Go escapes")]
struct Fields {
	r#type: u8,
	type_: u8,
	tag: u8,
	tag_: u8,
	pub tally: u8,
	Tally: u8,
	_Tally: u8,
}

/// Variants Go would hold in one field if escaping were not one to one.
#[allow(non_camel_case_types, reason = "the names Go escapes")]
enum Kind {
	tag(u8),
	tag_(u8),
	r#type(u8),
}

fn byte() -> &'static u8 {
	&42
}

fn bytes() -> &'static [u8; 3] {
	b"abc"
}

/// Run the case the first byte picks on the rest.
pub fn run(input: &[u8]) -> Vec<u8> {
	let Some((&pick, data)) = input.split_first() else {
		return Vec::new();
	};
	let at = |i: usize| data.get(i).copied().unwrap_or(0);
	match pick % 4 {
		0 => {
			let f = Fields {
				r#type: at(0),
				type_: at(1),
				tag: at(2),
				tag_: at(3),
				tally: at(6),
				Tally: at(7),
				_Tally: at(8),
			};
			let kind = match at(4) % 3 {
				0 => Kind::tag(at(5)),
				1 => Kind::tag_(at(5)),
				_ => Kind::r#type(at(5)),
			};
			let (k, v) = match kind {
				Kind::tag(v) => (0, v),
				Kind::tag_(v) => (1, v),
				Kind::r#type(v) => (2, v),
			};
			vec![f.r#type, f.type_, f.tag, f.tag_, f.tally, f.Tally, f._Tally, k, v]
		}
		1 => vec![std::ptr::eq(byte(), byte()) as u8, std::ptr::eq(bytes(), bytes()) as u8],
		2 => {
			// A constant's use is its own value, which changing leaves the next use alone.
			let mut start = Count::START;
			start.bump();
			let mut origin = ORIGIN;
			origin.sum = origin.sum.wrapping_add(at(0));
			let mut ring = &RING_A;
			for _ in 0..at(1) % 5 {
				ring = ring.next;
			}
			let mut out = vec![
				start.count,
				Count::START.count,
				origin.sum,
				ORIGIN.sum,
				HITS.count,
				std::ptr::eq(&HITS, HITS_AT) as u8,
				std::ptr::eq(&BYTES, &BYTES) as u8,
				std::ptr::eq(&TAIL[0], &LETTERS[1]) as u8,
				std::ptr::eq(&ALL[3], &TAIL[2]) as u8,
				TAIL.len() as u8,
				std::ptr::eq(&TEXT.as_bytes()[2], &LETTERS[2]) as u8,
				shape(),
				EDGE,
				EDGES.0,
				std::ptr::eq(RING_A.next.next, &RING_A) as u8,
				ring.value,
				MAX_LEN,
				Count::MAX,
				ENABLED as u8,
				LETTER as u8,
				(FLOOR >> 56) as u8,
				shapes::UNIT,
				shapes::SIDES,
				// Past the end where the second byte says, which panics as Rust's does.
				BYTES[usize::from(at(2) % 4)],
			];
			for &v in TABLE {
				out.push((v >> 8) as u8);
				out.push(v as u8);
			}
			for s in &[NAME, GREETING, TEXT] {
				for &b in s.as_bytes() {
					out.push(b);
				}
			}
			out
		}
		_ => {
			let mut a = Count::new(at(0));
			let b = Count::new(at(1));
			let bumped = a.bump();
			let meter = Meter::new(at(2));
			let (x, y) = unwrap(value::Value(at(5)));
			vec![
				bumped,
				tallies::sum(&a, &b).sum,
				meter.0,
				shapes::area(at(3), at(4)),
				x,
				y,
			]
		}
	}
}
