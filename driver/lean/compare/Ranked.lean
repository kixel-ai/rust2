import KioPureMachine
open kio_pure_machine

namespace kio_pure_machine.warm_settle

/-- `m` returns a value that satisfies `p`. -/
def Spec {α : Type} (m : Rust.M α) (p : α → Prop) : Prop := ∃ v, m = pure v ∧ p v

theorem Spec.bind {α β : Type} {m : Rust.M α} {f : α → Rust.M β} {p : α → Prop} {q : β → Prop}
    (h : Spec m p) (k : ∀ v, p v → Spec (f v) q) : Spec (m >>= f) q := by
  obtain ⟨v, rfl, hv⟩ := h
  simpa using k v hv

theorem Spec.pure {α : Type} {v : α} {p : α → Prop} (h : p v) : Spec (pure v) p := ⟨v, rfl, h⟩

theorem sub_one {r : UInt8} (h : 1 ≤ r) : r -? 1 = pure (r - 1) := by
  have := UInt8.le_iff_toNat_le.mp h
  have := r.toNat_lt
  apply Rust.sub_ok <;> simp [Rust.Scalar.val, Rust.Scalar.lo, Rust.Scalar.hi] at * <;> omega

theorem index {N : UInt64} {r : UInt8} (h1 : 1 ≤ r) (h2 : r.toUInt64 ≤ N) : (r - 1).toUInt64 < N := by
  have := UInt8.le_iff_toNat_le.mp h1
  have := UInt64.le_iff_toNat_le.mp h2
  rw [UInt64.lt_iff_toNat_lt, UInt8.toNat_toUInt64, UInt8.toNat_sub_of_le _ _ h1]
  simp at *
  omega

/-- What a relay's number and size need for its helpers not to fail. -/
def Numbered {N : UInt64} (self : Relay N) : Prop := N ≤ 16 ∧ 1 ≤ self.id ∧ self.id ≤ 16

/-- A relay whose route, if any, leads through one of its `N` relays. -/
def Routed {N : UInt64} (self : Relay N) : Prop :=
  ∀ hop cost, self.route = some (hop, cost) → 1 ≤ hop ∧ hop.toUInt64 ≤ N

theorem bit_ok {r : UInt8} (h1 : 1 ≤ r) (h2 : r ≤ 16) : Spec (bit r) fun _ => True := by
  have : r - 1 < 16 := by
    have := UInt8.le_iff_toNat_le.mp h1
    have := UInt8.le_iff_toNat_le.mp h2
    rw [UInt8.lt_iff_toNat_lt, UInt8.toNat_sub_of_le _ _ h1]
    simp at *
    omega
  simp only [bit, sub_one h1, pure_bind, this, decide_true]
  exact Spec.pure trivial

theorem link_ok {N : UInt64} (self : Relay N) {r : UInt8} (h1 : 1 ≤ r) (h2 : r.toUInt64 ≤ N) :
    Spec (self.link r) fun _ => True := by
  simp only [Relay.link, sub_one h1, pure_bind, index h1 h2, ↓reduceIte]
  exact Spec.pure trivial

theorem heard_ok {N : UInt64} (self : Relay N) {r : UInt8} (h1 : 1 ≤ r) (h2 : r.toUInt64 ≤ N) :
    Spec (self.heard' r) fun _ => True := by
  simp only [Relay.heard', sub_one h1, pure_bind, index h1 h2, ↓reduceIte]
  exact Spec.pure trivial

theorem is_open_ok {N : UInt64} (self : Relay N) {r : UInt8} (h1 : 1 ≤ r) (h2 : r.toUInt64 ≤ N) :
    Spec (self.is_open r) fun _ => True := by
  obtain ⟨l, hl, -⟩ := link_ok self h1 h2
  simp only [Relay.is_open, hl, pure_bind]
  split <;> (try split) <;> exact Spec.pure trivial

theorem down_ok {N : UInt64} (self : Relay N) {r : UInt8} (h1 : 1 ≤ r) (h2 : r.toUInt64 ≤ N) :
    Spec (self.down r) fun _ => True := by
  obtain ⟨o, ho, -⟩ := is_open_ok self h1 h2
  obtain ⟨h, hh, -⟩ := heard_ok self h1 h2
  simp only [Relay.down, ho, hh, pure_bind]
  split <;> exact Spec.pure trivial

theorem pull_ok {N : UInt64} (self : Relay N) (hn : Numbered self) {r : UInt8} (h1 : 1 ≤ r)
    (h2 : r.toUInt64 ≤ N) : Spec (self.pull r) fun _ => True := by
  obtain ⟨hN, hi1, hi2⟩ := hn
  have hr : r ≤ 16 := by
    have := UInt64.le_iff_toNat_le.mp h2
    have := UInt64.le_iff_toNat_le.mp hN
    apply UInt8.le_iff_toNat_le.mpr
    simp at *
    omega
  obtain ⟨h, hh, -⟩ := heard_ok self h1 h2
  obtain ⟨b, hb, -⟩ := bit_ok hi1 hi2
  obtain ⟨b', hb', -⟩ := bit_ok h1 hr
  simp only [Relay.pull, hh, hb, hb', pure_bind]
  split <;> (try split) <;> exact Spec.pure trivial

theorem succ_ok {N : UInt64} {d : UInt8} (hN : N ≤ 16) (h : d.toUInt64 ≤ N) : d +? 1 = pure (d + 1) := by
  have := UInt64.le_iff_toNat_le.mp h
  have := UInt64.le_iff_toNat_le.mp hN
  apply Rust.add_ok <;> simp [Rust.Scalar.val, Rust.Scalar.lo, Rust.Scalar.hi] at * <;> omega

theorem succ_pos {N : UInt64} {d : UInt8} (hN : N ≤ 16) (h : d.toUInt64 ≤ N) : 1 ≤ d + 1 := by
  have := UInt64.le_iff_toNat_le.mp h
  have := UInt64.le_iff_toNat_le.mp hN
  apply UInt8.le_iff_toNat_le.mpr
  rw [UInt8.toNat_add]
  simp at *
  omega

theorem succ_lt {N : UInt64} {d : UInt8} (hN : N ≤ 16) (h : d.toUInt64 ≤ N) :
    N.toNat + 1 - (d + 1).toNat < N.toNat + 1 - d.toNat := by
  have := UInt64.le_iff_toNat_le.mp h
  have := UInt64.le_iff_toNat_le.mp hN
  rw [UInt8.toNat_add]
  simp at *
  omega

theorem mine_loop_ok {N : UInt64} (self : Relay N) (hN : N ≤ 16) (rank : UInt32) (d : UInt8)
    (hd : 1 ≤ d) : Spec (Relay.mine_loop self rank d) fun _ => True := by
  rw [Relay.mine_loop]
  by_cases hle : d.toUInt64 ≤ N
  · obtain ⟨b, hb, -⟩ := down_ok self hd hle
    obtain ⟨h, hh, -⟩ := heard_ok self hd hle
    simp only [hle, ↓reduceIte, hb, hh, succ_ok hN hle, pure_bind]
    split <;> (try split) <;> (try split) <;> exact mine_loop_ok self hN _ _ (succ_pos hN hle)
  · simp only [hle, ↓reduceIte]
    exact Spec.pure trivial
termination_by N.toNat + 1 - d.toNat
decreasing_by all_goals exact succ_lt hN hle

theorem mine_ok {N : UInt64} (self : Relay N) (hN : N ≤ 16) : Spec self.mine fun _ => True := by
  unfold Relay.mine
  split <;> simpa using mine_loop_ok self hN _ 1 (by decide)

theorem push_below {cs : Array Choice} {x : Choice} {mine : UInt32}
    (hc : ∀ c ∈ cs, c.copy → c.rank < mine) (hx : x.rank < mine) :
    ∀ c ∈ cs.push x, c.copy → c.rank < mine := by
  intro c hm hcopy
  rcases Array.mem_push.mp hm with hm | rfl
  · exact hc c hm hcopy
  · exact hx

/-- The options loop, from relay `s` on, keeps every warm copy below `mine`. -/
theorem options_loop_ok {N : UInt64} (self : Relay N) (hn : Numbered self) (u : Option UInt8)
    (mine : UInt32) (choices : Array Choice) (s : UInt8) (hs : 1 ≤ s)
    (hc : ∀ c ∈ choices, c.copy → c.rank < mine) :
    Spec (self.options_loop u mine choices s) fun v => ∀ c ∈ v, c.copy → c.rank < mine := by
  have hN := hn.1
  rw [Relay.options_loop]
  by_cases hle : s.toUInt64 ≤ N
  · obtain ⟨h, hh, -⟩ := heard_ok self hs hle
    obtain ⟨o, ho, -⟩ := is_open_ok self hs hle
    obtain ⟨l, hl, -⟩ := link_ok self hs hle
    obtain ⟨d, hd, -⟩ := down_ok self hs hle
    obtain ⟨p, hp, -⟩ := pull_ok self hn hs hle
    obtain ⟨b, hb, -⟩ := bit_ok hn.2.1 hn.2.2
    simp only [hle, ↓reduceIte, hh, ho, hl, hd, hp, hb, succ_ok hN hle, pure_bind]
    repeat' split
    all_goals first
      | exact options_loop_ok self hn u mine _ _ (succ_pos hN hle) hc
      | exact options_loop_ok self hn u mine _ _ (succ_pos hN hle) (push_below hc ‹_›)
  · simp only [hle, ↓reduceIte]
    exact Spec.pure hc
termination_by N.toNat + 1 - s.toNat
decreasing_by all_goals exact succ_lt hN hle

/-- Every warm copy a relay may pull feeds a reader ranked strictly below every reader it
feeds itself: `options` returns, and each copy among its choices ranks below `mine`. -/
theorem options_rank_below {N : UInt64} (self : Relay N) (hn : Numbered self) (hr : Routed self)
    (u : Option UInt8) :
    ∃ mine, self.mine = pure mine ∧ Spec (self.options u) fun v => ∀ c ∈ v, c.copy → c.rank < mine := by
  obtain ⟨mine, hmine, -⟩ := mine_ok self hn.1
  refine ⟨mine, hmine, ?_⟩
  unfold Relay.options
  simp only [hmine, pure_bind]
  split
  · rename_i hop_cost hroute
    obtain ⟨h1, h2⟩ := hr hop_cost.1 hop_cost.2 hroute
    obtain ⟨d, hd, -⟩ := down_ok self h1 h2
    simp only [hd, pure_bind]
    split <;> simpa using options_loop_ok self hn u mine _ 1 (by decide) (by simp)
  · simpa using options_loop_ok self hn u mine _ 1 (by decide) (by simp)

end kio_pure_machine.warm_settle
