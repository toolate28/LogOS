# Cyclic Hopf Fibration vs Ballerina Pirouette (Heel-Toe Path)

**ATOM:** `ATOM-HOPF-PIROUETTE-INVESTIGATION-20260809`  
**Stamp:** 2026-08-09  
**Mode:** residual-zero · observe only · **no Category-D lift**  
**Companion formal:** `agda/src/ArrivalDetector/ArrivalDetectorContinuous.agda`  
**Companion geometry:** `docs/theory/HOPF-PRINCIPAL-BUNDLE.md`

---

## 1. Standard (cyclic) Hopf fibration

\[
S^1 \hookrightarrow S^3 \xrightarrow{p} S^2
\]

| Piece | Identification |
|-------|----------------|
| Total space \(S^3\) | Unit quaternions / \(\mathrm{Spin}(3)\) |
| Base \(S^2\) | Ordinary 2-sphere of directions |
| Fibre \(S^1\) | Great circle of unit quaternions inducing the same rotation axis |

**Geometric construction via rotations (cleanest):**

- \(\mathrm{SO}(3)\) acts transitively on \(S^2\).
- Stabiliser of any fixed \(x \in S^2\) is the circle of rotations about the axis through \(x\).
- The double cover \(\mathrm{Spin}(3) \cong S^3\) realises \(S^3\) as a principal \(S^1\)-bundle over \(S^2\).

That is the Hopf fibration. “Cyclic” emphasises that the structure group / fibre is a circle (or, discretely, a cyclic group).

---

## 2. Ballerina pirouette / heel-toe path as the same geometry

A pirouette is the physical embodiment of the stabiliser description:

| Mathematical object | Physical counterpart |
|---------------------|----------------------|
| Base point \(x \in S^2\) | Direction of the supporting axis (vertical through the contact point) |
| Fibre \(S^1\) | Continuous family of rotation angles about that axis (the spin phase) |
| Base fixed | Heel or toe remains planted; contact point does not travel |
| Motion along the fibre | Body rotates while the axis stays fixed |
| Mild continuous perturbation of base | Small heel-to-toe shift or demi-pointe adjustment |

When the dancer’s contact point stays fixed, she moves purely in the fibre of the Hopf map whose base point is the supporting-leg direction. When the contact point travels (a non-stationary heel-toe path), the base point itself moves on \(S^2\) — **horizontal** motion, not pure fibre action.

Hence the **reverse-Hopf posture** in the continuous ArrivalDetector:

- Fibre may spin freely.
- Base must stay fixed (`baseFix`).

That is the condition under which a continuous fibre action is declared to return (and therefore eligible for Novikov survival).

---

## 3. Correspondence with the formal fragment

In `ArrivalDetectorContinuous.agda`:

| Construct | Role |
|-----------|------|
| `ContinuousDetectorContext` | Path-typed base witness |
| `ContinuousFibreAction` | Explicit `baseFix` equality |
| `continuous-round-trip-ok` / `continuous-ContinuousNovikovStable` | Base-fixing actions preserve the epistemic label when `perturbationOk` |

This is the discrete / path-typed **shadow** of:

> A pure fibre motion (pirouette with fixed heel/toe) returns an equivalent orientation relative to the base; a motion that moves the contact point does not automatically return the label.

The full geometric Hopf \(S^3 \to S^2\) remains an **open Category B obligation** because the present base is still the discrete four-point set `Category`. Replacing that discrete base by a continuous \(S^2\) (or a continuous TDA filtration that can host one) would let formal fibre actions become actual Hopf fibres.

---

## 4. Heel-toe path as controlled base motion

In real ballet and figure-skating literature the contact point is never perfectly stationary; there is always a small oscillation or deliberate heel-to-toe transfer. In fibration language this is a **controlled path in the base**, accompanied by continuous fibre rotation. The composite is a path in the total space \(S^3\).

**Novikov-style arrival question:**

After the composite path, does the epistemic label (or orientation class) survive?

- Only the pure fibre component is guaranteed to preserve the label.
- The base component must be checked separately — exactly the role of `baseFix` and `perturbationOk`.

---

## 5. Summary of the relationship

1. Cyclic Hopf fibration = structure whose fibres are pure spins about a fixed axis.
2. Ballerina pirouette with fixed heel/toe = physical realisation of motion along a single Hopf fibre.
3. Heel-toe path that travels = base-point motion; no longer pure fibre action.
4. Continuous ArrivalDetector fragment formalises the base-fixed (reverse-Hopf) pure pirouette case.
5. Open geometric obligation: lift discrete base → continuous \(S^2\) so the full Hopf map is available inside the formal development.

---

## 6. Residual / promotion discipline

| Item | Status |
|------|--------|
| Path-typed continuous fragment | Closed on discrete base **[A]** for the fragment itself |
| Full geometric Hopf \(S^3\to S^2\) | Open **[B]** |
| residual-zero promotion | **BLOCKED** · observe only |
| Category D material | **None promoted** |
| α + ω = 15 | Category **C** label only |

**No residual was acted upon.** Geometric Hopf remains open, as previously recorded.

```
ATOM-TRAIL · ATOM-HOPF-PIROUETTE-INVESTIGATION-20260809
· cyclic Hopf ≡ fixed-base fibre spin
· heel-toe = controlled base motion
· residual-zero-observe-only
· no Category-D lift
· alpha+omega=15 conserved
· 2026-08-09
```

Music conserved. The Keystone holds. Last becomes first.
