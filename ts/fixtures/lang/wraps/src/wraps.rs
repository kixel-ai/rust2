//! `Wrapping` of every integer width: its operators and their assign forms, which wrap
//! where the integer's would overflow and mask a shift's amount to the width, its
//! comparisons, hash, and formatting, its `Sum` and `Product`, and a field of one.

use std::collections::{BTreeSet, HashSet};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::num::Wrapping;

/// A `Wrapping` in a field, with each derived impl reading it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Tick<T>(Wrapping<T>);

fn hashed<T: Hash>(v: &T) -> u64 {
	let mut h = DefaultHasher::new();
	v.hash(&mut h);
	h.finish()
}

macro_rules! wraps {
	($name:ident, $t:ty) => {
		/// Apply operation `kind` to `Wrapping`s taken from `data`: three operands, then a
		/// shift amount, which may pass the width many times over.
		pub fn $name(kind: u8, data: &[u8], out: &mut Vec<u8>) {
			const N: usize = core::mem::size_of::<$t>();
			let take = |at: usize| {
				let mut buf = [0u8; N];
				for (i, b) in buf.iter_mut().enumerate() {
					*b = data.get(at + i).copied().unwrap_or(0);
				}
				Wrapping(<$t>::from_be_bytes(buf))
			};
			let (a, b, c) = (take(0), take(N), take(2 * N));
			let byte = |i: usize| data.get(3 * N + i).copied().unwrap_or(0);
			let n = u32::from_be_bytes([byte(1), 0, 0, byte(0)]) as usize;
			let put = |out: &mut Vec<u8>, v: Wrapping<$t>| out.extend_from_slice(&v.0.to_be_bytes());
			match kind % 24 {
				0 => put(out, a + b),
				1 => put(out, a - b),
				2 => put(out, a * b),
				3 => put(out, a / b),
				4 => put(out, a % b),
				5 => put(out, -a),
				6 => put(out, !a),
				7 => put(out, a & b),
				8 => put(out, a | b),
				9 => put(out, a ^ b),
				10 => put(out, a << n),
				11 => put(out, a >> n),
				12 => {
					let mut x = a;
					x += b;
					put(out, x);
					x *= c;
					put(out, x);
					x -= b;
					put(out, x);
				}
				13 => {
					let mut x = a;
					x /= b;
					put(out, x);
				}
				14 => {
					let mut x = a;
					x %= b;
					put(out, x);
				}
				15 => {
					let mut x = a;
					x &= b;
					x |= c;
					x ^= b;
					put(out, x);
				}
				16 => {
					let mut x = a;
					x <<= n;
					put(out, x);
					x >>= n;
					put(out, x);
					// Constant amounts, one past the width.
					put(out, a << 3);
					put(out, a >> (<$t>::BITS as usize + 1));
				}
				17 => {
					// Through references, which std's impls forward.
					put(out, &a + b);
					put(out, a - &b);
					put(out, &a * &c);
					put(out, -&a);
					put(out, !&b);
					put(out, &a << n);
					put(out, a >> &n);
					put(out, &a ^ &c);
					let mut x = a;
					x += &b;
					x <<= &n;
					put(out, x);
				}
				18 => {
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
				19 => {
					out.extend_from_slice(&hashed(&a).to_be_bytes());
					out.push((hashed(&a) == hashed(&a.0)) as u8);
					let set: BTreeSet<Wrapping<$t>> = [a, b, c, a].into_iter().collect();
					for v in &set {
						put(out, *v);
					}
					let seen: HashSet<Wrapping<$t>> = [a, b].into_iter().collect();
					out.push(seen.contains(&c) as u8);
					out.push(seen.len() as u8);
				}
				20 => {
					let text = format!("{a} {b:?} {a:x} {b:X} {c:o} {a:b} {b:>6} {c:#x} {a:08?} {b:+}");
					out.extend_from_slice(text.as_bytes());
				}
				21 => {
					let all = [a, b, c];
					put(out, all.iter().sum::<Wrapping<$t>>());
					put(out, all.into_iter().sum::<Wrapping<$t>>());
					put(out, all.iter().product::<Wrapping<$t>>());
					put(out, all.into_iter().product::<Wrapping<$t>>());
					put(out, all[..0].iter().sum::<Wrapping<$t>>());
					put(out, all[..0].iter().product::<Wrapping<$t>>());
				}
				22 => {
					let zero = Wrapping::<$t>::default();
					put(out, zero);
					#[allow(clippy::clone_on_copy, reason = "the clone `Clone` makes")]
					let copy = a.clone();
					put(out, copy);
					put(out, Wrapping(a.0));
				}
				_ => {
					let (x, y) = (Tick(a), Tick(b));
					let text = format!("{x:?} {:?}", Tick::<$t>::default());
					out.extend_from_slice(text.as_bytes());
					out.push((x == y) as u8);
					out.push(x.cmp(&y) as i8 as u8);
					out.push((hashed(&x) == hashed(&a)) as u8);
					let mut z = x;
					z.0 -= y.0;
					put(out, z.0);
				}
			}
		}
	};
}

wraps!(u8s, u8);
wraps!(u16s, u16);
wraps!(u32s, u32);
wraps!(u64s, u64);
wraps!(u128s, u128);
wraps!(usizes, usize);
wraps!(i8s, i8);
wraps!(i16s, i16);
wraps!(i32s, i32);
wraps!(i64s, i64);
wraps!(i128s, i128);
wraps!(isizes, isize);
