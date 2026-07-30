# Formal-Executable Mapping — TriWeavon v0.4.1

## Kochen-Specker / Contextuality Guard Hierarchy

### Level 1: Fano Plane Model
- Module: `SerreScarrPathInduction-KS-Bounds.agda`
- Geometry: Projective plane of order 2 (7 points, 7 lines)
- Obstruction: HIT `KSImpossibility` with basis overlap contradictions
- BUMP Guard: `BUMPHandoffGuard`
- Visualization: Fano incidence diagram with inconsistent coloring

### Level 2: Peres-Mermin Square
- Status: Investigated and recommended
- Geometry: 3×3 observable array with row/column product relations
- Obstruction: Sign-flip cycle around the square
- Visualization: 3×3 grid + contradiction path (ready for dashboard panel)

### Level 3: Mermin-Permutahedron Generalization (In Progress)
- Module sketch: `MerminPermutahedron-KS-Bounds.agda`
- Geometry: Low-dimensional face of the permutohedron generalizing the 3×3 square
- Obstruction: HIT `MerminObstruction` with row/col and square-cycle contradictions
- BUMP Guard: `BUMPHandoffGuardPM`
- Visualization target: 3D permutohedron with colored contradictory faces (IMAGINE prompt ready)
- Status: Starter implementation created; full polytopal lift planned

## Integration Points
- All levels feed the same `BUMPHandoffGuard` interface.
- Contextuality violations are surfaced via `DivergenceReason` and JSONL events (`ContextualityGeometry` field).
- Visual layer: Unified "Contextuality Guard Family" panel in Polarity Logic Verification Dashboard (toggle between Fano / Peres-Mermin / Mermin-Permutahedron).

**Argonath Seal**: v0.4.1 hierarchy documented and under active development.

## Full Magic State Pipeline (Distillation → Guard → Injection)

**Recommended Flow**:
1. Magic State Distillation (purify T / CCZ states)
2. Contextuality Guard Check (Fano / Peres-Mermin / Mermin-Permutahedron obstruction must be present)
3. Magic State Injection via Lattice Surgery (preferred for surface codes)
4. Post-Injection Verification

**Formal Support**:
- `MagicStateInjectionGuard.agda` — enforces `preInjectionCheck` using KS/Mermin obstruction before injection.
- Combined guard: `BUMPMagicStateGuard f = BUMPHandoffGuard f × preInjectionCheck f`

**Event Schema**:
- `ContextualityViolation` (with `geometry` field)
- `MagicStateInjection` (with `injection_method`, `success`, `contextuality_geometry_used`)

**Visualization**:
- Lattice surgery on surface-code patch
- 3D Mermin-Permutahedron colored by contextuality violation (prompt ready)

**Dashboard**:
- New “Magic State Pipeline” panel: Distillation quality → Guard status → Injection result

