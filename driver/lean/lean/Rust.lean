/-!
# Rust

What every rust2-lean translation imports: Rust's panics and loops as a monad, and the
integer operations whose overflow Rust checks. Core Lean only, so a translation builds
without Mathlib or any other package.
-/

namespace Rust

/-- A Rust computation: a value, a panic with its message, or (`none`) a loop that never
ends. Loops are `partial_fixpoint` definitions in it. -/
abbrev M := ExceptT String Option

/-- Whether `m` returns `v`, for differential tests. -/
def returns {α : Type} [BEq α] (m : M α) (v : α) : Bool :=
  match m.run with
  | some (.ok x) => x == v
  | _ => false

/-- Whether `m` panics, for differential tests. -/
def panics {α : Type} (m : M α) : Bool :=
  match m.run with
  | some (.error _) => true
  | _ => false

/-- A fixed-width integer: its value as an `Int`, and its range. -/
class Scalar (α : Type) extends Add α, Sub α, Mul α where
  val : α → Int
  lo : Int
  hi : Int
  /-- Wrapping conversion from an `Int`, as Rust's `as` casts. -/
  ofInt : Int → α
  val_add (a b : α) : lo ≤ val a + val b → val a + val b ≤ hi → val (a + b) = val a + val b
  val_sub (a b : α) : lo ≤ val a - val b → val a - val b ≤ hi → val (a - b) = val a - val b
  val_mul (a b : α) : lo ≤ val a * val b → val a * val b ≤ hi → val (a * b) = val a * val b

export Scalar (val)

/-- `a + b`, panicking where Rust's overflow check does. -/
def add {α : Type} [Scalar α] (a b : α) : M α :=
  if Scalar.lo α ≤ val a + val b ∧ val a + val b ≤ Scalar.hi α then pure (a + b)
  else throw "attempt to add with overflow"

/-- `a - b`, panicking where Rust's overflow check does. -/
def sub {α : Type} [Scalar α] (a b : α) : M α :=
  if Scalar.lo α ≤ val a - val b ∧ val a - val b ≤ Scalar.hi α then pure (a - b)
  else throw "attempt to subtract with overflow"

/-- `a * b`, panicking where Rust's overflow check does. -/
def mul {α : Type} [Scalar α] (a b : α) : M α :=
  if Scalar.lo α ≤ val a * val b ∧ val a * val b ≤ Scalar.hi α then pure (a * b)
  else throw "attempt to multiply with overflow"

@[inherit_doc] infixl:65 " +? " => add
@[inherit_doc] infixl:65 " -? " => sub
@[inherit_doc] infixl:70 " *? " => mul

/-- `a as β`: wrapping, as Rust's integer casts are. -/
def cast {α : Type} (β : Type) [Scalar α] [Scalar β] (a : α) : β := Scalar.ofInt (val a)

theorem add_ok {α : Type} [Scalar α] (a b : α) (lo : Scalar.lo α ≤ val a + val b)
    (hi : val a + val b ≤ Scalar.hi α) : a +? b = pure (a + b) := by
  simp [add, lo, hi]

theorem sub_ok {α : Type} [Scalar α] (a b : α) (lo : Scalar.lo α ≤ val a - val b)
    (hi : val a - val b ≤ Scalar.hi α) : a -? b = pure (a - b) := by
  simp [sub, lo, hi]

theorem mul_ok {α : Type} [Scalar α] (a b : α) (lo : Scalar.lo α ≤ val a * val b)
    (hi : val a * val b ≤ Scalar.hi α) : a *? b = pure (a * b) := by
  simp [mul, lo, hi]

macro "unsigned_scalar" t:ident bits:num : command =>
  `(instance : Scalar $t where
      val x := (x.toNat : Int)
      lo := 0
      hi := 2 ^ $bits - 1
      ofInt i := $(Lean.mkIdent (t.getId ++ `ofNat)) (i % 2 ^ $bits).toNat
      val_add a b _ h := by
        have := $(Lean.mkIdent (t.getId ++ `toNat_add)) a b
        omega
      val_sub a b l _ := by
        have hle : b ≤ a := ($(Lean.mkIdent (t.getId ++ `le_iff_toNat_le))).mpr (by omega)
        have := $(Lean.mkIdent (t.getId ++ `toNat_sub_of_le)) a b hle
        omega
      val_mul a b _ h := by
        have := $(Lean.mkIdent (t.getId ++ `toNat_mul)) a b
        have : (a.toNat * b.toNat : Int) = ((a.toNat * b.toNat : Nat) : Int) := by push_cast; rfl
        omega)

macro "signed_scalar" t:ident bits:num : command =>
  `(instance : Scalar $t where
      val x := x.toInt
      lo := -2 ^ ($bits - 1)
      hi := 2 ^ ($bits - 1) - 1
      ofInt i := $(Lean.mkIdent (t.getId ++ `ofInt)) i
      val_add a b l h := by
        rw [$(Lean.mkIdent (t.getId ++ `toInt_add)):ident, Int.bmod_eq_of_le] <;> omega
      val_sub a b l h := by
        rw [$(Lean.mkIdent (t.getId ++ `toInt_sub)):ident, Int.bmod_eq_of_le] <;> omega
      val_mul a b l h := by
        rw [$(Lean.mkIdent (t.getId ++ `toInt_mul)):ident, Int.bmod_eq_of_le] <;> omega)

/-- `core::ops::ControlFlow`, which `?` matches on. -/
inductive ControlFlow (β γ : Type) where
  | Continue (x0 : γ)
  | Break (x0 : β)
deriving Repr, DecidableEq

instance {β γ : Type} [Inhabited γ] : Inhabited (ControlFlow β γ) := ⟨.Continue default⟩

/-- `Option`'s `?`: the value, or the `None` to return. -/
def branch {α : Type} : Option α → ControlFlow (Option Empty) α
  | some v => .Continue v
  | none => .Break none

/-- `checked_add` on an unsigned integer. -/
def checkedAdd {α : Type} [Scalar α] (a b : α) : Option α :=
  if val a + val b ≤ Scalar.hi α then some (a + b) else none

/-- `checked_sub` on an unsigned integer. -/
def checkedSub {α : Type} [Scalar α] (a b : α) : Option α :=
  if Scalar.lo α ≤ val a - val b then some (a - b) else none

unsigned_scalar UInt8 8
unsigned_scalar UInt16 16
unsigned_scalar UInt32 32
unsigned_scalar UInt64 64
signed_scalar Int8 8
signed_scalar Int16 16
signed_scalar Int32 32
signed_scalar Int64 64

end Rust

deriving instance DecidableEq for Except
