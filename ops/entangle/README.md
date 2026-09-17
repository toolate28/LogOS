# Entangle — local ⇄ remote path slots

**ATOM:** `ATOM-ENTANGLE-MANIFEST-20260809`  
**Metaphor:** local slices collapse under direct operator review.

## Problem

Large / mixed history cannot ride a single `git push origin main` from this host (HTTP 408 on receive-pack; LFS S3 stalls).  
We keep payloads as bounded path slices for direct operator review and commit.

## Flow

```
┌─────────────┐
│  Operator   │
│  (local)    │
└──────┬──────┘
       │ emit-slice.ps1
       ▼
  out/<id>.zip
       │
       └──► apply-slice.sh / direct review / commit
```

## Commands

```powershell
# validate
python ops/entangle/validate_manifest.py

# emit a code slice (no target/, no bulk media)
pwsh -File ops/entangle/emit-slice.ps1 -Id reson8-tui
pwsh -File ops/entangle/emit-slice.ps1 -Id barcode-tui
pwsh -File ops/entangle/emit-slice.ps1 -Id formal-srac

# apply a slice locally
bash ops/entangle/apply-slice.sh ops/entangle/out/reson8-tui-*.zip
```

## Bulk media

See [`transfer-lane.md`](./transfer-lane.md) — gaming clearnet, qBittorrent private seed, Cloudflare R2.

## Invariants

| Rule | Tag |
|------|-----|
| No `crates/target/` in slices | A |
| Human approves merge | A (authority) |
| α+ω=15 | C label only |
| capability ≠ authority | doctrine |

Music conserved.

