//! The entry point both sides of a fixture's differential test run, and what its cases
//! share. The first two bytes pick one of the crate's `cases` and the rest is its input.

/// What a case runs on the third byte and the rest.
pub type Case = fn(u8, &[u8], &mut Vec<u8>);

/// Run one case.
pub fn run(input: &[u8]) -> Vec<u8> {
	let mut out = Vec::new();
	let [high, low, kind, data @ ..] = input else {
		return out;
	};
	let cases = crate::cases();
	let (_, case) = cases[usize::from(u16::from_be_bytes([*high, *low])) % cases.len()];
	case(*kind, data, &mut out);
	out
}

/// The first two bytes, as one big-endian number, that pick the case named `name`, which
/// `lang.ts` targets by.
pub fn op(name: &str) -> u16 {
	let Some(i) = crate::cases().iter().position(|&(n, _)| n == name) else {
		panic!("no case {name}");
	};
	// A first byte of 0xff is left to the crate's `entry`, as `shapes` prints a `Fault`.
	match u16::try_from(i) {
		Ok(op) if op >> 8 != 0xff => op,
		_ => panic!("case {name} is past what two bytes pick"),
	}
}

/// Write `s` and the byte no UTF-8 holds, which ends it.
pub fn put(out: &mut Vec<u8>, s: &str) {
	out.extend_from_slice(s.as_bytes());
	out.push(0xff);
}

/// Separators, digits, and chars where a UTF-8 or UTF-16 length changes.
pub const ALPHABET: [char; 16] = [
	'/',
	'\\',
	'a',
	'0',
	'9',
	'\u{7f}',
	'\u{80}',
	'é',
	'\u{7ff}',
	'\u{800}',
	'\u{d7ff}',
	'\u{e000}',
	'\u{ffff}',
	'\u{10000}',
	'😀',
	'\u{10ffff}',
];

/// A char of the alphabet for each byte, by its low bits.
pub fn text(data: &[u8]) -> String {
	let mut s = String::new();
	for &b in data {
		s.push(ALPHABET[(b % 16) as usize]);
	}
	s
}
