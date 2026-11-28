//! What rust2-lean translates and refuses. `Session` holds the constructs Aeneas refused in
//! kixel-ai/kio#9, written as kio first wrote them.

#![feature(allocator_api)]

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Sub {
	pub opening: Option<u8>,
	pub turn: u8,
}

pub struct Session<const S: usize> {
	pub subs: [Sub; S],
	pub out: Vec<u8>,
}

impl<const S: usize> Session<S> {
	/// Matching an enum read in place through an array.
	pub fn opening(&self, s: usize) -> u8 {
		let Some(k) = self.subs[s].opening else {
			return 0;
		};
		k
	}

	/// A `while` loop reading `self.subs[s].field`.
	pub fn turns(&self) -> u16 {
		let mut total = 0u16;
		let mut s = 0;
		while s < S {
			total += self.subs[s].turn as u16;
			s += 1;
		}
		total
	}

	/// `Option<T> == Option<T>`.
	pub fn same(&self, s: usize, t: usize) -> bool {
		self.subs[s].opening == self.subs[t].opening
	}

	/// `?` on `Option`.
	pub fn next(&self, s: usize) -> Option<u8> {
		let k = self.subs[s].opening?;
		k.checked_add(1)
	}

	/// `Vec::truncate`, undoing a partial step.
	pub fn step(&mut self, s: usize) -> bool {
		let len = self.out.len();
		self.out.push(self.subs[s].turn);
		if self.subs[s].opening.is_none() {
			self.out.truncate(len);
			return false;
		}
		self.subs[s].turn += 1;
		true
	}
}

/// Constructs a translation can get subtly wrong.
pub mod cases {
	/// A tuple struct, whose field Lean names `x0`.
	#[derive(Clone, Copy, PartialEq)]
	pub struct Cost(pub u64);

	impl Cost {
		pub fn value(self) -> u64 {
			self.0
		}

		/// Named as Lean names the field, so it gets a `'`.
		pub fn x0(self) -> u64 {
			self.0
		}
	}

	/// A derived `Clone`, which copies.
	pub fn dup(c: &Cost) -> Cost {
		Clone::clone(c)
	}

	pub struct Record(pub u8);

	/// Clones the reference, not the `Record`, which has no `Clone`.
	pub fn dup_ref(r: &Record) -> &Record {
		Clone::clone(&r)
	}

	fn set(x: &mut u8) {
		*x = 42;
	}

	/// As `snapshot`, but the block writes `x` through a mutable borrow.
	pub fn snapshot_borrow(mut x: u8) -> (u8, u8) {
		(x, {
			set(&mut x);
			x
		})
	}

	/// `{ x }` is a copy, so writing through its borrow leaves `x` as it was.
	pub fn temp_borrow(x: u8) -> u8 {
		set(&mut { x });
		x
	}

	/// A `Vec`'s clone copies its elements, whatever its allocator.
	pub fn clone_bytes(v: &Vec<u8>) -> Vec<u8> {
		v.clone()
	}

	/// A type parameter's `clone` copies where every type the crate passes for it does.
	pub fn dup_generic<T: Clone>(t: &T) -> T {
		t.clone()
	}

	pub fn dup_pair(x: u8) -> (u8, u8) {
		(dup_generic(&x), dup_generic(&x))
	}

	/// Rust unwraps `o` whatever `b` is.
	pub fn then_unwrap(b: bool, o: Option<u8>) -> Option<u8> {
		b.then_some(o.unwrap())
	}

	/// A type that appears twice is repeated, not recursive.
	pub fn clone_pairs(v: &Vec<(Option<u8>, Option<u8>)>) -> Vec<(Option<u8>, Option<u8>)> {
		v.clone()
	}

	#[derive(Copy)]
	pub struct Plain(pub u8);

	impl Clone for Plain {
		fn clone(&self) -> Self {
			*self
		}
	}

	/// `#[derive(Clone, Copy)]` copies `Plain` without calling its hand-written `Clone`.
	#[derive(Clone, Copy)]
	pub struct Wrap(pub Plain);

	pub fn dup_wrap(w: &Wrap) -> Wrap {
		w.clone()
	}

	/// The tuple's first element is `x` as it was before the block sets it.
	pub fn snapshot(mut x: u8) -> (u8, u8) {
		(x, {
			x = 42;
			x
		})
	}

	pub trait Machine {
		fn step(&self) -> u8;
	}

	pub struct M(pub u8);

	impl M {
		pub fn step(&self) -> u8 {
			self.0
		}
	}

	impl Machine for M {
		fn step(&self) -> u8 {
			if self.0 == 0 { 1 } else { 0 }
		}
	}

	/// The inherent `step`, then the trait's.
	pub fn both(m: &M) -> (u8, u8) {
		(m.step(), Machine::step(m))
	}

	pub struct Pair<const A: usize, const B: usize>(pub [u8; A], pub [u8; B]);

	impl<const A: usize, const B: usize> Machine for Pair<A, B> {
		fn step(&self) -> u8 {
			0
		}
	}

	/// A trait call that resolves to an impl with more generic parameters than the trait.
	pub fn pair_step(p: &Pair<2, 3>) -> u8 {
		Machine::step(p)
	}

	pub fn wait() -> u8 {
		loop {}
	}

	/// Each `+` may overflow, so the list holds actions, and one panics only when
	/// `collect` reaches it.
	pub fn shifted(v: &[u8], x: u8) -> Vec<u8> {
		v.iter().map(|y| y + x).collect()
	}

	/// `any` stops at the first match, so a later item's overflow never panics.
	pub fn reaches(v: &[u8], x: u8) -> bool {
		v.iter().map(|y| y + x).any(|y| y == 0)
	}

	/// `all` over items that cannot panic, chained, copied and enumerated.
	pub fn rising(a: &[u8], b: &[u8]) -> bool {
		a.iter().chain(b.iter()).copied().enumerate().all(|(i, y)| i <= y as usize)
	}

	/// Each overlapping pair's sum.
	pub fn pair_sums(v: &[u8]) -> Vec<u16> {
		v.windows(2).map(|w| w[0] as u16 + w[1] as u16).collect()
	}

	/// The first and last items, by slice pattern and by method.
	pub fn ends(v: &[u8]) -> Option<(u8, u8, u8)> {
		match v {
			[first, .., last] => Some((*first, *last, *v.last()?)),
			_ => v.first().map(|f| (*f, *f, *f)),
		}
	}

	/// Each kind of range a slice is indexed by, which panics as Rust's does.
	pub fn ranges(v: &[u8], n: usize) -> (usize, usize, usize, usize) {
		(v[..n].len(), v[n..].len(), v[1..n].len(), v[..].len())
	}

	/// A panic message that Lean's string literal escapes.
	pub fn must(o: Option<u8>) -> u8 {
		o.expect("no \"value\"\n")
	}

	pub fn saturating(a: u8, b: u8) -> (u8, u8) {
		(a.saturating_add(b), a.saturating_sub(b))
	}

	pub fn widen(a: u8, b: bool) -> (u32, u8) {
		(u32::from(a), u8::from(b))
	}

	pub fn options(o: Option<u8>, b: bool) -> (bool, Option<u16>, u8, Option<u8>) {
		(o.is_some_and(|x| x > 1), o.map(|x| x as u16 * 2), o.unwrap_or(7), b.then_some(3))
	}

	/// `Reverse` flips a derived order, compared field by field.
	#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
	pub struct Rank {
		pub up: bool,
		pub low: std::cmp::Reverse<i8>,
	}

	pub fn ranked(r: Rank) -> (bool, std::cmp::Reverse<i8>) {
		(r.up, r.low)
	}

	/// An `Arc` is the value it shares.
	pub fn shared(v: &[u8]) -> std::sync::Arc<[u8]> {
		let a = std::sync::Arc::new(v.iter().copied().collect::<Vec<u8>>());
		a.iter().copied().collect()
	}

	/// Lookups in a sorted map and set, keyed by a derived order.
	pub fn lookups(
		m: &std::collections::BTreeMap<Rank, u16>,
		s: &std::collections::BTreeSet<u8>,
		r: Rank,
		k: u8,
	) -> (Option<u16>, bool, bool, usize, bool) {
		(m.get(&r).copied(), m.contains_key(&r), s.contains(&k), s.len(), m.is_empty())
	}

	/// Rust orders `Severity` by discriminant, against declaration order, so its Lean type
	/// gets no `Ord` to disagree with; `len` needs none.
	#[derive(PartialEq, Eq, PartialOrd, Ord)]
	pub enum Severity {
		High = 2,
		Low = 1,
	}

	pub fn severities(s: &std::collections::BTreeSet<Severity>) -> usize {
		s.len()
	}

	/// A local trait, which a type parameter's bound calls.
	pub trait Weight {
		fn weight(&self) -> u8;
	}

	pub fn heavier<W: Weight>(a: &W, b: &W) -> bool {
		a.weight() > b.weight()
	}

	use std::collections::{BTreeMap, BTreeSet, VecDeque};
	use std::time::Duration;

	/// A `Duration` constant, which Lean builds from its seconds and nanoseconds.
	const TICK: Duration = Duration::from_millis(1500);

	pub fn spans(ms: u64, ns: u64, s: u64) -> (Duration, Duration, Duration, Duration, Duration, Duration) {
		(
			Duration::from_millis(ms),
			Duration::from_nanos(ns),
			Duration::from_secs(s),
			Duration::ZERO,
			Duration::MAX,
			TICK,
		)
	}

	/// `Duration`'s checked and saturating arithmetic, which overflows at its seconds' `u64`.
	pub fn sums(
		a: Duration,
		b: Duration,
		k: u32,
	) -> (Option<Duration>, Option<Duration>, Option<Duration>, Duration, Duration, Duration) {
		(
			a.checked_add(b),
			a.checked_sub(b),
			a.checked_mul(k),
			a.saturating_add(b),
			a.saturating_sub(b),
			a.saturating_mul(k),
		)
	}

	/// Comparisons through `PartialOrd` and `Ord` on derived orders: a `Duration`'s, and a
	/// tuple's, which compares field by field.
	pub fn compared(
		a: Duration,
		b: Duration,
		p: (u8, Rank),
		q: (u8, Rank),
	) -> (bool, bool, bool, bool, Duration, Duration, std::cmp::Ordering, bool, bool) {
		(a < b, a <= b, a > b, a >= b, a.max(b), a.min(b), a.cmp(&b), p < q, p >= q)
	}

	/// A dependency's code, translated from its MIR, and its generic function instantiated
	/// with its own type, from a crate further down.
	pub fn ticks(i: kio::clock::instant::Instant, d: Duration, j: u64) -> (Option<u64>, u64, Duration, u64) {
		let t = fixture_dep::Ticks::of(i);
		let u = fixture_dep::Ticks::of(kio::clock::instant::Instant(j));
		(
			t.checked_add(d).map(fixture_dep::Ticks::get),
			t.saturating_add(d).get(),
			t.since(u),
			fixture_dep::larger(t, u).get(),
		)
	}

	#[derive(Clone, Copy, PartialEq, Eq, Debug)]
	pub enum Step {
		Back,
		Ahead(u16),
	}

	/// Functions passed where a closure is taken: constructors, and a `std` function.
	pub fn passed(
		o: Option<u16>,
		n: Option<u64>,
	) -> (Option<lenses::Seeded>, Option<Duration>, Option<Option<u16>>, Option<Result<u16, u8>>, Option<Step>) {
		(o.map(lenses::Seeded), n.map(Duration::from_nanos), o.map(Some), o.map(Ok), o.map(Step::Ahead))
	}

	/// `Duration` in `u128`s.
	pub fn nanos(d: Duration) -> (u128, u128, u128) {
		(d.as_nanos(), d.as_micros(), d.as_millis())
	}

	/// `u128`'s bitwise operations, and shifts by an amount of another type.
	pub fn wide_bits(a: u128, b: u128, s: u32) -> (u128, u128, u128, u128, u128, u128) {
		(a & b, a | b, a ^ b, !a, a << (s % 128), a >> (s % 128))
	}

	pub fn wide_order(a: u128, b: u128) -> (Option<u128>, Option<u128>, u128, u128, u128, u128, bool, std::cmp::Ordering) {
		(
			a.checked_add(b),
			a.checked_sub(b),
			a.saturating_add(b),
			a.saturating_sub(b),
			a.min(b),
			a.max(b),
			a < b,
			a.cmp(&b),
		)
	}

	/// Arithmetic whose overflow, or division by zero, panics.
	pub fn wide_arith(a: u128, b: u128, op: u8) -> u128 {
		match op % 5 {
			0 => a + b,
			1 => a - b,
			2 => a * b,
			3 => a / b,
			_ => a % b,
		}
	}

	pub fn signed_arith(x: i128, y: i128, op: u8) -> i128 {
		match op % 6 {
			0 => x + y,
			1 => x - y,
			2 => x * y,
			3 => x / y,
			4 => x % y,
			_ => -x,
		}
	}

	/// An arithmetic shift, and `i128`'s order.
	pub fn signed_bits(x: i128, y: i128, s: u32) -> (i128, i128, i128, i128, bool, std::cmp::Ordering, i128, i128) {
		(
			x >> (s % 128),
			x << (s % 128),
			x.saturating_add(y),
			x.saturating_sub(y),
			x <= y,
			x.cmp(&y),
			x.min(y),
			!x,
		)
	}

	/// Casts, which wrap, and conversions, which fail where the target cannot hold the value.
	#[allow(clippy::cast_possible_truncation, reason = "the casts under test")]
	pub fn wide_casts(
		a: u128,
		x: i128,
	) -> (u64, i64, i128, u128, Option<u64>, Option<i64>, Option<u128>, Option<i128>, u128, i128) {
		(
			a as u64,
			x as i64,
			a as i128,
			x as u128,
			u64::try_from(a).ok(),
			i64::try_from(x).ok(),
			u128::try_from(x).ok(),
			i128::try_from(a).ok(),
			u128::from(a as u64),
			i128::from(x as i64),
		)
	}

	/// Collections changed in place through a field of `&mut self`.
	pub struct Ledger {
		pub m: BTreeMap<Rank, u16>,
		pub s: BTreeSet<u8>,
		pub q: VecDeque<u8>,
	}

	impl Ledger {
		pub fn edit(&mut self, r: Rank, v: u16, k: u8) -> (Option<u16>, Option<(Rank, u16)>, Option<u16>, bool, bool, Option<u8>) {
			let old = self.m.insert(r, v);
			let first = self.m.pop_first();
			let gone = self.m.remove(&r);
			let new = self.s.insert(k);
			let had = self.s.remove(&(k ^ 1));
			let low = self.s.pop_first();
			(old, first, gone, new, had, low)
		}

		pub fn queue(&mut self, x: u8) -> (Option<u8>, bool, usize) {
			self.q.push_back(x);
			let front = self.q.pop_front();
			(front, self.q.is_empty(), self.q.len())
		}
	}

	/// Indexing a map whose keys are references, by a borrow of one.
	pub fn by_ref(m: &BTreeMap<&u8, u8>, k: &u8) -> u8 {
		m[&k]
	}

	pub fn key_list<K: Ord + Copy, V>(m: &BTreeMap<K, V>) -> Vec<K> {
		keys_of(m.keys())
	}

	/// A `Keys<K, V>` lowers to a `List K`, so a call names the `V` it erases.
	fn keys_of<K: Copy, V>(keys: std::collections::btree_map::Keys<'_, K, V>) -> Vec<K> {
		keys.copied().collect()
	}

	/// What a sorted map, a sorted set, and a queue read back, in order; `m[&r]` panics when
	/// `r` is absent.
	pub fn peeks(l: &Ledger, r: Rank) -> (Option<(Rank, u16)>, Vec<Rank>, Vec<u16>, Vec<(Rank, u16)>, Option<u8>, Vec<u8>, Vec<u8>, u16) {
		(
			l.m.get_key_value(&r).map(|(k, v)| (*k, *v)),
			l.m.keys().copied().collect(),
			l.m.values().copied().collect(),
			l.m.iter().map(|(k, v)| (*k, *v)).collect(),
			l.s.first().copied(),
			l.s.iter().copied().collect(),
			l.q.iter().copied().collect(),
			l.m[&r],
		)
	}

	/// `&mut`s into a map or an `Option`, from `get_mut`, `entry`, and `get_or_insert_default`,
	/// which a write through changes in place.
	pub mod lenses {
		use std::collections::BTreeMap;
		use std::collections::btree_map::Entry;

		#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
		pub struct Tally {
			pub hits: u16,
			pub last: u8,
		}

		/// A `Default` written by hand, which `or_default` calls.
		#[derive(Clone, Copy, PartialEq, Eq, Debug)]
		pub struct Seeded(pub u16);

		impl Default for Seeded {
			fn default() -> Self {
				Seeded(7)
			}
		}

		pub struct Book {
			pub tallies: BTreeMap<u8, Tally>,
			pub nested: BTreeMap<u8, BTreeMap<u8, u16>>,
			pub seeds: BTreeMap<u8, Seeded>,
			pub log: Option<Vec<u8>>,
		}

		impl Book {
			/// `?` on `get_mut`, then writes through it, whose overflow panics.
			pub fn bump(&mut self, k: u8, by: u16) -> Option<u16> {
				let t = self.tallies.get_mut(&k)?;
				t.hits += by;
				t.last = k;
				Some(t.hits)
			}

			/// `if let` and `let`-`else` on `get_mut`, `ok_or` and `?`, and `expect`.
			pub fn touch(&mut self, k: u8, j: u8) -> Result<u8, u8> {
				if let Some(t) = self.tallies.get_mut(&k) {
					t.last = j;
				}
				let Some(t) = self.tallies.get_mut(&j) else {
					return Err(0);
				};
				t.hits = t.hits.saturating_add(1);
				let t = self.tallies.get_mut(&k).ok_or(1)?;
				t.hits ^= 1;
				Ok(self.tallies.get_mut(&(k ^ j)).expect("a tally").last)
			}

			/// A key that changes while its `&mut` lives, which keeps the key it was made with.
			pub fn moved_key(&mut self, mut k: u8) -> Option<u8> {
				let t = self.tallies.get_mut(&k)?;
				k ^= 1;
				t.last = k;
				Some(k)
			}

			/// `entry` with `or_default`, nested, `or_insert`, `or_insert_with`, a `Default` written by
			/// hand, and a match on `Entry`.
			pub fn count(&mut self, k: u8, j: u8) -> (u16, u16) {
				*self.nested.entry(k).or_default().entry(j).or_insert(0) += 1;
				let t = self.tallies.entry(k).or_default();
				t.hits = t.hits.saturating_add(1);
				let s = self.seeds.entry(j).or_default();
				s.0 += 1;
				let w = self.tallies.entry(j).or_insert_with(|| Tally { hits: 100, last: j });
				w.last ^= k;
				if let Entry::Vacant(e) = self.seeds.entry(k ^ 1) {
					e.insert(Seeded(k as u16));
				}
				(self.nested[&k][&j], self.seeds[&j].0)
			}

			/// Arguments that may panic, which Rust evaluates whether or not the key is there.
			pub fn eager(&mut self, k: u8, f: Option<u16>, named: bool) -> (u16, u8) {
				let s = self.seed::<3, 2>(k, f);
				let name = if named { Some("a tally") } else { None };
				(s, self.tallies.get_mut(&k).expect(name.unwrap()).last)
			}

			/// Const parameters named as the variables a panicking argument and a loop's result
			/// are bound to, and two locals named as a Lean keyword.
			#[allow(non_upper_case_globals)]
			fn seed<const arg: usize, const r: usize>(&mut self, k: u8, f: Option<u16>) -> u16 {
				let mut s = self.seeds.entry(k).or_insert(Seeded(f.unwrap())).0;
				let mut i = 0;
				while i < arg {
					s ^= 1 << i;
					i += 1;
				}
				let mut by = r;
				while by > 0 {
					s ^= 0x100 << by;
					by -= 1;
				}
				let by = s;
				by
			}

			/// `get_or_insert_default` on an `Option`, `mem::take` through it, `mem::replace`, and
			/// a derived `Default`.
			pub fn logs(&mut self, x: u8) -> (Vec<u8>, Option<Vec<u8>>, Tally) {
				self.log.get_or_insert_default().push(x);
				let taken = std::mem::take(self.log.get_or_insert_default());
				self.log.get_or_insert_default().push(x ^ 1);
				let old = std::mem::replace(&mut self.log, None);
				(taken, old, Tally::default())
			}
		}
	}
}

/// An inherent `impl` outside its type's module, which `--start-from crate::cases` reaches.
mod elsewhere {
	impl super::cases::Cost {
		pub fn raw(self) -> u64 {
			self.0
		}
	}
}

/// An allocator other than `Global`, outside `refused` so the census skips its own body.
#[derive(Clone, Copy)]
pub struct Arena;

unsafe impl std::alloc::Allocator for Arena {
	fn allocate(&self, layout: std::alloc::Layout) -> Result<std::ptr::NonNull<[u8]>, std::alloc::AllocError> {
		std::alloc::Global.allocate(layout)
	}

	unsafe fn deallocate(&self, ptr: std::ptr::NonNull<u8>, layout: std::alloc::Layout) {
		unsafe { std::alloc::Global.deallocate(ptr, layout) }
	}
}

/// Outside the subset, each refused at its span.
pub mod refused {
	use super::Arena;
	use std::cell::Cell;
	use std::collections::BTreeSet;

	pub struct Loose(pub u8);

	impl PartialEq for Loose {
		fn eq(&self, _: &Self) -> bool {
			true
		}
	}

	#[derive(PartialEq)]
	pub struct Holder {
		pub loose: Loose,
	}

	pub fn holder_eq(a: &Holder, b: &Holder) -> bool {
		a == b
	}

	pub struct Norm(pub u8);

	impl Clone for Norm {
		fn clone(&self) -> Self {
			Norm(0)
		}
	}

	pub fn extend(v: &mut Vec<Norm>, s: &[Norm]) {
		v.extend_from_slice(s)
	}

	/// Clones its `T`, which translates as a copy only if every type the crate passes for it
	/// copies.
	pub fn dup<T: Clone>(t: &T) -> T {
		t.clone()
	}

	/// Passes its own `T` on, so `dup` relies on each type passed here too.
	pub fn dup_via<T: Clone>(t: &T) -> T {
		dup(t)
	}

	pub fn dup_byte(b: &u8) -> u8 {
		dup(b)
	}

	#[derive(PartialEq)]
	pub struct Reset(pub u8);

	impl Clone for Reset {
		fn clone(&self) -> Self {
			Reset(0)
		}
	}

	/// `Reset`'s `clone` does not copy, so `dup` is refused at this instantiation.
	pub fn dup_reset(r: &Reset) -> Reset {
		dup_via(r)
	}

	/// A static can name an instantiation no function does.
	pub static RESET: fn(&Reset) -> Reset = dup::<Reset>;

	/// The iterator is made before the loop, so each pass's `any` resumes where the last
	/// stopped, which a list read again does not.
	pub fn any_in_loop(v: &[u8]) -> u8 {
		let mut it = v.iter();
		let mut n = 0;
		while n < 2 && it.any(|_| true) {
			n += 1;
		}
		n
	}

	pub trait Weigh {
		fn weigh(&self) -> u64;
	}

	#[derive(PartialEq)]
	pub struct Heavy(pub u64);

	impl Weigh for Heavy {
		fn weigh(&self) -> u64 {
			self.0
		}
	}

	pub fn weigh_of<W: Weigh>(w: &W) -> u64 {
		w.weigh()
	}

	/// `Weigh for Heavy` would be a Lean instance, which the translation does not emit.
	pub fn weigh_heavy(h: &Heavy) -> u64 {
		weigh_of(h)
	}

	/// Lean's `Except` has no `Ord`.
	pub fn result_key(s: &BTreeSet<Result<u8, u8>>, k: Result<u8, u8>) -> bool {
		s.contains(&k)
	}

	/// A derived `Ord` puts `Low` first by discriminant; Lean's would put `High` first.
	#[derive(PartialEq, Eq, PartialOrd, Ord)]
	pub enum Backwards {
		High = 2,
		Low = 1,
	}

	pub fn backwards_key(s: &BTreeSet<Backwards>, k: Backwards) -> bool {
		s.contains(&k)
	}

	/// An inline constant can name an instantiation no function does.
	pub fn reset_const() -> fn(&Reset) -> Reset {
		const { dup::<Reset> }
	}

	pub trait Light {
		fn light(&self) -> u64;
	}

	impl<T: Copy + Into<u64>> Light for T {
		fn light(&self) -> u64 {
			(*self).into()
		}
	}

	pub fn light_of<L: Light>(l: &L) -> u64 {
		l.light()
	}

	/// `T`'s `Light` comes from the blanket impl, so this function has no Lean instance for it.
	pub fn copy_light<T: Copy + Into<u64>>(t: &T) -> u64 {
		light_of(t)
	}

	pub fn has<K: Ord>(s: &BTreeSet<K>, k: &K) -> bool {
		s.contains(k)
	}

	/// `has` orders `Backwards` as Lean's derived `Ord` would not.
	pub fn has_backwards(s: &BTreeSet<Backwards>, k: &Backwards) -> bool {
		has(s, k)
	}

	/// `Option<T>: Ord` gives this function no `Ord T`, which Lean's `Ord (Option T)` needs.
	pub fn has_some<T>(s: &BTreeSet<Option<T>>, k: &Option<T>) -> bool
	where
		Option<T>: Ord,
	{
		has(s, k)
	}

	/// The same bound, at a lookup the model translates directly.
	pub fn contains_some<T>(s: &BTreeSet<Option<T>>, k: &Option<T>) -> bool
	where
		Option<T>: Ord,
	{
		s.contains(k)
	}

	/// The same bound, through another crate's trait with `Ord` above it.
	pub fn contains_ordered<T>(s: &BTreeSet<Option<T>>, k: &Option<T>) -> bool
	where
		Option<T>: fixture_dep::Ordered,
	{
		s.contains(k)
	}

	/// The mapper is a function pointer, whose panics the list cannot hold.
	pub fn mapped(mut it: std::iter::Map<std::slice::Iter<'_, u8>, fn(&u8) -> bool>) -> bool {
		it.any(|x| x)
	}

	/// The `..` comes from an `unwrap`, which may panic.
	pub fn whole(s: &[u8], o: Option<std::ops::RangeFull>) -> usize {
		s[o.unwrap()].len()
	}

	/// A list cannot hold an endless iterator.
	pub fn endless(mut it: std::iter::Repeat<u8>) -> bool {
		it.any(|x| x == 0)
	}

	#[derive(Copy)]
	pub struct Zeroed(pub u8);

	impl Clone for Zeroed {
		fn clone(&self) -> Self {
			Zeroed(0)
		}
	}

	pub fn extend_copy(v: &mut Vec<Zeroed>, s: &[Zeroed]) {
		v.extend_from_slice(s)
	}

	#[derive(Clone)]
	pub struct Tree {
		pub children: Vec<Tree>,
	}

	pub fn grow(v: &mut Vec<Tree>, s: &[Tree]) {
		v.extend_from_slice(s)
	}

	pub fn boom() -> u8 {
		panic!("boom")
	}

	pub struct Packet {
		pub name: String,
	}

	pub fn pass(x: &Packet) -> &Packet {
		x
	}

	pub fn interior(c: &Cell<u8>) -> u8 {
		c.get()
	}

	pub fn recursive(n: u8) -> u8 {
		if n == 0 { 0 } else { recursive(n - 1) }
	}

	pub fn reborrow(v: &mut [u8]) -> &mut u8 {
		&mut v[0]
	}

	/// A custom allocator's `Clone` may panic, so cloning its `Vec` is not a copy.
	pub fn arena_clone(v: &Vec<u8, Arena>) -> Vec<u8, Arena> {
		v.clone()
	}

	#[derive(PartialEq)]
	pub struct Left(pub u8);

	#[derive(PartialEq)]
	pub struct Right(pub u8);

	impl PartialEq<Right> for Left {
		fn eq(&self, other: &Right) -> bool {
			self.0 == other.0
		}
	}

	/// Lean's `=` cannot compare an `Array Left` with an `Array Right`.
	pub fn cross_eq(a: &Vec<Left>, b: &Vec<Right>) -> bool {
		a == b
	}

	pub fn closure(x: u8) -> u8 {
		let f = |y: u8| y + x;
		f(1)
	}

	pub fn counts(v: &[u8]) -> usize {
		let mut n = 0;
		v.iter().for_each(|_| n += 1);
		n
	}

	/// `any` consumes what it reads, which the next `any` then skips.
	pub fn twice(v: &[u8]) -> bool {
		let mut it = v.iter();
		it.any(|&x| x == 0) && it.any(|&x| x == 1)
	}

	#[derive(PartialEq, Eq)]
	pub struct Odd(pub u8);

	impl PartialOrd for Odd {
		fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
			Some(self.cmp(other))
		}
	}

	impl Ord for Odd {
		fn cmp(&self, other: &Self) -> std::cmp::Ordering {
			other.0.cmp(&self.0)
		}
	}

	pub fn odd(s: &std::collections::BTreeSet<Odd>, k: &Odd) -> bool {
		s.contains(k)
	}

	/// A `String` key looked up by `str`, which a different `Ord` could order.
	pub fn named(m: &std::collections::BTreeMap<String, u8>, k: &str) -> Option<u8> {
		m.get(k).copied()
	}

	pub trait Source {
		type Out;
		fn pull(&self) -> Self::Out;
	}

	pub fn pull<S: Source<Out = u8>>(s: &S) -> u8 {
		s.pull()
	}

	pub fn sorted(v: &[u8]) -> std::collections::BTreeSet<u8> {
		v.iter().copied().collect()
	}

	pub fn numbered(v: &[u8]) -> Vec<(usize, u8)> {
		v.iter().map(|y| y + 1).enumerate().collect()
	}

	/// A parameter with no `Ord` bound, whose Lean type has no order to compare by.
	pub fn below<T: PartialOrd>(a: &T, b: &T) -> bool {
		a < b
	}

	pub fn oddest(a: Odd, b: Odd) -> Odd {
		a.max(b)
	}

	pub fn unname(m: &mut std::collections::BTreeMap<String, u8>, k: &str) -> Option<u8> {
		m.remove(k)
	}

	/// A `u128` method without a model.
	pub fn spread(a: u128) -> u32 {
		a.count_ones()
	}

	/// A closure returning a `&mut` into a map, which its caller would take for a copy.
	pub fn nested_mut(m: &mut std::collections::BTreeMap<u8, std::collections::BTreeMap<u8, u8>>, a: u8, b: u8) {
		if let Some(v) = m.get_mut(&a).and_then(|n| n.get_mut(&b)) {
			*v = 0;
		}
	}

	/// A type parameter's `Default`, which Lean's `Inhabited` need not match.
	pub fn fresh<T: Default>(m: &mut std::collections::BTreeMap<u8, T>, k: u8) {
		m.entry(k).or_default();
	}

	/// `?` converting its error through `From`.
	pub fn widened(m: &std::collections::BTreeMap<u8, u8>) -> Result<u8, u16> {
		Ok(*m.get(&0).ok_or(0u8)?)
	}

	/// A closure that writes through the `&mut` it is passed.
	pub fn map_mut(m: &mut std::collections::BTreeMap<u8, u8>, k: u8) -> Option<u8> {
		m.get_mut(&k).map(|v| {
			*v = 0;
			1
		})
	}

	/// A value holding a `&mut` from either of two `get_mut`s, whose lens differs by path.
	pub fn either_mut(m: &mut std::collections::BTreeMap<u8, u8>, c: bool) {
		let o = if c { m.get_mut(&0) } else { m.get_mut(&1) };
		if let Some(v) = o {
			*v = 0;
		}
	}

	/// A write through a `&mut` that replaces the lens a value holds.
	pub fn swap_lens(
		a: &mut std::collections::BTreeMap<u8, u8>,
		b: &mut std::collections::BTreeMap<u8, u8>,
	) {
		let mut slot = a.get_mut(&0);
		let _ = std::mem::replace(&mut slot, b.get_mut(&0));
		if let Some(v) = slot {
			*v = 1;
		}
	}

	/// An entry's handle passed to a function, which receives no lens with it.
	pub fn vacate(e: std::collections::btree_map::VacantEntry<'_, u8, u8>) {
		e.insert(0);
	}

	/// A `&mut` that no lens model makes.
	pub fn last_mut(v: &mut Vec<u8>) {
		if let Some(x) = v.last_mut() {
			*x = 0;
		}
	}

	/// A dependency's function, refused at its span in the dependency.
	pub fn elapsed(a: kio::clock::instant::Instant, b: kio::clock::instant::Instant) -> std::time::Duration {
		fixture_dep::Sub::sub(a, b)
	}
}

