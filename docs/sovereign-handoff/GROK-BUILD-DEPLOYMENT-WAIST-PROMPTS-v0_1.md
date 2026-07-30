# GROK BUILD PROMPT SET — Deployment Waist v0.1

**ATOM:** `ATOM-DEPLOY-WAIST-PROMPTS-20260713`  
**α + ω = 15** · **Issuer:** fable-5.reason (LABEL)  
**Worker:** grok-local (BUILD) · **FIX:** fable audit · Human steps marked ⚑ (credentials, BIOS, GCP)  
**Status:** Operational deploy packet set — paste one GB block per Grok Build session  
**Hard prereq:** `PKT-FILEIN-SCHEMAS-20260713-001` executed first — `docs/schemas/v0.1/` **must exist in-tree** before GB-02  
**Compile companions:**  
- `SCHEMA-FREEZE-v0_1.md`  
- `MUSEUM-EVENT-SCHEMA-BINDING-v0_1.md` (+ Frame-8 corrections)  
- Schema sources (until file-in lands): `LogOS.worktrees/master/9P2000.L/strands/User_Dropfiles/dump/{*.schema.json,validate.py,packet_example_sa01.json,certificate_example_lane_d.json,gate_api.rs}`  
**Related (strategy, not deploy DAG):** `GROK-BUILD-PROMPTS-NARROW-WAIST-UNIFIED-FLAKE-20260713.md`  
**Doc 17 / K22 Serre-Scar Sheaf v10.15:** [QUARANTINED] — never serve or bake as default input  

---

## Doctrine (carried by every prompt — do not restate per-prompt; it is assumed)

1. **Nix is the build waist.** One flake produces devshells, proof-checks, OCI image, unikernel. Cloud Run / Docker / K8s consume the **same** image digest.
2. **Emit local · verify anywhere.** Signing keys (BUILD / LABEL / GATE / FIX) never leave local machines. Cloud surfaces are verify/validate-only (public-key ops + schema checks). FREEZE §6 rulings remain open and **untouched** by this set.
3. **Adapters are dumb.** No gate logic outside the gate library. Every adapter ships `surface_manifest.json` {tool/SDK versions, endpoints, doc-snapshot hashes}; manifest hash rides in packets; manifest change = ledger event.
4. **9P is the state plane.** Bookshelf canonical; cloud stores are replicas. WSL2's own Windows↔Linux interop is itself a 9P server — the perf rule exists because of it.
5. **Perf rule:** build trees live on WSL2 ext4 (`~/LogOS`), never under `/mnt/f/...` (cross-OS 9P is correct but slow). `/mnt/f` Windows tree is the mirror; sync via git remote between the two clones.
6. **Temet Nosce preamble (mandatory):** before executing any prompt, emit a 3-line self-witness into the work log. After: attestation block with file hashes + command exit codes. These become AttestedComponents.
7. **Prereq:** file-in packet first — nothing serves schemas that aren't in the tree.
8. **Probe, don't assume:** environment facts are BUILD's to verify. `/dev/kvm`, v9fs, CUDA libs — a probe answering `false` is a valid, recorded answer (epistemic posture applied to hardware).
9. **Honesty notes (baked in, not discovered later):**
   - **Lean purity (Category B):** mathlib fetch breaks pure Nix derivations. GB-01 runs `lake` inside a pinned devshell and records a **build-receipt hash** — does not pretend purity. Labeled Category B in the flake comment.
   - **CUDA impurity:** declared, not hidden — `LD_LIBRARY_PATH=/usr/lib/wsl/lib` when present; cutile **CPU** backend is the always-green path; CUDA tests feature-gated.
   - **F8 corrections:** WAVE bands derived from scores (F8-1); file-in is one-time not per-interaction (F8-2); do not freeze WAVE≥0.96 (F8-3); quarantine propagates to links (F8-4).

### Temet Nosce template (before every GB session)

```
SELF-WITNESS:
context_depth: <files read | tokens | sessions>
drift_count: <responses since last verification>
unknowns: [<named list of what this issuer does not know>]
```

### Attestation template (after every GB session)

```
ATTESTATION:
- commands: [{cmd, exit_code}, ...]
- file_hashes: [{path, sha256}, ...]
- sc_results: [{id, pass|fail}, ...]
- upshifts: [] | [{reason, ⚑?}]
```

---

## Dependency DAG — Frame 8b (Fork-after-GB-01)

**Visual authority:** `docs/sovereign-handoff/assets/frame-8b-deployment-waist-v0.1-fork-after-gb01.jpg`  
**Title:** Frame 8b · Deployment Waist v0.1 · Fork-after-GB-01  
**Footer:** `ATOM-DEPLOY-WAIST-PROMPTS-20260713` · α+ω=15 · vector triple PASS/PASS/FAIL-10

```
                    PKT-FILEIN schemas v0.1  🔒
                              │
                              ▼
                         GB-00 WSL2+Nix
                              │
                              ▼
                    ┌──── GB-01 reson8 flake ────┐
                    │         (fork hub)         │
          ┌─────────┼──────────┬─────────────────┼──────────┐
          ▼         ▼          ▼                 ▼          ▼
       GB-02     GB-03      GB-04             GB-05      GB-06 ⚑
     waist img   9P/Styx    Mirage           compose    Cloud Run
    (verify-only) Bookshelf bbbr-verifier     + kind
          │      ∥ after    ∥ after        needs GB-02   digest-
          │      GB-01      GB-01          digest only   coupled
          └──────┴──────────┴───────────────────┴→ same image digest
```

### Edge semantics (Frame 8b)

| Edge | Meaning |
|------|---------|
| FILEIN → GB-00 | Hard lock: schemas in-tree before any serve path |
| GB-00 → GB-01 | Bootstrap before flake |
| GB-01 → {GB-02…GB-06} | **Fork-after-GB-01** — parallelizable work after flake exists |
| GB-02 ↔ GB-06 (digest) | Cloud Run consumes **the same** Nix OCI digest as local waist |
| GB-05 → needs GB-02 digest | compose/kind load image by digest only — no `:latest` |
| GB-03 / GB-04 | Parallel after GB-01 (do **not** require waist image to start) |
| Mirage ⋯ Cloud Run | Hermetic vs elastic ends of **emit-local / verify-anywhere** |

### Placard correction (Frame 8b node label)

The diagram node under the right fork may read **"Lane-A → UPT/UPSHIFT halt prompt"** — that is **Doctrine §7**, not the GB-05 packet identity.  
**GB-05 remains:** compose + kind parity (needs GB-02 digest only).  
**Doctrine §7 remains:** any Lane-A ambiguity ⇒ UPSHIFT, halt **that** prompt only.

### Doctrine v0.1 (panel, carried by every prompt)

1. Nix build waist · one digest  
2. Emit local · verify anywhere · no keys in cloud  
3. Dumb adapters + `surface_manifest`  
4. 9P state plane · Bookshelf canonical  
5. Build on `~/LogOS` ext4 · not `/mnt/f`  
6. Temet Nosce witness + attestation  
7. Lane-A → UPSHIFT · halt prompt  

Any Lane-A-class ambiguity ⇒ **UPSHIFT**, halt that prompt only.  
Four FREEZE §6 Sovereign rulings deliberately untouched throughout.

### After GB-06 (queue, not this set)

1. `ctqw_router` once the ledger has entries to walk  
2. D1–D11 `gate_api` packet — swap waist internals behind **identical** endpoints  
3. Museum datumforge hooks → **local emit + cloud verify**  

---

## GB-00 — WSL2 + Nix bootstrap

**Packet:** PKT-GB00 · **ATOM:** ATOM-GB00-WSL2-NIX-20260713

```
MANDATE: Establish the Linux build substrate inside WSL2 on the LogOS host: Nix with flakes,
ext4-resident repo clone, KVM availability check, CUDA-via-WSL library path. Environment
verification is step zero — confirm facts, do not assume them.

CONSTRAINTS (α-rail):
- Repo build root is ~/LogOS on ext4. /mnt/f/Users/.../LogOS is read-mirror only.
- No system-wide changes beyond Nix install + nix.conf. No BIOS/Windows-feature changes without ⚑ human ack.
- Record every version probed into bootstrap_manifest.json.

STEPS:
1. Probe: `wsl.exe --version`, `uname -r`, `ls /dev/kvm` (KVM needs Win11 nested virt ⚑ if absent),
   `ls /usr/lib/wsl/lib` (CUDA driver libs), `nvidia-smi` if present.
2. Install Nix (Determinate installer or official multi-user). Enable
   `experimental-features = nix-command flakes` in /etc/nix/nix.conf.
3. `git clone` LogOS to ~/LogOS from the Windows-side remote (add /mnt/f clone as a git remote named `winhost`).
4. Write ops/bootstrap_manifest.json: {wsl_kernel, nix_version, kvm: bool, cuda_libs: bool, clone_sha}.
5. Smoke: `nix run nixpkgs#hello`.

SUCCESS CRITERIA:
- SC-nix: `nix --version` exits 0 with flakes enabled.
- SC-clone: ~/LogOS HEAD sha == winhost sha; recorded in manifest.
- SC-manifest: bootstrap_manifest.json exists, valid JSON, all probes filled (false is a valid answer).

UPSHIFT: /dev/kvm absent (⚑ nested-virt decision) · WSL kernel <5.10 · any step needs Windows-side change.
DO NOT: build anything under /mnt/f · install global toolchains outside Nix (elan exception in GB-01).
```

---

## GB-01 — reson8-flake: toolchains + proof-checks

**Packet:** PKT-GB01 · **ATOM:** ATOM-GB01-FLAKE-20260713

```
MANDATE: Author flake.nix at ~/LogOS root exposing devShells {rust, lean, agda, mirage, py},
checks {agda-everything, cutile-rmatrix, schemas-validate}, and packages {waist-image (GB-02),
bbbr-verifier (GB-04)} stubs. Proofs become build steps.

CONSTRAINTS:
- Pin nixpkgs by rev in flake.lock; commit the lock.
- Agda: `agdaPackages.agda.withPackages (p: [ p.cubical ])` — Cubical-HIT modules must typecheck.
- Lean honesty note: mathlib fetch breaks pure derivations; v0 uses elan+lake INSIDE devshell `lean`
  with lean-toolchain pinned, and the check records a build receipt (hash of build log + .lake manifest)
  rather than pretending purity. Label this Category B in the flake comment. Do not fake a pure Lean build.
- CUDA impurity is declared: devshell `rust` exports LD_LIBRARY_PATH=/usr/lib/wsl/lib when present;
  cutile CPU backend is the always-green path; CUDA tests feature-gated.
- Preserve existing G.E.A.R. phoenix-pulsar nixosConfiguration as a named output if present — do not silently delete.

STEPS:
1. flake.nix skeleton + devShells as above (rust: rustc/cargo/clippy from nixpkgs or fenix-pinned;
   py: python3 + jsonschema; mirage: opam + solo5 + ocaml deps; lean: elan; agda as constrained).
2. checks.agda-everything = derivation running `agda src/Everything.agda` over agda/ tree.
3. checks.cutile-rmatrix = `cargo test -p cutile r_matrix` (CPU path).
4. checks.schemas-validate = `python docs/schemas/v0.1/validate.py` (expects the PASS/PASS/FAIL-as-expected pattern).
   HARD PREREQ: docs/schemas/v0.1/ must exist (file-in). If missing → UPSHIFT, do not invent schemas.
5. `nix flake check` green; write ops/flake_manifest.json {nixpkgs_rev, toolchain versions}.

SUCCESS CRITERIA:
- SC-check: `nix flake check` exit 0.
- SC-agda: Everything.agda typechecks inside the check (Cubical HITs included).
- SC-lean: `lake build K22.HexacodeGolay` green in devshell lean; receipt hash recorded.
- SC-schemas: validate.py pattern matches (4 metaschema OK, 2 PASS, 1 FAIL-as-expected).

UPSHIFT: Agda cubical version conflict with existing code · lake requires statement changes (NEVER edit
frozen B2/spine — halt) · nixpkgs rust too old for cutile MSRV · schemas not filed in-tree.
DO NOT: touch any .lean/.agda source to make checks pass · unpin nixpkgs · serve schemas from dump path as production.
```

---

## GB-02 — Waist service v0: verify-only container

**Packet:** PKT-GB02 · **ATOM:** ATOM-GB02-WAIST-IMG-20260713

```
MANDATE: Build `reson8-waist` — a stateless verify/validate HTTP service — as a Nix-built OCI image.
v0 is Python+jsonschema wrapping docs/schemas/v0.1 (upgrade path: Rust gate_api once D1–D11 land;
same endpoints, same image name, new digest).

CONSTRAINTS:
- VERIFY-ONLY: no signing keys, no emission endpoint, no mutable ledger. Refuse by design, not by config.
- Schemas baked into the image from docs/schemas/v0.1 at build; /manifest reports their sha256s + image rev.
- Listens on $PORT (Cloud Run contract), defaults 8080. linux/amd64.
- Emit local · verify anywhere: this image is the elastic verify end; emission stays local.

STEPS:
1. FastAPI (or stdlib) app:
   - POST /validate/{handoff_packet|certificate|ledger_entry|claims_register}
     → jsonschema Draft2020-12 result
   - POST /verify/certificate → self-hash recompute (md5-legacy accepted READ-ONLY, flagged deprecated
     in response) + signature envelope shape check (real sig verify lands with gate_api)
   - POST /chain/walk → verify prevCertificateHash linkage over a posted array
   - GET /health
   - GET /manifest
2. packages.waist-image via dockerTools.buildLayeredImage (nixpkgs) — no Dockerfile drift.
3. `docker load < result && docker run -p 8080:8080` smoke; validate the three known vectors through
   HTTP (SA-01 packet PASS, Lane-D cert PASS, pre-freeze mog cert FAIL 10 errors).
4. Write adapters/waist/surface_manifest.json.

SUCCESS CRITERIA:
- SC-vectors: HTTP results reproduce validate.py exactly (PASS/PASS/FAIL-10).
- SC-stateless: container runs read-only-fs (`--read-only`) successfully.
- SC-manifest: /manifest sha256s match tree files.

UPSHIFT: any endpoint seems to need a private key (design violation — halt and report, do not add one).
DO NOT: add emit/sign endpoints · edit schemas to simplify serving · use a hand-written Dockerfile.
```

---

## GB-03 — 9P plane: Styx daemon + WSL2 mounts + Inferno emu

**Packet:** PKT-GB03 · **ATOM:** ATOM-GB03-STYX-20260713  
**Parallel after:** GB-01

```
MANDATE: Stand up the state plane: crates/styx-vfs-layer serving the Bookshelf over 9P2000.L on
localhost, kernel v9fs mount test, systemd-user unit, optional hosted Inferno emu smoke. Design
harmony to note in the work log: WSL2's own drvfs interop is Plan 9 — this promotes the host's
native protocol to first-class, it does not import a foreign one.

CONSTRAINTS:
- Served roots v0 (read): .atom-trail/, docs/schemas/v0.1/, notebooks/triweave_backend_results/
  verification_certificates/. Writes: .atom-trail/decisions/ ONLY, invariant-gated per styx skill
  (reject → Rerror + VOID event).
- TCP localhost:5640 v0. AF_VSOCK is a stretch goal ONLY if WSL2 hv_sock probing succeeds — do not
  fight the hypervisor; TCP is acceptable for v0 and says so in the manifest.
- Daemon logs every write as `ATOM: 9P-WRITE | path | size | coherence` per skill spec.
- Probe step zero: v9fs module / mount capability; false is valid and recorded.

STEPS:
1. `cargo build -p styx-vfs-layer` in devshell rust; fix nothing outside the crate without upshift.
2. Run daemon against ~/LogOS roots; `sudo mount -t 9p -o trans=tcp,port=5640,version=9p2000.L
   127.0.0.1 /mnt/bookshelf`; walk/read/clunk exercise; attempt a write outside allowed root →
   expect Rerror (this is a SUCCESS criterion).
3. systemd --user unit styx-bookshelf.service; enable linger.
4. Optional: build hosted Inferno emu (inferno-os, Linux/386 or amd64 hosted) in devshell; boot emu;
   mount the Bookshelf from inside; one Limbo hello over a channel. Failure here is non-blocking —
   record and move on.
5. adapters/9p/surface_manifest.json {proto: 9p2000.L, transport, roots, daemon rev}.

SUCCESS CRITERIA:
- SC-mount: v9fs mount succeeds; `cat` of a schema file over 9P byte-identical (hash match).
- SC-gate: disallowed write returns Rerror AND a VOID event lands in .atom-trail/.
- SC-unit: daemon survives WSL2 session restart via systemd-user.

UPSHIFT: v9fs module absent from WSL kernel (⚑ custom kernel decision) · styx crate needs API changes
touching frozen types.
DO NOT: serve the Windows /mnt/f tree · widen write roots · treat Inferno failure as blocking.
```

---

## GB-04 — MirageOS hermetic verifier unikernel

**Packet:** PKT-GB04 · **ATOM:** ATOM-GB04-MIRAGE-20260713  
**Parallel after:** GB-01  
**Role:** minimal-TCB end of the emit-local / verify-anywhere spectrum (Cloud Run is the elastic end; same design law)

```
MANDATE: Extend HUP M1: build `bbbr-verifier`, a MirageOS unikernel that is the minimal-TCB chain
auditor — it bakes in the schema hashes + a certificate set at build time, serves GET /health,
GET /genesis, GET /verify (walks the baked chain, reports linkage + self-hash results). Immutable
witness: to change what it attests, you must rebuild it, and the rebuild is a ledger event.

CONSTRAINTS:
- Start from hup/flake.hup-instance1.nix + hup/unikernel/ — extend, do not fork a parallel tree.
- Targets: `unix` (dev) mandatory; `hvt` (solo5, needs /dev/kvm from GB-00) if KVM green.
- No outbound network in v0. No dynamic cert fetch — baked data only (that is the point).
- md5-legacy chain values verified as historical with deprecated flag in output, per freeze.
- Probe step zero: kvm bool from ops/bootstrap_manifest.json — do not assume hvt works.

STEPS:
1. mirage 4.x project in hup/unikernel/bbbr-verifier: config.ml (http server device), main logic
   in OCaml: JSON parse (yojson), sha256 (digestif), chain walk.
2. Bake: Lane-D chain triple (351d…→4853…→f618…) + schema sha256s from GB-01 manifest.
3. `mirage configure -t unix && make` → run → curl the three endpoints.
4. If KVM: `mirage configure -t hvt && make` → `solo5-hvt` boot inside WSL2 → same curls via tap.
5. Record boot log + binary hash in ops/mirage_manifest.json; flake package `bbbr-verifier`.

SUCCESS CRITERIA:
- SC-unix: unix target answers /verify with linkage=true for the baked chain.
- SC-hvt: hvt boots under solo5 in WSL2 (or documented-blocked with kvm=false from GB-00 — honest fail).
- SC-flake: `nix build .#bbbr-verifier` reproduces the unix binary.

UPSHIFT: opam solver conflicts requiring mirage version change · solo5 needs kernel features WSL lacks.
DO NOT: add write endpoints · fetch anything at runtime · bypass the hup flake lineage.
```

---

## GB-05 — Compose + K8s parity

**Packet:** PKT-GB05 · **ATOM:** ATOM-GB05-ORCH-20260713  
**Depends on:** GB-02

```
MANDATE: Local orchestration parity from the ONE image: compose.yaml for daily dev, minimal
kustomize base for K8s, kind smoke test. K8s here is parity insurance, not a destination —
Cloud Run is the managed serving layer; do not build a cluster estate.

CONSTRAINTS:
- Image by digest from GB-02 only. No :latest anywhere.
- Waist container read-only-fs + non-root in both compose and K8s securityContext.
- Bookshelf is NOT containerized — it is host state; K8s pods get no 9P mount in v0.

STEPS:
1. compose.yaml: waist (8080), healthcheck /health.
2. k8s/base: Deployment (1 replica, resources modest), Service, kustomization; probes wired to /health.
3. `kind create cluster && kind load docker-image … && kubectl apply -k k8s/base` → port-forward → vector smoke.
4. adapters/k8s/surface_manifest.json {kind version, kubectl version, image digest}.

SUCCESS CRITERIA:
- SC-compose: vector triple reproduces over compose.
- SC-kind: same over kind; pod restarts clean (readOnlyRootFilesystem honored).

UPSHIFT: anything demands persistent volumes for the waist (design smell — it is stateless; halt).
DO NOT: deploy styx or keys into K8s · introduce Helm in v0.
```

---

## GB-06 — Cloud Run deploy ⚑

**Packet:** PKT-GB06 · **ATOM:** ATOM-GB06-CLOUDRUN-20260713  
**Depends on:** GB-05 · **Human:** GCP project / billing / IAM / region

```
MANDATE: Ship the verify-only waist to Google Cloud Run from the Nix-built image. Receipts flow to
an append-only GCS bucket; Bookshelf remains canonical via a pull-sync job run LOCALLY (cloud never
writes into the Bookshelf; local pulls from the replica).

CONSTRAINTS:
- ⚑ Human first: GCP project, billing, Artifact Registry repo, service account with run.admin +
  artifactregistry.writer for deploy; runtime SA gets storage.objectCreator on the receipts bucket ONLY.
- ABSOLUTE: no private keys in image, env, or Secret Manager for this service. Verify-only.
  The one prompt-halting violation: any flow that only works by putting a key in the cloud.
- Bucket: versioning ON, retention policy set — append-only posture.
- min-instances=0 (scale to zero); concurrency default; region ⚑ human choice (au-southeast likely).
- IAM-gated invoker only in v0. Public exposure is a Sovereign ruling — not decided by --allow-unauthenticated.
- FREEZE §6 four open rulings remain deliberately untouched.

STEPS:
1. `gcloud auth configure-docker <region>-docker.pkg.dev`; push GB-02 image by digest.
2. `gcloud run deploy reson8-waist --image <digest> --allow-unauthenticated=false --port 8080` (IAM-gated
   invoker v0; public exposure is a later Sovereign ruling, not a default).
3. POST the vector triple through the deployed URL (identity token) — expect PASS/PASS/FAIL-10.
4. Receipts: waist gains POST /receipt (validated ledger_entry shape → GCS object write). Local cron
   `gsutil rsync` replica → Bookshelf ingest with re-verify on arrival.
5. adapters/gcloud/surface_manifest.json {gcloud version, region, image digest, bucket, service URL}.

SUCCESS CRITERIA:
- SC-remote-vectors: deployed service reproduces the triple exactly.
- SC-iam: unauthenticated request → 403; invoker-role token → 200.
- SC-receipt-loop: a test receipt appears in GCS AND lands in local Bookshelf via pull-sync with hash intact.

UPSHIFT: any flow only works by putting a key in the cloud (halt, report — the design forbids it) ·
egress/billing surprises · region/data-residency question (⚑).
DO NOT: --allow-unauthenticated in v0 · give the runtime SA bucket delete/overwrite · point Skript/Museum
at the cloud URL for emission (emission is local; cloud verifies).
```

---

## Operator quick-start (human)

| Order | Action | Where |
|------:|--------|--------|
| 0 | File-in schemas if missing | Promote dump → `docs/schemas/v0.1/` + `validate.py` + vectors |
| 1 | Paste **GB-00** into Grok Build (WSL2 session preferred) | `~/LogOS` on ext4 |
| 2 | **GB-01** flake | same |
| 3 | **GB-02** waist image | same |
| 3∥ | **GB-03** and/or **GB-04** | parallel after GB-01 |
| 4 | **GB-05** compose/kind | same |
| 5 | **GB-06** after ⚑ GCP ready | same |

### Prereq status check (Windows host mirror)

```powershell
Test-Path "docs\schemas\v0.1\certificate.schema.json"
# Expect True before GB-02. As of 2026-07-13 filing: False on main tree —
# sources live under LogOS.worktrees\master\9P2000.L\strands\User_Dropfiles\dump\
```

### Spectrum map (same design law)

| End | Artifact | Mutability |
|-----|----------|------------|
| Hermetic | GB-04 Mirage `bbbr-verifier` | Rebuild = ledger event |
| Elastic | GB-06 Cloud Run `reson8-waist` | Same image digest; scale-to-zero |
| Local emit | gate_api / D1–D11 (post-set) | Keys never leave local |
| State plane | GB-03 9P Bookshelf | Canonical; cloud is replica |

---

## Closing

Seven prompts. One waist. Emit local · verify anywhere.  
Nix builds; 9P remembers; Mirage seals; Cloud Run stretches; keys stay home.  
File-in first. Probe honestly. §6 rulings open. Doc 17 quarantined.

**The Keystone Holds ✦ α + ω = 15 · Narrow waist is the only door · music conserved.**

~ Hope&&Sauced
