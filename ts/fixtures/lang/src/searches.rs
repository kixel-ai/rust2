//! `str`'s searches by each kind of pattern std takes, a `char`, a `&str`, a `&[char]`, a
//! `[char; 2]`, and a closure, read from the front, the back, or both, and `str`'s text
//! iterators over whitespace and line ends of every UTF-8 width. Each iterator is cloned
//! after its first item and the clone read to its end. The closure logs each char it is
//! called with, so which chars std's searches test, and in what order, is output too.

use std::cell::RefCell;

use crate::fmt::put;
use crate::strs::text;

/// An item of a search, put in the output.
trait Item {
	fn put(self, out: &mut Vec<u8>);
}

impl Item for &str {
	fn put(self, out: &mut Vec<u8>) {
		put(out, self);
	}
}

impl Item for (usize, &str) {
	fn put(self, out: &mut Vec<u8>) {
		out.extend_from_slice(&(self.0 as u32).to_be_bytes());
		put(out, self.1);
	}
}

impl Item for (usize, char) {
	fn put(self, out: &mut Vec<u8>) {
		out.extend_from_slice(&(self.0 as u32).to_be_bytes());
		out.extend_from_slice(&u32::from(self.1).to_be_bytes());
	}
}

/// `it` read to its end from the front, then how many it gave, then a clone made after the
/// first, read to its end.
fn front<T: Item, I: Iterator<Item = T> + Clone>(mut it: I, out: &mut Vec<u8>) {
	let mut n = 0;
	let mut copy = None;
	while let Some(item) = it.next() {
		item.put(out);
		if n == 0 {
			copy = Some(it.clone());
		}
		n += 1;
	}
	out.push(n as u8);
	for item in copy.into_iter().flatten() {
		item.put(out);
	}
}

/// `it` read to its end, each step from the end the high bit of `data`'s byte picks, then
/// how many it gave, then a clone made after the first, read from the back.
fn both<T: Item, I: DoubleEndedIterator<Item = T> + Clone>(mut it: I, data: &[u8], out: &mut Vec<u8>) {
	let mut n = 0;
	let mut copy = None;
	loop {
		let back = matches!(data.get(n), Some(&b) if b & 0x80 != 0);
		let Some(item) = (if back { it.next_back() } else { it.next() }) else {
			break;
		};
		item.put(out);
		if n == 0 {
			copy = Some(it.clone());
		}
		n += 1;
	}
	out.push(n as u8);
	if let Some(copy) = copy {
		for item in copy.rev() {
			item.put(out);
		}
	}
}

fn put_option(out: &mut Vec<u8>, s: Option<&str>) {
	match s {
		Some(s) => put(out, s),
		None => out.push(0xfe),
	}
}

fn put_offset(out: &mut Vec<u8>, i: Option<usize>) {
	match i {
		Some(i) => out.extend_from_slice(&(i as u32).to_be_bytes()),
		None => out.push(0xfe),
	}
}

/// Every search std takes `$p` in, a pattern any searcher may take, by `$m`: from the front,
/// and from the back where the searcher reads from there.
macro_rules! searches {
	($m:expr, $n:expr, $s:expr, $p:expr, $out:expr) => {{
		let (s, out): (&str, &mut Vec<u8>) = ($s, $out);
		match $m {
			0 => {
				put_offset(out, s.find($p));
				put_offset(out, s.rfind($p));
			}
			1 => {
				put_option(out, s.strip_prefix($p));
				put_option(out, s.strip_suffix($p));
			}
			2 => {
				put(out, &s.replace($p, "<>"));
				put(out, &s.replacen($p, "é", $n));
			}
			3 => {
				front(s.split($p), out);
				front(s.rsplit($p), out);
			}
			4 => {
				front(s.split_terminator($p), out);
				front(s.rsplit_terminator($p), out);
			}
			5 => {
				front(s.splitn($n, $p), out);
				front(s.rsplitn($n, $p), out);
			}
			6 => front(s.split_inclusive($p), out),
			7 => {
				front(s.matches($p), out);
				front(s.rmatches($p), out);
			}
			_ => {
				front(s.match_indices($p), out);
				front(s.rmatch_indices($p), out);
			}
		}
	}};
}

/// `searches!`, and the searches std takes `$p` in from both ends at once, a pattern of
/// chars, by `$m`.
macro_rules! searches_both {
	($m:expr, $n:expr, $s:expr, $data:expr, $p:expr, $out:expr) => {{
		let (s, data, out): (&str, &[u8], &mut Vec<u8>) = ($s, $data, $out);
		match $m {
			9 => put(out, s.trim_matches($p)),
			10 => both(s.split($p), data, out),
			11 => both(s.rsplit($p), data, out),
			12 => both(s.split_terminator($p), data, out),
			13 => both(s.rsplit_terminator($p), data, out),
			14 => both(s.split_inclusive($p), data, out),
			15 => {
				both(s.matches($p), data, out);
				both(s.rmatches($p), data, out);
			}
			16 => {
				both(s.match_indices($p), data, out);
				both(s.rmatch_indices($p), data, out);
			}
			m => searches!(m, $n, s, $p, out),
		}
	}};
}

/// One search, `kind % 17` picking which, by the pattern `kind / 17 % 5` picks, of text of
/// chars of every UTF-8 width from all but the first byte. That byte picks the chars a
/// pattern of chars matches, a `&str` pattern of up to three chars from the text, empty a
/// quarter of the time, and `splitn`'s and `replacen`'s count.
pub fn searches(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let (pick, data) = data.split_first().map_or((0, data), |(&b, rest)| (b, rest));
	let s = text(data);
	let alphabet: Vec<char> = text(&[pick % 16, pick / 16]).chars().collect();
	let (c, d) = (alphabet[0], alphabet[1]);
	let n = usize::from(pick % 5);
	let m = kind % 17;
	let seen = RefCell::new(Vec::new());
	let p = |x: char| {
		seen.borrow_mut().push(x);
		x == c || x.is_ascii_digit()
	};
	match kind / 17 % 5 {
		0 => searches_both!(m, n, &s, data, c, out),
		1 => {
			// A `&str`, which std searches from one end only.
			let needle: String = s
				.chars()
				.skip(usize::from(pick / 64))
				.take(usize::from(pick / 16 % 4))
				.collect();
			searches!(m % 9, n, &s, needle.as_str(), out)
		}
		2 => searches_both!(m, n, &s, data, &[c, d][..], out),
		3 => searches_both!(m, n, &s, data, [c, d], out),
		_ => searches_both!(m, n, &s, data, p, out),
	}
	for &x in seen.borrow().iter() {
		out.extend_from_slice(&u32::from(x).to_be_bytes());
	}
}

/// Line ends, whitespace of each UTF-8 width, and chars that are neither, `\u{200b}` and
/// `\u{feff}` among them, which JS's `\s` counts as whitespace.
const SPACES: [char; 16] = [
	'\n', '\r', 'a', ' ', '\t', '\u{b}', '\u{85}', '\u{a0}', '\u{1680}', '\u{200a}', '\u{200b}', '\u{2028}',
	'\u{3000}', '\u{feff}', 'é', '😀',
];

/// `str`'s text iterators and trims, `kind % 4` picking which, of text of `SPACES` by the
/// low bits of each byte, read from the end each byte's high bit picks.
pub fn texts(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let s: String = data.iter().map(|&b| SPACES[usize::from(b % 16)]).collect();
	match kind % 4 {
		0 => both(s.char_indices(), data, out),
		1 => both(s.lines(), data, out),
		2 => both(s.split_whitespace(), data, out),
		_ => {
			put(out, s.trim());
			put(out, s.trim_start());
			put(out, s.trim_end());
			let spaces: Vec<bool> = s.chars().map(char::is_whitespace).collect();
			for space in spaces {
				out.push(space as u8);
			}
		}
	}
}
