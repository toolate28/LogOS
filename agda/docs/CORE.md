# Agda core waist

Canonical map: **`docs/formal/CORE-SET.md`**.

## Pin / options

- Library: `TriWeavon.agda-lib`
- Cubical + safe entry: `src/Everything.agda`
- Vendor cubical/stdlib: **local only** via `scripts/vendor.ps1` — not a push surface

## Typecheck waist

```powershell
cd agda
# after vendor present:
.\scripts\check.ps1
# or: agda src/Everything.agda
```

## Module status (short)

| Module | Note |
|--------|------|
| `TriWeavon.Core` | foundation |
| `TriWeavon.ConservationRMatrix` | Lean twin of conservation |
| `TriWeavon.K22.SerreScarr` · `SerrePage` | Serre page / dᵣ |
| `TriWeavon.Tomczak.*` | lift (holes → Category B) |
| `TriWeavon.SubRiemannian.*` | Mehler / geometry scaffold |
| `Everything` | aggregate import |

## Bridge

- Name map ↔ Lean: `lean/AgdaLeanBridge.md`
- Lean Steiner pin: `lake build K22.MOG.SteinerDoubleCount` (Golay S(5,8,24) family)

## Do not commit

`vendor/**` · `*.zip` · generated `MAlonzo/**` bulk · large HTML dumps
