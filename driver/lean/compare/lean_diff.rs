//! Writes differential cases for rust2-lean's kio translation: inputs and what native Rust
//! returns, as one line of integers per case, for `Diff.lean` to replay.

use std::collections::{HashSet, VecDeque};
use std::io::Write;
use std::panic::{AssertUnwindSafe, catch_unwind};

use kio_pure_machine::StateMachine;
use kio_pure_machine::check::Explore;
use kio_pure_machine::send_credit::world::configs;
use kio_pure_machine::send_credit::{Event, Mutant, Output, Session, Shape, Sub};
use kio_pure_machine::warm_settle::{self, Answer, Choice, Heard, Link, Relay, Sig};

/// An encoder of one case.
#[derive(Default)]
struct W(Vec<i64>);

trait Enc {
	fn enc(&self, w: &mut W);
}

impl W {
	fn put(&mut self, v: impl Enc) -> &mut Self {
		v.enc(self);
		self
	}
	fn int(&mut self, v: i64) -> &mut Self {
		self.0.push(v);
		self
	}
}

macro_rules! ints {
	($($t:ty),*) => {$(impl Enc for $t { fn enc(&self, w: &mut W) { w.0.push(*self as i64) } })*};
}
ints!(u8, u16, u32, usize, i16, bool);

impl<T: Enc> Enc for &T {
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
impl<T: Enc, const N: usize> Enc for [T; N] {
	fn enc(&self, w: &mut W) {
		self.iter().for_each(|v| v.enc(w));
	}
}
impl<T: Enc> Enc for Vec<T> {
	fn enc(&self, w: &mut W) {
		w.int(self.len() as i64);
		self.iter().for_each(|v| v.enc(w));
	}
}
impl<A: Enc, B: Enc> Enc for (A, B) {
	fn enc(&self, w: &mut W) {
		w.put(&self.0).put(&self.1);
	}
}
impl Enc for Mutant {
	fn enc(&self, w: &mut W) {
		w.int(*self as i64);
	}
}
impl<const S: usize> Enc for Shape<S> {
	fn enc(&self, w: &mut W) {
		w.put(self.priority).put(self.ascending).put(self.max_streams).put(self.mutant);
	}
}
impl Enc for kio_pure_machine::send_credit::Rank {
	fn enc(&self, w: &mut W) {
		w.put(self.priority).put(self.turn).put(self.in_group);
	}
}
impl Enc for kio_pure_machine::send_credit::Place {
	fn enc(&self, w: &mut W) {
		w.put(self.rank).put(self.arrival);
	}
}
impl<const G: usize> Enc for Sub<G> {
	fn enc(&self, w: &mut W) {
		w.put(self.opening).put(self.place).put(self.asked).put(self.turn).put(self.serving);
	}
}
impl<const S: usize, const G: usize> Enc for Session<S, G> {
	fn enc(&self, w: &mut W) {
		w.put(self.shape).put(self.subs).put(self.arrivals).put(self.resets).put(self.opened).put(self.freed);
	}
}
impl Enc for Event {
	fn enc(&self, w: &mut W) {
		match *self {
			Event::Poll { sub, cached, pick } => w.int(0).put(sub).put(cached).put(pick),
			Event::Finished { sub, group } => w.int(1).put(sub).put(group),
			Event::Freed => w.int(2),
		};
	}
}
impl Enc for Output {
	fn enc(&self, w: &mut W) {
		match *self {
			Output::Wake(s) => w.int(0).put(s),
			Output::Park(s) => w.int(1).put(s),
			Output::Take { sub, group } => w.int(2).put(sub).put(group),
			Output::Shed { sub, group } => w.int(3).put(sub).put(group),
			Output::Open { sub, group } => w.int(4).put(sub).put(group),
			Output::Reset { sub, group } => w.int(5).put(sub).put(group),
		};
	}
}
impl Enc for Result<(), kio_pure_machine::Disabled> {
	fn enc(&self, w: &mut W) {
		w.int(self.is_err() as i64);
	}
}
impl Enc for Sig {
	fn enc(&self, w: &mut W) {
		w.put(self.cost).put(self.path).put(self.rank);
	}
}
impl Enc for Answer {
	fn enc(&self, w: &mut W) {
		match *self {
			Answer::None => w.int(0),
			Answer::Wait => w.int(1),
			Answer::Live(s) => w.int(2).put(s),
			Answer::Lost => w.int(3),
		};
	}
}
impl Enc for Heard {
	fn enc(&self, w: &mut W) {
		w.put(self.pull).put(self.answer).put(self.advert);
	}
}
impl Enc for Link {
	fn enc(&self, w: &mut W) {
		w.put(self.open).put(self.cold).put(self.warm);
	}
}
impl Enc for Choice {
	fn enc(&self, w: &mut W) {
		w.put(self.via).put(self.cost).put(self.copy).put(self.rank);
	}
}
impl Enc for warm_settle::Pull {
	fn enc(&self, w: &mut W) {
		w.int(*self as i64);
	}
}
impl<const N: usize> Enc for Relay<N> {
	fn enc(&self, w: &mut W) {
		w.put(self.id).put(self.reader).put(self.route).put(self.links).put(self.heard).put(self.up).put(self.next).put(self.answered);
	}
}
impl Enc for warm_settle::Event {
	fn enc(&self, w: &mut W) {
		match *self {
			warm_settle::Event::Hear { from, heard } => w.int(0).put(from).put(heard),
			warm_settle::Event::React => w.int(1),
		};
	}
}

/// Native Rust's outcome: `0` and the panic's message as its length and bytes, else `1` and
/// what `f` returned.
fn outcome<T: Enc>(w: &mut W, f: impl FnOnce() -> T) {
	match catch_unwind(AssertUnwindSafe(f)) {
		Ok(v) => w.int(1).put(v),
		Err(payload) => {
			let msg = match (payload.downcast_ref::<&str>(), payload.downcast_ref::<String>()) {
				(Some(s), _) => s.to_string(),
				(_, Some(s)) => s.clone(),
				_ => panic!("a panic payload that is not a string"),
			};
			w.int(0).int(msg.len() as i64);
			for b in msg.bytes() {
				w.int(b as i64);
			}
			w
		}
	};
}

/// A deterministic xorshift, so the cases are the same on every run.
struct Rng(u64);

impl Rng {
	fn next(&mut self) -> u64 {
		self.0 ^= self.0 << 13;
		self.0 ^= self.0 >> 7;
		self.0 ^= self.0 << 17;
		self.0
	}
	fn below(&mut self, n: u64) -> u64 {
		self.next() % n
	}
	fn flip(&mut self) -> bool {
		self.below(2) == 0
	}
	/// Mostly small, sometimes near the top of the type, to reach overflow.
	fn u32(&mut self) -> u32 {
		match self.below(8) {
			0 => u32::MAX - self.below(4) as u32,
			1 => self.next() as u32,
			_ => self.below(12) as u32,
		}
	}
	fn u8(&mut self, n: u64) -> u8 {
		self.below(n) as u8
	}
	fn opt<T>(&mut self, f: impl FnOnce(&mut Self) -> T) -> Option<T> {
		if self.flip() { Some(f(self)) } else { None }
	}
	fn sig(&mut self) -> Sig {
		Sig {
			cost: self.u32(),
			path: self.next() as u16 & 0b11111,
			rank: self.u32(),
		}
	}
	fn heard(&mut self) -> Heard {
		Heard {
			pull: self.opt(Self::u32),
			answer: match self.below(4) {
				0 => Answer::None,
				1 => Answer::Wait,
				2 => Answer::Live(self.sig()),
				_ => Answer::Lost,
			},
			advert: self.opt(Self::sig),
		}
	}
	fn relay(&mut self) -> Relay<4> {
		Relay {
			id: self.u8(6),
			reader: self.opt(Self::u32),
			route: self.opt(|r| (r.u8(6), r.u32())),
			links: [(); 4].map(|_| {
				self.opt(|r| Link {
					open: r.below(4) != 0,
					cold: r.u32(),
					warm: r.u32(),
				})
			}),
			heard: [(); 4].map(|_| if self.flip() { Heard::EMPTY } else { self.heard() }),
			up: self.opt(|r| r.u8(6)),
			next: self.opt(|r| r.u8(6)),
			answered: self.next() as u16 & 0b11111,
		}
	}
	fn choice(&mut self) -> Choice {
		Choice {
			via: self.u8(6),
			cost: self.u32(),
			copy: self.flip(),
			rank: self.u32(),
		}
	}
	/// Any event, including ones out of range that make the step panic.
	fn event(&mut self) -> Event {
		match self.below(3) {
			0 => Event::Poll {
				sub: self.below(3) as usize,
				cached: self.next() as u16 & 0b1111,
				pick: self.opt(|r| r.u8(5)),
			},
			1 => Event::Finished {
				sub: self.below(3) as usize,
				group: self.u8(5),
			},
			_ => Event::Freed,
		}
	}
}

fn session_case(out: &mut impl Write, session: &Session<2, 3>, event: Event) {
	let mut w = W::default();
	w.int(1).put(session).put(event);
	let mut s = *session;
	outcome(&mut w, || {
		let mut o = Vec::new();
		let r = s.step(event, &mut o);
		(r, (s, o))
	});
	outcome(&mut w, || session.within_credit());
	outcome(&mut w, || session.well_formed());
	line(out, &w);
}

fn line(out: &mut impl Write, w: &W) {
	let text: Vec<String> = w.0.iter().map(i64::to_string).collect();
	writeln!(out, "{}", text.join(" ")).unwrap();
}

fn relay_cases(out: &mut impl Write, relay: &Relay<4>, event: warm_settle::Event) {
	let mut w = W::default();
	w.int(2).put(relay).put(event);
	let mut r = *relay;
	outcome(&mut w, || {
		let res = r.step(event, &mut Vec::new());
		(res, r)
	});
	outcome(&mut w, || relay.next_pulls());
	outcome(&mut w, || relay.options(None));
	let u = relay.up.or(relay.route.map(|r| r.0));
	w.put(u);
	outcome(&mut w, || relay.options(u));
	outcome(&mut w, || relay.mine());
	outcome(&mut w, || relay.demand());
	line(out, &w);
}

fn main() {
	std::panic::set_hook(Box::new(|_| {}));
	let budget: usize = std::env::args().nth(1).map_or(4000, |n| n.parse().unwrap());
	let mut out = std::io::BufWriter::new(std::io::stdout().lock());
	let mut rng = Rng(0x9e37_79b9_7f4a_7c15);

	// Every config's reachable states, sampled evenly to `budget` transitions each, plus an
	// arbitrary event from each sampled state.
	let runs = [
		configs::base(None),
		configs::live(None),
		configs::priority(None),
		configs::streams(None),
		configs::live(Some(Mutant::NoGiveWay)),
		configs::live(Some(Mutant::NoPreemption)),
		configs::streams(Some(Mutant::NoLeaveWake)),
	];
	for init in runs {
		let mut seen = HashSet::from([init]);
		let mut queue = VecDeque::from([init]);
		let mut steps = Vec::new();
		let mut events = Vec::new();
		while let Some(world) = queue.pop_front() {
			events.clear();
			world.events(&mut events);
			for &action in &events {
				let mut next = world;
				steps.push((world.session, world.event(action)));
				if next.step(action, &mut Vec::new()).is_ok() && seen.insert(next) {
					queue.push_back(next);
				}
			}
		}
		let stride = steps.len().div_ceil(budget).max(1);
		for (session, event) in steps.iter().step_by(stride) {
			session_case(&mut out, session, *event);
			session_case(&mut out, session, rng.event());
		}
	}
	// A fresh session from an arbitrary shape.
	for _ in 0..budget / 4 {
		let shape = Shape {
			priority: [rng.u8(3), rng.u8(3)],
			ascending: [rng.flip(), rng.flip()],
			max_streams: rng.u8(4),
			mutant: None,
		};
		let mut session = Session::<2, 3>::new(shape);
		for _ in 0..8 {
			let event = rng.event();
			session_case(&mut out, &session, event);
			let mut next = session;
			if catch_unwind(AssertUnwindSafe(|| next.step(event, &mut Vec::new()))).is_ok_and(|r| r.is_ok()) {
				session = next;
			}
		}
	}

	// Random relays, walked a few steps each.
	for _ in 0..budget {
		let mut relay = rng.relay();
		for _ in 0..4 {
			let event = if rng.below(3) == 0 {
				warm_settle::Event::React
			} else {
				warm_settle::Event::Hear {
					from: rng.u8(6),
					heard: rng.heard(),
				}
			};
			relay_cases(&mut out, &relay, event);
			let mut next = relay;
			if catch_unwind(AssertUnwindSafe(|| next.step(event, &mut Vec::new()))).is_ok_and(|r| r.is_ok()) {
				relay = next;
			}
		}
	}
	// `best` on arbitrary lists.
	for _ in 0..budget {
		let choices: Vec<Choice> = (0..rng.below(6)).map(|_| rng.choice()).collect();
		let mut w = W::default();
		w.int(3).put(&choices);
		outcome(&mut w, || warm_settle::best(&choices));
		line(&mut out, &w);
	}
}
