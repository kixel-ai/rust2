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

/-- `saturating_add`: the sum, clamped to the type's range. -/
def saturatingAdd {α : Type} [Scalar α] (a b : α) : α :=
  Scalar.ofInt (max (Scalar.lo α) (min (Scalar.hi α) (val a + val b)))

/-- `saturating_sub`: the difference, clamped to the type's range. -/
def saturatingSub {α : Type} [Scalar α] (a b : α) : α :=
  Scalar.ofInt (max (Scalar.lo α) (min (Scalar.hi α) (val a - val b)))

/-- `std::cmp::Reverse`: the value, ordered the other way. -/
structure Reverse (α : Type) where
  x0 : α
deriving Repr, DecidableEq, Inhabited

instance {α : Type} [Ord α] : Ord (Reverse α) where
  compare a b := compare b.x0 a.x0

/-- Tuples order lexicographically, as Rust's do. -/
instance {α β : Type} [Ord α] [Ord β] : Ord (α × β) := lexOrd

instance : Ord Unit where
  compare _ _ := .eq

/-- `Option::expect` and `unwrap`: the value, or a panic with `msg`. -/
def expect {α : Type} (o : Option α) (msg : String) : M α :=
  match o with
  | some x => pure x
  | none => throw msg

/-- `Option::is_some_and` with a closure that may panic. -/
def isSomeAndM {α : Type} (o : Option α) (f : α → M Bool) : M Bool :=
  match o with
  | some x => f x
  | none => pure false

/-- `Option::map` with a closure that may panic. -/
def mapM {α β : Type} (o : Option α) (f : α → M β) : M (Option β) :=
  match o with
  | some x => some <$> f x
  | none => pure none

/-- `&s[..e]`, panicking as Rust's range index does. -/
def sliceTo {α : Type} (s : Array α) (e : UInt64) : M (Array α) :=
  if e.toNat ≤ s.size then pure (s.extract 0 e.toNat)
  else throw s!"range end index {e} out of range for slice of length {s.size}"

/-- `&s[b..]`, panicking as Rust's range index does. -/
def sliceFrom {α : Type} (s : Array α) (b : UInt64) : M (Array α) :=
  if b.toNat ≤ s.size then pure (s.extract b.toNat s.size)
  else throw s!"range start index {b} out of range for slice of length {s.size}"

/-- `&s[b..e]`, panicking as Rust's range index does. -/
def slice {α : Type} (s : Array α) (b e : UInt64) : M (Array α) :=
  if b.toNat > s.size then throw s!"range start index {b} out of range for slice of length {s.size}"
  else if e.toNat > s.size then throw s!"range end index {e} out of range for slice of length {s.size}"
  else if b > e then throw s!"slice index starts at {b} but ends at {e}"
  else pure (s.extract b.toNat e.toNat)

/-! Iterators. A std iterator is the list of the items it yields, eagerly made: a closure
an adaptor runs is pure, or the list holds each item as the action that makes it
(`List (M α)`), so a panic happens only when a consumer reaches that item, as Rust's
laziness has it. -/

/-- `slice::windows`: each run of `n` neighbours, in order. -/
def windows {α : Type} (s : Array α) (n : UInt64) : M (List (Array α)) :=
  if n = 0 then throw "window size must be non-zero"
  else pure ((List.range (s.size + 1 - n.toNat)).map fun i => s.extract i (i + n.toNat))

/-- `Iterator::enumerate`. -/
def enumerate {α : Type} (l : List α) : List (UInt64 × α) :=
  (List.range l.length).zip l |>.map fun (i, x) => (i.toUInt64, x)

/-- Run each item's action, in order: `collect` of an iterator whose items may panic. -/
def run {α : Type} (l : List (Unit → M α)) : M (List α) :=
  l.mapM (· ())

/-- An item of an iterator whose items may panic, made only when a consumer reaches it. -/
def defer {α β : Type} (f : α → M β) (a : α) : Unit → M β := fun _ => f a

/-- `map` by `f`, which cannot panic, over a deferred item. -/
def deferMap {α β : Type} (f : α → β) (t : Unit → M α) : Unit → M β := fun _ => f <$> t ()

/-- `map` by `f`, which may panic, over a deferred item. -/
def deferBind {α β : Type} (f : α → M β) (t : Unit → M α) : Unit → M β := fun _ => t () >>= f

/-- An item that cannot panic, among deferred ones. -/
def ready {α : Type} (a : α) : Unit → M α := fun _ => pure a

/-- A consumer's step: make the deferred item, then apply `g` to it. -/
def force {α β : Type} (g : α → M β) (t : Unit → M α) : M β := t () >>= g

/-- `BTreeMap`: its entries in ascending key order, each key once. -/
structure BTreeMap (κ ν : Type) where
  entries : List (κ × ν)
deriving Repr, DecidableEq, Inhabited, Ord

namespace BTreeMap

/-- `BTreeMap::new`. -/
def empty {κ ν : Type} : BTreeMap κ ν := ⟨[]⟩

/-- `BTreeMap::get`. -/
def get {κ ν : Type} [Ord κ] (m : BTreeMap κ ν) (k : κ) : Option ν :=
  (m.entries.find? fun e => compare e.1 k == .eq).map (·.2)

/-- `BTreeMap::contains_key`. -/
def containsKey {κ ν : Type} [Ord κ] (m : BTreeMap κ ν) (k : κ) : Bool :=
  m.entries.any fun e => compare e.1 k == .eq

/-- `BTreeMap::len`. -/
def len {κ ν : Type} (m : BTreeMap κ ν) : UInt64 :=
  m.entries.length.toUInt64

end BTreeMap

/-- `BTreeSet`: its items in ascending order, each once. -/
structure BTreeSet (κ : Type) where
  items : List κ
deriving Repr, DecidableEq, Inhabited, Ord

namespace BTreeSet

/-- `BTreeSet::new`. -/
def empty {κ : Type} : BTreeSet κ := ⟨[]⟩

/-- `BTreeSet::contains`. -/
def contains {κ : Type} [Ord κ] (s : BTreeSet κ) (k : κ) : Bool :=
  s.items.any fun x => compare x k == .eq

/-- `BTreeSet::len`. -/
def len {κ : Type} (s : BTreeSet κ) : UInt64 :=
  s.items.length.toUInt64

end BTreeSet

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
