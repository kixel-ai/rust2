//! Enums Go holds as their parts, a part of over four words behind a pointer: each case's
//! output changes if a callee writes what its caller still reads, a part is read through
//! a pointer something else changes, or a variable keeps a value it should lose.

/// Over four words, so a part of its own behind a pointer.
type Five = (u64, u64, u64, u64, u64);

#[derive(Clone, Copy)]
pub enum Big {
	Wide(Five),
	Narrow(u8),
}

/// An error of over four words, as `DecodeError` is.
#[derive(Debug)]
enum Error {
	Short,
	Text(Vec<u8>, usize, Option<u8>),
	Code(u16),
}

impl From<u16> for Error {
	fn from(code: u16) -> Self {
		Error::Code(code)
	}
}

fn wide(x: u8) -> Big {
	match x % 3 {
		0 => Big::Narrow(x),
		_ => Big::Wide((x as u64, 2, 3, 4, x as u64 * 5)),
	}
}

fn sum(b: Big) -> u64 {
	match b {
		Big::Wide((a, b, c, d, e)) => a + b + c + d + e,
		Big::Narrow(n) => n as u64,
	}
}

/// Changes the value it owns, which its caller's copy must not see.
fn changed(mut b: Big) -> u64 {
	if let Big::Wide(ref mut five) = b {
		five.0 += 100;
	}
	sum(b)
}

/// Overwrites through `r` what `b` was copied from.
fn overwrite(b: Big, r: &mut Big) -> u64 {
	*r = Big::Narrow(7);
	sum(b)
}

/// Fields whose names joined to their variants' are one: `A_B` and `c`, `A` and `B_c`.
#[allow(non_camel_case_types, non_snake_case, reason = "the names that collide")]
enum Joined {
	A_B { c: Five },
	A { B_c: u64 },
}

fn joined(x: u8) -> Joined {
	match x % 2 {
		0 => Joined::A { B_c: x as u64 },
		_ => Joined::A_B {
			c: (1, 2, 3, 4, x as u64),
		},
	}
}

#[inline(never)]
fn unjoined(j: Joined) -> u64 {
	match j {
		Joined::A_B { c } => c.0 + c.4,
		Joined::A { B_c } => B_c * 3,
	}
}

/// Builds its result through a reference to it.
fn filled(x: u8) -> Big {
	let mut r = Big::Narrow(0);
	fill(&mut r, x);
	r
}

#[inline(never)]
fn fill(r: &mut Big, x: u8) {
	if x.is_multiple_of(2) {
		*r = Big::Wide((1, x as u64, 1, x as u64, 1));
	}
}

/// Its parts rotated: each written from the argument's. A root, which Go callers call
/// with its enums whole.
pub fn rotate(b: Big) -> Big {
	match b {
		Big::Wide((a, b, c, d, e)) => Big::Wide((e, a, b, c, d)),
		n => n,
	}
}

fn by_ref(b: Big) -> u64 {
	let r = &b;
	sum(*r)
}

fn parse(data: &[u8]) -> Result<Five, Error> {
	let Some((&first, rest)) = data.split_first() else {
		return Err(Error::Short);
	};
	match first % 4 {
		0 => Err(Error::Text(rest.to_vec(), rest.len(), rest.first().copied())),
		1 => Err(Error::from(first as u16 * 3)),
		_ => Ok((first as u64, rest.len() as u64, 0, 0, 1)),
	}
}

/// Two parses through `?`, the error handed on through each.
fn parsed(data: &[u8]) -> Result<(Five, Five), Error> {
	let a = parse(data)?;
	let b = parse(data.get(1..).unwrap_or_default())?;
	Ok((a, b))
}

fn code(e: u8) -> Result<u8, u16> {
	match e % 2 {
		0 => Ok(e),
		_ => Err(e as u16),
	}
}

/// A `u16` error converted by `?`.
fn converted(e: u8) -> Result<Big, Error> {
	Ok(Big::Narrow(code(e)?))
}

fn put_five(out: &mut Vec<u8>, f: Five) {
	out.extend_from_slice(&f.0.to_le_bytes());
	out.extend_from_slice(&f.1.to_le_bytes());
	out.extend_from_slice(&f.2.to_le_bytes());
	out.extend_from_slice(&f.3.to_le_bytes());
	out.extend_from_slice(&f.4.to_le_bytes());
}

fn put_error(out: &mut Vec<u8>, e: &Error) {
	match e {
		Error::Short => out.push(0xf0),
		Error::Text(v, n, first) => {
			out.push(0xf1);
			out.extend_from_slice(v);
			out.push(*n as u8);
			out.push(first.unwrap_or(0xff));
		}
		Error::Code(c) => out.extend_from_slice(&c.to_le_bytes()),
	}
}

/// Run the case the first byte picks on the rest.
pub fn run(input: &[u8]) -> Vec<u8> {
	let Some((&pick, data)) = input.split_first() else {
		return Vec::new();
	};
	let x = data.first().copied().unwrap_or(0);
	let y = data.get(1).copied().unwrap_or(0);
	let mut out = Vec::new();
	match pick % 10 {
		0 => {
			let b = wide(x);
			let c = changed(b);
			put_five(&mut out, (c, sum(b), 0, 0, 0));
		}
		1 => {
			let mut b = wide(x);
			let s = overwrite(b, &mut b);
			let mut c = wide(y);
			let r = &mut c;
			let t = overwrite(*r, r);
			put_five(&mut out, (s, sum(b), t, sum(c), 0));
		}
		2 => put_five(&mut out, (sum(filled(x)), unjoined(joined(y)), 0, 0, 0)),
		3 => {
			let mut b = wide(x);
			for _ in 0..y % 7 {
				b = rotate(b);
			}
			if let Big::Wide(f) = b {
				put_five(&mut out, f);
			}
		}
		4 => {
			let f: fn(u8) -> Big = wide;
			put_five(&mut out, (sum(f(x)), by_ref(f(y)), 0, 0, 0));
		}
		5 => match parsed(data) {
			Ok((a, b)) => {
				put_five(&mut out, a);
				put_five(&mut out, b);
			}
			Err(e) => put_error(&mut out, &e),
		},
		6 => {
			// Into memory: a box's and an array's elements.
			let mut boxed = Box::new(wide(x));
			let before = sum(*boxed);
			*boxed = wide(y);
			let mut v = [wide(x), *boxed];
			v[0] = wide(x.wrapping_add(y));
			let total = sum(v[0]) + sum(v[1]);
			put_five(&mut out, (before, sum(*boxed), total, 0, 0));
		}
		7 => {
			// The last wide value seen, kept across iterations.
			let mut last = Big::Narrow(0);
			let mut i = 0;
			while i < data.len() {
				if data[i] % 3 != 0 {
					last = wide(data[i]);
				}
				i += 1;
			}
			put_five(&mut out, (sum(last), 0, 0, 0, 0));
		}
		8 => match converted(x) {
			Ok(b) => put_five(&mut out, (sum(b), 0, 0, 0, 0)),
			Err(e) => put_error(&mut out, &e),
		},
		_ => {
			let nested: Option<Big> = (x % 2 == 0).then(|| wide(y));
			let s = nested.map(changed);
			put_five(&mut out, (s.unwrap_or(9), nested.map_or(1, sum), 0, 0, 0));
		}
	}
	out
}
