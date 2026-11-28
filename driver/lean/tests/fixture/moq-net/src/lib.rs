//! A stand-in for moq-net, whose `path::Path` rust2-lean models as the string it holds.

pub mod path {
	/// Shaped as moq-net's: a representation the model replaces with its string.
	#[derive(Clone)]
	pub struct Path(&'static str);

	impl Path {
		/// Reads the representation, which only the model's own methods may.
		pub fn slice_from(&self, n: usize) -> Path {
			Path(&self.0[n..])
		}

		/// Modeled through `head_len`, which this `Path` lacks.
		pub fn head(&self, n: usize) -> Option<Path> {
			(n == 0).then(|| self.clone())
		}
	}
}
