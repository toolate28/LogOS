# SAIF list — outstanding human actions

**ATOM:** `ATOM-SAIF-HUMAN-QUEUE-20260723`  
**For:** Matthew (operator)  
**Rule:** Category C `α+ω=15` is a **label only**. Do not use 15 as CPU/IO/limit magic.  
**Status key:** ⚑ = you · ⚒ = agent can finish after ⚑ · ⏸ = blocked on design/open formal pin

---

## Priority A — unblocks ship path

### A1. GCP / Cloud Run (GB-06) ⚑
**Why:** Last deploy-waist packet; agent cannot invent project/IAM.  
**Do:**
1. Create/select GCP project + billing  
2. Choose region (e.g. `australia-southeast1`)  
3. Artifact Registry docker repo  
4. Deploy SA: `run.admin` + `artifactregistry.writer`  
5. Runtime SA: `storage.objectCreator` **only** on receipts bucket (no delete)  
6. Bucket: versioning ON + retention (append-only posture)  
7. On WSL: install `gcloud`, `gcloud auth login`, `gcloud auth configure-docker REGION-docker.pkg.dev`  
**Reply to agent with:**
```
GCP ready
project=…
region=…
ar_repo=…
runtime_sa=…
receipts_bucket=…
```
**Then ⚒:** push image by digest · `gcloud run deploy` · vector/IAM/receipt smokes  
**Refs:** `ops/GB06-worklog.md` · `adapters/gcloud/surface_manifest.json`

### A2. Git push surfaces ⚑
**Why:** WSL `~/LogOS` is ahead of winhost; winhost rejects push to checked-out `master`.  
**Do (pick one):**
- **Preferred:** On Windows in `F:\Users\Matthew Ruhnau\LogOS`, pull from WSL remote or cherry-pick commits `8551ea93` / `de59f3d3` / unitary commits; then `git push origin master` to GitHub  
- **Or:** Make a bare mirror and push WSL → bare → F: worktree  
- **Or:** From F: add remote `wsl` = `\\wsl$\kali-linux\home\toolated\LogOS` and fetch/merge  
**Do not** force-push unless you intend to rewrite published history.  
**Then ⚒:** agent can verify `git status` clean + origin in sync

### A3. Sudo once for 9P v9fs mount (optional SC) ⚑
**Why:** Styx TCP smoke is green; kernel mount needs elevated rights.  
```bash
sudo mkdir -p /mnt/bookshelf
sudo mount -t 9p -o trans=tcp,port=5640,version=9p2000.L,uname=$USER 127.0.0.1 /mnt/bookshelf
sha256sum ~/LogOS/docs/schemas/v0.1/certificate.schema.json \
  /mnt/bookshelf/schemas/certificate.schema.json
```
**Then ⚒:** record SC-mount in `adapters/9p/surface_manifest.json`

---

## Priority B — verify loop / Claude Code

### B1. First Claude Code cold start ⚑
**Why:** VERIFY strand cert is the gate; BUILD must not self-certify.  
**Do:**
1. Open LogOS root in Claude Code  
2. Point it at `CLAUDECODE-INIT-v0_1.md` (repo root)  
3. Run three-surface survey only (no deploy, no push)  
4. Emit cert to `.atom-trail/certs/claude-code/` via  
   `pwsh -File ops/claude-code/Emit-ClaudeCodeCert.ps1 -AsInitRun` if scaffold exists; else follow init doc  
**Expect:** honest fail if wrangler/als/lake missing — **amber is correct**  
**Refs:** `CLAUDECODE-INIT-v0_1.md` · `ops/claude-code/*` if present

### B2. Wrangler / SpiralSafe inventory ⚑ (read-only)
**Do:** Confirm which tree is SoT for SpiralSafe; note Vectorize **768/cosine** if index exists (immutable).  
**Do not** `wrangler deploy` until cert path is green.

### B3. Reload unitary profile after module update ⚑
```powershell
. $PROFILE
tw help
tw fix      # auto-start down services via WSL
tw verify
```
If banner still feels noisy: edit profile hook to `Start-TriWeavonUnitary -Quiet` and run `tw` manually.

---

## Priority C — formal pins (slow, high value)

### C1. Cubical pin decision (GB-01 UPSHIFT) ⏸/⚑
**Why:** Agda Everything / `ArrivalDetector` may not typecheck on nixpkgs cubical-0.9.  
**Do:** Sovereign choice — pin older cubical **or** formal-layer packet (do **not** casually edit frozen `.agda` to silence LSP).  
**Then ⚒:** flake pin + LSP diagnostics into TUI

### C2. Lean mathlib warm + OB2 sorry seams ⏸
**Path:** `lean/TriWeavon/SubRiemannian/OB2_StrainVorticity.lean`  
**Do:** When ready for formal sprint, `lake build TriWeavon` after mathlib fetch; track sorries as amber, not green.

### C3. Opam mirage for bbbr hvt (optional) ⏸
KVM is present; mirage CLI was absent. Only if unikernel hvt is a near-term goal.

---

## Priority D — hygiene / optional

| ID | Action | Who |
|----|--------|-----|
| D1 | Re-pin compose/k8s digests after any waist rebuild | ⚒ after image change |
| D2 | Enable systemd --user linger for styx (if user bus available) | ⚑ |
| D3 | Crystalline redispersion **plan** review (do not implement yet) | ⚑ read `9P2000.L/strands/claude/CLAUDE-PLAN-REQUEST-CRYSTALLINE-REDISPERSION.md` |
| D4 | IMAGINE storyboard generation from handoff | ⚑ Claude Desktop + IMAGINE |
| D5 | Docker Desktop on Windows host if you want `docker` outside WSL | ⚑ optional |

---

## Daily SAIF loop (when building)

```
1. Temet Nosce (context / drift / unknowns)
2. tw                 → next actions
3. tw fix / tw up     → stack
4. tw verify          → smokes
5. Work only on labeled A/B; never promote D→A
6. Cert from Claude Code before any cloud deploy
```

---

## What is NOT on you right now

- Re-deriving α+ω as physics  
- Self-certifying Grok Build output  
- Editing frozen formal sources to force green LSP  
- `--allow-unauthenticated` on Cloud Run v0  

---

✦ Ordered for one human with a day job · Hope&&Sauced · The Keystone Holds
