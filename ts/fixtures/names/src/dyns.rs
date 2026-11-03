/// A trait named as `shapes::Shape` is, whose object only `added` makes.
pub trait Shape {
	fn sides(&self) -> u8;
}

impl Shape for u32 {
	fn sides(&self) -> u8 {
		3
	}
}
