/// A count through `&mut self`, and the impl that forwards it through a `&mut`.
trait Tally {
	fn tally(&mut self, n: u8);
	fn total(&self) -> u32;
}

impl Tally for u32 {
	fn tally(&mut self, n: u8) {
		*self += u32::from(n);
	}

	fn total(&self) -> u32 {
		*self
	}
}

impl<T: Tally + ?Sized> Tally for &mut T {
	fn tally(&mut self, n: u8) {
		(**self).tally(n);
	}

	fn total(&self) -> u32 {
		(**self).total()
	}
}

fn tally_all<T: Tally>(t: &mut T, data: &[u8]) -> u32 {
	for &b in data {
		t.tally(b);
	}
	t.total()
}

pub fn run(data: &[u8]) -> u32 {
	let mut count = 0;
	tally_all(&mut &mut count, data)
}

/// Through more `&mut`s than `run`: its impl's instances spell their suffixes alike but for
/// the references, and sort first.
pub fn deeper(n: u32) -> u32 {
	let mut count = n;
	tally_all(&mut &mut &mut &mut count, &[1])
}
