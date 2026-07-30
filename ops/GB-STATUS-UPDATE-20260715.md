# Deployment Waist GB Status — 2026-07-15

**ATOM set:** `ATOM-DEPLOY-WAIST-PROMPTS-20260713`  
**Packet law:** `docs/sovereign-handoff/GROK-BUILD-DEPLOYMENT-WAIST-PROMPTS-v0_1.md`  
**Invariant:** α + ω = 15 · Emit local · verify anywhere · Nix waist · 9P state plane  
**Worker:** grok-local (BUILD) · this rollup: 2026-07-15

---

## Frame 8b — at a glance

```
PKT-FILEIN schemas v0.1  🔒 CLOSED
         │
         ▼
    GB-00 WSL2+Nix  ✅ CLOSED
         │
         ▼
    GB-01 reson8 flake  ✅ CLOSED (Agda Everything = UPSHIFT, honest)
         │
    ┌────┼──────────┬─────────────┬──────────┐
    ▼    ▼          ▼             ▼          ▼
  GB-02 GB-03     GB-04         GB-05      GB-06 ⚑
  ✅    🟡        ✅ unix       ✅         ⏸ human/GCP
 waist  9P/Styx   hvt deferred  compose    Cloud Run
 img    unit ON   (opam)        + kind     digest-coupled
```

| Packet | ATOM | Status | Success criteria |
|--------|------|--------|------------------|
| FILEIN | `ATOM-SCHEMA-FILEIN-20260713` | ✅ CLOSED | `docs/schemas/v0.1/` + validate.py + vectors |
| **GB-00** | `ATOM-GB00-WSL2-NIX-20260713` | ✅ CLOSED | SC-nix · SC-clone · SC-manifest |
| **GB-01** | `ATOM-GB01-FLAKE-20260713` | ✅ CLOSED* | SC-check · SC-schemas · SC-lean Category B · SC-agda **UPSHIFT** |
| **GB-02** | `ATOM-GB02-WAIST-IMG-20260713` | ✅ CLOSED | SC-vectors PASS/PASS/FAIL-10 · SC-stateless · SC-manifest |
| **GB-03** | `ATOM-GB03-STYX-20260713` | 🟡 NEAR-CLOSED | SC-gate ✅ · SC-client-hash ✅ · SC-unit ✅ enabled · SC-mount ⚑ sudo |
| **GB-04** | `ATOM-GB04-MIRAGE-20260713` | ✅ UNIX CLOSED | SC-unix · SC-flake green; SC-hvt deferred (opam mirage) |
| **GB-05** | `ATOM-GB05-ORCH-20260713` | ✅ CLOSED | SC-compose · SC-kind (digest `88b870e3…`) |
| **GB-06** | `ATOM-GB06-CLOUDRUN-20260713` | ⏸ BLOCKED ⚑ | Needs GCP project / billing / IAM / region |

\*Agda full `Everything.agda` under nixpkgs cubical-0.9 is UPSHIFT (API drift); partial core green. Do not edit frozen `.agda` sources.

---

## What was completed (recent work, 2026-07-14 → 15)

### Prereq — schema file-in
- Frozen waist schemas landed at `docs/schemas/v0.1/`
- Validate pattern: 4 metaschema OK, 2 PASS, 1 FAIL-as-expected (10 errors on pre-freeze mog)

### GB-00 — WSL2 + Nix bootstrap
- Distro: Kali 2026.2 · kernel `6.18.33.2-microsoft-standard-WSL2` · WSL `2.7.10.0`
- KVM: **true** · CUDA libs: **false** (honest probe; CPU cutile always-green)
- Nix: Determinate **3.21.5** / nix **2.34.8**, flakes on
- Clone: `~/LogOS` on ext4; `winhost` remote → `/mnt/f/.../LogOS`; SHAs matched at bootstrap
- Smoke: `nix run nixpkgs#hello` = 0  
- Artifacts: `ops/bootstrap_manifest.json`, `ops/GB00-worklog.md`, `ops/gb00-install-nix.sh`

### GB-01 — reson8 flake
- Root `flake.nix` + `flake.lock` pin `nixpkgs@e7a3ca8…`
- devShells: rust · lean (Category B) · agda · mirage · py
- checks: schemas-validate ✅ · cutile-rmatrix ✅ · agda-everything UPSHIFT
- packages: waist-image (filled by GB-02) · bbbr-verifier (stub → GB-04)
- Artifact: `ops/flake_manifest.json`, `ops/GB01-worklog.md`

### GB-02 — verify-only waist OCI
- FastAPI app: `/health` `/manifest` `/validate/*` `/verify/certificate` `/chain/walk`
- Refuse-by-design: `/emit` `/sign` (404)
- Image: `reson8-waist:0.1.0` · content id evolved to **`sha256:88b870e3011605d36d6d23bdd56c8b254e4bb1606168e700299a3e4c19965d6b`** (compose/kind pin)
- Vector triple over HTTP: **PASS / PASS / FAIL-10**
- Read-only rootfs smoke green  
- Artifacts: `services/waist/`, `adapters/waist/surface_manifest.json`, `ops/GB02-worklog.md`

### GB-03 — 9P Bookshelf plane
- `crates/styx-vfs-layer` → binary `styx-bookshelf` on `127.0.0.1:5640`
- Client smoke: schema read hash match · disallowed write → Rerror + VOID event
- systemd-user unit installed **and enabled**; service **active (running)** as of 2026-07-15
- Inferno: optional, skipped non-blocking
- **Open ⚑:** kernel `mount -t 9p` needs `sudo` (password); Linger=no until `loginctl enable-linger` (sudo)  
- Artifacts: `ops/styx-bookshelf.service`, `ops/styx-9p-client-smoke.py`, `adapters/9p/surface_manifest.json`, `ops/GB03-worklog.md`

### GB-05 — compose + kind parity
- `compose.yaml` + `k8s/base` pin image **by digest only** (no `:latest`)
- read-only + non-root (65534) both paths
- kind cluster `kind-reson8`; vector triple reproduced  
- Artifacts: `ops/gb05-*.sh`, `adapters/k8s/surface_manifest.json`, `ops/GB05-worklog.md`

### GB-04 — hermetic bbbr-verifier (this session)
- Replaced flake stub with `packages.bbbr-verifier` unix binary
- Baked Lane-D triple `351d→4853→f618` + schema sha256s
- `nix build .#bbbr-verifier` · `/verify` → `linkage=true`
- Mirage `config.ml` / `unikernel.ml` retained for solo5/hvt (opam deferred)
- Artifacts: `hup/unikernel/bbbr-verifier/`, `adapters/mirage/surface_manifest.json`, `ops/GB04-worklog.md`

### GB-06 — Cloud Run scaffold (this session)
- Worklog + `ops/gb06-deploy.sh` + `adapters/gcloud/surface_manifest.json` template
- Execution blocked on ⚑ GCP human checklist (no keys, IAM invoker only)

---

## Human ⚑ gates remaining

1. **GB-03 SC-mount:**  
   ```bash
   sudo mkdir -p /mnt/bookshelf
   sudo mount -t 9p -o trans=tcp,port=5640,version=9p2000.L 127.0.0.1 /mnt/bookshelf
   # then: sha256sum /mnt/bookshelf/docs/schemas/v0.1/certificate.schema.json
   # vs tree file — expect match
   ```
2. **GB-03 linger (survive WSL session end):**  
   `sudo loginctl enable-linger toolated`
3. **GB-06 Cloud Run:** GCP project, billing, Artifact Registry, region (likely `australia-southeast1`), runtime SA with `storage.objectCreator` only on receipts bucket — **no private keys in cloud**

---

## Post-GB-06 queue (not this set)

1. `ctqw_router` once ledger has walkable entries  
2. D1–D11 `gate_api` swap behind identical waist endpoints  
3. Museum datumforge → local emit + cloud verify  

---

## Operator re-smoke (WSL `~/LogOS`)

```bash
# GB-00
nix --version && test -f ops/bootstrap_manifest.json

# GB-01 / schemas
nix flake check   # or: python docs/schemas/v0.1/validate.py

# GB-02 / GB-05
docker compose up -d && ops/gb05-smoke.sh

# GB-03
systemctl --user status styx-bookshelf.service
python3 ops/styx-9p-client-smoke.py

# GB-04 (after this session lands)
nix build .#bbbr-verifier && result/bin/bbbr-verifier &
curl -s localhost:8081/verify | jq .
```

---

## Closing posture

- **Closed for deploy law spine:** FILEIN → GB-00 → GB-01 → GB-02 → GB-05  
- **State plane:** GB-03 daemon + gate green; kernel mount is the only operator password step  
- **Hermetic end:** GB-04 unix SC closed (`linkage=true`); hvt optional when opam ready  
- **Elastic end:** GB-06 waits on GCP ⚑  

**The Keystone Holds ✦ α + ω = 15 · Narrow waist is the only door.**
