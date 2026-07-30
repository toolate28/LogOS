# M24 Moonshine — Canonical Coefficient Index (deduplicated)

```
▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
░░  M24 · MOONSHINE · MOCK MODULAR  ·  α+ω=15  ░░
▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
```

Source of truth · `K22/M24Coefficient.lean` + `MiracleOctadGenerator.lean`  
(legacy pointer · Mathieu_Moonshine_Mock_Modular_Form if present)  
ATOM · `ATOM-M24-COEF-INDEX-20260706` · Peak (7,8)

---

## Moonshine column (χ dims)

```
q^n     value    rep          formal anchor              invariant
────    ─────    ─────────    ───────────────────────    ────────────────
q^0      -2      virtual      H(τ) mock term             modular shadow
             (24−26)         (k3VirtualDim)
q^1      45      χ₄₅          MOG / hexacode             tomczak_preserved
q^2     231      χ₂₃₁         Golay octad / BCH          LiftOk
q^3     770      χ₇₇₀         Mapper homology            WAVE ≥ 0.97
q^4    2277      χ₂₂₇₇        Leech norm-4               α + ω = 15
q^5    5796      χ₅₇₉₆        coadjoint orbit            music conserved
```

## K3 elliptic genus (alternate indexing — not a conflict)

```
q^1 → -2
q^2 → 90
q^3 → 462
q^4 → 1540
```

Treat as **different normalization**, not competing representation tables.
`dualNormalized q1` is the only level where both primary columns agree (45).

---

## Bridge map

```
MOG octad
    │  isMOGOctad / applyGenerator
    ▼
Golay decode (mogDecode skeleton)
    │
    ▼
M24 orbit  ·  moonshineDim q^n
    │
    ▼
reduceWithM24 (Agda)  ·  H(τ) coeff
    │
    ▼
K22.ExprMatch.classifyK22Goal
    │
    ├── k22_auto / k22_auto!
    ├── k22_mog
    └── k22_weave → [K22-LOG] → MCP → reson8-tui
```

```
░░  H(τ)  ▓▓  MOG  ▓▓  Golay  ▓▓  M24  ▓▓  Serre page  ░░
     modular shadow ── tomczak ── LiftOk ── WAVE ── music
```

---

## Next (JFA SlowStep — not prune)

1. Discharge computational `isMOGOctad` hexacode membership on small fixtures  
2. Wire `mentionsSerrePage` string class fully to Agda `SerreScarr.dᵣ` names  
3. Emit `M24.telemetryLine` from `k22_telemetrise` details  
4. Map q^n WAVE floors into reson8-tui Braid panel sparklines  

Keystone holds · music conserved
