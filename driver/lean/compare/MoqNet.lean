-- Translated from the `moq_net` crate by rust2-lean, with overflow checks on.
import Rust
import MoqPattern

set_option linter.unusedVariables false
-- Rust's wide tuples need deeper instance searches than Lean's default.
set_option synthInstance.maxSize 1024

namespace moq_net

/-- `route::relay::NodeId` -/
structure route.relay.NodeId where
  relay : UInt64
  marked : Bool
deriving Repr, DecidableEq, Inhabited, Ord

/-- `route::relay::Hop` -/
structure route.relay.Hop where
  node : route.relay.NodeId
  session : UInt64
deriving Repr, DecidableEq, Inhabited, Ord

/-- `route::relay::Offer` -/
structure route.relay.Offer where
  path : (Array route.relay.Hop)
  draining : Bool
  priority : Int8
  cost : UInt64
  marked : Bool
deriving Repr, DecidableEq, Inhabited

/-- `route::relay::Here` -/
structure route.relay.Here where
  closed : (Rust.BTreeMap (UInt64 × UInt64) UInt64)
  dead : (Rust.BTreeMap route.relay.Hop route.relay.NodeId)
  draining : (Rust.BTreeSet route.relay.NodeId)
deriving Repr, DecidableEq, Inhabited

/-- `route::relay::Role` -/
inductive route.relay.Role where
  | Routing (x0 : route.relay.NodeId)
  | Scoped (x0 : (Array String))
  | Endpoint
deriving Repr, DecidableEq, Inhabited

/-- `route::relay::Port` -/
structure route.relay.Port where
  cost : UInt64
  priority : Int8
  upstream : Bool
  role : route.relay.Role
deriving Repr, DecidableEq, Inhabited

/-- `route::relay::Best` -/
structure route.relay.Best (N : Type) where
  via : N
  offer : route.relay.Offer
deriving Repr, DecidableEq, Inhabited

/-- `route::relay::Claim` -/
structure route.relay.Claim (A : Type) where
  item : A
  cost : UInt64
  priority : Int8
  draining : Bool
  unavailable : Bool
  chain : (Array UInt64)
  marked : Bool
deriving Repr, DecidableEq, Inhabited

/-- `route::Item` -/
class route.Item (Self : Type) where
  lowered : Self → UInt64 → Rust.M Self
  above : Self → Rust.M UInt64
  epoch : Self → Rust.M UInt64

/-- `path::Path::as_str`, modeled on the string a `Path` holds -/
def path.Path.as_str (self : String) : String :=
  self

def path.Path.prefix_len (self : String) («prefix» : String) : Rust.M (Option UInt64) := do
  let prefix1 := path.Path.as_str «prefix»
  if prefix1.isEmpty then
    return (some (0 : UInt64) : (Option UInt64))
  else
    let s := path.Path.as_str self
    if prefix1.isPrefixOf s then
      if (Rust.strLen s) = (Rust.strLen prefix1) then
        return (some (Rust.strLen s) : (Option UInt64))
      else
        let t29 := Rust.asBytes s
        let t42 := (some (47 : UInt8) : (Option UInt8))
        let t39 := Rust.strLen prefix1
        return (if decide ((t29[(Rust.strLen prefix1).toNat]?) = t42) then some · else none) (← t39 +? 1)
    else
      return (none : (Option UInt64))

def path.Path.has_prefix (self : String) («prefix» : String) : Rust.M Bool := do
  return (← path.Path.prefix_len self «prefix»).isSome

def path.Path.parts (self : String) : _root_.moq_pattern.parts.Parts := Id.run do
  let t3 := path.Path.as_str self
  return _root_.moq_pattern.parts.parts t3

def path.Path.head_len_loop (parts : _root_.moq_pattern.parts.Parts) («end» : UInt64) (iter : (UInt64 × UInt64)) : Rust.M (Option UInt64) := do
  let mut parts := parts
  let mut «end» := «end»
  let mut iter := iter
  let r := Rust.rangeNext iter
  iter := r.1
  let t13 := r.2
  match id t13 with
  | none =>
    return (some «end» : (Option UInt64))
  | some t13_0 =>
    let i := t13_0
    let r1 ← _root_.moq_pattern.parts.Parts.Iterator.next parts
    let t24 := r1.1
    parts := r1.2
    let t23 := Rust.branch t24
    match id t23 with
    | .Continue val =>
      let t20 := Rust.strLen val
      let t31 := if decide (i > 0) then (1 : UInt64) else 0
      let t19 := (← t20 +? t31)
      «end» := (← «end» +? t19)
      return (← path.Path.head_len_loop parts «end» iter)
    | .Break t23_01 =>
      return (none : (Option UInt64))
partial_fixpoint

def path.Path.head_len (self : String) (n : UInt64) : Rust.M (Option UInt64) := do
  let parts := path.Path.parts self
  let «end» : UInt64 := 0
  let iter := ((0 : UInt64), n)
  let r2 ← path.Path.head_len_loop parts «end» iter
  return r2

/-- `path::Path::head`, modeled on the string a `Path` holds -/
def path.Path.head (self : String) (n : UInt64) : Rust.M (Option String) := do
  match (← path.Path.head_len self n) with
  | none => pure none
  | some e => pure (some (← Rust.strSliceTo self e))

def path.Path.is_empty (self : String) : Bool := Id.run do
  let t3 := path.Path.as_str self
  return t3.isEmpty

def route.relay.Offer.rank (self : route.relay.Offer) : (Bool × (Rust.Reverse Int8) × UInt64) :=
  (self.draining, ({ x0 := self.priority } : (Rust.Reverse Int8)), self.cost)

def route.relay.Offer.own (me : route.relay.NodeId) (origin : route.relay.NodeId) (upstream : Bool) : (Option route.relay.Offer) := Id.run do
  let hop := ({ node := me, session := (0 : UInt64) } : route.relay.Hop)
  let mut path1 : (Array route.relay.Hop) := default
  if origin.marked then
    path1 := #v[({ node := origin, session := (0 : UInt64) } : route.relay.Hop), hop].toArray
  else
    path1 := #v[hop].toArray
  let own := ({ path := path1, draining := false, priority := (127 : Int8), cost := (0 : UInt64), marked := origin.marked } : route.relay.Offer)
  let mut t19 : Bool := default
  if own.marked then
    t19 := upstream
  else
    t19 := false
  return if !t19 then some own else none

def route.relay.names.closure0 (arg1 : UInt64) (hop : route.relay.Hop) : Bool := Id.run do
  let t5 := arg1
  return decide (hop.node.relay = t5)

def route.relay.names (path1 : (Array route.relay.Hop)) (relay : UInt64) : Bool := Id.run do
  let t4 := path1.toList
  return t4.any (route.relay.names.closure0 relay)

def route.relay.Offer.onward (self : route.relay.Offer) (me : route.relay.NodeId) (session : UInt64) (to : route.relay.NodeId) (upstream : Bool) : (Option route.relay.Offer) := Id.run do
  let t11 := self.path
  let t9 := t11
  if route.relay.names t9 to.relay then
    return (none : (Option route.relay.Offer))
  else if self.marked then
    if upstream then
      return (none : (Option route.relay.Offer))
    else
      let mut t21 := self.path
      let mut path1 := ((t21.toList) ++ (#v[({ node := me, session := session } : route.relay.Hop)].toList)).toArray
      let mut t29 := self
      return (some ({ path := path1, draining := t29.draining, priority := t29.priority, cost := t29.cost, marked := t29.marked } : route.relay.Offer) : (Option route.relay.Offer))
  else
    let mut t21 := self.path
    let mut path1 := ((t21.toList) ++ (#v[({ node := me, session := session } : route.relay.Hop)].toList)).toArray
    let mut t29 := self
    return (some ({ path := path1, draining := t29.draining, priority := t29.priority, cost := t29.cost, marked := t29.marked } : route.relay.Offer) : (Option route.relay.Offer))

def route.relay.floor_of (hop : route.relay.Hop) (far : route.relay.NodeId) : ((UInt64 × UInt64) × UInt64) :=
  ((hop.node.relay, far.relay), hop.session)

def route.relay.pairs.closure0 (arg1 : Unit) (pair : (Array route.relay.Hop)) : Rust.M ((UInt64 × UInt64) × UInt64) := do
  let t4 : UInt64 := 1
  let t5 := UInt64.ofNat pair.size
  unless t4 < t5 do throw s!"index out of bounds: the len is {t5} but the index is {t4}"
  let t8 : UInt64 := 0
  let t9 := UInt64.ofNat pair.size
  unless t8 < t9 do throw s!"index out of bounds: the len is {t9} but the index is {t8}"
  return route.relay.floor_of pair[t4.toNat]! pair[t8.toNat]!.node

def route.relay.pairs (path1 : (Array route.relay.Hop)) : Rust.M (List (Rust.Lazy ((UInt64 × UInt64) × UInt64))) := do
  return Rust.lazyMap (route.relay.pairs.closure0 ()) (Rust.lazy (← Rust.windows path1 (2 : UInt64)))

def route.relay.Here.arrive.closure0.closure0 (arg1 : UInt64) (highest : UInt64) : Bool := Id.run do
  let t5 := arg1
  return decide (t5 ≤ highest)

def route.relay.Here.arrive.closure0 (arg1 : (Rust.BTreeMap (UInt64 × UInt64) UInt64)) (arg2 : ((UInt64 × UInt64) × UInt64)) : Bool := Id.run do
  let pair := arg2.1
  let «at» := arg2.2
  let t11 := arg1
  let t8 := pair
  return Option.any (route.relay.Here.arrive.closure0.closure0 «at») (t11.get t8)

def route.relay.Here.arrive.closure1 (arg1 : (Rust.BTreeMap route.relay.Hop route.relay.NodeId)) (hop : route.relay.Hop) : Bool := Id.run do
  let t5 := arg1
  return t5.containsKey hop

def route.relay.Here.arrive.closure2 (arg1 : (Rust.BTreeSet route.relay.NodeId)) (hop : route.relay.Hop) : Bool := Id.run do
  let t6 := arg1
  let t5 := hop.node
  return t6.contains t5

def route.relay.Here.arrive (self : route.relay.Here) (port : route.relay.Port) (draining : Bool) (offer : route.relay.Offer) : Rust.M (Option route.relay.Offer) := do
  let t11 := offer.path
  let t9 := t11
  let t7 := (← route.relay.pairs t9)
  let t58 := self.closed
  let floored := (← Rust.lazyAny (pure ∘ (route.relay.Here.arrive.closure0 t58)) t7)
  let t18 := offer.path
  if (UInt64.ofNat t18.size) ≥ 32 then
    return (none : (Option route.relay.Offer))
  else if floored then
    return (none : (Option route.relay.Offer))
  else
    let t25 := offer.path
    let t23 := t25.toList
    let t59 := self.dead
    if t23.any (route.relay.Here.arrive.closure1 t59) then
      return (none : (Option route.relay.Offer))
    else
      let t31 := offer.path
      let mut t33 : Bool := default
      if offer.draining then
        t33 := true
      else if draining then
        t33 := true
      else
        let t39 := offer.path
        let t37 := t39.toList
        let t60 := self.draining
        t33 := t37.any (route.relay.Here.arrive.closure2 t60)
      let t47 := Rust.branch (Rust.checkedAdd offer.cost port.cost)
      let t43 := min offer.priority port.priority
      match t47 with
      | .Continue t47_0 =>
        let val := t47_0
        let mut t56 : Bool := default
        if offer.marked then
          t56 := true
        else
          t56 := port.upstream
        return (some ({ path := t31, draining := t33, priority := t43, cost := val, marked := t56 } : route.relay.Offer) : (Option route.relay.Offer))
      | .Break t47_01 =>
        return (none : (Option route.relay.Offer))

def route.relay.order {N : Type} [Inhabited N] [DecidableEq N] [Ord N] (best : (route.relay.Best N)) (current : (Option N)) : Rust.M (Bool × (Rust.Reverse Int8) × UInt64 × Bool × Bool × UInt64 × route.relay.NodeId × N) := do
  let t6 := best.offer.rank
  let draining := t6.1
  let priority := t6.2.1
  let cost := t6.2.2
  let t19 := best.offer.path
  let t25 := best.offer.path
  let t28 := "a heard route names its sender"
  let t22 := (← Rust.expect (t25.back?) t28)
  return (draining, priority, cost, best.offer.marked, decide (current ≠ (some best.via : (Option N))), UInt64.ofNat t19.size, t22.node, best.via)

def route.relay.Claim.rank {A : Type} [Inhabited A] [DecidableEq A] [route.Item A] (self : (route.relay.Claim A)) : (Bool × (Rust.Reverse Int8) × UInt64) :=
  (self.draining, ({ x0 := self.priority } : (Rust.Reverse Int8)), self.cost)

def route.relay.Claim.lowered {A : Type} [Inhabited A] [DecidableEq A] [route.Item A] (self : (route.relay.Claim A)) («by» : UInt64) : Rust.M (route.relay.Claim A) := do
  let t6 := self
  return ({ item := (← route.Item.lowered self.item «by»), cost := t6.cost, priority := t6.priority, draining := t6.draining, unavailable := t6.unavailable, chain := t6.chain, marked := t6.marked } : (route.relay.Claim A))

def route.relay.Claim.joined {A : Type} [Inhabited A] [DecidableEq A] [route.Item A] (self : (route.relay.Claim A)) (route1 : route.relay.Offer) : ((Bool × (Rust.Reverse Int8) × UInt64) × Bool) := Id.run do
  let t6 := route1.rank
  let draining := t6.1
  let priority := t6.2.1.x0
  let cost := t6.2.2
  let mut t9 : Bool := default
  if draining then
    t9 := true
  else
    t9 := self.draining
  let rank := (t9, ({ x0 := min priority self.priority } : (Rust.Reverse Int8)), min (Rust.saturatingAdd cost self.cost) 4611686018427387903)
  let mut t20 : Bool := default
  if self.marked then
    t20 := true
  else
    t20 := route1.marked
  return (rank, t20)

def route.relay.Claim.over {A : Type} [Inhabited A] [DecidableEq A] [route.Item A] (self : (route.relay.Claim A)) (route1 : route.relay.Offer) : (route.relay.Claim A) := Id.run do
  let t7 := self.joined route1
  let draining := t7.1.1
  let priority := t7.1.2.1.x0
  let cost := t7.1.2.2
  let marked := t7.2
  let t14 := self
  return ({ item := t14.item, cost := cost, priority := priority, draining := draining, unavailable := t14.unavailable, chain := t14.chain, marked := marked } : (route.relay.Claim A))

def route.relay.stamp (chain : (Array UInt64)) (me : UInt64) (forwards : Bool) : (Array UInt64) := Id.run do
  let mut chain := chain
  if forwards then
    let t9 := chain
    let t14 := me
    if (t9.back?) ≠ (some t14 : (Option UInt64)) then
      chain := chain.push me
      return chain
    else
      return chain
  else
    return chain

def route.relay.Claim.exported {A : Type} [Inhabited A] [DecidableEq A] [route.Item A] (self : (route.relay.Claim A)) (me : UInt64) (forwards : Bool) (upstream : Bool) : (Option (route.relay.Claim A)) := Id.run do
  if upstream then
    if self.marked then
      return (none : (Option (route.relay.Claim A)))
    else
      let mut claim := self
      let mut r := route.relay.stamp claim.chain me forwards
      claim := { claim with chain := r }
      return if decide ((UInt64.ofNat claim.chain.size) < 32) then some claim else none
  else
    let mut claim := self
    let mut r := route.relay.stamp claim.chain me forwards
    claim := { claim with chain := r }
    return if decide ((UInt64.ofNat claim.chain.size) < 32) then some claim else none

def route.relay.takes.closure0 (arg1 : (UInt64 × (Array UInt64))) (arg2 : (UInt64 × UInt64)) : Rust.M Bool := do
  let «at» := arg2.1
  let relay := arg2.2
  let t15 := arg1.1
  if relay ≠ t15 then
    let t16 := arg1.2
    let t10 := (← Rust.sliceTo t16 «at»)
    return !(t10.contains relay)
  else
    return false

def route.relay.takes (chain : (Array UInt64)) (me : UInt64) (forwards : Bool) : Rust.M Bool := do
  let t6 := Rust.enumerate (chain.toList)
  let once := (← List.allM (route.relay.takes.closure0 (me, chain)) t6)
  if once then
    let t14 := UInt64.ofNat chain.size
    let t16 := if forwards then (1 : UInt64) else 0
    return decide ((← t14 +? t16) < 32)
  else
    return false

def route.relay.Claim.adopted {A : Type} [Inhabited A] [DecidableEq A] [route.Item A] (self : (route.relay.Claim A)) (me : UInt64) (forwards : Bool) (port : route.relay.Port) : Rust.M (Option (route.relay.Claim A)) := do
  let t10 := self.chain
  let t8 := t10
  let d := (← route.relay.takes t8 me forwards)
  if d then
    let mut claim := self
    claim := { claim with cost := min (Rust.saturatingAdd claim.cost port.cost) 4611686018427387903 }
    claim := { claim with priority := min claim.priority port.priority }
    claim := { claim with marked := claim.marked || port.upstream }
    let r := route.relay.stamp claim.chain me forwards
    claim := { claim with chain := r }
    return (some claim : (Option (route.relay.Claim A)))
  else
    return (none : (Option (route.relay.Claim A)))

def route.relay.claim_order {A : Type} [Inhabited A] [DecidableEq A] [route.Item A] (claim : (route.relay.Claim A)) (route1 : (Option route.relay.Offer)) (origin : route.relay.NodeId) : Rust.M ((Bool × (Rust.Reverse Int8) × UInt64) × Bool × UInt64 × route.relay.NodeId) := do
  let mut t6 : ((Bool × (Rust.Reverse Int8) × UInt64) × Bool) := default
  match route1 with
  | none =>
    t6 := (claim.rank, claim.marked)
  | some route2 =>
    t6 := claim.joined route2
  let rank := t6.1
  let marked := t6.2
  let t15 := UInt64.ofNat claim.chain.size
  let t17 := if route1.isSome then (1 : UInt64) else 0
  let taken := (← t15 +? t17)
  return (rank, marked, taken, origin)

def route.relay.precedence {A : Type} [Inhabited A] [DecidableEq A] [route.Item A] (item : A) : Rust.M (UInt64 × (Rust.Reverse UInt64)) := do
  let t2 := (← route.Item.above item)
  return (t2, ({ x0 := (← route.Item.epoch item) } : (Rust.Reverse UInt64)))

def route.relay.debounce (hop : Rust.Duration) : Rust.Duration :=
  Rust.Duration.saturatingMul hop ((32 : UInt64).toUInt32)

end moq_net
