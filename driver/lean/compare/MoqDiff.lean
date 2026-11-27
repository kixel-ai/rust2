-- Replays native Rust's outcomes, from `moq_diff.rs`, against rust2-lean's translation of
-- moq-net's route algebra.
import MoqNet
import Std.Data.HashMap
open moq_net

-- `order`'s wide tuple needs a deeper instance search than Lean's default.
set_option synthInstance.maxSize 1024

abbrev D := StateT (List Int) (Except String)

def int : D Int := do
  match (← get) with
  | x :: xs => set xs; pure x
  | [] => throw "truncated case"
def nat : D Nat := do let x ← int; if x < 0 then throw "negative" else pure x.toNat
def u64 : D UInt64 := do
  let n ← nat
  if n < 2 ^ 64 then pure n.toUInt64 else throw "past u64"
def i8 : D Int8 := do
  let x ← int
  if -128 ≤ x ∧ x < 128 then pure (Int8.ofInt x) else throw "past i8"
def boolean : D Bool := (· != 0) <$> int
def opt (d : D α) : D (Option α) := do if (← int) == 0 then pure none else some <$> d
def pair (a : D α) (b : D β) : D (α × β) := do let x ← a; pure (x, ← b)
def arr (d : D α) : D (Array α) := do
  let n ← nat
  let mut a := #[]
  for _ in [0:n] do a := a.push (← d)
  pure a
def rev (d : D α) : D (Rust.Reverse α) := do pure ⟨← d⟩
def map (k : D κ) (v : D ν) : D (Rust.BTreeMap κ ν) := do pure ⟨(← arr (pair k v)).toList⟩
def set (k : D κ) : D (Rust.BTreeSet κ) := do pure ⟨(← arr k).toList⟩

/-- `moq_diff.rs`'s `Gen`, and its `Item` impl as written there. -/
structure Gen where
  above : UInt64
  epoch : UInt64
deriving DecidableEq, Inhabited, Repr

instance : route.Item Gen where
  lowered g n := do pure { g with above := ← g.above +? n }
  above g := pure g.above
  epoch g := pure g.epoch

open route.relay in
section
def node : D NodeId := do pure { relay := ← u64, marked := ← boolean }
def hop : D Hop := do pure { node := ← node, session := ← u64 }
def offer : D Offer := do
  pure { path := ← arr hop, draining := ← boolean, priority := ← i8, cost := ← u64, marked := ← boolean }
def role : D Role := do
  match (← nat) with
  | 0 => pure (.Routing (← node))
  | 1 => if (← nat) == 0 then pure (.Scoped #[]) else throw "scoped paths"
  | 2 => pure .Endpoint
  | _ => throw "role"
def port : D Port := do pure { cost := ← u64, priority := ← i8, upstream := ← boolean, role := ← role }
def here : D Here := do
  pure { closed := ← map (pair u64 u64) u64, dead := ← map hop node, draining := ← set node }
def gen : D Gen := do pure { above := ← u64, epoch := ← u64 }
def claim : D (Claim Gen) := do
  pure { item := ← gen, cost := ← u64, priority := ← i8, draining := ← boolean, unavailable := ← boolean,
         chain := ← arr u64, marked := ← boolean }
def best : D (Best UInt64) := do pure { via := ← u64, offer := ← offer }
def rank : D (Bool × Rust.Reverse Int8 × UInt64) := pair boolean (pair (rev i8) u64)
end

/-- A panic's message, as its length and bytes. -/
def str : D String := do
  let n ← nat
  let mut bs := ByteArray.empty
  for _ in [0:n] do bs := bs.push (← nat).toUInt8
  match String.fromUTF8? bs with
  | some s => pure s
  | none => throw "message is not UTF-8"

/-- Checks the translation's outcome against native Rust's next, `0` and the message for a
panic, else `1` and the value. Divergence never matches. -/
def expect [DecidableEq α] (what : String) (got : Rust.M α) (d : D α) : D (List String) := do
  let rust ← (if (← boolean) then Except.ok <$> d else Except.error <$> str)
  match got.run, rust with
  | some (.ok v), .ok w => pure (if v = w then [s!"+{what}"] else [s!"{what}: values differ"])
  | some (.error e), .error m => pure (if e = m then [s!"={what}"] else [s!"{what}: Lean threw {e}, Rust panicked with {m}"])
  | some (.ok _), .error m => pure [s!"{what}: Rust panicked with {m}, Lean returned"]
  | some (.error e), .ok _ => pure [s!"{what}: Lean threw {e}, Rust returned"]
  | none, _ => pure [s!"{what}: Lean diverged"]

open route.relay in
def case : D (List String) := do
  match (← nat) with
  | 1 =>
    let me ← node; let origin ← node; let upstream ← boolean
    expect "Offer::own" (pure (Offer.own me origin upstream)) (opt offer)
  | 2 =>
    let o ← offer; let me ← node; let session ← u64; let to ← node; let upstream ← boolean
    expect "Offer::onward" (pure (o.onward me session to upstream)) (opt offer)
  | 3 =>
    let h ← here; let p ← port; let draining ← boolean; let o ← offer
    expect "Here::arrive" (h.arrive p draining o) (opt offer)
  | 4 =>
    let b ← best; let current ← opt u64
    expect "order" (order b current)
      (pair boolean (pair (rev i8) (pair u64 (pair boolean (pair boolean (pair u64 (pair node u64)))))))
  | 5 =>
    let c ← claim; let o ← offer
    expect "Claim::over" (pure (c.over o)) claim
  | 6 =>
    let c ← claim; let me ← u64; let forwards ← boolean; let upstream ← boolean
    expect "Claim::exported" (pure (c.exported me forwards upstream)) (opt claim)
  | 7 =>
    let c ← claim; let me ← u64; let forwards ← boolean; let p ← port
    expect "Claim::adopted" (c.adopted me forwards p) (opt claim)
  | 8 =>
    let c ← claim; let n ← u64
    expect "Claim::lowered" (c.lowered n) claim
  | 9 =>
    let c ← claim; let o ← opt offer; let origin ← node
    expect "claim_order" (claim_order c o origin) (pair rank (pair boolean (pair u64 node)))
  | 10 =>
    let g ← gen
    expect "precedence" (precedence g) (pair u64 (rev u64))
  | 11 =>
    let h ← hop; let far ← node
    expect "floor_of" (pure (floor_of h far)) (pair (pair u64 u64) u64)
  | _ => throw "kind"

def main (args : List String) : IO UInt32 := do
  let lines ← IO.FS.lines (args.headD "cases.txt")
  let mut failed := 0
  let mut checked := 0
  let mut tally : Std.HashMap String Nat := {}
  for (line, i) in lines.toList.zipIdx do
    let ints := (line.splitOn " ").filterMap String.toInt?
    match case.run ints with
    | .error e => IO.eprintln s!"case {i + 1}: {e}"; failed := failed + 1
    | .ok (errs, rest) =>
      checked := checked + 1
      unless rest.isEmpty do IO.eprintln s!"case {i + 1}: trailing input"; failed := failed + 1
      for e in errs do
        if e.startsWith "=" || e.startsWith "+" then
          tally := tally.alter e (some <| ·.getD 0 + 1)
          continue
        if failed < 20 then IO.eprintln s!"case {i + 1}: {e}"
        failed := failed + 1
  let shown := tally.toList.toArray.qsort (·.1 < ·.1)
  IO.println s!"{checked} cases, {failed} mismatches; matched (+ returned, = both panicked): {shown.toList}"
  pure (if failed == 0 then 0 else 1)
