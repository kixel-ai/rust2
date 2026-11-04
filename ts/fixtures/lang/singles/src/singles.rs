//! `f32`, as `floats` has `f64`: arithmetic, which rounds each result to a single, casts
//! to and from `f64` and every integer width, its methods and constants, sums, places,
//! `Duration`'s `f32` methods, and formatting and parsing. Operands are bits, so NaNs,
//! infinities, and subnormals come up; `caller.ts` names the values random bits rarely
//! make. A NaN result is written as one NaN, since neither side specifies which NaN an
//! operation makes, and the functions std leaves to libm, kind 7, agree within the ULPs
//! `caller.ts` allows.

use std::f32::consts;
use std::time::Duration;

use crate::harness::put;

/// The bits of an `f32` from the front of `data`, big-endian and zero-padded.
fn take_bits(data: &[u8]) -> (u32, &[u8]) {
	let mut buf = [0u8; 4];
	let n = data.len().min(buf.len());
	buf[..n].copy_from_slice(&data[..n]);
	(u32::from_be_bytes(buf), &data[n..])
}

/// An `f32` from the front of `data`.
fn take(data: &[u8]) -> (f32, &[u8]) {
	let (bits, rest) = take_bits(data);
	(f32::from_bits(bits), rest)
}

/// `x`'s bits, any NaN as `f32::NAN`'s.
fn put_f32(out: &mut Vec<u8>, x: f32) {
	let x = if x.is_nan() { f32::NAN } else { x };
	out.extend_from_slice(&x.to_bits().to_be_bytes());
}

/// `x`'s bits, any NaN as `f64::NAN`'s.
fn put_f64(out: &mut Vec<u8>, x: f64) {
	let x = if x.is_nan() { f64::NAN } else { x };
	out.extend_from_slice(&x.to_bits().to_be_bytes());
}

fn put_duration(out: &mut Vec<u8>, d: Duration) {
	out.extend_from_slice(&d.as_secs().to_be_bytes());
	out.extend_from_slice(&d.subsec_nanos().to_be_bytes());
}

/// What `try_from_secs_f32` made, or its error printed.
fn put_secs(out: &mut Vec<u8>, d: Result<Duration, std::time::TryFromFloatSecsError>) {
	match d {
		Ok(d) => put_duration(out, d),
		Err(e) => put(out, &format!("{e}|{e:?}")),
	}
}

/// A struct with an `f32` field, changed in place through a `&mut`.
#[derive(Clone, Copy, PartialEq, Default)]
struct Gauge {
	level: f32,
	peak: Option<f32>,
}

fn raise(level: &mut f32, by: f32) {
	*level += by;
	*level *= 2.0;
}

/// A NaN whose sign and payload a constant keeps, which only its bits make.
const PAYLOAD: f32 = f32::from_bits(0xffc0_0123);
/// A signaling NaN, which JS's conversion from a single would quiet.
const SIGNALING: f32 = f32::from_bits(0x7f80_0001);
const CONSTANTS: [f32; 21] = [
	f32::MAX,
	f32::MIN,
	f32::MIN_POSITIVE,
	f32::EPSILON,
	f32::INFINITY,
	f32::NEG_INFINITY,
	f32::NAN,
	-0.0,
	consts::PI,
	consts::TAU,
	consts::E,
	consts::SQRT_2,
	consts::FRAC_1_SQRT_2,
	consts::LN_2,
	consts::LN_10,
	consts::LOG2_E,
	consts::LOG10_E,
	consts::FRAC_1_PI,
	1e-40,
	0.1,
	16_777_216.0,
];

pub fn singles(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let (a, rest) = take(data);
	let (b, rest) = take(rest);
	match kind % 10 {
		0 => {
			// Arithmetic, each result a single, and comparisons, NaN's included.
			for x in [
				a + b,
				a - b,
				a * b,
				a / b,
				a % b,
				-a,
				a + 1.5,
				0.1 * a,
				a * b + a,
				(a - b) / (a + b),
			] {
				put_f32(out, x);
			}
			for bit in [
				a == b,
				a != b,
				a < b,
				a <= b,
				a > b,
				a >= b,
				a == 0.0,
				a.lt(&b),
				(&a).ge(&&b),
			] {
				out.push(bit as u8);
			}
			let mut c = a;
			c += b;
			c -= 0.25;
			c *= b;
			c /= 3.0;
			c %= a;
			put_f32(out, c);
			// Through `f64` and back: exact one way, rounded once the other.
			let (wa, wb) = (a as f64, f64::from(b));
			for x in [wa, wa * wb, wa / wb, wa + 1e-30] {
				put_f64(out, x);
			}
			for x in [
				(wa * wb) as f32,
				(wa / wb) as f32,
				(wa + 1e-30) as f32,
				(wa + wb) as f32,
			] {
				put_f32(out, x);
			}
		}
		1 => {
			// To every integer type: truncated, saturated at its bounds, and 0 for NaN.
			out.extend_from_slice(&(a as u8).to_be_bytes());
			out.extend_from_slice(&(a as u16).to_be_bytes());
			out.extend_from_slice(&(a as u32).to_be_bytes());
			out.extend_from_slice(&(a as u64).to_be_bytes());
			out.extend_from_slice(&(a as u128).to_be_bytes());
			out.extend_from_slice(&(a as usize).to_be_bytes());
			out.extend_from_slice(&(a as i8).to_be_bytes());
			out.extend_from_slice(&(a as i16).to_be_bytes());
			out.extend_from_slice(&(a as i32).to_be_bytes());
			out.extend_from_slice(&(a as i64).to_be_bytes());
			out.extend_from_slice(&(a as i128).to_be_bytes());
			out.extend_from_slice(&(a as isize).to_be_bytes());
		}
		2 => {
			// From every integer type and from `f64`, rounded once to the nearest, ties to
			// even, where through the nearest `f64` would round twice.
			let mut wide = [0u8; 16];
			let n = data.len().min(16);
			wide[..n].copy_from_slice(&data[..n]);
			let v = u128::from_be_bytes(wide);
			let w = f64::from_bits(v as u64);
			for x in [
				v as u8 as f32,
				v as u16 as f32,
				v as u32 as f32,
				v as u64 as f32,
				v as f32,
				v as usize as f32,
				v as i8 as f32,
				v as i16 as f32,
				v as i32 as f32,
				v as i64 as f32,
				v as i128 as f32,
				v as isize as f32,
				(v >> 64) as u64 as f32,
				(v >> 64) as i64 as f32,
				f32::from(v as u16),
				f32::from(v as i16),
				f32::from(v & 1 == 1),
				(v as u8).into(),
				w as f32,
				u64::MAX as f32,
				i64::MIN as f32,
				u128::MAX as f32,
				i32::MAX as f32,
			] {
				put_f32(out, x);
			}
			// And back, where the round trip saturates.
			out.extend_from_slice(&((v as u64) as f32 as u64).to_be_bytes());
			out.extend_from_slice(&((v as i128) as f32 as i128).to_be_bytes());
			out.extend_from_slice(&((v as u32) as f32 as u32).to_be_bytes());
		}
		3 => {
			// Rounding, and the methods that round once to a single.
			for x in [
				a.floor(),
				a.ceil(),
				a.trunc(),
				a.round(),
				a.round_ties_even(),
				a.fract(),
				a.abs(),
				a.signum(),
				a.sqrt(),
				a.recip(),
				a.to_degrees(),
				a.to_radians(),
			] {
				put_f32(out, x);
			}
		}
		4 => {
			// The sign bit and the class, and the singles next to `a`.
			let bits = [
				a.is_nan(),
				a.is_finite(),
				a.is_infinite(),
				a.is_normal(),
				a.is_subnormal(),
				a.is_sign_positive(),
				a.is_sign_negative(),
			];
			for bit in bits {
				out.push(bit as u8);
			}
			for x in [a.copysign(b), 1.5f32.copysign(a), a.next_up(), a.next_down()] {
				put_f32(out, x);
			}
		}
		5 => {
			// Two operands. std leaves which zero `max` and `min` pick between 0.0 and -0.0.
			let zeros = a == 0.0 && b == 0.0;
			for x in [a.max(b), a.min(b)] {
				put_f32(out, if zeros { 0.0 } else { x });
			}
			for x in [
				a.midpoint(b),
				a.div_euclid(b),
				a.rem_euclid(b),
				a.powi((b.to_bits() as i32) % 300),
			] {
				put_f32(out, x);
			}
			out.push(a.total_cmp(&b) as i8 as u8);
			out.push(a.partial_cmp(&b).map_or(9, |o| o as i8 as u8));
		}
		6 => {
			// Bits and bytes, a NaN's sign and payload kept from the bits they're made of.
			let bits = take_bits(data).0;
			out.extend_from_slice(&f32::from_bits(bits).to_bits().to_be_bytes());
			out.extend_from_slice(&f32::from_bits(bits).to_be_bytes());
			out.extend_from_slice(&f32::from_bits(bits).to_le_bytes());
			out.extend_from_slice(&f32::from_be_bytes(bits.to_be_bytes()).to_bits().to_be_bytes());
			out.extend_from_slice(&f32::from_le_bytes(bits.to_le_bytes()).to_bits().to_be_bytes());
			out.extend_from_slice(&f32::from_ne_bytes(bits.to_le_bytes()).to_ne_bytes());
			out.push(f32::from_bits(bits).is_sign_negative() as u8);
			out.extend_from_slice(&f32::from_bits(bits).copysign(-1.0).to_bits().to_be_bytes());
			if !a.is_nan() {
				out.extend_from_slice(&a.to_bits().to_be_bytes());
			}
			put_f32(out, f32::from_bits(a.to_bits() ^ 1));
		}
		7 => {
			// What std leaves to libm, within the ULPs `caller.ts` allows.
			let (s, c) = a.sin_cos();
			for x in [
				a.sin(),
				a.cos(),
				a.tan(),
				a.asin(),
				a.acos(),
				a.atan(),
				a.atan2(b),
				a.sinh(),
				a.cosh(),
				a.tanh(),
				a.asinh(),
				a.acosh(),
				a.atanh(),
				a.exp(),
				a.exp2(),
				a.exp_m1(),
				a.ln(),
				a.log(b),
				a.log2(),
				a.log10(),
				a.ln_1p(),
				a.powf(b),
				a.cbrt(),
				a.hypot(b),
				s,
				c,
			] {
				put_f32(out, x);
			}
		}
		8 => {
			// `Duration`'s `f32` methods, with their errors and panics.
			let d = Duration::new(rest.first().map_or(0, |&s| u64::from(s)) << 20, 123_456_789);
			put_f32(out, d.as_secs_f32());
			put_f32(
				out,
				Duration::new(u64::from(b.to_bits()), a.to_bits() % 1_000_000_000).as_secs_f32(),
			);
			put_f32(out, d.div_duration_f32(Duration::from_millis(1500)));
			put_f32(out, Duration::ZERO.div_duration_f32(Duration::ZERO));
			put_secs(out, Duration::try_from_secs_f32(a));
			put_secs(out, Duration::try_from_secs_f32(a.abs()));
			put_secs(out, Duration::try_from_secs_f32(a.abs() * 1e-9));
			match kind / 10 % 4 {
				0 => put_duration(out, Duration::from_secs_f32(a)),
				1 => put_duration(out, d.mul_f32(a)),
				2 => put_duration(out, d.div_f32(a)),
				_ => put_duration(out, Duration::MAX.mul_f32(b)),
			}
		}
		_ => {
			// Constants, sums and products, `Default`, places, and fields.
			for x in CONSTANTS {
				put_f32(out, x);
			}
			out.extend_from_slice(&PAYLOAD.to_bits().to_be_bytes());
			out.extend_from_slice(&SIGNALING.to_bits().to_be_bytes());
			out.push(PAYLOAD.is_sign_negative() as u8);
			let v = [a, b, 0.1];
			put_f32(out, v.iter().sum());
			put_f32(out, v.into_iter().product());
			put_f32(out, [-0.0f32; 0].iter().sum());
			put_f32(out, [-0.0f32, -0.0].into_iter().sum::<f32>());
			put_f32(out, f32::default());
			let mut g = Gauge {
				level: a,
				peak: Some(b),
			};
			raise(&mut g.level, b);
			if let Some(peak) = &mut g.peak {
				*peak = peak.max(g.level);
			}
			put_f32(out, g.level);
			put_f32(out, g.peak.unwrap_or(-1.0));
			out.push((g == Gauge::default()) as u8);
			out.push((g.peak == Some(b)) as u8);
			let mut local = a;
			raise(&mut local, 0.1);
			put_f32(out, local);
		}
	}
}

/// A reading whose derived `Debug` writes each `f32` with the placeholder's options.
#[derive(Debug)]
struct Reading {
	level: f32,
	history: Vec<f32>,
}

/// `mul_add`, rounded once, of three operands, or of 32 sets made from a seed: any bits,
/// a product's error alone, products of 13-bit odd significands, which a double holds
/// exactly halfway between two singles, plus a tiny addend, subnormal sums, products past
/// `f32::MAX` that the sum brings back, and half the product's last place.
pub fn mul_adds(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	if kind % 2 == 0 {
		let (a, rest) = take(data);
		let (b, rest) = take(rest);
		let (c, _) = take(rest);
		put_f32(out, a.mul_add(b, c));
		return;
	}
	let mut state = take_bits(data).0 | 1;
	for _ in 0..32 {
		let r = step(&mut state);
		let pick = |shift: u32, n: u32| ((r >> shift) % n) as i32;
		let ea = pick(4, 32) - 16;
		let e = pick(9, 32) - 16;
		let a = near(step(&mut state), ea);
		let b = near(step(&mut state), e - ea);
		let c = step(&mut state);
		let (a, b, c) = match r % 6 {
			0 => (f32::from_bits(r), f32::from_bits(step(&mut state)), f32::from_bits(c)),
			1 => (a, b, -(a * b)),
			2 => {
				let odd = |x: f32| f32::from_bits((x.to_bits() & 0xffff_f000) | 0x800);
				(odd(a), odd(b), near(c, e - 26 - pick(14, 100)))
			}
			3 => {
				let ea = pick(14, 120) - 60;
				let e = -130 - pick(21, 30);
				let (a, b) = (near(r, ea), near(c, e - ea));
				(
					a,
					b,
					if r >> 31 == 0 {
						-(a * b)
					} else {
						near(c >> 1, e + pick(26, 30) - 15)
					},
				)
			}
			4 => {
				let ea = 100 + pick(14, 27);
				let a = near(r, ea);
				let b = near(c, 127 - ea + pick(21, 4) - 1);
				(a, b, near(c >> 1, 124 + pick(26, 4)).copysign(-(a * b)))
			}
			_ => {
				let p = (a * b).abs();
				let half = [0.5, -0.5, 0.25, -0.75, 1.5][pick(14, 5) as usize];
				(a, b, (p.next_up() - p) * half)
			}
		};
		put_f32(out, a.mul_add(b, c));
	}
}

/// A single of exponent `e`, or the nearest a finite one has, with the significand and sign of `bits`.
fn near(bits: u32, e: i32) -> f32 {
	let biased = (e + 127).clamp(0, 254) as u32;
	f32::from_bits((bits & 0x807f_ffff) | (biased << 23))
}

/// xorshift32, for the singles and decimals a case makes from its seed.
fn step(state: &mut u32) -> u32 {
	*state ^= *state << 13;
	*state ^= *state >> 17;
	*state ^= *state << 5;
	*state
}

/// `f32`'s `Display`, `Debug`, `LowerExp`, and `UpperExp`, with a precision from the byte
/// after the single, and every other option.
pub fn formats(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let (a, rest) = take(data);
	let p = usize::from(rest.first().copied().unwrap_or(0));
	let w = p / 4;
	match kind % 5 {
		0 => {
			put(
				out,
				&format!("{a} {a:?} {a:e} {a:E} {a:.p$} {a:.p$?} {a:.p$e} {a:.p$E} {a:+} {a:+?} {a:+.p$e}"),
			);
			put(
				out,
				&format!("{a:08.3}|{a:+012e}|{a:<9}|{a:^12?}|{a:*>10.1}|{a:#?}|{a:-^+15.2E}|{a:010}|{a:>w$.p$}"),
			);
			put(out, &a.to_string());
		}
		1 => {
			// Every digit a single has: a subnormal's run to 149 places.
			put(out, &format!("{a:.160} {a:.120e} {a:.160?}"));
		}
		2 => {
			// The panic prints both bounds with `Debug`.
			let (b, rest) = take(rest);
			let (c, _) = take(rest);
			put_f32(out, a.clamp(b, c));
		}
		3 => {
			let r = Reading {
				level: a,
				history: vec![a * 0.5, -0.0, 1e-4, 1e16, f32::NAN],
			};
			put(out, &format!("{r:?}|{r:.2?}|{r:+012?}"));
		}
		_ => {
			// The shortest digits of many singles, of every exponent, then of ones with a
			// full significand and a few binary digits past the point, which can end half a
			// digit past two as short and as close, where std takes the greater.
			let mut state = take_bits(data).0 | 1;
			for _ in 0..64 {
				let x = f32::from_bits(step(&mut state));
				put(out, &format!("{x} {x:?} {x:e}"));
			}
			for _ in 0..64 {
				let r = step(&mut state);
				let x = (r >> 8) as f32 / (1u32 << (r % 16 + 1)) as f32;
				put(out, &format!("{x:?} {x:e}"));
			}
		}
	}
}

/// `str::parse::<f32>`, with its errors, of the input as text, of bytes mapped into the
/// grammar's alphabet, or of decimals made from a seed.
pub fn parses(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	const ALPHABET: &[u8] = b"0123456789012345678901234567890.eE+-_infINFtyTYa ";
	match kind % 3 {
		0 => {
			if let Ok(s) = std::str::from_utf8(data) {
				one(s, out);
			}
		}
		1 => {
			let mut s = String::new();
			for &b in data {
				s.push(char::from(ALPHABET[usize::from(b) % ALPHABET.len()]));
			}
			one(&s, out);
		}
		_ => {
			// Halfway between two singles, which a double holds exactly, written to a few
			// digits short of exact and just past: the nearest double of each lands on the
			// halfway point, so rounding through one would round twice.
			let mut state = take_bits(data).0 | 1;
			for _ in 0..16 {
				let x = f32::from_bits(step(&mut state) & 0x7fff_ffff);
				let half = (f64::from(x) + f64::from(x.next_up())) / 2.0;
				let p = (step(&mut state) % 30 + 8) as usize;
				one(&format!("{half:.p$e}"), out);
				one(&format!("{half:e}"), out);
			}
			// Up to 20 digits with a point among them, and an exponent within a few of each
			// end of a single's range.
			for _ in 0..16 {
				let r = step(&mut state);
				let len = (r % 20 + 1) as usize;
				let point = (r >> 8) as usize % (len + 1);
				let mut s = String::new();
				for i in 0..len {
					if i == point {
						s.push('.');
					}
					s.push(char::from(b'0' + (step(&mut state) % 10) as u8));
				}
				one(&format!("{s}e{}", ((r >> 16) % 100) as i64 - 50), out);
			}
		}
	}
}

/// What parsing `s` makes: the single's bits and its `Debug`, or the error printed.
fn one(s: &str, out: &mut Vec<u8>) {
	match s.parse::<f32>() {
		Ok(v) => {
			put_f32(out, v);
			put(out, &format!("{v:?}"));
		}
		Err(e) => put(out, &format!("{e}|{e:?}")),
	}
	out.push(<f32 as std::str::FromStr>::from_str(s).is_ok() as u8);
}
