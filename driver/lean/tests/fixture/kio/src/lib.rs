//! A stand-in for kio's `Instant`, which rust2-lean recognizes by where kio defines it.

pub mod clock {
	pub mod instant {
		#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
		pub struct Instant(pub u64);
	}
}
