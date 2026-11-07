//! Byte storage handed between Go code and the translation every way a root function's
//! signature can pass it, which `tests/hand/hand_test.go` holds onto from Go, and `run`,
//! each way the translation grows, copies, and shrinks a `Vec<u8>`.

/// Bytes Go code holds only through methods.
pub struct Buf {
	data: Vec<u8>,
}

impl Buf {
	/// An empty buffer.
	pub fn new() -> Self {
		Self { data: Vec::new() }
	}

	/// Append `n` bytes of `n`.
	pub fn fill(&mut self, n: u8) {
		append(&mut self.data, n);
	}

	/// What it holds.
	pub fn bytes(&self) -> &[u8] {
		&self.data
	}
}

impl Default for Buf {
	fn default() -> Self {
		Self::new()
	}
}

/// Bytes Go code reads directly.
pub struct Open {
	pub data: Vec<u8>,
}

/// One of two holders of bytes.
pub enum Either {
	Left(Vec<u8>),
	Right(u8, Vec<u8>),
	Neither,
}

/// Append `n` bytes of `n`.
pub fn append(v: &mut Vec<u8>, n: u8) {
	for _ in 0..n {
		v.push(n);
	}
}

/// Append `n` bytes of `n`, then panic if `n` is odd.
pub fn append_or_panic(v: &mut Vec<u8>, n: u8) {
	append(v, n);
	assert!(n.is_multiple_of(2), "odd");
}

/// `n` bytes of `n`, with the room of `n` more.
pub fn fill(n: u8) -> Vec<u8> {
	let mut v = Vec::with_capacity(2 * n as usize);
	append(&mut v, n);
	v
}

/// `v` with `n` bytes of `n` appended.
pub fn grow(mut v: Vec<u8>, n: u8) -> Vec<u8> {
	append(&mut v, n);
	v
}

/// `b` with `n` bytes of `n` appended to what it holds.
pub fn boxed(mut b: Box<Open>, n: u8) -> Box<Open> {
	append(&mut b.data, n);
	b
}

/// Append `n` bytes of `n` to what `b` holds.
pub fn reboxed(b: &mut Box<Open>, n: u8) {
	append(&mut b.data, n);
}

/// Clear `v`, then append `n` bytes of `n`.
pub fn refill(v: &mut Vec<u8>, n: u8) {
	v.clear();
	append(v, n);
}

/// Replace `v`'s first buffer with `n` bytes of `n`, outgrow `v`, then that buffer: its
/// length.
pub fn regrow(v: &mut Vec<Vec<u8>>, n: u8) -> usize {
	v[0] = fill(n);
	for _ in 0..64 {
		v.push(Vec::new());
	}
	append(&mut v[0], 200);
	v[0].len()
}

/// `n` buffers, the `i`th of `i` bytes.
pub fn nested(n: u8) -> Vec<Vec<u8>> {
	let mut out = Vec::new();
	for i in 0..n {
		out.push(fill(i));
	}
	out
}

/// `n` bytes of `n`, if `n` is not zero.
pub fn maybe(n: u8) -> Option<Vec<u8>> {
	match n {
		0 => None,
		n => Some(fill(n)),
	}
}

/// Append `n` bytes of `n` to `v`, giving what it then holds.
pub fn view(v: &mut Vec<u8>, n: u8) -> &[u8] {
	append(v, n);
	v
}

/// An iterator over `xs`.
pub fn iter(xs: &[u8]) -> std::slice::Iter<'_, u8> {
	xs.iter()
}

/// An iterator over `v`'s buffers.
pub fn iter_nested(v: &[Vec<u8>]) -> std::slice::Iter<'_, Vec<u8>> {
	v.iter()
}

/// Push `n` bytes of `n` onto `v`, then pop and outgrow them, leaving the slot vacated.
pub fn vacate(v: &mut Vec<Vec<u8>>, n: u8) -> usize {
	v.push(fill(n));
	let mut last = v.pop().unwrap_or_default();
	append(&mut last, 200);
	last.len()
}

/// Switch `e` to the other side, its bytes with `n` bytes of `n` appended.
pub fn flip(e: &mut Either, n: u8) {
	*e = match std::mem::replace(e, Either::Neither) {
		Either::Left(v) => Either::Right(n, grow(v, n)),
		Either::Right(_, v) => Either::Left(grow(v, n)),
		Either::Neither => Either::Left(fill(n)),
	};
}

/// Each way the translation makes, grows, copies, and shrinks a `Vec<u8>`, its sizes
/// from the input's bytes: what each ends up holding.
pub fn run(input: &[u8]) -> Vec<u8> {
	let mut out = Vec::new();
	let mut held: Vec<Vec<u8>> = Vec::new();
	for (i, &b) in input.iter().enumerate() {
		let n = b as usize * (i + 1);
		let mut v = match b % 4 {
			0 => Vec::new(),
			1 => Vec::with_capacity(n),
			2 => input.to_vec(),
			_ => match held.last() {
				Some(last) => last.clone(),
				None => Vec::new(),
			},
		};
		for k in 0..n {
			v.push(k as u8);
		}
		v.extend_from_slice(input);
		match b % 3 {
			0 => v.truncate(n / 2),
			1 => v.clear(),
			_ => {
				v.pop();
			}
		}
		v.extend_from_slice(&out);
		out.extend_from_slice(&(v.len() as u32).to_be_bytes());
		let mut sum = 0u8;
		for &x in &v {
			sum = sum.wrapping_mul(31).wrapping_add(x);
		}
		out.push(sum);
		held.push(v);
		out.truncate(out.len().min(4096));
	}
	for v in &held {
		out.extend_from_slice(&v[..v.len().min(3)]);
	}
	out
}
