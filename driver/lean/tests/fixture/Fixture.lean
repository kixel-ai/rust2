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

def Session.opening {S : UInt64} (self : (Session S)) (s : UInt64) : Rust.M UInt8 := do
  unless s < S do throw "index out of bounds"
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
    unless t10 < S do throw "index out of bounds"
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
  unless s < S do throw "index out of bounds"
  unless t < S do throw "index out of bounds"
  return decide (self.subs[s.toNat]!.opening = self.subs[t.toNat]!.opening)

def Session.next {S : UInt64} (self : (Session S)) (s : UInt64) : Rust.M (Option UInt8) := do
  unless s < S do throw "index out of bounds"
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
  unless s < S do throw "index out of bounds"
  self := { self with out := self.out.push self.subs[s.toNat]!.turn }
  unless s < S do throw "index out of bounds"
  if self.subs[s.toNat]!.opening.isNone then
    self := { self with out := self.out.extract 0 len.toNat }
    return (false, self)
  else
    unless s < S do throw "index out of bounds"
    self := { self with subs := self.subs.set! s.toNat { self.subs[s.toNat]! with turn := (← self.subs[s.toNat]!.turn +? 1) } }
    return (true, self)

end fixture
