# Transfer lane — bulk media / LFS bypass

**ATOM:** `ATOM-ENTANGLE-TRANSFER-LANE-20260809`  
**Why:** GitHub `git-receive-pack` 408 + S3 LFS ~2 KB/s stalled the main push. Bulk showcase video/img must **not** ride the commit pack.

## Diagnosis (collapsed)

| Hypothesis | Verdict |
|------------|---------|
| Branch protection gate | **False** — API 404, no protection on `main` |
| CI policy gate | **Post-push only** — PR #48 fails tree/rust jobs after landing |
| LFS pre-push hang | **True** — dual concurrent `git lfs pre-push` + S3 abort |
| ~1 GB `crates/target/` in history | **True** — stripped via filter-branch on local main |
| Pathological clearnet→us-east-1 S3 | **True** — curl PUT ~2 KB/s then connection abort |

## Lanes

### A. Git slice (default) — code only

```powershell
pwsh -File ops/entangle/emit-slice.ps1 -Id reson8-tui
# attach zip to workflow_dispatch entangle ingest OR commit on entangle/reson8-tui branch
```

Hard excludes: `target/`, `.git`, `*.pdb`, `*.rlib`, files > 8 MB.

### B. Clearnet gaming lane — max TCP for bulk

Uses existing `ops/net/LogOS.NetProxy.ps1`:

```powershell
pwsh -File ops/net/LogOS.NetProxy.ps1 -Action start-gaming   # proxies off
pwsh -File ops/net/LogOS.NetProxy.ps1 -Action optimize
# then either:
#  1) git lfs push --object-id origin <oid>   (one at a time)
#  2) Transfer lane B+/C below
```

**Rule:** never Tor/I2P for LFS/S3 or large git packs.

### C. qBittorrent peer / private track

For multi-host reconcile (WSL ↔ winhost ↔ remote builder):

1. Pack `ops/entangle/out/<id>-*.zip` or LFS object store export.
2. Seed via **private** torrent (or local network magnet) — not public DHT for private code.
3. Remote builder downloads → `apply-slice.sh` → PR.

Optional: qBittorrent Web API scripted from `logos-net` later (Category B until wired).

### D. Cloudflare R2 / Pages asset

Showcase videos live under `coherence-mcp/coherence-site/public/showcase/video/`:

1. Upload MP4s to R2 (or CF Stream) with `wrangler r2 object put`.
2. Keep **git LFS pointers or HTML URL rewrites** in repo (small).
3. Site fetch from R2 CDN — clone no longer needs 300 MB LFS.

This is the preferred long-term fix for the six showcase MP4s (~290 MB).

## Operator checklist

- [ ] `logos-net -Action start-gaming` before any bulk upload
- [ ] One LFS OID at a time if staying on Git LFS
- [ ] Prefer R2 for public showcase media
- [ ] Never re-add `crates/target/` — keep in `.gitignore`
- [ ] Use entangle PR slots for code; transfer-lane for media

Music conserved · Structure sovereign
