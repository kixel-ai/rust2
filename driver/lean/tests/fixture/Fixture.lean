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

/-- `cases::Severity` -/
inductive cases.Severity where
  | High
  | Low
deriving Repr, DecidableEq, Inhabited

/-- `cases::Ledger` -/
structure cases.Ledger where
  m : (Rust.BTreeMap cases.Rank UInt16)
  s : (Rust.BTreeSet UInt8)
  q : (Rust.VecDeque UInt8)
deriving Repr, DecidableEq, Inhabited

/-- `cases::lenses::Seeded` -/
structure cases.lenses.Seeded where
  x0 : UInt16
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
  return (← Rust.run ((v.toList).map (Rust.defer (cases.shifted.closure0 x)))).toArray

def cases.reaches.closure0 (arg1 : UInt8) (y : UInt8) : Rust.M UInt8 := do
  let t5 := arg1
  return (← y +? t5)

def cases.reaches.closure1 (arg1 : Unit) (y : UInt8) : Bool :=
  decide (y = 0)

def cases.reaches (v : (Array UInt8)) (x : UInt8) : Rust.M Bool := do
  let t4 := (v.toList).map (Rust.defer (cases.reaches.closure0 x))
  return (← List.anyM (Rust.force (pure ∘ (cases.reaches.closure1 ()))) t4)

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
  return (← Rust.run ((← Rust.windows v (2 : UInt64)).map (Rust.defer (cases.pair_sums.closure0 ())))).toArray

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

def cases.lenses.Book.eager (self : cases.lenses.Book) (k : UInt8) (f : (Option UInt16)) (named : Bool) : Rust.M ((UInt16 × UInt8) × cases.lenses.Book) := do
  let mut self := self
  let arg := ({ x0 := (← Rust.expect f "called `Option::unwrap()` on a `None` value") } : cases.lenses.Seeded)
  self := { self with seeds := self.seeds.orInsertWith k (fun _ => arg) }
  let s := (self.seeds.valueAt k).x0
  let mut name : (Option String) := default
  if named then
    name := (some "a tally" : (Option String))
  else
    name := (none : (Option String))
  let t21 := k
  let t23 := (← Rust.expect name "called `Option::unwrap()` on a `None` value")
  unless (self.tallies.get t21).isSome do throw t23
  return ((s, (self.tallies.valueAt t21).last), self)

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

def cases.Cost.raw (self : cases.Cost) : UInt64 :=
  self.x0

end fixture
