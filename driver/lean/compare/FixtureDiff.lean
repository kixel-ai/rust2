-- Replays native Rust's outcomes, from the fixture's `tests/lean_diff.rs`, against
-- rust2-lean's translation of the fixture's `cases`.
import Fixture
import Std.Data.HashMap
open fixture

-- The widest cases' tuples need more than the default to find their `DecidableEq`.
set_option synthInstance.maxSize 4096

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
def u128 : D Rust.UInt128 := bounded 128 Rust.UInt128.ofNat
def i8 : D Int8 := do
  let x ← int
  if -128 ≤ x ∧ x < 128 then pure (Int8.ofInt x) else throw "past i8"
def i64 : D Int64 := do
  let x ← int
  if -2 ^ 63 ≤ x ∧ x < 2 ^ 63 then pure (Int64.ofInt x) else throw "past i64"
def i128 : D Rust.Int128 := do
  let x ← int
  if -2 ^ 127 ≤ x ∧ x < 2 ^ 127 then pure (Rust.Int128.ofInt x) else throw "past i128"
def boolean : D Bool := (· != 0) <$> int
def opt (d : D α) : D (Option α) := do if (← int) == 0 then pure none else some <$> d
def pair (a : D α) (b : D β) : D (α × β) := do let x ← a; pure (x, ← b)
def arr (d : D α) : D (Array α) := do
  let n ← nat
  let mut a := #[]
  for _ in [0:n] do a := a.push (← d)
  pure a
def rank : D cases.Rank := do pure { up := ← boolean, low := ⟨← i8⟩ }
def dur : D Rust.Duration := do pure ⟨← u64, ← u32⟩
def ordering : D Ordering := do
  match (← int) with
  | -1 => pure .lt
  | 0 => pure .eq
  | 1 => pure .gt
  | _ => throw "an ordering"
def ledger : D cases.Ledger := do
  pure { m := ⟨(← arr (pair rank u16)).toList⟩, s := ⟨(← arr u8).toList⟩, q := ⟨(← arr u8).toList⟩ }
def tally : D cases.lenses.Tally := do pure { hits := ← u16, last := ← u8 }
def book : D cases.lenses.Book := do
  let tallies ← arr (pair u8 tally)
  let nested ← arr (pair u8 (do pure (⟨(← arr (pair u8 u16)).toList⟩ : Rust.BTreeMap UInt8 UInt16)))
  let seeds ← arr (pair u8 (do pure (⟨← u16⟩ : cases.lenses.Seeded)))
  pure { tallies := ⟨tallies.toList⟩, nested := ⟨nested.toList⟩, seeds := ⟨seeds.toList⟩, log := ← opt (arr u8) }
def result (a : D α) (e : D ε) : D (Except ε α) := do
  if (← int) == 0 then .error <$> e else .ok <$> a
def instant : D kio.clock.instant.Instant := do pure ⟨← u64⟩
def step : D cases.Step := do if (← int) == 0 then pure .Back else .Ahead <$> u16

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

def char : D Char := do
  let n ← nat
  if n.isValidChar then pure (Char.ofNat n) else throw "not a char"
def queue (d : D α) : D (Rust.VecDeque α) := do pure ⟨(← arr d).toList⟩
def set (d : D α) : D (Rust.BTreeSet α) := do pure ⟨(← arr d).toList⟩

open cases.text cases.adaptors in
def case : D (List String) := do
  match (← nat) with
  | 0 =>
    let rs ← arr (pair nat nat)
    pure (if rs.toList = Rust.debugEscaped.toList then ["+debugEscaped"] else ["debugEscaped: differs from Rust's"])
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
  | 13 =>
    let b ← boolean; let o ← opt u8
    expect "then_unwrap" (cases.then_unwrap b o) (opt u8)
  | 14 =>
    let ms ← u64; let ns ← u64; let s ← u64
    expect "spans" (pure (cases.spans ms ns s)) (pair dur (pair dur (pair dur (pair dur (pair dur dur)))))
  | 15 =>
    let a ← dur; let b ← dur; let k ← u32
    expect "sums" (pure (cases.sums a b k))
      (pair (opt dur) (pair (opt dur) (pair (opt dur) (pair dur (pair dur dur)))))
  | 16 =>
    let a ← dur; let b ← dur; let p ← pair u8 rank; let q ← pair u8 rank
    expect "compared" (pure (cases.compared a b p q))
      (pair boolean (pair boolean (pair boolean (pair boolean (pair dur (pair dur (pair ordering (pair boolean boolean))))))))
  | 17 =>
    let l ← ledger; let r ← rank; let v ← u16; let k ← u8
    expect "edit" (pure (l.edit r v k))
      (pair (pair (opt u16) (pair (opt (pair rank u16)) (pair (opt u16) (pair boolean (pair boolean (opt u8)))))) ledger)
  | 18 =>
    let l ← ledger; let x ← u8
    expect "queue" (pure (l.queue x)) (pair (pair (opt u8) (pair boolean u64)) ledger)
  | 19 =>
    let l ← ledger; let r ← rank
    expect "peeks" (cases.peeks l r)
      (pair (opt (pair rank u16)) (pair (arr rank) (pair (arr u16) (pair (arr (pair rank u16))
        (pair (opt u8) (pair (arr u8) (pair (arr u8) u16)))))))
  | 20 =>
    let b ← book; let k ← u8; let n ← u16
    expect "bump" (b.bump k n) (pair (opt u16) book)
  | 21 =>
    let b ← book; let k ← u8; let j ← u8
    expect "touch" (b.touch k j) (pair (result u8 u8) book)
  | 22 => let b ← book; let k ← u8; expect "moved_key" (pure (b.moved_key k)) (pair (opt u8) book)
  | 23 =>
    let b ← book; let k ← u8; let j ← u8
    expect "count" (b.count k j) (pair (pair u16 u16) book)
  | 24 =>
    let b ← book; let x ← u8
    expect "logs" (pure (b.logs x)) (pair (pair (arr u8) (pair (opt (arr u8)) tally)) book)
  | 25 =>
    let b ← book; let k ← u8; let f ← opt u16; let named ← boolean
    expect "eager" (b.eager k f named) (pair (pair u16 u8) book)
  | 26 =>
    let i ← instant; let d ← dur; let j ← u64
    expect "ticks" (pure (cases.ticks i d j)) (pair (opt u64) (pair u64 (pair dur u64)))
  | 27 =>
    let o ← opt u16; let n ← opt u64
    expect "passed" (cases.passed o n)
      (pair (opt (do pure (⟨← u16⟩ : cases.lenses.Seeded))) (pair (opt dur) (pair (opt (opt u16))
        (pair (opt (result u16 u8)) (pair (opt step) (opt u16))))))
  | 28 => let d ← dur; expect "nanos" (pure (cases.nanos d)) (pair u128 (pair u128 u128))
  | 29 =>
    let a ← u128; let b ← u128; let s ← u32
    expect "wide_bits" (cases.wide_bits a b s) (pair u128 (pair u128 (pair u128 (pair u128 (pair u128 (pair u128 u128))))))
  | 30 =>
    let a ← u128; let b ← u128
    expect "wide_order" (pure (cases.wide_order a b))
      (pair (opt u128) (pair (opt u128) (pair u128 (pair u128 (pair u128 (pair u128 (pair boolean ordering)))))))
  | 31 => let a ← u128; let b ← u128; let op ← u8; expect "wide_arith" (cases.wide_arith a b op) u128
  | 32 => let x ← i128; let y ← i128; let op ← u8; expect "signed_arith" (cases.signed_arith x y op) i128
  | 33 =>
    let x ← i128; let y ← i128; let s ← u32
    expect "signed_bits" (cases.signed_bits x y s)
      (pair i128 (pair i128 (pair i128 (pair i128 (pair boolean (pair ordering (pair i128 i128)))))))
  | 34 =>
    let a ← u128; let x ← i128
    expect "wide_casts" (pure (cases.wide_casts a x))
      (pair u64 (pair i64 (pair i128 (pair u128 (pair (opt u64) (pair (opt i64) (pair (opt u128)
        (pair (opt i128) (pair u128 i128)))))))))
  | 35 => let s ← str; expect "measure" (pure (measure s)) (pair u64 (pair boolean (arr u8)))
  | 36 =>
    let s ← str; let b ← u64; let e ← u64; let how ← u8
    expect "cut" (cut s b e how) str
  | 37 =>
    let s ← str; let c ← char; let p ← str; let i ← u64
    expect "search" (pure (search s c p i)) (pair (opt u64) (pair boolean (pair boolean (opt u8))))
  | 38 => let v ← arr u8; let x ← u8; expect "tails" (tails v x) (pair u64 u64)
  | 39 =>
    let v ← arr u8; let x ← u8
    expect "bumped" (do pure (← last_bumped v, ← find_bumped v x)) (pair (opt u8) (opt u8))
  | 40 =>
    let n ← u8; let v ← arr u8; let m ← arr (pair u8 u16); let s ← arr u8; let o ← opt u8; let q ← queue u8
    expect "loops" (loops n v ⟨m.toList⟩ ⟨s.toList⟩ o q) (pair u16 (pair u16 (pair u16 (pair u16 u16))))
  | 41 => let n ← u8; expect "countdown" (countdown n) u16
  | 42 => let c ← char; expect "classify" (pure (classify c)) (pair u8 (pair u32 (pair u8 (pair i8 (pair i64 char)))))
  | 43 => let s ← str; let b ← u8; expect "shadowed" (pure (shadowed s b () () () ⟨b⟩ () ⟨none, b⟩)) (pair boolean (pair char (pair u8 u8)))
  | 44 =>
    let v ← arr u8; let x ← u8
    expect "sift" (pure (sift v x))
      (pair (arr u8) (pair (arr u8) (pair (arr u8) (pair (opt u8) (opt u8)))))
  | 45 =>
    let v ← arr u8; let x ← u8
    pure ((← expect "filtered" (filtered v x) (arr u8))
      ++ (← expect "any_filtered" (any_filtered v x) boolean)
      ++ (← expect "find_filter_mapped" (find_filter_mapped v x) (opt u8))
      ++ (← expect "find_mapped" (find_mapped v x) (opt u8))
      ++ (← expect "found" (found v x) (opt u8)))
  | 46 =>
    let v ← arr u8; let w ← arr u8; let x ← u8
    pure ((← expect "flat_runs" (flat_runs v x) (arr u8))
      ++ (← expect "flat_back" (flat_back v x) (pair (arr u8) (opt u8)))
      ++ (← expect "flat_first" (flat_first v x) (opt u8))
      ++ (← expect "flat_inner" (flat_inner v w) (pair u64 boolean)))
  | 47 =>
    let v ← arr u8; let x ← u8
    pure ((← expect "least" (least v x) (pair u8 (pair u16 (pair (opt u8) (opt u8)))))
      ++ (← expect "sum_lazy" (sum_lazy v x) u8)
      ++ (← expect "least_lazy" (least_lazy v x) (opt u8)))
  | 48 =>
    let m ← arr (pair u8 u16); let a ← u8; let b ← u8
    expect "sources" (sources ⟨m.toList⟩ a b)
      (pair (arr u8) (pair (arr u8) (pair (arr u16) u64)))
  | 49 =>
    let v ← arr u8; let x ← u8; let s ← set u8
    let ms ← arr (do pure (⟨(← arr (pair u8 u16)).toList⟩ : Rust.BTreeMap UInt8 UInt16))
    pure ((← expect "gather" (gather v s ms)
        (pair (set u8) (pair (set u8) (pair (arr u8) (pair (arr u64) (pair (arr u8) (pair (arr u16) boolean)))))))
      ++ (← expect "peeked" (pure (peeked v x)) (pair (opt u8) (arr u8)))
      ++ (← expect "peeked_lazy" (peeked_lazy v x) (opt u8))
      ++ (← expect "peeked_rest" (peeked_rest v x) (pair (opt u8) (arr u8))))
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
