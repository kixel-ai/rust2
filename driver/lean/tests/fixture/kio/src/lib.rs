//! A crate two below the fixture, whose type a translated function of the one between names.

pub mod clock {
	pub mod instant {
		#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
		pub struct Instant(pub u64);
	}
}
