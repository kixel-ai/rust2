//! `str::from_utf8` from every offset of its input, and the `Utf8Error` it fails with:
//! where it stops, how many bytes are invalid there, and how it prints; and the same input
//! decoded lossily, split into chunks, and kept by a `FromUtf8Error`. Input is either
//! raw, or bytes from an alphabet of UTF-8's boundaries, so random input makes the
//! truncated, overlong, surrogate, and past-the-last-char sequences std refuses as often
//! as the ones it takes.

use crate::harness::put;

/// Lead bytes at each width's edges and past them, then continuation bytes at the edges
/// of the ranges a lead byte allows after it.
const BYTES: [u8; 32] = [
	0x00, 0x41, 0x7f, 0xc0, 0xc1, 0xc2, 0xdf, 0xe0, 0xe1, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf4, 0xf5, 0x80, 0x8f, 0x90,
	0x9f, 0xa0, 0xbf, 0x80, 0x8f, 0x90, 0x9f, 0xa0, 0xbf, 0x80, 0xbf, 0x98, 0xbb,
];

/// The raw input, or each byte's from `BYTES`.
fn input(kind: u8, data: &[u8]) -> Vec<u8> {
	match kind % 2 {
		0 => data.to_vec(),
		_ => data.iter().map(|&b| BYTES[(b % 32) as usize]).collect(),
	}
}

pub fn decodes(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let bytes = input(kind, data);
	for start in 0..=bytes.len() {
		match std::str::from_utf8(&bytes[start..]) {
			Ok(s) => {
				put(out, &format!("{s:?}"));
				out.push(s.len() as u8);
			}
			Err(e) => {
				out.push(e.valid_up_to() as u8);
				out.push(e.error_len().map_or(0xff, |n| n as u8));
				put(out, &format!("{e}|{e:>64}|{e:?}|{e:#?}"));
			}
		}
	}
	// `str`'s own `from_utf8`, a call of its own.
	match <str>::from_utf8(&bytes) {
		Ok(s) => out.push(s.len() as u8),
		Err(e) => out.push(e.valid_up_to() as u8),
	}
}

pub fn lossy(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let bytes = input(kind, data);
	for start in 0..=bytes.len() {
		let rest = &bytes[start..];
		// A `Cow` of the bytes if they are valid, else of a `String` with each invalid
		// sequence replaced.
		let text = String::from_utf8_lossy(rest);
		put(out, &format!("{text}|{text:?}"));
		out.push(text.len() as u8);
		let owned: String = text.into_owned();
		put(out, &owned);

		let mut chunks = rest.utf8_chunks();
		put(out, &format!("{chunks:?}|{chunks:#?}"));
		let mut last = None;
		while let Some(chunk) = chunks.next() {
			put(out, chunk.valid());
			out.extend_from_slice(chunk.invalid());
			out.push(chunk.invalid().len() as u8);
			put(out, &format!("{chunk:?}|{chunk:#?}"));
			// What it has left, which a clone reads on from independently.
			put(out, &format!("{chunks:?}"));
			out.push(chunks.clone().count() as u8);
			out.push((chunk.clone() == chunk) as u8);
			out.push((last.as_ref() == Some(&chunk)) as u8);
			last = Some(chunk);
		}
		put(out, &format!("{chunks:?}"));
	}

	match String::from_utf8(bytes) {
		Ok(s) => put(out, &s),
		Err(e) => {
			out.extend_from_slice(e.as_bytes());
			let u = e.utf8_error();
			out.push(u.valid_up_to() as u8);
			out.push(u.error_len().map_or(0xff, |n| n as u8));
			put(out, &format!("{u}"));
			put(out, &String::from_utf8_lossy(e.as_bytes()));
			out.push((e.clone() == e) as u8);
			let back: Vec<u8> = e.into_bytes();
			out.extend_from_slice(&back);
		}
	}
}
