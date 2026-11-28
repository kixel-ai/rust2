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

end kio
