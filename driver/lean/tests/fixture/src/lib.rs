//! What rust2-lean translates and refuses. `Session` holds the constructs Aeneas refused in
//! kixel-ai/kio#9, written as kio first wrote them.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Sub {
	pub opening: Option<u8>,
	pub turn: u8,
}

pub struct Session<const S: usize> {
	pub subs: [Sub; S],
	pub out: Vec<u8>,
}

impl<const S: usize> Session<S> {
	/// Matching an enum read in place through an array.
	pub fn opening(&self, s: usize) -> u8 {
		let Some(k) = self.subs[s].opening else {
			return 0;
		};
		k
	}

	/// A `while` loop reading `self.subs[s].field`.
	pub fn turns(&self) -> u16 {
		let mut total = 0u16;
		let mut s = 0;
		while s < S {
			total += self.subs[s].turn as u16;
			s += 1;
		}
		total
	}

	/// `Option<T> == Option<T>`.
	pub fn same(&self, s: usize, t: usize) -> bool {
		self.subs[s].opening == self.subs[t].opening
	}

	/// `?` on `Option`.
	pub fn next(&self, s: usize) -> Option<u8> {
		let k = self.subs[s].opening?;
		k.checked_add(1)
	}

	/// `Vec::truncate`, undoing a partial step.
	pub fn step(&mut self, s: usize) -> bool {
		let len = self.out.len();
		self.out.push(self.subs[s].turn);
		if self.subs[s].opening.is_none() {
			self.out.truncate(len);
			return false;
		}
		self.subs[s].turn += 1;
		true
	}
}

/// Outside the subset, each refused at its span.
pub mod refused {
	use std::cell::Cell;

	pub fn interior(c: &Cell<u8>) -> u8 {
		c.get()
	}

	pub fn recursive(n: u8) -> u8 {
		if n == 0 { 0 } else { recursive(n - 1) }
	}

	pub fn reborrow(v: &mut [u8]) -> &mut u8 {
		&mut v[0]
	}

	pub fn closure(x: u8) -> u8 {
		let f = |y: u8| y + x;
		f(1)
	}
}
