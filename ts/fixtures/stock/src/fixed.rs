//! arrayvec's `ArrayString`, which translates from its source: text pushed into a fixed
//! buffer of bytes through `str::as_ptr` and `ptr::copy_nonoverlapping`, and read back
//! through `slice::from_raw_parts`, as moq-net's `Path::concat` joins a long path.

use arrayvec::ArrayString;

/// A char of each UTF-8 width, and the separator a path joins on.
const CHARS: [char; 5] = ['/', 'a', 'é', '\u{800}', '😀'];

/// Each byte an edit of an `ArrayString` of 12 bytes, whose high bits pick a char or a
/// length, writing what it returns, then the text.
pub fn strings(data: &[u8], out: &mut Vec<u8>) {
	let mut s = ArrayString::<12>::new();
	for &b in data {
		let c = CHARS[usize::from(b >> 3) % CHARS.len()];
		match b % 8 {
			0 => out.push(u8::from(s.try_push(c).is_ok())),
			1 => {
				// A run of the char, which may not fit.
				let run: String = std::iter::repeat_n(c, usize::from(b >> 5)).collect();
				match s.try_push_str(&run) {
					Ok(()) => out.push(0),
					Err(e) => out.extend_from_slice(e.element().as_bytes()),
				}
			}
			2 => out.extend_from_slice(&s.pop().map_or(0, u32::from).to_le_bytes()),
			3 => {
				// To the char boundary the high bits pick.
				let ends: Vec<usize> = s.char_indices().map(|(i, _)| i).chain([s.len()]).collect();
				s.truncate(ends[usize::from(b >> 3) % ends.len()]);
			}
			4 if !s.is_empty() => out.extend_from_slice(&u32::from(s.remove(0)).to_le_bytes()),
			5 => match ArrayString::<12>::from(s.as_str()) {
				Ok(copy) => s = copy,
				Err(e) => out.push(e.element().len() as u8),
			},
			6 if s.remaining_capacity() >= c.len_utf8() => s.push_str(c.encode_utf8(&mut [0; 4])),
			_ => s.clear(),
		}
		out.extend([s.len() as u8, s.remaining_capacity() as u8, u8::from(s.is_full())]);
	}
	out.extend_from_slice(s.as_bytes());
}
