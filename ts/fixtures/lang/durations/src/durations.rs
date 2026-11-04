//! `Duration`: made from each unit and read back in each, added, subtracted, scaled, and
//! divided with std's checks and panics, in place through each kind of place, summed,
//! compared, ordered, hashed, held in a field and as a key, printed, and made by constants. Seconds past 2^53 and up to `u64::MAX` take
//! the runtime past its numbers; `caller.ts` names those cases.

use std::cmp::Ordering;
use std::collections::{BTreeMap, HashSet};
use std::time::Duration;

use crate::harness::put;

const TIMEOUT: Duration = Duration::from_millis(1500);
const LIMITS: [Duration; 3] = [Duration::ZERO, Duration::MAX, Duration::from_nanos(999)];

/// A `Duration` in a field, through each derived impl.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Window {
	start: Duration,
	len: Option<Duration>,
}

/// A `u64` and a `u32` from the front of `data`, big-endian and zero-padded.
fn take(data: &[u8]) -> (u64, u32, &[u8]) {
	let mut buf = [0u8; 12];
	let n = data.len().min(buf.len());
	buf[..n].copy_from_slice(&data[..n]);
	let [s0, s1, s2, s3, s4, s5, s6, s7, n0, n1, n2, n3] = buf;
	let secs = u64::from_be_bytes([s0, s1, s2, s3, s4, s5, s6, s7]);
	(secs, u32::from_be_bytes([n0, n1, n2, n3]), &data[n..])
}

/// A `Duration` from the front of `data`, whose nanoseconds never carry.
fn duration(data: &[u8]) -> (Duration, &[u8]) {
	let (secs, nanos, rest) = take(data);
	(Duration::new(secs, nanos % 1_000_000_000), rest)
}

fn put_duration(out: &mut Vec<u8>, d: Duration) {
	out.extend_from_slice(&d.as_secs().to_be_bytes());
	out.extend_from_slice(&d.subsec_nanos().to_be_bytes());
}

fn put_opt(out: &mut Vec<u8>, d: Option<Duration>) {
	match d {
		Some(d) => {
			out.push(1);
			put_duration(out, d);
		}
		None => out.push(0),
	}
}

/// The crate's own `+=` onto a `Duration`, which adds twice.
struct Twice(Duration);

impl std::ops::AddAssign<Twice> for Duration {
	fn add_assign(&mut self, by: Twice) {
		*self += by.0;
		*self += by.0;
	}
}

/// `-=` through a `&mut`, which is an `rs.Place`.
fn shrink(d: &mut Duration, by: Duration) {
	*d -= by;
}

pub fn durations(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let (a, rest) = duration(data);
	let (b, rest) = duration(rest);
	let mut word = [0u8; 4];
	let k = rest.len().min(word.len());
	word[..k].copy_from_slice(&rest[..k]);
	let n = u32::from_be_bytes(word);
	match kind % 14 {
		0 => {
			// `new`, which carries nanoseconds past a second and panics past `u64::MAX`.
			let (secs, nanos, _) = take(data);
			put_duration(out, Duration::new(secs, nanos));
		}
		1 => {
			// Each unit in, and each out.
			let (units, _, _) = take(data);
			put_duration(out, Duration::from_secs(units));
			put_duration(out, Duration::from_millis(units));
			put_duration(out, Duration::from_micros(units));
			put_duration(out, Duration::from_nanos(units));
			out.extend_from_slice(&b.as_secs().to_be_bytes());
			out.extend_from_slice(&b.subsec_millis().to_be_bytes());
			out.extend_from_slice(&b.subsec_micros().to_be_bytes());
			out.extend_from_slice(&b.as_millis().to_be_bytes());
			out.extend_from_slice(&b.as_micros().to_be_bytes());
			out.extend_from_slice(&b.as_nanos().to_be_bytes());
			out.push(b.is_zero() as u8);
		}
		2 => {
			// What overflows to `None`, saturates, or never overflows.
			put_opt(out, a.checked_add(b));
			put_opt(out, a.checked_sub(b));
			put_opt(out, a.checked_mul(n));
			put_opt(out, a.checked_div(n));
			put_duration(out, a.saturating_add(b));
			put_duration(out, a.saturating_sub(b));
			put_duration(out, a.saturating_mul(n));
			put_duration(out, a.abs_diff(b));
		}
		3 => put_duration(out, a + b),
		4 => put_duration(out, a - b),
		5 => {
			put_duration(out, n * a);
			put_duration(out, a * n);
		}
		6 => put_duration(out, a / n),
		7 => {
			// Comparisons, as values, in `Option`s, and as keys.
			let bits = [
				a == b,
				a != b,
				a < b,
				a <= b,
				a > b,
				a >= b,
				a.cmp(&b).is_lt(),
				a.partial_cmp(&b) == Some(Ordering::Greater),
				Some(a) == Some(b),
				Some(a) == None,
			];
			for bit in bits {
				out.push(bit as u8);
			}
			put_duration(out, a.max(b));
			put_duration(out, a.min(b));
			let mut seen: BTreeMap<Duration, u8> = BTreeMap::new();
			for (i, d) in [a, b, TIMEOUT, a, Duration::ZERO].into_iter().enumerate() {
				seen.insert(d, i as u8);
			}
			for (d, i) in &seen {
				put_duration(out, *d);
				out.push(*i);
			}
			let set: HashSet<Duration> = [a, b, TIMEOUT].into_iter().collect();
			out.push(set.len() as u8);
			out.push(set.contains(&Duration::new(a.as_secs(), a.subsec_nanos())) as u8);
		}
		8 => {
			// `Debug` in each unit, padded by each spec but `0`, which pads as a space does.
			let c = Duration::new(0, b.subsec_nanos());
			let d = Duration::from_nanos(u64::from(n % 1_000_000));
			put(
				out,
				&format!("{a:?}|{c:?}|{d:?}|{c:>16?}|{d:*^14?}|{c:<12?}|{a:08?}|{:?}", Some(b)),
			);
			// A precision rounds the fraction half to even, carrying into the whole part.
			let p = (n % 13) as usize;
			put(
				out,
				&format!(
					"{a:.0?}|{a:.3?}|{c:.1?}|{d:.12?}|{a:+?}|{c:+12.2?}|{d:<10.1?}|{a:.p$?}|{:.1?}",
					Some(b)
				),
			);
			for d in [
				1_500_000_000,
				2_500_000_000,
				1_250_000,
				1_350_000,
				999_500,
				1_999_999_999,
			] {
				let d = Duration::from_nanos(d);
				put(out, &format!("{d:.0?} {d:.1?} {d:.2?}"));
			}
		}
		10 => {
			// `+=` and `-=` on a local, a field, what an `Option`'s `&mut` borrows, an
			// element, and through a `&mut`, each panicking as `+` and `-` do, and the
			// crate's own `+=`.
			let mut c = a;
			c += b;
			put_duration(out, c);
			let mut w = Window { start: c, len: Some(b) };
			w.start -= b;
			put_duration(out, w.start);
			if let Some(len) = &mut w.len {
				*len -= a.min(b);
			}
			put_opt(out, w.len);
			let mut v = [a, b];
			v[1] += a;
			put_duration(out, v[1]);
			shrink(&mut v[0], b);
			put_duration(out, v[0]);
			let mut d = a;
			d += Twice(b);
			put_duration(out, d);
		}
		11 => {
			// `*=` and `/=` by a `u32`, the last by zero panicking.
			let mut c = a;
			c *= n;
			put_duration(out, c);
			let mut v = vec![a];
			v[0] /= n;
			put_duration(out, v[0]);
		}
		12 => {
			// Each larger unit, panicking past `u64::MAX` seconds, minutes before hours.
			let (units, _, _) = take(data);
			let mut wide = [0u8; 16];
			let k = data.len().min(wide.len());
			wide[..k].copy_from_slice(&data[..k]);
			put_duration(out, Duration::from_mins(units));
			put_duration(out, Duration::from_hours(units));
			put_duration(out, Duration::from_nanos_u128(u128::from_be_bytes(wide)));
		}
		13 => {
			// `Sum` of values and of references, whose nanoseconds carry at the end, past
			// `u64::MAX` seconds a panic.
			let parts = [a, b, Duration::new(0, n % 1_000_000_000)];
			put_duration(out, parts.iter().sum());
			put_duration(out, [b, a].into_iter().sum::<Duration>());
			put_duration(out, parts.iter().filter(|d| !d.is_zero()).copied().sum());
		}
		_ => {
			// Constants, `Default`, and a field.
			for d in LIMITS {
				put_duration(out, d);
			}
			put_duration(out, TIMEOUT);
			put_duration(out, Duration::default());
			put_opt(out, Duration::MAX.checked_add(Duration::from_nanos(1)));
			let w = Window {
				start: a,
				len: (b > TIMEOUT).then_some(b),
			};
			let v = w;
			out.push((w == Window::default()) as u8);
			out.push(w.cmp(&Window { start: b, len: None }).is_lt() as u8);
			let windows: HashSet<Window> = [w, v, Window::default()].into_iter().collect();
			out.push(windows.len() as u8);
			put(out, &format!("{w:?}|{:#?}", Window::default()));
		}
	}
}
