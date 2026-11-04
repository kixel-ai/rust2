//! Rust that translates, for the stand-in `tsc` of `spans.toml`, which fails each `return`
//! and each argument, to be reported at the Rust each comes from: a call, a name, or an
//! operator.

pub struct Meter {
	pub reads: Vec<u32>,
}

impl Meter {
	pub fn scaled(&self, by: u32, at: usize) -> u32 {
		self.reads[at] * by
	}
}

pub fn tail(a: u32, b: u32) -> u32 {
	let c = a ^ b;
	c.wrapping_mul(a) + b
}

pub fn early(a: u32) -> u32 {
	if a > 3 {
		return a.rotate_left(2);
	}
	a.count_ones()
}

pub fn method(m: &Meter, a: u32) -> u32 {
	m.scaled(a.wrapping_add(1), m.reads.len() - 1)
}

pub fn chained(v: &[u32]) -> u32 {
	v.iter().map(|x| x.leading_zeros()).max().unwrap_or(0)
}

pub fn operators(m: &Meter, a: u32, at: usize) -> u32 {
	m.scaled(a ^ 7, at / 2)
}
