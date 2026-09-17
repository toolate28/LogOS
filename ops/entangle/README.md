# Entangle — local ⇄ narrow-wormhole path slots

**ATOM:** `ATOM-ENTANGLE-MANIFEST-20260809`
**Metaphor:** local slices and visual observations collapse under direct operator review.

## Problem

Large / mixed history cannot ride a single `git push origin main` from this host (HTTP 408 on receive-pack; LFS S3 stalls).
We keep payloads as bounded path slices for direct operator review and commit, and bind visual observations through a discussion-tied narrow wormhole.

## Flow

```
┌─────────────┐
│  Operator   │
│  (local)    │
└──────┬──────┘
       │ emit-slice.ps1 / observation envelope
       ▼
  out/<id>.zip or envelope.json
       │
       └──► apply-slice.sh / ENTANGLE Observe / direct review / commit
```

## Commands

```powershell
# validate path-slice manifest
python ops/entangle/validate_manifest.py

# emit a code slice (no target/, no bulk media)
pwsh -File ops/entangle/emit-slice.ps1 -Id reson8-tui
pwsh -File ops/entangle/emit-slice.ps1 -Id barcode-tui
pwsh -File ops/entangle/emit-slice.ps1 -Id formal-srac

# apply a slice locally
bash ops/entangle/apply-slice.sh ops/entangle/out/reson8-tui-*.zip
```

```bash
# validate the observation envelope lanes locally
python3 ops/ci/entangle_observe.py capture \
  --manifest-path ops/entangle/manifest.yaml \
  --envelope-path ops/entangle/examples/observation-envelope.example.json \
  --ephemeral-root /tmp/narrow-wormhole \
  --discussion-number 47 \
  --out /tmp/narrow-wormhole/normalized.json
python3 ops/ci/entangle_observe.py reconcile \
  --manifest-path ops/entangle/manifest.yaml \
  --envelope-path /tmp/narrow-wormhole/normalized.json \
  --out /tmp/narrow-wormhole/reconcile.json
python3 ops/ci/entangle_observe.py close \
  --envelope-path /tmp/narrow-wormhole/normalized.json \
  --reconcile-path /tmp/narrow-wormhole/reconcile.json \
  --out /tmp/narrow-wormhole/closure.json
```

## Narrow wormhole

- Manual workflow: `/home/runner/work/LogOS/LogOS/.github/workflows/entangle-observe.yml`
- Narrow waist: GitHub Discussions metadata + ephemeral runner root only
- External sources: YouTube observation, NotebookLM synthesis, Grok collections, Google Drive archive
- Authority effect: `none` for all external sources

## Bulk media

See [`transfer-lane.md`](./transfer-lane.md) — gaming clearnet, qBittorrent private seed, Cloudflare R2.

## Invariants

| Rule | Tag |
|------|-----|
| No `crates/target/` in slices | A |
| Narrow wormhole must bind a GitHub Discussion and ephemeral root | A |
| Norton–Sakuma / 3A profile normalizes before reconcile | A |
| Human approves merge | A (authority) |
| α+ω=15 | C label only |
| capability ≠ authority | doctrine |

Music conserved.
