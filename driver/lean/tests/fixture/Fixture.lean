-- Translated from the `fixture` crate by rust2-lean, with overflow checks on.
import Rust
import FixtureDep
import Kio

set_option linter.unusedVariables false
-- Rust's wide tuples need deeper instance searches than Lean's default.
set_option synthInstance.maxSize 1024

namespace fixture

/-- `Sub` -/
structure Sub where
  opening : (Option UInt8)
  turn : UInt8
deriving Repr, DecidableEq, Inhabited

/-- `Session` -/
structure Session (S : UInt64) where
  subs : (Vector Sub S.toNat)
  out : (Array UInt8)
deriving Repr, DecidableEq, Inhabited

/-- `cases::Cost` -/
structure cases.Cost where
  x0 : UInt64
deriving Repr, DecidableEq, Inhabited

/-- `cases::Record` -/
structure cases.Record where
  x0 : UInt8
deriving Repr, DecidableEq, Inhabited

/-- `cases::Plain` -/
structure cases.Plain where
  x0 : UInt8
deriving Repr, DecidableEq, Inhabited

/-- `cases::Wrap` -/
structure cases.Wrap where
  x0 : cases.Plain
deriving Repr, DecidableEq, Inhabited

/-- `cases::M` -/
structure cases.M where
  x0 : UInt8
deriving Repr, DecidableEq, Inhabited

/-- `cases::Pair` -/
structure cases.Pair (A : UInt64) (B : UInt64) where
  x0 : (Vector UInt8 A.toNat)
  x1 : (Vector UInt8 B.toNat)
deriving Repr, DecidableEq, Inhabited

/-- `cases::Rank` -/
structure cases.Rank where
  up : Bool
  low : (Rust.Reverse Int8)
deriving Repr, DecidableEq, Inhabited, Ord

/-- `cases::Severity` -/
inductive cases.Severity where
  | High
  | Low
deriving Repr, DecidableEq, Inhabited

/-- `cases::lenses::Seeded` -/
structure cases.lenses.Seeded where
  x0 : UInt16
deriving Repr, DecidableEq, Inhabited

/-- `cases::Step` -/
inductive cases.Step where
  | Back
  | Ahead (x0 : UInt16)
deriving Repr, DecidableEq, Inhabited

/-- `cases::Ledger` -/
structure cases.Ledger where
  m : (Rust.BTreeMap cases.Rank UInt16)
  s : (Rust.BTreeSet UInt8)
  q : (Rust.VecDeque UInt8)
deriving Repr, DecidableEq, Inhabited

/-- `cases::lenses::Tally` -/
structure cases.lenses.Tally where
  hits : UInt16
  last : UInt8
deriving Repr, DecidableEq, Inhabited

/-- `cases::lenses::Book` -/
structure cases.lenses.Book where
  tallies : (Rust.BTreeMap UInt8 cases.lenses.Tally)
  nested : (Rust.BTreeMap UInt8 (Rust.BTreeMap UInt8 UInt16))
  seeds : (Rust.BTreeMap UInt8 cases.lenses.Seeded)
  log : (Option (Array UInt8))
deriving Repr, DecidableEq, Inhabited

/-- `Int` -/
structure Int' where
  x0 : UInt8
deriving Repr, DecidableEq, Inhabited

/-- `cases::text::Countdown` -/
structure cases.text.Countdown where
  x0 : UInt8
deriving Repr, DecidableEq, Inhabited

/-- `cases::Weight` -/
class cases.Weight (Self : Type) where
  weight : Self → Rust.M UInt8

def Session.opening {S : UInt64} (self : (Session S)) (s : UInt64) : Rust.M UInt8 := do
  unless s < S do throw s!"index out of bounds: the len is {S} but the index is {s}"
  match self.subs[s.toNat]!.opening with
  | some k =>
    return k
  | none =>
    return (0 : UInt8)

def Session.turns_loop {S : UInt64} (self : (Session S)) (total : UInt16) (s : UInt64) : Rust.M UInt16 := do
  let mut total := total
  let mut s := s
  if s < S then
    let t10 := s
    unless t10 < S do throw s!"index out of bounds: the len is {S} but the index is {t10}"
    let t8 := self.subs[t10.toNat]!.turn.toUInt16
    total := (← total +? t8)
    s := (← s +? 1)
    return (← Session.turns_loop (S := S) self total s)
  else
    return (total)
partial_fixpoint

def Session.turns {S : UInt64} (self : (Session S)) : Rust.M UInt16 := do
  let total : UInt16 := 0
  let s : UInt64 := 0
  let r ← Session.turns_loop (S := S) self total s
  return r

def Session.same {S : UInt64} (self : (Session S)) (s : UInt64) (t : UInt64) : Rust.M Bool := do
  unless s < S do throw s!"index out of bounds: the len is {S} but the index is {s}"
  unless t < S do throw s!"index out of bounds: the len is {S} but the index is {t}"
  return decide (self.subs[s.toNat]!.opening = self.subs[t.toNat]!.opening)

def Session.next {S : UInt64} (self : (Session S)) (s : UInt64) : Rust.M (Option UInt8) := do
  unless s < S do throw s!"index out of bounds: the len is {S} but the index is {s}"
  let t4 := Rust.branch self.subs[s.toNat]!.opening
  match t4 with
  | .Continue val =>
    let k := val
    return Rust.checkedAdd k (1 : UInt8)
  | .Break t4_01 =>
    return (none : (Option UInt8))

def Session.step {S : UInt64} (self : (Session S)) (s : UInt64) : Rust.M (Bool × (Session S)) := do
  let mut self := self
  let len := UInt64.ofNat self.out.size
  unless s < S do throw s!"index out of bounds: the len is {S} but the index is {s}"
  self := { self with out := self.out.push self.subs[s.toNat]!.turn }
  unless s < S do throw s!"index out of bounds: the len is {S} but the index is {s}"
  if self.subs[s.toNat]!.opening.isNone then
    self := { self with out := self.out.extract 0 len.toNat }
    return (false, self)
  else
    unless s < S do throw s!"index out of bounds: the len is {S} but the index is {s}"
    self := { self with subs := self.subs.set! s.toNat { self.subs[s.toNat]! with turn := (← self.subs[s.toNat]!.turn +? 1) } }
    return (true, self)

def cases.Cost.value (self : cases.Cost) : UInt64 :=
  self.x0

def cases.Cost.x0' (self : cases.Cost) : UInt64 :=
  self.x0

def cases.dup (c : cases.Cost) : cases.Cost :=
  c

def cases.dup_ref (r : cases.Record) : cases.Record := Id.run do
  let t4 := r
  let t2 := t4
  return t2

def cases.set (x : UInt8) : UInt8 := Id.run do
  let mut x := x
  x := (42 : UInt8)
  return x

def cases.snapshot_borrow (x : UInt8) : (UInt8 × UInt8) := Id.run do
  let mut x := x
  let r := cases.set x
  let t2 := x
  x := r
  return (t2, x)

def cases.temp_borrow (x : UInt8) : UInt8 := Id.run do
  let mut t5 := x
  let r := cases.set t5
  t5 := r
  return x

def cases.clone_bytes (v : (Array UInt8)) : (Array UInt8) :=
  v

def cases.dup_generic {T : Type} [Inhabited T] (t : T) : T :=
  t

def cases.dup_pair (x : UInt8) : (UInt8 × UInt8) := Id.run do
  let t4 := x
  let t7 := x
  return (cases.dup_generic t4, cases.dup_generic t7)

def cases.then_unwrap (b : Bool) (o : (Option UInt8)) : Rust.M (Option UInt8) := do
  return (if b then some · else none) (← Rust.expect o "called `Option::unwrap()` on a `None` value")

def cases.clone_pairs (v : (Array ((Option UInt8) × (Option UInt8)))) : (Array ((Option UInt8) × (Option UInt8))) :=
  v

def cases.Plain.Clone.clone (self : cases.Plain) : cases.Plain :=
  self

def cases.dup_wrap (w : cases.Wrap) : cases.Wrap :=
  w

def cases.snapshot (x : UInt8) : (UInt8 × UInt8) := Id.run do
  let mut x := x
  let t2 := x
  x := (42 : UInt8)
  return (t2, x)

def cases.M.step (self : cases.M) : UInt8 :=
  self.x0

def cases.M.Machine.step (self : cases.M) : UInt8 := Id.run do
  if self.x0 = 0 then
    return (1 : UInt8)
  else
    return (0 : UInt8)

def cases.both (m : cases.M) : (UInt8 × UInt8) :=
  (m.step, cases.M.Machine.step m)

def cases.Pair.Machine.step {A : UInt64} {B : UInt64} (self : (cases.Pair A B)) : UInt8 :=
  (0 : UInt8)

def cases.pair_step (p : (cases.Pair 2 3)) : UInt8 :=
  cases.Pair.Machine.step p

def cases.wait_loop  : Rust.M Empty := do
  return (← cases.wait_loop)
partial_fixpoint

def cases.wait : Rust.M UInt8 := do
  let r ← cases.wait_loop
  return (Empty.elim r)

def cases.shifted.closure0 (arg1 : UInt8) (y : UInt8) : Rust.M UInt8 := do
  let t5 := arg1
  return (← y +? t5)

def cases.shifted (v : (Array UInt8)) (x : UInt8) : Rust.M (Array UInt8) := do
  return (← Rust.run (Rust.lazyMap (cases.shifted.closure0 x) (Rust.lazy (v.toList)))).toArray

def cases.reaches.closure0 (arg1 : UInt8) (y : UInt8) : Rust.M UInt8 := do
  let t5 := arg1
  return (← y +? t5)

def cases.reaches.closure1 (arg1 : Unit) (y : UInt8) : Bool :=
  decide (y = 0)

def cases.reaches (v : (Array UInt8)) (x : UInt8) : Rust.M Bool := do
  let t4 := Rust.lazyMap (cases.reaches.closure0 x) (Rust.lazy (v.toList))
  return (← Rust.lazyAny (pure ∘ (cases.reaches.closure1 ())) t4)

def cases.rising.closure0 (arg1 : Unit) (arg2 : (UInt64 × UInt8)) : Bool := Id.run do
  let i := arg2.1
  let y := arg2.2
  return decide (i ≤ (y.toUInt64))

def cases.rising (a : (Array UInt8)) (b : (Array UInt8)) : Bool := Id.run do
  let t4 := Rust.enumerate ((a.toList) ++ (b.toList))
  return t4.all (cases.rising.closure0 ())

def cases.pair_sums.closure0 (arg1 : Unit) (w : (Array UInt8)) : Rust.M UInt16 := do
  let t5 : UInt64 := 0
  let t6 := UInt64.ofNat w.size
  unless t5 < t6 do throw s!"index out of bounds: the len is {t6} but the index is {t5}"
  let t3 := w[t5.toNat]!.toUInt16
  let t10 : UInt64 := 1
  let t11 := UInt64.ofNat w.size
  unless t10 < t11 do throw s!"index out of bounds: the len is {t11} but the index is {t10}"
  let t8 := w[t10.toNat]!.toUInt16
  return (← t3 +? t8)

def cases.pair_sums (v : (Array UInt8)) : Rust.M (Array UInt16) := do
  return (← Rust.run (Rust.lazyMap (cases.pair_sums.closure0 ()) (Rust.lazy (← Rust.windows v (2 : UInt64))))).toArray

def cases.ends.closure0 (arg1 : Unit) (f : UInt8) : (UInt8 × UInt8 × UInt8) :=
  (f, f, f)

def cases.ends (v : (Array UInt8)) : (Option (UInt8 × UInt8 × UInt8)) := Id.run do
  if (UInt64.ofNat v.size) ≥ 2 then
    let first := v[(0 : UInt64).toNat]!
    let last := v[v.size - 1]!
    let t13 := Rust.branch (v.back?)
    let t9 := first
    let t10 := last
    match t13 with
    | .Continue val =>
      return (some (t9, t10, val) : (Option (UInt8 × UInt8 × UInt8)))
    | .Break t13_01 =>
      return (none : (Option (UInt8 × UInt8 × UInt8)))
  else
    return (v[0]?).map (cases.ends.closure0 ())

def cases.ranges (v : (Array UInt8)) (n : UInt64) : Rust.M (UInt64 × UInt64 × UInt64 × UInt64) := do
  let t5 := (← Rust.sliceTo v n)
  let t11 := (← Rust.sliceFrom v n)
  let t17 := (← Rust.slice v ((1 : UInt64), n).1 ((1 : UInt64), n).2)
  let t23 := v
  return (UInt64.ofNat t5.size, UInt64.ofNat t11.size, UInt64.ofNat t17.size, UInt64.ofNat t23.size)

def cases.must (o : (Option UInt8)) : Rust.M UInt8 := do
  let t4 := "no \"value\"\n"
  return (← Rust.expect o t4)

def cases.saturating (a : UInt8) (b : UInt8) : (UInt8 × UInt8) :=
  (Rust.saturatingAdd a b, Rust.saturatingSub a b)

def cases.widen (a : UInt8) (b : Bool) : (UInt32 × UInt8) :=
  (Rust.cast UInt32 a, if b then (1 : UInt8) else 0)

def cases.options.closure0 (arg1 : Unit) (x : UInt8) : Bool :=
  decide (x > 1)

def cases.options.closure1 (arg1 : Unit) (x : UInt8) : Rust.M UInt16 := do
  let t3 := x.toUInt16
  return (← t3 *? 2)

def cases.options (o : (Option UInt8)) (b : Bool) : Rust.M (Bool × (Option UInt16) × UInt8 × (Option UInt8)) := do
  return (Option.any (cases.options.closure0 ()) o, (← Rust.mapM o (cases.options.closure1 ())), o.getD (7 : UInt8), if b then some (3 : UInt8) else none)

def cases.ranked (r : cases.Rank) : (Bool × (Rust.Reverse Int8)) :=
  (r.up, r.low)

def cases.shared (v : (Array UInt8)) : (Array UInt8) := Id.run do
  let a := (v.toList).toArray
  let t12 := a
  let t10 := t12
  return (t10.toList).toArray

def cases.lookups (m : (Rust.BTreeMap cases.Rank UInt16)) (s : (Rust.BTreeSet UInt8)) (r : cases.Rank) (k : UInt8) : ((Option UInt16) × Bool × Bool × UInt64 × Bool) := Id.run do
  let t9 := r
  let t13 := r
  let t17 := k
  return (m.get t9, m.containsKey t13, s.contains t17, s.len, m.entries.isEmpty)

def cases.severities (s : (Rust.BTreeSet cases.Severity)) : UInt64 :=
  s.len

def cases.heavier {W : Type} [Inhabited W] [cases.Weight W] (a : W) (b : W) : Rust.M Bool := do
  let t3 := (← cases.Weight.weight a)
  return decide (t3 > (← cases.Weight.weight b))

def cases.spans (ms : UInt64) (ns : UInt64) (s : UInt64) : (Rust.Duration × Rust.Duration × Rust.Duration × Rust.Duration × Rust.Duration × Rust.Duration) :=
  (Rust.Duration.fromMillis ms, Rust.Duration.fromNanos ns, Rust.Duration.fromSecs s, (⟨0, 0⟩ : Rust.Duration), (⟨18446744073709551615, 999999999⟩ : Rust.Duration), (⟨1, 500000000⟩ : Rust.Duration))

def cases.sums (a : Rust.Duration) (b : Rust.Duration) (k : UInt32) : ((Option Rust.Duration) × (Option Rust.Duration) × (Option Rust.Duration) × Rust.Duration × Rust.Duration × Rust.Duration) :=
  (Rust.Duration.checkedAdd a b, Rust.Duration.checkedSub a b, Rust.Duration.checkedMul a k, Rust.Duration.saturatingAdd a b, Rust.Duration.saturatingSub a b, Rust.Duration.saturatingMul a k)

def cases.compared (a : Rust.Duration) (b : Rust.Duration) (p : (UInt8 × cases.Rank)) (q : (UInt8 × cases.Rank)) : (Bool × Bool × Bool × Bool × Rust.Duration × Rust.Duration × Ordering × Bool × Bool) := Id.run do
  let t26 := b
  return (decide (compare a b = .lt), decide (compare a b ≠ .gt), decide (compare a b = .gt), decide (compare a b ≠ .lt), Rust.ordMax a b, Rust.ordMin a b, compare a t26, decide (compare p q = .lt), decide (compare p q ≠ .lt))

def cases.ticks (i : _root_.kio.clock.instant.Instant) (d : Rust.Duration) (j : UInt64) : ((Option UInt64) × UInt64 × Rust.Duration × UInt64) := Id.run do
  let t := _root_.fixture_dep.Ticks.of i
  let u := _root_.fixture_dep.Ticks.of ({ x0 := j } : _root_.kio.clock.instant.Instant)
  return ((t.checked_add d).map (_root_.fixture_dep.Ticks.get), _root_.fixture_dep.Ticks.get (t.saturating_add d), t.since u, _root_.fixture_dep.Ticks.get (_root_.fixture_dep.larger t u))

def elsewhere.doubled (x : UInt16) : Rust.M UInt16 := do
  return (← x *? 2)

def cases.passed (o : (Option UInt16)) (n : (Option UInt64)) : Rust.M ((Option cases.lenses.Seeded) × (Option Rust.Duration) × (Option (Option UInt16)) × (Option (Except UInt8 UInt16)) × (Option cases.Step) × (Option UInt16)) := do
  return (o.map cases.lenses.Seeded.mk, n.map Rust.Duration.fromNanos, o.map some, o.map Except.ok, o.map cases.Step.Ahead, (← Rust.mapM o (elsewhere.doubled)))

def cases.nanos (d : Rust.Duration) : (Rust.UInt128 × Rust.UInt128 × Rust.UInt128) :=
  (Rust.Duration.asNanos d, Rust.Duration.asMicros d, Rust.Duration.asMillis d)

def cases.wide_bits (a : Rust.UInt128) (b : Rust.UInt128) (s : UInt32) : Rust.M (Rust.UInt128 × Rust.UInt128 × Rust.UInt128 × Rust.UInt128 × Rust.UInt128 × Rust.UInt128 × Rust.UInt128) := do
  unless ¬((128 : UInt32) = (0 : UInt32)) do throw "attempt to calculate the remainder with a divisor of zero"
  let t17 := s % 128
  let t20 := decide (t17 < 128)
  unless t20 do throw "attempt to shift left with overflow"
  unless ¬((128 : UInt32) = (0 : UInt32)) do throw "attempt to calculate the remainder with a divisor of zero"
  let t23 := s % 128
  let t26 := decide (t23 < 128)
  unless t26 do throw "attempt to shift right with overflow"
  let t31 := decide ((Rust.cast UInt32 (1 : Int32)) < 128)
  unless t31 do throw "attempt to shift left with overflow"
  let t34 := decide ((3 : UInt8) < (128 : UInt8))
  unless t34 do throw "attempt to shift right with overflow"
  return (a &&& b, a ||| b, a ^^^ b, ~~~a, a <<< (Rust.cast Rust.UInt128 t17), a >>> (Rust.cast Rust.UInt128 t23), (a <<< (Rust.cast Rust.UInt128 (1 : Int32))) ||| (b >>> (Rust.cast Rust.UInt128 (3 : UInt8))))

def cases.wide_order (a : Rust.UInt128) (b : Rust.UInt128) : ((Option Rust.UInt128) × (Option Rust.UInt128) × Rust.UInt128 × Rust.UInt128 × Rust.UInt128 × Rust.UInt128 × Bool × Ordering) := Id.run do
  let t27 := b
  return (Rust.checkedAdd a b, Rust.checkedSub a b, Rust.saturatingAdd a b, Rust.saturatingSub a b, min a b, max a b, decide (a < b), compare a t27)

def cases.wide_arith (a : Rust.UInt128) (b : Rust.UInt128) (op : UInt8) : Rust.M Rust.UInt128 := do
  unless ¬((5 : UInt8) = (0 : UInt8)) do throw "attempt to calculate the remainder with a divisor of zero"
  if (op % 5) = 0 then
    return (← a +? b)
  else if (op % 5) = 1 then
    return (← a -? b)
  else if (op % 5) = 2 then
    return (← a *? b)
  else if (op % 5) = 3 then
    unless ¬(b = 0) do throw "attempt to divide by zero"
    return a / b
  else
    unless ¬(b = 0) do throw "attempt to calculate the remainder with a divisor of zero"
    return a % b

def cases.signed_arith (x : Rust.Int128) (y : Rust.Int128) (op : UInt8) : Rust.M Rust.Int128 := do
  unless ¬((6 : UInt8) = (0 : UInt8)) do throw "attempt to calculate the remainder with a divisor of zero"
  if (op % 6) = 0 then
    return (← x +? y)
  else if (op % 6) = 1 then
    return (← x -? y)
  else if (op % 6) = 2 then
    return (← x *? y)
  else if (op % 6) = 3 then
    unless ¬(y = 0) do throw "attempt to divide by zero"
    let t21 := (decide (y = (-1))) && (decide (x = (-170141183460469231731687303715884105728)))
    unless !t21 do throw "attempt to divide with overflow"
    return x / y
  else if (op % 6) = 4 then
    unless ¬(y = 0) do throw "attempt to calculate the remainder with a divisor of zero"
    let t27 := (decide (y = (-1))) && (decide (x = (-170141183460469231731687303715884105728)))
    unless !t27 do throw "attempt to calculate the remainder with overflow"
    return x % y
  else
    unless ¬(x = (-170141183460469231731687303715884105728)) do throw "attempt to negate with overflow"
    return -x

def cases.signed_bits (x : Rust.Int128) (y : Rust.Int128) (s : UInt32) : Rust.M (Rust.Int128 × Rust.Int128 × Rust.Int128 × Rust.Int128 × Bool × Ordering × Rust.Int128 × Rust.Int128) := do
  unless ¬((128 : UInt32) = (0 : UInt32)) do throw "attempt to calculate the remainder with a divisor of zero"
  let t6 := s % 128
  let t9 := decide (t6 < 128)
  unless t9 do throw "attempt to shift right with overflow"
  unless ¬((128 : UInt32) = (0 : UInt32)) do throw "attempt to calculate the remainder with a divisor of zero"
  let t12 := s % 128
  let t15 := decide (t12 < 128)
  unless t15 do throw "attempt to shift left with overflow"
  let t28 := y
  return (x >>> (Rust.cast Rust.Int128 t6), x <<< (Rust.cast Rust.Int128 t12), Rust.saturatingAdd x y, Rust.saturatingSub x y, decide (x ≤ y), compare x t28, min x y, ~~~x)

def cases.wide_casts (a : Rust.UInt128) (x : Rust.Int128) : (UInt64 × Int64 × Rust.Int128 × Rust.UInt128 × (Option UInt64) × (Option Int64) × (Option Rust.UInt128) × (Option Rust.Int128) × Rust.UInt128 × Rust.Int128) :=
  (Rust.cast UInt64 a, Rust.cast Int64 x, Rust.cast Rust.Int128 a, Rust.cast Rust.UInt128 x, (Rust.tryFrom UInt64 a).toOption, (Rust.tryFrom Int64 x).toOption, (Rust.tryFrom Rust.UInt128 x).toOption, (Rust.tryFrom Rust.Int128 a).toOption, Rust.cast Rust.UInt128 (Rust.cast UInt64 a), Rust.cast Rust.Int128 (Rust.cast Int64 x))

def cases.Ledger.edit (self : cases.Ledger) (r : cases.Rank) (v : UInt16) (k : UInt8) : (((Option UInt16) × (Option (cases.Rank × UInt16)) × (Option UInt16) × Bool × Bool × (Option UInt8)) × cases.Ledger) := Id.run do
  let mut self := self
  let r1 := Rust.BTreeMap.insert self.m r v
  self := { self with m := r1.1 }
  let old := r1.2
  let r2 := Rust.BTreeMap.popFirst self.m
  self := { self with m := r2.1 }
  let first := r2.2
  let t14 := r
  let r3 := Rust.BTreeMap.remove self.m t14
  self := { self with m := r3.1 }
  let gone := r3.2
  let r4 := Rust.BTreeSet.insert self.s k
  self := { self with s := r4.1 }
  let new := r4.2
  let t21 := k ^^^ 1
  let r5 := Rust.BTreeSet.remove self.s t21
  self := { self with s := r5.1 }
  let had := r5.2
  let r6 := Rust.BTreeSet.popFirst self.s
  self := { self with s := r6.1 }
  let low := r6.2
  return ((old, first, gone, new, had, low), self)

def cases.Ledger.queue (self : cases.Ledger) (x : UInt8) : (((Option UInt8) × Bool × UInt64) × cases.Ledger) := Id.run do
  let mut self := self
  self := { self with q := self.q.pushBack x }
  let r := Rust.VecDeque.popFront self.q
  self := { self with q := r.1 }
  let front := r.2
  return ((front, self.q.items.isEmpty, self.q.items.length.toUInt64), self)

def cases.by_ref (m : (Rust.BTreeMap UInt8 UInt8)) (k : UInt8) : Rust.M UInt8 := do
  let t6 := k
  let t3 := (← Rust.BTreeMap.index m t6)
  return t3

def cases.keys_of {K : Type} [Inhabited K] {V : Type} [Inhabited V] (keys : (List K)) : (Array K) :=
  keys.toArray

def cases.key_list {K : Type} [Inhabited K] [DecidableEq K] [Ord K] {V : Type} [Inhabited V] (m : (Rust.BTreeMap K V)) : (Array K) :=
  cases.keys_of (V := V) (m.entries.map (·.1))

def cases.peeks.closure0 (arg1 : Unit) (arg2 : (cases.Rank × UInt16)) : (cases.Rank × UInt16) := Id.run do
  let k := arg2.1
  let v := arg2.2
  return (k, v)

def cases.peeks.closure1 (arg1 : Unit) (arg2 : (cases.Rank × UInt16)) : (cases.Rank × UInt16) := Id.run do
  let k := arg2.1
  let v := arg2.2
  return (k, v)

def cases.peeks (l : cases.Ledger) (r : cases.Rank) : Rust.M ((Option (cases.Rank × UInt16)) × (Array cases.Rank) × (Array UInt16) × (Array (cases.Rank × UInt16)) × (Option UInt8) × (Array UInt8) × (Array UInt8) × UInt16) := do
  let t7 := r
  let t37 := r
  let t34 := (← Rust.BTreeMap.index l.m t37)
  return ((l.m.getKeyValue t7).map (cases.peeks.closure0 ()), (l.m.entries.map (·.1)).toArray, (l.m.entries.map (·.2)).toArray, ((l.m.entries).map (cases.peeks.closure1 ())).toArray, l.s.items.head?, (l.s.items).toArray, (l.q.items).toArray, t34)

def cases.lenses.Seeded.Default.default' : cases.lenses.Seeded :=
  ({ x0 := (7 : UInt16) } : cases.lenses.Seeded)

def cases.lenses.Book.bump (self : cases.lenses.Book) (k : UInt8) («by» : UInt16) : Rust.M ((Option UInt16) × cases.lenses.Book) := do
  let mut self := self
  let t9 := k
  let t5 := Rust.branch (self.tallies.get t9)
  match t5 with
  | .Continue t5_0 =>
    self := { self with tallies := self.tallies.setAt t9 { self.tallies.valueAt t9 with hits := (← (self.tallies.valueAt t9).hits +? «by») } }
    self := { self with tallies := self.tallies.setAt t9 { self.tallies.valueAt t9 with last := k } }
    return ((some (self.tallies.valueAt t9).hits : (Option UInt16)), self)
  | .Break t5_01 =>
    return ((none : (Option UInt16)), self)

def cases.lenses.Book.touch (self : cases.lenses.Book) (k : UInt8) (j : UInt8) : Rust.M ((Except UInt8 UInt8) × cases.lenses.Book) := do
  let mut self := self
  let t8 := k
  let t5 := self.tallies.get t8
  match t5 with
  | some t5_0 =>
    self := { self with tallies := self.tallies.setAt t8 { self.tallies.valueAt t8 with last := j } }
  | none =>
    pure ()
  let t17 := j
  let t14 := self.tallies.get t17
  match t14 with
  | some t14_0 =>
    self := { self with tallies := self.tallies.setAt t17 { self.tallies.valueAt t17 with hits := Rust.saturatingAdd (self.tallies.valueAt t17).hits 1 } }
    let t27 := k
    let t22 := Rust.branchExcept (Rust.okOr (self.tallies.get t27) (1 : UInt8))
    match t22 with
    | .Continue t22_0 =>
      self := { self with tallies := self.tallies.setAt t27 { self.tallies.valueAt t27 with hits := (self.tallies.valueAt t27).hits ^^^ 1 } }
      let t38 := k ^^^ j
      let t43 := "a tally"
      unless (self.tallies.get t38).isSome do throw t43
      return ((Except.ok (self.tallies.valueAt t38).last : (Except UInt8 UInt8)), self)
    | .Break residual =>
      return ((Rust.fromResidualExcept residual : (Except UInt8 UInt8)), self)
  | none =>
    return ((Except.error (0 : UInt8) : (Except UInt8 UInt8)), self)

def cases.lenses.Book.moved_key (self : cases.lenses.Book) (k : UInt8) : ((Option UInt8) × cases.lenses.Book) := Id.run do
  let mut self := self
  let mut k := k
  let t8 := k
  let key := t8
  let t4 := Rust.branch (self.tallies.get key)
  match t4 with
  | .Continue t4_0 =>
    k := k ^^^ 1
    self := { self with tallies := self.tallies.setAt key { self.tallies.valueAt key with last := k } }
    return ((some k : (Option UInt8)), self)
  | .Break t4_01 =>
    return ((none : (Option UInt8)), self)

def cases.lenses.Book.count.closure0 (arg1 : UInt8) : cases.lenses.Tally := Id.run do
  let t3 := arg1
  return ({ hits := (100 : UInt16), last := t3 } : cases.lenses.Tally)

def cases.lenses.Book.count (self : cases.lenses.Book) (k : UInt8) (j : UInt8) : Rust.M ((UInt16 × UInt16) × cases.lenses.Book) := do
  let mut self := self
  self := { self with nested := self.nested.orInsertWith k (fun _ => (Rust.BTreeMap.empty : (Rust.BTreeMap UInt8 UInt16))) }
  self := { self with nested := self.nested.setAt k ((self.nested.valueAt k).orInsertWith j (fun _ => (0 : UInt16))) }
  self := { self with nested := self.nested.setAt k ((self.nested.valueAt k).setAt j (← ((self.nested.valueAt k).valueAt j) +? 1)) }
  self := { self with tallies := self.tallies.orInsertWith k (fun _ => ({ hits := (0 : UInt16), last := (0 : UInt8) } : cases.lenses.Tally)) }
  self := { self with tallies := self.tallies.setAt k { self.tallies.valueAt k with hits := Rust.saturatingAdd (self.tallies.valueAt k).hits 1 } }
  self := { self with seeds := self.seeds.orInsertWith j (fun _ => (cases.lenses.Seeded.Default.default')) }
  self := { self with seeds := self.seeds.setAt j { self.seeds.valueAt j with x0 := (← (self.seeds.valueAt j).x0 +? 1) } }
  self := { self with tallies := self.tallies.orInsertWith j (fun _ => (cases.lenses.Book.count.closure0 j)) }
  self := { self with tallies := self.tallies.setAt j { self.tallies.valueAt j with last := (self.tallies.valueAt j).last ^^^ k } }
  let t32 := self.seeds.get (k ^^^ 1)
  match t32 with
  | none =>
    self := { self with seeds := self.seeds.setAt (k ^^^ 1) ({ x0 := k.toUInt16 } : cases.lenses.Seeded) }
  | some _ =>
    pure ()
  let t49 := k
  let t46 := (← Rust.BTreeMap.index self.nested t49)
  let t51 := j
  let t44 := (← Rust.BTreeMap.index t46 t51)
  let t56 := j
  let t53 := (← Rust.BTreeMap.index self.seeds t56)
  return ((t44, t53.x0), self)

def cases.lenses.Book.seed_loop0 {arg : UInt64} {r : UInt64} (self : cases.lenses.Book) (s : UInt16) (i : UInt64) : Rust.M UInt16 := do
  let mut s := s
  let mut i := i
  if i < arg then
    let t18 := i
    let t19 := decide (t18 < 16)
    unless t19 do throw "attempt to shift left with overflow"
    s := s ^^^ (1 <<< t18.toUInt16)
    i := (← i +? 1)
    return (← cases.lenses.Book.seed_loop0 (arg := arg) (r := r) self s i)
  else
    return s
partial_fixpoint

def cases.lenses.Book.seed_loop1 {arg : UInt64} {r : UInt64} (self : cases.lenses.Book) (s : UInt16) («by» : UInt64) : Rust.M (UInt16 × cases.lenses.Book) := do
  let mut s := s
  let mut «by» := «by»
  if «by» > 0 then
    let t29 := «by»
    let t30 := decide (t29 < 16)
    unless t30 do throw "attempt to shift left with overflow"
    s := s ^^^ (256 <<< t29.toUInt16)
    «by» := (← «by» -? 1)
    return (← cases.lenses.Book.seed_loop1 (arg := arg) (r := r) self s «by»)
  else
    let by1 := s
    return ((by1, self))
partial_fixpoint

def cases.lenses.Book.seed {arg : UInt64} {r : UInt64} (self : cases.lenses.Book) (k : UInt8) (f : (Option UInt16)) : Rust.M (UInt16 × cases.lenses.Book) := do
  let mut self := self
  let arg1 := ({ x0 := (← Rust.expect f "called `Option::unwrap()` on a `None` value") } : cases.lenses.Seeded)
  self := { self with seeds := self.seeds.orInsertWith k (fun _ => arg1) }
  let mut s := (self.seeds.valueAt k).x0
  let i : UInt64 := 0
  let r1 ← cases.lenses.Book.seed_loop0 (arg := arg) (r := r) self s i
  s := r1
  let «by» := r
  let r2 ← cases.lenses.Book.seed_loop1 (arg := arg) (r := r) self s «by»
  return r2

def cases.lenses.Book.eager (self : cases.lenses.Book) (k : UInt8) (f : (Option UInt16)) (named : Bool) : Rust.M ((UInt16 × UInt8) × cases.lenses.Book) := do
  let mut self := self
  let r ← cases.lenses.Book.seed (arg := 3) (r := 2) self k f
  let s := r.1
  self := r.2
  let mut name : (Option String) := default
  if named then
    name := (some "a tally" : (Option String))
  else
    name := (none : (Option String))
  let t17 := k
  let t19 := (← Rust.expect name "called `Option::unwrap()` on a `None` value")
  unless (self.tallies.get t17).isSome do throw t19
  return ((s, (self.tallies.valueAt t17).last), self)

def cases.lenses.Book.logs (self : cases.lenses.Book) (x : UInt8) : (((Array UInt8) × (Option (Array UInt8)) × cases.lenses.Tally) × cases.lenses.Book) := Id.run do
  let mut self := self
  self := { self with log := some (self.log.getD (#[] : (Array UInt8))) }
  self := { self with log := some (self.log.get!.push x) }
  self := { self with log := some (self.log.getD (#[] : (Array UInt8))) }
  let taken := self.log.get!
  self := { self with log := some (#[] : (Array UInt8)) }
  self := { self with log := some (self.log.getD (#[] : (Array UInt8))) }
  self := { self with log := some (self.log.get!.push (x ^^^ 1)) }
  let old := self.log
  self := { self with log := (none : (Option (Array UInt8))) }
  return ((taken, old, ({ hits := (0 : UInt16), last := (0 : UInt8) } : cases.lenses.Tally)), self)

def cases.text.measure (s : String) : (UInt64 × Bool × (Array UInt8)) := Id.run do
  let t7 := Rust.asBytes s
  return (Rust.strLen s, s.isEmpty, t7)

def cases.text.cut (s : String) (b : UInt64) (e : UInt64) (how : UInt8) : Rust.M String := do
  unless ¬((4 : UInt8) = (0 : UInt8)) do throw "attempt to calculate the remainder with a divisor of zero"
  let mut t5 : String := default
  if (how % 4) = 0 then
    let t10 := (← Rust.strSliceTo s e)
    let t9 := t10
    t5 := t9
  else if (how % 4) = 1 then
    let t15 := (← Rust.strSliceFrom s b)
    let t14 := t15
    t5 := t14
  else if (how % 4) = 2 then
    let t20 := (← Rust.strSlice s (b, e).1 (b, e).2)
    let t19 := t20
    t5 := t19
  else
    let t26 := s
    let t25 := t26
    t5 := t25
  return t5

def cases.text.search (s : String) (c : Char) (p : String) (i : UInt64) : ((Option UInt64) × Bool × Bool × (Option UInt8)) := Id.run do
  let t17 := Rust.asBytes s
  return (Rust.strFind s c, p.isPrefixOf s, s.toList.head? == some c, t17[i.toNat]?)

def cases.text.tails.closure0 (arg1 : UInt8) (arg2 : UInt8) : Bool := Id.run do
  let t6 := arg2
  let b := t6
  let t7 := arg1
  return decide (b = t7)

def cases.text.tails.closure1 (arg1 : Unit) (b : UInt8) : Rust.M UInt8 := do
  return (← b +? 1)

def cases.text.tails (v : (Array UInt8)) (x : UInt8) : Rust.M (UInt64 × UInt64) := do
  return ((((v.toList).reverse).takeWhile (cases.text.tails.closure0 x)).length.toUInt64, (← Rust.run (Rust.lazyMap (cases.text.tails.closure1 ()) (Rust.lazy (v.toList)))).length.toUInt64)

def cases.text.last_bumped.closure0 (arg1 : Unit) (b : UInt8) : Rust.M UInt8 := do
  return (← b +? 1)

def cases.text.last_bumped (v : (Array UInt8)) : Rust.M (Option UInt8) := do
  let mut it := Rust.lazyRev (Rust.lazyMap (cases.text.last_bumped.closure0 ()) (Rust.lazy (v.toList)))
  let r ← Rust.listNextM it
  it := r.1
  return r.2

def cases.text.find_bumped.closure0 (arg1 : Unit) (b : UInt8) : Rust.M UInt8 := do
  return (← b +? 1)

def cases.text.find_bumped_loop (x : UInt8) (n : UInt8) (iter : (List (Rust.Lazy UInt8))) : Rust.M (Option UInt8) := do
  let mut n := n
  let mut iter := iter
  let r ← Rust.listNextM iter
  iter := r.1
  let t13 := r.2
  match id t13 with
  | none =>
    return (none : (Option UInt8))
  | some b =>
    if b = x then
      return (some n : (Option UInt8))
    else
      n := (← n +? 1)
      return (← cases.text.find_bumped_loop x n iter)
partial_fixpoint

def cases.text.find_bumped (v : (Array UInt8)) (x : UInt8) : Rust.M (Option UInt8) := do
  let n : UInt8 := 0
  let iter := Rust.lazyMap (cases.text.find_bumped.closure0 ()) (Rust.lazy (v.toList))
  let r1 ← cases.text.find_bumped_loop x n iter
  return r1

def cases.text.loops_loop0 (v : (Array UInt8)) (m : (Rust.BTreeMap UInt8 UInt16)) (s : (Rust.BTreeSet UInt8)) (o : (Option UInt8)) (q : (Rust.VecDeque UInt8)) (range : UInt16) (iter : (UInt8 × UInt8)) : Rust.M UInt16 := do
  let mut range := range
  let mut iter := iter
  let r := Rust.rangeNext iter
  iter := r.1
  let t15 := r.2
  match id t15 with
  | none =>
    return range
  | some i =>
    let t21 := i.toUInt16
    range := (← range +? t21)
    return (← cases.text.loops_loop0 v m s o q range iter)
partial_fixpoint

def cases.text.loops_loop1 (v : (Array UInt8)) (m : (Rust.BTreeMap UInt8 UInt16)) (s : (Rust.BTreeSet UInt8)) (o : (Option UInt8)) (q : (Rust.VecDeque UInt8)) (range : UInt16) (items : UInt16) (iter1 : (List UInt8)) : Rust.M UInt16 := do
  let mut items := items
  let mut iter1 := iter1
  let r1 := Rust.listNext iter1
  iter1 := r1.1
  let t33 := r1.2
  match id t33 with
  | none =>
    return items
  | some x =>
    let t41 := items
    let t40 := (← t41 *? 3)
    let t43 := x.toUInt16
    let t39 := (← t40 +? t43)
    unless ¬((1000 : UInt16) = (0 : UInt16)) do throw "attempt to calculate the remainder with a divisor of zero"
    items := t39 % 1000
    return (← cases.text.loops_loop1 v m s o q range items iter1)
partial_fixpoint

def cases.text.loops_loop2 (v : (Array UInt8)) (m : (Rust.BTreeMap UInt8 UInt16)) (s : (Rust.BTreeSet UInt8)) (o : (Option UInt8)) (q : (Rust.VecDeque UInt8)) (range : UInt16) (items : UInt16) (iter2 : (List UInt8)) : Rust.M UInt16 := do
  let mut items := items
  let mut iter2 := iter2
  let r2 := Rust.listNext iter2
  iter2 := r2.1
  let t52 := r2.2
  match id t52 with
  | none =>
    return items
  | some x1 =>
    items := items ^^^ (x1.toUInt16)
    return (← cases.text.loops_loop2 v m s o q range items iter2)
partial_fixpoint

def cases.text.loops_loop3 (m : (Rust.BTreeMap UInt8 UInt16)) (s : (Rust.BTreeSet UInt8)) (o : (Option UInt8)) (q : (Rust.VecDeque UInt8)) (range : UInt16) (items : UInt16) (iter3 : (List UInt8)) : Rust.M UInt16 := do
  let mut items := items
  let mut iter3 := iter3
  let r3 := Rust.listNext iter3
  iter3 := r3.1
  let t65 := r3.2
  match id t65 with
  | none =>
    return items
  | some x2 =>
    let t72 := items
    let t73 := x2.toUInt16
    let t71 := (← t72 +? t73)
    unless ¬((1000 : UInt16) = (0 : UInt16)) do throw "attempt to calculate the remainder with a divisor of zero"
    items := t71 % 1000
    return (← cases.text.loops_loop3 m s o q range items iter3)
partial_fixpoint

def cases.text.loops_loop4 (m : (Rust.BTreeMap UInt8 UInt16)) (s : (Rust.BTreeSet UInt8)) (o : (Option UInt8)) (q : (Rust.VecDeque UInt8)) (range : UInt16) (items : UInt16) (map : UInt16) (iter4 : (List (UInt8 × UInt16))) : Rust.M UInt16 := do
  let mut map := map
  let mut iter4 := iter4
  let r4 := Rust.listNext iter4
  iter4 := r4.1
  let t83 := r4.2
  match id t83 with
  | none =>
    return map
  | some t83_0 =>
    let k := t83_0.1
    let x3 := t83_0.2
    let t93 := map
    let t90 := ((← t93 *? 7) ^^^ (k.toUInt16)) ^^^ x3
    unless ¬((1000 : UInt16) = (0 : UInt16)) do throw "attempt to calculate the remainder with a divisor of zero"
    map := t90 % 1000
    return (← cases.text.loops_loop4 m s o q range items map iter4)
partial_fixpoint

def cases.text.loops_loop5 (m : (Rust.BTreeMap UInt8 UInt16)) (s : (Rust.BTreeSet UInt8)) (o : (Option UInt8)) (q : (Rust.VecDeque UInt8)) (range : UInt16) (items : UInt16) (map : UInt16) (iter5 : (List UInt8)) : Rust.M UInt16 := do
  let mut map := map
  let mut iter5 := iter5
  let r5 := Rust.listNext iter5
  iter5 := r5.1
  let t105 := r5.2
  match id t105 with
  | none =>
    return map
  | some k1 =>
    let t111 := k1.toUInt16
    map := (← map +? t111)
    return (← cases.text.loops_loop5 m s o q range items map iter5)
partial_fixpoint

def cases.text.loops_loop6 (m : (Rust.BTreeMap UInt8 UInt16)) (s : (Rust.BTreeSet UInt8)) (o : (Option UInt8)) (q : (Rust.VecDeque UInt8)) (range : UInt16) (items : UInt16) (map : UInt16) (iter6 : (List (UInt8 × UInt16))) : Rust.M UInt16 := do
  let mut map := map
  let mut iter6 := iter6
  let r6 := Rust.listNext iter6
  iter6 := r6.1
  let t120 := r6.2
  match id t120 with
  | none =>
    return map
  | some t120_0 =>
    let k2 := t120_0.1
    let x4 := t120_0.2
    let t128 := map ^^^ x4
    unless ¬((1000 : UInt16) = (0 : UInt16)) do throw "attempt to calculate the remainder with a divisor of zero"
    let t127 := t128 % 1000
    let t132 := k2.toUInt16
    map := (← t127 +? t132)
    return (← cases.text.loops_loop6 m s o q range items map iter6)
partial_fixpoint

def cases.text.loops_loop7 (s : (Rust.BTreeSet UInt8)) (o : (Option UInt8)) (q : (Rust.VecDeque UInt8)) (range : UInt16) (items : UInt16) (map : UInt16) (iter7 : (List UInt8)) : Rust.M UInt16 := do
  let mut map := map
  let mut iter7 := iter7
  let r7 := Rust.listNext iter7
  iter7 := r7.1
  let t142 := r7.2
  match id t142 with
  | none =>
    return map
  | some k3 =>
    map := map ^^^ (k3.toUInt16)
    return (← cases.text.loops_loop7 s o q range items map iter7)
partial_fixpoint

def cases.text.loops_loop8 (s : (Rust.BTreeSet UInt8)) (o : (Option UInt8)) (q : (Rust.VecDeque UInt8)) (range : UInt16) (items : UInt16) (map : UInt16) (set : UInt16) (iter8 : (List UInt8)) : Rust.M UInt16 := do
  let mut set := set
  let mut iter8 := iter8
  let r8 := Rust.listNext iter8
  iter8 := r8.1
  let t156 := r8.2
  match id t156 with
  | none =>
    return set
  | some k4 =>
    let t164 := set
    let t163 := (← t164 *? 5)
    let t166 := k4.toUInt16
    let t162 := (← t163 +? t166)
    unless ¬((1000 : UInt16) = (0 : UInt16)) do throw "attempt to calculate the remainder with a divisor of zero"
    set := t162 % 1000
    return (← cases.text.loops_loop8 s o q range items map set iter8)
partial_fixpoint

def cases.text.loops_loop9 (o : (Option UInt8)) (q : (Rust.VecDeque UInt8)) (range : UInt16) (items : UInt16) (map : UInt16) (set : UInt16) (iter9 : (List UInt8)) : Rust.M UInt16 := do
  let mut set := set
  let mut iter9 := iter9
  let r9 := Rust.listNext iter9
  iter9 := r9.1
  let t175 := r9.2
  match id t175 with
  | none =>
    return set
  | some k5 =>
    set := set ^^^ (k5.toUInt16)
    return (← cases.text.loops_loop9 o q range items map set iter9)
partial_fixpoint

def cases.text.loops_loop10 (o : (Option UInt8)) (q : (Rust.VecDeque UInt8)) (range : UInt16) (items : UInt16) (map : UInt16) (set : UInt16) (iter10 : (List UInt8)) : Rust.M UInt16 := do
  let mut set := set
  let mut iter10 := iter10
  let r10 := Rust.listNext iter10
  iter10 := r10.1
  let t188 := r10.2
  match id t188 with
  | none =>
    return set
  | some x5 =>
    let t194 := x5.toUInt16
    set := (← set +? t194)
    return (← cases.text.loops_loop10 o q range items map set iter10)
partial_fixpoint

def cases.text.loops_loop11 (q : (Rust.VecDeque UInt8)) (range : UInt16) (items : UInt16) (map : UInt16) (set : UInt16) (iter11 : (List UInt8)) : Rust.M UInt16 := do
  let mut set := set
  let mut iter11 := iter11
  let r11 := Rust.listNext iter11
  iter11 := r11.1
  let t202 := r11.2
  match id t202 with
  | none =>
    return set
  | some x6 =>
    set := set ^^^ (x6.toUInt16)
    return (← cases.text.loops_loop11 q range items map set iter11)
partial_fixpoint

def cases.text.loops_loop12 (q : (Rust.VecDeque UInt8)) (range : UInt16) (items : UInt16) (map : UInt16) (set : UInt16) (iter12 : (List UInt8)) : Rust.M (UInt16 × UInt16 × UInt16 × UInt16 × UInt16) := do
  let mut set := set
  let mut iter12 := iter12
  let r12 := Rust.listNext iter12
  iter12 := r12.1
  let t215 := r12.2
  match id t215 with
  | none =>
    return ((range, items, map, set, (q.items.length.toUInt64).toUInt16))
  | some x7 =>
    let t223 := set
    let t221 := (← t223 *? 3) ^^^ (x7.toUInt16)
    unless ¬((1000 : UInt16) = (0 : UInt16)) do throw "attempt to calculate the remainder with a divisor of zero"
    set := t221 % 1000
    return (← cases.text.loops_loop12 q range items map set iter12)
partial_fixpoint

def cases.text.loops (n : UInt8) (v : (Array UInt8)) (m : (Rust.BTreeMap UInt8 UInt16)) (s : (Rust.BTreeSet UInt8)) (o : (Option UInt8)) (q : (Rust.VecDeque UInt8)) : Rust.M (UInt16 × UInt16 × UInt16 × UInt16 × UInt16) := do
  let mut range : UInt16 := 0
  let iter := ((0 : UInt8), n)
  let r13 ← cases.text.loops_loop0 v m s o q range iter
  range := r13
  let mut items : UInt16 := 0
  let t29 := v
  let iter1 := t29.toList
  let r14 ← cases.text.loops_loop1 v m s o q range items iter1
  items := r14
  let iter2 := v.toList
  let r15 ← cases.text.loops_loop2 v m s o q range items iter2
  items := r15
  let iter3 := v.toList
  let r16 ← cases.text.loops_loop3 m s o q range items iter3
  items := r16
  let mut map : UInt16 := 0
  let iter4 := m.entries
  let r17 ← cases.text.loops_loop4 m s o q range items map iter4
  map := r17
  let iter5 := m.entries.map (·.1)
  let r18 ← cases.text.loops_loop5 m s o q range items map iter5
  map := r18
  let iter6 := m.entries
  let r19 ← cases.text.loops_loop6 m s o q range items map iter6
  map := r19
  let iter7 := m.entries.map (·.1)
  let r20 ← cases.text.loops_loop7 s o q range items map iter7
  map := r20
  let mut set : UInt16 := 0
  let iter8 := s.items
  let r21 ← cases.text.loops_loop8 s o q range items map set iter8
  set := r21
  let iter9 := s.items
  let r22 ← cases.text.loops_loop9 o q range items map set iter9
  set := r22
  let iter10 := o.toList
  let r23 ← cases.text.loops_loop10 o q range items map set iter10
  set := r23
  let iter11 := o.toList
  let r24 ← cases.text.loops_loop11 q range items map set iter11
  set := r24
  let iter12 := q.items
  let r25 ← cases.text.loops_loop12 q range items map set iter12
  return r25

def cases.text.classify (c : Char) : (UInt8 × UInt32 × UInt8 × Int8 × Int64 × Char) := Id.run do
  let mut «class» : UInt8 := default
  if c = 'a' then
    «class» := (1 : UInt8)
  else if c = '/' then
    «class» := (2 : UInt8)
  else if c = (Char.ofNat 92) then
    «class» := (2 : UInt8)
  else if c = (Char.ofNat 233) then
    «class» := (3 : UInt8)
  else if c = (Char.ofNat 1114111) then
    «class» := (4 : UInt8)
  else
    «class» := (0 : UInt8)
  return («class», c.val, (c.val).toUInt8, Rust.cast Int8 (c.val), Rust.cast Int64 (c.val), Char.ofNat ((c.val).toUInt8).toNat)

def cases.text.shadowed {Char'' : Type} [Inhabited Char''] {UInt8'' : Type} [Inhabited UInt8''] {Int'' : Type} [Inhabited Int''] {Sub'' : Type} [Inhabited Sub''] (s : String) (b : UInt8) (arg3 : Char'') (arg4 : UInt8'') (arg5 : Int'') (x : Int') (arg7 : Sub'') (y : Sub) : (Bool × Char × UInt8 × UInt8) :=
  (s.toList.head? == some (Char.ofNat b.toNat), Char.ofNat b.toNat, x.x0, y.turn)

def cases.text.Countdown.Iterator.next (self : cases.text.Countdown) : ((Option UInt8) × cases.text.Countdown) := Id.run do
  let mut self := self
  let t3 := Rust.branch (Rust.checkedSub self.x0 (1 : UInt8))
  match t3 with
  | .Continue val =>
    self := { self with x0 := val }
    return ((some self.x0 : (Option UInt8)), self)
  | .Break t3_01 =>
    return ((none : (Option UInt8)), self)

def cases.text.countdown_loop (t : UInt16) (iter : cases.text.Countdown) : Rust.M UInt16 := do
  let mut t := t
  let mut iter := iter
  let r := cases.text.Countdown.Iterator.next iter
  let t10 := r.1
  iter := r.2
  match id t10 with
  | none =>
    return (t)
  | some x =>
    let t17 := x.toUInt16
    let t16 := (← t17 *? 2)
    t := (← t +? t16)
    return (← cases.text.countdown_loop t iter)
partial_fixpoint

def cases.text.countdown (n : UInt8) : Rust.M UInt16 := do
  let t : UInt16 := 0
  let iter := ({ x0 := n } : cases.text.Countdown)
  let r1 ← cases.text.countdown_loop t iter
  return r1

def cases.adaptors.sift.closure0 (arg1 : UInt8) (arg2 : UInt8) : Bool := Id.run do
  let t6 := arg2
  let b := t6
  let t7 := arg1
  return decide (b > t7)

def cases.adaptors.sift.closure1 (arg1 : UInt8) (arg2 : UInt8) : (Option UInt8) := Id.run do
  let b := arg2
  let t6 := arg1
  return Rust.checkedAdd b t6

def cases.adaptors.sift.closure2 (arg1 : UInt8) (arg2 : UInt8) : (Vector UInt8 2) := Id.run do
  let b := arg2
  let t6 := arg1
  return #v[b, t6]

def cases.adaptors.sift.closure3 (arg1 : UInt8) (arg2 : UInt8) : Bool := Id.run do
  let b := arg2
  let t6 := arg1
  return decide (b = t6)

def cases.adaptors.sift.closure4 (arg1 : UInt8) (arg2 : UInt8) : (Option UInt8) := Id.run do
  let b := arg2
  let t6 := arg1
  return Rust.checkedSub b t6

def cases.adaptors.sift (v : (Array UInt8)) (x : UInt8) : ((Array UInt8) × (Array UInt8) × (Array UInt8) × (Option UInt8) × (Option UInt8)) := Id.run do
  let t24 := v.toList
  let t31 := v.toList
  return (((v.toList).filter (cases.adaptors.sift.closure0 x)).toArray, ((v.toList).filterMap (cases.adaptors.sift.closure1 x)).toArray, ((v.toList).flatMap (Vector.toList ∘ (cases.adaptors.sift.closure2 x))).toArray, t24.find? (cases.adaptors.sift.closure3 x), t31.findSome? (cases.adaptors.sift.closure4 x))

def cases.adaptors.filtered.closure0 (arg1 : UInt8) (arg2 : UInt8) : Rust.M UInt8 := do
  let b := arg2
  let t7 := arg1
  let t5 := t7
  return (← b +? t5)

def cases.adaptors.filtered.closure1 (arg1 : Unit) (arg2 : UInt8) : Rust.M Bool := do
  let b := arg2
  unless ¬((2 : UInt8) = (0 : UInt8)) do throw "attempt to calculate the remainder with a divisor of zero"
  return decide ((b % 2) = 0)

def cases.adaptors.filtered (v : (Array UInt8)) (x : UInt8) : Rust.M (Array UInt8) := do
  return (← Rust.run (Rust.lazyFilter (cases.adaptors.filtered.closure1 ()) (Rust.lazyMap (cases.adaptors.filtered.closure0 x) (Rust.lazy (v.toList))))).toArray

def cases.adaptors.any_filtered.closure0 (arg1 : UInt8) (arg2 : UInt8) : Rust.M Bool := do
  let t8 := arg2
  let b := t8
  let t9 := arg1
  let t6 := t9
  return decide ((← b +? t6) > 200)

def cases.adaptors.any_filtered.closure1 (arg1 : Unit) (arg2 : UInt8) : Bool := Id.run do
  let b := arg2
  return decide (b = 0)

def cases.adaptors.any_filtered (v : (Array UInt8)) (x : UInt8) : Rust.M Bool := do
  let t4 := Rust.lazyFilter (cases.adaptors.any_filtered.closure0 x) (Rust.lazy (v.toList))
  return (← Rust.lazyAny (pure ∘ (cases.adaptors.any_filtered.closure1 ())) t4)

def cases.adaptors.find_filter_mapped.closure0 (arg1 : UInt8) (arg2 : UInt8) : Rust.M (Option UInt8) := do
  let b := arg2
  let t8 := arg1
  return Rust.checkedSub (← b +? 1) t8

def cases.adaptors.find_filter_mapped.closure1 (arg1 : Unit) (arg2 : UInt8) : Bool := Id.run do
  let p := arg2
  return decide (p > 100)

def cases.adaptors.find_filter_mapped (v : (Array UInt8)) (x : UInt8) : Rust.M (Option UInt8) := do
  let t4 := Rust.lazyFilterMap (cases.adaptors.find_filter_mapped.closure0 x) (Rust.lazy (v.toList))
  return (← Rust.lazyFind (pure ∘ (cases.adaptors.find_filter_mapped.closure1 ())) t4)

def cases.adaptors.find_mapped.closure0 (arg1 : UInt8) (arg2 : UInt8) : Rust.M (Option UInt8) := do
  let b := arg2
  let t8 := arg1
  let t6 := t8
  return Rust.checkedSub (← b +? t6) (100 : UInt8)

def cases.adaptors.find_mapped (v : (Array UInt8)) (x : UInt8) : Rust.M (Option UInt8) := do
  let t4 := v.toList
  return (← List.findSomeM? (cases.adaptors.find_mapped.closure0 x) t4)

def cases.adaptors.found.closure0 (arg1 : UInt8) (arg2 : UInt8) : Rust.M Bool := do
  let b := arg2
  let t8 := arg1
  let t6 := t8
  return decide ((← b +? t6) = 255)

def cases.adaptors.found (v : (Array UInt8)) (x : UInt8) : Rust.M (Option UInt8) := do
  let t4 := v.toList
  return (← List.findM? (cases.adaptors.found.closure0 x) t4)

def cases.adaptors.flat_runs.closure0 (arg1 : UInt8) (arg2 : UInt8) : Rust.M UInt8 := do
  let b := arg2
  let t7 := arg1
  let t5 := t7
  return (← b +? t5)

def cases.adaptors.flat_runs.closure1 (arg1 : Unit) (b : UInt8) : Rust.M (Vector UInt8 2) := do
  unless ¬((2 : UInt8) = (0 : UInt8)) do throw "attempt to divide by zero"
  return #v[b, b / 2]

def cases.adaptors.flat_runs (v : (Array UInt8)) (x : UInt8) : Rust.M (Array UInt8) := do
  return (← Rust.run (Rust.lazyFlatMap (cases.adaptors.flat_runs.closure1 ()) Vector.toList (Rust.lazyMap (cases.adaptors.flat_runs.closure0 x) (Rust.lazy (v.toList))))).toArray

def cases.adaptors.flat_back.closure0 (arg1 : UInt8) (arg2 : UInt8) : Rust.M (Vector UInt8 2) := do
  let b := arg2
  let t9 := arg1
  let t6 := t9
  return #v[(← b +? t6), b]

def cases.adaptors.flat_back.closure1 (arg1 : UInt8) (arg2 : UInt8) : Rust.M (Vector UInt8 2) := do
  let b := arg2
  let t9 := arg1
  let t7 := t9
  return #v[b, (← b -? t7)]

def cases.adaptors.flat_back (v : (Array UInt8)) (x : UInt8) : Rust.M ((Array UInt8) × (Option UInt8)) := do
  let mut t12 := Rust.lazyRev (Rust.lazyFlatMap (cases.adaptors.flat_back.closure1 x) Vector.toList (Rust.lazy (v.toList)))
  let t3 := (← Rust.run (Rust.lazyRev (Rust.lazyFlatMap (cases.adaptors.flat_back.closure0 x) Vector.toList (Rust.lazy (v.toList))))).toArray
  let r ← Rust.listNextM t12
  t12 := r.1
  let t10 := r.2
  return (t3, t10)

def cases.adaptors.flat_first.closure0 (arg1 : UInt8) (arg2 : UInt8) : Rust.M (Option UInt8) := do
  let b := arg2
  let t8 := arg1
  return Rust.checkedSub (← b +? 1) t8

def cases.adaptors.flat_first (v : (Array UInt8)) (x : UInt8) : Rust.M (Option UInt8) := do
  let mut t4 := Rust.lazyFlatMap pure Option.toList (Rust.lazyMap (cases.adaptors.flat_first.closure0 x) (Rust.lazy (v.toList)))
  let r ← Rust.listNextM t4
  t4 := r.1
  return r.2

def cases.adaptors.flat_inner.closure0.closure0 (arg1 : UInt8) (arg2 : UInt8) : Rust.M UInt8 := do
  let c := arg2
  let t5 := arg1
  return (← c +? t5)

def cases.adaptors.flat_inner.closure0 (arg1 : (Array UInt8)) (arg2 : UInt8) : (List (Rust.Lazy UInt8)) := Id.run do
  let b := arg2
  let t7 := arg1
  return Rust.lazyMap (cases.adaptors.flat_inner.closure0.closure0 b) (Rust.lazy (t7.toList))

def cases.adaptors.flat_inner.closure1.closure0 (arg1 : UInt8) (arg2 : UInt8) : Rust.M UInt8 := do
  let c := arg2
  let t5 := arg1
  return (← c *? t5)

def cases.adaptors.flat_inner.closure1 (arg1 : (Array UInt8)) (arg2 : UInt8) : (List (Rust.Lazy UInt8)) := Id.run do
  let b := arg2
  let t7 := arg1
  return Rust.lazyMap (cases.adaptors.flat_inner.closure1.closure0 b) (Rust.lazy (t7.toList))

def cases.adaptors.flat_inner.closure2 (arg1 : Unit) (c : UInt8) : Bool :=
  decide (c = 0)

def cases.adaptors.flat_inner (v : (Array UInt8)) (w : (Array UInt8)) : Rust.M (UInt64 × Bool) := do
  let t11 := (v.toList).flatMap (cases.adaptors.flat_inner.closure1 w)
  let t3 := (← Rust.run ((v.toList).flatMap (cases.adaptors.flat_inner.closure0 w))).length.toUInt64
  return (t3, (← Rust.lazyAny (pure ∘ (cases.adaptors.flat_inner.closure2 ())) t11))

def cases.adaptors.least.closure0 (arg1 : Unit) (arg2 : UInt8) : Rust.M UInt16 := do
  let b := arg2
  let t4 := b.toUInt16
  return (← t4 *? 200)

def cases.adaptors.least.closure1 (arg1 : UInt8) (arg2 : UInt8) : UInt8 := Id.run do
  let b := arg2
  let t6 := arg1
  return b ^^^ t6

def cases.adaptors.least.closure2 (arg1 : Unit) (arg2 : UInt8) : Rust.M UInt8 := do
  let b := arg2
  unless ¬((4 : UInt8) = (0 : UInt8)) do throw "attempt to calculate the remainder with a divisor of zero"
  return b % 4

def cases.adaptors.least (v : (Array UInt8)) (x : UInt8) : Rust.M (UInt8 × UInt16 × (Option UInt8) × (Option UInt8)) := do
  let t3 := (← Rust.sum (v.toList))
  let t6 := (← Rust.lazyFold (· +? ·) (Rust.Scalar.ofInt 0) (Rust.lazyMap (cases.adaptors.least.closure0 ()) (Rust.lazy (v.toList))))
  return (t3, t6, Rust.minByKey id ((v.toList).map (cases.adaptors.least.closure1 x)), (← Rust.lazyMinByKey (cases.adaptors.least.closure2 ()) (Rust.lazy (v.toList))))

def cases.adaptors.sum_lazy.closure0 (arg1 : UInt8) (arg2 : UInt8) : Rust.M UInt8 := do
  let b := arg2
  let t7 := arg1
  let t5 := t7
  return (← b -? t5)

def cases.adaptors.sum_lazy (v : (Array UInt8)) (x : UInt8) : Rust.M UInt8 := do
  return (← Rust.lazyFold (· +? ·) (Rust.Scalar.ofInt 0) (Rust.lazyMap (cases.adaptors.sum_lazy.closure0 x) (Rust.lazy (v.toList))))

def cases.adaptors.least_lazy.closure0 (arg1 : UInt8) (arg2 : UInt8) : Rust.M UInt8 := do
  let b := arg2
  let t7 := arg1
  let t5 := t7
  return (← b -? t5)

def cases.adaptors.least_lazy.closure1 (arg1 : Unit) (arg2 : UInt8) : Rust.M (UInt8 × UInt8) := do
  let b := arg2
  let t4 := (← b *? 3)
  unless ¬((50 : UInt8) = (0 : UInt8)) do throw "attempt to divide by zero"
  return (t4, b / 50)

def cases.adaptors.least_lazy (v : (Array UInt8)) (x : UInt8) : Rust.M (Option UInt8) := do
  return (← Rust.lazyMinByKey (cases.adaptors.least_lazy.closure1 ()) (Rust.lazyMap (cases.adaptors.least_lazy.closure0 x) (Rust.lazy (v.toList))))

def cases.adaptors.sources.closure0 (arg1 : Unit) (i : UInt8) : Rust.M UInt8 := do
  unless ¬((2 : UInt8) = (0 : UInt8)) do throw "attempt to divide by zero"
  return i / 2

def cases.adaptors.sources.closure1 (arg1 : Unit) (arg2 : UInt8) : Rust.M Bool := do
  let i := arg2
  unless ¬((3 : UInt8) = (0 : UInt8)) do throw "attempt to calculate the remainder with a divisor of zero"
  return decide ((i % 3) = 0)

def cases.adaptors.sources.closure2 (arg1 : (Rust.BTreeMap UInt8 UInt16)) (i : UInt8) : (Option UInt16) := Id.run do
  let t7 := arg1
  let t6 := i
  return t7.get t6

def cases.adaptors.sources (m : (Rust.BTreeMap UInt8 UInt16)) (a : UInt8) (b : UInt8) : Rust.M ((Array UInt8) × (Array UInt8) × (Array UInt16) × UInt64) := do
  let t4 := (← Rust.run ((Rust.lazy [a]) ++ (Rust.lazyMap (cases.adaptors.sources.closure0 ()) (Rust.lazy (Rust.rangeItems (a, b)))))).toArray
  let t13 := (← Rust.run (Rust.lazyRev (Rust.lazyFilter (cases.adaptors.sources.closure1 ()) (Rust.lazy (Rust.rangeInclusiveItems (a, b)))))).toArray
  return (t4, t13, ((Rust.rangeInclusiveItems ((0 : UInt8), a)).filterMap (cases.adaptors.sources.closure2 m)).toArray, (← Rust.drain (pure ∘ (cases.text.Countdown.Iterator.next)) ({ x0 := a } : cases.text.Countdown)).length.toUInt64)

def cases.adaptors.gather.closure0 (arg1 : Unit) (arg2 : UInt8) : Rust.M UInt8 := do
  let b := arg2
  unless ¬((2 : UInt8) = (0 : UInt8)) do throw "attempt to divide by zero"
  return b / 2

def cases.adaptors.gather.closure1 (arg1 : Unit) (arg2 : UInt8) : Rust.M UInt8 := do
  let b := arg2
  return (← b +? 1)

def cases.adaptors.gather.closure2 (arg1 : Unit) (arg2 : UInt8) : Rust.M UInt8 := do
  let b := arg2
  return (← b *? 2)

def cases.adaptors.gather.closure3 (arg1 : Unit) (arg2 : UInt8) : Rust.M UInt8 := do
  let b := arg2
  unless ¬((5 : UInt8) = (0 : UInt8)) do throw "attempt to calculate the remainder with a divisor of zero"
  return b % 5

def cases.adaptors.gather (v : (Array UInt8)) (s : (Rust.BTreeSet UInt8)) (ms : (Array (Rust.BTreeMap UInt8 UInt16))) : Rust.M ((Rust.BTreeSet UInt8) × (Rust.BTreeSet UInt8) × (Array UInt8) × (Array UInt64) × ((Array UInt8) × (Array UInt16) × Bool)) := do
  let mut t := Rust.BTreeSet.ofList #v[(UInt64.ofNat v.size).toUInt8, (7 : UInt8), (7 : UInt8)].toList
  t := t.extend (← Rust.run (Rust.lazyMap (cases.adaptors.gather.closure0 ()) (Rust.lazy (v.toList))))
  t := t.extend (← Rust.run (Rust.lazyMap (cases.adaptors.gather.closure1 ()) (Rust.lazy (v.toList))))
  let mut u := (#[] : (Array UInt8))
  u := u ++ (← Rust.run (Rust.lazyMap (cases.adaptors.gather.closure2 ()) (Rust.lazy (v.toList)))).toArray
  u := u ++ (s.items).toArray
  return (t, Rust.BTreeSet.ofList (← Rust.run (Rust.lazyMap (cases.adaptors.gather.closure3 ()) (Rust.lazy (v.toList)))), u, ((ms.toList).map Rust.BTreeMap.len).toArray, (((ms.toList).flatMap Rust.BTreeMap.keys).toArray, ((ms.toList).flatMap Rust.BTreeMap.values).toArray, Option.any Rust.BTreeMap.isEmpty (ms[0]?)))

def cases.adaptors.peeked.closure0 (arg1 : UInt8) (arg2 : UInt8) : Bool := Id.run do
  let b := arg2
  let t6 := arg1
  return decide (b ≠ t6)

def cases.adaptors.peeked (v : (Array UInt8)) (x : UInt8) : ((Option UInt8) × (Array UInt8)) := Id.run do
  let mut it := (v.toList).filter (cases.adaptors.peeked.closure0 x)
  let r := Rust.listPeek it
  it := r.1
  let t11 := r.2
  let first := t11
  return (first, it.toArray)

def cases.adaptors.peeked_lazy.closure0 (arg1 : UInt8) (arg2 : UInt8) : Rust.M UInt8 := do
  let b := arg2
  let t7 := arg1
  let t5 := t7
  return (← b +? t5)

def cases.adaptors.peeked_lazy (v : (Array UInt8)) (x : UInt8) : Rust.M (Option UInt8) := do
  let mut it := Rust.lazyMap (cases.adaptors.peeked_lazy.closure0 x) (Rust.lazy (v.toList))
  let r ← Rust.lazyPeek it
  it := r.1
  let t9 := r.2
  return t9

def cases.adaptors.peeked_rest.closure0 (arg1 : UInt8) (arg2 : UInt8) : Rust.M UInt8 := do
  let b := arg2
  let t7 := arg1
  let t5 := t7
  return (← b +? t5)

def cases.adaptors.peeked_rest (v : (Array UInt8)) (x : UInt8) : Rust.M ((Option UInt8) × (Array UInt8)) := do
  let mut it := Rust.lazyMap (cases.adaptors.peeked_rest.closure0 x) (Rust.lazy (v.toList))
  let r ← Rust.lazyPeek it
  it := r.1
  let t10 := r.2
  let first := t10
  return (first, (← Rust.run it).toArray)

def cases.Cost.raw (self : cases.Cost) : UInt64 :=
  self.x0

end fixture
