-- Replays native Rust's outcomes, from `lean-diff`, against rust2-lean's translation.
import KioPureMachine
import Std.Data.HashMap
open kio_pure_machine

abbrev D := StateT (List Int) (Except String)

def int : D Int := do
  match (← get) with
  | x :: xs => set xs; pure x
  | [] => throw "truncated case"
def nat : D Nat := do let x ← int; if x < 0 then throw "negative" else pure x.toNat
def u8 : D UInt8 := (·.toUInt8) <$> nat
def u16 : D UInt16 := (·.toUInt16) <$> nat
def u32 : D UInt32 := (·.toUInt32) <$> nat
def u64 : D UInt64 := (·.toUInt64) <$> nat
def i16 : D Int16 := (Int16.ofInt ·) <$> int
def boolean : D Bool := (· != 0) <$> int
def opt (d : D α) : D (Option α) := do if (← int) == 0 then pure none else some <$> d
def pair (a : D α) (b : D β) : D (α × β) := do let x ← a; pure (x, ← b)
def arr (d : D α) : D (Array α) := do
  let n ← nat
  let mut a := #[]
  for _ in [0:n] do a := a.push (← d)
  pure a
def vec (n : Nat) (d : D α) : D (Vector α n) := do
  let mut a := #[]
  for _ in [0:n] do a := a.push (← d)
  if h : a.size = n then pure ⟨a, h⟩ else throw "length"

open send_credit in
section
def mutant : D Mutant := do
  match (← nat) with | 0 => pure .NoGiveWay | 1 => pure .NoPreemption | 2 => pure .NoLeaveWake | _ => throw "mutant"
def shape : D (Shape 2) := do pure { priority := ← vec 2 u8, ascending := ← vec 2 boolean, max_streams := ← u8, mutant := ← opt mutant }
def rank : D Rank := do pure { priority := ← u8, turn := ← u8, in_group := ← i16 }
def place : D Place := do pure { rank := ← rank, arrival := ← u8 }
def sub : D (Sub 3) := do pure { opening := ← opt u8, place := ← opt place, asked := ← boolean, turn := ← u8, serving := ← vec 3 boolean }
def session : D (Session 2 3) := do
  pure { shape := ← shape, subs := ← vec 2 sub, arrivals := ← u8, resets := ← u8, opened := ← u8, freed := ← u8 }
def event : D Event := do
  match (← nat) with
  | 0 => pure (.Poll (← u64) (← u16) (← opt u8))
  | 1 => pure (.Finished (← u64) (← u8))
  | 2 => pure .Freed
  | _ => throw "event"
def output : D Output := do
  match (← nat) with
  | 0 => pure (.Wake (← u64)) | 1 => pure (.Park (← u64))
  | 2 => pure (.Take (← u64) (← u8)) | 3 => pure (.Shed (← u64) (← u8))
  | 4 => pure (.Open (← u64) (← u8)) | 5 => pure (.Reset (← u64) (← u8))
  | _ => throw "output"
end

def disabled : D (Except Disabled Unit) := do pure (if (← boolean) then .error {} else .ok ())

open warm_settle in
section
def sig : D Sig := do pure { cost := ← u32, path := ← u16, rank := ← u32 }
def answer : D Answer := do
  match (← nat) with | 0 => pure .None | 1 => pure .Wait | 2 => pure (.Live (← sig)) | 3 => pure .Lost | _ => throw "answer"
def heard : D Heard := do pure { pull := ← opt u32, answer := ← answer, advert := ← opt sig }
def link : D Link := do pure { «open» := ← boolean, cold := ← u32, warm := ← u32 }
def relay : D (Relay 4) := do
  pure { id := ← u8, reader := ← opt u32, route := ← opt (pair u8 u32), links := ← vec 4 (opt link),
         heard := ← vec 4 heard, up := ← opt u8, next := ← opt u8, answered := ← u16 }
def wevent : D Event := do
  match (← nat) with | 0 => pure (.Hear (← u8) (← heard)) | 1 => pure .React | _ => throw "event"
def choice : D Choice := do pure { via := ← u8, cost := ← u32, copy := ← boolean, rank := ← u32 }
end

/-- An ASCII string, as its length and bytes. -/
def str : D String := do
  let n ← nat
  let mut cs := []
  for _ in [0:n] do cs := Char.ofNat (← nat) :: cs
  pure (String.ofList cs.reverse)

/-- Checks the translation's outcome against native Rust's next, `0` and the message for a
panic, else `1` and the value. Divergence never matches. -/
def expect [DecidableEq α] (what : String) (got : Rust.M α) (d : D α) : D (List String) := do
  let rust ← (if (← boolean) then Except.ok <$> d else Except.error <$> str)
  match got.run, rust with
  | some (.ok v), .ok w => pure (if v = w then [] else [s!"{what}: values differ"])
  | some (.error e), .error m => pure (if e = m then [s!"={what}"] else [s!"{what}: Lean threw {e}, Rust panicked with {m}"])
  | some (.ok _), .error m => pure [s!"{what}: Rust panicked with {m}, Lean returned"]
  | some (.error e), .ok _ => pure [s!"{what}: Lean threw {e}, Rust returned"]
  | none, _ => pure [s!"{what}: Lean diverged"]

def case : D (List String) := do
  match (← nat) with
  | 1 =>
    let s ← session
    let e ← event
    let a ← expect "Session::step" (s.step e #[]) (do pure (← disabled, ← session, ← arr output))
    let b ← expect "within_credit" s.within_credit boolean
    let c ← expect "well_formed" s.well_formed boolean
    pure (a ++ b ++ c)
  | 2 =>
    let r ← relay
    let e ← wevent
    let a ← expect "Relay::step" (warm_settle.Relay.StateMachine.step r e #[]) (do pure (← disabled, ← relay, #[]))
    let b ← expect "next_pulls" r.next_pulls (pair (opt u8) (opt u8))
    let c ← expect "options(None)" (r.options none) (arr choice)
    let u ← opt u8
    let d ← expect "options(u)" (r.options u) (arr choice)
    let m ← expect "mine" r.mine u32
    let n ← expect "demand" r.demand boolean
    pure (a ++ b ++ c ++ d ++ m ++ n)
  | 3 =>
    let cs ← arr choice
    expect "best" (warm_settle.best cs) (opt choice)
  | _ => throw "kind"

def main (args : List String) : IO UInt32 := do
  let lines ← IO.FS.lines (args.headD "cases.txt")
  let mut failed := 0
  let mut checked := 0
  let mut panics : Std.HashMap String Nat := {}
  for (line, i) in lines.toList.zipIdx do
    let ints := (line.splitOn " ").filterMap String.toInt?
    match case.run ints with
    | .error e => IO.eprintln s!"case {i + 1}: {e}"; failed := failed + 1
    | .ok (errs, rest) =>
      checked := checked + 1
      unless rest.isEmpty do IO.eprintln s!"case {i + 1}: trailing input"; failed := failed + 1
      for e in errs do
        if e.startsWith "=" then
          panics := panics.alter (e.drop 1).toString (some <| ·.getD 0 + 1)
          continue
        if failed < 20 then IO.eprintln s!"case {i + 1}: {e}"
        failed := failed + 1
  IO.println s!"{checked} cases, {failed} mismatches; both panicked: {panics.toList}"
  pure (if failed == 0 then 0 else 1)
