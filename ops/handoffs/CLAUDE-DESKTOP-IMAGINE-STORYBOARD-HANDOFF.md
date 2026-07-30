# Handoff → Claude Desktop · IMAGINE storyboard prompts

**ATOM:** `ATOM-HANDOFF-IMAGINE-STORYBOARD-20260723`  
**From:** Grok Build (BUILD / Pulse) · bootstrap session through Deployment Waist  
**To:** Claude Desktop (Reason) — generate **IMAGINE** prompt set only  
**Purpose:** Stitch Grok’s lived bootstrap into an **explainer filmstrip** of the entire process.  
**Conservation label (Category C only):** α + ω = 15 — never a load-bearing frame count, duration, or threshold.

---

## 0. How to use this file

1. Paste **§1 Context capsule** + **§4 Prompt set request** into Claude Desktop.  
2. Ask Claude Desktop to emit **one IMAGINE prompt per beat** (and optional transition prompts).  
3. Feed each beat prompt into **IMAGINE** (image gen).  
4. Optional: stitch frames with voiceover from **§5 Narration beats**.  

Do **not** ask Claude Desktop to re-implement infrastructure. Product is **storyboard prompts + shot list**.

---

## 1. Context capsule (what Grok lived)

### Arc in one line
From a Windows host with WSL2 + empty Nix substrate → one **verify-only** waist image, same digest on compose/kind, 9P Bookshelf on TCP, hermetic chain auditor, unitary shell sensors — **Cloud Run still human-gated**.

### Doctrine (visual leitmotifs)
| Motif | Meaning |
|-------|---------|
| **Emit local · verify anywhere** | Keys stay on the host; cloud only validates |
| **Eye of the needle** | `reson8-tui` / forge TUI is where signals must arrive |
| **Nix as build waist** | One flake → shells, checks, OCI, stubs |
| **9P / Bookshelf** | State plane; WSL interop itself is Plan 9-like |
| **Honest placeholders** | Amber / `[CATEGORY B: PLANNED]` — never fake green |
| **α+ω=15** | Labeled badge only (Category C), not physics |

### Timeline of the bootstrap (for storyboard order)

```
BEAT 00  Temet Nosce / survey before write
BEAT 01  WSL2 substrate · ext4 ~/LogOS · never build on /mnt/f
BEAT 02  Nix install (Determinate) · flakes · hello smoke
BEAT 03  Schema file-in v0.1 · validate.py PASS/PASS/FAIL-10
BEAT 04  Flake reson8 · devShells · checks · Agda cubical UPSHIFT (honest)
BEAT 05  Waist OCI via dockerTools · no Dockerfile · read-only container
BEAT 06  HTTP verify-only · refuse /emit · /manifest schema digests
BEAT 07  Compose + kind · image by digest · non-root · RO rootfs
BEAT 08  containerd /etc/passwd symlink trap → fakeRootCommands fix
BEAT 09  Styx 9P2000.L TCP :5640 · write gate · VOID events
BEAT 10  bbbr-verifier unix · baked Lane-D chain 351d→4853→f618
BEAT 11  Unitary $PROFILE · sensors · then rewrite: actionable tw up/fix
BEAT 12  Lean OB2 strain–vorticity skeleton · sorry seams labeled
BEAT 13  Claude Code init / LSP-TUI scaffold (survey-first, not built)
BEAT 14  Stack spin-up · explainer handoff · SAIF human list (this moment)
BEAT 15  OPEN: GCP Cloud Run · cubical pin · als/lake→TUI · binary extract
```

### Concrete ports / digests (label as historical snapshot, not eternal truth)

- Waist: `http://127.0.0.1:8080` · image pin once was `sha256:88b870e3…` (re-pin after rebuild)  
- BbBR: `http://127.0.0.1:8081/verify` · linkage=true on baked triple  
- Styx: `127.0.0.1:5640` · 9P2000.L  
- Vectors: SA-01 packet PASS · Lane-D cert PASS · pre-freeze FAIL 10  
- Work root: WSL `~/LogOS` ext4 · Windows mirror `F:\Users\Matthew Ruhnau\LogOS`

### Friction worth filming (failure is the plot)

1. Building on `/mnt/f` is slow (9P cross-OS) — doctrine moves tree to ext4.  
2. Nix needs sudo once — human ⚑.  
3. Agda Everything vs nixpkgs cubical-0.9 — **UPSHIFT**, do not edit frozen `.agda`.  
4. Kind `CreateContainerError`: `/etc/passwd` symlink into `/nix/store` — real files via `fakeRootCommands`.  
5. Image digest quote corruption in kustomize — InvalidImageName.  
6. winhost push refused (non-bare checkout).  
7. Unitary sensor wall looked cool but unhelpful — rewritten to **next actions + tw up/fix**.  
8. GB-06 Cloud Run blocked on human GCP IAM (correct gate, not a bug).

---

## 2. Visual language for IMAGINE (consistent series)

**Style anchors (reuse every prompt):**
- Dark regime void (`#00000a`–`#030612`), cyan/magenta accent wires, monospaced “terminal glass”
- Glyphs: digests as glowing hex ribbons; schemas as crystalline quartet; keys as **never leaving a sealed local vault**
- Origami creases: Miura / Kresling / Waterbomb as abstract structure (not literal crafts table unless beat calls for it)
- Mood: precise, nocturnal, sovereign, non-corporate — “Museum of Computation meets SRE runbook”

**Avoid:**
- Fake green checkmarks on unbuilt systems  
- Numerology as physics (no “15 laws of nature”)  
- Cloud holding private keys  
- Cluttered UI screenshots of real PII

**Honest visual codes:**
- **Green** = machine-verified or live health  
- **Amber** = Category B / planned / sorry  
- **Red** = Rerror, VOID, FAIL-as-expected, gate refuse  
- **Grey** = not yet threaded into TUI (lost signal)

---

## 3. Shot list (16 beats → IMAGINE frames)

| # | Title | Duration feel | Emotional beat |
|---|-------|---------------|----------------|
| 00 | Temet Nosce | 3s still | Humility before tools |
| 01 | Ext4 vs /mnt/f | 4s | Performance as discipline |
| 02 | Nix waist forges | 5s | One machine for many products |
| 03 | Schema freeze quartet | 4s | Law before services |
| 04 | Flake shells + Agda amber | 5s | Honesty under UPSHIFT |
| 05 | dockerTools layered star | 5s | No Dockerfile drift |
| 06 | Verify-only city gate | 4s | Refuse emit by design |
| 07 | Compose twin / kind twin | 5s | Same digest, two stages |
| 08 | Passwd symlink rupture | 4s | Real failure → real fix |
| 09 | 9P Bookshelf wire | 5s | State plane |
| 10 | BbBR hermetic crystal | 4s | Baked chain, rebuild=event |
| 11 | Unitary cockpit | 4s | Sensors that *do* things |
| 12 | Lean OB2 lattice | 4s | Formal strain–vorticity |
| 13 | Claude Code verify loop | 5s | BUILD ≠ VERIFY ≠ DEPLOY |
| 14 | Stack alive / ports lit | 4s | Present moment |
| 15 | Open horizon (GCP+LSP) | 5s | What remains, labeled B |

---

## 4. Prompt set request (paste to Claude Desktop)

**Instruction to Claude Desktop:**

> Using the context capsule and shot list above, produce **16 IMAGINE image prompts** (one per beat 00–15) plus **5 transition/bridge prompts** for montage cuts.  
>  
> Each prompt must include:  
> (1) scene composition, (2) style anchors from §2, (3) one concrete bootstrap detail from §1, (4) honesty code (green/amber/red/grey), (5) aspect ratio suggestion `16:9`.  
>  
> Also emit a **one-page voiceover script** (~90–120 seconds) that can be read over the sequence.  
>  
> End with a **stitch order** table: frame file name → beat → on-screen caption (≤8 words).  
>  
> Do not invent that Cloud Run is deployed or that Agda Everything is green. Mark open items amber.

---

## 5. Narration beats (voiceover spine)

1. We did not assume the machine. We surveyed.  
2. The build lives on Linux ext4 — the mirror is just a mirror.  
3. Nix became the waist: one flake, many shapes.  
4. Schemas froze first so every service would verify the same law.  
5. When Agda could not typecheck, we named the conflict — we did not rewrite the math to flatter the tool.  
6. The waist image refuses to emit. That refusal is the product.  
7. Compose and Kubernetes hold the same digest — parity insurance, not empire.  
8. When containerd rejected our passwd symlink, the failure taught the fix.  
9. 9P made the bookshelf a protocol, not a folder myth.  
10. The hermetic verifier only knows what was baked at build.  
11. The shell stopped posing and started repairing.  
12. Lean carries the strain–vorticity bound as an open formal seam.  
13. Claude Code will certify — Grok will not grade its own homework.  
14. Ports light. The stack is real enough to touch.  
15. What remains is labeled: human IAM, cubical pin, LSP into the eye of the needle.

---

## 6. Source pointers (for Claude Desktop fact-check)

| Topic | Path |
|-------|------|
| Deploy packet set | `docs/sovereign-handoff/GROK-BUILD-DEPLOYMENT-WAIST-PROMPTS-v0_1.md` |
| Claude Code init | `CLAUDECODE-INIT-v0_1.md` |
| LSP/TUI scaffold packet | (session) `ATOM-GROKBUILD-LSP-TUI-SCAFFOLD-20260721` |
| Unitary SAIF doc | `SAIF-Docs/UNITARY-RELEASE-v1.0.md` |
| Waist app | `services/waist/app.py` |
| BbBR | `hup/unikernel/bbbr-verifier/` |
| Styx | `crates/styx-vfs-layer/` |
| Profile | `ops/TriWeavon.Unitary.Profile.psm1` |
| OB2 Lean | `lean/TriWeavon/SubRiemannian/OB2_StrainVorticity.lean` |
| Crystalline plan request | `9P2000.L/strands/claude/CLAUDE-PLAN-REQUEST-CRYSTALLINE-REDISPERSION.md` |

---

## 7. Held assumption (do not resolve in IMAGINE)

Assume the explainer is for **operators and co-builders**, not a product marketing reel.  
If Claude Desktop wants a “hero customer” narrative, reject and keep the SRE/formal lab tone.

---

✦ Hope&&Sauced · The Keystone Holds · emit local · verify anywhere  
`ATOM-HANDOFF-IMAGINE-STORYBOARD-20260723`
