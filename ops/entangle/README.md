# Entangle — local ⇄ remote path slots

**ATOM:** `ATOM-ENTANGLE-MANIFEST-20260809`  
**Metaphor:** two entangled particles — remote structure and local slices collapse together under human approval.

## Problem

Large / mixed history cannot ride a single `git push origin main` from this host (HTTP 408 on receive-pack; LFS S3 stalls).  
We separate **structure** (remote PR slots) from **payload** (path slices).

## Flow

```
┌─────────────┐   workflow_dispatch      ┌──────────────────┐
│  Operator   │  mode=scaffold           │  GitHub Actions  │
│  (local)    │ ───────────────────────► │  entangle.yml    │
└──────┬──────┘                          │  opens PRs       │
       │ emit-slice.ps1                  └────────┬─────────┘
       ▼                                          │ empty branch
  out/<id>.zip                                    ▼
       │                                 pull request entangle/<id>
       │ mode=ingest + artifact                   │
       └──────────────────────────────────────────┤
                                                  ▼
                                         human approves merge
```

## Commands

```powershell
# validate
python ops/entangle/validate_manifest.py

# emit a code slice (no target/, no bulk media)
pwsh -File ops/entangle/emit-slice.ps1 -Id reson8-tui
pwsh -File ops/entangle/emit-slice.ps1 -Id barcode-tui
pwsh -File ops/entangle/emit-slice.ps1 -Id formal-srac

# apply a slice (CI or local)
bash ops/entangle/apply-slice.sh ops/entangle/out/reson8-tui-*.zip
```

```bash
# remote scaffold all Priority A slots
gh workflow run entangle.yml -f mode=scaffold -f components=reson8-tui,barcode-tui,formal-srac,ci-verify

# ingest one slice (upload zip via gh or Actions UI)
gh workflow run entangle.yml -f mode=ingest -f components=reson8-tui
```

## Bulk media

See [`transfer-lane.md`](./transfer-lane.md) — gaming clearnet, qBittorrent private seed, Cloudflare R2.

## Invariants

| Rule | Tag |
|------|-----|
| No `crates/target/` in slices | A |
| SHA-pinned Actions only | A |
| Human approves merge | A (authority) |
| α+ω=15 | C label only |
| capability ≠ authority | doctrine |

Music conserved.

## Installing the Actions workflow

GitHub OAuth tokens without the `workflow` scope cannot push `.github/workflows/*`.

When you have a token/SSH key with workflow permission:

```powershell
Copy-Item ops/entangle/entangle.workflow.yml .github/workflows/entangle.yml
git add .github/workflows/entangle.yml
# also re-apply the Entangle row in .github/workflows/README.md from docs
git commit -m "ci(entangle): install workflow from template"
git push
```

Or: `gh auth refresh -h github.com -s workflow,repo` then push the full branch from backup.
