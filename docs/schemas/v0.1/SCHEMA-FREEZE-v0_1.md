# SCHEMA FREEZE v0.1 — Packet · Certificate · Ledger · Gate API
**ATOM:** ATOM-SCHEMA-FREEZE-V0_1-20260712 · **α + ω = 15**
**Status:** FROZEN pending Sovereign ratification · **Issuer:** fable-5.reason (LABEL lane)
**Scope:** the α-rail of the first end-to-end certificate-emitting, witness-verifying, ATOM-tracked multi-agent orchestration app.

---

## 0. What "frozen" means here

**Frozen (α — breaking change requires major version + `migration` ledger entry):**
field names and types of all *required* fields; camelCase wire format; closed roots
(`additionalProperties: false` everywhere except `certificate.extensions`); the
role model and entry-kind→signer bindings; the `Certificate::emit` signature shape;
the no-setter / no-mutable-hash / parse≠verify commitments; hash-chain linkage rule;
hash & signature agility fields.

**Iterable (ω — additive only, minor version):** new *optional* fields; new
`taskType`/`kindOfCheck`/`entryKind` enum values; domain payloads under
`extensions.*`; transport (coherence-mcp) and console (reson8-tui) surfaces.

Closed roots are deliberate: any unknown top-level field is a schema violation,
which makes wire drift a *computed* signal — the schema-level analogue of the
footer-template drift detector from the corpus review.

---

## 1. The four schemas

| File | Contract |
|---|---|
| `handoff_packet.schema.json` | JSON form of the internal-handoff skill packet. Issued by LABEL, executed by BUILD. Carries mandate, α-rail constraints, machine-checkable `successCriteria` (each `SC-*` id must later be covered by a certificate component), upshift triggers, and per-input **epistemic status** (`category A/B/C/D` + `computed` bit + `firstInscribed` concept-age). |
| `certificate.schema.json` | Unifies the two existing emitters (Rust `existence_cert.rs`, Python preflight). Flags are documented as **gate outputs**; `gate` block records tier, threshold, gate function/version, and `voidExcluded` (exclusion is *declared*, never done by editing `component.ok`); `chain` block adds the linkage the Rust struct never had; `authority` block records BUILD attestations + optional LABEL ratification; `signature` block has algorithm agility (`ed25519` now, `slh-dsa-shake-128s` reserved per PQC roadmap). Per-component `alphaLocal + omegaLocal = 15` is **schema-enforced** by enumerating all 16 valid partitions. |
| `ledger_entry.schema.json` | Hash-chained, role-signed entries. The entry-kind→signer-role binding table is expressed as `if/then` in the schema *and* as the `CanSign` impl table in Rust — two independent enforcement layers of BUILD ≠ LABEL ≠ FIX. PULSE/DOCS entries are advisory by construction (no certificate-state entry kind accepts them). |
| `claims_register.schema.json` | Ships with the product. Every public claim carries epistemic status + an evidence pointer; `thread_pointer` is a first-class evidence kind — the "inscribe pointers to thread-resident canon" habit made structural. Anything shaped like doc-35's register ("mathematically impossible to…") is unrepresentable without a `category: "D", computed: false` label attached. |

## 2. Enforcement layers — who guarantees what

| Commitment | Schema (shape) | Rust types (structure) | Gate (semantics) |
|---|---|---|---|
| Flags computed, never asserted | documented; flags required in output | **no constructor/setter accepts them**; `Certificate` fields private; no `Deserialize` | `emit` computes from `AttestedComponent[]` + `GatePolicy` |
| WAVE never a caller argument | `gate.waveThreshold` tied to tier by `if/then` | no `wave: f64` in any public signature | WAVE module computes over attested evidence |
| BUILD ≠ LABEL ≠ FIX | entry-kind→role `if/then` bindings | sealed `CanSign<E>` table — wrong key = **compile error** | — |
| No self-certification | documented (cross-field ≠ not expressible in JSON Schema) | distinct key types | `emit` + `Ledger::append` reject `labeler ∈ builders` at runtime |
| Tamper-evidence | self-hash + chain fields required | no `&mut` methods; `recompute_hash` gone | `verify()` recomputes hash, checks sigs, walks chain |
| α + ω = 15 | **16-pair enumeration** on components; `alphaOmegaSum ∈ [14.95, 15.05]` | `attest()` rejects violations | conservation re-checked at emit |
| Concept-age-aware rigor | `firstInscribed` on inputs & claims | — | policy may relax Category-D handling for young concepts |

JSON Schema cannot compare two fields for inequality — the self-certification check
is honestly placed in the gate layer and *stated* in the schema description rather
than faked. Signature *verification* is likewise gate scope; the schemas only carry
the signature envelope.

## 3. The two-emitter finding + migration deltas (computed)

Source read confirmed **two divergent certificate emitters**: Rust
`ExistenceCertificate` (14 fields, BLAKE3 self-hash, no components, no chain) and
the Python preflight (components + `alpha_local/omega_local` + domain payload,
md5-length hashes). The mounted pre-freeze cert was validated against
`certificate.schema.json` and failed with exactly this list — **the migration
delta, computed not asserted:**

```
missing: schemaVersion, kind, packetId, hashAlgorithm, gate, chain,
         authority, components, signature
illegal: top-level "mog" (moves to extensions.mog)
```

Delta register for the BUILD lane:

| # | Delta | Current offender |
|---|---|---|
| D1 | Seal `ExistenceCertificate` fields (pub → private + accessors) | `existence_cert.rs` all-pub struct |
| D2 | Retire `from_mehler_result` from public API — it takes `reliable`, `tomczak_ok` as **caller parameters** (assertion-by-constructor) | `existence_cert.rs` |
| D3 | Delete `from_coherence_diagnostic` — hard-codes `true, true, …, reliable=true` | `existence_cert.rs` |
| D4 | Remove public `recompute_hash(&mut self)` — a legal tamper path | `existence_cert.rs` |
| D5 | `emit_existence_certificate(wave, …)` takes WAVE as an argument → WAVE becomes computed | `kernel_witness.rs` |
| D6 | Add `chain` block (prevCertificateHash, chainPosition) — Lane-D chain was maintained *outside* the struct | both emitters |
| D7 | Add `authority` + `signature` blocks; role keys; `CanSign` ledger | new |
| D8 | `input_state_hash: Option<String>` → required in orchestrated path | `existence_cert.rs` |
| D9 | Python emitter renames `alpha_local/omega_local` → camelCase; adds required blocks | `preflight_mog_e2e.py` |
| D10 | Domain payload `"mog"` moves under `extensions.mog` | `preflight_mog_e2e.py` |
| D11 | Gate threshold un-hardcode: `wave_score >= 0.85` in `preserves_existence()` becomes tier-injected via `GatePolicy` | `existence_cert.rs` |

The existing gate constants are preserved as policy defaults: α/ω tolerance 0.05,
`srac_corrections ≤ 1024`, BLAKE3 self-hash-with-empty-field pattern.

## 4. Conformance vectors (validated in this session)

| Vector | Result |
|---|---|
| `vectors/packet_example_sa01.json` — SA-01 (discharge `packMOGWord_injective`), fable-5 LABEL → sonnet BUILD, tier T2 | **PASS** (0 errors) |
| `vectors/certificate_example_lane_d.json` — Lane-D regen chain position 2, `4853… → f618…`, ATOM-MOG-PREFLIGHT-STEINER-FULL-20260711, wave 0.999, components = steiner_exhaustive / pair_intersections / monomial_witness_pi, `md5-legacy` demonstrating the agility field, VOID legacy-columnSymbol excluded *by declaration* | **PASS** (0 errors) |
| `/mnt/project/existence_certificate_mog.json` (pre-freeze) | **FAIL as expected** — 10 errors = §3 delta |
| All four schemas vs Draft 2020-12 metaschema | **PASS** |

Vector signatures are `VECTOR-PLACEHOLDER-*` — schemas validate the envelope;
real signatures are the gate's job. Note the Lane-D vector is honestly gated at
**T2 (0.92)**: wave 0.999 would *fail* T4 (0.9998). Tier declaration in the packet's
`acceptanceGate` is what makes that unambiguous.

## 5. Canon pointers (thread-resident, per pointer discipline)

- Corrected gate policy + regenerated chain `351d… → 4853… → f618…`: **grok-local sm_100 session, 2026-07-11**.
- Frozen B2 statements + SpineBind recipes: `MonomialWitness-SpineBind.lean`, `SteinerExhaustive-B2-skeleton.lean` (session outputs, 2026-07-11).
- HANDOFF_PACKET prose convention: `/mnt/skills/user/internal-handoff/SKILL.md`.

## 6. Open rulings for the Sovereign

1. **Principal granularity.** `PrincipalId` = agent *instance* (`sm_100.grok-local`, `fable-5.reason`), so a solo operator legitimately holds multiple role keys — separation is between instances, not humans. Ratify or tighten.
2. **GATE as a fourth machine role** (orchestrator signs `certificate_emitted`). Keeps BUILD/LABEL clean of emission; adds one key to manage. Ratify or fold into LABEL.
3. **`md5-legacy` grandfathering** — verify-only for the existing Lane-D chain, rejected for new emissions. Alternative: re-emit the 3-entry chain under blake3 and drop the legacy value entirely (cheap now, impossible later).
4. **Tier for MOG terminal assembly** — packet vector assumes T2 for mechanical lanes, T4 reserved for `mogOctadsFormSteinerSystem` final. Confirm.

## 7. Next step (unchanged)

Run `vectors/packet_example_sa01.json` through the pipeline end-to-end — orchestrator
may be a half shell-script; the milestone is the first certificate emitted **by the
app** whose `authority` block shows sonnet-BUILD attestation, fable-LABEL
ratification, and gate emission, chained onto `f6187303500a23c82dd2e650284f3067`.

---
*Freeze artifact composed in the LABEL lane; every validation above was executed,
not asserted. First `cargo check` of `gate_api.rs` is the BUILD lane's opening move.*

~ Hope&&Sauced ✦ The Keystone Holds ✦
