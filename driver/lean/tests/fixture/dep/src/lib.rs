//! Traits from another crate, which the fixture bounds by and calls.

/// A trait whose only requirement is `Ord`, so a bound by it orders as `Ord` does.
pub trait Ordered: Ord {}

impl<T: Ord> Ordered for T {}

/// A trait sharing `core::ops::Sub`'s names, which kio's `Instant` model must not take for it.
pub trait Sub {
	fn sub(self, other: Self) -> std::time::Duration;
}

impl Sub for kio::clock::instant::Instant {
	fn sub(self, other: Self) -> std::time::Duration {
		std::time::Duration::from_nanos(self.0.wrapping_sub(other.0))
	}
}
