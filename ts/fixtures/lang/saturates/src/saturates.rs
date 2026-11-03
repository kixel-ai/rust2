//! `Saturating` of every integer width: its operators and their assign forms, which
//! saturate where the integer's would overflow, its comparisons, hash, and formatting, an
//! unsigned one's `Sum` and `Product`, and a field of one; and the saturating methods of its
//! integer and of a `NonZero` of it.

use std::collections::{BTreeSet, HashSet};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::num::{NonZero, Saturating};

/// A `Saturating` in a field, with each derived impl reading it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Gauge<T>(Saturating<T>);

fn hashed<T: Hash>(v: &T) -> u64 {
	let mut h = DefaultHasher::new();
	v.hash(&mut h);
	h.finish()
}

macro_rules! saturates {
	($name:ident, $t:ty, $sign:ident) => {
		/// Apply operation `kind` to `Saturating`s taken from `data`: three operands.
		pub fn $name(kind: u8, data: &[u8], out: &mut Vec<u8>) {
			const N: usize = core::mem::size_of::<$t>();
			let take = |at: usize| {
				let mut buf = [0u8; N];
				for (i, b) in buf.iter_mut().enumerate() {
					*b = data.get(at + i).copied().unwrap_or(0);
				}
				Saturating(<$t>::from_be_bytes(buf))
			};
			let (a, b, c) = (take(0), take(N), take(2 * N));
			let put = |out: &mut Vec<u8>, v: Saturating<$t>| out.extend_from_slice(&v.0.to_be_bytes());
			match kind % 24 {
				0 => put(out, a + b),
				1 => put(out, a - b),
				2 => put(out, a * b),
				3 => put(out, a / b),
				4 => put(out, a % b),
				5 => saturates!(@$sign, $t, a, b, c, put, out),
				6 => put(out, !a),
				7 => put(out, a & b),
				8 => put(out, a | b),
				9 => put(out, a ^ b),
				10 => {
					let mut x = a;
					x += b;
					put(out, x);
					x *= c;
					put(out, x);
					x -= b;
					put(out, x);
				}
				11 => {
					let mut x = a;
					x /= b;
					put(out, x);
				}
				12 => {
					let mut x = a;
					x %= b;
					put(out, x);
				}
				13 => {
					let mut x = a;
					x &= b;
					x |= c;
					x ^= b;
					put(out, x);
				}
				14 => {
					// By the integer, and a reference to one.
					let mut x = a;
					x += b.0;
					put(out, x);
					x -= &c.0;
					put(out, x);
					x *= b.0;
					put(out, x);
					x &= &c.0;
					x |= b.0;
					x ^= c.0;
					put(out, x);
					x /= &b.0;
					put(out, x);
					x %= c.0;
					put(out, x);
				}
				15 => {
					// Through references, which std's impls forward.
					put(out, &a + b);
					put(out, a - &b);
					put(out, &a * &c);
					put(out, !&b);
					put(out, &a ^ &c);
					put(out, &a & b);
					put(out, a | &c);
					let mut x = a;
					x += &b;
					x *= &c;
					put(out, x);
					put(out, &a % &b);
					put(out, &c / &b);
				}
				16 => {
					let bits = [
						a == b,
						a != b,
						a < b,
						a <= b,
						a > b,
						a >= b,
						a.cmp(&b).is_lt(),
						a.partial_cmp(&c).is_some_and(|o| o.is_gt()),
					];
					for bit in bits {
						out.push(bit as u8);
					}
					put(out, a.max(b));
					put(out, a.min(c));
					put(out, a.clamp(b.min(c), b.max(c)));
				}
				17 => {
					out.extend_from_slice(&hashed(&a).to_be_bytes());
					out.push((hashed(&a) == hashed(&a.0)) as u8);
					let set: BTreeSet<Saturating<$t>> = [a, b, c, a].into_iter().collect();
					for v in &set {
						put(out, *v);
					}
					let seen: HashSet<Saturating<$t>> = [a, b].into_iter().collect();
					out.push(seen.contains(&c) as u8);
					out.push(seen.len() as u8);
				}
				18 => {
					let text = format!("{a} {b:?} {a:x} {b:X} {c:o} {a:b} {b:>6} {c:#x} {a:08?} {b:+}");
					out.extend_from_slice(text.as_bytes());
				}
				19 => saturates!(@$sign sums, $t, a, b, c, put, out),
				20 => {
					let zero = Saturating::<$t>::default();
					put(out, zero);
					#[allow(clippy::clone_on_copy, reason = "the clone `Clone` makes")]
					let copy = a.clone();
					put(out, copy);
					put(out, Saturating(a.0));
				}
				21 => {
					let (x, y) = (Gauge(a), Gauge(b));
					let text = format!("{x:?} {:?}", Gauge::<$t>::default());
					out.extend_from_slice(text.as_bytes());
					out.push((x == y) as u8);
					out.push(x.cmp(&y) as i8 as u8);
					out.push((hashed(&x) == hashed(&a)) as u8);
					let mut z = x;
					z.0 -= y.0;
					put(out, z.0);
				}
				22 => {
					// The `NonZero`'s, before the integer's, which panics on a zero divisor.
					if let (Some(x), Some(y)) = (NonZero::new(a.0), NonZero::new(b.0)) {
						out.extend_from_slice(&x.saturating_mul(y).get().to_be_bytes());
					}
					put(out, Saturating(a.0.saturating_mul(b.0)));
					put(out, Saturating(a.0.saturating_add(b.0)));
					put(out, Saturating(a.0.saturating_sub(b.0)));
					put(out, Saturating(a.0.saturating_div(b.0)));
				}
				_ => saturates!(@$sign nonzero, $t, a, b, c, put, out),
			}
		}
	};
	// `Neg`, which only a signed `Saturating` has.
	(@signed, $t:ty, $a:ident, $b:ident, $c:ident, $put:ident, $out:ident) => {{
		$put($out, -$a);
		$put($out, -&$b);
		$put($out, Saturating($a.0.saturating_neg()));
	}};
	(@unsigned, $t:ty, $a:ident, $b:ident, $c:ident, $put:ident, $out:ident) => {
		$out.push(0)
	};
	// `Sum` and `Product`, which std implements only for an unsigned `Saturating`.
	(@signed sums, $t:ty, $a:ident, $b:ident, $c:ident, $put:ident, $out:ident) => {
		$out.push(0)
	};
	(@unsigned sums, $t:ty, $a:ident, $b:ident, $c:ident, $put:ident, $out:ident) => {{
		let all = [$a, $b, $c];
		$put($out, all.iter().sum::<Saturating<$t>>());
		$put($out, all.into_iter().sum::<Saturating<$t>>());
		$put($out, all.iter().product::<Saturating<$t>>());
		$put($out, all.into_iter().product::<Saturating<$t>>());
		$put($out, all[..0].iter().sum::<Saturating<$t>>());
		$put($out, all[..0].iter().product::<Saturating<$t>>());
	}};
	// A signed `NonZero`'s `saturating_neg`, or an unsigned one's `saturating_add`.
	(@signed nonzero, $t:ty, $a:ident, $b:ident, $c:ident, $put:ident, $out:ident) => {
		if let Some(x) = NonZero::new($a.0) {
			$out.extend_from_slice(&x.saturating_neg().get().to_be_bytes());
		}
	};
	(@unsigned nonzero, $t:ty, $a:ident, $b:ident, $c:ident, $put:ident, $out:ident) => {
		if let Some(x) = NonZero::new($a.0) {
			$out.extend_from_slice(&x.saturating_add($b.0).get().to_be_bytes());
		}
	};
}

saturates!(u8s, u8, unsigned);
saturates!(u16s, u16, unsigned);
saturates!(u32s, u32, unsigned);
saturates!(u64s, u64, unsigned);
saturates!(u128s, u128, unsigned);
saturates!(usizes, usize, unsigned);
saturates!(i8s, i8, signed);
saturates!(i16s, i16, signed);
saturates!(i32s, i32, signed);
saturates!(i64s, i64, signed);
saturates!(i128s, i128, signed);
saturates!(isizes, isize, signed);
