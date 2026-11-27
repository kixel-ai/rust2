//! Writes differential cases for rust2-lean's translation of the fixture's `cases`, the
//! models `MoqDiff.lean` does not reach: inputs and what native Rust returns, one line of
//! integers per case, for `FixtureDiff.lean` to replay. Run by `compare/fixture.sh`.

use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet};
use std::io::Write;
use std::panic::{AssertUnwindSafe, catch_unwind};

use fixture::cases::{self, Rank};

/// xorshift64, so a seed always draws the same cases.
struct Rng(u64);

impl Rng {
	fn below(&mut self, n: u64) -> u64 {
		self.0 ^= self.0 << 13;
		self.0 ^= self.0 >> 7;
		self.0 ^= self.0 << 17;
		self.0 % n
	}

	fn one_in(&mut self, n: u64) -> bool {
		self.below(n) == 0
	}

	/// Bytes near the ends of `u8`, where overflow is.
	fn byte(&mut self) -> u8 {
		[0, 1, 2, 3, 100, 127, 128, 200, 253, 254, 255][self.below(11) as usize]
	}

	fn bytes(&mut self) -> Vec<u8> {
		(0..self.below(6)).map(|_| self.byte()).collect()
	}

	fn rank(&mut self) -> Rank {
		Rank {
			up: self.one_in(2),
			low: Reverse([i8::MIN, -1, 0, 1, i8::MAX][self.below(5) as usize]),
		}
	}
}

/// An encoder of one case.
#[derive(Default)]
struct W(Vec<i128>);

trait Enc {
	fn enc(&self, w: &mut W);
}

impl W {
	fn put(&mut self, v: impl Enc) -> &mut Self {
		v.enc(self);
		self
	}

	fn int(&mut self, v: i128) -> &mut Self {
		self.0.push(v);
		self
	}
}

macro_rules! ints {
	($($t:ty),*) => {$(impl Enc for $t { fn enc(&self, w: &mut W) { w.0.push(*self as i128) } })*};
}
ints!(u8, u16, u32, usize, i8, bool);

impl<T: Enc + ?Sized> Enc for &T {
	fn enc(&self, w: &mut W) {
		(*self).enc(w)
	}
}

impl<T: Enc> Enc for Option<T> {
	fn enc(&self, w: &mut W) {
		match self {
			None => w.int(0),
			Some(v) => w.int(1).put(v),
		};
	}
}

impl<T: Enc> Enc for [T] {
	fn enc(&self, w: &mut W) {
		w.int(self.len() as i128);
		self.iter().for_each(|v| v.enc(w));
	}
}

impl<T: Enc> Enc for Vec<T> {
	fn enc(&self, w: &mut W) {
		self.as_slice().enc(w)
	}
}

impl<A: Enc, B: Enc> Enc for (A, B) {
	fn enc(&self, w: &mut W) {
		w.put(&self.0).put(&self.1);
	}
}

impl<T: Enc> Enc for Reverse<T> {
	fn enc(&self, w: &mut W) {
		self.0.enc(w)
	}
}

impl Enc for Rank {
	fn enc(&self, w: &mut W) {
		w.put(self.up).put(self.low);
	}
}

impl<K: Enc, V: Enc> Enc for BTreeMap<K, V> {
	fn enc(&self, w: &mut W) {
		w.int(self.len() as i128);
		self.iter().for_each(|e| e.enc(w));
	}
}

impl<K: Enc> Enc for BTreeSet<K> {
	fn enc(&self, w: &mut W) {
		w.int(self.len() as i128);
		self.iter().for_each(|k| k.enc(w));
	}
}

/// What `f` returns, `1` then the value, or `0` and its panic's message.
fn outcome<T: Enc>(w: &mut W, f: impl FnOnce() -> T) {
	match catch_unwind(AssertUnwindSafe(f)) {
		Ok(v) => w.int(1).put(v),
		Err(e) => {
			let msg = e
				.downcast_ref::<&str>()
				.map(|s| s.to_string())
				.or_else(|| e.downcast_ref::<String>().cloned())
				.unwrap_or_default();
			w.int(0).int(msg.len() as i128);
			msg.bytes().for_each(|b| _ = w.int(b.into()));
			w
		}
	};
}

fn case(rng: &mut Rng) -> W {
	let mut w = W::default();
	let kind = 1 + rng.below(13);
	w.int(kind.into());
	match kind {
		1 => {
			let (v, x) = (rng.bytes(), rng.byte());
			w.put(&v).put(x);
			outcome(&mut w, || cases::shifted(&v, x));
		}
		2 => {
			let (v, x) = (rng.bytes(), rng.byte());
			w.put(&v).put(x);
			outcome(&mut w, || cases::reaches(&v, x));
		}
		3 => {
			let (a, b) = (rng.bytes(), rng.bytes());
			w.put(&a).put(&b);
			outcome(&mut w, || cases::rising(&a, &b));
		}
		4 => {
			let v = rng.bytes();
			w.put(&v);
			outcome(&mut w, || cases::pair_sums(&v));
		}
		5 => {
			let v = rng.bytes();
			w.put(&v);
			outcome(&mut w, || cases::ends(&v).map(|(a, b, c)| (a, (b, c))));
		}
		6 => {
			let (v, n) = (rng.bytes(), rng.below(7) as usize);
			w.put(&v).put(n);
			outcome(&mut w, || {
				let (a, b, c, d) = cases::ranges(&v, n);
				(a, (b, (c, d)))
			});
		}
		7 => {
			let o = (!rng.one_in(3)).then(|| rng.byte());
			w.put(o);
			outcome(&mut w, || cases::must(o));
		}
		8 => {
			let (a, b) = (rng.byte(), rng.byte());
			w.put(a).put(b);
			outcome(&mut w, || cases::saturating(a, b));
		}
		9 => {
			let (a, b) = (rng.byte(), rng.one_in(2));
			w.put(a).put(b);
			outcome(&mut w, || cases::widen(a, b));
		}
		10 => {
			let (o, b) = ((!rng.one_in(3)).then(|| rng.byte()), rng.one_in(2));
			w.put(o).put(b);
			outcome(&mut w, || {
				let (a, b, c, d) = cases::options(o, b);
				(a, (b, (c, d)))
			});
		}
		11 => {
			let v = rng.bytes();
			w.put(&v);
			outcome(&mut w, || cases::shared(&v).to_vec());
		}
		13 => {
			let (b, o) = (rng.one_in(2), (!rng.one_in(3)).then(|| rng.byte()));
			w.put(b).put(o);
			outcome(&mut w, || cases::then_unwrap(b, o));
		}
		_ => {
			let m: BTreeMap<Rank, u16> = (0..rng.below(5)).map(|_| (rng.rank(), rng.below(1000) as u16)).collect();
			let s: BTreeSet<u8> = (0..rng.below(5)).map(|_| rng.byte()).collect();
			let (r, k) = (rng.rank(), rng.byte());
			w.put(&m).put(&s).put(r).put(k);
			outcome(&mut w, || {
				let (a, b, c, d, e) = cases::lookups(&m, &s, r, k);
				(a, (b, (c, (d, e))))
			});
		}
	}
	w
}

/// `LEAN_DIFF_OUT` names the file to write, and `LEAN_DIFF_CASES` how many cases.
#[test]
#[ignore = "run by rust2's driver/lean/compare/fixture.sh"]
fn write_cases() {
	let path = std::env::var("LEAN_DIFF_OUT").expect("LEAN_DIFF_OUT");
	let cases: u64 = std::env::var("LEAN_DIFF_CASES").map_or(100_000, |n| n.parse().expect("a count"));
	std::panic::set_hook(Box::new(|_| {}));
	let mut out = std::io::BufWriter::new(std::fs::File::create(path).expect("create"));
	let mut rng = Rng(0x9e37_79b9_7f4a_7c15);
	for _ in 0..cases {
		let w = case(&mut rng);
		let line: Vec<String> = w.0.iter().map(i128::to_string).collect();
		writeln!(out, "{}", line.join(" ")).expect("write");
	}
	let _ = std::panic::take_hook();
}
