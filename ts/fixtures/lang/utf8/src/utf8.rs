//! `str::from_utf8` from every offset of its input, and the `Utf8Error` it fails with:
//! where it stops, how many bytes are invalid there, and how it prints. Input is either
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

pub fn decodes(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let bytes: Vec<u8> = match kind % 2 {
		0 => data.to_vec(),
		_ => data.iter().map(|&b| BYTES[(b % 32) as usize]).collect(),
	};
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
