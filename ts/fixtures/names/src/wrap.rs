use std::fmt;

/// A generic type, with a method generic over a closure.
#[derive(Debug)]
pub struct Wrap<T>(T);

impl<T> Wrap<T> {
	pub fn new(value: T) -> Self {
		Self(value)
	}

	pub fn into_inner(self) -> T {
		self.0
	}

	pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Wrap<U> {
		Wrap(f(self.0))
	}
}

impl<T: fmt::Display> fmt::Display for Wrap<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "[{}]", self.0)
	}
}

/// Printed by `Display` in `run`, and by `Debug` only in `added`.
#[derive(Clone, Copy, Debug)]
pub struct Pair(pub u8, pub u8);

impl fmt::Display for Pair {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}-{}", self.0, self.1)
	}
}
