-- Translated from the `moq_net` crate by rust2-lean, with overflow checks on.
import Rust

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

/-- `path::Repr` -/
inductive path.Repr' where
  | Inline (len : UInt8) (buf : (Vector UInt8 22))
  | Static (x0 : String)
  | Shared (start : UInt16) (buf : String)
deriving Repr, DecidableEq, Inhabited

/-- `path::Path` -/
structure path.Path where
  x0 : path.Repr'
deriving Repr, DecidableEq, Inhabited

/-- `route::relay::Role` -/
inductive route.relay.Role where
  | Routing (x0 : route.relay.NodeId)
  | Scoped (x0 : (Array path.Path))
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

def route.relay.pairs (path1 : (Array route.relay.Hop)) : Rust.M (List (Unit → Rust.M ((UInt64 × UInt64) × UInt64))) := do
  return (← Rust.windows path1 (2 : UInt64)).map (Rust.defer (route.relay.pairs.closure0 ()))

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
  let floored := (← List.anyM (Rust.force (pure ∘ (route.relay.Here.arrive.closure0 t58))) t7)
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
