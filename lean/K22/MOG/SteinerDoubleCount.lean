/-
  SteinerDoubleCount.lean — residual infrastructure for
  `mogOctadsFormSteinerSystem_via_transport` (MonomialWitness Lane A).

  Bridge status (CB-1 timeline):
    S1–S2: discharged toward A-repo (Nat path, this pin)
      S1: maskWeightN_eq_card_maskToOctad / maskToOctad_card (fold + nodup)
      S2: maskToOctad_injective_on_octads (low-bit Finset + high-bit vanish)
    S3: double_count_5sets packing bound (bipartite fiber sum) — A-repo when lake-green
    S4: packing_eq_implies_steiner (Johnson equality ⇒ exact covering) — A-repo when lake-green
    S5: golayOctadBlocks_card / _pack — concrete Golay packing (octad_count + ∩≤4)
        ⇒ golay_octads_form_steiner closes by S4 (Golay blocks form S(5,8,24) on pin)
    S6: mask↔octad round-trips (this file; no MonomialWitness import — cycle break)
    CB-1 residual (`mogOctadsFormSteinerSystem_via_transport`) lives in MonomialWitness
        and imports this module after the cycle break.
    α+ω=15 is Category C only — not a load-bearing identity of these proofs.
    V^♮ / Monster / moonshine: A-lit horizon only — not formalised here.

  Runtime-only (not formal): BitVec.popCount, SWAR, __popc, warp shuffle reduce.

  ATOM: LANE-A-STEINER-DOUBLECOUNT-SURFACE-20260729 | α + ω = 15 (Category C)
  CB-1: docs/componentry/ATOMS/ATOM-CB1-STEINER-TRANSPORT-TIMELINE-20260730.md
-/

import Mathlib.Data.Finset.Basic
import Mathlib.Data.Finset.Card
import Mathlib.Data.Finset.Powerset
import Mathlib.Data.Nat.Choose.Basic
import Mathlib.Data.Nat.Bitwise
import Mathlib.Data.List.FinRange
import Mathlib.Algebra.BigOperators.Group.Finset
import Mathlib.Algebra.Order.BigOperators.Group.Finset
import K22.HexacodeGolay
import K22.MiracleOctadGenerator

open MiracleOctadGenerator (MOGPoint)

namespace K22.MOG.SteinerDoubleCount

set_option maxHeartbeats 8000000

/-! ## 0. Binomial design constants -/

theorem binom_24_5 : Nat.choose 24 5 = 42504 := by native_decide
theorem binom_8_5 : Nat.choose 8 5 = 56 := by native_decide
theorem steiner_count_identity : 759 * Nat.choose 8 5 = Nat.choose 24 5 := by
  native_decide
theorem lambda_s_table :
    (Nat.choose (24 - 0) (5 - 0) / Nat.choose (8 - 0) (5 - 0) = 759) ∧
    (Nat.choose (24 - 1) (5 - 1) / Nat.choose (8 - 1) (5 - 1) = 253) ∧
    (Nat.choose (24 - 2) (5 - 2) / Nat.choose (8 - 2) (5 - 2) = 77) ∧
    (Nat.choose (24 - 3) (5 - 3) / Nat.choose (8 - 3) (5 - 3) = 21) ∧
    (Nat.choose (24 - 4) (5 - 4) / Nat.choose (8 - 4) (5 - 4) = 5) ∧
    (Nat.choose (24 - 5) (5 - 5) / Nat.choose (8 - 5) (5 - 5) = 1) := by
  native_decide

/-! ## 1. Mask ↔ Finset bridge -/

/-- Support of the mask on `Fin 24`, using the same `bitOn` as `maskWeightN`. -/
def maskToOctad (m : Nat) : Finset MOGPoint :=
  (Finset.univ : Finset MOGPoint).filter fun p => bitOn m p.val

/-- Pack support bits by OR of unit powers (over `finRange`, no `toList`). -/
def octadToMask (s : Finset MOGPoint) : Nat :=
  ((List.finRange 24).filter (fun i : Fin 24 => i ∈ s)).foldl
    (fun acc i => acc ||| (2 ^ i.val)) 0

theorem mem_maskToOctad_iff (m : Nat) (p : MOGPoint) :
    p ∈ maskToOctad m ↔ bitOn m p.val = true := by
  simp [maskToOctad]

/-! ### S1: weight ↔ support card -/

private theorem foldl_bitOn_from_acc (m : Nat) :
    ∀ (l : List Nat) (acc : Nat),
      l.foldl (fun a p => if bitOn m p then a + 1 else a) acc =
        acc + l.countP (fun p => bitOn m p) := by
  intro l
  induction l with
  | nil =>
    intro acc
    simp
  | cons x xs ih =>
    intro acc
    simp only [List.foldl_cons, List.countP_cons]
    cases hx : bitOn m x
    · simp [hx, ih acc]
    · simp [hx, ih (acc + 1)]; omega

private theorem mem_filter_map_val_finRange (m : Nat) (n : Nat) :
    n ∈ ((List.finRange 24).filter (fun p : Fin 24 => bitOn m p.val)).map Fin.val ↔
      n ∈ (List.range 24).filter (fun p => bitOn m p) := by
  simp only [List.mem_map, List.mem_filter, List.mem_finRange, List.mem_range]
  constructor
  · rintro ⟨p, ⟨_, hb⟩, rfl⟩
    exact ⟨p.isLt, hb⟩
  · intro ⟨hn, hb⟩
    exact ⟨⟨n, hn⟩, ⟨True.intro, hb⟩, rfl⟩

/-- Length (not order) agreement: `finRange` filter mapped by `Fin.val` is a
permutation of the `range` filter (both nodup). Pin has no `List.Nodup.ext`. -/
private theorem filter_map_val_finRange_length (m : Nat) :
    (((List.finRange 24).filter (fun p : Fin 24 => bitOn m p.val)).map Fin.val).length =
      ((List.range 24).filter (fun p => bitOn m p)).length := by
  have h1 :
      (((List.finRange 24).filter (fun p : Fin 24 => bitOn m p.val)).map Fin.val).Nodup := by
    have := List.Nodup.filter (fun p : Fin 24 => bitOn m p.val) (List.nodup_finRange 24)
    exact List.Nodup.map Fin.val_injective this
  have h2 : ((List.range 24).filter (fun p => bitOn m p)).Nodup :=
    List.Nodup.filter _ (List.nodup_range 24)
  exact List.Perm.length_eq
    ((List.perm_ext_iff_of_nodup h1 h2).mpr fun n => mem_filter_map_val_finRange m n)

theorem maskWeightN_eq_card_maskToOctad (m : Nat) :
    maskWeightN m = (maskToOctad m).card := by
  simp only [maskWeightN, maskToOctad]
  have hfold := foldl_bitOn_from_acc m (List.range 24) 0
  simp only [Nat.zero_add] at hfold
  rw [hfold, List.countP_eq_length_filter]
  -- range filter length = finRange filter length
  have hlen :
      ((List.range 24).filter (fun p => bitOn m p)).length =
        ((List.finRange 24).filter (fun p : Fin 24 => bitOn m p.val)).length := by
    have := (filter_map_val_finRange_length m).symm
    simpa [List.length_map] using this
  rw [hlen]
  have hnodup :
      ((List.finRange 24).filter (fun p : Fin 24 => bitOn m p.val)).Nodup :=
    List.Nodup.filter _ (List.nodup_finRange 24)
  rw [← List.toFinset_card_of_nodup hnodup]
  congr 1
  ext p
  simp [List.mem_toFinset, List.mem_filter, List.mem_finRange]

theorem maskToOctad_card (m : Nat) (_hm : m < 2 ^ 24 := by decide) :
    (maskToOctad m).card = maskWeightN m :=
  (maskWeightN_eq_card_maskToOctad m).symm

theorem maskToOctad_card_of_weight_eight (m : Nat) (hw : maskWeightN m = 8) :
    (maskToOctad m).card = 8 := by
  rw [maskWeightN_eq_card_maskToOctad] at hw
  exact hw

/-! ### Bit extensionality wrappers (Init on lean 4.8.0) -/

theorem eq_of_testBit_eq_of_lt_two_pow {n m k : Nat}
    (hn : n < 2 ^ k) (hm : m < 2 ^ k)
    (h : ∀ i < k, n.testBit i = m.testBit i) : n = m := by
  apply Nat.eq_of_testBit_eq
  intro i
  by_cases hi : i < k
  · exact h i hi
  · have hle : k ≤ i := Nat.le_of_not_lt hi
    have hpow : 2 ^ k ≤ 2 ^ i := Nat.pow_le_pow_of_le_right (by decide : 0 < 2) hle
    rw [Nat.testBit_eq_false_of_lt (lt_of_lt_of_le hn hpow)]
    rw [Nat.testBit_eq_false_of_lt (lt_of_lt_of_le hm hpow)]

/-! ### S2: injectivity on 24-bit Golay octad masks -/

theorem testBit_eq_of_maskToOctad_eq {a b : Nat}
    (h : maskToOctad a = maskToOctad b) {i : Nat} (hi : i < 24) :
    a.testBit i = b.testBit i := by
  have hba : bitOn a i = true ↔ bitOn b i = true := by
    constructor
    · intro ha
      have : (⟨i, hi⟩ : Fin 24) ∈ maskToOctad a := (mem_maskToOctad_iff a _).mpr ha
      rw [h] at this
      exact (mem_maskToOctad_iff b _).mp this
    · intro hb
      have : (⟨i, hi⟩ : Fin 24) ∈ maskToOctad b := (mem_maskToOctad_iff b _).mpr hb
      rw [← h] at this
      exact (mem_maskToOctad_iff a _).mp this
  have ha := bitOn_eq_testBit a i
  have hb := bitOn_eq_testBit b i
  cases ca : bitOn a i <;> cases cb : bitOn b i
  · have ta : a.testBit i = false := by
      have := ha.symm.trans ca
      simpa [decide_eq_false_iff_not, Bool.not_eq_true] using this
    have tb : b.testBit i = false := by
      have := hb.symm.trans cb
      simpa [decide_eq_false_iff_not, Bool.not_eq_true] using this
    simp [ta, tb]
  · exfalso; have := hba; simp [ca, cb] at this
  · exfalso; have := hba; simp [ca, cb] at this
  · have ta : a.testBit i = true := by
      have := ha.symm.trans ca; simpa using this
    have tb : b.testBit i = true := by
      have := hb.symm.trans cb; simpa using this
    simp [ta, tb]

theorem eq_of_maskToOctad_eq_of_lt {a b : Nat}
    (ha : a < 2 ^ 24) (hb : b < 2 ^ 24)
    (h : maskToOctad a = maskToOctad b) : a = b := by
  apply eq_of_testBit_eq_of_lt_two_pow ha hb
  intro i hi
  exact testBit_eq_of_maskToOctad_eq h hi

private theorem golayBasisN_lt (i : Nat) : golayBasisN i < 2 ^ 24 := by
  unfold golayBasisN; split <;> decide

theorem golayEncode_lt (msg : Nat) : golayEncode msg < 2 ^ 24 := by
  unfold golayEncode
  have hxor : ∀ x y : Nat, x < 2 ^ 24 → y < 2 ^ 24 → Nat.xor x y < 2 ^ 24 :=
    fun x y hx hy => Nat.xor_lt_two_pow hx hy
  -- foldr (fun i acc => if bit then xor basis i acc else acc)
  suffices h :
      ∀ (l : List Nat) (acc : Nat), acc < 2 ^ 24 →
        l.foldr (fun i acc =>
          if bitOn msg i then Nat.xor (golayBasisN i) acc else acc) acc < 2 ^ 24 by
    exact h (List.range 12) 0 (by decide)
  intro l
  induction l with
  | nil => intro acc hacc; simpa
  | cons i t ih =>
    intro acc hacc
    simp only [List.foldr_cons]
    cases hbit : bitOn msg i
    · simp [hbit]; exact ih acc hacc
    · simp [hbit]
      -- foldr = f i (foldr t acc)
      exact hxor (golayBasisN i) _ (golayBasisN_lt i) (ih acc hacc)

theorem golayCode_lt {m : Nat} (hm : m ∈ golayCode) : m < 2 ^ 24 := by
  simp only [golayCode, Finset.mem_image] at hm
  rcases hm with ⟨msg, _, rfl⟩
  exact golayEncode_lt msg

theorem maskToOctad_injective_on_octads
    {a b : Nat}
    (ha : a ∈ golayCode.filter (fun n => maskWeightN n = 8))
    (hb : b ∈ golayCode.filter (fun n => maskWeightN n = 8))
    (heq : maskToOctad a = maskToOctad b) :
    a = b := by
  simp only [Finset.mem_filter] at ha hb
  exact eq_of_maskToOctad_eq_of_lt (golayCode_lt ha.1) (golayCode_lt hb.1) heq

/-! ### S6: mask ↔ Finset bijection surface (Gemini strand patch, on-pin)

Round-trips for `maskToOctad` / `octadToMask`. No `card = 8` hypothesis is required:
the maps are inverse on all subsets of `Fin 24` and all masks `< 2^24`.
Weight-8 specialisation is the `OctadEquiv` surface used by CB-1 glue in MonomialWitness.
-/

/-- Fold of `||| 2^i` sets bit `k` iff it was already set or some list entry is `k`. -/
private theorem bitOn_foldl_or_pow :
    ∀ (l : List (Fin 24)) (acc : Nat) (k : Nat),
      bitOn (l.foldl (fun a i => a ||| (2 ^ i.val)) acc) k = true ↔
        bitOn acc k = true ∨ ∃ i ∈ l, i.val = k := by
  intro l
  induction l with
  | nil =>
    intro acc k
    simp [List.foldl_nil]
  | cons i rest ih =>
    intro acc k
    simp only [List.foldl_cons, List.mem_cons]
    have hstep :
        bitOn (acc ||| (2 ^ i.val)) k = true ↔
          bitOn acc k = true ∨ i.val = k := by
      simp [bitOn_eq_testBit, Nat.testBit_lor, Nat.testBit_two_pow,
        Bool.or_eq_true, decide_eq_true_eq]
    rw [ih (acc ||| (2 ^ i.val)) k, hstep]
    constructor
    · rintro (hacc | ⟨j, hj, rfl⟩)
      · rcases hacc with hacc | hi
        · exact Or.inl hacc
        · exact Or.inr ⟨i, Or.inl rfl, hi⟩
      · exact Or.inr ⟨j, Or.inr hj, rfl⟩
    · rintro (hacc | ⟨j, hj, hjv⟩)
      · exact Or.inl (Or.inl hacc)
      · rcases hj with rfl | hj
        · exact Or.inl (Or.inr hjv)
        · exact Or.inr ⟨j, hj, hjv⟩

/-- Public bit characterization of `octadToMask` (Gemini α-rail / CB-1 glue). -/
theorem bitOn_octadToMask_iff (s : Finset MOGPoint) (k : Nat) (hk : k < 24) :
    bitOn (octadToMask s) k = true ↔ (⟨k, hk⟩ : MOGPoint) ∈ s := by
  classical
  unfold octadToMask
  have h0 : bitOn 0 k = false := by
    simp [bitOn_eq_testBit, Nat.testBit_zero]
  have h' := bitOn_foldl_or_pow
    ((List.finRange 24).filter (fun i : Fin 24 => i ∈ s)) 0 k
  simp only [h0, Bool.false_eq_true, false_or] at h'
  rw [h']
  constructor
  · rintro ⟨i, hi, hival⟩
    have hi' := List.mem_filter.mp hi
    have : i = ⟨k, hk⟩ := Fin.ext hival
    simpa [this, decide_eq_true_eq] using hi'.2
  · intro hks
    refine ⟨⟨k, hk⟩, ?_, rfl⟩
    refine List.mem_filter.mpr ⟨List.mem_finRange _, ?_⟩
    simpa [decide_eq_true_eq] using hks

/-- **S6.1** Forward round-trip: Finset → mask → Finset. -/
theorem maskToOctad_octadToMask (s : Finset MOGPoint) :
    maskToOctad (octadToMask s) = s := by
  ext p
  simp only [mem_maskToOctad_iff]
  exact bitOn_octadToMask_iff s p.val p.isLt

private theorem two_pow_val_lt (i : Fin 24) : 2 ^ i.val < 2 ^ 24 :=
  pow_lt_pow_right (by decide : (1 : Nat) < 2) i.isLt

private theorem lor_lt_two_pow_24 {a b : Nat}
    (ha : a < 2 ^ 24) (hb : b < 2 ^ 24) : a ||| b < 2 ^ 24 :=
  Nat.bitwise_lt (f := fun x y => x || y) ha hb

theorem octadToMask_lt (s : Finset MOGPoint) : octadToMask s < 2 ^ 24 := by
  unfold octadToMask
  suffices h :
      ∀ (l : List (Fin 24)) (acc : Nat), acc < 2 ^ 24 →
        l.foldl (fun a i => a ||| (2 ^ i.val)) acc < 2 ^ 24 by
    exact h _ 0 (by decide)
  intro l
  induction l with
  | nil =>
    intro acc hacc
    simpa using hacc
  | cons i t ih =>
    intro acc hacc
    simp only [List.foldl_cons]
    exact ih _ (lor_lt_two_pow_24 hacc (two_pow_val_lt i))

private theorem bitOn_octadToMask_eq_mem (m : Nat) (i : Nat) (hi : i < 24) :
    bitOn (octadToMask (maskToOctad m)) i = true ↔ bitOn m i = true := by
  rw [bitOn_octadToMask_iff (maskToOctad m) i hi, mem_maskToOctad_iff]

/-- **S6.2** Reverse round-trip: mask → Finset → mask (on 24-bit domain). -/
theorem octadToMask_maskToOctad (m : Nat) (hm : m < 2 ^ 24) :
    octadToMask (maskToOctad m) = m := by
  apply eq_of_testBit_eq_of_lt_two_pow (octadToMask_lt (maskToOctad m)) hm
  intro i hi
  have heq : bitOn (octadToMask (maskToOctad m)) i = true ↔ bitOn m i = true :=
    bitOn_octadToMask_eq_mem m i hi
  -- convert both sides via bitOn_eq_testBit
  cases hmbit : bitOn m i <;> cases hobit : bitOn (octadToMask (maskToOctad m)) i
  · have ta : m.testBit i = false := by
      simpa [bitOn_eq_testBit, decide_eq_false_iff_not, Bool.not_eq_true] using hmbit
    have tb : (octadToMask (maskToOctad m)).testBit i = false := by
      simpa [bitOn_eq_testBit, decide_eq_false_iff_not, Bool.not_eq_true] using hobit
    simp [ta, tb]
  · exfalso; have := heq; simp [hmbit, hobit] at this
  · exfalso; have := heq; simp [hmbit, hobit] at this
  · have ta : m.testBit i = true := by simpa [bitOn_eq_testBit] using hmbit
    have tb : (octadToMask (maskToOctad m)).testBit i = true := by
      simpa [bitOn_eq_testBit] using hobit
    simp [ta, tb]

/-- **S6.3** Weight of the packed mask equals Finset card. -/
theorem octadToMask_card (s : Finset MOGPoint) :
    maskWeightN (octadToMask s) = s.card := by
  rw [maskWeightN_eq_card_maskToOctad, maskToOctad_octadToMask]

/-- Weight-8 Golay form of reverse round-trip (uses `golayCode_lt`). -/
theorem octadToMask_maskToOctad_golay_octad
    {m : Nat} (hm : m ∈ golayCode.filter (fun n => maskWeightN n = 8)) :
    octadToMask (maskToOctad m) = m := by
  simp only [Finset.mem_filter] at hm
  exact octadToMask_maskToOctad m (golayCode_lt hm.1)

/-- **S6.4** Equiv surface: 24-bit masks ↔ subsets of `Fin 24` (Gemini `OctadEquiv` core). -/
noncomputable def maskFinsetEquiv : { m : Nat // m < 2 ^ 24 } ≃ Finset MOGPoint where
  toFun := fun ⟨m, _⟩ => maskToOctad m
  invFun := fun s => ⟨octadToMask s, octadToMask_lt s⟩
  left_inv := fun ⟨m, hm⟩ => Subtype.ext (octadToMask_maskToOctad m hm)
  right_inv := fun s => maskToOctad_octadToMask s

/-! ## 2. Abstract packing (S3 packing bound; S4 Johnson equality case) -/

/-- Bipartite double count of incidences `(b, s)` with `s ⊆ b` and `s.card = 5`:
    `∑_b #(powersetCard 5 b) = ∑_{|s|=5} #(blocks containing s)`. -/
private theorem sum_powersetCard_eq_sum_fibers (B : Finset (Finset MOGPoint)) :
    (B.sum fun b => (b.powersetCard 5).card) =
      (((Finset.univ : Finset MOGPoint).powersetCard 5).sum fun s =>
        (B.filter (fun b => s ⊆ b)).card) := by
  classical
  -- Incidence set of pairs (b, s)
  let P : Finset (Finset MOGPoint × Finset MOGPoint) :=
    (B.product ((Finset.univ : Finset MOGPoint).powersetCard 5)).filter
      fun p => p.2 ⊆ p.1
  -- Left: fiber over blocks
  have hL : P.card = B.sum fun b => (b.powersetCard 5).card := by
    have H : ∀ p ∈ P, p.1 ∈ B := by
      intro p hp
      exact (Finset.mem_product.mp (Finset.mem_filter.mp hp).1).1
    rw [Finset.card_eq_sum_card_fiberwise (f := Prod.fst) (t := B) H]
    refine Finset.sum_congr rfl ?_
    intro b hb
    have hEq :
        P.filter (fun p => p.1 = b) =
          (b.powersetCard 5).image fun s => (b, s) := by
      ext p
      constructor
      · intro hp
        obtain ⟨hpP, hp1⟩ := Finset.mem_filter.mp hp
        obtain ⟨hprod, hsub⟩ := Finset.mem_filter.mp hpP
        have hs : p.2 ∈ (Finset.univ : Finset MOGPoint).powersetCard 5 :=
          (Finset.mem_product.mp hprod).2
        have hcard : p.2.card = 5 := (Finset.mem_powersetCard.mp hs).2
        cases p with
        | mk b' s =>
          change b' = b at hp1
          subst hp1
          exact Finset.mem_image.mpr
            ⟨s, Finset.mem_powersetCard.mpr ⟨hsub, hcard⟩, rfl⟩
      · intro hp
        obtain ⟨s, hs, rfl⟩ := Finset.mem_image.mp hp
        obtain ⟨hsub, hcard⟩ := Finset.mem_powersetCard.mp hs
        refine Finset.mem_filter.mpr ⟨?_, rfl⟩
        exact Finset.mem_filter.mpr
          ⟨Finset.mem_product.mpr
            ⟨hb, Finset.mem_powersetCard.mpr ⟨Finset.subset_univ _, hcard⟩⟩, hsub⟩
    rw [hEq, Finset.card_image_of_injective]
    intro x y h
    exact (Prod.mk.inj h).2
  -- Right: fiber over 5-sets
  have hR :
      P.card =
        (((Finset.univ : Finset MOGPoint).powersetCard 5).sum fun s =>
          (B.filter (fun b => s ⊆ b)).card) := by
    have H : ∀ p ∈ P, p.2 ∈ (Finset.univ : Finset MOGPoint).powersetCard 5 := by
      intro p hp
      exact (Finset.mem_product.mp (Finset.mem_filter.mp hp).1).2
    rw [Finset.card_eq_sum_card_fiberwise
      (f := Prod.snd) (t := (Finset.univ : Finset MOGPoint).powersetCard 5) H]
    refine Finset.sum_congr rfl ?_
    intro s hs
    have hEq :
        P.filter (fun p => p.2 = s) =
          (B.filter (fun b => s ⊆ b)).image fun b => (b, s) := by
      ext p
      constructor
      · intro hp
        obtain ⟨hpP, hp2⟩ := Finset.mem_filter.mp hp
        obtain ⟨hprod, hsub⟩ := Finset.mem_filter.mp hpP
        have hb : p.1 ∈ B := (Finset.mem_product.mp hprod).1
        cases p with
        | mk b s' =>
          change s' = s at hp2
          subst hp2
          exact Finset.mem_image.mpr
            ⟨b, Finset.mem_filter.mpr ⟨hb, hsub⟩, rfl⟩
      · intro hp
        obtain ⟨b, hb, rfl⟩ := Finset.mem_image.mp hp
        obtain ⟨hbB, hsub⟩ := Finset.mem_filter.mp hb
        refine Finset.mem_filter.mpr ⟨?_, rfl⟩
        exact Finset.mem_filter.mpr ⟨Finset.mem_product.mpr ⟨hbB, hs⟩, hsub⟩
    rw [hEq, Finset.card_image_of_injective]
    intro x y h
    exact (Prod.mk.inj h).1
  exact hL.symm.trans hR

/-- **S3:** under packing + octad-sized blocks, `|B| * C(8,5) ≤ C(24,5)`. -/
theorem double_count_5sets
    (B : Finset (Finset MOGPoint))
    (hcard : ∀ b ∈ B, b.card = 8)
    (hpack : ∀ s : Finset MOGPoint, s.card = 5 →
      (B.filter (fun b => s ⊆ b)).card ≤ 1) :
    B.card * Nat.choose 8 5 ≤ Nat.choose 24 5 := by
  classical
  have hleft :
      (B.sum fun b => (b.powersetCard 5).card) = B.card * Nat.choose 8 5 := by
    have h1 :
        (B.sum fun b => (b.powersetCard 5).card) =
          B.sum fun _ => Nat.choose 8 5 := by
      refine Finset.sum_congr rfl ?_
      intro b hb
      rw [Finset.card_powersetCard, hcard b hb]
    have h2 : (B.sum fun _ => Nat.choose 8 5) = B.card * Nat.choose 8 5 := by
      simp [Finset.sum_const, nsmul_eq_mul]
    exact h1.trans h2
  have hright :
      (((Finset.univ : Finset MOGPoint).powersetCard 5).sum fun s =>
          (B.filter (fun b => s ⊆ b)).card) ≤
        Nat.choose 24 5 := by
    have hle :
        (((Finset.univ : Finset MOGPoint).powersetCard 5).sum fun s =>
            (B.filter (fun b => s ⊆ b)).card) ≤
          ((Finset.univ : Finset MOGPoint).powersetCard 5).sum fun _ => 1 :=
      Finset.sum_le_sum fun s hs =>
        hpack s (Finset.mem_powersetCard.mp hs).2
    have hones :
        ((Finset.univ : Finset MOGPoint).powersetCard 5).sum (fun _ => 1) =
          Nat.choose 24 5 := by
      simp [Finset.sum_const, nsmul_eq_mul, Finset.card_powersetCard,
        Finset.card_univ, Fintype.card_fin]
    exact hle.trans_eq hones
  calc
    B.card * Nat.choose 8 5
        = B.sum fun b => (b.powersetCard 5).card := hleft.symm
    _ = ((Finset.univ : Finset MOGPoint).powersetCard 5).sum fun s =>
          (B.filter (fun b => s ⊆ b)).card := sum_powersetCard_eq_sum_fibers B
    _ ≤ Nat.choose 24 5 := hright

/-- Pure arithmetic: if `0 ≤ f a ≤ 1` on `s` and `∑ f = |s|`, then `f ≡ 1`. -/
private theorem sum_le_one_eq_card_implies_eq_one
    {α : Type*} [DecidableEq α] (s : Finset α) (f : α → ℕ)
    (hle : ∀ a ∈ s, f a ≤ 1)
    (hsum : s.sum f = s.card) :
    ∀ a ∈ s, f a = 1 := by
  intro a ha
  -- If f a ≠ 1 then f a = 0 (from ≤ 1); then ∑f ≤ |s|-1, contradiction.
  by_cases hfa : f a = 1
  · exact hfa
  · have fa0 : f a = 0 := by
      have hlea := hle a ha
      match hf : f a with
      | 0 => rfl
      | 1 => exact (hfa hf).elim
      | n + 2 =>
        have : n + 2 ≤ 1 := by
          rw [hf] at hlea
          exact hlea
        omega
    have hsum_erase : s.sum f = (s.erase a).sum f := by
      rw [← Finset.sum_erase_add s f ha, fa0, Nat.add_zero]
    have hle_erase : (s.erase a).sum f ≤ (s.erase a).card := by
      have h1 :
          (s.erase a).sum f ≤ (s.erase a).sum fun _ => (1 : ℕ) :=
        Finset.sum_le_sum fun x hx => hle x (Finset.mem_of_mem_erase hx)
      -- card_eq_sum_ones : card = ∑ 1; avoid sum_const cast issues on pin
      have h2 : (s.erase a).sum (fun _ => (1 : ℕ)) = (s.erase a).card :=
        (Finset.card_eq_sum_ones (s.erase a)).symm
      exact h1.trans_eq h2
    have hpos : 0 < s.card := Finset.card_pos.mpr ⟨a, ha⟩
    have hcard_erase : (s.erase a).card = s.card - 1 := Finset.card_erase_of_mem ha
    have hcontra : s.card ≤ s.card - 1 := by
      calc
        s.card = s.sum f := hsum.symm
        _ = (s.erase a).sum f := hsum_erase
        _ ≤ (s.erase a).card := hle_erase
        _ = s.card - 1 := hcard_erase
    exact absurd hcontra (Nat.not_le.mpr (Nat.sub_lt hpos (by decide : (0 : ℕ) < 1)))

/-- **S4 / Johnson equality case:** packing + `|B| = 759` ⇒ every 5-set lies in
    exactly one block (Steiner covering property). -/
theorem packing_eq_implies_steiner
    (B : Finset (Finset MOGPoint))
    (hcard : ∀ b ∈ B, b.card = 8)
    (hpack : ∀ s : Finset MOGPoint, s.card = 5 →
      (B.filter (fun b => s ⊆ b)).card ≤ 1)
    (hB : B.card = 759) :
    ∀ s : Finset MOGPoint, s.card = 5 →
      ∃! b : Finset MOGPoint, b ∈ B ∧ s ⊆ b := by
  classical
  -- Global numerical equality from |B| = 759 and binomial identity
  have heq : B.card * Nat.choose 8 5 = Nat.choose 24 5 := by
    rw [hB]
    exact steiner_count_identity
  -- Left incidence count = |B| · C(8,5)
  have hleft :
      (B.sum fun b => (b.powersetCard 5).card) = B.card * Nat.choose 8 5 := by
    have h1 :
        (B.sum fun b => (b.powersetCard 5).card) =
          B.sum fun _ => Nat.choose 8 5 := by
      refine Finset.sum_congr rfl ?_
      intro b hb
      rw [Finset.card_powersetCard, hcard b hb]
    have h2 : (B.sum fun _ => Nat.choose 8 5) = B.card * Nat.choose 8 5 := by
      simp [Finset.sum_const, nsmul_eq_mul]
    exact h1.trans h2
  -- Fiber sum equals C(24,5)
  have hsum_fibers :
      (((Finset.univ : Finset MOGPoint).powersetCard 5).sum fun s =>
          (B.filter (fun b => s ⊆ b)).card) =
        Nat.choose 24 5 := by
    calc
      (((Finset.univ : Finset MOGPoint).powersetCard 5).sum fun s =>
            (B.filter (fun b => s ⊆ b)).card)
          = B.sum fun b => (b.powersetCard 5).card :=
        (sum_powersetCard_eq_sum_fibers B).symm
      _ = B.card * Nat.choose 8 5 := hleft
      _ = Nat.choose 24 5 := heq
  -- Domain of 5-sets has cardinality C(24,5)
  have hdom_card :
      ((Finset.univ : Finset MOGPoint).powersetCard 5).card = Nat.choose 24 5 := by
    simp [Finset.card_powersetCard, Finset.card_univ, Fintype.card_fin]
  -- Therefore every fiber has size exactly 1
  have hfiber_eq_one :
      ∀ s ∈ (Finset.univ : Finset MOGPoint).powersetCard 5,
        (B.filter (fun b => s ⊆ b)).card = 1 := by
    refine sum_le_one_eq_card_implies_eq_one
      ((Finset.univ : Finset MOGPoint).powersetCard 5)
      (fun s => (B.filter (fun b => s ⊆ b)).card)
      ?_
      ?_
    · intro s hs
      exact hpack s (Finset.mem_powersetCard.mp hs).2
    · rw [hsum_fibers, hdom_card]
  -- Convert card = 1 into ExistsUnique membership
  intro s hs
  have hs_mem : s ∈ (Finset.univ : Finset MOGPoint).powersetCard 5 := by
    simp [Finset.mem_powersetCard, hs]
  have h1 := hfiber_eq_one s hs_mem
  rw [Finset.card_eq_one] at h1
  obtain ⟨b, hbEq⟩ := h1
  refine ⟨b, ?_, ?_⟩
  · have hb_mem : b ∈ B.filter (fun b => s ⊆ b) := by
      rw [hbEq]
      exact Finset.mem_singleton_self b
    exact Finset.mem_filter.mp hb_mem
  · intro y hy
    have hy_mem : y ∈ B.filter (fun b => s ⊆ b) :=
      Finset.mem_filter.mpr hy
    rw [hbEq, Finset.mem_singleton] at hy_mem
    exact hy_mem

/-! ## 3. Concrete Golay instance (S5)

`golayOctadBlocks` is the image of the weight-8 Golay codewords under `maskToOctad`.
Cardinality 759 is `octad_count` plus S2 injectivity. Packing is the mask-level
intersection bound (distinct octads meet in ≤4 points) lifted to Finsets via
`maskToOctad (a &&& b) = maskToOctad a ∩ maskToOctad b`. With S4 this yields a
concrete Steiner system \(S(5,8,24)\) on the Golay octad family — still **not**
the MOG/`isMOGOctad` residual (CB-1) and **not** \(M_{24}\)/Monster/\(V^\natural\).
-/

/-- Bitwise AND of masks is intersection of supports. -/
theorem maskToOctad_inter (a b : Nat) :
    maskToOctad (a &&& b) = maskToOctad a ∩ maskToOctad b := by
  ext p
  simp only [mem_maskToOctad_iff, Finset.mem_inter, bitOn_and, Bool.and_eq_true]

noncomputable def golayOctadBlocks : Finset (Finset MOGPoint) :=
  (golayCode.filter (fun n => maskWeightN n = 8)).image maskToOctad

/-- **S5 (card):** weight-8 Golay codewords map injectively to 759 distinct octads. -/
theorem golayOctadBlocks_card : golayOctadBlocks.card = 759 := by
  classical
  unfold golayOctadBlocks
  have hinj :
      Set.InjOn maskToOctad (golayCode.filter (fun n => maskWeightN n = 8)) := by
    intro a ha b hb heq
    exact maskToOctad_injective_on_octads ha hb heq
  rw [Finset.card_image_of_injOn hinj, octad_count]

theorem golayOctadBlocks_members_card8 :
    ∀ b ∈ golayOctadBlocks, b.card = 8 := by
  intro b hb
  simp only [golayOctadBlocks, Finset.mem_image, Finset.mem_filter] at hb
  rcases hb with ⟨m, ⟨_, hw⟩, rfl⟩
  exact maskToOctad_card_of_weight_eight m hw

/-- **S5 (pack):** distinct Golay octads meet in ≤4 points ⇒ at most one block per 5-set. -/
theorem golayOctadBlocks_pack :
    ∀ s : Finset MOGPoint, s.card = 5 →
      (golayOctadBlocks.filter (fun b => s ⊆ b)).card ≤ 1 := by
  intro s hs
  classical
  rw [Finset.card_le_one]
  intro b1 hb1 b2 hb2
  obtain ⟨hb1B, hs1⟩ := Finset.mem_filter.mp hb1
  obtain ⟨hb2B, hs2⟩ := Finset.mem_filter.mp hb2
  simp only [golayOctadBlocks, Finset.mem_image] at hb1B hb2B
  rcases hb1B with ⟨a, ha, rfl⟩
  rcases hb2B with ⟨b, hb, rfl⟩
  -- If a ≠ b then |maskToOctad a ∩ maskToOctad b| ≤ 4, but s ⊆ both and |s| = 5.
  have hab : a = b := by
    by_contra hne
    have hsub : s ⊆ maskToOctad (a &&& b) := by
      intro p hp
      rw [maskToOctad_inter, Finset.mem_inter]
      exact ⟨hs1 hp, hs2 hp⟩
    have hge : 5 ≤ maskWeightN (a &&& b) := by
      have hle_card : s.card ≤ (maskToOctad (a &&& b)).card :=
        Finset.card_le_card hsub
      rw [hs, ← maskWeightN_eq_card_maskToOctad] at hle_card
      exact hle_card
    have hle : maskWeightN (a &&& b) ≤ 4 := by
      have hinter := octad_intersection_masks a ha b hb hne
      have hmem :
          maskWeightN (a &&& b) = 0 ∨
            maskWeightN (a &&& b) = 2 ∨ maskWeightN (a &&& b) = 4 := by
        simpa [Finset.mem_insert, Finset.mem_singleton] using hinter
      rcases hmem with h0 | h2 | h4 <;> omega
    exact (Nat.not_le.mpr (Nat.lt_of_le_of_lt hle (by decide : (4 : Nat) < 5))) hge
  rw [hab]

/-- Golay weight-8 supports form a Steiner system \(S(5,8,24)\) (S4 + S5). -/
theorem golay_octads_form_steiner :
    ∀ s : Finset MOGPoint, s.card = 5 →
      ∃! b : Finset MOGPoint, b ∈ golayOctadBlocks ∧ s ⊆ b := by
  intro s hs
  exact packing_eq_implies_steiner golayOctadBlocks
    golayOctadBlocks_members_card8 golayOctadBlocks_pack golayOctadBlocks_card s hs

/-- Weight-8 Golay masks ≃ golay octad blocks (Gemini `OctadEquiv` on the packing family). -/
noncomputable def octadMaskEquiv :
    { m : Nat // m ∈ golayCode.filter (fun n => maskWeightN n = 8) } ≃
      { s : Finset MOGPoint // s ∈ golayOctadBlocks } where
  toFun := fun ⟨m, hm⟩ =>
    ⟨maskToOctad m, by
      simp only [golayOctadBlocks, Finset.mem_image]
      exact ⟨m, hm, rfl⟩⟩
  invFun := fun ⟨s, hs⟩ => by
    classical
    -- s = maskToOctad m for unique m in the weight-8 Golay filter
    have : ∃ m, m ∈ golayCode.filter (fun n => maskWeightN n = 8) ∧ maskToOctad m = s := by
      simpa [golayOctadBlocks, Finset.mem_image] using hs
    exact ⟨Classical.choose this, (Classical.choose_spec this).1⟩
  left_inv := fun ⟨m, hm⟩ => by
    classical
    apply Subtype.ext
    simp only
    have hspec :=
      Classical.choose_spec
        (show ∃ m', m' ∈ golayCode.filter (fun n => maskWeightN n = 8) ∧
            maskToOctad m' = maskToOctad m by
          exact ⟨m, hm, rfl⟩)
    -- choose may differ but injectivity forces equality
    exact maskToOctad_injective_on_octads hspec.1 hm hspec.2
  right_inv := fun ⟨s, hs⟩ => by
    classical
    apply Subtype.ext
    simp only
    exact (Classical.choose_spec
      (show ∃ m, m ∈ golayCode.filter (fun n => maskWeightN n = 8) ∧ maskToOctad m = s by
        simpa [golayOctadBlocks, Finset.mem_image] using hs)).2

end K22.MOG.SteinerDoubleCount
