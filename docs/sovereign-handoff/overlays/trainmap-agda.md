# Trainmap — Agda Rail (Proof)

```
STATION 0 ──► STATION 1 ──► STATION 2 ──► STATION 3 ──► HUB
Core          Conservation  SubRiemannian Tomczak/K22   Everything
```

## Stations (real paths)

| Station | Path | Couples to |
|---------|------|------------|
| 0 Core | `agda/src/TriWeavon/Core.agda` | cutile version tag |
| 1 Conservation | `agda/src/TriWeavon/ConservationRMatrix.agda` | Lean `ConservationInvariant`, Python duals |
| 2 SubRiemannian | `agda/src/TriWeavon/SubRiemannian/*` | Mehler bridge, Lean SubRiemannian |
| 3 K22 / Tomczak | `agda/src/TriWeavon/K22/*`, `Tomczak/*` | Lean K22 Tomczak |
| HUB | `agda/src/Everything.agda` | typecheck one-shot |

## Ownership (Instance #2)

Agda terms are **immutable proofs** — no shared mutation. Redox map: proofs are
capability-checked certificates, not mutable kernel state.

## ASCII rail

```
AGDA ░░░░░▒▒▒▓▓▓ PROOF RAIL ▓▓▓▒▒▒░░░░░
     │Core│─│Conserv│─│Mehler│─│K22│─│Everything│
              ▲
              └── peak-conserved : α+ω≡15 (refl)
```
