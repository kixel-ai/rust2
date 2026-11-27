-- Translated from the `fixture` crate by rust2-lean, with overflow checks on.
import Rust

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
    return (← Session.turns_loop self total s)
  else
    return (total)
partial_fixpoint

def Session.turns {S : UInt64} (self : (Session S)) : Rust.M UInt16 := do
  let total : UInt16 := 0
  let s : UInt64 := 0
  let r ← Session.turns_loop self total s
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
  return (← Rust.run ((v.toList).map (cases.shifted.closure0 x))).toArray

def cases.reaches.closure0 (arg1 : UInt8) (y : UInt8) : Rust.M UInt8 := do
  let t5 := arg1
  return (← y +? t5)

def cases.reaches.closure1 (arg1 : Unit) (y : UInt8) : Bool :=
  decide (y = 0)

def cases.reaches (v : (Array UInt8)) (x : UInt8) : Rust.M Bool := do
  let t4 := (v.toList).map (cases.reaches.closure0 x)
  return (← List.anyM (· >>= (pure ∘ (cases.reaches.closure1 ()))) t4)

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
  return (← Rust.run ((← Rust.windows v (2 : UInt64)).map (cases.pair_sums.closure0 ()))).toArray

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

def cases.heavier {W : Type} [Inhabited W] [cases.Weight W] (a : W) (b : W) : Rust.M Bool := do
  let t3 := (← cases.Weight.weight a)
  return decide (t3 > (← cases.Weight.weight b))

def cases.Cost.raw (self : cases.Cost) : UInt64 :=
  self.x0

end fixture
