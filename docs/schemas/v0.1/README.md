# Schemas v0.1 — frozen waist (filed)

**Packet:** `PKT-FILEIN-SCHEMAS-20260713-001`  
**ATOM:** `ATOM-SCHEMA-FILEIN-20260713`  
**Invariant:** α + ω = 15  
**Status:** In-tree. One-time migration complete (F8-2). Runtime emits **certificates + ledger_entry**, not schemas-per-interaction.

## Authority

| Doc | Role |
|-----|------|
| [`SCHEMA-FREEZE-v0_1.md`](./SCHEMA-FREEZE-v0_1.md) | Freeze authority — closed roots, D1–D11, §6 open rulings |
| [`MUSEUM-EVENT-SCHEMA-BINDING-v0_1.md`](./MUSEUM-EVENT-SCHEMA-BINDING-v0_1.md) | Museum boundary → artifact kind / signer / CSEP; Frame-8 corrections |
| [`gate_api.rs`](./gate_api.rs) | Signature freeze only (`todo!()` bodies). Build path: `cutile` feature `schema_freeze_v0_1` |

**Doc 17 / K22 Serre-Scar Sheaf v10.15:** **[QUARANTINED]** (F8-4) — never serve or bake as default input.

## Quartet

| Schema | Purpose |
|--------|---------|
| `certificate.schema.json` | Gate-emitted certificate wire format |
| `handoff_packet.schema.json` | Packet / mandate / SC / constraints |
| `ledger_entry.schema.json` | Append-only ledger entry |
| `claims_register.schema.json` | Claims register |

## Frame-8 corrections (restated)

| ID | Law |
|----|-----|
| **F8-1** | WAVE **scores** are invariant; **bands** are derived — never transcribe band labels independently. |
| **F8-2** | File-in is **one-time**. Runtime emits certificates + ledger_entry, not schemas-per-interaction. |
| **F8-3** | Do not freeze WAVE ≥ 0.96 as law. Use HUP tier thresholds only (0.85 / 0.92 / 0.9998). |
| **F8-4** | Doc 17 / K22 v10.15 stays quarantined in Related links. |

## Conformance

```bash
cd docs/schemas/v0.1
# FAIL vector path (Claude Projects mount) must resolve for full suite:
#   /mnt/project/existence_certificate_mog.json  →  vectors/existence_certificate_pre_freeze.json
python validate.py
# Expect: 4× metaschema OK, 2× vector PASS, 1× FAIL-as-expected
```

## Vectors

| File | Expect |
|------|--------|
| `vectors/packet_example_sa01.json` | PASS vs handoff_packet |
| `vectors/certificate_example_lane_d.json` | PASS vs certificate |
| `vectors/existence_certificate_pre_freeze.json` | FAIL vs certificate (migration delta) |

## Deploy law

Operational GB set: `docs/sovereign-handoff/GROK-BUILD-DEPLOYMENT-WAIST-PROMPTS-v0_1.md`  
Nothing in **GB-02…GB-06** may serve schemas that are not filed here.

## Source (pre-file-in)

`LogOS.worktrees/master/9P2000.L/strands/User_Dropfiles/dump/`

✦ α + ω = 15 · The Keystone Holds
