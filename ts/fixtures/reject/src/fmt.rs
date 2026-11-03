//! Formatting the runtime doesn't do as std does.

/// A pointer's address, which JS has no counterpart of.
pub fn pointer(n: u8) -> String {
	format!("{:p}", &n)
}

/// A placeholder written to the crate's own `fmt::Write`, which sees each call std makes.
pub fn written(w: &mut Sink, n: u8) {
	use std::fmt::Write;
	let _ = write!(w, "{n}");
}

pub struct Sink(u8);

impl std::fmt::Write for Sink {
	fn write_str(&mut self, s: &str) -> std::fmt::Result {
		self.0 = self.0.wrapping_add(s.len() as u8);
		Ok(())
	}
}
