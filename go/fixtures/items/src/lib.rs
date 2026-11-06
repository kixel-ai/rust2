//! Items whose Go names or identity take care: fields and variants whose escapes could
//! meet, constant memory, which every use of shares, and the `pub` API, which Go callers
//! reach by the paths the crate exports it at.

/// A `pub` type in a private module, exported by its re-export with its `pub` methods.
mod counter {
	pub struct Counter {
		pub count: u8,
	}

	impl Counter {
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

/// Another crate's module, whose `pub` items Go callers reach as this crate's.
pub use items_dep::shapes;

/// Fields Go would name alike if escaping were not one to one.
struct Fields {
	r#type: u8,
	type_: u8,
	tag: u8,
	tag_: u8,
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
	match pick % 3 {
		0 => {
			let f = Fields {
				r#type: at(0),
				type_: at(1),
				tag: at(2),
				tag_: at(3),
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
			vec![f.r#type, f.type_, f.tag, f.tag_, k, v]
		}
		1 => vec![std::ptr::eq(byte(), byte()) as u8, std::ptr::eq(bytes(), bytes()) as u8],
		_ => {
			let mut a = Count::new(at(0));
			let b = Count::new(at(1));
			let bumped = a.bump();
			let meter = Meter::new(at(2));
			vec![bumped, tallies::sum(&a, &b).sum, meter.0, shapes::area(at(3), at(4))]
		}
	}
}
