-- Replays native Rust's outcomes, from the fixture's `tests/lean_diff.rs`, against
-- rust2-lean's translation of the fixture's `cases`.
import Fixture
import Std.Data.HashMap
open fixture

abbrev D := StateT (List Int) (Except String)

def int : D Int := do
  match (← get) with
  | x :: xs => set xs; pure x
  | [] => throw "truncated case"
def nat : D Nat := do let x ← int; if x < 0 then throw "negative" else pure x.toNat
def bounded (bits : Nat) (f : Nat → α) : D α := do
  let n ← nat
  if n < 2 ^ bits then pure (f n) else throw s!"past u{bits}"
def u8 : D UInt8 := bounded 8 Nat.toUInt8
def u16 : D UInt16 := bounded 16 Nat.toUInt16
def u32 : D UInt32 := bounded 32 Nat.toUInt32
def u64 : D UInt64 := bounded 64 Nat.toUInt64
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
def rank : D cases.Rank := do pure { up := ← boolean, low := ⟨← i8⟩ }

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

def case : D (List String) := do
  match (← nat) with
  | 1 => let v ← arr u8; let x ← u8; expect "shifted" (cases.shifted v x) (arr u8)
  | 2 => let v ← arr u8; let x ← u8; expect "reaches" (cases.reaches v x) boolean
  | 3 => let a ← arr u8; let b ← arr u8; expect "rising" (pure (cases.rising a b)) boolean
  | 4 => let v ← arr u8; expect "pair_sums" (cases.pair_sums v) (arr u16)
  | 5 => let v ← arr u8; expect "ends" (pure (cases.ends v)) (opt (pair u8 (pair u8 u8)))
  | 6 =>
    let v ← arr u8; let n ← u64
    expect "ranges" (cases.ranges v n) (pair u64 (pair u64 (pair u64 u64)))
  | 7 => let o ← opt u8; expect "must" (cases.must o) u8
  | 8 => let a ← u8; let b ← u8; expect "saturating" (pure (cases.saturating a b)) (pair u8 u8)
  | 9 => let a ← u8; let b ← boolean; expect "widen" (pure (cases.widen a b)) (pair u32 u8)
  | 10 =>
    let o ← opt u8; let b ← boolean
    expect "options" (cases.options o b) (pair boolean (pair (opt u16) (pair u8 (opt u8))))
  | 11 => let v ← arr u8; expect "shared" (pure (cases.shared v)) (arr u8)
  | 12 =>
    let m ← arr (pair rank u16); let s ← arr u8; let r ← rank; let k ← u8
    expect "lookups" (pure (cases.lookups ⟨m.toList⟩ ⟨s.toList⟩ r k))
      (pair (opt u16) (pair boolean (pair boolean (pair u64 boolean))))
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
