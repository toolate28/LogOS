# GB-00 Work Log — ATOM-GB00-WSL2-NIX-20260713
# ATOM-DEPLOY-WAIST-PROMPTS-20260713 · worker grok-local (BUILD)

## Status: ✅ CLOSED (2026-07-14)

## Temet Nosce (pre-execution)
context_depth: session 019f5f1b + LogOS master d98cd7f3 + PKT-FILEIN-SCHEMAS-20260713-001 complete + ALIAS/Betti report (research context, not code)
drift_count: 0 (first verified probe this turn)
unknowns:
  - nix install may require interactive sudo password (⚑) — **resolved**
  - cuda_libs: /usr/lib/wsl/lib has DXCore/D3D only; nvidia-smi absent (honest false)
  - winhost clone size / time over 9P one-shot — **resolved**
  - ALIAS Betti Deviation Record is research blueprint; not a GB-00 blocker

## Probe results (step 1)
- wsl_kernel: 6.18.33.2-microsoft-standard-WSL2
- wsl_version: 2.7.10.0 (Windows 10.0.26200)
- kvm: true (/dev/kvm present)
- cuda_libs: false (no libcuda*/libnvidia* under /usr/lib/wsl/lib)
- nvidia_smi: absent
- nix: Determinate Nix 3.21.5 / nix 2.34.8 · flakes enabled
- docker: 28.5.2 present
- distro: Kali 2026.2
- ext4 root: ~/LogOS clone
- winhost: /mnt/f/Users/Matthew Ruhnau/LogOS

## Steps completed
- 2026-07-14T15:49:11+10:00 probes complete (kvm=true, cuda_libs=false, kernel=6.18.33)
- 2026-07-14T15:49:11+10:00 git clone winhost -> ~/LogOS; SHA d98cd7f3 match
- 2026-07-14T15:49:11+10:00 remote winhost configured
- 2026-07-14T07:43:44Z bootstrap_manifest.json written (JSON valid) — final SC values
- Nix install via Determinate → SC-nix green
- Smoke: `nix run nixpkgs#hello` exit 0

## Attestation (final)
context_depth: GB-00 closed; SC-nix/clone/manifest green
drift_count: 0
unknowns_remaining: cuda_passthrough_optional (not blocking)

### Success criteria
| ID | Result |
|----|--------|
| SC-nix | ✅ true |
| SC-clone | ✅ true (sha_match) |
| SC-manifest | ✅ true |

### Artifact
- `ops/bootstrap_manifest.json` (authoritative probe record)
- `ops/gb00-install-nix.sh`

### ALIAS/Betti context (research map — not implemented this packet)
- certificate.schema already has bettiProxyBelowThreshold
- gate_api.rs freezes alpha+omega==15; ConservationViolated
- Do not expand schemas v0.1 without freeze authority
