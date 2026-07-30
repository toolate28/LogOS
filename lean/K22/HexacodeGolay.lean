/-
  HexacodeGolay.lean

  Verified algebraic core for the Miracle Octad Generator (MOG):

  * `GF4` — the field GF(4) as an explicit inductive type with a genuine
    `Field` instance (all axioms discharged by `decide`).
  * `hexacodeGenerator` — the standard 3×6 generator matrix of the hexacode,
    the enumeration `hexacodeCodewords` of all 64 codewords, and an exact
    membership test `isHexacodeword` (a GF(4) parity check `H · wᵀ = 0`).
  * The binary Golay code [24,12,8] realised via the MOG rule, with the
    octad count (759) and the full weight enumerator verified by
    `native_decide`.  The Golay verification runs on a pure `Nat`/`Bool`
    computation engine (`golayMaskOkN`) that mirrors the GF(4) definitions
    but avoids a `native_decide` reflection issue on this toolchain.

  Everything in this file is fully proved; there are no `sorry`s.
-/

import Mathlib.Data.Matrix.Basic
import Mathlib.Algebra.Field.Basic
import Mathlib.Data.Fintype.BigOperators
import Mathlib.Data.Finset.Basic
import Mathlib.Data.Nat.Bitwise
import Mathlib.Data.List.Range

namespace GF4

set_option linter.dupNamespace false

/-! ## GF(4) as an explicit field -/

/-- The field with four elements, `GF(4) = {0, 1, ω, ω̄}`, where `ω` is a
primitive element satisfying `ω² = ω + 1 = ω̄` and `ω³ = 1`. -/
inductive GF4 where
  | zero
  | one
  | omega
  | omegabar
deriving DecidableEq, Repr, Inhabited

open GF4

instance : Fintype GF4 := ⟨{zero, one, omega, omegabar}, fun x => by cases x <;> decide⟩

/-- Addition table of GF(4) (characteristic two). -/
def add : GF4 → GF4 → GF4
  | zero,     y        => y
  | x,        zero     => x
  | one,      one      => zero
  | one,      omega    => omegabar
  | one,      omegabar => omega
  | omega,    one      => omegabar
  | omega,    omega    => zero
  | omega,    omegabar => one
  | omegabar, one      => omega
  | omegabar, omega    => one
  | omegabar, omegabar => zero

/-- Multiplication table of GF(4). -/
def mul : GF4 → GF4 → GF4
  | zero,     _        => zero
  | _,        zero     => zero
  | one,      y        => y
  | x,        one      => x
  | omega,    omega    => omegabar
  | omega,    omegabar => one
  | omegabar, omega    => one
  | omegabar, omegabar => omega

/-- Multiplicative inverse (with `0⁻¹ = 0`). -/
def inv : GF4 → GF4
  | zero     => zero
  | one      => one
  | omega    => omegabar
  | omegabar => omega

instance : Zero GF4 := ⟨zero⟩
instance : One GF4  := ⟨one⟩
instance : Add GF4  := ⟨add⟩
instance : Mul GF4  := ⟨mul⟩
instance : Neg GF4  := ⟨id⟩
instance : Inv GF4  := ⟨inv⟩

/-- `GF4` is a field.  Every axiom is a finite statement over the four
elements, discharged by `decide`. -/
instance : Field GF4 where
  add := add
  mul := mul
  neg := id
  zero := zero
  one := one
  inv := inv
  add_assoc := by decide
  zero_add := by decide
  add_zero := by decide
  add_comm := by decide
  left_distrib := by decide
  right_distrib := by decide
  zero_mul := by decide
  mul_zero := by decide
  mul_assoc := by decide
  one_mul := by decide
  mul_one := by decide
  mul_comm := by decide
  add_left_neg := by decide
  exists_pair_ne := ⟨zero, one, by decide⟩
  mul_inv_cancel := by decide
  inv_zero := by decide
  nsmul := nsmulRec
  zsmul := zsmulRec
  qsmul := _
  nnqsmul := _

end GF4

open GF4 (GF4)

/-! ## The hexacode (GF(4) presentation) -/

/-- Standard 3×6 generator matrix of the hexacode over `GF(4)`,
in the normalization compatible with the MOG (columns ordered
`∞, 0, 1, 2, 3, 4`):
```
1 0 0 1 1  1
0 1 0 1 ω  ω̄
0 0 1 1 ω̄ ω
```
-/
def hexacodeGenerator : Matrix (Fin 3) (Fin 6) GF4 :=
  Matrix.of fun i j =>
    match i.val, j.val with
    | 0, 0 => GF4.one      | 0, 1 => GF4.zero     | 0, 2 => GF4.zero
    | 0, 3 => GF4.one      | 0, 4 => GF4.one      | 0, 5 => GF4.one
    | 1, 0 => GF4.zero     | 1, 1 => GF4.one      | 1, 2 => GF4.zero
    | 1, 3 => GF4.one      | 1, 4 => GF4.omega    | 1, 5 => GF4.omegabar
    | 2, 0 => GF4.zero     | 2, 1 => GF4.zero     | 2, 2 => GF4.one
    | 2, 3 => GF4.one      | 2, 4 => GF4.omegabar | 2, 5 => GF4.omega
    | _, _ => GF4.zero

/-- All 64 codewords of the hexacode, obtained by running the generator
matrix over every message vector in `GF(4)³`. -/
def hexacodeCodewords : Finset (Fin 6 → GF4) :=
  Finset.univ.image fun (m : Fin 3 → GF4) => Matrix.vecMul m hexacodeGenerator

/-- Parity-check matrix `H = [Aᵀ | I]` of the hexacode (where `A` is the
non-identity block of the generator).  A word is a hexacodeword iff
`H · wᵀ = 0`. -/
def hexacodeParity : Matrix (Fin 3) (Fin 6) GF4 :=
  Matrix.of fun i j =>
    match i.val, j.val with
    | 0, 0 => GF4.one      | 0, 1 => GF4.one      | 0, 2 => GF4.one
    | 0, 3 => GF4.one      | 0, 4 => GF4.zero     | 0, 5 => GF4.zero
    | 1, 0 => GF4.one      | 1, 1 => GF4.omega    | 1, 2 => GF4.omegabar
    | 1, 3 => GF4.zero     | 1, 4 => GF4.one      | 1, 5 => GF4.zero
    | 2, 0 => GF4.one      | 2, 1 => GF4.omegabar | 2, 2 => GF4.omega
    | 2, 3 => GF4.zero     | 2, 4 => GF4.zero     | 2, 5 => GF4.one
    | _, _ => GF4.zero

/-- Exact membership test for the hexacode via the parity check `H · wᵀ = 0`. -/
def isHexacodeword (word : Fin 6 → GF4) : Bool :=
  (List.finRange 3).all fun i =>
    decide ((Finset.univ.sum fun j => hexacodeParity i j * word j) = 0)

/-- The hexacode (image of the generator) has exactly 64 codewords
(dimension 3 over GF(4)), and every one of them passes the parity check
`isHexacodeword`; i.e. the generator and parity-check descriptions agree. -/
theorem hexacode_facts :
    hexacodeCodewords.card = 64 ∧
    (∀ m : Fin 3 → GF4, isHexacodeword (Matrix.vecMul m hexacodeGenerator) = true) := by
  native_decide

/-- The hexacode has exactly 64 codewords (dimension 3 over GF(4)). -/
theorem hexacode_card : hexacodeCodewords.card = 64 := hexacode_facts.1

/-- Every generator codeword passes the `isHexacodeword` parity check. -/
theorem hexacode_generator_isHexacodeword (m : Fin 3 → GF4) :
    isHexacodeword (Matrix.vecMul m hexacodeGenerator) = true :=
  hexacode_facts.2 m

/-! ## The binary Golay code via the MOG rule

We index the 24 MOG cells by `Fin 24`, cell `(row r, column c)` corresponding
to `r * 6 + c` (rows `0..3`, columns `0..5`).  A subset of cells is a bitmask
`n : ℕ` (bit `p` set iff cell `p` is occupied).

Rows are labelled by the four elements of `GF(4)` in the order `0, 1, ω, ω̄`
(i.e. row `r` carries the label with `Nat` code `r`); the *score* of a column
is the sum of the labels of its occupied rows.  A cell pattern is a Golay
codeword iff every column count has the same parity as the top-row count, and
the six column scores form a hexacodeword.

The computation is carried out on a `Nat`/`Bool` engine (GF(4) elements coded
as `0, 1, 2, 3`), which mirrors the GF(4) definitions above exactly. -/

/-- GF(4) addition on the `Nat` codes `0,1,2,3` (`= 0,1,ω,ω̄`). -/
def gfAddN : Nat → Nat → Nat
  | 0, y => y | x, 0 => x
  | 1, 1 => 0 | 1, 2 => 3 | 1, 3 => 2
  | 2, 1 => 3 | 2, 2 => 0 | 2, 3 => 1
  | 3, 1 => 2 | 3, 2 => 1 | 3, 3 => 0
  | _, _ => 0

/-- GF(4) multiplication on the `Nat` codes `0,1,2,3`. -/
def gfMulN : Nat → Nat → Nat
  | 0, _ => 0 | _, 0 => 0
  | 1, y => y | x, 1 => x
  | 2, 2 => 3 | 2, 3 => 1
  | 3, 2 => 1 | 3, 3 => 2
  | _, _ => 0

/-- Dot product `Σ aᵢ wᵢ` of two length-6 GF(4) vectors given by their codes. -/
def gfDot6 (a0 a1 a2 a3 a4 a5 w0 w1 w2 w3 w4 w5 : Nat) : Nat :=
  gfAddN (gfAddN (gfAddN (gfMulN a0 w0) (gfMulN a1 w1))
                 (gfAddN (gfMulN a2 w2) (gfMulN a3 w3)))
         (gfAddN (gfMulN a4 w4) (gfMulN a5 w5))

/-- Parity-check membership test for the hexacode on six `Nat`-coded symbols,
using the rows of `H = [Aᵀ | I]`. -/
def isHexacodewordN (w0 w1 w2 w3 w4 w5 : Nat) : Bool :=
  (gfDot6 1 1 1 1 0 0 w0 w1 w2 w3 w4 w5 == 0) &&
  (gfDot6 1 2 3 0 1 0 w0 w1 w2 w3 w4 w5 == 0) &&
  (gfDot6 1 3 2 0 0 1 w0 w1 w2 w3 w4 w5 == 0)

/-- Is bit `i` of `n` set? -/
def bitOn (n i : Nat) : Bool := (n / 2 ^ i) % 2 == 1

/-- Bit `i` of `n` as a `Nat` (0 or 1). -/
def bitVal (n i : Nat) : Nat := (n / 2 ^ i) % 2

/-- Score (as a `Nat` code) of column `c` of the mask `n`.  Row 0 carries the
label `0`, so only rows 1,2,3 contribute (with codes `1, 2, 3`). -/
def maskColScoreN (n c : Nat) : Nat :=
  gfAddN (gfAddN (bitVal n (6 + c)) (gfMulN 2 (bitVal n (12 + c))))
         (gfMulN 3 (bitVal n (18 + c)))

/-- Number of occupied cells in column `c`. -/
def maskColCountN (n c : Nat) : Nat :=
  bitVal n c + bitVal n (6 + c) + bitVal n (12 + c) + bitVal n (18 + c)

/-- Number of occupied cells in the top row (row 0). -/
def maskTopCountN (n : Nat) : Nat :=
  bitVal n 0 + bitVal n 1 + bitVal n 2 + bitVal n 3 + bitVal n 4 + bitVal n 5

/-- Total number of occupied cells (Hamming weight). -/
def maskWeightN (n : Nat) : Nat :=
  (List.range 24).foldl (fun a p => if bitOn n p then a + 1 else a) 0

/-! ### Submask weight monotonicity (strong induction on `/2`)

`maskWeightN` is a 24-bit Hamming weight (fold over positions `0..23`).
Ordinary induction is awkward for the recursive identity
`weight(n) = n%2 + weight(n/2)` because the call is on **`n/2`**, not `n-1`.
We prove unbounded popcount by **`Nat.strongRecOn`**, then transfer to the
24-bit fold via `bitOn` ↔ `Nat.testBit` and `Nat.testBit_land`.
-/

/-- Unbounded popcount in the strong-recursion shape
`popcountN n = n%2 + popcountN (n/2)`. -/
def popcountN : Nat → Nat
  | 0 => 0
  | n + 1 => (n + 1) % 2 + popcountN ((n + 1) / 2)
termination_by n => n
decreasing_by
  exact Nat.div_lt_self (Nat.zero_lt_succ n) (by decide : 1 < 2)

/-- `bitOn` agrees with Mathlib/core `Nat.testBit`. -/
theorem bitOn_eq_testBit (n i : Nat) : bitOn n i = decide (n.testBit i) := by
  -- testBit n i = ((n >>> i) &&& 1) ≠ 0; equivalent to (n / 2^i) % 2 = 1
  simp only [bitOn, Nat.testBit, Nat.shiftRight_eq_div_pow, Nat.and_one_is_mod,
    bne_iff_ne, decide_not, Nat.mod_two_ne_zero]
  -- Align `= 1` vs `≠ 0` for a bit
  have h : n / 2 ^ i % 2 = 0 ∨ n / 2 ^ i % 2 = 1 := Nat.mod_two_eq_zero_or_one _
  rcases h with h | h <;> simp [h]

/-- AND distributes over a one-bit right shift (`/ 2`). -/
theorem and_div_two (x y : Nat) : (x &&& y) / 2 = (x / 2) &&& (y / 2) := by
  apply Nat.eq_of_testBit_eq
  intro i
  -- testBit ((x&&&y)/2) i = testBit (x&&&y) (i+1) = testBit x (i+1) && testBit y (i+1)
  --                      = testBit (x/2) i && testBit (y/2) i
  calc
    ((x &&& y) / 2).testBit i = (x &&& y).testBit i.succ :=
      (Nat.testBit_succ (x &&& y) i).symm
    _ = (x.testBit i.succ && y.testBit i.succ) := Nat.testBit_land x y i.succ
    _ = ((x / 2).testBit i && (y / 2).testBit i) := by
      rw [← Nat.testBit_succ, ← Nat.testBit_succ]
    _ = ((x / 2) &&& (y / 2)).testBit i := (Nat.testBit_land (x / 2) (y / 2) i).symm

/-- Least-bit monotonicity for submasks. -/
theorem submask_mod_two (x y : Nat) (h : x &&& y = x) : x % 2 ≤ y % 2 := by
  have hx : x % 2 = 0 ∨ x % 2 = 1 := Nat.mod_two_eq_zero_or_one x
  have hy : y % 2 = 0 ∨ y % 2 = 1 := Nat.mod_two_eq_zero_or_one y
  -- From h: testBit x 0 → testBit y 0
  have hbit : x.testBit 0 → y.testBit 0 := by
    intro hx0
    have hb := congrArg (fun n : Nat => n.testBit 0) h
    change (x &&& y).testBit 0 = x.testBit 0 at hb
    rw [Nat.testBit_land] at hb
    cases hy0 : y.testBit 0 <;> simp [hx0, hy0] at hb ⊢
  -- testBit _ 0 ↔ % 2 = 1
  have t0 (n : Nat) : n.testBit 0 = decide (n % 2 = 1) := by
    simp [Nat.testBit, Nat.and_one_is_mod, bne_iff_ne]
    have hn : n % 2 = 0 ∨ n % 2 = 1 := Nat.mod_two_eq_zero_or_one n
    rcases hn with hn | hn <;> simp [hn]
  rcases hx with hx | hx <;> rcases hy with hy | hy <;> simp [hx, hy]
  -- remaining: x%2=1, y%2=0 impossible under submask
  have : y.testBit 0 = true := hbit (by simp [t0, hx])
  simp [t0, hy] at this

/-- Submask property is preserved under `/ 2`. -/
theorem submask_div_two (x y : Nat) (h : x &&& y = x) :
    (x / 2) &&& (y / 2) = x / 2 := by
  calc (x / 2) &&& (y / 2) = (x &&& y) / 2 := by rw [← and_div_two]
    _ = x / 2 := by rw [h]

/-- Full popcount is monotone under submasks — **strong induction** on `x`
(recursive call on `x/2`). -/
theorem popcountN_le_of_submask (x y : Nat) (h : x &&& y = x) :
    popcountN x ≤ popcountN y := by
  induction x using Nat.strongRecOn generalizing y with
  | ind x ih =>
    match x with
    | 0 =>
      simp only [popcountN]
      exact Nat.zero_le _
    | x + 1 =>
      have hlt : (x + 1) / 2 < x + 1 :=
        Nat.div_lt_self (Nat.zero_lt_succ x) (by decide : 1 < 2)
      have hsub : ((x + 1) / 2) &&& (y / 2) = (x + 1) / 2 :=
        submask_div_two (x + 1) y h
      have hrec : popcountN ((x + 1) / 2) ≤ popcountN (y / 2) :=
        ih ((x + 1) / 2) hlt (y / 2) hsub
      have hbit : (x + 1) % 2 ≤ y % 2 := submask_mod_two (x + 1) y h
      have hy_eq : popcountN y = y % 2 + popcountN (y / 2) := by
        match y with
        | 0 => simp [popcountN]
        | y + 1 => rfl
      have hx_eq : popcountN (x + 1) = (x + 1) % 2 + popcountN ((x + 1) / 2) := rfl
      omega

/-- `bitOn` of a bitwise AND is the conjunction of the bits. -/
theorem bitOn_and (x y i : Nat) :
    bitOn (x &&& y) i = (bitOn x i && bitOn y i) := by
  simp [bitOn_eq_testBit, Nat.testBit_land]

/-- If `x` is a submask of `y`, every set bit of `x` is set in `y`. -/
theorem bitOn_of_submask (x y i : Nat) (h : x &&& y = x)
    (hx : bitOn x i = true) : bitOn y i = true := by
  have hxy : bitOn (x &&& y) i = true := by rwa [h]
  have hboth : bitOn x i = true ∧ bitOn y i = true := by
    simpa [bitOn_and, Bool.and_eq_true] using hxy
  exact hboth.2

/-- Fold helper: weight accumulation is monotone when bits of `x` imply bits of `y`. -/
theorem foldl_bitWeight_le (x y : Nat)
    (hbit : ∀ i, bitOn x i = true → bitOn y i = true) :
    ∀ (ps : List Nat) (accx accy : Nat), accx ≤ accy →
      (ps.foldl (fun a p => if bitOn x p then a + 1 else a) accx) ≤
      (ps.foldl (fun a p => if bitOn y p then a + 1 else a) accy)
  | [], accx, accy, hacc => by simpa using hacc
  | p :: rest, accx, accy, hacc => by
    simp only [List.foldl_cons]
    by_cases hx : bitOn x p = true
    · have hy : bitOn y p = true := hbit p hx
      simp only [hx, hy, ite_true]
      exact foldl_bitWeight_le x y hbit rest (accx + 1) (accy + 1)
        (Nat.succ_le_succ hacc)
    · simp only [hx, ite_false]
      by_cases hy : bitOn y p = true
      · simp only [hy, ite_true]
        exact foldl_bitWeight_le x y hbit rest accx (accy + 1)
          (Nat.le_trans hacc (Nat.le_succ accy))
      · simp only [hy, ite_false]
        exact foldl_bitWeight_le x y hbit rest accx accy hacc

/-- 24-bit Golay mask weight is monotone under submasks. -/
theorem maskWeightN_le_of_submask (x y : Nat) (h : x &&& y = x) :
    maskWeightN x ≤ maskWeightN y := by
  have hbit : ∀ i, bitOn x i = true → bitOn y i = true := fun i hx =>
    bitOn_of_submask x y i h hx
  simpa [maskWeightN] using
    foldl_bitWeight_le x y hbit (List.range 24) 0 0 (Nat.le_refl 0)

/-- The MOG membership rule: the mask is a binary Golay codeword. -/
def golayMaskOkN (n : Nat) : Bool :=
  isHexacodewordN (maskColScoreN n 0) (maskColScoreN n 1) (maskColScoreN n 2)
    (maskColScoreN n 3) (maskColScoreN n 4) (maskColScoreN n 5) &&
  (let p := maskTopCountN n % 2
   (maskColCountN n 0 % 2 == p) && (maskColCountN n 1 % 2 == p) &&
   (maskColCountN n 2 % 2 == p) && (maskColCountN n 3 % 2 == p) &&
   (maskColCountN n 4 % 2 == p) && (maskColCountN n 5 % 2 == p))

/-- Explicit basis of 12 generators of the Golay code (as bitmasks),
extracted from the MOG rule. -/
def golayBasisN : Nat → Nat
  | 0 => 12782640 | 1 => 6391320  | 2 => 3195660 | 3 => 1597830
  | 4 => 798915   | 5 => 266366   | 6 => 198419  | 7 => 117552
  | 8 => 3510     | 9 => 26405    | 10 => 975    | 11 => 38117
  | _ => 0

/-- Encode a 12-bit message `msg` as the corresponding Golay codeword bitmask
(the GF(2) linear combination of the basis vectors). -/
def golayEncode (msg : Nat) : Nat :=
  (List.range 12).foldr
    (fun i acc => if bitOn msg i then Nat.xor (golayBasisN i) acc else acc) 0

/-- The Golay code: the image of all `2¹²` messages under `golayEncode`. -/
def golayCode : Finset Nat :=
  (Finset.range 4096).image golayEncode

/-! ### Verified properties of the Golay code -/

/-- The Golay code has exactly `4096 = 2¹²` codewords: the 12 basis vectors
are linearly independent. -/
theorem golay_card : golayCode.card = 4096 := by native_decide

/-- Every generated codeword satisfies the MOG (Golay) membership rule:
no generated codeword violates it. -/
theorem golay_all_ok_card :
    (golayCode.filter fun n => golayMaskOkN n = false).card = 0 := by native_decide

/-- **There are exactly 759 octads.**  The Golay code has exactly 759
codewords of weight 8. -/
theorem octad_count :
    (golayCode.filter fun n => maskWeightN n = 8).card = 759 := by native_decide

/-- Any two distinct octads (weight-8 codewords) meet in 0, 2, or 4 points
(verified at the bitmask level: the popcount of the bitwise AND). -/
theorem octad_intersection_masks :
    ∀ a ∈ golayCode.filter (fun n => maskWeightN n = 8),
      ∀ b ∈ golayCode.filter (fun n => maskWeightN n = 8), a ≠ b →
        maskWeightN (a &&& b) ∈ ({0, 2, 4} : Finset ℕ) := by
  native_decide

/-- Full weight enumerator of the binary Golay code:
`1 + 759 x⁸ + 2576 x¹² + 759 x¹⁶ + x²⁴`. -/
theorem golay_weight_enumerator :
    (golayCode.filter fun n => maskWeightN n = 0).card = 1 ∧
    (golayCode.filter fun n => maskWeightN n = 8).card = 759 ∧
    (golayCode.filter fun n => maskWeightN n = 12).card = 2576 ∧
    (golayCode.filter fun n => maskWeightN n = 16).card = 759 ∧
    (golayCode.filter fun n => maskWeightN n = 24).card = 1 := by native_decide

/-- The minimum distance of the Golay code is 8: no codeword has weight
strictly between 0 and 8. -/
theorem golay_min_distance :
    (golayCode.filter fun n => 0 < maskWeightN n ∧ maskWeightN n < 8).card = 0 := by
  native_decide

/-! ## Syndrome map from the generator basis (self-dual H ≅ G)

The extended binary Golay code is self-dual: the twelve basis vectors form both a
generator matrix `G` and a parity-check matrix `H`.  The syndrome of a 24-bit
mask is the vector of GF(2) inner products against those basis vectors, packed
as a natural number in `{0, …, 2¹² − 1}`.

This is the algebraic bridge consumed by `K22.MOG.SyndromeLookupConcrete`.
-/

/-- GF(2) inner product of two bitmasks (parity of the popcount of bitwise AND). -/
def gf2DotMask (a b : Nat) : Bool :=
  (List.range 24).foldl
    (fun acc i => if bitOn a i && bitOn b i then !acc else acc) false

/-- Syndrome of a 24-bit mask: bit `i` is `⟨golayBasisN i, mask⟩`. -/
def golaySyndromeN (mask : Nat) : Nat :=
  (List.range 12).foldl
    (fun acc i =>
      if gf2DotMask (golayBasisN i) mask then acc ||| (2 ^ i) else acc) 0

/-- All correctable error masks (Hamming weight ≤ 3).  Cardinality
`C(24,0)+C(24,1)+C(24,2)+C(24,3) = 2325`. -/
def correctableMasks : List Nat :=
  let w0 := [0]
  let w1 := (List.range 24).map (fun i => 2 ^ i)
  let w2 :=
    (List.range 24).bind fun i =>
      (List.range i).map fun j => (2 ^ i) ||| (2 ^ j)
  let w3 :=
    (List.range 24).bind fun i =>
      (List.range i).bind fun j =>
        (List.range j).map fun k => (2 ^ i) ||| (2 ^ j) ||| (2 ^ k)
  w0 ++ w1 ++ w2 ++ w3

/-- There are exactly 2325 correctable (weight ≤ 3) patterns on 24 points. -/
theorem correctableMasks_length : correctableMasks.length = 2325 := by native_decide

/-- Every entry of `correctableMasks` has weight ≤ 3. -/
theorem correctableMasks_weight_le :
    correctableMasks.all (fun m => decide (maskWeightN m ≤ 3)) = true := by
  native_decide

/-- Syndrome is injective on the weight-≤3 sphere (d = 8 ⇒ t = 3 uniqueness).
O(n) Finset cardinality check: `|syndromes| = |masks|` on the 2325-sphere. -/
def syndromeInjectiveCorrectable : Bool :=
  let syns := correctableMasks.map golaySyndromeN
  decide (syns.toFinset.card = syns.length)

theorem golay_syndrome_injective_correctable :
    syndromeInjectiveCorrectable = true := by native_decide

/-- Exhaustive sphere decoder: recover the unique wt ≤ 3 error mask from a
12-bit syndrome, if one exists. -/
def golayDecodeSyndromeN (s : Nat) : Option Nat :=
  correctableMasks.find? (fun m => golaySyndromeN m = s)

/-- On every correctable mask, encode-then-decode is the identity.
Uses the same syndrome map; uniqueness of `find?` follows from injectivity. -/
theorem golay_decode_correct_on_correctable :
    correctableMasks.all
      (fun m => decide (golayDecodeSyndromeN (golaySyndromeN m) = some m)) = true := by
  native_decide
