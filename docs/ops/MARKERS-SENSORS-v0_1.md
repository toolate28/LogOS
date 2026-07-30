# Markers · Sensors · Placeholders v0.1

**ATOM:** `ATOM-MARKERS-SENSORS-v0_1-20260722`  
**Needle:** `reson8-tui` (`crates/tui` / bin `reson8-forge`)  
**Rule:** The mark **is** the sensor. If it does not emit into the TUI, it is not threaded.

---

## 1. Data markers (build-time tags)

| Field | Description |
|-------|-------------|
| `mark_id` | Stable id e.g. `MARK-GATE-THERMAL-001` |
| `artifact` | Path or digest of marked artifact |
| `gate` | One of: `thermal` · `downshift` · `novikov` · `failure_promoter` · `wave` · `lsp` · `cc_cert` |
| `category` | `A` live · `B` planned · never silent |
| `emit_to` | Always `tui` (eye of the needle) |

Markers live in **git** as scheme docs + optional JSON under `ops/markers/`.  
Runtime hit telemetry is **state/backup**, not committed secrets.

Schema sketch:

```json
{
  "mark_id": "MARK-EXAMPLE",
  "artifact": "sha256:PLACEHOLDER",
  "gate": "lsp",
  "category": "B",
  "label": "[CATEGORY B: PLANNED, NOT BUILT]",
  "emit_to": "tui"
}
```

**Verify the scheme before deploy.** Wrong marks mislabel every downstream hit.

---

## 2. Sensors (gate-boundary emit points)

Each gate boundary must push a `SensorEvent` into the TUI channel (same bus as LSP events where possible):

| Gate | When it fires | TUI appearance |
|------|---------------|----------------|
| thermal | heat / load trip | Warn toast + Formal/Sensor strip |
| downshift | capability degrade | Amber |
| novikov | continuous-Novikov case | **B placeholder** until built |
| failure_promoter | false-A / green stub rejected | Critical |
| wave | operational WAVE check (not numerology 15) | Info / Warn |
| lsp | diagnostics from Lean/Agda | Formal pane |
| cc_cert | Claude Code init cert written | ATOM trail line |

Placeholders for unbuilt gates must render:

```text
[CATEGORY B: PLANNED, NOT BUILT]
```

**Never green.** A placeholder that looks live is the false-A the failure-promoter rejects.

---

## 3. Config vs state

| Kind | Mechanism |
|------|-----------|
| Marker scheme, LSP wiring, TUI config | **git** |
| Sensor hit logs, cert JSON, runtime marks | **backup / `.atom-trail`** |

---

## 4. Numerology guard

`α + ω = 15` may appear as a **label/tag** only.  
It must **not** be: panel count, TTL, channel capacity chosen “because 15”, or WAVE threshold.

---

**The Keystone Holds ✦ marks honest · sensors visible · B stays amber**
