-- Translated from the `fixture` crate by rust2-lean, with overflow checks on.
import Rust

set_option linter.unusedVariables false

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

def cases.Cost.Clone.clone (self : cases.Cost) : cases.Cost :=
  self

def cases.Cost.PartialEq.eq (self : cases.Cost) (other : cases.Cost) : Bool :=
  decide (self.x0 = other.x0)

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

def cases.clone_pairs (v : (Array ((Option UInt8) × (Option UInt8)))) : (Array ((Option UInt8) × (Option UInt8))) :=
  v

def cases.Plain.Clone.clone (self : cases.Plain) : cases.Plain :=
  self

def cases.Wrap.Clone.clone (self : cases.Wrap) : cases.Wrap :=
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

def cases.Cost.raw (self : cases.Cost) : UInt64 :=
  self.x0

end fixture
