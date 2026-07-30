# Lean core waist

Canonical map: **`docs/formal/CORE-SET.md`**.

## Pin

- Toolchain: `leanprover/lean4:v4.8.0` (`lean-toolchain`)
- mathlib: `v4.8.0` (`lakefile.lean` / `lake-manifest.json`)

## Build waist (priority order)

```powershell
cd lean
lake build K22.HexacodeGolay
lake build K22.MOG.SteinerDoubleCount   # S1–S6 · S5 card/pack green
lake build K22.MOG.MonomialWitness      # CB-1 residual (B until green)
lake build K22.Existence
```

## Module status (short)

| Module | Note |
|--------|------|
| `K22.HexacodeGolay` | 759 octads · packing masks · syndrome |
| `K22.MOG.SteinerDoubleCount` | **S5 discharged** · `golay_octads_form_steiner` |
| `K22.MOG.MonomialWitness` | π transport · CB-1 open if not lake-green |
| `K22.MiracleOctadGenerator` | direct Steiner still `sorry` |
| `TriWeavon.ConservationInvariant` | CONSERVATION_SUM = 15 (software constant) |

## Bridge

- Name map ↔ Agda: `AgdaLeanBridge.md`
- Agda twin entry: `agda/src/Everything.agda`

## Do not commit

`lean/.lake/**` · `*.olean` · `*.ilean` · IR hashes
