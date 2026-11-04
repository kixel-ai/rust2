//! Byte slices split by std, and the tuples, `Option`s, and `Result`s of them a body
//! takes apart, passes on, or keeps whole.

/// Why a read came up short.
#[derive(Debug)]
enum Short {
	Empty,
	Need(usize),
}

/// The first byte, then a big-endian `u16`, by `?` on `ok_or`.
fn header(data: &[u8]) -> Result<(u8, u16, &[u8]), Short> {
	let (&a, rest) = data.split_first().ok_or(Short::Empty)?;
	let (b, rest) = rest.split_first_chunk::<2>().ok_or(Short::Need(2))?;
	Ok((a, u16::from_be_bytes(*b), rest))
}

/// Two bytes, by `?` on an `Option`.
fn two(data: &[u8]) -> Option<(u8, u8, &[u8])> {
	let (&a, rest) = data.split_first()?;
	let (&b, rest) = rest.split_first()?;
	Some((a, b, rest))
}

/// `n` bytes, or how many it needed.
fn take(data: &[u8], n: usize) -> Result<(&[u8], &[u8]), Short> {
	data.split_at_checked(n).ok_or(Short::Need(n))
}

/// The byte after the first, or an error that is itself an enum, held whole.
fn second(data: &[u8], e: Option<u8>) -> Result<u8, Option<u8>> {
	let (_, rest) = data.split_at_checked(1).ok_or(e)?;
	Ok(rest.first().copied().unwrap_or(0))
}

/// A use that needs the slice itself.
#[inline(never)]
fn opaque(s: &[u8]) -> u8 {
	s.iter().fold(s.len() as u8, |a, b| (a ^ (a >> 3)).wrapping_add(*b))
}

/// An error made by a call that does something, which runs even when it isn't needed.
fn code(out: &mut Vec<u8>, n: u8) -> u8 {
	out.push(0xee);
	n
}

pub fn splits(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	match kind % 8 {
		0 => {
			// A slice reassigned from its own tail, in a loop.
			let mut rest = data;
			let mut sum = 0u8;
			while let Some((&b, tail)) = rest.split_first() {
				sum = sum.wrapping_add(b);
				rest = tail;
				if b == 0 {
					break;
				}
			}
			out.push(sum);
			out.extend_from_slice(rest);
		}
		1 => {
			// Panics past the end.
			let (a, b) = data.split_at(usize::from(data.first().copied().unwrap_or(0) % 8));
			out.push(a.len() as u8);
			out.extend_from_slice(b);
		}
		2 => match header(data) {
			Ok((a, b, rest)) => {
				out.push(a);
				out.extend_from_slice(&b.to_be_bytes());
				out.push(opaque(rest));
			}
			Err(Short::Empty) => out.push(0xf0),
			Err(Short::Need(n)) => out.push(0xf1 ^ n as u8),
		},
		3 => {
			// Made whole in a loop, which keeps it an object.
			let (head, tail) = data.split_at(data.len() / 2);
			for i in 0..3u8 {
				out.push(opaque(head) ^ i);
			}
			out.push(opaque(tail));
		}
		4 => {
			// A swap, each part assigned another's at once.
			let mut pair = data.split_at(data.len() / 3);
			for _ in 0..data.len() % 3 {
				pair = (pair.1, pair.0);
			}
			out.push(pair.0.len() as u8);
			if !pair.1.is_empty() {
				out.push(pair.1[0]);
			}
			out.extend_from_slice(pair.0);
			// Two variables swapped through a tuple, which reads each before either changes.
			let (mut a, mut b) = data.split_at(data.len() / 2);
			for _ in 0..data.len() % 3 {
				(a, b) = (b, a);
				out.push(opaque(a));
			}
			out.push(a.len() as u8);
			out.push(opaque(b));
		}
		5 => {
			let mut rest = data;
			while let Some((a, b, tail)) = two(rest) {
				out.push(a ^ b);
				rest = tail;
			}
			out.push(rest.len() as u8);
		}
		6 => {
			// Kept whole on one path, read apart on the other.
			let split = data.split_first_chunk::<4>();
			match split {
				Some((word, rest)) if word[0] & 1 == 0 => {
					out.extend_from_slice(&u32::from_be_bytes(*word).wrapping_add(3).to_be_bytes());
					out.push(rest.len() as u8);
				}
				_ => out.push(split.map_or(0xff, |(w, r)| w[3] ^ r.len() as u8)),
			}
			match take(data, 3) {
				Ok((front, back)) => {
					out.push(opaque(front));
					out.push(back.len() as u8);
				}
				Err(e) => out.push(match e {
					Short::Empty => 0,
					Short::Need(n) => n as u8,
				}),
			}
		}
		_ => {
			// The error is made before the split is looked at.
			let first = data.split_first().ok_or(code(out, 7));
			match first {
				Ok((&b, rest)) => {
					out.push(b);
					out.push(rest.len() as u8);
				}
				Err(n) => out.push(n),
			}
			// An error enum held whole, matched by its own variants.
			let err = match data.len() % 2 {
				0 => Short::Empty,
				_ => Short::Need(data.len()),
			};
			match data.split_first().ok_or(err) {
				Ok((&b, _)) => out.push(b),
				Err(Short::Empty) => out.push(0xe0),
				Err(Short::Need(n)) => out.push(0xe1 ^ n as u8),
			}
			match second(data, data.first().copied()) {
				Ok(b) => out.push(b),
				Err(Some(b)) => out.push(b ^ 0x55),
				Err(None) => out.push(0xaa),
			}
			let found: Result<(&[u8], &[u8]), u8> = data.split_at_checked(2).ok_or(1);
			if let Ok((front, back)) = found {
				out.push(front[1] ^ back.len() as u8);
			}
		}
	}
}
