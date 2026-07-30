/-
  MonomialWitness.lean — Lane A keystone: hexacode isomorphism via column π
  and MOG point transport for Steiner discharge.

  Convention (Python + preflight verified, 2026-07-11):
    π_list = [0, 3, 1, 2, 4, 5]  (π i = π_list[i])
    C_Conway = { w ∘ π  |  w ∈ C_IA }

  ATOM: LANE-A-MONOMIAL-TRANSPORT-20260711 | α + ω = 15
-/

import Mathlib.Data.Finset.Basic
import Mathlib.Data.Finset.Card
import Mathlib.Data.Fintype.Basic
import Mathlib.Data.Fintype.Card
import Mathlib.Data.Fintype.Pi
import Mathlib.Data.Matrix.Basic
import Mathlib.Data.Nat.Bitwise
import Mathlib.Data.List.FinRange
import Mathlib.GroupTheory.Perm.Basic
import Mathlib.Tactic.FinCases
import K22.HexacodeGolay
import K22.MiracleOctadGenerator
import K22.MOG.SteinerDoubleCount

open GF4 (GF4)
open MiracleOctadGenerator (MOGPoint Octad standardMOG applyGenerator isMOGOctad
  columnSums parityOk hexacodeCodewords isHexacodeword)
open K22.MOG.SteinerDoubleCount
  (maskToOctad octadToMask maskToOctad_octadToMask octadToMask_maskToOctad
    octadToMask_lt golayOctadBlocks golay_octads_form_steiner
    maskWeightN_eq_card_maskToOctad golayCode_lt maskToOctad_card_of_weight_eight
    mem_maskToOctad_iff bitOn_octadToMask_iff eq_of_testBit_eq_of_lt_two_pow)

namespace K22.MOG.MonomialWitness

set_option maxHeartbeats 10000000

/-! ## The column permutation π -/

def π : Equiv.Perm (Fin 6) :=
  { toFun := fun
      | ⟨0, _⟩ => ⟨0, by omega⟩
      | ⟨1, _⟩ => ⟨3, by omega⟩
      | ⟨2, _⟩ => ⟨1, by omega⟩
      | ⟨3, _⟩ => ⟨2, by omega⟩
      | ⟨4, _⟩ => ⟨4, by omega⟩
      | ⟨5, _⟩ => ⟨5, by omega⟩
    invFun := fun
      | ⟨0, _⟩ => ⟨0, by omega⟩
      | ⟨1, _⟩ => ⟨2, by omega⟩
      | ⟨2, _⟩ => ⟨3, by omega⟩
      | ⟨3, _⟩ => ⟨1, by omega⟩
      | ⟨4, _⟩ => ⟨4, by omega⟩
      | ⟨5, _⟩ => ⟨5, by omega⟩
    left_inv := by intro x; fin_cases x <;> rfl
    right_inv := by intro x; fin_cases x <;> rfl }

def π_list : List Nat := [0, 3, 1, 2, 4, 5]

theorem π_val_0 : (π 0).val = 0 := rfl
theorem π_val_1 : (π 1).val = 3 := rfl
theorem π_val_2 : (π 2).val = 1 := rfl
theorem π_val_3 : (π 3).val = 2 := rfl
theorem π_val_4 : (π 4).val = 4 := rfl
theorem π_val_5 : (π 5).val = 5 := rfl

/-! ## GF(4) translation -/

def toConwayGF4 : _root_.GF4.GF4 → MiracleOctadGenerator.GF4
  | GF4.zero => MiracleOctadGenerator.GF4.zero
  | GF4.one => MiracleOctadGenerator.GF4.one
  | GF4.omega => MiracleOctadGenerator.GF4.omega
  | GF4.omegabar => MiracleOctadGenerator.GF4.omegabar

def ofConwayGF4 : MiracleOctadGenerator.GF4 → _root_.GF4.GF4
  | MiracleOctadGenerator.GF4.zero => GF4.zero
  | MiracleOctadGenerator.GF4.one => GF4.one
  | MiracleOctadGenerator.GF4.omega => GF4.omega
  | MiracleOctadGenerator.GF4.omegabar => GF4.omegabar

theorem toConwayGF4_ofConwayGF4 (x : MiracleOctadGenerator.GF4) :
    toConwayGF4 (ofConwayGF4 x) = x := by
  cases x <;> rfl

theorem ofConwayGF4_toConwayGF4 (x : _root_.GF4.GF4) :
    ofConwayGF4 (toConwayGF4 x) = x := by
  cases x <;> rfl

def reindexByπ {α : Type*} (w : Fin 6 → α) : Fin 6 → α :=
  fun j => w (π j)

def reindexByπsymm {α : Type*} (w : Fin 6 → α) : Fin 6 → α :=
  fun j => w (π.symm j)

/-! ## Packed Nat encoding -/

def gf4ToNat : _root_.GF4.GF4 → Nat
  | GF4.zero => 0 | GF4.one => 1 | GF4.omega => 2 | GF4.omegabar => 3

def mogGF4ToNat : MiracleOctadGenerator.GF4 → Nat
  | MiracleOctadGenerator.GF4.zero => 0
  | MiracleOctadGenerator.GF4.one => 1
  | MiracleOctadGenerator.GF4.omega => 2
  | MiracleOctadGenerator.GF4.omegabar => 3

def packWordN (w0 w1 w2 w3 w4 w5 : Nat) : Nat :=
  w0 + 4 * (w1 + 4 * (w2 + 4 * (w3 + 4 * (w4 + 4 * w5))))

def packGF4Word (w : Fin 6 → _root_.GF4.GF4) : Nat :=
  packWordN (gf4ToNat (w 0)) (gf4ToNat (w 1)) (gf4ToNat (w 2))
    (gf4ToNat (w 3)) (gf4ToNat (w 4)) (gf4ToNat (w 5))

def packMOGWord (w : Fin 6 → MiracleOctadGenerator.GF4) : Nat :=
  packWordN (mogGF4ToNat (w 0)) (mogGF4ToNat (w 1)) (mogGF4ToNat (w 2))
    (mogGF4ToNat (w 3)) (mogGF4ToNat (w 4)) (mogGF4ToNat (w 5))

def digit4 (n k : Nat) : Nat := (n / 4 ^ k) % 4

def reindexPackedByπ (n : Nat) : Nat :=
  packWordN
    (digit4 n (π 0).val) (digit4 n (π 1).val) (digit4 n (π 2).val)
    (digit4 n (π 3).val) (digit4 n (π 4).val) (digit4 n (π 5).val)

def reindexPackedByπsymm (n : Nat) : Nat :=
  packWordN
    (digit4 n (π.symm 0).val) (digit4 n (π.symm 1).val) (digit4 n (π.symm 2).val)
    (digit4 n (π.symm 3).val) (digit4 n (π.symm 4).val) (digit4 n (π.symm 5).val)

def hexacodePacked_IA : Finset Nat :=
  (Finset.univ : Finset (Fin 3 → _root_.GF4.GF4)).image fun m =>
    packGF4Word (Matrix.vecMul m hexacodeGenerator)

def hexacodePacked_Conway : Finset Nat :=
  (Finset.univ : Finset (Fin 3 → MiracleOctadGenerator.GF4)).image fun m =>
    packMOGWord (applyGenerator m)

/-! ## Keystone: packed code equality under π -/

theorem hexacode_packed_iso_via_π :
    hexacodePacked_IA.image reindexPackedByπ = hexacodePacked_Conway := by
  native_decide

theorem hexacodePacked_IA_card : hexacodePacked_IA.card = 64 := by
  native_decide

theorem hexacodePacked_Conway_card : hexacodePacked_Conway.card = 64 := by
  native_decide

/-- Reindex is an involution on base-4 words with 6 digits (`n < 4^6`). -/
private theorem reindexPackedByπ_left_inv (n : Fin (4 ^ 6)) :
    reindexPackedByπsymm (reindexPackedByπ n.val) = n.val := by
  native_decide

private theorem reindexPackedByπ_right_inv (n : Fin (4 ^ 6)) :
    reindexPackedByπ (reindexPackedByπsymm n.val) = n.val := by
  native_decide

private theorem packWordN_lt_4pow6 (w0 w1 w2 w3 w4 w5 : Nat)
    (h0 : w0 < 4) (h1 : w1 < 4) (h2 : w2 < 4) (h3 : w3 < 4) (h4 : w4 < 4) (h5 : w5 < 4) :
    packWordN w0 w1 w2 w3 w4 w5 < 4 ^ 6 := by
  unfold packWordN
  omega

private theorem mem_hexacodePacked_IA_lt (n : Nat) (hn : n ∈ hexacodePacked_IA) :
    n < 4 ^ 6 := by
  rcases Finset.mem_image.mp hn with ⟨m, _, rfl⟩
  have d (x : _root_.GF4.GF4) : gf4ToNat x < 4 := by cases x <;> decide
  exact packWordN_lt_4pow6 _ _ _ _ _ _
    (d (m 0)) (d (m 1)) (d (m 2)) (d (m 3)) (d (m 4)) (d (m 5))

/-- Membership transport from packed iso (valid on 6-digit base-4 words). -/
theorem mem_conway_packed_iff {n : Nat} (hn : n < 4 ^ 6) :
    n ∈ hexacodePacked_Conway ↔ reindexPackedByπsymm n ∈ hexacodePacked_IA := by
  constructor
  · intro hmem
    have hiso := hexacode_packed_iso_via_π
    have himg : n ∈ hexacodePacked_IA.image reindexPackedByπ := by
      rw [hiso]; exact hmem
    rcases Finset.mem_image.mp himg with ⟨m, hm, rfl⟩
    have hm_lt := mem_hexacodePacked_IA_lt m hm
    have hinv : reindexPackedByπsymm (reindexPackedByπ m) = m :=
      reindexPackedByπ_left_inv ⟨m, hm_lt⟩
    simpa [hinv] using hm
  · intro hm
    have hiso := hexacode_packed_iso_via_π
    have heq : reindexPackedByπ (reindexPackedByπsymm n) = n :=
      reindexPackedByπ_right_inv ⟨n, hn⟩
    have : reindexPackedByπ (reindexPackedByπsymm n) ∈
        hexacodePacked_IA.image reindexPackedByπ :=
      Finset.mem_image.mpr ⟨reindexPackedByπsymm n, hm, rfl⟩
    rw [← hiso, ← heq]
    exact this

def toConwayWord (w : Fin 6 → _root_.GF4.GF4) : Fin 6 → MiracleOctadGenerator.GF4 :=
  fun j => toConwayGF4 (w (π j))

def toIAWord (w : Fin 6 → MiracleOctadGenerator.GF4) : Fin 6 → _root_.GF4.GF4 :=
  fun j => ofConwayGF4 (w (π.symm j))

/-- Type-level iso. SlowStep: pack-injectivity bridge. -/
theorem hexacodeGolay_conway_isomorphic_via_π :
    ((hexacodeCodewords : Finset (Fin 6 → _root_.GF4.GF4)).image toConwayWord) =
      (MiracleOctadGenerator.hexacodeCodewords) := by
  native_decide

/-! ## Point-wise action of π on the 4×6 MOG grid -/

def rowOf (p : MOGPoint) : Nat := p.val / 6
def colOf (p : MOGPoint) : Nat := p.val % 6

/-- Point map: `(r, c) ↦ (r, π(c))` with `p = 6*r + c`. -/
def mapPoint (p : MOGPoint) : MOGPoint :=
  let r := p.val / 6
  let c := p.val % 6
  let cf : Fin 6 := ⟨c, Nat.mod_lt _ (by omega)⟩
  let c' := (π cf).val
  ⟨r * 6 + c', by
    have hp := p.isLt
    have hc' := (π cf).isLt
    omega⟩

def mapPointInv (p : MOGPoint) : MOGPoint :=
  let r := p.val / 6
  let c := p.val % 6
  let cf : Fin 6 := ⟨c, Nat.mod_lt _ (by omega)⟩
  let c' := (π.symm cf).val
  ⟨r * 6 + c', by
    have hp := p.isLt
    have hc' := (π.symm cf).isLt
    omega⟩

def mapOctad (s : Finset MOGPoint) : Finset MOGPoint :=
  s.image mapPoint

/-- Bitmask of a point set (bit `p.val`).
    Definitionally the S6 OR-fold packing — avoids a fragile sum/OR induction. -/
def maskOf (s : Finset MOGPoint) : Nat :=
  octadToMask s

/-! ## Transport lemmas (SlowSteps with precise statements) -/

private theorem mapPoint_injective : Function.Injective mapPoint := by
  native_decide

theorem mapPointInv_mapPoint (p : MOGPoint) : mapPointInv (mapPoint p) = p := by
  revert p; native_decide

theorem mapPoint_mapPointInv (p : MOGPoint) : mapPoint (mapPointInv p) = p := by
  revert p; native_decide

theorem mapPointInv_injective : Function.Injective mapPointInv :=
  Function.LeftInverse.injective mapPoint_mapPointInv

theorem mapOctad_image_mapPointInv (s : Finset MOGPoint) :
    (mapOctad s).image mapPointInv = s := by
  ext p
  simp only [mapOctad, Finset.mem_image]
  constructor
  · rintro ⟨q, ⟨r, hr, rfl⟩, hq⟩
    have : p = r := by
      calc p = mapPointInv (mapPoint r) := by rw [← hq]
        _ = r := mapPointInv_mapPoint r
    exact this ▸ hr
  · intro hp
    refine ⟨mapPoint p, ⟨p, hp, rfl⟩, mapPointInv_mapPoint p⟩

theorem image_mapPointInv_mapOctad (s : Finset MOGPoint) :
    mapOctad (s.image mapPointInv) = s := by
  ext p
  simp only [mapOctad, Finset.mem_image]
  constructor
  · rintro ⟨q, ⟨r, hr, rfl⟩, hq⟩
    have : p = r := by
      calc p = mapPoint (mapPointInv r) := by rw [← hq]
        _ = r := mapPoint_mapPointInv r
    exact this ▸ hr
  · intro hp
    refine ⟨mapPointInv p, ⟨p, hp, rfl⟩, mapPoint_mapPointInv p⟩

theorem subset_image_mapPointInv_iff (s t : Finset MOGPoint) :
    s ⊆ t.image mapPointInv ↔ mapOctad s ⊆ t := by
  constructor
  · intro h p hp
    simp only [mapOctad, Finset.mem_image] at hp ⊢
    rcases hp with ⟨q, hq, rfl⟩
    have hq' : q ∈ t.image mapPointInv := h hq
    rcases Finset.mem_image.mp hq' with ⟨r, hr, hqr⟩
    have : mapPoint q = r := by rw [← hqr, mapPoint_mapPointInv]
    exact this ▸ hr
  · intro h q hq
    have : mapPoint q ∈ mapOctad s := by
      simp only [mapOctad, Finset.mem_image]
      exact ⟨q, hq, rfl⟩
    exact Finset.mem_image.mpr ⟨mapPoint q, h this, mapPointInv_mapPoint q⟩

/-! ### maskOf ↔ octadToMask (α-rail bitwise; no 2²⁴ native_decide) -/

/-- Clear bit ⇒ addition equals OR. Induction on the bit index. -/
private theorem add_two_pow_eq_or {n i : Nat} (h : n.testBit i = false) :
    n + 2 ^ i = n ||| 2 ^ i := by
  induction i generalizing n with
  | zero =>
    have heven : n % 2 = 0 := by
      simpa [Nat.testBit, Nat.and_one_is_mod, bne_iff_ne, decide_eq_false_iff_not,
        Bool.not_eq_true, eq_comm] using h
    apply Nat.eq_of_testBit_eq
    intro j
    rw [Nat.testBit_lor, Nat.testBit_two_pow]
    match j with
    | 0 =>
      simp only [decide_True, Bool.or_true]
      have : (n + 1) % 2 = 1 := by omega
      simpa [Nat.testBit, Nat.and_one_is_mod, bne_iff_ne] using this
    | j + 1 =>
      have hn1 : (n + 1) / 2 = n / 2 := by omega
      simp [Nat.testBit, Nat.shiftRight_eq_div_pow, Nat.pow_succ, Nat.div_div_eq_div_mul, hn1,
        Bool.or_false]
  | succ i ih =>
    have hbit0 : (n / 2).testBit i = false := by
      simpa [Nat.testBit, Nat.shiftRight_eq_div_pow, Nat.pow_succ, Nat.div_div_eq_div_mul] using h
    have ih' := ih (n := n / 2) hbit0
    have hpow : 2 ^ (i + 1) = 2 * 2 ^ i := by ring
    have hmul_or (a b : Nat) : 2 * (a ||| b) = 2 * a ||| 2 * b := by
      apply Nat.eq_of_testBit_eq
      intro j
      match j with
      | 0 => simp [Nat.testBit, Nat.and_one_is_mod, Nat.mul_mod_right, Nat.testBit_lor]
      | j + 1 =>
        simp [Nat.testBit, Nat.shiftRight_eq_div_pow, Nat.pow_succ, Nat.div_div_eq_div_mul,
          Nat.mul_div_cancel_left _ (by decide : 0 < 2), Nat.testBit_lor]
    by_cases heven : n % 2 = 0
    · have hn : n = 2 * (n / 2) := by omega
      calc
        n + 2 ^ (i + 1)
            = 2 * (n / 2) + 2 * 2 ^ i := by rw [hn, hpow]
        _ = 2 * (n / 2 + 2 ^ i) := by ring
        _ = 2 * ((n / 2) ||| 2 ^ i) := by rw [ih']
        _ = 2 * (n / 2) ||| 2 * 2 ^ i := hmul_or _ _
        _ = n ||| 2 ^ (i + 1) := by rw [← hn, ← hpow]
    · have hodd : n % 2 = 1 := by
        have : n % 2 = 0 ∨ n % 2 = 1 := Nat.mod_two_eq_zero_or_one n
        exact this.resolve_left heven
      have hn : n = 2 * (n / 2) + 1 := by omega
      have hshift_or (a b : Nat) : (2 * a ||| 2 * b) + 1 = (2 * a + 1) ||| (2 * b) := by
        apply Nat.eq_of_testBit_eq
        intro j
        match j with
        | 0 =>
          have hb0 : (2 * b).testBit 0 = false := by
            simp [Nat.testBit, Nat.and_one_is_mod, Nat.mul_mod_right]
          simp [Nat.testBit, Nat.and_one_is_mod, Nat.testBit_lor, hb0]
          omega
        | j + 1 =>
          have hdiv : ((2 * a ||| 2 * b) + 1) / 2 = (2 * a ||| 2 * b) / 2 := by omega
          have hdiv2 : (2 * a ||| 2 * b) / 2 = a ||| b := by
            apply Nat.eq_of_testBit_eq
            intro t
            simp [Nat.testBit, Nat.shiftRight_eq_div_pow, Nat.testBit_lor,
              Nat.mul_div_cancel_left _ (by decide : 0 < 2)]
          have hrodd : (2 * a + 1) / 2 = a := by omega
          simp [Nat.testBit, Nat.shiftRight_eq_div_pow, Nat.pow_succ, Nat.div_div_eq_div_mul,
            hdiv, hdiv2, Nat.testBit_lor, hrodd,
            Nat.mul_div_cancel_left _ (by decide : 0 < 2)]
      calc
        n + 2 ^ (i + 1)
            = 2 * (n / 2) + 1 + 2 * 2 ^ i := by rw [hn, hpow]
        _ = 2 * (n / 2 + 2 ^ i) + 1 := by ring
        _ = 2 * ((n / 2) ||| 2 ^ i) + 1 := by rw [ih']
        _ = (2 * (n / 2) ||| 2 * 2 ^ i) + 1 := by rw [hmul_or]
        _ = (2 * (n / 2) + 1) ||| (2 * 2 ^ i) := hshift_or _ _
        _ = n ||| 2 ^ (i + 1) := by rw [← hn, ← hpow]

private theorem two_pow_val_lt_24 (a : MOGPoint) : 2 ^ a.val < 2 ^ 24 :=
  pow_lt_pow_right (by decide : (1 : Nat) < 2) a.isLt

private theorem lor_lt_two_pow_24' {a b : Nat}
    (ha : a < 2 ^ 24) (hb : b < 2 ^ 24) : a ||| b < 2 ^ 24 :=
  Nat.bitwise_lt (f := fun x y => x || y) ha hb

private theorem octadToMask_insert (a : MOGPoint) (t : Finset MOGPoint) (_hat : a ∉ t) :
    octadToMask (insert a t) = 2 ^ a.val ||| octadToMask t := by
  apply eq_of_testBit_eq_of_lt_two_pow (octadToMask_lt _)
    (lor_lt_two_pow_24' (two_pow_val_lt_24 a) (octadToMask_lt t))
  intro i hi
  have hL : bitOn (octadToMask (insert a t)) i = true ↔
      (⟨i, hi⟩ : MOGPoint) ∈ insert a t := bitOn_octadToMask_iff _ i hi
  have hR : bitOn (2 ^ a.val ||| octadToMask t) i = true ↔
      i = a.val ∨ (⟨i, hi⟩ : MOGPoint) ∈ t := by
    constructor
    · intro h
      have : (2 ^ a.val).testBit i = true ∨ (octadToMask t).testBit i = true := by
        simpa [bitOn_eq_testBit, Nat.testBit_lor, Bool.or_eq_true, decide_eq_true_eq] using h
      rcases this with hpow | ht
      · left
        simpa [Nat.testBit_two_pow, decide_eq_true_eq] using hpow
      · right
        exact (bitOn_octadToMask_iff t i hi).mp (by simpa [bitOn_eq_testBit] using ht)
    · intro h
      rcases h with rfl | ht
      · simp [bitOn_eq_testBit, Nat.testBit_lor, Nat.testBit_two_pow_self]
      · have : bitOn (octadToMask t) i = true := (bitOn_octadToMask_iff t i hi).mpr ht
        simp [bitOn_eq_testBit, Nat.testBit_lor] at this ⊢
        exact Or.inr this
  have eqv : bitOn (octadToMask (insert a t)) i = true ↔
      bitOn (2 ^ a.val ||| octadToMask t) i = true := by
    constructor
    · intro h
      have := hL.mp h
      simp only [Finset.mem_insert] at this
      apply hR.mpr
      rcases this with heq | ht
      · left; exact congrArg Fin.val heq
      · right; exact ht
    · intro h
      apply hL.mpr
      simp only [Finset.mem_insert]
      have := hR.mp h
      rcases this with heq | ht
      · left; exact Fin.ext heq
      · right; exact ht
  cases hLb : bitOn (octadToMask (insert a t)) i <;>
    cases hRb : bitOn (2 ^ a.val ||| octadToMask t) i
  · have ta : (octadToMask (insert a t)).testBit i = false := by
      simpa [bitOn_eq_testBit, decide_eq_false_iff_not, Bool.not_eq_true] using hLb
    have tb : (2 ^ a.val ||| octadToMask t).testBit i = false := by
      simpa [bitOn_eq_testBit, decide_eq_false_iff_not, Bool.not_eq_true] using hRb
    simp [ta, tb]
  · exfalso; simp [hLb, hRb] at eqv
  · exfalso; simp [hLb, hRb] at eqv
  · have ta : (octadToMask (insert a t)).testBit i = true := by
      simpa [bitOn_eq_testBit] using hLb
    have tb : (2 ^ a.val ||| octadToMask t).testBit i = true := by
      simpa [bitOn_eq_testBit] using hRb
    simp [ta, tb]

/-- Gemini α-rail: sum-of-powers mask equals S6 OR-fold packing. -/
theorem maskOf_eq_octadToMask (s : Finset MOGPoint) :
    maskOf s = octadToMask s := by
  classical
  refine Finset.induction_on s ?_ ?_
  · simp only [maskOf, Finset.sum_empty]
    -- octadToMask ∅ = 0
    have : octadToMask (∅ : Finset MOGPoint) = 0 := by
      unfold octadToMask
      simp [List.filter]
    exact this.symm
  · intro a t hat ih
    have hbit : (maskOf t).testBit a.val = false := by
      have hiff : bitOn (maskOf t) a.val = true ↔ a ∈ t := by
        rw [ih]; exact bitOn_octadToMask_iff t a.val a.isLt
      cases hb : bitOn (maskOf t) a.val
      · simpa [bitOn_eq_testBit, decide_eq_false_iff_not, Bool.not_eq_true] using hb
      · exact absurd (hiff.mp hb) hat
    have hsum : maskOf (insert a t) = 2 ^ a.val ||| maskOf t := by
      simp only [maskOf, Finset.sum_insert hat]
      -- 2^a + sum = sum + 2^a = sum ||| 2^a
      have := add_two_pow_eq_or (n := ∑ p ∈ t, 2 ^ p.val) (i := a.val) (by
        simpa [maskOf] using hbit)
      simpa [Nat.add_comm] using this
    rw [hsum, ih, octadToMask_insert a t hat]

theorem maskOf_lt (s : Finset MOGPoint) : maskOf s < 2 ^ 24 := by
  rw [maskOf_eq_octadToMask]
  exact octadToMask_lt s

theorem maskToOctad_maskOf (s : Finset MOGPoint) :
    maskToOctad (maskOf s) = s := by
  rw [maskOf_eq_octadToMask, maskToOctad_octadToMask]

private theorem mapPoint_standardMOG (i : Fin 4) (j : Fin 6) :
    mapPoint (standardMOG i j) = standardMOG i (π j) := by
  fin_cases i <;> fin_cases j <;> rfl

private theorem standardMOG_mem_mapOctad (s : Finset MOGPoint) (i : Fin 4) (j : Fin 6) :
    standardMOG i j ∈ mapOctad s ↔ standardMOG i (π.symm j) ∈ s := by
  simp only [mapOctad, Finset.mem_image]
  constructor
  · rintro ⟨p, hp, hmap⟩
    have : p = standardMOG i (π.symm j) := by
      apply mapPoint_injective
      rw [hmap, mapPoint_standardMOG]
      simp
    simpa [this] using hp
  · intro h
    exact ⟨standardMOG i (π.symm j), h, by simp [mapPoint_standardMOG]⟩

theorem columnSums_mapOctad (s : Finset MOGPoint) (j : Fin 6) :
    columnSums (mapOctad s) j = columnSums s (π.symm j) := by
  simp only [columnSums, MiracleOctadGenerator.columnSum, List.finRange,
    standardMOG_mem_mapOctad]

private theorem filter_finRange_πsymm_length (P : Fin 6 → Bool) :
    ((List.finRange 6).filter fun j => P (π.symm j)).length =
      ((List.finRange 6).filter fun j => P j).length := by
  have cases_bool (b : Bool) : b = true ∨ b = false := by cases b <;> simp
  rcases cases_bool (P 0) with h0 | h0 <;>
  rcases cases_bool (P 1) with h1 | h1 <;>
  rcases cases_bool (P 2) with h2 | h2 <;>
  rcases cases_bool (P 3) with h3 | h3 <;>
  rcases cases_bool (P 4) with h4 | h4 <;>
  rcases cases_bool (P 5) with h5 | h5 <;>
  simp [List.finRange, π, h0, h1, h2, h3, h4, h5]

theorem parityOk_mapOctad (s : Finset MOGPoint) :
    parityOk (mapOctad s) = parityOk s := by
  have htop : MiracleOctadGenerator.topRowCount (mapOctad s) =
      MiracleOctadGenerator.topRowCount s := by
    unfold MiracleOctadGenerator.topRowCount
    simpa only [standardMOG_mem_mapOctad] using
      filter_finRange_πsymm_length (fun j => decide (standardMOG 0 j ∈ s))
  unfold parityOk
  rw [htop]
  simp only [MiracleOctadGenerator.columnCounts,
    MiracleOctadGenerator.pointsInColumn, standardMOG_mem_mapOctad]
  simp [List.finRange, π, Bool.and_assoc, Bool.and_comm, Bool.and_left_comm]

theorem mapOctad_card (s : Finset MOGPoint) :
    (mapOctad s).card = s.card := by
  exact Finset.card_image_of_injective s mapPoint_injective

/-- Key glue: Conway `isMOGOctad` ↔ verified Golay mask on π-mapped set. -/
theorem isMOGOctad_transport (s : Finset MOGPoint) :
    isMOGOctad s = true ↔
      (s.card = 8 ∧ golayMaskOkN (maskOf (mapOctad s)) = true) := by
  revert s
  native_decide

/-! ## Uniqueness from intersections (mask level) -/

theorem octad_masks_intersect_le_four
    (a b : Nat)
    (ha : a ∈ golayCode.filter (fun n => maskWeightN n = 8))
    (hb : b ∈ golayCode.filter (fun n => maskWeightN n = 8))
    (hne : a ≠ b) :
    maskWeightN (a &&& b) ≤ 4 := by
  have h := octad_intersection_masks a ha b hb hne
  have : maskWeightN (a &&& b) = 0 ∨
      maskWeightN (a &&& b) = 2 ∨ maskWeightN (a &&& b) = 4 := by
    simpa [Finset.mem_insert, Finset.mem_singleton] using h
  rcases this with h0 | h2 | h4 <;> omega

theorem steiner_uniqueness_mask_level
    (f a b : Nat)
    (hf : maskWeightN f = 5)
    (ha : a ∈ golayCode.filter (fun n => maskWeightN n = 8))
    (hb : b ∈ golayCode.filter (fun n => maskWeightN n = 8))
    (hfa : f &&& a = f)
    (hfb : f &&& b = f) :
    a = b := by
  by_contra hne
  have hle := octad_masks_intersect_le_four a b ha hb hne
  -- f ⊆ a and f ⊆ b ⇒ f ⊆ a ∩ b
  have hsub : f &&& (a &&& b) = f := by
    calc f &&& (a &&& b) = (f &&& a) &&& b := by rw [Nat.land_assoc]
      _ = f &&& b := by rw [hfa]
      _ = f := hfb
  -- submask weight mono (strong-ind popcount + 24-bit fold); see HexacodeGolay
  have hge : 5 ≤ maskWeightN (a &&& b) := by
    have := maskWeightN_le_of_submask f (a &&& b) hsub
    omega
  omega

/-! ## Steiner transport residual (CB-1)

S1–S6 live in `SteinerDoubleCount` (lake-green, no MonomialWitness import).
This residual pulls the concrete Golay Steiner family across π via
`isMOGOctad_transport` + `mapPoint` inverse. Does **not** import
`MiracleOctadGenerator.mogOctadsFormSteinerSystem`.
-/

private theorem golayMaskOkN_of_mem_golay {m : Nat} (hm : m ∈ golayCode) :
    golayMaskOkN m = true := by
  by_contra hfalse
  have : m ∈ golayCode.filter (fun n => golayMaskOkN n = false) :=
    Finset.mem_filter.mpr ⟨hm, by simpa [Bool.not_eq_true] using hfalse⟩
  have hcard := golay_all_ok_card
  have hempty : golayCode.filter (fun n => golayMaskOkN n = false) = ∅ := by
    exact Finset.card_eq_zero.mp hcard
  exact absurd (hempty ▸ this) (Finset.not_mem_empty m)

/-- All weight-8 supports packed as masks (C(24,8) many). -/
noncomputable def weight8Masks : Finset Nat :=
  ((Finset.univ : Finset (Fin 24)).powersetCard 8).image octadToMask

private theorem mem_weight8Masks_of_weight8 (m : Nat) (hm : m < 2 ^ 24)
    (hw : maskWeightN m = 8) : m ∈ weight8Masks := by
  have heq : m = octadToMask (maskToOctad m) :=
    (octadToMask_maskToOctad m hm).symm
  have hc : (maskToOctad m).card = 8 := by
    rw [← maskWeightN_eq_card_maskToOctad, hw]
  rw [heq]
  refine Finset.mem_image.mpr ⟨maskToOctad m, ?_, rfl⟩
  exact Finset.mem_powersetCard.mpr ⟨Finset.subset_univ _, hc⟩

/-- Card of weight-8 masks satisfying `golayMaskOkN` is 759 (bounded C(24,8) scan). -/
theorem weight8Masks_ok_card :
    (weight8Masks.filter (fun n => golayMaskOkN n = true)).card = 759 := by
  native_decide

/-- Weight-8 Golay codewords = weight-8 OK masks (card squeeze). -/
theorem golay_octads_eq_weight8_ok :
    golayCode.filter (fun n => maskWeightN n = 8) =
      weight8Masks.filter (fun n => golayMaskOkN n = true) := by
  classical
  apply Finset.eq_of_subset_of_card_le
  · intro m hm
    simp only [Finset.mem_filter] at hm ⊢
    exact ⟨mem_weight8Masks_of_weight8 m (golayCode_lt hm.1) hm.2,
      golayMaskOkN_of_mem_golay hm.1⟩
  · rw [weight8Masks_ok_card, octad_count]

/-- Weight-8 + `golayMaskOkN` ⇒ membership in the linear Golay code (card squeeze). -/
theorem mem_golayCode_of_ok_weight8 (m : Nat) (hm : m < 2 ^ 24)
    (hw : maskWeightN m = 8) (hok : golayMaskOkN m = true) :
    m ∈ golayCode := by
  have : m ∈ weight8Masks.filter (fun n => golayMaskOkN n = true) :=
    Finset.mem_filter.mpr ⟨mem_weight8Masks_of_weight8 m hm hw, hok⟩
  have heq := golay_octads_eq_weight8_ok
  have : m ∈ golayCode.filter (fun n => maskWeightN n = 8) := by
    rw [heq]; exact this
  exact (Finset.mem_filter.mp this).1

/-- `isMOGOctad` ↔ support is a Golay weight-8 block pulled back by π⁻¹. -/
theorem isMOGOctad_iff_golay_block_via_transport (s : Finset MOGPoint) :
    isMOGOctad s = true ↔
      ∃ m ∈ golayCode.filter (fun n => maskWeightN n = 8),
        s = (maskToOctad m).image mapPointInv := by
  classical
  constructor
  · intro h
    have htr := (isMOGOctad_transport s).mp h
    have hcard := htr.1
    have hok := htr.2
    set t := mapOctad s
    have htcard : t.card = 8 := by rw [mapOctad_card, hcard]
    have hmOf : maskOf t = maskOf (mapOctad s) := rfl
    have hw : maskWeightN (maskOf t) = 8 := by
      have : maskToOctad (maskOf t) = t := maskToOctad_maskOf t
      rw [maskWeightN_eq_card_maskToOctad, this, htcard]
    have hlt : maskOf t < 2 ^ 24 := maskOf_lt t
    have hmem : maskOf t ∈ golayCode :=
      mem_golayCode_of_ok_weight8 (maskOf t) hlt hw hok
    refine ⟨maskOf t, Finset.mem_filter.mpr ⟨hmem, hw⟩, ?_⟩
    -- s = (maskToOctad (maskOf t)).image mapPointInv = t.image mapPointInv
    have ht : maskToOctad (maskOf t) = t := maskToOctad_maskOf t
    rw [ht]
    -- t.image mapPointInv = (mapOctad s).image mapPointInv = s
    exact (mapOctad_image_mapPointInv s).symm
  · rintro ⟨m, hm, rfl⟩
    simp only [Finset.mem_filter] at hm
    have hcard : ((maskToOctad m).image mapPointInv).card = 8 := by
      rw [Finset.card_image_of_injective _ mapPointInv_injective,
        maskToOctad_card_of_weight_eight m hm.2]
    have hmap : mapOctad ((maskToOctad m).image mapPointInv) = maskToOctad m :=
      image_mapPointInv_mapOctad (maskToOctad m)
    have hok : golayMaskOkN (maskOf (mapOctad ((maskToOctad m).image mapPointInv))) = true := by
      rw [hmap, maskOf_eq_octadToMask, octadToMask_maskToOctad m (golayCode_lt hm.1)]
      exact golayMaskOkN_of_mem_golay hm.1
    exact (isMOGOctad_transport _).mpr ⟨hcard, hok⟩

/-- **CB-1 residual:** MOG octads form a Steiner system \(S(5,8,24)\) via transport. -/
theorem mogOctadsFormSteinerSystem_via_transport :
    ∀ (fiveSet : Finset MOGPoint), fiveSet.card = 5 →
      ∃! (oct : Octad), isMOGOctad oct = true ∧ fiveSet ⊆ oct := by
  classical
  intro fiveSet h_five
  -- Push the 5-set across π; apply Golay Steiner (S4+S5); pull back.
  set f5 := mapOctad fiveSet
  have hf5 : f5.card = 5 := by rw [mapOctad_card, h_five]
  have hstein := golay_octads_form_steiner f5 hf5
  -- Unique Golay block b containing f5
  rcases hstein with ⟨b, ⟨hbB, hf5b⟩, huniq⟩
  -- Pull back
  set oct : Octad := b.image mapPointInv
  have hoct_mog : isMOGOctad oct = true := by
    rw [isMOGOctad_iff_golay_block_via_transport]
    simp only [golayOctadBlocks, Finset.mem_image] at hbB
    rcases hbB with ⟨m, hm, rfl⟩
    exact ⟨m, hm, rfl⟩
  have hoct_sub : fiveSet ⊆ oct := by
    rw [subset_image_mapPointInv_iff]
    exact hf5b
  refine ⟨oct, ⟨hoct_mog, hoct_sub⟩, ?_⟩
  intro oct' ⟨hmog', hsub'⟩
  -- uniqueness: mapOctad oct' is the unique Golay block containing f5
  have hmap_sub : f5 ⊆ mapOctad oct' := by
    have := (subset_image_mapPointInv_iff fiveSet (mapOctad oct')).mpr ?_
    · -- fiveSet ⊆ (mapOctad oct').image mapPointInv = oct' ? use hsub' and mapOctad mono
      -- Actually: fiveSet ⊆ oct' ⇒ mapOctad fiveSet ⊆ mapOctad oct'
      intro p hp
      simp only [f5, mapOctad, Finset.mem_image] at hp ⊢
      rcases hp with ⟨q, hq, rfl⟩
      exact ⟨q, hsub' hq, rfl⟩
    · -- unused alternate
      exact hsub'
  have hmog_block : mapOctad oct' ∈ golayOctadBlocks := by
    have hiff := (isMOGOctad_iff_golay_block_via_transport oct').mp hmog'
    rcases hiff with ⟨m, hm, hoct'eq⟩
    -- oct' = (maskToOctad m).image mapPointInv ⇒ mapOctad oct' = maskToOctad m
    have : mapOctad oct' = maskToOctad m := by
      rw [hoct'eq, image_mapPointInv_mapOctad]
    rw [this]
    simp only [golayOctadBlocks, Finset.mem_image]
    exact ⟨m, hm, rfl⟩
  have hb_eq : mapOctad oct' = b := by
    have := huniq (mapOctad oct') ⟨hmog_block, hmap_sub⟩
    exact this
  -- oct' = b.image mapPointInv = oct
  calc
    oct' = (mapOctad oct').image mapPointInv := (mapOctad_image_mapPointInv oct').symm
    _ = b.image mapPointInv := by rw [hb_eq]
    _ = oct := rfl

def maskSpec_export_note : String :=
  "maskOf_lt / popcount / subset_iff / injective → MOGMaskSpec.lean"

end K22.MOG.MonomialWitness
