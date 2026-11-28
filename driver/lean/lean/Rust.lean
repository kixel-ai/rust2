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

/-- `β::try_from(a)` between integers: the value if `β` holds it, else `TryFromIntError`,
which holds nothing. -/
def tryFrom {α : Type} (β : Type) [Scalar α] [Scalar β] (a : α) : Except Unit β :=
  if Scalar.lo β ≤ val a ∧ val a ≤ Scalar.hi β then .ok (Scalar.ofInt (val a)) else .error ()

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

/-- `u128`, as Lean has no 128-bit integer: its bits. Arithmetic wraps, as `+` on Lean's
fixed-width integers does; `+?` and the like check it as Rust does. -/
structure UInt128 where
  toBitVec : BitVec 128
deriving DecidableEq, Inhabited

namespace UInt128
def ofNat (n : Nat) : UInt128 := ⟨BitVec.ofNat 128 n⟩
def toNat (x : UInt128) : Nat := x.toBitVec.toNat
instance {n : Nat} : OfNat UInt128 n := ⟨ofNat n⟩
instance : Add UInt128 := ⟨fun a b => ⟨a.toBitVec + b.toBitVec⟩⟩
instance : Sub UInt128 := ⟨fun a b => ⟨a.toBitVec - b.toBitVec⟩⟩
instance : Mul UInt128 := ⟨fun a b => ⟨a.toBitVec * b.toBitVec⟩⟩
instance : Div UInt128 := ⟨fun a b => ⟨a.toBitVec / b.toBitVec⟩⟩
instance : Mod UInt128 := ⟨fun a b => ⟨a.toBitVec % b.toBitVec⟩⟩
instance : AndOp UInt128 := ⟨fun a b => ⟨a.toBitVec &&& b.toBitVec⟩⟩
instance : OrOp UInt128 := ⟨fun a b => ⟨a.toBitVec ||| b.toBitVec⟩⟩
instance : XorOp UInt128 := ⟨fun a b => ⟨a.toBitVec ^^^ b.toBitVec⟩⟩
instance : Complement UInt128 := ⟨fun a => ⟨~~~a.toBitVec⟩⟩
instance : HShiftLeft UInt128 UInt128 UInt128 := ⟨fun a b => ⟨a.toBitVec <<< (b.toNat % 128)⟩⟩
instance : HShiftRight UInt128 UInt128 UInt128 := ⟨fun a b => ⟨a.toBitVec >>> (b.toNat % 128)⟩⟩
instance : LT UInt128 := ⟨fun a b => a.toNat < b.toNat⟩
instance : LE UInt128 := ⟨fun a b => a.toNat ≤ b.toNat⟩
instance (a b : UInt128) : Decidable (a < b) := inferInstanceAs (Decidable (a.toNat < b.toNat))
instance (a b : UInt128) : Decidable (a ≤ b) := inferInstanceAs (Decidable (a.toNat ≤ b.toNat))
instance : Ord UInt128 := ⟨fun a b => compare a.toNat b.toNat⟩
instance : Min UInt128 := minOfLe
instance : Max UInt128 := maxOfLe
instance : Repr UInt128 := ⟨fun x _ => repr x.toNat⟩
instance : ToString UInt128 := ⟨fun x => toString x.toNat⟩
end UInt128

/-- `i128`: its bits, as two's complement. `/` and `%` truncate, as Rust's do. -/
structure Int128 where
  toBitVec : BitVec 128
deriving DecidableEq, Inhabited

namespace Int128
def ofInt (i : Int) : Int128 := ⟨BitVec.ofInt 128 i⟩
def toInt (x : Int128) : Int := x.toBitVec.toInt
instance {n : Nat} : OfNat Int128 n := ⟨⟨BitVec.ofNat 128 n⟩⟩
instance : Neg Int128 := ⟨fun a => ⟨-a.toBitVec⟩⟩
instance : Add Int128 := ⟨fun a b => ⟨a.toBitVec + b.toBitVec⟩⟩
instance : Sub Int128 := ⟨fun a b => ⟨a.toBitVec - b.toBitVec⟩⟩
instance : Mul Int128 := ⟨fun a b => ⟨a.toBitVec * b.toBitVec⟩⟩
instance : Div Int128 := ⟨fun a b => ⟨a.toBitVec.sdiv b.toBitVec⟩⟩
instance : Mod Int128 := ⟨fun a b => ⟨a.toBitVec.srem b.toBitVec⟩⟩
instance : AndOp Int128 := ⟨fun a b => ⟨a.toBitVec &&& b.toBitVec⟩⟩
instance : OrOp Int128 := ⟨fun a b => ⟨a.toBitVec ||| b.toBitVec⟩⟩
instance : XorOp Int128 := ⟨fun a b => ⟨a.toBitVec ^^^ b.toBitVec⟩⟩
instance : Complement Int128 := ⟨fun a => ⟨~~~a.toBitVec⟩⟩
instance : HShiftLeft Int128 Int128 Int128 := ⟨fun a b => ⟨a.toBitVec <<< (b.toBitVec.toNat % 128)⟩⟩
instance : HShiftRight Int128 Int128 Int128 := ⟨fun a b => ⟨a.toBitVec.sshiftRight (b.toBitVec.toNat % 128)⟩⟩
instance : LT Int128 := ⟨fun a b => a.toInt < b.toInt⟩
instance : LE Int128 := ⟨fun a b => a.toInt ≤ b.toInt⟩
instance (a b : Int128) : Decidable (a < b) := inferInstanceAs (Decidable (a.toInt < b.toInt))
instance (a b : Int128) : Decidable (a ≤ b) := inferInstanceAs (Decidable (a.toInt ≤ b.toInt))
instance : Ord Int128 := ⟨fun a b => compare a.toInt b.toInt⟩
instance : Min Int128 := minOfLe
instance : Max Int128 := maxOfLe
instance : Repr Int128 := ⟨fun x _ => repr x.toInt⟩
instance : ToString Int128 := ⟨fun x => toString x.toInt⟩
end Int128

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

/-- `Ord::max` on an order Lean derives: `b` unless `a` is greater, as Rust picks. -/
def ordMax {α : Type} [Ord α] (a b : α) : α := if compare a b == .gt then a else b

/-- `Ord::min` on an order Lean derives: `a` unless it is greater, as Rust picks. -/
def ordMin {α : Type} [Ord α] (a b : α) : α := if compare a b == .gt then b else a

/-- `std::time::Duration`: whole seconds, and the nanoseconds past them, always below a
second. Ordered as Rust's derive orders it, seconds first. -/
structure Duration where
  secs : UInt64
  nanos : UInt32
deriving Repr, DecidableEq, Inhabited, Ord

namespace Duration

/-- `Duration::ZERO`. -/
def ZERO : Duration := ⟨0, 0⟩

/-- `Duration::MAX`. -/
def MAX : Duration := ⟨0xFFFFFFFFFFFFFFFF, 999999999⟩

/-- The whole duration in nanoseconds. -/
def toNanos (d : Duration) : Nat := d.secs.toNat * 1000000000 + d.nanos.toNat

/-- The duration `n` nanoseconds long, or `none` past `Duration::MAX`. -/
def ofNanos? (n : Nat) : Option Duration :=
  if n / 1000000000 < 2 ^ 64 then some ⟨(n / 1000000000).toUInt64, (n % 1000000000).toUInt32⟩
  else none

/-- `Duration::from_secs`. -/
def fromSecs (s : UInt64) : Duration := ⟨s, 0⟩

/-- `Duration::from_millis`. -/
def fromMillis (ms : UInt64) : Duration := ⟨ms / 1000, (ms % 1000).toUInt32 * 1000000⟩

/-- `Duration::from_nanos`. -/
def fromNanos (ns : UInt64) : Duration := ⟨ns / 1000000000, (ns % 1000000000).toUInt32⟩

/-- `Duration::checked_add`. -/
def checkedAdd (a b : Duration) : Option Duration := ofNanos? (a.toNanos + b.toNanos)

/-- `Duration::checked_sub`. -/
def checkedSub (a b : Duration) : Option Duration :=
  if b.toNanos ≤ a.toNanos then ofNanos? (a.toNanos - b.toNanos) else none

/-- `Duration::checked_mul`. -/
def checkedMul (a : Duration) (n : UInt32) : Option Duration := ofNanos? (a.toNanos * n.toNat)

/-- `Duration::saturating_add`. -/
def saturatingAdd (a b : Duration) : Duration := (checkedAdd a b).getD MAX

/-- `Duration::saturating_sub`. -/
def saturatingSub (a b : Duration) : Duration := (checkedSub a b).getD ZERO

/-- `Duration::saturating_mul`. -/
def saturatingMul (a : Duration) (n : UInt32) : Duration := (checkedMul a n).getD MAX

/-- `Duration::as_nanos`, which never overflows a `u128`. -/
def asNanos (d : Duration) : UInt128 := UInt128.ofNat d.toNanos

/-- `Duration::as_micros`. -/
def asMicros (d : Duration) : UInt128 := UInt128.ofNat (d.toNanos / 1000)

/-- `Duration::as_millis`. -/
def asMillis (d : Duration) : UInt128 := UInt128.ofNat (d.toNanos / 1000000)

end Duration

/-- `VecDeque`: its items, front first. -/
structure VecDeque (α : Type) where
  items : List α
deriving Repr, DecidableEq, Inhabited, Ord

namespace VecDeque

/-- `VecDeque::new`. -/
def empty {α : Type} : VecDeque α := ⟨[]⟩

/-- `VecDeque::push_back`. -/
def pushBack {α : Type} (d : VecDeque α) (x : α) : VecDeque α := ⟨d.items ++ [x]⟩

/-- `VecDeque::pop_front`: the queue, and the item it took. -/
def popFront {α : Type} (d : VecDeque α) : VecDeque α × Option α := (⟨d.items.drop 1⟩, d.items.head?)

end VecDeque

/-- `Option::expect` and `unwrap`: the value, or a panic with `msg`. -/
def expect {α : Type} (o : Option α) (msg : String) : M α :=
  match o with
  | some x => pure x
  | none => throw msg

/-- `Option::ok_or`. -/
def okOr {α ε : Type} (o : Option α) (e : ε) : Except ε α :=
  match o with
  | some x => .ok x
  | none => .error e

/-- `Result`'s `?`: the value, or the error to return. -/
def branchExcept {α ε : Type} : Except ε α → ControlFlow (Except ε Empty) α
  | .ok v => .Continue v
  | .error e => .Break (.error e)

/-- What `Result`'s `?` returns with the error it took. -/
def fromResidualExcept {α ε : Type} : Except ε Empty → Except ε α
  | .error e => .error e
  | .ok v => nomatch v

/-- `Option::unwrap_or_default` with a `Default` that may panic, made only for `none`. -/
def getDM {α : Type} (o : Option α) (f : Unit → M α) : M α :=
  match o with
  | some x => pure x
  | none => f ()

/-- `Option::get_or_insert_default` with a `Default` that may panic: the `Option` after. -/
def getOrInsertWithM {α : Type} (o : Option α) (f : Unit → M α) : M (Option α) :=
  some <$> getDM o f

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

/-! Strings. A `str` is a Lean `String`, which holds UTF-8 exactly, and Rust's byte indices
are its byte positions. -/

/-- `str::len`: its length in bytes. -/
def strLen (s : String) : UInt64 := s.utf8ByteSize.toUInt64

/-- `str::as_bytes`. -/
def asBytes (s : String) : Array UInt8 := s.toUTF8.data

/-- Whether byte `i` of `s` begins a character or ends `s`, as `str::is_char_boundary`. -/
def charBoundary (s : String) (i : Nat) : Bool :=
  i == 0 || i == s.utf8ByteSize || (i < s.utf8ByteSize && s.toUTF8.get! i &&& 0xC0 != 0x80)

/-- The non-ASCII characters whose `Debug` is a `\u{..}` escape, as half-open ranges of code
points: the grapheme extenders and those Rust does not print, as the toolchain's `core` has
them. `lean-fixture` checks them against native Rust. -/
def debugEscaped : Array (Nat × Nat) := #[
  (0x80, 0xa1), (0xad, 0xae), (0x300, 0x370), (0x378, 0x37a), (0x380, 0x384), (0x38b, 0x38c), (0x38d, 0x38e), (0x3a2, 0x3a3),
  (0x483, 0x48a), (0x530, 0x531), (0x557, 0x558), (0x590, 0x5be), (0x5bf, 0x5c0), (0x5c1, 0x5c3), (0x5c4, 0x5c6), (0x5c7, 0x5d0),
  (0x5eb, 0x5ef), (0x5f5, 0x606), (0x610, 0x61b), (0x61c, 0x61d), (0x64b, 0x660), (0x670, 0x671), (0x6d6, 0x6de), (0x6df, 0x6e5),
  (0x6e7, 0x6e9), (0x6ea, 0x6ee), (0x70e, 0x710), (0x711, 0x712), (0x730, 0x74d), (0x7a6, 0x7b1), (0x7b2, 0x7c0), (0x7eb, 0x7f4),
  (0x7fb, 0x7fe), (0x816, 0x81a), (0x81b, 0x824), (0x825, 0x828), (0x829, 0x830), (0x83f, 0x840), (0x859, 0x85e), (0x85f, 0x860),
  (0x86b, 0x870), (0x890, 0x8a0), (0x8ca, 0x903), (0x93a, 0x93b), (0x93c, 0x93d), (0x941, 0x949), (0x94d, 0x94e), (0x951, 0x958),
  (0x962, 0x964), (0x981, 0x982), (0x984, 0x985), (0x98d, 0x98f), (0x991, 0x993), (0x9a9, 0x9aa), (0x9b1, 0x9b2), (0x9b3, 0x9b6),
  (0x9ba, 0x9bd), (0x9be, 0x9bf), (0x9c1, 0x9c7), (0x9c9, 0x9cb), (0x9cd, 0x9ce), (0x9cf, 0x9dc), (0x9de, 0x9df), (0x9e2, 0x9e6),
  (0x9fe, 0xa03), (0xa04, 0xa05), (0xa0b, 0xa0f), (0xa11, 0xa13), (0xa29, 0xa2a), (0xa31, 0xa32), (0xa34, 0xa35), (0xa37, 0xa38),
  (0xa3a, 0xa3e), (0xa41, 0xa59), (0xa5d, 0xa5e), (0xa5f, 0xa66), (0xa70, 0xa72), (0xa75, 0xa76), (0xa77, 0xa83), (0xa84, 0xa85),
  (0xa8e, 0xa8f), (0xa92, 0xa93), (0xaa9, 0xaaa), (0xab1, 0xab2), (0xab4, 0xab5), (0xaba, 0xabd), (0xac1, 0xac9), (0xaca, 0xacb),
  (0xacd, 0xad0), (0xad1, 0xae0), (0xae2, 0xae6), (0xaf2, 0xaf9), (0xafa, 0xb02), (0xb04, 0xb05), (0xb0d, 0xb0f), (0xb11, 0xb13),
  (0xb29, 0xb2a), (0xb31, 0xb32), (0xb34, 0xb35), (0xb3a, 0xb3d), (0xb3e, 0xb40), (0xb41, 0xb47), (0xb49, 0xb4b), (0xb4d, 0xb5c),
  (0xb5e, 0xb5f), (0xb62, 0xb66), (0xb78, 0xb83), (0xb84, 0xb85), (0xb8b, 0xb8e), (0xb91, 0xb92), (0xb96, 0xb99), (0xb9b, 0xb9c),
  (0xb9d, 0xb9e), (0xba0, 0xba3), (0xba5, 0xba8), (0xbab, 0xbae), (0xbba, 0xbbf), (0xbc0, 0xbc1), (0xbc3, 0xbc6), (0xbc9, 0xbca),
  (0xbcd, 0xbd0), (0xbd1, 0xbe6), (0xbfb, 0xc01), (0xc04, 0xc05), (0xc0d, 0xc0e), (0xc11, 0xc12), (0xc29, 0xc2a), (0xc3a, 0xc3d),
  (0xc3e, 0xc41), (0xc45, 0xc58), (0xc5b, 0xc5c), (0xc5e, 0xc60), (0xc62, 0xc66), (0xc70, 0xc77), (0xc81, 0xc82), (0xc8d, 0xc8e),
  (0xc91, 0xc92), (0xca9, 0xcaa), (0xcb4, 0xcb5), (0xcba, 0xcbd), (0xcbf, 0xcc1), (0xcc2, 0xcc3), (0xcc5, 0xcdc), (0xcdf, 0xce0),
  (0xce2, 0xce6), (0xcf0, 0xcf1), (0xcf4, 0xd02), (0xd0d, 0xd0e), (0xd11, 0xd12), (0xd3b, 0xd3d), (0xd3e, 0xd3f), (0xd41, 0xd46),
  (0xd49, 0xd4a), (0xd4d, 0xd4e), (0xd50, 0xd54), (0xd57, 0xd58), (0xd62, 0xd66), (0xd80, 0xd82), (0xd84, 0xd85), (0xd97, 0xd9a),
  (0xdb2, 0xdb3), (0xdbc, 0xdbd), (0xdbe, 0xdc0), (0xdc7, 0xdd0), (0xdd2, 0xdd8), (0xddf, 0xde6), (0xdf0, 0xdf2), (0xdf5, 0xe01),
  (0xe31, 0xe32), (0xe34, 0xe3f), (0xe47, 0xe4f), (0xe5c, 0xe81), (0xe83, 0xe84), (0xe85, 0xe86), (0xe8b, 0xe8c), (0xea4, 0xea5),
  (0xea6, 0xea7), (0xeb1, 0xeb2), (0xeb4, 0xebd), (0xebe, 0xec0), (0xec5, 0xec6), (0xec7, 0xed0), (0xeda, 0xedc), (0xee0, 0xf00),
  (0xf18, 0xf1a), (0xf35, 0xf36), (0xf37, 0xf38), (0xf39, 0xf3a), (0xf48, 0xf49), (0xf6d, 0xf7f), (0xf80, 0xf85), (0xf86, 0xf88),
  (0xf8d, 0xfbe), (0xfc6, 0xfc7), (0xfcd, 0xfce), (0xfdb, 0x1000), (0x102d, 0x1031), (0x1032, 0x1038), (0x1039, 0x103b), (0x103d, 0x103f),
  (0x1058, 0x105a), (0x105e, 0x1061), (0x1071, 0x1075), (0x1082, 0x1083), (0x1085, 0x1087), (0x108d, 0x108e), (0x109d, 0x109e), (0x10c6, 0x10c7),
  (0x10c8, 0x10cd), (0x10ce, 0x10d0), (0x115f, 0x1161), (0x1249, 0x124a), (0x124e, 0x1250), (0x1257, 0x1258), (0x1259, 0x125a), (0x125e, 0x1260),
  (0x1289, 0x128a), (0x128e, 0x1290), (0x12b1, 0x12b2), (0x12b6, 0x12b8), (0x12bf, 0x12c0), (0x12c1, 0x12c2), (0x12c6, 0x12c8), (0x12d7, 0x12d8),
  (0x1311, 0x1312), (0x1316, 0x1318), (0x135b, 0x1360), (0x137d, 0x1380), (0x139a, 0x13a0), (0x13f6, 0x13f8), (0x13fe, 0x1400), (0x1680, 0x1681),
  (0x169d, 0x16a0), (0x16f9, 0x1700), (0x1712, 0x171f), (0x1732, 0x1735), (0x1737, 0x1740), (0x1752, 0x1760), (0x176d, 0x176e), (0x1771, 0x1780),
  (0x17b4, 0x17b6), (0x17b7, 0x17be), (0x17c6, 0x17c7), (0x17c9, 0x17d4), (0x17dd, 0x17e0), (0x17ea, 0x17f0), (0x17fa, 0x1800), (0x180b, 0x1810),
  (0x181a, 0x1820), (0x1879, 0x1880), (0x1885, 0x1887), (0x18a9, 0x18aa), (0x18ab, 0x18b0), (0x18f6, 0x1900), (0x191f, 0x1923), (0x1927, 0x1929),
  (0x192c, 0x1930), (0x1932, 0x1933), (0x1939, 0x1940), (0x1941, 0x1944), (0x196e, 0x1970), (0x1975, 0x1980), (0x19ac, 0x19b0), (0x19ca, 0x19d0),
  (0x19db, 0x19de), (0x1a17, 0x1a19), (0x1a1b, 0x1a1e), (0x1a56, 0x1a57), (0x1a58, 0x1a61), (0x1a62, 0x1a63), (0x1a65, 0x1a6d), (0x1a73, 0x1a80),
  (0x1a8a, 0x1a90), (0x1a9a, 0x1aa0), (0x1aae, 0x1b04), (0x1b34, 0x1b3e), (0x1b42, 0x1b45), (0x1b4d, 0x1b4e), (0x1b6b, 0x1b74), (0x1b80, 0x1b82),
  (0x1ba2, 0x1ba6), (0x1ba8, 0x1bae), (0x1be6, 0x1be7), (0x1be8, 0x1bea), (0x1bed, 0x1bee), (0x1bef, 0x1bfc), (0x1c2c, 0x1c34), (0x1c36, 0x1c3b),
  (0x1c4a, 0x1c4d), (0x1c8b, 0x1c90), (0x1cbb, 0x1cbd), (0x1cc8, 0x1cd3), (0x1cd4, 0x1ce1), (0x1ce2, 0x1ce9), (0x1ced, 0x1cee), (0x1cf4, 0x1cf5),
  (0x1cf8, 0x1cfa), (0x1cfb, 0x1d00), (0x1dc0, 0x1e00), (0x1f16, 0x1f18), (0x1f1e, 0x1f20), (0x1f46, 0x1f48), (0x1f4e, 0x1f50), (0x1f58, 0x1f59),
  (0x1f5a, 0x1f5b), (0x1f5c, 0x1f5d), (0x1f5e, 0x1f5f), (0x1f7e, 0x1f80), (0x1fb5, 0x1fb6), (0x1fc5, 0x1fc6), (0x1fd4, 0x1fd6), (0x1fdc, 0x1fdd),
  (0x1ff0, 0x1ff2), (0x1ff5, 0x1ff6), (0x1fff, 0x2010), (0x2028, 0x2030), (0x205f, 0x2070), (0x2072, 0x2074), (0x20c5, 0x2100), (0x218c, 0x2190),
  (0x242a, 0x2440), (0x244b, 0x2460), (0x2b74, 0x2b76), (0x2cef, 0x2cf2), (0x2cf4, 0x2cf9), (0x2d26, 0x2d27), (0x2d28, 0x2d2d), (0x2d2e, 0x2d30),
  (0x2d68, 0x2d6f), (0x2d71, 0x2d80), (0x2d97, 0x2da0), (0x2da7, 0x2da8), (0x2daf, 0x2db0), (0x2db7, 0x2db8), (0x2dbf, 0x2dc0), (0x2dc7, 0x2dc8),
  (0x2dcf, 0x2dd0), (0x2dd7, 0x2dd8), (0x2ddf, 0x2e00), (0x2e5e, 0x2e60), (0x2e64, 0x2e80), (0x2e9a, 0x2e9b), (0x2ef4, 0x2f00), (0x2fd6, 0x2ff0),
  (0x3000, 0x3001), (0x302a, 0x3030), (0x3040, 0x3041), (0x3097, 0x309b), (0x3100, 0x3105), (0x3130, 0x3131), (0x3164, 0x3165), (0x318f, 0x3190),
  (0x31e6, 0x31ef), (0x321f, 0x3220), (0xa48d, 0xa490), (0xa4c7, 0xa4d0), (0xa62c, 0xa640), (0xa66f, 0xa673), (0xa674, 0xa67e), (0xa69e, 0xa6a0),
  (0xa6f0, 0xa6f2), (0xa6f8, 0xa700), (0xa7de, 0xa7e2), (0xa7e3, 0xa7f1), (0xa802, 0xa803), (0xa806, 0xa807), (0xa80b, 0xa80c), (0xa825, 0xa827),
  (0xa82c, 0xa830), (0xa83a, 0xa840), (0xa878, 0xa880), (0xa8c4, 0xa8ce), (0xa8da, 0xa8f2), (0xa8ff, 0xa900), (0xa926, 0xa92e), (0xa947, 0xa952),
  (0xa953, 0xa95f), (0xa97d, 0xa983), (0xa9b3, 0xa9b4), (0xa9b6, 0xa9ba), (0xa9bc, 0xa9be), (0xa9c0, 0xa9c1), (0xa9ce, 0xa9cf), (0xa9da, 0xa9de),
  (0xa9e5, 0xa9e6), (0xa9ff, 0xaa00), (0xaa29, 0xaa2f), (0xaa31, 0xaa33), (0xaa35, 0xaa40), (0xaa43, 0xaa44), (0xaa4c, 0xaa4d), (0xaa4e, 0xaa50),
  (0xaa5a, 0xaa5c), (0xaa7c, 0xaa7d), (0xaab0, 0xaab1), (0xaab2, 0xaab5), (0xaab7, 0xaab9), (0xaabe, 0xaac0), (0xaac1, 0xaac2), (0xaac3, 0xaadb),
  (0xaaec, 0xaaee), (0xaaf6, 0xab01), (0xab07, 0xab09), (0xab0f, 0xab11), (0xab17, 0xab20), (0xab27, 0xab28), (0xab2f, 0xab30), (0xab6e, 0xab70),
  (0xabe5, 0xabe6), (0xabe8, 0xabe9), (0xabed, 0xabf0), (0xabfa, 0xac00), (0xd7a4, 0xd7b0), (0xd7c7, 0xd7cb), (0xd7fc, 0xf900), (0xfa6e, 0xfa70),
  (0xfada, 0xfb00), (0xfb07, 0xfb13), (0xfb18, 0xfb1d), (0xfb1e, 0xfb1f), (0xfb37, 0xfb38), (0xfb3d, 0xfb3e), (0xfb3f, 0xfb40), (0xfb42, 0xfb43),
  (0xfb45, 0xfb46), (0xfdd0, 0xfdf0), (0xfe00, 0xfe10), (0xfe1a, 0xfe30), (0xfe53, 0xfe54), (0xfe67, 0xfe68), (0xfe6c, 0xfe70), (0xfe75, 0xfe76),
  (0xfefd, 0xff01), (0xffa0, 0xffa1), (0xffbf, 0xffc2), (0xffc8, 0xffca), (0xffd0, 0xffd2), (0xffd8, 0xffda), (0xffdd, 0xffe0), (0xffe7, 0xffe8),
  (0xffef, 0xfffc), (0xfffe, 0x10000), (0x1000c, 0x1000d), (0x10027, 0x10028), (0x1003b, 0x1003c), (0x1003e, 0x1003f), (0x1004e, 0x10050), (0x1005e, 0x10080),
  (0x100fb, 0x10100), (0x10103, 0x10107), (0x10134, 0x10137), (0x1018f, 0x10190), (0x1019d, 0x101a0), (0x101a1, 0x101d0), (0x101fd, 0x10280), (0x1029d, 0x102a0),
  (0x102d1, 0x102e1), (0x102fc, 0x10300), (0x10324, 0x1032d), (0x1034b, 0x10350), (0x10376, 0x10380), (0x1039e, 0x1039f), (0x103c4, 0x103c8), (0x103d6, 0x10400),
  (0x1049e, 0x104a0), (0x104aa, 0x104b0), (0x104d4, 0x104d8), (0x104fc, 0x10500), (0x10528, 0x10530), (0x10564, 0x1056f), (0x1057b, 0x1057c), (0x1058b, 0x1058c),
  (0x10593, 0x10594), (0x10596, 0x10597), (0x105a2, 0x105a3), (0x105b2, 0x105b3), (0x105ba, 0x105bb), (0x105bd, 0x105c0), (0x105f4, 0x10600), (0x10737, 0x10740),
  (0x10756, 0x10760), (0x10768, 0x10780), (0x10786, 0x10787), (0x107b1, 0x107b2), (0x107c0, 0x10800), (0x10806, 0x10808), (0x10809, 0x1080a), (0x10836, 0x10837),
  (0x10839, 0x1083c), (0x1083d, 0x1083f), (0x10856, 0x10857), (0x1089f, 0x108a7), (0x108b0, 0x108e0), (0x108f3, 0x108f4), (0x108f6, 0x108fb), (0x1091c, 0x1091f),
  (0x1093a, 0x1093f), (0x1095a, 0x10980), (0x109b8, 0x109bc), (0x109d0, 0x109d2), (0x10a01, 0x10a10), (0x10a14, 0x10a15), (0x10a18, 0x10a19), (0x10a36, 0x10a40),
  (0x10a49, 0x10a50), (0x10a59, 0x10a60), (0x10aa0, 0x10ac0), (0x10ae5, 0x10aeb), (0x10af7, 0x10b00), (0x10b36, 0x10b39), (0x10b56, 0x10b58), (0x10b73, 0x10b78),
  (0x10b92, 0x10b99), (0x10b9d, 0x10ba9), (0x10bb0, 0x10c00), (0x10c49, 0x10c80), (0x10cb3, 0x10cc0), (0x10cf3, 0x10cfa), (0x10d24, 0x10d30), (0x10d3a, 0x10d40),
  (0x10d66, 0x10d6e), (0x10d86, 0x10d8e), (0x10d90, 0x10e60), (0x10e7f, 0x10e80), (0x10eaa, 0x10ead), (0x10eae, 0x10eb0), (0x10eb2, 0x10ec2), (0x10ec8, 0x10ec9),
  (0x10ecb, 0x10ed0), (0x10eef, 0x10f00), (0x10f28, 0x10f30), (0x10f46, 0x10f51), (0x10f5a, 0x10f70), (0x10f82, 0x10f86), (0x10f8a, 0x10fb0), (0x10fcc, 0x10fe0),
  (0x10ff7, 0x11000), (0x11001, 0x11002), (0x11038, 0x11047), (0x1104e, 0x11052), (0x11070, 0x11071), (0x11073, 0x11075), (0x11076, 0x11082), (0x110b3, 0x110b7),
  (0x110b9, 0x110bb), (0x110bd, 0x110be), (0x110c2, 0x110d0), (0x110e9, 0x110f0), (0x110fa, 0x11103), (0x11127, 0x1112c), (0x1112d, 0x11136), (0x11148, 0x11150),
  (0x11173, 0x11174), (0x11177, 0x11182), (0x111b6, 0x111bf), (0x111c0, 0x111c1), (0x111c9, 0x111cd), (0x111cf, 0x111d0), (0x111e0, 0x111e1), (0x111f5, 0x11200),
  (0x11212, 0x11213), (0x1122f, 0x11232), (0x11234, 0x11238), (0x1123e, 0x1123f), (0x11241, 0x11280), (0x11287, 0x11288), (0x11289, 0x1128a), (0x1128e, 0x1128f),
  (0x1129e, 0x1129f), (0x112aa, 0x112b0), (0x112df, 0x112e0), (0x112e3, 0x112f0), (0x112fa, 0x11302), (0x11304, 0x11305), (0x1130d, 0x1130f), (0x11311, 0x11313),
  (0x11329, 0x1132a), (0x11331, 0x11332), (0x11334, 0x11335), (0x1133a, 0x1133d), (0x1133e, 0x1133f), (0x11340, 0x11341), (0x11345, 0x11347), (0x11349, 0x1134b),
  (0x1134d, 0x11350), (0x11351, 0x1135d), (0x11364, 0x11380), (0x1138a, 0x1138b), (0x1138c, 0x1138e), (0x1138f, 0x11390), (0x113b6, 0x113b7), (0x113b8, 0x113b9),
  (0x113bb, 0x113ca), (0x113cb, 0x113cc), (0x113ce, 0x113d1), (0x113d2, 0x113d3), (0x113d6, 0x113d7), (0x113d9, 0x11400), (0x11438, 0x11440), (0x11442, 0x11445),
  (0x11446, 0x11447), (0x1145c, 0x1145d), (0x1145e, 0x1145f), (0x11462, 0x11480), (0x114b0, 0x114b1), (0x114b3, 0x114b9), (0x114ba, 0x114bb), (0x114bd, 0x114be),
  (0x114bf, 0x114c1), (0x114c2, 0x114c4), (0x114c8, 0x114d0), (0x114da, 0x11580), (0x115af, 0x115b0), (0x115b2, 0x115b8), (0x115bc, 0x115be), (0x115bf, 0x115c1),
  (0x115dc, 0x11600), (0x11633, 0x1163b), (0x1163d, 0x1163e), (0x1163f, 0x11641), (0x11645, 0x11650), (0x1165a, 0x11660), (0x1166d, 0x11680), (0x116ab, 0x116ac),
  (0x116ad, 0x116ae), (0x116b0, 0x116b8), (0x116ba, 0x116c0), (0x116ca, 0x116d0), (0x116e4, 0x11700), (0x1171b, 0x1171e), (0x1171f, 0x11720), (0x11722, 0x11726),
  (0x11727, 0x11730), (0x11747, 0x11800), (0x1182f, 0x11838), (0x11839, 0x1183b), (0x1183c, 0x118a0), (0x118f3, 0x118ff), (0x11907, 0x11909), (0x1190a, 0x1190c),
  (0x11914, 0x11915), (0x11917, 0x11918), (0x11930, 0x11931), (0x11936, 0x11937), (0x11939, 0x1193f), (0x11943, 0x11944), (0x11947, 0x11950), (0x1195a, 0x119a0),
  (0x119a8, 0x119aa), (0x119d4, 0x119dc), (0x119e0, 0x119e1), (0x119e5, 0x11a00), (0x11a01, 0x11a0b), (0x11a33, 0x11a39), (0x11a3b, 0x11a3f), (0x11a47, 0x11a50),
  (0x11a51, 0x11a57), (0x11a59, 0x11a5c), (0x11a8a, 0x11a97), (0x11a98, 0x11a9a), (0x11aa3, 0x11ab0), (0x11af9, 0x11b00), (0x11b0b, 0x11b61), (0x11b62, 0x11b65),
  (0x11b66, 0x11b67), (0x11b68, 0x11bc0), (0x11be2, 0x11bf0), (0x11bfa, 0x11c00), (0x11c09, 0x11c0a), (0x11c30, 0x11c3e), (0x11c3f, 0x11c40), (0x11c46, 0x11c50),
  (0x11c6d, 0x11c70), (0x11c90, 0x11ca9), (0x11caa, 0x11cb1), (0x11cb2, 0x11cb4), (0x11cb5, 0x11d00), (0x11d07, 0x11d08), (0x11d0a, 0x11d0b), (0x11d31, 0x11d46),
  (0x11d47, 0x11d50), (0x11d5a, 0x11d60), (0x11d66, 0x11d67), (0x11d69, 0x11d6a), (0x11d8f, 0x11d93), (0x11d95, 0x11d96), (0x11d97, 0x11d98), (0x11d99, 0x11da0),
  (0x11daa, 0x11db0), (0x11ddc, 0x11de0), (0x11dea, 0x11df1), (0x11df2, 0x11ee0), (0x11ef3, 0x11ef5), (0x11ef9, 0x11f02), (0x11f11, 0x11f12), (0x11f36, 0x11f3e),
  (0x11f40, 0x11f43), (0x11f5a, 0x11fb0), (0x11fb1, 0x11fc0), (0x11ff2, 0x11fff), (0x1239a, 0x12400), (0x12544, 0x12550), (0x12687, 0x12f90), (0x12ff3, 0x13000),
  (0x13430, 0x13441), (0x13447, 0x13460), (0x143fb, 0x14400), (0x14647, 0x16100), (0x1611e, 0x1612a), (0x1612d, 0x16130), (0x1613a, 0x16800), (0x16a39, 0x16a40),
  (0x16a5f, 0x16a60), (0x16a6a, 0x16a6e), (0x16abf, 0x16ac0), (0x16aca, 0x16ad0), (0x16aee, 0x16af5), (0x16af6, 0x16b00), (0x16b30, 0x16b37), (0x16b46, 0x16b50),
  (0x16b5a, 0x16b5b), (0x16b62, 0x16b63), (0x16b78, 0x16b7d), (0x16b90, 0x16d40), (0x16d7a, 0x16e40), (0x16e9b, 0x16ea0), (0x16eb9, 0x16ebb), (0x16ed4, 0x16f00),
  (0x16f4b, 0x16f50), (0x16f88, 0x16f93), (0x16fa0, 0x16fe0), (0x16fe4, 0x16ff2), (0x16ff7, 0x17000), (0x18cdb, 0x18cff), (0x18d21, 0x18d80), (0x18df3, 0x18e00),
  (0x19192, 0x191a0), (0x191d3, 0x1aff0), (0x1aff4, 0x1aff5), (0x1affc, 0x1affd), (0x1afff, 0x1b000), (0x1b129, 0x1b132), (0x1b133, 0x1b150), (0x1b153, 0x1b155),
  (0x1b156, 0x1b164), (0x1b169, 0x1b170), (0x1b2fc, 0x1bc00), (0x1bc6b, 0x1bc70), (0x1bc7d, 0x1bc80), (0x1bc89, 0x1bc90), (0x1bc9a, 0x1bc9c), (0x1bc9d, 0x1bc9f),
  (0x1bca0, 0x1cc00), (0x1ccfd, 0x1cd00), (0x1ceb4, 0x1ceba), (0x1ced1, 0x1ced2), (0x1ced5, 0x1cedd), (0x1cefe, 0x1cf50), (0x1cfc4, 0x1d000), (0x1d0f6, 0x1d100),
  (0x1d127, 0x1d129), (0x1d165, 0x1d16a), (0x1d16d, 0x1d183), (0x1d185, 0x1d18c), (0x1d1aa, 0x1d1ae), (0x1d242, 0x1d245), (0x1d246, 0x1d253), (0x1d25b, 0x1d25d),
  (0x1d25f, 0x1d260), (0x1d280, 0x1d2c0), (0x1d2d4, 0x1d2e0), (0x1d2f4, 0x1d300), (0x1d357, 0x1d360), (0x1d379, 0x1d400), (0x1d455, 0x1d456), (0x1d49d, 0x1d49e),
  (0x1d4a0, 0x1d4a2), (0x1d4a3, 0x1d4a5), (0x1d4a7, 0x1d4a9), (0x1d4ad, 0x1d4ae), (0x1d4ba, 0x1d4bb), (0x1d4bc, 0x1d4bd), (0x1d4c4, 0x1d4c5), (0x1d506, 0x1d507),
  (0x1d50b, 0x1d50d), (0x1d515, 0x1d516), (0x1d51d, 0x1d51e), (0x1d53a, 0x1d53b), (0x1d53f, 0x1d540), (0x1d545, 0x1d546), (0x1d547, 0x1d54a), (0x1d551, 0x1d552),
  (0x1d6a7, 0x1d6a8), (0x1d7cc, 0x1d7ce), (0x1da00, 0x1da37), (0x1da3b, 0x1da6d), (0x1da75, 0x1da76), (0x1da84, 0x1da85), (0x1da8c, 0x1db00), (0x1db1d, 0x1df00),
  (0x1df82, 0x1df90), (0x1df97, 0x1dfcd), (0x1e000, 0x1e030), (0x1e06e, 0x1e100), (0x1e12d, 0x1e137), (0x1e13e, 0x1e140), (0x1e14a, 0x1e14e), (0x1e150, 0x1e290),
  (0x1e2ae, 0x1e2c0), (0x1e2ec, 0x1e2f0), (0x1e2fa, 0x1e2ff), (0x1e300, 0x1e4d0), (0x1e4ec, 0x1e4f0), (0x1e4fa, 0x1e5d0), (0x1e5ee, 0x1e5f0), (0x1e5fb, 0x1e5ff),
  (0x1e600, 0x1e6c0), (0x1e6df, 0x1e6e0), (0x1e6e3, 0x1e6e4), (0x1e6e6, 0x1e6e7), (0x1e6ee, 0x1e6f0), (0x1e6f5, 0x1e6fe), (0x1e700, 0x1e7e0), (0x1e7e7, 0x1e7e8),
  (0x1e7ec, 0x1e7ed), (0x1e7ef, 0x1e7f0), (0x1e7ff, 0x1e800), (0x1e8c5, 0x1e8c7), (0x1e8d0, 0x1e900), (0x1e944, 0x1e94b), (0x1e94c, 0x1e950), (0x1e95a, 0x1e95e),
  (0x1e960, 0x1ec71), (0x1ecb5, 0x1ed01), (0x1ed3e, 0x1ee00), (0x1ee04, 0x1ee05), (0x1ee20, 0x1ee21), (0x1ee23, 0x1ee24), (0x1ee25, 0x1ee27), (0x1ee28, 0x1ee29),
  (0x1ee33, 0x1ee34), (0x1ee38, 0x1ee39), (0x1ee3a, 0x1ee3b), (0x1ee3c, 0x1ee42), (0x1ee43, 0x1ee47), (0x1ee48, 0x1ee49), (0x1ee4a, 0x1ee4b), (0x1ee4c, 0x1ee4d),
  (0x1ee50, 0x1ee51), (0x1ee53, 0x1ee54), (0x1ee55, 0x1ee57), (0x1ee58, 0x1ee59), (0x1ee5a, 0x1ee5b), (0x1ee5c, 0x1ee5d), (0x1ee5e, 0x1ee5f), (0x1ee60, 0x1ee61),
  (0x1ee63, 0x1ee64), (0x1ee65, 0x1ee67), (0x1ee6b, 0x1ee6c), (0x1ee73, 0x1ee74), (0x1ee78, 0x1ee79), (0x1ee7d, 0x1ee7e), (0x1ee7f, 0x1ee80), (0x1ee8a, 0x1ee8b),
  (0x1ee9c, 0x1eea1), (0x1eea4, 0x1eea5), (0x1eeaa, 0x1eeab), (0x1eebc, 0x1eef0), (0x1eef2, 0x1f000), (0x1f02c, 0x1f030), (0x1f094, 0x1f0a0), (0x1f0af, 0x1f0b1),
  (0x1f0c0, 0x1f0c1), (0x1f0d0, 0x1f0d1), (0x1f0f6, 0x1f100), (0x1f1af, 0x1f1e6), (0x1f203, 0x1f210), (0x1f23c, 0x1f240), (0x1f249, 0x1f250), (0x1f252, 0x1f260),
  (0x1f266, 0x1f300), (0x1f6da, 0x1f6dc), (0x1f6ed, 0x1f6f0), (0x1f6fd, 0x1f700), (0x1f7dc, 0x1f7e0), (0x1f7ec, 0x1f7f0), (0x1f80c, 0x1f810), (0x1f848, 0x1f850),
  (0x1f85a, 0x1f860), (0x1f888, 0x1f890), (0x1f8ae, 0x1f8b0), (0x1f8bc, 0x1f8c0), (0x1f8c2, 0x1f8d0), (0x1f8d9, 0x1f900), (0x1fa58, 0x1fa60), (0x1fa6e, 0x1fa70),
  (0x1fa7d, 0x1fa80), (0x1fac7, 0x1fac8), (0x1fac9, 0x1facc), (0x1fade, 0x1fadf), (0x1faec, 0x1faef), (0x1fafb, 0x1fb00), (0x1fb93, 0x1fb94), (0x1fbfb, 0x20000),
  (0x2a6e0, 0x2a700), (0x2b81f, 0x2b820), (0x2ceae, 0x2ceb0), (0x2ebe1, 0x2ebf0), (0x2ee5e, 0x2f800), (0x2fa1e, 0x30000), (0x3134b, 0x31350), (0x3347a, 0x3d000),
  (0x3fc40, 0x110000)
]

/-- A non-ASCII character as Rust's `Debug` shows it, which is all a `str` index's panic names. -/
def charDebug (c : Char) : String :=
  if debugEscaped.any fun r => r.1 ≤ c.toNat && c.toNat < r.2 then
    "'\\u{" ++ String.ofList (Nat.toDigits 16 c.toNat) ++ "}'"
  else "'" ++ c.toString ++ "'"

/-- The panic of `&s[b..e]` where `b` or `e` is past the end, they are backwards, or one falls
inside a character, as `core`'s `slice_error_fail` words it. -/
def strSliceFail (s : String) (b e : Nat) : String :=
  let len := s.utf8ByteSize
  if b > len then s!"start byte index {b} is out of bounds for string of length {len}"
  else if e > len then s!"end byte index {e} is out of bounds for string of length {len}"
  else if b > e then s!"byte range starts at {b} but ends at {e}"
  else
    let (side, i) := if charBoundary s b then ("end", e) else ("start", b)
    let floor := ((List.range (i + 1)).reverse.find? (charBoundary s ·)).getD 0
    let ceil := ((List.range' i (len + 1 - i)).find? (charBoundary s ·)).getD len
    s!"{side} byte index {i} is not a char boundary; it is inside \
      {charDebug (String.Pos.Raw.get s ⟨floor⟩)} (bytes {floor}..{ceil} of string)"

/-- `&s[b..e]`, panicking as Rust's `str` index does. -/
def strSlice (s : String) (b e : UInt64) : M String :=
  if b ≤ e ∧ charBoundary s b.toNat ∧ charBoundary s e.toNat then
    pure (String.Pos.Raw.extract s ⟨b.toNat⟩ ⟨e.toNat⟩)
  else throw (strSliceFail s b.toNat e.toNat)

/-- `&s[..e]`, which Rust checks as `&s[0..e]`. -/
def strSliceTo (s : String) (e : UInt64) : M String := strSlice s 0 e

/-- `&s[b..]`, which Rust checks as `&s[b..s.len()]`. -/
def strSliceFrom (s : String) (b : UInt64) : M String := strSlice s b (strLen s)

/-- The byte index of the first `c` among `cs`, the characters of a string from byte `i` on. -/
def findChar (c : Char) : List Char → Nat → Option Nat
  | [], _ => none
  | x :: xs, i => if x = c then some i else findChar c xs (i + x.utf8Size)

/-- `str::find` of a character: the byte index it first starts at. -/
def strFind (s : String) (c : Char) : Option UInt64 := (findChar c s.toList 0).map (·.toUInt64)

/-- `Iterator::next` on a `Range`, as its bounds: the range after, and the item. -/
def rangeNext {α : Type} [Scalar α] (r : α × α) : (α × α) × Option α :=
  if val r.1 < val r.2 then ((r.1 + Scalar.ofInt 1, r.2), some r.1) else (r, none)

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

/-- `Iterator::next`: the items after, and the first. -/
def listNext {α : Type} (l : List α) : List α × Option α :=
  match l with
  | [] => ([], none)
  | x :: r => (r, some x)

/-- `Iterator::next` on items that may panic: the first is made here. -/
def listNextM {α : Type} (l : List (Unit → M α)) : M (List (Unit → M α) × Option α) :=
  match l with
  | [] => pure ([], none)
  | t :: r => do pure (r, some (← t ()))

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

/-- `BTreeMap::get_key_value`. -/
def getKeyValue {κ ν : Type} [Ord κ] (m : BTreeMap κ ν) (k : κ) : Option (κ × ν) :=
  m.entries.find? fun e => compare e.1 k == .eq

/-- `map[&k]`, panicking as Rust's does when `k` is absent. -/
def index {κ ν : Type} [Ord κ] (m : BTreeMap κ ν) (k : κ) : M ν :=
  expect (m.get k) "no entry found for key"

/-- Insert into ascending entries: the entries, and the value `k` had. Rust keeps the key
it had, which a structural key cannot tell apart. -/
def insertIn {κ ν : Type} [Ord κ] (k : κ) (v : ν) : List (κ × ν) → List (κ × ν) × Option ν
  | [] => ([(k, v)], none)
  | e :: rest =>
    match compare k e.1 with
    | .lt => ((k, v) :: e :: rest, none)
    | .eq => ((e.1, v) :: rest, some e.2)
    | .gt => let r := insertIn k v rest; (e :: r.1, r.2)

/-- `BTreeMap::insert`: the map, and the value `k` had. -/
def insert {κ ν : Type} [Ord κ] (m : BTreeMap κ ν) (k : κ) (v : ν) : BTreeMap κ ν × Option ν :=
  let r := insertIn k v m.entries
  (⟨r.1⟩, r.2)

/-- `BTreeMap::remove`: the map, and the value `k` had. -/
def remove {κ ν : Type} [Ord κ] (m : BTreeMap κ ν) (k : κ) : BTreeMap κ ν × Option ν :=
  (⟨m.entries.filter fun e => compare e.1 k != .eq⟩, m.get k)

/-- The value at `k`, which a `&mut` from `get_mut` or `entry` reads. Such a lens exists only
while `k` is in the map, so `default` is never the answer. -/
def valueAt {κ ν : Type} [Ord κ] [Inhabited ν] (m : BTreeMap κ ν) (k : κ) : ν :=
  (m.get k).getD default

/-- Write `v` at `k`, as a `&mut` from `get_mut` or `entry` does. -/
def setAt {κ ν : Type} [Ord κ] (m : BTreeMap κ ν) (k : κ) (v : ν) : BTreeMap κ ν :=
  (m.insert k v).1

/-- `entry(k).or_insert_with(f)`, `or_insert`, and `or_default`: the map, holding `f ()` at
`k` if it held nothing there. -/
def orInsertWith {κ ν : Type} [Ord κ] (m : BTreeMap κ ν) (k : κ) (f : Unit → ν) : BTreeMap κ ν :=
  if m.containsKey k then m else m.setAt k (f ())

/-- `orInsertWith` with an `f` that may panic, run only if `k` is absent. -/
def orInsertWithM {κ ν : Type} [Ord κ] (m : BTreeMap κ ν) (k : κ) (f : Unit → M ν) : M (BTreeMap κ ν) :=
  if m.containsKey k then pure m else m.setAt k <$> f ()

/-- `BTreeMap::pop_first`: the map, and its first entry. -/
def popFirst {κ ν : Type} (m : BTreeMap κ ν) : BTreeMap κ ν × Option (κ × ν) :=
  (⟨m.entries.drop 1⟩, m.entries.head?)

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

/-- Insert into ascending items, unless one equal to `k` is there already. -/
def insertIn {κ : Type} [Ord κ] (k : κ) : List κ → List κ
  | [] => [k]
  | x :: rest =>
    match compare k x with
    | .lt => k :: x :: rest
    | .eq => x :: rest
    | .gt => x :: insertIn k rest

/-- `BTreeSet::insert`: the set, and whether `k` was new to it. -/
def insert {κ : Type} [Ord κ] (s : BTreeSet κ) (k : κ) : BTreeSet κ × Bool :=
  (⟨insertIn k s.items⟩, !s.contains k)

/-- `BTreeSet::remove`: the set, and whether `k` was in it. -/
def remove {κ : Type} [Ord κ] (s : BTreeSet κ) (k : κ) : BTreeSet κ × Bool :=
  (⟨s.items.filter fun x => compare x k != .eq⟩, s.contains k)

/-- `BTreeSet::pop_first`: the set, and its first item. -/
def popFirst {κ : Type} (s : BTreeSet κ) : BTreeSet κ × Option κ :=
  (⟨s.items.drop 1⟩, s.items.head?)

end BTreeSet

unsigned_scalar UInt8 8
unsigned_scalar UInt16 16
unsigned_scalar UInt32 32
unsigned_scalar UInt64 64
signed_scalar Int8 8
signed_scalar Int16 16
signed_scalar Int32 32
signed_scalar Int64 64

instance : Scalar UInt128 where
  val x := (x.toNat : Int)
  lo := 0
  hi := 2 ^ 128 - 1
  ofInt i := UInt128.ofNat (i % 2 ^ 128).toNat
  val_add a b _ h := by
    show ((a.toBitVec + b.toBitVec).toNat : Int) = _
    simp only [BitVec.toNat_add, UInt128.toNat] at *
    rw [Nat.mod_eq_of_lt (by omega)]; push_cast; rfl
  val_sub a b l _ := by
    show ((a.toBitVec - b.toBitVec).toNat : Int) = _
    simp only [UInt128.toNat] at *
    have hle : b.toBitVec ≤ a.toBitVec := by rw [BitVec.le_def]; omega
    rw [BitVec.toNat_sub_of_le hle]; omega
  val_mul a b _ h := by
    show ((a.toBitVec * b.toBitVec).toNat : Int) = _
    simp only [BitVec.toNat_mul, UInt128.toNat] at *
    have : (a.toBitVec.toNat * b.toBitVec.toNat : Int) = ((a.toBitVec.toNat * b.toBitVec.toNat : Nat) : Int) := by push_cast; rfl
    rw [Nat.mod_eq_of_lt (by omega)]; omega

instance : Scalar Int128 where
  val x := x.toInt
  lo := -2 ^ 127
  hi := 2 ^ 127 - 1
  ofInt i := Int128.ofInt i
  val_add a b l h := by
    show (a.toBitVec + b.toBitVec).toInt = _
    simp only [Int128.toInt] at *
    rw [BitVec.toInt_add, Int.bmod_eq_of_le] <;> omega
  val_sub a b l h := by
    show (a.toBitVec - b.toBitVec).toInt = _
    simp only [Int128.toInt] at *
    rw [BitVec.toInt_sub, Int.bmod_eq_of_le] <;> omega
  val_mul a b l h := by
    show (a.toBitVec * b.toBitVec).toInt = _
    simp only [Int128.toInt] at *
    rw [BitVec.toInt_mul, Int.bmod_eq_of_le] <;> omega

end Rust

deriving instance DecidableEq for Except
