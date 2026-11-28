//! A trait from another crate, which the fixture bounds by.

/// A trait whose only requirement is `Ord`, so a bound by it orders as `Ord` does.
pub trait Ordered: Ord {}

impl<T: Ord> Ordered for T {}
