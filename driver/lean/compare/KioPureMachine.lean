-- Translated from the `kio_pure_machine` crate by rust2-lean, with overflow checks on.
import Rust

set_option linter.unusedVariables false

namespace kio_pure_machine

/-- `send_credit::Mutant` -/
inductive send_credit.Mutant where
  | NoGiveWay
  | NoPreemption
  | NoLeaveWake
deriving Repr, DecidableEq, Inhabited

/-- `send_credit::Shape` -/
structure send_credit.Shape (S : UInt64) where
  priority : (Vector UInt8 S.toNat)
  ascending : (Vector Bool S.toNat)
  max_streams : UInt8
  mutant : (Option send_credit.Mutant)
deriving Repr, DecidableEq, Inhabited

/-- `send_credit::Rank` -/
structure send_credit.Rank where
  priority : UInt8
  turn : UInt8
  in_group : Int16
deriving Repr, DecidableEq, Inhabited

/-- `send_credit::Place` -/
structure send_credit.Place where
  rank : send_credit.Rank
  arrival : UInt8
deriving Repr, DecidableEq, Inhabited

/-- `send_credit::Sub` -/
structure send_credit.Sub (G : UInt64) where
  opening : (Option UInt8)
  place : (Option send_credit.Place)
  asked : Bool
  turn : UInt8
  serving : (Vector Bool G.toNat)
deriving Repr, DecidableEq, Inhabited

/-- `send_credit::Session` -/
structure send_credit.Session (S : UInt64) (G : UInt64) where
  shape : (send_credit.Shape S)
  subs : (Vector (send_credit.Sub G) S.toNat)
  arrivals : UInt8
  resets : UInt8
  opened : UInt8
  freed : UInt8
deriving Repr, DecidableEq, Inhabited

/-- `Disabled` -/
structure Disabled where
deriving Repr, DecidableEq, Inhabited

/-- `send_credit::Output` -/
inductive send_credit.Output where
  | Wake (x0 : UInt64)
  | Park (x0 : UInt64)
  | Take (sub : UInt64) (group : UInt8)
  | Shed (sub : UInt64) (group : UInt8)
  | Open (sub : UInt64) (group : UInt8)
  | Reset (sub : UInt64) (group : UInt8)
deriving Repr, DecidableEq, Inhabited

/-- `send_credit::Event` -/
inductive send_credit.Event where
  | Poll (sub : UInt64) (cached : UInt16) (pick : (Option UInt8))
  | Finished (sub : UInt64) (group : UInt8)
  | Freed
deriving Repr, DecidableEq, Inhabited

/-- `warm_settle::Choice` -/
structure warm_settle.Choice where
  via : UInt8
  cost : UInt32
  copy : Bool
  rank : UInt32
deriving Repr, DecidableEq, Inhabited

/-- `warm_settle::Link` -/
structure warm_settle.Link where
  «open» : Bool
  cold : UInt32
  warm : UInt32
deriving Repr, DecidableEq, Inhabited

/-- `warm_settle::Sig` -/
structure warm_settle.Sig where
  cost : UInt32
  path : UInt16
  rank : UInt32
deriving Repr, DecidableEq, Inhabited

/-- `warm_settle::Answer` -/
inductive warm_settle.Answer where
  | None
  | Wait
  | Live (x0 : warm_settle.Sig)
  | Lost
deriving Repr, DecidableEq, Inhabited

/-- `warm_settle::Heard` -/
structure warm_settle.Heard where
  pull : (Option UInt32)
  answer : warm_settle.Answer
  advert : (Option warm_settle.Sig)
deriving Repr, DecidableEq, Inhabited

/-- `warm_settle::Relay` -/
structure warm_settle.Relay (N : UInt64) where
  id : UInt8
  reader : (Option UInt32)
  route : (Option (UInt8 × UInt32))
  links : (Vector (Option warm_settle.Link) N.toNat)
  heard : (Vector warm_settle.Heard N.toNat)
  up : (Option UInt8)
  next : (Option UInt8)
  answered : UInt16
deriving Repr, DecidableEq, Inhabited

/-- `warm_settle::Pull` -/
inductive warm_settle.Pull where
  | Live
  | Looped
  | Lost
  | Pending
deriving Repr, DecidableEq, Inhabited

/-- `warm_settle::Event` -/
inductive warm_settle.Event where
  | Hear («from» : UInt8) (heard : warm_settle.Heard)
  | React
deriving Repr, DecidableEq, Inhabited

def send_credit.Session.new {S : UInt64} {G : UInt64} (shape : (send_credit.Shape S)) : (send_credit.Session S G) := Id.run do
  let sub := ({ opening := (none : (Option UInt8)), place := (none : (Option send_credit.Place)), asked := false, turn := (0 : UInt8), serving := Vector.replicate G.toNat false } : (send_credit.Sub G))
  return ({ shape := shape, subs := Vector.replicate S.toNat sub, arrivals := (0 : UInt8), resets := (0 : UInt8), opened := (0 : UInt8), freed := (0 : UInt8) } : (send_credit.Session S G))

def send_credit.Session.sub {S : UInt64} {G : UInt64} (self : (send_credit.Session S G)) (s : UInt64) : Rust.M (send_credit.Sub G) := do
  unless s < S do throw s!"index out of bounds: the len is {S} but the index is {s}"
  return self.subs[s.toNat]!

def send_credit.Session.freed'_loop {S : UInt64} {G : UInt64} (self : (send_credit.Session S G)) (out : (Array send_credit.Output)) (s : UInt64) : Rust.M ((Except Disabled Unit) × (send_credit.Session S G) × (Array send_credit.Output)) := do
  let mut self := self
  let mut out := out
  let mut s := s
  if s < S then
    let t17 := (← self.sub s)
    if t17.asked then
      let t20 := s
      unless t20 < S do throw s!"index out of bounds: the len is {S} but the index is {t20}"
      self := { self with subs := self.subs.set! t20.toNat { self.subs[t20.toNat]! with asked := false } }
      out := out.push (send_credit.Output.Wake s : send_credit.Output)
    s := (← s +? 1)
    return (← send_credit.Session.freed'_loop self out s)
  else
    return (((Except.ok () : (Except Disabled Unit)), self, out))
partial_fixpoint

def send_credit.Session.freed' {S : UInt64} {G : UInt64} (self : (send_credit.Session S G)) (out : (Array send_credit.Output)) : Rust.M ((Except Disabled Unit) × (send_credit.Session S G) × (Array send_credit.Output)) := do
  let mut self := self
  if self.freed ≥ self.opened then
    return ((Except.error ({} : Disabled) : (Except Disabled Unit)), self, out)
  else
    self := { self with freed := (← self.freed +? 1) }
    let s : UInt64 := 0
    let r ← send_credit.Session.freed'_loop self out s
    return r

def send_credit.is {T : Type} [Inhabited T] [DecidableEq T] (x : (Option T)) (v : T) : Bool := Id.run do
  match x with
  | none =>
    return false
  | some y =>
    return decide (y = v)

def send_credit.Session.taken_loop {S : UInt64} {G : UInt64} (self : (send_credit.Session S G)) (s : UInt64) (taken : UInt16) (j : UInt64) : Rust.M UInt16 := do
  let mut taken := taken
  let mut j := j
  if j < G then
    let t11 := (← self.sub s)
    let t14 := j
    unless t14 < G do throw s!"index out of bounds: the len is {G} but the index is {t14}"
    if t11.serving[t14.toNat]! then
      let mut t26 := j
      let mut t27 := decide (t26 < 16)
      unless t27 do throw "attempt to shift left with overflow"
      taken := taken ||| (1 <<< t26.toUInt16)
    else
      let t18 := (← self.sub s)
      let t23 := j
      let d := send_credit.is t18.opening ((← t23 +? 1).toUInt8)
      if d then
        let mut t26 := j
        let mut t27 := decide (t26 < 16)
        unless t27 do throw "attempt to shift left with overflow"
        taken := taken ||| (1 <<< t26.toUInt16)
    j := (← j +? 1)
    return (← send_credit.Session.taken_loop self s taken j)
  else
    return (taken)
partial_fixpoint

def send_credit.Session.taken {S : UInt64} {G : UInt64} (self : (send_credit.Session S G)) (s : UInt64) : Rust.M UInt16 := do
  let taken : UInt16 := 0
  let j : UInt64 := 0
  let r ← send_credit.Session.taken_loop self s taken j
  return r

def send_credit.Session.placed_loop {S : UInt64} {G : UInt64} (self : (send_credit.Session S G)) (n : UInt8) (s : UInt64) : Rust.M UInt8 := do
  let mut n := n
  let mut s := s
  if s < S then
    let t11 := (← self.sub s)
    if t11.place.isSome then
      n := (← n +? 1)
    s := (← s +? 1)
    return (← send_credit.Session.placed_loop self n s)
  else
    return (n)
partial_fixpoint

def send_credit.Session.placed {S : UInt64} {G : UInt64} (self : (send_credit.Session S G)) : Rust.M UInt8 := do
  let n : UInt8 := 0
  let s : UInt64 := 0
  let r ← send_credit.Session.placed_loop self n s
  return r

def send_credit.Session.credit {S : UInt64} {G : UInt64} (self : (send_credit.Session S G)) : Rust.M Bool := do
  let t5 := self.shape.max_streams.toUInt16
  let t7 := self.freed.toUInt16
  return decide ((self.opened.toUInt16) < (← t5 +? t7))

def send_credit.Session.«open» {S : UInt64} {G : UInt64} (self : (send_credit.Session S G)) (s : UInt64) (k : UInt8) (out : (Array send_credit.Output)) : Rust.M ((send_credit.Session S G) × (Array send_credit.Output)) := do
  let mut self := self
  let mut out := out
  self := { self with opened := (← self.opened +? 1) }
  unless s < S do throw s!"index out of bounds: the len is {S} but the index is {s}"
  let t9 := (← k -? 1).toUInt64
  unless t9 < G do throw s!"index out of bounds: the len is {G} but the index is {t9}"
  self := { self with subs := self.subs.set! s.toNat { self.subs[s.toNat]! with serving := self.subs[s.toNat]!.serving.set! t9.toNat true } }
  self := { self with subs := self.subs.set! s.toNat { self.subs[s.toNat]! with opening := (none : (Option UInt8)) } }
  self := { self with subs := self.subs.set! s.toNat { self.subs[s.toNat]! with turn := self.opened } }
  out := out.push (send_credit.Output.Open s k : send_credit.Output)
  return (self, out)

def send_credit.Session.in_group {S : UInt64} {G : UInt64} (self : (send_credit.Session S G)) (s : UInt64) (k : UInt8) : Rust.M Int16 := do
  unless s < S do throw s!"index out of bounds: the len is {S} but the index is {s}"
  if self.shape.ascending[s.toNat]! then
    let t7 := Rust.cast Int16 k
    unless ¬(t7 = (-32768)) do throw "attempt to negate with overflow"
    return -t7
  else
    return Rust.cast Int16 k

def send_credit.Session.rank {S : UInt64} {G : UInt64} (self : (send_credit.Session S G)) (s : UInt64) (k : UInt8) : Rust.M send_credit.Rank := do
  unless s < S do throw s!"index out of bounds: the len is {S} but the index is {s}"
  let t8 := (← self.sub s)
  return ({ priority := self.shape.priority[s.toNat]!, turn := t8.turn, in_group := (← self.in_group s k) } : send_credit.Rank)

def send_credit.Session.join {S : UInt64} {G : UInt64} (self : (send_credit.Session S G)) (s : UInt64) (k : UInt8) : Rust.M (send_credit.Session S G) := do
  let mut self := self
  let t4 := (some ({ rank := (← self.rank s k), arrival := self.arrivals } : send_credit.Place) : (Option send_credit.Place))
  unless s < S do throw s!"index out of bounds: the len is {S} but the index is {s}"
  self := { self with subs := self.subs.set! s.toNat { self.subs[s.toNat]! with place := t4 } }
  self := { self with arrivals := (← self.arrivals +? 1) }
  return self

def send_credit.above (a : send_credit.Rank) (b : send_credit.Rank) : Bool := Id.run do
  if a.priority > b.priority then
    return true
  else if a.priority = b.priority then
    if a.turn < b.turn then
      return true
    else if a.turn = b.turn then
      return decide (a.in_group > b.in_group)
    else
      return false
  else
    return false

def send_credit.same (a : send_credit.Rank) (b : send_credit.Rank) : Bool := Id.run do
  if a.priority = b.priority then
    if a.turn = b.turn then
      return decide (a.in_group = b.in_group)
    else
      return false
  else
    return false

def send_credit.Session.before {S : UInt64} {G : UInt64} (self : (send_credit.Session S G)) (s : UInt64) (t : UInt64) : Rust.M Bool := do
  let t6 := (← self.sub s)
  let t10 := (← self.sub t)
  let t4 := (t6.place, t10.place)
  match t4.1 with
  | some t4_0 =>
    match t4.2 with
    | some t4_01 =>
      let a := t4_0
      let b := t4_01
      if send_credit.above a.rank b.rank then
        return true
      else if send_credit.same a.rank b.rank then
        return decide (a.arrival < b.arrival)
      else
        return false
    | none =>
      return false
  | none =>
    return false

def send_credit.Session.ahead_loop {S : UInt64} {G : UInt64} (self : (send_credit.Session S G)) (s : UInt64) (n : UInt8) (t : UInt64) : Rust.M UInt8 := do
  let mut n := n
  let mut t := t
  if t < S then
    let d := (← self.before t s)
    if d then
      n := (← n +? 1)
    t := (← t +? 1)
    return (← send_credit.Session.ahead_loop self s n t)
  else
    return (n)
partial_fixpoint

def send_credit.Session.ahead {S : UInt64} {G : UInt64} (self : (send_credit.Session S G)) (s : UInt64) : Rust.M UInt8 := do
  let n : UInt8 := 0
  let t : UInt64 := 0
  let r ← send_credit.Session.ahead_loop self s n t
  return r

def send_credit.Session.lowest_loop1 {S : UInt64} {G : UInt64} (self : (send_credit.Session S G)) (only : (Option UInt64)) (low : (Option (UInt64 × UInt8))) (t : UInt64) (j : UInt64) : Rust.M (Option (UInt64 × UInt8)) := do
  let mut low := low
  let mut j := j
  if j < G then
    let t20 := j
    let k := (← t20 +? 1).toUInt8
    let t24 := (← self.sub t)
    let t27 := j
    unless t27 < G do throw s!"index out of bounds: the len is {G} but the index is {t27}"
    if t24.serving[t27.toNat]! then
      let mut t29 : (Option (UInt64 × UInt8)) := default
      match id low with
      | some low_0 =>
        let u1 := low_0.1
        let l1 := low_0.2
        let t37 := (← self.rank u1 l1)
        let d := send_credit.above t37 (← self.rank t k)
        if d then
          t29 := (some (t, k) : (Option (UInt64 × UInt8)))
        else
          let u := low_0.1
          let l := low_0.2
          t29 := (some (u, l) : (Option (UInt64 × UInt8)))
      | none =>
        t29 := (some (t, k) : (Option (UInt64 × UInt8)))
      low := t29
    j := (← j +? 1)
    return (← send_credit.Session.lowest_loop1 self only low t j)
  else
    return low
partial_fixpoint

def send_credit.Session.lowest_loop0 {S : UInt64} {G : UInt64} (self : (send_credit.Session S G)) (only : (Option UInt64)) (low : (Option (UInt64 × UInt8))) (t : UInt64) : Rust.M (Option (UInt64 × UInt8)) := do
  let mut low := low
  let mut t := t
  if t < S then
    if only.isNone then
      let mut j : UInt64 := 0
      let r2 ← send_credit.Session.lowest_loop1 self only low t j
      low := r2
    else if send_credit.is only t then
      let mut j : UInt64 := 0
      let r1 ← send_credit.Session.lowest_loop1 self only low t j
      low := r1
    t := (← t +? 1)
    return (← send_credit.Session.lowest_loop0 self only low t)
  else
    return (low)
partial_fixpoint

def send_credit.Session.lowest {S : UInt64} {G : UInt64} (self : (send_credit.Session S G)) (only : (Option UInt64)) : Rust.M (Option (UInt64 × UInt8)) := do
  let low := (none : (Option (UInt64 × UInt8)))
  let t : UInt64 := 0
  let r ← send_credit.Session.lowest_loop0 self only low t
  return r

def send_credit.Session.preempts {S : UInt64} {G : UInt64} (self : (send_credit.Session S G)) (s : UInt64) (k : UInt8) (t : UInt64) (j : UInt8) : Rust.M Bool := do
  unless s < S do throw s!"index out of bounds: the len is {S} but the index is {s}"
  unless t < S do throw s!"index out of bounds: the len is {S} but the index is {t}"
  if self.shape.priority[s.toNat]! > self.shape.priority[t.toNat]! then
    return true
  else if s = t then
    let t16 := (← self.in_group s k)
    return decide (t16 > (← self.in_group t j))
  else
    return false

def send_credit.Session.candidate {S : UInt64} {G : UInt64} (self : (send_credit.Session S G)) (s : UInt64) (k : UInt8) : Rust.M (Option (UInt64 × UInt8)) := do
  let t7 := (← self.lowest (none : (Option UInt64)))
  match t7 with
  | some t7_0 =>
    let t := t7_0.1
    let j := t7_0.2
    let d := (← self.preempts s k t j)
    if d then
      return (some (t, j) : (Option (UInt64 × UInt8)))
    else
      let t22 := (← self.lowest (some s : (Option UInt64)))
      match t22 with
      | some t22_0 =>
        let j2 := t22_0.2
        let d1 := (← self.preempts s k s j2)
        if d1 then
          let j1 := t22_0.2
          return (some (s, j1) : (Option (UInt64 × UInt8)))
        else
          return (none : (Option (UInt64 × UInt8)))
      | none =>
        return (none : (Option (UInt64 × UInt8)))
  | none =>
    return (none : (Option (UInt64 × UInt8)))

def send_credit.Session.make_room_loop0 {S : UInt64} {G : UInt64} (self : (send_credit.Session S G)) (out : (Array send_credit.Output)) (owed : (Vector Bool S.toNat)) (s : UInt64) : Rust.M (Vector Bool S.toNat) := do
  let mut owed := owed
  let mut s := s
  if s < S then
    let t12 := (← self.sub s)
    let mut t9 : Bool := default
    if t12.place.isSome then
      t9 := decide ((← self.ahead s) ≥ self.resets)
    else
      t9 := false
    let t19 := s
    unless t19 < S do throw s!"index out of bounds: the len is {S} but the index is {t19}"
    owed := owed.set! t19.toNat t9
    s := (← s +? 1)
    return (← send_credit.Session.make_room_loop0 self out owed s)
  else
    return owed
partial_fixpoint

def send_credit.Session.make_room_loop2 {S : UInt64} {G : UInt64} (self : (send_credit.Session S G)) (out : (Array send_credit.Output)) (owed : (Vector Bool S.toNat)) (changed : Bool) (first : (Option UInt64)) (s1 : UInt64) : Rust.M (Option UInt64) := do
  let mut first := first
  let mut s1 := s1
  if s1 < S then
    let t34 := s1
    unless t34 < S do throw s!"index out of bounds: the len is {S} but the index is {t34}"
    if owed[t34.toNat]! then
      let mut t36 : (Option UInt64) := default
      match id first with
      | some first_0 =>
        let f1 := first_0
        let d := (← self.before s1 f1)
        if d then
          t36 := (some s1 : (Option UInt64))
        else
          let f := first_0
          t36 := (some f : (Option UInt64))
      | none =>
        t36 := (some s1 : (Option UInt64))
      first := t36
    s1 := (← s1 +? 1)
    return (← send_credit.Session.make_room_loop2 self out owed changed first s1)
  else
    return first
partial_fixpoint

def send_credit.Session.make_room_loop1 {S : UInt64} {G : UInt64} (self : (send_credit.Session S G)) (out : (Array send_credit.Output)) (owed : (Vector Bool S.toNat)) (changed : Bool) : Rust.M (Bool × (send_credit.Session S G) × (Array send_credit.Output)) := do
  let mut self := self
  let mut out := out
  let mut owed := owed
  let mut changed := changed
  let mut first := (none : (Option UInt64))
  let s1 : UInt64 := 0
  let r2 ← send_credit.Session.make_room_loop2 self out owed changed first s1
  first := r2
  match id first with
  | some first_0 =>
    let s2 := first_0
    unless s2 < S do throw s!"index out of bounds: the len is {S} but the index is {s2}"
    owed := owed.set! s2.toNat false
    let mut t57 : Bool := default
    match self.shape.mutant with
    | some mutant_0 =>
      match id mutant_0 with
      | .NoPreemption =>
        t57 := true
      | _ =>
        t57 := false
    | none =>
      t57 := false
    if t57 then
      return (← send_credit.Session.make_room_loop1 self out owed changed)
    else
      let t63 := (← self.sub s2)
      match t63.opening with
      | some k =>
        let t67 := (← self.candidate s2 k)
        match id t67 with
        | some t67_0 =>
          let t := t67_0.1
          let j := t67_0.2
          unless t < S do throw s!"index out of bounds: the len is {S} but the index is {t}"
          let t76 := (← j -? 1).toUInt64
          unless t76 < G do throw s!"index out of bounds: the len is {G} but the index is {t76}"
          self := { self with subs := self.subs.set! t.toNat { self.subs[t.toNat]! with serving := self.subs[t.toNat]!.serving.set! t76.toNat false } }
          self := { self with resets := (← self.resets +? 1) }
          out := out.push (send_credit.Output.Reset t j : send_credit.Output)
          out := out.push (send_credit.Output.Wake t : send_credit.Output)
          changed := true
        | none =>
          pure ()
        return (← send_credit.Session.make_room_loop1 self out owed changed)
      | none =>
        return (← send_credit.Session.make_room_loop1 self out owed changed)
  | none =>
    return ((changed, self, out))
partial_fixpoint

def send_credit.Session.make_room {S : UInt64} {G : UInt64} (self : (send_credit.Session S G)) (out : (Array send_credit.Output)) : Rust.M (Bool × (send_credit.Session S G) × (Array send_credit.Output)) := do
  let mut owed := Vector.replicate S.toNat false
  let s : UInt64 := 0
  let r ← send_credit.Session.make_room_loop0 self out owed s
  owed := r
  let changed := false
  let r1 ← send_credit.Session.make_room_loop1 self out owed changed
  return r1

def send_credit.Session.is_head {S : UInt64} {G : UInt64} (self : (send_credit.Session S G)) (s : UInt64) : Rust.M Bool := do
  let t5 := (← self.sub s)
  if t5.place.isSome then
    return decide ((← self.ahead s) = 0)
  else
    return false

def send_credit.Session.head_loop {S : UInt64} {G : UInt64} (self : (send_credit.Session S G)) (s : UInt64) : Rust.M (Option UInt64) := do
  let mut s := s
  if s < S then
    let d := (← self.is_head s)
    if d then
      return (some s : (Option UInt64))
    else
      s := (← s +? 1)
      return (← send_credit.Session.head_loop self s)
  else
    return (none : (Option UInt64))
partial_fixpoint

def send_credit.Session.head {S : UInt64} {G : UInt64} (self : (send_credit.Session S G)) : Rust.M (Option UInt64) := do
  let s : UInt64 := 0
  let r ← send_credit.Session.head_loop self s
  return r

def send_credit.Session.leave {S : UInt64} {G : UInt64} (self : (send_credit.Session S G)) (s : UInt64) (out : (Array send_credit.Output)) : Rust.M ((send_credit.Session S G) × (Array send_credit.Output)) := do
  let mut self := self
  let mut out := out
  let was_head := (← self.is_head s)
  unless s < S do throw s!"index out of bounds: the len is {S} but the index is {s}"
  self := { self with subs := self.subs.set! s.toNat { self.subs[s.toNat]! with place := (none : (Option send_credit.Place)) } }
  let placed := (← self.placed)
  if self.resets > placed then
    self := { self with resets := placed }
  if was_head then
    let mut t18 : Bool := default
    match self.shape.mutant with
    | some mutant_0 =>
      match mutant_0 with
      | .NoLeaveWake =>
        t18 := true
      | _ =>
        t18 := false
    | none =>
      t18 := false
    if t18 then
      return (self, out)
    else
      let t21 := (← self.head)
      match t21 with
      | some head =>
        out := out.push (send_credit.Output.Wake head : send_credit.Output)
        return (self, out)
      | none =>
        return (self, out)
  else
    return (self, out)

def send_credit.Session.poll_open {S : UInt64} {G : UInt64} (self : (send_credit.Session S G)) (s : UInt64) (out : (Array send_credit.Output)) : Rust.M (Bool × (send_credit.Session S G) × (Array send_credit.Output)) := do
  let mut self := self
  let mut out := out
  let t6 := (← self.sub s)
  match t6.opening with
  | some opening_0 =>
    let k := opening_0
    let t13 := (← self.sub s)
    if t13.place.isNone then
      let d := decide ((← self.placed) = 0)
      if d then
        let d1 := (← self.credit)
        if d1 then
          let r3 ← self.«open» s k out
          self := r3.1
          out := r3.2
        else
          let r1 ← self.join s k
          self := r1
          unless s < S do throw s!"index out of bounds: the len is {S} but the index is {s}"
          self := { self with subs := self.subs.set! s.toNat { self.subs[s.toNat]! with asked := true } }
          let r2 ← self.make_room out
          self := r2.2.1
          out := r2.2.2
        return (true, self, out)
      else
        let mut changed := false
        let mut t41 := (← self.sub s)
        if t41.place.isNone then
          let mut r ← self.join s k
          self := r
          changed := true
        let mut d2 := (← self.is_head s)
        if d2 then
          let mut d3 := (← self.credit)
          if d3 then
            let mut r5 ← self.«open» s k out
            self := r5.1
            out := r5.2
            if self.resets > 0 then
              self := { self with resets := (← self.resets -? 1) }
            let mut r7 ← self.leave s out
            self := r7.1
            out := r7.2
            return (true, self, out)
          else
            let mut t75 := (← self.sub s)
            if ¬(t75.asked) then
              unless s < S do throw s!"index out of bounds: the len is {S} but the index is {s}"
              self := { self with subs := self.subs.set! s.toNat { self.subs[s.toNat]! with asked := true } }
              changed := true
            let mut r6 ← self.make_room out
            let mut t80 := r6.1
            self := r6.2.1
            out := r6.2.2
            if t80 then
              return (true, self, out)
            else
              return (changed, self, out)
        else
          let mut r4 ← self.make_room out
          let mut t53 := r4.1
          self := r4.2.1
          out := r4.2.2
          if t53 then
            return (true, self, out)
          else
            return (changed, self, out)
    else
      let mut changed := false
      let mut t41 := (← self.sub s)
      if t41.place.isNone then
        let mut r ← self.join s k
        self := r
        changed := true
      let mut d2 := (← self.is_head s)
      if d2 then
        let mut d3 := (← self.credit)
        if d3 then
          let mut r5 ← self.«open» s k out
          self := r5.1
          out := r5.2
          if self.resets > 0 then
            self := { self with resets := (← self.resets -? 1) }
          let mut r7 ← self.leave s out
          self := r7.1
          out := r7.2
          return (true, self, out)
        else
          let mut t75 := (← self.sub s)
          if ¬(t75.asked) then
            unless s < S do throw s!"index out of bounds: the len is {S} but the index is {s}"
            self := { self with subs := self.subs.set! s.toNat { self.subs[s.toNat]! with asked := true } }
            changed := true
          let mut r6 ← self.make_room out
          let mut t80 := r6.1
          self := r6.2.1
          out := r6.2.2
          if t80 then
            return (true, self, out)
          else
            return (changed, self, out)
      else
        let mut r4 ← self.make_room out
        let mut t53 := r4.1
        self := r4.2.1
        out := r4.2.2
        if t53 then
          return (true, self, out)
        else
          return (changed, self, out)
  | none =>
    return (false, self, out)

def send_credit.Session.stuck_loop {S : UInt64} {G : UInt64} (self : (send_credit.Session S G)) (s : UInt64) (w : UInt8) (held : Bool) (j : UInt64) : Rust.M Bool := do
  let mut held := held
  let mut j := j
  if j < G then
    let t17 := (← self.sub s)
    let t20 := j
    unless t20 < G do throw s!"index out of bounds: the len is {G} but the index is {t20}"
    if t17.serving[t20.toNat]! then
      let t29 := j
      let t24 := (← self.in_group s ((← t29 +? 1).toUInt8))
      let d := decide (t24 ≤ (← self.in_group s w))
      if d then
        return false
      else
        held := true
        j := (← j +? 1)
        return (← send_credit.Session.stuck_loop self s w held j)
    else
      j := (← j +? 1)
      return (← send_credit.Session.stuck_loop self s w held j)
  else
    return held
partial_fixpoint

def send_credit.Session.stuck {S : UInt64} {G : UInt64} (self : (send_credit.Session S G)) (s : UInt64) : Rust.M Bool := do
  let t5 := (← self.sub s)
  match t5.opening with
  | some opening_0 =>
    let w := opening_0
    let held := false
    let j : UInt64 := 0
    let r ← send_credit.Session.stuck_loop self s w held j
    return r
  | none =>
    return false

def send_credit.Session.admits {S : UInt64} {G : UInt64} (self : (send_credit.Session S G)) (s : UInt64) (k : UInt8) : Rust.M Bool := do
  let t6 := (← self.sub s)
  match t6.opening with
  | some opening_0 =>
    let w := opening_0
    let mut t10 : Bool := default
    match self.shape.mutant with
    | some mutant_0 =>
      match mutant_0 with
      | .NoGiveWay =>
        t10 := true
      | _ =>
        t10 := false
    | none =>
      t10 := false
    if t10 then
      return false
    else
      let d := (← self.stuck s)
      if d then
        unless s < S do throw s!"index out of bounds: the len is {S} but the index is {s}"
        if self.shape.ascending[s.toNat]! then
          return decide (k < w)
        else
          return true
      else
        return false
  | none =>
    return true

def send_credit.Session.cursor {S : UInt64} {G : UInt64} (self : (send_credit.Session S G)) (s : UInt64) (k : UInt8) (out : (Array send_credit.Output)) : Rust.M ((send_credit.Session S G) × (Array send_credit.Output)) := do
  let mut self := self
  let mut out := out
  let t6 := (← self.sub s)
  match t6.opening with
  | some opening_0 =>
    let w := opening_0
    let t13 := (← self.in_group s k)
    let d := decide (t13 < (← self.in_group s w))
    if d then
      out := out.push (send_credit.Output.Shed s k : send_credit.Output)
      return (self, out)
    else
      out := out.push (send_credit.Output.Shed s w : send_credit.Output)
      unless s < S do throw s!"index out of bounds: the len is {S} but the index is {s}"
      self := { self with subs := self.subs.set! s.toNat { self.subs[s.toNat]! with opening := (none : (Option UInt8)) } }
      let t37 := (← self.sub s)
      if t37.place.isSome then
        let r ← self.leave s out
        self := r.1
        out := r.2
      out := out.push (send_credit.Output.Take s k : send_credit.Output)
      unless s < S do throw s!"index out of bounds: the len is {S} but the index is {s}"
      self := { self with subs := self.subs.set! s.toNat { self.subs[s.toNat]! with opening := (some k : (Option UInt8)) } }
      return (self, out)
  | none =>
    out := out.push (send_credit.Output.Take s k : send_credit.Output)
    unless s < S do throw s!"index out of bounds: the len is {S} but the index is {s}"
    self := { self with subs := self.subs.set! s.toNat { self.subs[s.toNat]! with opening := (some k : (Option UInt8)) } }
    return (self, out)

def send_credit.Session.poll_loop {S : UInt64} {G : UInt64} (self : (send_credit.Session S G)) (s : UInt64) (cached : UInt16) (pick : (Option UInt8)) (out : (Array send_credit.Output)) (next : (send_credit.Session S G)) (scratch : (Array send_credit.Output)) (changed : Bool) (ready : Bool) (j : UInt64) : Rust.M ((Except Disabled Unit) × (send_credit.Session S G) × (Array send_credit.Output)) := do
  let mut self := self
  let mut out := out
  let mut next := next
  let mut scratch := scratch
  let mut ready := ready
  let mut j := j
  if j < G then
    let t33 := j
    let t34 := decide (t33 < 16)
    unless t34 do throw "attempt to shift left with overflow"
    if (cached &&& (1 <<< t33.toUInt16)) ≠ 0 then
      let t40 := j
      let d1 := (← next.admits s ((← t40 +? 1).toUInt8))
      if d1 then
        ready := true
    j := (← j +? 1)
    return (← send_credit.Session.poll_loop self s cached pick out next scratch changed ready j)
  else if ready then
    match id pick with
    | some pick_0 =>
      let k := pick_0
      if k < 1 then
        return (((Except.error ({} : Disabled) : (Except Disabled Unit)), self, out))
      else if (k.toUInt64) > G then
        return (((Except.error ({} : Disabled) : (Except Disabled Unit)), self, out))
      else
        let t67 := (← k -? 1)
        let t70 := decide (t67 < 16)
        unless t70 do throw "attempt to shift left with overflow"
        if (cached &&& (1 <<< t67.toUInt16)) = 0 then
          return (((Except.error ({} : Disabled) : (Except Disabled Unit)), self, out))
        else
          let d2 := (← next.admits s k)
          if d2 then
            let r1 ← next.cursor s k scratch
            next := r1.1
            scratch := r1.2
            self := next
            let mut t89 := scratch
            let mut t87 := t89
            out := out ++ t87
            return (((Except.ok () : (Except Disabled Unit)), self, out))
          else
            return (((Except.error ({} : Disabled) : (Except Disabled Unit)), self, out))
    | none =>
      return (((Except.error ({} : Disabled) : (Except Disabled Unit)), self, out))
  else if changed then
    self := next
    let mut t89 := scratch
    let mut t87 := t89
    out := out ++ t87
    return (((Except.ok () : (Except Disabled Unit)), self, out))
  else
    scratch := scratch.push (send_credit.Output.Park s : send_credit.Output)
    self := next
    let mut t89 := scratch
    let mut t87 := t89
    out := out ++ t87
    return (((Except.ok () : (Except Disabled Unit)), self, out))
partial_fixpoint

def send_credit.Session.poll {S : UInt64} {G : UInt64} (self : (send_credit.Session S G)) (s : UInt64) (cached : UInt16) (pick : (Option UInt8)) (out : (Array send_credit.Output)) : Rust.M ((Except Disabled Unit) × (send_credit.Session S G) × (Array send_credit.Output)) := do
  let d := decide ((cached &&& (← self.taken s)) ≠ 0)
  if d then
    return ((Except.error ({} : Disabled) : (Except Disabled Unit)), self, out)
  else
    let mut next := self
    let mut scratch := (#[] : (Array send_credit.Output))
    let r ← next.poll_open s scratch
    let changed := r.1
    next := r.2.1
    scratch := r.2.2
    let ready := false
    let j : UInt64 := 0
    let r2 ← send_credit.Session.poll_loop self s cached pick out next scratch changed ready j
    return r2

def send_credit.Session.finished {S : UInt64} {G : UInt64} (self : (send_credit.Session S G)) (s : UInt64) (k : UInt8) (out : (Array send_credit.Output)) : Rust.M ((Except Disabled Unit) × (send_credit.Session S G) × (Array send_credit.Output)) := do
  let mut self := self
  let mut out := out
  if k < 1 then
    return ((Except.error ({} : Disabled) : (Except Disabled Unit)), self, out)
  else if (k.toUInt64) > G then
    return ((Except.error ({} : Disabled) : (Except Disabled Unit)), self, out)
  else
    let t12 := (← self.sub s)
    let t15 := (← k -? 1).toUInt64
    unless t15 < G do throw s!"index out of bounds: the len is {G} but the index is {t15}"
    if t12.serving[t15.toNat]! then
      unless s < S do throw s!"index out of bounds: the len is {S} but the index is {s}"
      let t24 := (← k -? 1).toUInt64
      unless t24 < G do throw s!"index out of bounds: the len is {G} but the index is {t24}"
      self := { self with subs := self.subs.set! s.toNat { self.subs[s.toNat]! with serving := self.subs[s.toNat]!.serving.set! t24.toNat false } }
      out := out.push (send_credit.Output.Wake s : send_credit.Output)
      return ((Except.ok () : (Except Disabled Unit)), self, out)
    else
      return ((Except.error ({} : Disabled) : (Except Disabled Unit)), self, out)

def send_credit.Session.step {S : UInt64} {G : UInt64} (self : (send_credit.Session S G)) (event : send_credit.Event) (out : (Array send_credit.Output)) : Rust.M ((Except Disabled Unit) × (send_credit.Session S G) × (Array send_credit.Output)) := do
  let mut self := self
  let mut out := out
  match event with
  | .Poll event_0 event_1 event_2 =>
    let sub1 := event_0
    if sub1 < S then
      let sub := event_0
      let cached := event_1
      let pick := event_2
      let r1 ← self.poll sub cached pick out
      let mut ret := r1.1
      self := r1.2.1
      out := r1.2.2
      return (ret, self, out)
    else
      return ((Except.error ({} : Disabled) : (Except Disabled Unit)), self, out)
  | .Finished event_01 event_11 =>
    let sub3 := event_01
    if sub3 < S then
      let sub2 := event_01
      let group := event_11
      let r2 ← self.finished sub2 group out
      let mut ret := r2.1
      self := r2.2.1
      out := r2.2.2
      return (ret, self, out)
    else
      return ((Except.error ({} : Disabled) : (Except Disabled Unit)), self, out)
  | .Freed =>
    let r ← self.freed' out
    let mut ret := r.1
    self := r.2.1
    out := r.2.2
    return (ret, self, out)

def send_credit.Session.within_credit {S : UInt64} {G : UInt64} (self : (send_credit.Session S G)) : Rust.M Bool := do
  let t5 := self.shape.max_streams.toUInt16
  let t7 := self.freed.toUInt16
  return decide ((self.opened.toUInt16) ≤ (← t5 +? t7))

def send_credit.Session.well_formed_loop {S : UInt64} {G : UInt64} (self : (send_credit.Session S G)) (s : UInt64) : Rust.M Bool := do
  let mut s := s
  if s < S then
    let sub := (← self.sub s)
    match sub.opening with
    | none =>
      if sub.place.isSome then
        return false
      else
        s := (← s +? 1)
        return (← send_credit.Session.well_formed_loop self s)
    | some opening_0 =>
      let k1 := opening_0
      if k1 < 1 then
        return false
      else if (k1.toUInt64) > G then
        return false
      else
        let t26 := k1
        let t24 := (← t26 -? 1).toUInt64
        unless t24 < G do throw s!"index out of bounds: the len is {G} but the index is {t24}"
        if sub.serving[t24.toNat]! then
          return false
        else
          s := (← s +? 1)
          return (← send_credit.Session.well_formed_loop self s)
  else
    let d := decide (self.resets ≤ (← self.placed))
    if d then
      return decide (self.freed ≤ self.opened)
    else
      return false
partial_fixpoint

def send_credit.Session.well_formed {S : UInt64} {G : UInt64} (self : (send_credit.Session S G)) : Rust.M Bool := do
  let s : UInt64 := 0
  let r ← send_credit.Session.well_formed_loop self s
  return r

def send_credit.Session.StateMachine.step {S : UInt64} {G : UInt64} (self : (send_credit.Session S G)) (event : send_credit.Event) (out : (Array send_credit.Output)) : Rust.M ((Except Disabled Unit) × (send_credit.Session S G) × (Array send_credit.Output)) := do
  let mut self := self
  let mut out := out
  let r ← self.step event out
  let ret := r.1
  self := r.2.1
  out := r.2.2
  return (ret, self, out)

def warm_settle.better (a : warm_settle.Choice) (b : warm_settle.Choice) : Bool := Id.run do
  if a.cost < b.cost then
    return true
  else if a.cost = b.cost then
    if a.copy then
      if b.copy then
        if a.copy = b.copy then
          return decide (a.via > b.via)
        else
          return false
      else
        return true
    else if a.copy = b.copy then
      return decide (a.via > b.via)
    else
      return false
  else
    return false

def warm_settle.best_loop (choices : (Array warm_settle.Choice)) (best : (Option warm_settle.Choice)) (i : UInt64) : Rust.M (Option warm_settle.Choice) := do
  let mut best := best
  let mut i := i
  if i < (UInt64.ofNat choices.size) then
    let t11 := i
    let t12 := UInt64.ofNat choices.size
    unless t11 < t12 do throw s!"index out of bounds: the len is {t12} but the index is {t11}"
    let choice := choices[t11.toNat]!
    let mut t14 : (Option warm_settle.Choice) := default
    match id best with
    | some best_0 =>
      let b1 := best_0
      let t21 := choice
      let t23 := b1
      if warm_settle.better t21 t23 then
        t14 := (some choice : (Option warm_settle.Choice))
      else
        let b := best_0
        t14 := (some b : (Option warm_settle.Choice))
    | none =>
      t14 := (some choice : (Option warm_settle.Choice))
    best := t14
    i := (← i +? 1)
    return (← warm_settle.best_loop choices best i)
  else
    return (best)
partial_fixpoint

def warm_settle.best (choices : (Array warm_settle.Choice)) : Rust.M (Option warm_settle.Choice) := do
  let best := (none : (Option warm_settle.Choice))
  let i : UInt64 := 0
  let r ← warm_settle.best_loop choices best i
  return r

def warm_settle.Relay.link {N : UInt64} (self : (warm_settle.Relay N)) (r : UInt8) : Rust.M (Option warm_settle.Link) := do
  let t3 := (← r -? 1).toUInt64
  unless t3 < N do throw s!"index out of bounds: the len is {N} but the index is {t3}"
  return self.links[t3.toNat]!

def warm_settle.Relay.is_open {N : UInt64} (self : (warm_settle.Relay N)) (r : UInt8) : Rust.M Bool := do
  let t3 := (← self.link r)
  match t3 with
  | some t3_0 =>
    if t3_0.«open» then
      return true
    else
      return false
  | none =>
    return false

def warm_settle.Relay.heard' {N : UInt64} (self : (warm_settle.Relay N)) (r : UInt8) : Rust.M warm_settle.Heard := do
  let t3 := (← r -? 1).toUInt64
  unless t3 < N do throw s!"index out of bounds: the len is {N} but the index is {t3}"
  return self.heard[t3.toNat]!

def warm_settle.Relay.down {N : UInt64} (self : (warm_settle.Relay N)) (c : UInt8) : Rust.M Bool := do
  let d := (← self.is_open c)
  if d then
    let t7 := (← self.heard' c)
    return t7.pull.isSome
  else
    return false

def warm_settle.Relay.demand_loop {N : UInt64} (self : (warm_settle.Relay N)) (c : UInt8) : Rust.M Bool := do
  let mut c := c
  if (c.toUInt64) ≤ N then
    let d := (← self.down c)
    if d then
      return true
    else
      c := (← c +? 1)
      return (← warm_settle.Relay.demand_loop self c)
  else
    return false
partial_fixpoint

def warm_settle.Relay.demand {N : UInt64} (self : (warm_settle.Relay N)) : Rust.M Bool := do
  if self.reader.isSome then
    return true
  else
    let c : UInt8 := 1
    let r ← warm_settle.Relay.demand_loop self c
    return r

def warm_settle.bit (r : UInt8) : Rust.M UInt16 := do
  let t2 := (← r -? 1)
  let t5 := decide (t2 < 16)
  unless t5 do throw "attempt to shift left with overflow"
  return 1 <<< t2.toUInt16

def warm_settle.Relay.pull {N : UInt64} (self : (warm_settle.Relay N)) (s : UInt8) : Rust.M warm_settle.Pull := do
  let t3 := (← self.heard' s)
  match t3.answer with
  | .Live sig1 =>
    let d := decide ((sig1.path &&& (← warm_settle.bit self.id)) = 0)
    if d then
      return (warm_settle.Pull.Live : warm_settle.Pull)
    else
      return (warm_settle.Pull.Looped : warm_settle.Pull)
  | .Lost =>
    return (warm_settle.Pull.Lost : warm_settle.Pull)
  | _ =>
    let d1 := decide ((self.answered &&& (← warm_settle.bit s)) ≠ 0)
    if d1 then
      return (warm_settle.Pull.Lost : warm_settle.Pull)
    else
      return (warm_settle.Pull.Pending : warm_settle.Pull)

def warm_settle.Relay.mine_loop {N : UInt64} (self : (warm_settle.Relay N)) (rank : UInt32) (d : UInt8) : Rust.M UInt32 := do
  let mut rank := rank
  let mut d := d
  if (d.toUInt64) ≤ N then
    let d1 := (← self.down d)
    if d1 then
      let t15 := (← self.heard' d)
      match t15.pull with
      | some pull_0 =>
        let pull := pull_0
        if pull < rank then
          rank := pull
      | none =>
        pure ()
    d := (← d +? 1)
    return (← warm_settle.Relay.mine_loop self rank d)
  else
    return (rank)
partial_fixpoint

def warm_settle.Relay.mine {N : UInt64} (self : (warm_settle.Relay N)) : Rust.M UInt32 := do
  let mut rank : UInt32 := default
  match self.reader with
  | none =>
    rank := (4294967295 : UInt32)
  | some own =>
    rank := own
  let d : UInt8 := 1
  let r ← warm_settle.Relay.mine_loop self rank d
  return r

def warm_settle.is (x : (Option UInt8)) (v : UInt8) : Bool := Id.run do
  match x with
  | none =>
    return false
  | some y =>
    return decide (y = v)

def warm_settle.Relay.options_loop {N : UInt64} (self : (warm_settle.Relay N)) (u : (Option UInt8)) (mine : UInt32) (choices : (Array warm_settle.Choice)) (s : UInt8) : Rust.M (Array warm_settle.Choice) := do
  let mut choices := choices
  let mut s := s
  if (s.toUInt64) ≤ N then
    let heard := (← self.heard' s)
    let mut sig : (Option warm_settle.Sig) := default
    match heard.advert with
    | none =>
      if warm_settle.is u s then
        let t80 := (warm_settle.Pull.Live : warm_settle.Pull)
        let d2 := decide ((← self.pull s) = t80)
        if d2 then
          match heard.answer with
          | .Live sig3 =>
            sig := (some sig3 : (Option warm_settle.Sig))
          | _ =>
            sig := (none : (Option warm_settle.Sig))
        else
          sig := (none : (Option warm_settle.Sig))
      else
        sig := (none : (Option warm_settle.Sig))
    | some advert_0 =>
      let d1 := (← self.is_open s)
      if d1 then
        let sig1 := advert_0
        sig := (some sig1 : (Option warm_settle.Sig))
      else
        sig := (none : (Option warm_settle.Sig))
    let t50 := (sig, (← self.link s))
    match t50.1 with
    | some t50_0 =>
      match t50.2 with
      | some t50_01 =>
        let sig4 := t50_0
        let link := t50_01
        let d3 := (← self.down s)
        if ¬(d3) then
          let d4 := decide ((sig4.path &&& (← warm_settle.bit self.id)) = 0)
          if d4 then
            if sig4.rank < mine then
              choices := choices.push ({ via := s, cost := link.cold, copy := true, rank := sig4.rank } : warm_settle.Choice)
      | none =>
        pure ()
    | none =>
      pure ()
    s := (← s +? 1)
    return (← warm_settle.Relay.options_loop self u mine choices s)
  else
    return (choices)
partial_fixpoint

def warm_settle.Relay.options {N : UInt64} (self : (warm_settle.Relay N)) (u : (Option UInt8)) : Rust.M (Array warm_settle.Choice) := do
  let mine := (← self.mine)
  let mut choices := (#[] : (Array warm_settle.Choice))
  match self.route with
  | some route_0 =>
    let hop := route_0.1
    let cost := route_0.2
    let d := (← self.down hop)
    if ¬(d) then
      choices := choices.push ({ via := hop, cost := cost, copy := false, rank := (4294967295 : UInt32) } : warm_settle.Choice)
  | none =>
    pure ()
  let s : UInt8 := 1
  let r ← warm_settle.Relay.options_loop self u mine choices s
  return r

def warm_settle.toward (u : UInt8) (x : (Option UInt8)) (t : (Option UInt8)) : ((Option UInt8) × (Option UInt8)) := Id.run do
  match t with
  | some t_0 =>
    let t2 := t_0
    if t2 = u then
      return ((some u : (Option UInt8)), (none : (Option UInt8)))
    else
      let t6 := t_0
      if warm_settle.is x t6 then
        return ((some u : (Option UInt8)), x)
      else
        return ((some u : (Option UInt8)), t)
  | none =>
    return ((some u : (Option UInt8)), t)

def warm_settle.Relay.decide' {N : UInt64} (self : (warm_settle.Relay N)) (u : (Option UInt8)) (x : (Option UInt8)) : Rust.M ((Option UInt8) × (Option UInt8)) := do
  let choices := (← self.options u)
  let t12 := choices
  let t10 := t12
  let t8 := (← warm_settle.best t10)
  let mut best : (Option UInt8) := default
  match t8 with
  | none =>
    best := (none : (Option UInt8))
  | some choice =>
    best := (some choice.via : (Option UInt8))
  match u with
  | some u_0 =>
    let u1 := u_0
    match (← self.pull u1) with
    | .Live =>
      if best.isSome then
        return warm_settle.toward u1 x best
      else
        return (best, (none : (Option UInt8)))
    | .Pending =>
      if warm_settle.is best u1 then
        return ((some u1 : (Option UInt8)), x)
      else
        return (best, (none : (Option UInt8)))
    | _ =>
      return (best, (none : (Option UInt8)))
  | none =>
    return (best, (none : (Option UInt8)))

def warm_settle.Relay.next_pulls {N : UInt64} (self : (warm_settle.Relay N)) : Rust.M ((Option UInt8) × (Option UInt8)) := do
  let d := (← self.demand)
  if d then
    let mut t10 : ((Option UInt8) × (Option UInt8)) := default
    match self.next with
    | none =>
      t10 := (self.up, (none : (Option UInt8)))
    | some next_0 =>
      let x1 := next_0
      match (← self.pull x1) with
      | .Live =>
        t10 := ((some x1 : (Option UInt8)), (none : (Option UInt8)))
      | .Looped =>
        t10 := (self.up, (none : (Option UInt8)))
      | .Lost =>
        t10 := (self.up, (none : (Option UInt8)))
      | .Pending =>
        t10 := (self.up, (some x1 : (Option UInt8)))
    let u := t10.1
    let x := t10.2
    return (← self.decide' u x)
  else
    return ((none : (Option UInt8)), (none : (Option UInt8)))

def warm_settle.Relay.StateMachine.step_loop {N : UInt64} (self : (warm_settle.Relay N)) (arg3 : (Array Unit)) (up : (Option UInt8)) (next : (Option UInt8)) (answered : UInt16) (s : UInt8) : Rust.M ((Except Disabled Unit) × (warm_settle.Relay N) × (Array Unit)) := do
  let mut self := self
  let mut answered := answered
  let mut s := s
  if (s.toUInt64) ≤ N then
    let d2 := decide ((self.answered &&& (← warm_settle.bit s)) ≠ 0)
    if d2 then
      if warm_settle.is up s then
        let mut t94 := (warm_settle.Pull.Lost : warm_settle.Pull)
        let mut d3 := decide ((← self.pull s) ≠ t94)
        if d3 then
          answered := answered ||| (← warm_settle.bit s)
      else if warm_settle.is next s then
        let mut t94 := (warm_settle.Pull.Lost : warm_settle.Pull)
        let mut d3 := decide ((← self.pull s) ≠ t94)
        if d3 then
          answered := answered ||| (← warm_settle.bit s)
    s := (← s +? 1)
    return (← warm_settle.Relay.StateMachine.step_loop self arg3 up next answered s)
  else
    self := { self with up := up }
    self := { self with next := next }
    self := { self with answered := answered }
    return (((Except.ok () : (Except Disabled Unit)), self, arg3))
partial_fixpoint

def warm_settle.Relay.StateMachine.step {N : UInt64} (self : (warm_settle.Relay N)) (event : warm_settle.Event) (arg3 : (Array Unit)) : Rust.M ((Except Disabled Unit) × (warm_settle.Relay N) × (Array Unit)) := do
  let mut self := self
  match event with
  | .Hear event_0 event_1 =>
    let «from» := event_0
    let heard := event_1
    if «from» < 1 then
      return ((Except.error ({} : Disabled) : (Except Disabled Unit)), self, arg3)
    else if («from».toUInt64) > N then
      return ((Except.error ({} : Disabled) : (Except Disabled Unit)), self, arg3)
    else
      let d := (← self.is_open «from»)
      if d then
        let d1 := decide ((← self.heard' «from») = heard)
        if d1 then
          return ((Except.error ({} : Disabled) : (Except Disabled Unit)), self, arg3)
        else
          let t25 := (← «from» -? 1).toUInt64
          unless t25 < N do throw s!"index out of bounds: the len is {N} but the index is {t25}"
          self := { self with heard := self.heard.set! t25.toNat heard }
          if warm_settle.is self.up «from» then
            let mut t37 : Bool := default
            match heard.answer with
            | .Live answer_0 =>
              t37 := true
            | _ =>
              t37 := false
            if t37 then
              self := { self with answered := self.answered ||| (← warm_settle.bit «from») }
          else if warm_settle.is self.next «from» then
            let mut t37 : Bool := default
            match heard.answer with
            | .Live answer_0 =>
              t37 := true
            | _ =>
              t37 := false
            if t37 then
              self := { self with answered := self.answered ||| (← warm_settle.bit «from») }
          return ((Except.ok () : (Except Disabled Unit)), self, arg3)
      else
        return ((Except.error ({} : Disabled) : (Except Disabled Unit)), self, arg3)
  | .React =>
    let t44 := (← self.next_pulls)
    let up := t44.1
    let next := t44.2
    if (up, next) = (self.up, self.next) then
      return ((Except.error ({} : Disabled) : (Except Disabled Unit)), self, arg3)
    else
      let answered : UInt16 := 0
      let s : UInt8 := 1
      let r ← warm_settle.Relay.StateMachine.step_loop self arg3 up next answered s
      return r

end kio_pure_machine
