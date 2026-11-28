-- Translated from the `kio` crate by rust2-lean, which a translated crate calls.
import Rust

set_option linter.unusedVariables false
-- Rust's wide tuples need deeper instance searches than Lean's default.
set_option synthInstance.maxSize 1024

namespace kio

/-- `kio::clock::instant::Instant` -/
structure clock.instant.Instant where
  x0 : UInt64
deriving Repr, DecidableEq, Inhabited, Ord

def clock.instant.Instant.checked_duration_since (self : clock.instant.Instant) (earlier : clock.instant.Instant) : (Option Rust.Duration) :=
  (Rust.checkedSub self.x0 earlier.x0).map Rust.Duration.fromNanos

def clock.instant.Instant.saturating_duration_since (self : clock.instant.Instant) (earlier : clock.instant.Instant) : Rust.Duration :=
  (self.checked_duration_since earlier).getD Rust.Duration.ZERO

def clock.instant.Instant.duration_since (self : clock.instant.Instant) (earlier : clock.instant.Instant) : Rust.Duration :=
  self.saturating_duration_since earlier

def clock.instant.Instant.checked_add (self : clock.instant.Instant) (duration : Rust.Duration) : (Option clock.instant.Instant) := Id.run do
  let t4 := Rust.branch ((Rust.tryFrom UInt64 (Rust.Duration.asNanos duration)).toOption)
  match t4 with
  | .Continue val =>
    let nanos := val
    return (Rust.checkedAdd self.x0 nanos).map clock.instant.Instant.mk
  | .Break t4_01 =>
    return (none : (Option clock.instant.Instant))

def clock.instant.Instant.checked_sub (self : clock.instant.Instant) (duration : Rust.Duration) : (Option clock.instant.Instant) := Id.run do
  let t4 := Rust.branch ((Rust.tryFrom UInt64 (Rust.Duration.asNanos duration)).toOption)
  match t4 with
  | .Continue val =>
    let nanos := val
    return (Rust.checkedSub self.x0 nanos).map clock.instant.Instant.mk
  | .Break t4_01 =>
    return (none : (Option clock.instant.Instant))

def clock.instant.Instant.saturating_add (self : clock.instant.Instant) (duration : Rust.Duration) : clock.instant.Instant :=
  (self.checked_add duration).getD ({ x0 := (18446744073709551615 : UInt64) } : clock.instant.Instant)

def clock.instant.Instant.saturating_sub (self : clock.instant.Instant) (duration : Rust.Duration) : clock.instant.Instant :=
  (self.checked_sub duration).getD ({ x0 := (0 : UInt64) } : clock.instant.Instant)

def clock.instant.Instant.Sub.sub (self : clock.instant.Instant) (earlier : clock.instant.Instant) : Rust.Duration :=
  self.duration_since earlier

end kio
