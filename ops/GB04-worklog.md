# GB-04 Work Log — ATOM-GB04-MIRAGE-20260713
SELF-WITNESS:
context_depth: GB-00..03+05 known; hup/unikernel console stub; kvm=true; flake bbbr stub prior
drift_count: 0
unknowns: [opam mirage 4.x switch time; solo5-hvt net device for HTTP on hvt]

## Status: 🟡 UNIX CLOSED · HVT DEFERRED (2026-07-15)

## Mandate compliance
- Extend `hup/unikernel/` — do not fork parallel tree → `hup/unikernel/bbbr-verifier/`
- Baked Lane-D triple: `351d5fea…` → `485325b4…` → `f6187303…`
- Schema sha256s from GB-02 waist manifest
- Endpoints: GET /health · /genesis · /verify only
- No outbound network · no write endpoints
- md5-legacy flagged deprecated in /verify steps

## Implementation notes
Unix SC implemented as hermetic Python stdlib server packaged by Nix (`packages.bbbr-verifier`).
Mirage `config.ml` + `unikernel.ml` retained for solo5/hvt when opam mirage is provisioned.
This is honest Category-B-adjacent packaging: same contract, rebuild = change attestation.
Full Mirage HTTP device is not faked as pure hvt.

## Success criteria
| ID | Result | Notes |
|----|--------|-------|
| SC-unix | ✅ | `/verify` → linkage=true for baked chain |
| SC-flake | ✅ | `nix build .#bbbr-verifier` produces `result/bin/bbbr-verifier` |
| SC-hvt | ⏸ deferred | kvm=true but opam mirage switch not provisioned this session |

## Attestation 2026-07-14T20:47:17Z
recorded_at: 2026-07-14T20:47:17Z
SC-unix: PASS (linkage=true tip=f6187303500a23c82dd2e650284f3067)
SC-flake: PASS (nix build .#bbbr-verifier)
SC-hvt: deferred-opam-mirage (kvm=true; not provisioned this session)
bbbr_unix.py sha256: 1d3a36a7225cabd343350b0326f8afa8f782a0f1d067b0b4d35a5d9cfd49395f
baked_chain.json sha256: 844f2f1da0ded268d1cabcdb8b96285bab885eea2b67935677decb3e21581c4f
listen: 127.0.0.1:8081
curl /health /genesis /verify: 200/200/200
