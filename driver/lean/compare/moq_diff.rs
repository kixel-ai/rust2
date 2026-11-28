//! Writes differential cases for rust2-lean's translation of moq-net's route algebra: inputs
//! and what native Rust returns, as one line of integers per case, for `MoqDiff.lean` to
//! replay. `moq.sh` copies it into moq-net's `route::tests` as `lean_diff`, since what it
//! calls is crate-private, and runs it with the test profile's overflow checks.
//! `moq_vectors.rs` writes the calls behind moq's route-algebra vectors with its encoder.

use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet};
use std::io::Write;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::time::Duration;

use kio::clock::Instant;

use crate::route::relay::{
	Best, Claim, Here, Hop, NodeId, Offer, Port, Role, claim_order, debounce, floor_of, order, precedence,
};

use super::*;

/// Attributes whose `lowered` panics on overflow, so the class's actions are exercised.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Gen {
	above: u64,
	epoch: u64,
}

impl Item for Gen {
	fn lowered(&self, by: u64) -> Self {
		Gen {
			above: self.above + by,
			epoch: self.epoch,
		}
	}

	fn above(&self) -> u64 {
		self.above
	}

	fn epoch(&self) -> u64 {
		self.epoch
	}
}

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

	fn pick<T: Copy>(&mut self, items: &[T]) -> T {
		items[self.below(items.len() as u64) as usize]
	}

	/// Near the ends of a `Duration`'s seconds, and where its nanoseconds carry.
	fn duration(&mut self) -> Duration {
		let secs = self.pick(&[0, 1, 2, 1 << 63, u64::MAX - 1, u64::MAX]);
		Duration::new(secs, self.pick(&[0, 1, 499_999_999, 500_000_000, 999_999_999]))
	}

	/// Near the ends of the clock's `u64` of nanoseconds.
	fn instant(&mut self) -> Instant {
		let nanos = self.pick(&[0, 1, 999_999_999, 1_000_000_000, 1 << 63, u64::MAX - 1, u64::MAX]);
		Instant::ZERO.saturating_add(Duration::from_nanos(nanos))
	}
}

/// An encoder of one case.
#[derive(Default)]
pub(super) struct W(pub Vec<i128>);

pub(super) trait Enc {
	fn enc(&self, w: &mut W);
}

impl W {
	pub fn put(&mut self, v: impl Enc) -> &mut Self {
		v.enc(self);
		self
	}

	pub fn int(&mut self, v: i128) -> &mut Self {
		self.0.push(v);
		self
	}
}

macro_rules! ints {
	($($t:ty),*) => {$(impl Enc for $t { fn enc(&self, w: &mut W) { w.0.push(*self as i128) } })*};
}
ints!(u32, u64, usize, i8, bool);

impl Enc for Duration {
	fn enc(&self, w: &mut W) {
		w.put(self.as_secs()).put(self.subsec_nanos());
	}
}

/// As its nanoseconds from the clock's origin, which a `u64` holds.
impl Enc for Instant {
	fn enc(&self, w: &mut W) {
		w.int(self.saturating_duration_since(Instant::ZERO).as_nanos() as i128);
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

macro_rules! tuples {
	($(($($t:ident $i:tt),*)),*) => {$(
		impl<$($t: Enc),*> Enc for ($($t,)*) {
			fn enc(&self, w: &mut W) {
				$(w.put(&self.$i);)*
			}
		}
	)*};
}
tuples!((A 0, B 1), (A 0, B 1, C 2), (A 0, B 1, C 2, D 3), (A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7));

impl<T: Enc> Enc for Reverse<T> {
	fn enc(&self, w: &mut W) {
		self.0.enc(w)
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

impl Enc for NodeId {
	fn enc(&self, w: &mut W) {
		w.put(self.relay).put(self.marked);
	}
}

impl Enc for Hop {
	fn enc(&self, w: &mut W) {
		w.put(self.node).put(self.session);
	}
}

impl Enc for Offer {
	fn enc(&self, w: &mut W) {
		w.put(&*self.path)
			.put(self.draining)
			.put(self.priority)
			.put(self.cost)
			.put(self.marked);
	}
}

impl Enc for Port {
	fn enc(&self, w: &mut W) {
		w.put(self.cost).put(self.priority).put(self.upstream);
		match &self.role {
			Role::Routing(id) => w.int(0).put(id),
			Role::Scoped(paths) => w.int(1).int(paths.len() as i128),
			Role::Endpoint => w.int(2),
		};
	}
}

impl Enc for Gen {
	fn enc(&self, w: &mut W) {
		w.put(self.above).put(self.epoch);
	}
}

impl<A: Enc> Enc for Claim<A> {
	fn enc(&self, w: &mut W) {
		w.put(&self.item)
			.put(self.cost)
			.put(self.priority)
			.put(self.draining)
			.put(self.unavailable)
			.put(&self.chain)
			.put(self.marked);
	}
}

impl<N: Enc> Enc for Best<N> {
	fn enc(&self, w: &mut W) {
		w.put(&self.via).put(&self.offer);
	}
}

/// Integers where the algebra's sums and bounds turn: small, near the cost cap, and near
/// the top of the type.
const COSTS: [u64; 8] = [0, 1, 2, 7, MAX_COST - 1, MAX_COST, u64::MAX - 1, u64::MAX];
const PRIORITIES: [i8; 6] = [i8::MIN, -1, 0, 1, 5, i8::MAX];

fn node(rng: &mut Rng) -> NodeId {
	NodeId {
		relay: rng.below(6),
		marked: rng.one_in(4),
	}
}

fn hop(rng: &mut Rng) -> Hop {
	Hop {
		node: node(rng),
		session: match rng.one_in(10) {
			true => u64::MAX,
			false => rng.below(4),
		},
	}
}

/// A length around `MAX_HOPS` now and then, else short, sometimes empty.
fn len(rng: &mut Rng) -> usize {
	match rng.below(10) {
		0 => MAX_HOPS - 2 + rng.below(4) as usize,
		1 => 0,
		_ => 1 + rng.below(5) as usize,
	}
}

fn offer(rng: &mut Rng) -> Offer {
	let n = len(rng);
	Offer {
		path: (0..n).map(|_| hop(rng)).collect(),
		draining: rng.one_in(4),
		priority: rng.pick(&PRIORITIES),
		cost: rng.pick(&COSTS),
		marked: rng.one_in(3),
	}
}

fn port(rng: &mut Rng) -> Port {
	Port {
		cost: rng.pick(&COSTS),
		priority: rng.pick(&PRIORITIES),
		upstream: rng.one_in(3),
		role: match rng.below(3) {
			0 => Role::Routing(node(rng)),
			1 => Role::Scoped(Vec::new()),
			_ => Role::Endpoint,
		},
	}
}

/// What a node holds, as [`Here`] borrows it.
struct State {
	closed: BTreeMap<(Id, Id), u64>,
	dead: BTreeMap<Hop, NodeId>,
	draining: BTreeSet<NodeId>,
}

fn state(rng: &mut Rng) -> State {
	let mut state = State {
		closed: BTreeMap::new(),
		dead: BTreeMap::new(),
		draining: BTreeSet::new(),
	};
	for _ in 0..rng.below(5) {
		state.closed.insert((rng.below(6), rng.below(6)), rng.below(4));
	}
	for _ in 0..rng.below(3) {
		state.dead.insert(hop(rng), node(rng));
	}
	for _ in 0..rng.below(3) {
		state.draining.insert(node(rng));
	}
	state
}

impl Enc for State {
	fn enc(&self, w: &mut W) {
		w.put(&self.closed).put(&self.dead).put(&self.draining);
	}
}

/// A chain, now and then naming a relay twice or reaching the longest path.
fn chain(rng: &mut Rng) -> Vec<Id> {
	(0..len(rng)).map(|_| rng.below(8)).collect()
}

fn claim(rng: &mut Rng) -> Claim<Gen> {
	Claim {
		item: Gen {
			above: rng.pick(&COSTS),
			epoch: rng.pick(&COSTS),
		},
		cost: rng.pick(&COSTS),
		priority: rng.pick(&PRIORITIES),
		draining: rng.one_in(4),
		unavailable: rng.one_in(4),
		chain: chain(rng),
		marked: rng.one_in(3),
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
			let (me, origin, upstream) = (node(rng), node(rng), rng.one_in(2));
			w.put(me).put(origin).put(upstream);
			outcome(&mut w, || Offer::own(me, origin, upstream));
		}
		2 => {
			let (o, me, session, to, upstream) = (offer(rng), node(rng), rng.pick(&COSTS), node(rng), rng.one_in(2));
			w.put(&o).put(me).put(session).put(to).put(upstream);
			outcome(&mut w, || o.onward(me, session, to, upstream));
		}
		3 => {
			let (s, p, draining, o) = (state(rng), port(rng), rng.one_in(3), offer(rng));
			w.put(&s).put(&p).put(draining).put(&o);
			let here = Here {
				closed: &s.closed,
				dead: &s.dead,
				draining: &s.draining,
			};
			outcome(&mut w, || here.arrive(&p, draining, &o));
		}
		4 => {
			let best = Best {
				via: rng.below(4),
				offer: offer(rng),
			};
			let current = (!rng.one_in(3)).then(|| rng.below(4));
			w.put(&best).put(current);
			outcome(&mut w, || order(&best, current));
		}
		5 => {
			let (c, o) = (claim(rng), offer(rng));
			w.put(&c).put(&o);
			outcome(&mut w, || c.over(&o));
		}
		6 => {
			let (c, me, forwards, upstream) = (claim(rng), rng.below(8), !rng.one_in(4), rng.one_in(2));
			w.put(&c).put(me).put(forwards).put(upstream);
			outcome(&mut w, || c.exported(me, forwards, upstream));
		}
		7 => {
			let (c, me, forwards, p) = (claim(rng), rng.below(8), !rng.one_in(4), port(rng));
			w.put(&c).put(me).put(forwards).put(&p);
			outcome(&mut w, || c.adopted(me, forwards, &p));
		}
		8 => {
			let (c, by) = (claim(rng), rng.pick(&COSTS));
			w.put(&c).put(by);
			outcome(&mut w, || c.lowered(by));
		}
		9 => {
			let (c, o, origin) = (claim(rng), (!rng.one_in(3)).then(|| offer(rng)), node(rng));
			w.put(&c).put(&o).put(origin);
			outcome(&mut w, || claim_order(&c, o.as_ref(), origin));
		}
		10 => {
			let c = claim(rng);
			w.put(&c.item);
			outcome(&mut w, || precedence(&c.item));
		}
		12 => {
			let d = rng.duration();
			w.put(d);
			outcome(&mut w, || debounce(d));
		}
		// kio's own `Instant` methods, which `moq.sh` translates from kio.
		13 => {
			let (i, d, j) = (rng.instant(), rng.duration(), rng.instant());
			w.put(i).put(d).put(j);
			outcome(&mut w, || {
				(
					i.checked_add(d),
					i.checked_sub(d),
					i.saturating_add(d),
					i.saturating_sub(d),
					i.checked_duration_since(j),
					i.saturating_duration_since(j),
					i.duration_since(j),
					i - j,
				)
			});
		}
		_ => {
			let (h, far) = (hop(rng), node(rng));
			w.put(h).put(far);
			outcome(&mut w, || floor_of(h, far));
		}
	}
	w
}

/// `LEAN_DIFF_OUT` names the file to write, and `LEAN_DIFF_CASES` how many cases.
#[test]
#[ignore = "run by rust2's driver/lean/compare/moq.sh"]
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
