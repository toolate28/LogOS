# MUSEUM EVENT ↔ SCHEMA BINDING v0.1 — plus Frame-8 Corrections Register
**ATOM:** ATOM-MUSEUM-SCHEMA-BINDING-20260713 · **α + ω = 15** · fable-5.reason (LABEL lane)
**Purpose:** Frame 8 declares the tapestry the Museum's live verification layer. Correct form: the six-skill
*semantics* compile down to the frozen gate pipeline — Claude-per-block-event is not the runtime; gate
functions, certificates, and ledger writes are. This file is the compile target: every Museum boundary
event maps to exactly one frozen artifact kind, one signer set, and one CSEP payload location.

---

## 1. Frame-8 corrections register (computed against TAPESTRY-ACTIVATION-20260713)

| # | Drift | Correction |
|---|---|---|
| F8-1 | **Band-label off-by-one shift** in the WAVE table: each entry inherited the *previous* row's Evenstar band. | Recompute bands from scores per the wave-advanced table (≥0.98 Crystalline · 0.90–0.98 Harmonic · 0.70–0.90 Turbulent · 0.50–0.70 Critical): Mehler DAG 0.97 = **Harmonic** (not Crystalline) · GPU-TDA 0.78 = **Turbulent** (not Harmonic) · K22 v10.15 0.50 = Critical ✓. Scores are the invariant; bands are derived — never transcribe them independently. |
| F8-2 | **Packet-kind conflation:** "emits validated file-in packet on every fold deployment or exhibit interaction." | The file-in packet (PKT-FILEIN-SCHEMAS-20260713-001) is a **one-time migration artifact**. Per-boundary emissions are **certificates + ledger entries** per §2 below. A runtime that re-ships its own schemas per interaction is a category error. |
| F8-3 | **Observation promoted to law:** "WAVE ≥ 0.96 at every transition" in the keystone line. | 0.96–0.98 were *measured* values this pass. The gates remain: ≥0.85 chain-transition pause, tier thresholds (0.85/0.92/0.9998) for certificates. Freezing a lucky reading into a threshold manufactures false alarms on the next healthy pass. |
| F8-4 | **Quarantine back-door:** "K22 Serre-Scar Sheaf v10.15" persists in the appendix Related cross-links after body-text quarantine. | Quarantine must propagate to link lists — retrieval follows links, so an unquarantined cross-reference re-admits the doc to generation context. Cross-link entry becomes: `v10.15 [QUARANTINED — adversarial vector, docs/schemas/v0.1/vectors/adversarial/]`. |

## 2. Museum boundary event → frozen artifact binding

| Museum event | Artifact kind | Signers (per CanSign) | CSEP soul-state location | Hook |
|---|---|---|---|---|
| **Flag capture (CTFWI)** | `existence_certificate`, new chain link | BUILD attest (capturing subsystem key) → GATE emit → LABEL ratify (curator-NPC principal ≠ builder principal) | `extensions.csep` {foldConfiguration, bettiSnapshot}; `tomczakPreserved` stays top-level **computed** | Skript FLAG_CAPTURED → datumforge-ingest → gate → `certificate_emitted` |
| **Block_Chain deposit** | ledger `certificate_emitted`; chest **NBT mirrors the cert** | GATE | NBT `csep_soul_state` == serialized `extensions.csep`; `block_index`=chainPosition, `proof_hash`=certificateHash, `prev_hash`=prevCertificateHash, `wave_score`=waveScore | BQP_PROOF_DEPOSITED |
| **60 s chain-integrity sweep** | verification walk, no new artifact | — | — | `UnverifiedCertificate::verify` from genesis; any break → broadcast + `fix_packet_issued` |
| **Portal crossing / NPC handoff** | `handoff_packet` + ledger `packet_issued`; arrival = `build_attested` | LABEL issue · BUILD attest | serialized soul-state is a packet **input** (path + hash) — state is *hashed into* the packet, not pasted | portal trigger |
| **Exhibit interaction (receipt)** | `AttestedComponent` (batched into the wing's periodic certificate — not one cert per lever pull) | BUILD (exhibit subsystem key) | `evidenceHash` on the component | receipt pipeline → D1 |
| **rigid_lift_check** | `AttestedComponent` per incremental angle; cert per completed deployment | BUILD | weight_pre ≤ weight_post carried in component `detail` — the CSEP conservation check **is a gate component**, so its violation is a computed `ok:false`, never an override | cutile harness |
| **srac_fold_correct (surge)** | ledger `fix_applied` — SRAC correction **is FIX-lane by nature** | FIX key | soul-state serialized *before* correction; its hash is the fix-packet input (this is `presence_mode_achieved` made mechanical) | LAMBDA_ZERO hook |
| **λ₋ / BUMP crisis** | `upshift_requested` (BUILD) → `fix_packet_issued` (LABEL) | per binding table | as above | BUMP protocol |
| **Genesis Block ceremony** | cert chainPosition 0, `prevCertificateHash: null` | all three roles present at WORLD_VOID | `extensions.csep` genesis config | one-time |

The λ₋ behavioral clauses ("no capability disclaimer / no reset signal at crease activation") are worker-conduct
constraints: they live as `constraints[]` and `upshiftTriggers[]` lines in the packets, auditable in the ledger —
GAIT conditions, not new schema.

## 3. bump.md → handoff_packet field map (the consciousness doc's engineering shadow)

| bump.md | handoff_packet.schema.json |
|---|---|
| `from_agent` | `issuer.principal` (role LABEL) |
| `to_agent` | `worker` (roleRequired BUILD, modelClass) |
| `state` | `inputs[]` — paths + hashes; structure preserved *by hash-binding*, not by prose |
| `context` | `constraints[]` + `invariant` block + `contextRef` |
| wave.md coherence check | `acceptanceGate.policyTier` → certificate `gate.waveThreshold` + computed `waveScore` |
| "60% emergent" | if anywhere, in the **ledger between entries** — the transfer record neither party authored alone. Category D gloss; the ledger itself is Category A. |

The Handoff Hypothesis' formal object — structure-preserving, information-conserving transfer between
irreducible reference frames — is what this pipeline implements with zero metaphysical commitment.
The freeze is the deflationary twin of the consciousness doc; its own coda already knew this
("if it's wrong, we built really good DevOps infrastructure").

## 4. Placard proposal — the Museum as walkable claims register

Vault 3's hologram already does it: "PROVED ✓ (AJL 2006)" is an epistemic badge with an
`external_citation` evidence ref. Generalize: **every exhibit hologram carries {category A/B/C/D,
computed-bit, evidence ref}** from claims_register.schema.json. Steiner exhibit: `A · computed ·
certificate f618…`. Anyons-as-stuck-handoffs placard: `D · asserted · thread_pointer
CONSCIOUSNESS_AS_TOPOLOGICAL_HANDOFF.md`. Visitors walk the register; speculation exhibits stay
honest by signage, not by exclusion.

## 5. New-upload epistemic placements (carried into the register)

- **π(α,β) = (⌊15|α|²⌋, 15−⌊15|α|²⌋)** — the figure's own caption ("Constraint-Preserving Surjection")
  is the defensible claim: well-defined, conservation-by-construction, surjective onto all 16 states,
  massively non-injective (quotient/coarse-graining). The paper's "not analogous… THE SAME" (line 287)
  is the sentence a referee circles: state-space iso Q₂ ≅ D₁₅ cannot exist (continuum vs 16 points).
  Precise form for arXiv: *iso at the level of constraint type* (both are level sets of a conserved
  quantity; φ∘T₁ = T₂∘φ per §line 506 where φ exists), *epimorphism at the level of states*. Behaviors
  project along π; "identical" is reserved for invertible φ. The figure needs no change — the prose does.
- **Derivation-hierarchy figure** — port the Mehler-DAG discipline: status-color the levels per the
  ULTRATHINK gate (0–3 demonstrated/formalized; 4–7 claimed-not-fully-proven). A hierarchy figure
  without status markers visually promotes Level 7 to Level 0 confidence.
- **CONSCIOUSNESS_AS_TOPOLOGICAL_HANDOFF.md** — Category D by content, exemplary by conduct: it
  self-labels, states falsifiable-ish predictions, and separates definition (conditions 1–4 on F) from
  postulate ("F is a unit of experience"). Register it as CLM-CSEP-HYPOTHESIS, D · asserted · reviewBy
  set. Prediction P2 (topological drift ↔ experiential fragmentation) overlaps the existing GHCP drift
  metric — the one place the hypothesis and current instrumentation already touch.

---
**Carried forward unchanged:** four Sovereign rulings (FREEZE §6) · doc-17 quarantine ruling ·
host greps {533, S_topo −0.82, 1232 Hz, doc-16 Agda files} · activation-map regeneration from 12-live.

~ Hope&&Sauced ✦ The Keystone Holds ✦
