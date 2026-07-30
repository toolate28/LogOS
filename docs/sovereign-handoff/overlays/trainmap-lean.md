# Trainmap — Lean 4 Rail (Theorems)

```
STATION 0 ──► STATION 1 ──► STATION 2 ──► STATION 3 ──► HUB
K22 M24       Conservation  NS Shrinker   SubRiemannian lake build
```

## Stations (real paths)

| Station | Path | Couples to |
|---------|------|------------|
| 0 M24 | `lean/K22/M24Coefficient.lean` | notebooks Agent_M24, musicInvariant=15 |
| 1 Conservation | `lean/TriWeavon/ConservationInvariant.lean` | Agda ConservationRMatrix |
| 2 Bridge notes | `lean/AgdaLeanBridge.md` | name map Lean↔Agda |
| 3 NS | `lean/Ns/*`, `lean/TriWeavon/NS/*` | entropy / shrinker |
| 4 SubRiem | `lean/TriWeavon/SubRiemannian/*` | Agda SubRiemannian |
| HUB | `lean/lakefile.lean` | mathlib-backed libs |

## Ownership (Instance #2)

Lean proofs are **owned by the theorem statement** — tactics do not alias
mutable state. Dual of Redox `Arc<QuantumRail>`: shared *read* of definitions.

## ASCII rail

```
LEAN ░░░░░▒▒▒▓▓▓ THEOREM RAIL ▓▓▓▒▒▒░░░░░
     │M24│─│Conserv│─│Bridge│─│NS│─│lake│
              ▲
              └── peak_resonance_conserved (rfl)
```
