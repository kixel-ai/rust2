//! References to arrays of bytes, `&[u8; N]`, as a split makes them: read, copied, written
//! through, and kept in fields, tuples, results, and closures, and passed across calls,
//! while the bytes they share change.

/// A header split off the front of its input, keeping both parts.
struct Header<'a> {
	tag: &'a [u8; 2],
	rest: &'a [u8],
}

fn header(data: &[u8]) -> Option<Header<'_>> {
	let (tag, rest) = data.split_first_chunk::<2>()?;
	Some(Header { tag, rest })
}

fn take4(data: &[u8]) -> Result<(&[u8; 4], &[u8]), u8> {
	data.split_first_chunk::<4>().ok_or(data.len() as u8)
}

fn word(chunk: &[u8; 4]) -> u32 {
	u32::from_be_bytes(*chunk)
}

fn pick<'a>(a: &'a [u8; 2], b: &'a [u8; 2], first: bool) -> &'a [u8; 2] {
	match first {
		true => a,
		false => b,
	}
}

fn reader(chunk: &[u8; 2]) -> impl Fn(usize) -> u8 + '_ {
	move |i| chunk[i % 2]
}

fn bump(chunk: &mut [u8; 2]) {
	chunk[0] = chunk[0].wrapping_add(1);
	chunk[1] ^= 0xff;
}

fn ends(chunk: &[u8; 4]) -> (u8, &[u8; 2], u8) {
	let [first, middle @ .., last] = chunk;
	(*first, middle, *last)
}

pub fn chunks(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	match kind % 6 {
		0 => {
			// Read in place, through a field, a result, and calls.
			if let Some(h) = header(data) {
				out.extend_from_slice(h.tag);
				out.push(h.tag[1]);
				out.push(h.rest.len() as u8);
			}
			match take4(data) {
				Ok((chunk, rest)) => {
					out.extend_from_slice(&word(chunk).to_be_bytes());
					out.extend_from_slice(&i16::from_be_bytes([chunk[1], chunk[2]]).to_be_bytes());
					let (first, middle, last) = ends(chunk);
					out.extend_from_slice(&[first, last, middle[0], middle[1], middle.len() as u8]);
					out.extend_from_slice(rest);
				}
				Err(len) => out.push(len),
			}
		}
		1 => {
			// Copied out, then the bytes it was copied from change.
			let mut buf = [0u8; 4];
			for (i, b) in data.iter().take(4).enumerate() {
				buf[i] = *b;
			}
			let held = &mut buf;
			let before = *held;
			held[0] = held[0].wrapping_add(7);
			let after = *held;
			*held = [9, 8, 7, 6];
			out.extend_from_slice(&before);
			out.extend_from_slice(&after);
			out.extend_from_slice(&buf);
			let mut pair = [data.len() as u8, 3];
			bump(&mut pair);
			bump(&mut pair);
			out.extend_from_slice(&pair);
		}
		2 => {
			// Kept in a tuple and a closure, and chosen across a call.
			let Some((a, rest)) = data.split_first_chunk::<2>() else {
				return;
			};
			let b = rest.split_first_chunk::<2>().map_or(a, |(b, _)| b);
			let kept = (a, b, rest);
			let chosen = pick(kept.0, kept.1, data.len() % 2 == 0);
			let read = reader(chosen);
			out.extend_from_slice(&[read(0), read(1), read(3)]);
			let both = |i: usize| a[i] ^ b[i];
			out.extend_from_slice(&[both(0), both(1)]);
			out.extend_from_slice(kept.1);
			out.push(kept.2.len() as u8);
		}
		3 => {
			// Byte strings, and formatted.
			let lit: &[u8; 3] = b"abc";
			let chosen = match data.first() {
				Some(b) if b % 2 == 0 => lit,
				_ => b"xyz",
			};
			out.extend_from_slice(chosen);
			out.push(chosen[1]);
			out.extend_from_slice(format!("{chosen:?} {:?}", *lit).as_bytes());
			if let Some((chunk, _)) = data.split_first_chunk::<3>() {
				out.extend_from_slice(format!("{chunk:?} {:?}", *chunk).as_bytes());
			}
		}
		4 => {
			// Elements of a vector of arrays, by reference.
			let mut pairs: Vec<[u8; 2]> = data.iter().map(|&b| [b, b.wrapping_add(3)]).collect();
			if pairs.len() >= 2 {
				let first = pairs[0];
				pairs[1][0] = 0xaa;
				let held = &mut pairs[0];
				held[1] = 0x55;
				out.extend_from_slice(&first);
				out.extend_from_slice(&pairs[0]);
				out.extend_from_slice(&pairs[1]);
			}
			for pair in pairs.iter() {
				out.push(pair[0] ^ pair[1]);
			}
			for pair in &mut pairs {
				pair[0] = pair[1];
			}
			let copies: Vec<[u8; 2]> = pairs.iter().copied().collect();
			out.extend(pairs.iter().map(|p| p[0]).chain(copies.iter().map(|p| p[1])));
		}
		_ => {
			// Split repeatedly, each chunk read where it is.
			let mut rest = data;
			let mut sum = 0u32;
			while let Some((chunk, tail)) = rest.split_first_chunk::<3>() {
				sum = (sum << 5 | sum >> 27).wrapping_add(u32::from(u16::from_be_bytes([chunk[0], chunk[2]])));
				out.push(chunk[1]);
				rest = tail;
			}
			out.extend_from_slice(&sum.to_be_bytes());
			out.extend_from_slice(rest);
		}
	}
}
