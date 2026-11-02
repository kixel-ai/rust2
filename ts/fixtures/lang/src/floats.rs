//! `f64`: arithmetic and comparisons, `as` casts to and from every integer width, its
//! methods and constants, sums, places, `Duration`'s `f64` methods, and formatting and
//! parsing. Operands are
//! bits, so NaNs, infinities, and subnormals come up; `caller.ts` names the values random
//! bits rarely make. A NaN result is written as one NaN, since neither side specifies
//! which NaN an operation makes, and the functions std leaves to libm, kind 7, agree
//! within the ULPs `caller.ts` allows.

use std::f64::consts;
use std::time::Duration;

use crate::fmt::put;

/// An `f64` from the front of `data`, its bits big-endian and zero-padded.
fn take(data: &[u8]) -> (f64, &[u8]) {
	let (bits, rest) = take_bits(data);
	(f64::from_bits(bits), rest)
}

/// The bits [`take`] reads, as a seed, which no NaN quieted on the way.
fn take_bits(data: &[u8]) -> (u64, &[u8]) {
	let mut buf = [0u8; 8];
	let n = data.len().min(buf.len());
	buf[..n].copy_from_slice(&data[..n]);
	(u64::from_be_bytes(buf), &data[n..])
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

/// What `try_from_secs_f64` made, or its error printed both ways.
fn put_secs(out: &mut Vec<u8>, d: Result<Duration, std::time::TryFromFloatSecsError>) {
	match d {
		Ok(d) => put_duration(out, d),
		Err(e) => put(out, &format!("{e}|{e:?}|{e:#?}")),
	}
}

/// A struct with an `f64` field, changed in place through a `&mut`.
#[derive(Clone, Copy, PartialEq, Default)]
struct Gauge {
	level: f64,
	peak: Option<f64>,
}

fn raise(level: &mut f64, by: f64) {
	*level += by;
	*level *= 2.0;
}

/// Each float constant std has, in a `const` of the crate's.
const HALF: f64 = 0.5;
/// A NaN whose sign and payload a constant keeps, which only its bits make.
const PAYLOAD: f64 = f64::from_bits(0xfff8_0000_0000_0123);
const CONSTANTS: [f64; 22] = [
	f64::MAX,
	f64::MIN,
	f64::MIN_POSITIVE,
	f64::EPSILON,
	f64::INFINITY,
	f64::NEG_INFINITY,
	f64::NAN,
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
	consts::FRAC_PI_2,
	consts::FRAC_1_PI,
	1e-300,
	123456789.125,
	HALF,
];

pub fn floats(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let (a, rest) = take(data);
	let (b, rest) = take(rest);
	match kind % 10 {
		0 => {
			// Arithmetic, which JS rounds as Rust does, and comparisons, NaN's included.
			for x in [a + b, a - b, a * b, a / b, a % b, -a, a + 1.5, 0.1 * a] {
				put_f64(out, x);
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
			put_f64(out, c);
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
			// From every integer type, rounded once to the nearest, ties to even.
			let mut wide = [0u8; 16];
			let n = data.len().min(16);
			wide[..n].copy_from_slice(&data[..n]);
			let v = u128::from_be_bytes(wide);
			for x in [
				v as u8 as f64,
				v as u16 as f64,
				v as u32 as f64,
				v as u64 as f64,
				v as f64,
				v as usize as f64,
				v as i8 as f64,
				v as i16 as f64,
				v as i32 as f64,
				v as i64 as f64,
				v as i128 as f64,
				v as isize as f64,
				f64::from(v as u32),
				f64::from(v as i16),
				f64::from(v & 1 == 1),
				(v as u8).into(),
				u64::MAX as f64,
				i64::MIN as f64,
				u128::MAX as f64,
			] {
				put_f64(out, x);
			}
			// And back, where the round trip saturates.
			out.extend_from_slice(&((v as u64) as f64 as u64).to_be_bytes());
			out.extend_from_slice(&((v as i128) as f64 as i128).to_be_bytes());
		}
		3 => {
			// Rounding, which JS's `Math.round` does half up rather than away from zero.
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
				put_f64(out, x);
			}
		}
		4 => {
			// The sign bit and the class, and the doubles next to `a`.
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
			for x in [a.copysign(b), 1.5f64.copysign(a), a.next_up(), a.next_down()] {
				put_f64(out, x);
			}
		}
		5 => {
			// Two operands. std leaves which zero `max` and `min` pick between 0.0 and -0.0.
			let zeros = a == 0.0 && b == 0.0;
			for x in [a.max(b), a.min(b)] {
				put_f64(out, if zeros { 0.0 } else { x });
			}
			for x in [a.midpoint(b), a.div_euclid(b), a.rem_euclid(b)] {
				put_f64(out, x);
			}
			out.push(a.total_cmp(&b) as i8 as u8);
			out.push(a.partial_cmp(&b).map_or(9, |o| o as i8 as u8));
		}
		6 => {
			// Bits and bytes. A NaN's are the engine's, so only a number's round trip.
			if !a.is_nan() {
				out.extend_from_slice(&a.to_bits().to_be_bytes());
				out.extend_from_slice(&a.to_be_bytes());
				out.extend_from_slice(&a.to_le_bytes());
				out.extend_from_slice(&a.to_ne_bytes());
				let be = a.to_be_bytes();
				let le = a.to_le_bytes();
				for x in [f64::from_be_bytes(be), f64::from_le_bytes(le), f64::from_ne_bytes(le)] {
					put_f64(out, x);
				}
			}
			put_f64(out, f64::from_bits(a.to_bits() ^ 1));
		}
		7 => {
			// What std leaves to libm, within the ULPs `caller.ts` allows.
			let n = (b.to_bits() as i32) % 64;
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
				a.powi(n),
				a.cbrt(),
				a.hypot(b),
				s,
				c,
			] {
				put_f64(out, x);
			}
		}
		8 => {
			// `Duration`'s `f64` methods, through std's integer algorithm, with its errors and panics.
			let d = Duration::new(rest.first().map_or(0, |&s| u64::from(s)) << 20, 123_456_789);
			put_f64(out, d.as_secs_f64());
			put_f64(out, d.div_duration_f64(Duration::from_millis(1500)));
			put_f64(out, Duration::ZERO.div_duration_f64(Duration::ZERO));
			put_secs(out, Duration::try_from_secs_f64(a));
			put_secs(out, Duration::try_from_secs_f64(a.abs()));
			put_secs(out, Duration::try_from_secs_f64(a.abs() * 1e-9));
			match kind / 10 % 4 {
				0 => put_duration(out, Duration::from_secs_f64(a)),
				1 => put_duration(out, d.mul_f64(a)),
				2 => put_duration(out, d.div_f64(a)),
				_ => put_duration(out, Duration::MAX.mul_f64(b)),
			}
		}
		_ => {
			// Constants, sums and products, `Default`, places, and fields.
			for x in CONSTANTS {
				put_f64(out, x);
			}
			out.extend_from_slice(&PAYLOAD.to_bits().to_be_bytes());
			out.push(PAYLOAD.is_sign_negative() as u8);
			let v = [a, b, 0.5];
			put_f64(out, v.iter().sum());
			put_f64(out, v.into_iter().product());
			put_f64(out, [-0.0f64; 0].iter().sum());
			put_f64(out, [-0.0f64, -0.0].into_iter().sum::<f64>());
			put_f64(out, f64::default());
			let mut g = Gauge {
				level: a,
				peak: Some(b),
			};
			raise(&mut g.level, b);
			if let Some(peak) = &mut g.peak {
				*peak = peak.max(g.level);
			}
			put_f64(out, g.level);
			put_f64(out, g.peak.unwrap_or(-1.0));
			out.push((g == Gauge::default()) as u8);
			out.push((g.peak == Some(b)) as u8);
			let mut local = a;
			raise(&mut local, 1.0);
			put_f64(out, local);
		}
	}
}

/// `mul_add`, rounded once, of three operands, or of 32 sets made from a seed: any bits,
/// a product's error alone, half the product's last place, where rounding the product
/// first would tie, a product of half `c`'s last place, subnormal sums, products past `f64::MAX` that the sum brings back, and
/// each side of where the runtime's fast path ends. Each result follows its operands, which
/// `fma.ts` rounds exactly where native's differs.
pub fn mul_adds(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	if kind % 2 == 0 {
		let (a, rest) = take(data);
		let (b, rest) = take(rest);
		let (c, _) = take(rest);
		put_mul_add(out, a, b, c);
		return;
	}
	let mut state = take_bits(data).0 | 1;
	for _ in 0..32 {
		let r = step(&mut state);
		let pick = |shift: u32, n: u64| ((r >> shift) % n) as i64;
		let ea = pick(8, 64) - 32;
		let e = pick(16, 64) - 32;
		let a = near(step(&mut state), ea);
		let b = near(step(&mut state), e - ea);
		let c = step(&mut state);
		let (a, b, c) = match r % 6 {
			0 => (f64::from_bits(r), f64::from_bits(step(&mut state)), f64::from_bits(c)),
			1 => (a, b, -(a * b)),
			2 if r >> 63 == 0 => {
				let p = (a * b).abs();
				let half = [0.5, -0.5, 0.25, -0.75, 1.5][pick(24, 5) as usize];
				(a, b, (p.next_up() - p) * half)
			}
			// A product that rounds to half `c`'s last place half the time, so `p + c` ties
			// and only the product's error decides it.
			2 => {
				let c = near(c, e);
				let half = (c.abs().next_up() - c.abs()) * 0.5;
				(a, half / a, c)
			}
			3 => {
				let ea = pick(24, 1000) - 500;
				let e = -1000 - pick(40, 100);
				let (a, b) = (near(r, ea), near(c, e - ea));
				(
					a,
					b,
					if r >> 63 == 0 {
						-(a * b)
					} else {
						near(c >> 1, e + pick(48, 60) - 30)
					},
				)
			}
			4 => {
				let ea = 990 + pick(24, 60);
				let a = near(r, ea);
				let b = near(c, 1023 - ea + pick(40, 4) - 1);
				(a, b, near(c >> 1, 1020 + pick(48, 4)).copysign(-(a * b)))
			}
			_ => {
				let ea = if r >> 63 == 0 {
					990 + pick(24, 10)
				} else {
					pick(24, 1000) - 500
				};
				let (a, b) = (near(r, ea), near(c, -968 + pick(40, 8) - 4 - ea));
				let p = (a * b).abs();
				(a, b, (p.next_up() - p) * 0.5)
			}
		};
		put_mul_add(out, a, b, c);
	}
}

fn put_mul_add(out: &mut Vec<u8>, a: f64, b: f64, c: f64) {
	for x in [a, b, c, a.mul_add(b, c)] {
		put_f64(out, x);
	}
}

/// A double of exponent `e`, or the nearest a finite one has, with the significand and sign of `bits`.
fn near(bits: u64, e: i64) -> f64 {
	let biased = (e + 1023).clamp(0, 2046) as u64;
	f64::from_bits((bits & 0x800f_ffff_ffff_ffff) | (biased << 52))
}

/// A reading whose derived `Debug` writes each `f64` with the placeholder's options.
#[derive(Debug)]
struct Reading {
	level: f64,
	peak: Option<f64>,
	history: Vec<f64>,
}

/// xorshift64, for the doubles and decimals a case makes from its seed.
fn step(state: &mut u64) -> u64 {
	*state ^= *state << 13;
	*state ^= *state >> 7;
	*state ^= *state << 17;
	*state
}

/// `f64`'s `Display`, `Debug`, `LowerExp`, and `UpperExp`, with a precision from the byte
/// after the double, which rounds the exact value half to even, and every other option.
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
				&format!(
					"{a:08.3}|{a:+012e}|{a:<9}|{a:^12?}|{a:*>10.1}|{a:#?}|{a:x?}|{a:-^+15.2E}|{a:010}|{a:+010?}|{a:>w$.p$}|{a:é<w$?}"
				),
			);
			put(out, &a.to_string());
		}
		1 => {
			// Every digit a double has: a subnormal's run to 1074 places.
			put(out, &format!("{a:.1100} {a:.800e} {a:.1100?} {a:.65535}"));
		}
		2 => {
			// The panic prints both bounds with `Debug`.
			let (b, rest) = take(rest);
			let (c, _) = take(rest);
			put_f64(out, a.clamp(b, c));
		}
		3 => {
			let r = Reading {
				level: a,
				peak: Some(a * 0.5),
				history: vec![a, -0.0, 1e-7, 1e16, f64::NAN],
			};
			put(out, &format!("{r:?}|{r:#?}|{r:.2?}|{r:+012?}"));
		}
		_ => {
			// The shortest digits of many doubles, of every exponent, then of ones with a full
			// significand and a few binary digits past the point, which can end half a digit
			// past two as short and as close, where std takes the greater.
			let mut state = take_bits(data).0 | 1;
			for _ in 0..64 {
				let x = f64::from_bits(step(&mut state));
				put(out, &format!("{x} {x:?} {x:e}"));
			}
			for _ in 0..64 {
				let r = step(&mut state);
				let x = (r >> 11) as f64 / (1u64 << (r % 24 + 1)) as f64;
				put(out, &format!("{x:?} {x:e}"));
			}
		}
	}
}

/// `str::parse::<f64>` by std's grammar, with its errors, of the input as text, of bytes
/// mapped into the grammar's alphabet, or of decimals made from a seed.
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
			// Up to 40 digits with a point among them, past 19 where std leaves its fast path,
			// and an exponent within a few of each end of a double's range.
			let mut state = take_bits(data).0 | 1;
			for _ in 0..16 {
				let r = step(&mut state);
				let len = (r % 40 + 1) as usize;
				let point = (r >> 8) as usize % (len + 1);
				let mut s = String::new();
				for i in 0..len {
					if i == point {
						s.push('.');
					}
					s.push(char::from(b'0' + (step(&mut state) % 10) as u8));
				}
				one(&format!("{s}e{}", ((r >> 16) % 700) as i64 - 350), out);
			}
		}
	}
}

/// What parsing `s` makes: the double's bits and its `Debug`, or the error printed each way.
fn one(s: &str, out: &mut Vec<u8>) {
	match s.parse::<f64>() {
		Ok(v) => {
			put_f64(out, v);
			put(out, &format!("{v:?}"));
		}
		Err(e) => put(out, &format!("{e}|{e:?}|{e:#?}|{e:>30}|{e:.7}")),
	}
	out.push(<f64 as std::str::FromStr>::from_str(s).is_ok() as u8);
}
