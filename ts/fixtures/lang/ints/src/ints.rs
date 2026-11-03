//! Every integer type's arithmetic, one operation per case so a panic hides nothing, and
//! its parsing.

use std::convert::Infallible;
use std::num::IntErrorKind;

/// Take an integer's big-endian bytes from the front of `data`, zero-padded.
macro_rules! take {
	($t:ty, $data:expr) => {{
		let data: &[u8] = $data;
		let mut buf = [0u8; core::mem::size_of::<$t>()];
		let n = data.len().min(buf.len());
		buf[..n].copy_from_slice(&data[..n]);
		(<$t>::from_be_bytes(buf), &data[n..])
	}};
}

/// `a.div_ceil(b)` of an unsigned type, which a signed one lacks.
macro_rules! div_ceil {
	(unsigned, $t:ty, $a:expr, $b:expr) => {
		Some($a.div_ceil($b))
	};
	(signed, $t:ty, $a:expr, $b:expr) => {
		None::<$t>
	};
}

/// `a.is_multiple_of(b)` of an unsigned type, which a signed one lacks.
macro_rules! is_multiple_of {
	(unsigned, $a:expr, $b:expr) => {
		Some($a.is_multiple_of($b))
	};
	(signed, $a:expr, $b:expr) => {
		None::<bool>
	};
}

macro_rules! ops {
	($name:ident, $t:ty, $wide:ty, $sign:ident) => {
		/// Apply operation `op` to operands taken from `data`.
		pub fn $name(op: u8, data: &[u8], out: &mut Vec<u8>) {
			let (a, data) = take!($t, data);
			let (b, data) = take!($t, data);
			// A third operand, from the bytes the shift is read from.
			let (c, _) = take!($t, data);
			let shift = data.first().copied().unwrap_or(0) as u32 % (2 * <$t>::BITS);
			let put = |out: &mut Vec<u8>, v: $t| {
				out.extend_from_slice(&(v as $wide).to_be_bytes());
				// A wide value equals its bytes read back only in its one form.
				out.push((<$t>::from_be_bytes(v.to_be_bytes()) == v) as u8);
			};
			let put_opt = |out: &mut Vec<u8>, v: Option<$t>| match v {
				Some(v) => {
					out.push(1);
					out.extend_from_slice(&(v as $wide).to_be_bytes());
				}
				None => out.push(0),
			};
			match op % 32 {
				0 => put(out, a + b),
				1 => put(out, a - b),
				2 => put(out, a * b),
				3 => put(out, a / b),
				4 => put(out, a % b),
				5 => put(out, a & b),
				6 => put(out, a | b),
				7 => put(out, a ^ b),
				8 => put(out, a << shift),
				9 => put(out, a >> shift),
				10 => put(out, !a),
				11 => put_opt(out, a.checked_add(b)),
				12 => put_opt(out, a.checked_sub(b)),
				13 => put_opt(out, a.checked_mul(b)),
				14 => put(out, a.saturating_add(b)),
				15 => put(out, a.saturating_sub(b)),
				16 => put(out, a.wrapping_add(b)),
				17 => put(out, a.wrapping_sub(b)),
				18 => out.extend_from_slice(&a.leading_ones().to_be_bytes()),
				19 => out.extend_from_slice(&a.leading_zeros().to_be_bytes()),
				20 => put(out, a.max(b)),
				21 => put(out, a.min(b)),
				22 => {
					out.push((a == b) as u8);
					out.push((a < b) as u8);
					out.push((a >= b) as u8);
					// Against a literal, which a wide value equals only in one form.
					out.push((a.wrapping_sub(b) == 0) as u8);
				}
				23 => {
					// Products nothing reads still overflow, of a sum that does too.
					let x = a + b;
					let _ = x * b;
					let _ = (x - b) * a;
					out.push(1);
				}
				24 => put(out, a.saturating_mul(b)),
				25 => put(out, a.wrapping_mul(b)),
				// Panics unless `b <= c`.
				26 => put(out, a.clamp(b, c)),
				27 => {
					if let Some(v) = div_ceil!($sign, $t, a, b) {
						put(out, v);
					}
				}
				28 => {
					if let Some(m) = is_multiple_of!($sign, a, b) {
						out.push(m as u8);
					}
				}
				// Each byte order and back, the big-endian bytes read the other way, and every
				// array again after a read, which must leave it as it was.
				29 => {
					let (le, ne, be) = (a.to_le_bytes(), a.to_ne_bytes(), a.to_be_bytes());
					put(out, <$t>::from_le_bytes(le));
					put(out, <$t>::from_ne_bytes(ne));
					put(out, <$t>::from_le_bytes(be));
					put(out, <$t>::from_ne_bytes(be));
					out.extend_from_slice(&le);
					out.extend_from_slice(&ne);
					out.extend_from_slice(&be);
				}
				// Each width's edges, where a count is 0 or every bit, and through a `NonZero`.
				30 => {
					for v in [0, 1, <$t>::MAX, <$t>::MIN, a, a << shift % <$t>::BITS] {
						out.extend_from_slice(&v.trailing_zeros().to_be_bytes());
						match core::num::NonZero::new(v) {
							Some(n) => {
								out.extend_from_slice(&n.trailing_zeros().to_be_bytes());
								out.extend_from_slice(&n.leading_zeros().to_be_bytes());
							}
							None => out.push(0xff),
						}
					}
				}
				_ => {
					// Compound assignment, with constants folded beside variables.
					let mut x = a;
					x += b;
					x -= 1;
					x *= 2;
					x >>= 1;
					x <<= 1;
					x |= 1 << 3;
					x ^= b;
					x &= !(1 << 2);
					// By a constant, whose overflow check a signed type folds.
					x %= 7;
					x /= 3;
					put(out, x);
				}
			}
		}
	};
}

ops!(u8_op, u8, u128, unsigned);
ops!(u16_op, u16, u128, unsigned);
ops!(u32_op, u32, u128, unsigned);
ops!(u64_op, u64, u128, unsigned);
ops!(u128_op, u128, u128, unsigned);
ops!(usize_op, usize, u128, unsigned);
ops!(i8_op, i8, i128, signed);
ops!(i16_op, i16, i128, signed);
ops!(i32_op, i32, i128, signed);
ops!(i64_op, i64, i128, signed);
ops!(i128_op, i128, i128, signed);
ops!(isize_op, isize, i128, signed);

macro_rules! neg {
	($name:ident, $t:ty) => {
		/// Negate, which overflows at the minimum.
		pub fn $name(data: &[u8], out: &mut Vec<u8>) {
			let (a, _) = take!($t, data);
			out.extend_from_slice(&((-a) as i128).to_be_bytes());
		}
	};
}

neg!(i8_neg, i8);
neg!(i16_neg, i16);
neg!(i32_neg, i32);
neg!(i64_neg, i64);
neg!(i128_neg, i128);
neg!(isize_neg, isize);

/// A cast's bytes, then its text, which shows the sign the result reads with.
macro_rules! put_cast {
	($out:expr, $v:expr) => {{
		let v = $v;
		$out.extend_from_slice(&v.to_be_bytes());
		$out.extend_from_slice(v.to_string().as_bytes());
	}};
}

macro_rules! cast_to {
	($v:expr, $to:expr, $out:expr) => {{
		let v = $v;
		let out: &mut Vec<u8> = $out;
		match $to % 12 {
			0 => put_cast!(out, v as u8),
			1 => put_cast!(out, v as u16),
			2 => put_cast!(out, v as u32),
			3 => put_cast!(out, v as u64),
			4 => put_cast!(out, v as u128),
			5 => put_cast!(out, v as usize),
			6 => put_cast!(out, v as i8),
			7 => put_cast!(out, v as i16),
			8 => put_cast!(out, v as i32),
			9 => put_cast!(out, v as i64),
			10 => put_cast!(out, v as i128),
			_ => put_cast!(out, v as isize),
		}
	}};
}

/// Cast a value of type `from` to type `to`, both indexes into the twelve types.
pub fn cast(from: u8, to: u8, data: &[u8], out: &mut Vec<u8>) {
	match from % 12 {
		0 => cast_to!(take!(u8, data).0, to, out),
		1 => cast_to!(take!(u16, data).0, to, out),
		2 => cast_to!(take!(u32, data).0, to, out),
		3 => cast_to!(take!(u64, data).0, to, out),
		4 => cast_to!(take!(u128, data).0, to, out),
		5 => cast_to!(take!(usize, data).0, to, out),
		6 => cast_to!(take!(i8, data).0, to, out),
		7 => cast_to!(take!(i16, data).0, to, out),
		8 => cast_to!(take!(i32, data).0, to, out),
		9 => cast_to!(take!(i64, data).0, to, out),
		10 => cast_to!(take!(i128, data).0, to, out),
		_ => cast_to!(take!(isize, data).0, to, out),
	}
}

/// Conversions that check, and ones that cannot fail.
pub fn convert(data: &[u8], out: &mut Vec<u8>) {
	let (wide, data) = take!(u64, data);
	let (signed, _) = take!(i32, data);
	out.push(u8::try_from(wide).is_ok() as u8);
	match u32::try_from(wide) {
		Ok(v) => out.extend_from_slice(&v.to_be_bytes()),
		Err(_) => out.push(0xff),
	}
	match usize::try_from(signed) {
		Ok(v) => out.extend_from_slice(&v.to_be_bytes()),
		Err(_) => out.push(0xfe),
	}
	match i8::try_from(signed) {
		Ok(v) => out.extend_from_slice(&v.to_be_bytes()),
		Err(_) => out.push(0xfd),
	}
	let tripled: Result<u64, _> = (wide as u128 * 3).try_into();
	out.extend_from_slice(&tripled.unwrap_or(u64::MAX).to_be_bytes());
	let narrowed: Result<u16, _> = signed.try_into();
	out.extend_from_slice(&narrowed.unwrap_or(0xffff).to_be_bytes());
	let widened: u64 = u64::from(wide as u16);
	let into: i64 = signed.into();
	out.extend_from_slice(&widened.to_be_bytes());
	out.extend_from_slice(&into.to_be_bytes());
	out.push(true as u8 + false as u8);
	// Arrays written in place, with constant and sign bytes, and one behind a reference.
	let [a, b, c, d] = (signed as u32).to_be_bytes();
	out.extend_from_slice(&i8::from_be_bytes([a]).to_be_bytes());
	out.extend_from_slice(&u16::from_be_bytes([a, b]).to_be_bytes());
	out.extend_from_slice(&i16::from_be_bytes([a, b]).to_be_bytes());
	out.extend_from_slice(&u32::from_be_bytes([a, b, c, d]).to_be_bytes());
	out.extend_from_slice(&u32::from_be_bytes([0, b & 0x3f, c, d]).to_be_bytes());
	out.extend_from_slice(&i32::from_be_bytes([a, 0, c, d]).to_be_bytes());
	out.extend_from_slice(&u32::from_be_bytes([0, 0, 0, 0]).to_be_bytes());
	out.extend_from_slice(&u64::from_be_bytes([d, c, b, a, a, b, c, d]).to_be_bytes());
	let bytes = &[d, c, b, a];
	out.extend_from_slice(&i32::from_be_bytes(*bytes).to_be_bytes());
	// Halves joined back, which the runtime does in one call, of each width it takes.
	out.extend_from_slice(&from_halves((wide >> 32) as u32, wide as u32).to_be_bytes());
	let narrow = ((wide >> 40) as u8 as u64) << 32 | (wide as u16) as u64;
	out.extend_from_slice(&narrow.to_be_bytes());
	// Named constants past `i32::MAX`, which JS's 32-bit operators read as negative.
	out.extend_from_slice(&(HIGH >> 1).to_be_bytes());
	out.extend_from_slice(&(TOP >> 4).to_be_bytes());
}

const HIGH: u32 = u32::MAX;
const TOP: usize = 1 << 31;

/// A `u64` from its high and low 32 bits, as moq-net's varints join them.
fn from_halves(hi: u32, lo: u32) -> u64 {
	((hi as u64) << 32) | lo as u64
}

/// The digits of every radix, in order.
const DIGITS: &[u8; 36] = b"0123456789abcdefghijklmnopqrstuvwxyz";

/// `n` in `radix`, without a sign.
fn digits(mut n: u128, radix: u32) -> String {
	let mut out = Vec::new();
	loop {
		out.push(DIGITS[(n % radix as u128) as usize]);
		n /= radix as u128;
		if n == 0 {
			break;
		}
	}
	out.reverse();
	String::from_utf8(out).unwrap()
}

/// The digits of one more than `s`, in `radix`.
fn succ(s: &str, radix: u32) -> String {
	let mut out = s.as_bytes().to_vec();
	for d in out.iter_mut().rev() {
		let next = DIGITS.iter().position(|c| c == d).unwrap() + 1;
		if next < radix as usize {
			*d = DIGITS[next];
			return String::from_utf8(out).unwrap();
		}
		*d = b'0';
	}
	out.insert(0, b'1');
	String::from_utf8(out).unwrap()
}

/// The strings around where a type of `bits` bits, signed or not, overflows in `radix`:
/// its bounds and one past them, with signs and leading zeros, at and past the length
/// below which std skips its overflow checks, malformed signs, and stray characters.
fn edges(max: u128, min: u128, bits: usize, radix: u32) -> Vec<String> {
	let (top, past) = (digits(max, radix), succ(&digits(max, radix), radix));
	let mut all = vec![
		top.clone(),
		format!("+{top}"),
		past.clone(),
		format!("+{past}"),
		format!("000{top}"),
	];
	let upper = top.bytes().map(|b| if b > b'9' { b - 32 } else { b }).collect();
	all.extend([
		format!("-{top}"),
		format!("000{past}"),
		String::from_utf8(upper).unwrap(),
	]);
	if min > 0 {
		let (bottom, below) = (digits(min, radix), succ(&digits(min, radix), radix));
		all.extend([
			format!("-{bottom}"),
			format!("-{below}"),
			format!("-000{bottom}"),
			format!("-00{below}"),
		]);
	}
	// std's unchecked length: `bits / 4` digits, one fewer for a signed type.
	let unchecked = bits / 4 - (min > 0) as usize;
	let high = char::from(DIGITS[radix as usize - 1]);
	for len in [unchecked - 1, unchecked, unchecked + 1] {
		let mut run = String::new();
		for _ in 0..len.max(1) {
			run.push(high);
		}
		all.extend([format!("-{run}"), run]);
	}
	let stray = char::from(DIGITS[radix as usize % 36]);
	for s in [
		"", "+", "-", "0", "+0", "-0", "+-1", "-+1", "++1", "--1", " 1", "1 ", "1_0", "\u{e9}", "1\u{e9}",
	] {
		all.push(s.to_string());
	}
	all.extend([format!("1{stray}"), format!("{stray}")]);
	all
}

/// One parse of `s` in `radix`: the value, or the error each way std prints it and its
/// kind as a match and an `==` read it.
macro_rules! parse_one {
	($t:ty, $s:expr, $radix:expr, $out:expr) => {{
		let (s, radix, out): (&str, u32, &mut Vec<u8>) = ($s, $radix, $out);
		match <$t>::from_str_radix(s, radix) {
			Ok(v) => crate::harness::put(out, &format!("{v}")),
			Err(e) => {
				let kind = match e.kind() {
					IntErrorKind::Empty => 0,
					IntErrorKind::InvalidDigit => 1,
					IntErrorKind::PosOverflow => 2,
					IntErrorKind::NegOverflow => 3,
					_ => 4,
				};
				let over = *e.kind() == IntErrorKind::PosOverflow || e.kind() != &IntErrorKind::NegOverflow;
				crate::harness::put(out, &format!("{e}|{e:?}|{:?}|{e:>42}|{e:<3.5}|{kind}|{over}", e.kind()));
			}
		}
		if radix == 10 {
			let parsed = s.parse::<$t>();
			let from = <$t as std::str::FromStr>::from_str(s);
			let max = match from {
				Ok(v) => (v == <$t>::MAX) as u8 * 4,
				Err(_) => 2,
			};
			out.push(parsed.is_ok() as u8 + max);
		}
	}};
}

/// Parse into the integer type `kind` picks, in the way it also picks: text, bytes mapped
/// into the alphabet of signs and digits, every edge of every radix, or a radix of any
/// value, which panics outside 2 to 36 as `from_str_radix` or `char::to_digit` does.
pub fn parses(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	macro_rules! by_type {
		($($i:literal => $t:ty, $bits:literal, $min:expr;)*) => {
			match kind % 12 {
				$($i => parse_as!($t, $bits, $min),)*
				_ => unreachable!(),
			}
		};
	}
	macro_rules! parse_as {
		($t:ty, $bits:literal, $min:expr) => {{
			let Some((&first, rest)) = data.split_first() else {
				return;
			};
			let radix = 2 + u32::from(first) % 35;
			match kind / 12 % 4 {
				0 => {
					if let Ok(s) = std::str::from_utf8(rest) {
						for c in s.chars() {
							out.extend_from_slice(&c.to_digit(radix).unwrap_or(99).to_be_bytes());
						}
						parse_one!($t, s, radix, out);
					}
				}
				1 => {
					const ALPHABET: &[u8] = b"0123456789+-azAZ_ 0189fF";
					let mut s = String::new();
					for &b in rest {
						s.push(char::from(ALPHABET[usize::from(b) % ALPHABET.len()]));
					}
					parse_one!($t, &s, radix, out);
				}
				2 => {
					for radix in 2..=36 {
						for s in edges(<$t>::MAX as u128, $min, $bits, radix) {
							parse_one!($t, &s, radix, out);
						}
					}
				}
				_ => {
					let radix = u32::from(first) | u32::from(rest.first().copied().unwrap_or(0)) << 24;
					if rest.get(1).is_some_and(|b| b & 1 == 1) {
						out.extend_from_slice(&'7'.to_digit(radix).unwrap_or(99).to_be_bytes());
					}
					let text = if rest.len() > 2 { &rest[2..] } else { &[] };
					parse_one!($t, std::str::from_utf8(text).unwrap_or("1"), radix, out);
				}
			}
		}};
	}
	by_type! {
		0 => u8, 8, 0;
		1 => u16, 16, 0;
		2 => u32, 32, 0;
		3 => u64, 64, 0;
		4 => u128, 128, 0;
		5 => usize, 32, 0;
		6 => i8, 8, i8::MAX as u128 + 1;
		7 => i16, 16, i16::MAX as u128 + 1;
		8 => i32, 32, i32::MAX as u128 + 1;
		9 => i64, 64, i64::MAX as u128 + 1;
		10 => i128, 128, i128::MAX as u128 + 1;
		11 => isize, 32, isize::MAX as u128 + 1;
	}
}

/// Each conversion from `$from` that can't fail, of what `data` holds and the type's
/// limits: `try_from` and `try_into` as std types them, `Result<_, Infallible>`.
macro_rules! widen {
	($data:expr, $out:expr, $from:ty => $($to:ty),+) => {{
		let (taken, _) = take!($from, $data);
		for v in [taken, <$from>::MIN, <$from>::MAX] {
			$(
				let named: Result<$to, Infallible> = <$to>::try_from(v);
				match named {
					Ok(w) => $out.extend_from_slice(&w.to_be_bytes()),
					Err(never) => match never {},
				}
				let into: Result<$to, Infallible> = v.try_into();
				let Ok(w) = into;
				$out.push(u8::from(into == named) | u8::from(w == <$to>::from(v)) << 1);
			)+
		}
	}};
}

/// Every integer conversion that can't fail on wasm32, from the type `kind` picks: to
/// itself, wider, and to `usize` and `isize` where std has `From`; and a `char`'s and the
/// crate's own, which convert the same way.
pub fn widenings(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	match kind % 13 {
		0 => widen!(data, out, u8 => u8, u16, u32, u64, u128, usize, i16, i32, i64, i128, isize),
		1 => widen!(data, out, u16 => u16, u32, u64, u128, usize, i32, i64, i128),
		2 => widen!(data, out, u32 => u32, u64, u128, i64, i128),
		3 => widen!(data, out, u64 => u64, u128, i128),
		4 => widen!(data, out, u128 => u128),
		5 => widen!(data, out, usize => usize),
		6 => widen!(data, out, i8 => i8, i16, i32, i64, i128, isize),
		7 => widen!(data, out, i16 => i16, i32, i64, i128, isize),
		8 => widen!(data, out, i32 => i32, i64, i128),
		9 => widen!(data, out, i64 => i64, i128),
		10 => widen!(data, out, i128 => i128),
		11 => widen!(data, out, isize => isize),
		_ => {
			let (b, _) = take!(u8, data);
			for b in [b, u8::MIN, u8::MAX] {
				let named: Result<char, Infallible> = char::try_from(b);
				let Ok(c) = named;
				for c in [c, char::MIN, char::MAX] {
					let wide: Result<u32, Infallible> = u32::try_from(c);
					let wider: Result<u64, Infallible> = c.try_into();
					let widest: Result<u128, Infallible> = c.try_into();
					let (Ok(a), Ok(b), Ok(c)) = (wide, wider, widest);
					out.extend_from_slice(&a.to_be_bytes());
					out.extend_from_slice(&b.to_be_bytes());
					out.extend_from_slice(&c.to_be_bytes());
				}
				let own: Result<Scaled, Infallible> = b.try_into();
				let Ok(Scaled(s)) = own;
				out.extend_from_slice(&s.to_be_bytes());
			}
		}
	}
}

/// A type of the crate's own that `From` makes, which `try_from` converts by.
struct Scaled(u32);

impl From<u8> for Scaled {
	fn from(b: u8) -> Self {
		Scaled(u32::from(b) * 1000)
	}
}
