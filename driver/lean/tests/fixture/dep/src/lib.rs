//! A dependency of the fixture, which rust2-lean translates from its MIR as it does the
//! fixture's own code.

use std::time::Duration;

/// A trait whose only requirement is `Ord`, so a bound by it orders as `Ord` does.
pub trait Ordered: Ord {}

impl<T: Ord> Ordered for T {}

/// A trait sharing `core::ops::Sub`'s names, which no `std` model may take for it.
pub trait Sub {
	fn sub(self, other: Self) -> std::time::Duration;
}

impl Sub for kio::clock::instant::Instant {
	fn sub(self, other: Self) -> std::time::Duration {
		std::time::Duration::from_nanos(self.0.wrapping_sub(other.0))
	}
}

/// A clock reading in nanoseconds, written as kio writes its `Instant`: a private constant
/// its public methods reach, `?` on a conversion's `Result::ok`, and a constructor and
/// `Duration::from_nanos` passed as functions.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Ticks(u64);

impl Ticks {
	pub(crate) const MAX: Self = Self(u64::MAX);

	/// The reading of the crate below this one.
	pub fn of(i: kio::clock::instant::Instant) -> Self {
		Self(i.0)
	}

	pub fn get(self) -> u64 {
		self.0
	}

	/// `None` past the clock's range.
	pub fn checked_add(self, d: Duration) -> Option<Self> {
		let nanos = u64::try_from(d.as_nanos()).ok()?;
		self.0.checked_add(nanos).map(Self)
	}

	pub fn saturating_add(self, d: Duration) -> Self {
		self.checked_add(d).unwrap_or(Self::MAX)
	}

	/// Zero if `earlier` is later.
	pub fn since(self, earlier: Self) -> Duration {
		self.0.checked_sub(earlier.0).map(Duration::from_nanos).unwrap_or_default()
	}
}

/// The larger of two, generically, which the fixture instantiates with its own types.
pub fn larger<T: Ord + Copy>(a: T, b: T) -> T {
	if a < b { b } else { a }
}
