//! Writes differential cases for rust2-lean's translation of the fixture's `cases`, the
//! models `MoqDiff.lean` does not reach: inputs and what native Rust returns, one line of
//! integers per case, for `FixtureDiff.lean` to replay. Run by `compare/fixture.sh`.

use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::io::Write;
use std::panic::{AssertUnwindSafe, catch_unwind};

use std::time::Duration;

use fixture::cases::lenses::{Book, Seeded, Tally};
use fixture::cases::{self, Ledger, Rank, Step, text};
use kio::clock::instant::Instant;

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

	/// Counts near the ends of a `u64`, and near where milliseconds and nanoseconds carry.
	fn count(&mut self) -> u64 {
		[0, 1, 999, 1000, 999_999_999, 1_000_000_000, 1 << 63, u64::MAX - 1, u64::MAX][self.below(9) as usize]
	}

	fn duration(&mut self) -> Duration {
		let secs = [0, 1, 2, 1 << 63, u64::MAX - 1, u64::MAX][self.below(6) as usize];
		let nanos = [0, 1, 499_999_999, 500_000_000, 999_999_999][self.below(5) as usize];
		Duration::new(secs, nanos)
	}

	fn ledger(&mut self) -> Ledger {
		Ledger {
			m: (0..self.below(4)).map(|_| (self.rank(), self.below(1000) as u16)).collect(),
			s: (0..self.below(4)).map(|_| self.byte()).collect(),
			q: (0..self.below(3)).map(|_| self.byte()).collect(),
		}
	}

	/// `u16`s near its ends, where a write through a `&mut` overflows.
	fn wide(&mut self) -> u16 {
		[0, 1, 7, 100, u16::MAX - 1, u16::MAX][self.below(6) as usize]
	}

	/// Keys from a few, so a lookup both finds one and misses.
	fn key(&mut self) -> u8 {
		self.below(4) as u8
	}

	/// A character of each UTF-8 width, a `/` and a `\\`, and a combining one and an unassigned
	/// one, whose `Debug` is an escape.
	fn char(&mut self) -> char {
		['a', '/', '\\', 'é', '\u{301}', '日', '😀', '\u{10ffff}'][self.below(8) as usize]
	}

	fn text(&mut self) -> String {
		(0..self.below(5)).map(|_| self.char()).collect()
	}

	/// A byte index into `s`, past its end now and then.
	fn index(&mut self, s: &str) -> usize {
		self.below(s.len() as u64 + 3) as usize
	}

	fn book(&mut self) -> Book {
		Book {
			tallies: (0..self.below(4))
				.map(|_| (self.key(), Tally { hits: self.wide(), last: self.byte() }))
				.collect(),
			nested: (0..self.below(3))
				.map(|_| (self.key(), (0..self.below(3)).map(|_| (self.key(), self.wide())).collect()))
				.collect(),
			seeds: (0..self.below(3)).map(|_| (self.key(), Seeded(self.wide()))).collect(),
			log: (!self.one_in(3)).then(|| self.bytes()),
		}
	}

	/// Near the ends of a `u128`, and of the `u64` within it.
	fn huge(&mut self) -> u128 {
		let picks = [0, 1, 2, u64::MAX as u128, 1 << 64, (1 << 127) - 1, 1 << 127, u128::MAX - 1, u128::MAX];
		picks[self.below(picks.len() as u64) as usize]
	}

	/// Near the ends of an `i128`, and of the `i64` within it.
	fn signed(&mut self) -> i128 {
		let picks = [i128::MIN, i128::MIN + 1, i64::MIN as i128, -2, -1, 0, 1, 2, i64::MAX as i128 + 1, i128::MAX];
		picks[self.below(picks.len() as u64) as usize]
	}

	/// Shift amounts, some past a `u128`'s width before the `% 128` that the cases take.
	fn shift(&mut self) -> u32 {
		[0, 1, 63, 64, 127, 128, 200][self.below(7) as usize]
	}

	fn rank(&mut self) -> Rank {
		Rank {
			up: self.one_in(2),
			low: Reverse([i8::MIN, -1, 0, 1, i8::MAX][self.below(5) as usize]),
		}
	}
}

/// An encoder of one case, each integer in decimal, which a `u128` past `i128` needs.
#[derive(Default)]
struct W(Vec<String>);

trait Enc {
	fn enc(&self, w: &mut W);
}

impl W {
	fn put(&mut self, v: impl Enc) -> &mut Self {
		v.enc(self);
		self
	}

	fn int(&mut self, v: i128) -> &mut Self {
		self.0.push(v.to_string());
		self
	}
}

macro_rules! ints {
	($($t:ty),*) => {$(impl Enc for $t { fn enc(&self, w: &mut W) { w.0.push((*self as i128).to_string()) } })*};
}
ints!(u8, u16, u32, u64, usize, i8, i64, i128, bool);

impl Enc for char {
	fn enc(&self, w: &mut W) {
		w.put(*self as u32);
	}
}

/// Its length, then its bytes.
impl Enc for str {
	fn enc(&self, w: &mut W) {
		w.int(self.len() as i128);
		self.bytes().for_each(|b| _ = w.put(b));
	}
}

impl Enc for u128 {
	fn enc(&self, w: &mut W) {
		w.0.push(self.to_string());
	}
}

impl Enc for Instant {
	fn enc(&self, w: &mut W) {
		w.put(self.0);
	}
}

impl Enc for Step {
	fn enc(&self, w: &mut W) {
		match self {
			Step::Back => w.int(0),
			Step::Ahead(n) => w.int(1).put(*n),
		};
	}
}

impl Enc for Duration {
	fn enc(&self, w: &mut W) {
		w.put(self.as_secs()).put(self.subsec_nanos());
	}
}

impl Enc for std::cmp::Ordering {
	fn enc(&self, w: &mut W) {
		w.int(*self as i128);
	}
}

impl<T: Enc> Enc for VecDeque<T> {
	fn enc(&self, w: &mut W) {
		w.int(self.len() as i128);
		self.iter().for_each(|v| v.enc(w));
	}
}

impl Enc for Ledger {
	fn enc(&self, w: &mut W) {
		w.put(&self.m).put(&self.s).put(&self.q);
	}
}

impl Enc for Tally {
	fn enc(&self, w: &mut W) {
		w.put(self.hits).put(self.last);
	}
}

impl Enc for Seeded {
	fn enc(&self, w: &mut W) {
		w.put(self.0);
	}
}

impl Enc for Book {
	fn enc(&self, w: &mut W) {
		w.put(&self.tallies).put(&self.nested).put(&self.seeds).put(&self.log);
	}
}

impl<T: Enc, E: Enc> Enc for Result<T, E> {
	fn enc(&self, w: &mut W) {
		match self {
			Ok(v) => w.int(1).put(v),
			Err(e) => w.int(0).put(e),
		};
	}
}

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
	let kind = 1 + rng.below(43);
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
		14 => {
			let (ms, ns, s) = (rng.count(), rng.count(), rng.count());
			w.put(ms).put(ns).put(s);
			outcome(&mut w, || {
				let (a, b, c, d, e, f) = cases::spans(ms, ns, s);
				(a, (b, (c, (d, (e, f)))))
			});
		}
		15 => {
			let (a, b) = (rng.duration(), rng.duration());
			let k = [0, 1, 2, 3, u32::MAX][rng.below(5) as usize];
			w.put(a).put(b).put(k);
			outcome(&mut w, || {
				let (c, d, e, f, g, h) = cases::sums(a, b, k);
				(c, (d, (e, (f, (g, h)))))
			});
		}
		16 => {
			let (a, b) = (rng.duration(), rng.duration());
			let (p, q) = ((rng.byte() % 2, rng.rank()), (rng.byte() % 2, rng.rank()));
			w.put(a).put(b).put(p).put(q);
			outcome(&mut w, || {
				let (c, d, e, f, g, h, i, j, k) = cases::compared(a, b, p, q);
				(c, (d, (e, (f, (g, (h, (i, (j, k))))))))
			});
		}
		17 => {
			let mut l = rng.ledger();
			// A key the map may hold already, so `insert` replaces and `remove` finds it.
			let r = l.m.keys().next().copied().filter(|_| rng.one_in(2)).unwrap_or_else(|| rng.rank());
			let (v, k) = (rng.below(1000) as u16, rng.byte());
			w.put(&l).put(r).put(v).put(k);
			outcome(&mut w, || {
				let (a, b, c, d, e, f) = l.edit(r, v, k);
				((a, (b, (c, (d, (e, f))))), l)
			});
		}
		18 => {
			let mut l = rng.ledger();
			let x = rng.byte();
			w.put(&l).put(x);
			outcome(&mut w, || {
				let (a, b, c) = l.queue(x);
				((a, (b, c)), l)
			});
		}
		19 => {
			let l = rng.ledger();
			let r = l.m.keys().last().copied().filter(|_| !rng.one_in(3)).unwrap_or_else(|| rng.rank());
			w.put(&l).put(r);
			outcome(&mut w, || {
				let (a, b, c, d, e, f, g, h) = cases::peeks(&l, r);
				(a, (b, (c, (d, (e, (f, (g, h)))))))
			});
		}
		20 => {
			let mut b = rng.book();
			let (k, by) = (rng.key(), rng.wide());
			w.put(&b).put(k).put(by);
			outcome(&mut w, || (b.bump(k, by), b));
		}
		21 => {
			let mut b = rng.book();
			let (k, j) = (rng.key(), rng.key());
			w.put(&b).put(k).put(j);
			outcome(&mut w, || (b.touch(k, j), b));
		}
		22 => {
			let mut b = rng.book();
			let k = rng.key();
			w.put(&b).put(k);
			outcome(&mut w, || (b.moved_key(k), b));
		}
		23 => {
			let mut b = rng.book();
			let (k, j) = (rng.key(), rng.key());
			w.put(&b).put(k).put(j);
			outcome(&mut w, || (b.count(k, j), b));
		}
		24 => {
			let mut b = rng.book();
			let x = rng.byte();
			w.put(&b).put(x);
			outcome(&mut w, || {
				let (taken, old, fresh) = b.logs(x);
				((taken, (old, fresh)), b)
			});
		}
		25 => {
			let mut b = rng.book();
			let (k, f, named) = (rng.key(), (!rng.one_in(3)).then(|| rng.wide()), !rng.one_in(3));
			w.put(&b).put(k).put(f).put(named);
			outcome(&mut w, || (b.eager(k, f, named), b));
		}
		26 => {
			let (i, d, j) = (Instant(rng.count()), rng.duration(), rng.count());
			w.put(i).put(d).put(j);
			outcome(&mut w, || {
				let (a, b, c, e) = cases::ticks(i, d, j);
				(a, (b, (c, e)))
			});
		}
		27 => {
			let o = (!rng.one_in(3)).then(|| rng.wide());
			let n = (!rng.one_in(3)).then(|| rng.count());
			w.put(o).put(n);
			outcome(&mut w, || {
				let (a, b, c, d, e, f) = cases::passed(o, n);
				(a, (b, (c, (d, (e, f)))))
			});
		}
		28 => {
			let d = rng.duration();
			w.put(d);
			outcome(&mut w, || {
				let (a, b, c) = cases::nanos(d);
				(a, (b, c))
			});
		}
		29 => {
			let (a, b, s) = (rng.huge(), rng.huge(), rng.shift());
			w.put(a).put(b).put(s);
			outcome(&mut w, || {
				let (c, d, e, f, g, h, i) = cases::wide_bits(a, b, s);
				(c, (d, (e, (f, (g, (h, i))))))
			});
		}
		30 => {
			let (a, b) = (rng.huge(), rng.huge());
			w.put(a).put(b);
			outcome(&mut w, || {
				let (c, d, e, f, g, h, i, j) = cases::wide_order(a, b);
				(c, (d, (e, (f, (g, (h, (i, j)))))))
			});
		}
		31 => {
			let (a, b, op) = (rng.huge(), rng.huge(), rng.byte());
			w.put(a).put(b).put(op);
			outcome(&mut w, || cases::wide_arith(a, b, op));
		}
		32 => {
			let (x, y, op) = (rng.signed(), rng.signed(), rng.byte());
			w.put(x).put(y).put(op);
			outcome(&mut w, || cases::signed_arith(x, y, op));
		}
		33 => {
			let (x, y, s) = (rng.signed(), rng.signed(), rng.shift());
			w.put(x).put(y).put(s);
			outcome(&mut w, || {
				let (c, d, e, f, g, h, i, j) = cases::signed_bits(x, y, s);
				(c, (d, (e, (f, (g, (h, (i, j)))))))
			});
		}
		34 => {
			let (a, x) = (rng.huge(), rng.signed());
			w.put(a).put(x);
			outcome(&mut w, || {
				let (k, l, m, n, o, p, q, r, t, u) = cases::wide_casts(a, x);
				(k, (l, (m, (n, (o, (p, (q, (r, (t, u)))))))))
			});
		}
		35 => {
			let s = rng.text();
			w.put(s.as_str());
			outcome(&mut w, || {
				let (n, e, b) = text::measure(&s);
				(n, (e, b))
			});
		}
		36 => {
			let s = rng.text();
			let (b, e, how) = (rng.index(&s), rng.index(&s), rng.byte());
			w.put(s.as_str()).put(b).put(e).put(how);
			outcome(&mut w, || text::cut(&s, b, e, how));
		}
		37 => {
			let (s, c, p) = (rng.text(), rng.char(), rng.text());
			let i = rng.index(&s);
			w.put(s.as_str()).put(c).put(p.as_str()).put(i);
			outcome(&mut w, || {
				let (f, a, b, g) = text::search(&s, c, &p, i);
				(f, (a, (b, g)))
			});
		}
		38 => {
			let (v, x) = (rng.bytes(), rng.byte());
			w.put(&v).put(x);
			outcome(&mut w, || text::tails(&v, x));
		}
		39 => {
			let (v, x) = (rng.bytes(), rng.byte());
			w.put(&v).put(x);
			outcome(&mut w, || (text::last_bumped(&v), text::find_bumped(&v, x)));
		}
		40 => {
			let (n, v, o) = (rng.byte(), rng.bytes(), (!rng.one_in(3)).then(|| rng.byte()));
			let m: BTreeMap<u8, u16> = (0..rng.below(4))
				.map(|_| (rng.byte(), rng.below(1000) as u16))
				.collect();
			let s: BTreeSet<u8> = (0..rng.below(4)).map(|_| rng.byte()).collect();
			let q: VecDeque<u8> = (0..rng.below(4)).map(|_| rng.byte()).collect();
			w.put(n).put(&v).put(&m).put(&s).put(o).put(&q);
			outcome(&mut w, || {
				let (a, b, c, d, e) = text::loops(n, v, &m, s, o, &q);
				(a, (b, (c, (d, e))))
			});
		}
		41 => {
			let n = rng.byte();
			w.put(n);
			outcome(&mut w, || text::countdown(n));
		}
		42 => {
			let c = rng.char();
			w.put(c);
			outcome(&mut w, || {
				let (k, a, b, d, e, f) = text::classify(c);
				(k, (a, (b, (d, (e, f)))))
			});
		}
		43 => {
			let (s, b) = (rng.text(), rng.byte());
			w.put(s.as_str()).put(b);
			outcome(&mut w, || text::shadowed(&s, b, (), ()));
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

/// The non-ASCII characters whose `Debug` is an escape, as half-open ranges of code points,
/// a surrogate joining whichever run is around it, for the runtime's `debugEscaped`.
fn debug_escaped() -> Vec<String> {
	let mut ranges = Vec::new();
	let mut start = None;
	for u in 0x80..=0x11_0000u32 {
		let escaped = match char::from_u32(u) {
			Some(c) => format!("{c:?}").starts_with("'\\"),
			None => start.is_some() && u < 0x11_0000,
		};
		match (escaped, start) {
			(true, None) => start = Some(u),
			(false, Some(s)) => {
				ranges.push(s);
				ranges.push(u);
				start = None;
			}
			_ => {}
		}
	}
	let mut out = vec![(ranges.len() / 2).to_string()];
	out.extend(ranges.iter().map(u32::to_string));
	out
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
	writeln!(out, "0 {}", debug_escaped().join(" ")).expect("write");
	for _ in 0..cases {
		let w = case(&mut rng);
		writeln!(out, "{}", w.0.join(" ")).expect("write");
	}
	let _ = std::panic::take_hook();
}
