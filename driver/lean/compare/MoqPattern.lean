-- Translated from the `moq_pattern` crate by rust2-lean, which a translated crate calls.
import Rust

set_option linter.unusedVariables false
-- Rust's wide tuples need deeper instance searches than Lean's default.
set_option synthInstance.maxSize 1024

namespace moq_pattern

/-- `moq_pattern::parts::Parts` -/
structure parts.Parts where
  rest : (Option String)
deriving Repr, DecidableEq, Inhabited

def parts.parts (path : String) : parts.Parts :=
  ({ rest := if !(path.isEmpty) then some path else none } : parts.Parts)

def parts.escaped.closure0 (arg1 : Unit) (arg2 : UInt8) : Bool := Id.run do
  let t5 := arg2
  let b := t5
  return decide (b = 92)

def parts.escaped (bytes1 : (Array UInt8)) («at» : UInt64) : Rust.M Bool := do
  let t9 := (← Rust.sliceTo bytes1 «at»)
  let t4 := (((t9.toList).reverse).takeWhile (parts.escaped.closure0 ())).length.toUInt64
  unless ¬((2 : UInt64) = (0 : UInt64)) do throw "attempt to calculate the remainder with a divisor of zero"
  return decide ((t4 % 2) = 1)

def parts.Parts.Iterator.next_loop (self : parts.Parts) (s : String) (bytes1 : (Array UInt8)) («from» : UInt64) : Rust.M ((Option String) × parts.Parts) := do
  let mut self := self
  let mut «from» := «from»
  let t17 := (← Rust.strSliceFrom s «from»)
  let t15 := Rust.strFind t17 '/'
  match id t15 with
  | some i =>
    let t24 := «from»
    let «at» := (← t24 +? i)
    let d := (← parts.escaped bytes1 «at»)
    if d then
      «from» := (← «at» +? 1)
      return (← parts.Parts.Iterator.next_loop self s bytes1 «from»)
    else
      let t35 := (← Rust.strSliceFrom s (← «at» +? 1))
      let t34 := t35
      self := { self with rest := (some t34 : (Option String)) }
      let t43 := (← Rust.strSliceTo s «at»)
      let t42 := t43
      return (((some t42 : (Option String)), self))
  | none =>
    self := { self with rest := (none : (Option String)) }
    return (((some s : (Option String)), self))
partial_fixpoint

def parts.Parts.Iterator.next (self : parts.Parts) : Rust.M ((Option String) × parts.Parts) := do
  let t3 := Rust.branch self.rest
  match t3 with
  | .Continue val =>
    let s := val
    let bytes1 := Rust.asBytes s
    let «from» : UInt64 := 0
    let r ← parts.Parts.Iterator.next_loop self s bytes1 «from»
    return r
  | .Break t3_01 =>
    return ((none : (Option String)), self)

end moq_pattern
