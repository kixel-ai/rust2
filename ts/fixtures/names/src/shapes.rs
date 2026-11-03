/// A type named as `dyns::Shape` is.
pub enum Shape {
	Dot,
	Line,
}

impl Shape {
	pub fn sides(&self) -> u8 {
		match self {
			Self::Dot => 0,
			Self::Line => 1,
		}
	}
}
