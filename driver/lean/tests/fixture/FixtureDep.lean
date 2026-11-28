-- Translated from the `fixture_dep` crate by rust2-lean, which a translated crate calls.
import Rust
import Kio

set_option linter.unusedVariables false
-- Rust's wide tuples need deeper instance searches than Lean's default.
set_option synthInstance.maxSize 1024

namespace fixture_dep

/-- `fixture_dep::Ticks` -/
structure Ticks where
  x0 : UInt64
deriving Repr, DecidableEq, Inhabited, Ord

def Ticks.of (i : kio.clock.instant.Instant) : Ticks :=
  ({ x0 := i.x0 } : Ticks)

def Ticks.checked_add (self : Ticks) (d : Rust.Duration) : (Option Ticks) := Id.run do
  let t4 := Rust.branch ((Rust.tryFrom UInt64 (Rust.Duration.asNanos d)).toOption)
  match t4 with
  | .Continue val =>
    let nanos := val
    return (Rust.checkedAdd self.x0 nanos).map Ticks.mk
  | .Break t4_01 =>
    return (none : (Option Ticks))

def Ticks.saturating_add (self : Ticks) (d : Rust.Duration) : Ticks :=
  (self.checked_add d).getD ({ x0 := (18446744073709551615 : UInt64) } : Ticks)

def Ticks.get (self : Ticks) : UInt64 :=
  self.x0

def Ticks.since (self : Ticks) (earlier : Ticks) : Rust.Duration :=
  ((Rust.checkedSub self.x0 earlier.x0).map Rust.Duration.fromNanos).getD Rust.Duration.ZERO

def larger {T : Type} [Inhabited T] [DecidableEq T] [Ord T] (a : T) (b : T) : T := Id.run do
  if compare a b = .lt then
    return b
  else
    return a

end fixture_dep
