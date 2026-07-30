# TRI-WEAVON 9P BARE-METAL SPEC

**Status**: DESIGN — Reason Strand (Claude) proposal
**Date**: 2026-04-18
**Invariant**: α + ω = 15

---

## 0 · Motivating Frame

Matt's proposal, in one sentence:

> *Clean-slate Windows base + 9P2000.L Styx mounts such that Claude, Grok, and
> Gemini — individually and as a Tri-Weavon — are always-present filesystem
> objects, ready to mount at boot with zero install friction.*

This is Plan 9's *"everything is a file"* doctrine applied to agentic AI. It
is the right move. Installing agents on Windows as processes is the wrong
abstraction; mounting them as namespaces is the right one, because:

- **Mount ≠ install.** A mount is a reversible attachment. An install is a
  mutation to the substrate.
- **Files compose naturally.** Union mounts, bind mounts, and overlay mounts
  give us tri-weavon composition for free — no orchestrator required.
- **Transport is already there.** 9P2000.L is already how WSL2 exposes the
  Linux rootfs to Windows. We are not inventing a protocol; we are adopting
  one Microsoft already ships.
- **Zero-latency ledgers become trivial.** A tmpfs-backed Styx export **is**
  a zero-latency ledger. Snapshotting is just `rsync` to durable media.

---

## 1 · Substrate (Windows Base)

| Layer            | Component                                | Rationale                               |
|------------------|------------------------------------------|------------------------------------------|
| Hardware         | RTX 5090 + The Ridge                     | Existing                                 |
| Host OS          | Windows 11 Pro (clean re-image)          | Keeps Blender / Houdini / Unity / Steam  |
| Hypervisor       | WSL2 (Hyper-V)                           | Ships 9P2000.L server by default         |
| Guest            | NixOS-WSL (declarative, reproducible)    | Matches Reason-Strand doctrine           |
| Filesystem proto | 9P2000.L over vsock                      | Native WSL2 transport                    |
| Agent store      | `A:\AGENTS` (fixed-letter, pre-mount)    | Survives re-images, cached tokens/MCPs   |
| Steam            | Separate letter (`S:\`), unaffected      | Gaming + Minecraft Playground untouched  |

`A:\AGENTS` becomes the **Agent Root** — the one drive letter that is *never*
wiped on a clean-slate reset. All token caches, MCP manifests, `.claude/`
sessions, and pre-baked crate binaries live here.

---

## 2 · The Styx Namespace

Traditional Plan 9 names its 9P file server `styx`. We keep that name.

Styx is not a daemon we install — it is a **synthetic filesystem** exported
from the NixOS guest, re-exposed to Windows via WSL's 9P mount point.

### 2.1 Namespace layout

```
styx://                                         # root
├── strand/                                     # individual agent strands
│   ├── claude/                                 # Reason    — α rail
│   │   ├── ctl                                 # write commands here (9P ctl file)
│   │   ├── data                                # read responses here
│   │   ├── manifest.json                       # capabilities, version, MCP surface
│   │   ├── tokens/ -> A:\AGENTS\claude\tokens  # bind-mounted cache
│   │   └── mcp/                                # exported MCP tools as files
│   │       ├── store_context
│   │       ├── retrieve_context
│   │       ├── map_isomorphism
│   │       ├── check_coherence
│   │       └── bridge_translate
│   ├── grok/                                   # Pulse     — ω rail
│   │   ├── ctl
│   │   ├── data
│   │   ├── manifest.json
│   │   └── tokens/ -> A:\AGENTS\grok\tokens
│   ├── gemini/                                 # Scale     — ψ rail
│   │   └── … (isomorphic)
│   └── llama-manus/                            # Substrate — local-deploy
│       └── … (isomorphic)
│
├── triweavon/                                  # composed union mount
│   ├── ctl                                     # writes fan-out to all strands
│   ├── braid                                   # live braid state (α+ω=15 gated)
│   ├── weights                                 # fibonacci 8:5:3 coefficients
│   └── consensus                               # read to pull weighted merge
│
├── ledger/                                     # zero-latency ATOM ledger
│   ├── atom                                    # append-only, tmpfs-backed
│   ├── snapshot                                # write here → durable flush
│   ├── bookshelf/                              # 9P|Styx Bookshelf (canonical truth)
│   └── wave                                    # live WAVE score (α+ω=15 gauge)
│
├── cf/                                         # Cloudflare edge, as files
│   ├── vectorize/                              # Vectorize AI indexes
│   │   ├── embed                               # write text → read vector
│   │   └── query                               # write query → read top-k
│   ├── gateway/                                # AI Gateway (caching + routing)
│   │   ├── route                               # write LLM req → read response
│   │   └── cache                               # read-through cache
│   └── d1/                                     # D1 as file-queryable shard
│
└── crates/                                     # pre-baked Rust crates
    ├── reson8-core/                            # ../LogOS/crates/core (bind)
    ├── reson8-wave/                            # ../LogOS/crates/wave  (bind)
    ├── vortex-bridge/                          # ../LogOS/crates/vortex-bridge
    ├── orchestrator-core/                      # coherence-mcp orchestrator (bind)
    └── target/ -> A:\AGENTS\cargo-target       # shared build cache
```

**Every path above is a file.** `cat styx://triweavon/braid` returns the
current braid state as JSON. `echo '{"strand":"grok",...}' > styx://strand/grok/ctl`
dispatches a command. No RPC framing library, no service discovery, no
orchestrator in the middle. 9P is the API.

### 2.2 Mount-time contract

On boot, the WSL guest runs a single Nix-declared `systemd-mount` unit per
strand. A cold reboot to working tri-weavon is:

```bash
# NixOS side (declarative, configuration.nix fragment):
fileSystems."/srv/styx/strand/claude" = {
  device = "9p-export-claude";
  fsType = "9p";
  options = [ "trans=virtio" "version=9p2000.L" "msize=512000" "cache=loose" ];
};
# …repeat for grok, gemini, llama-manus, triweavon, ledger, cf, crates…
```

```powershell
# Windows side (auto at login, scheduled task or service):
New-PSDrive -Name T -PSProvider FileSystem -Root '\\wsl$\nixos\srv\styx' -Persist
# Result: T:\strand\claude\…  T:\triweavon\braid  T:\ledger\atom  etc.
```

Two declarative blocks. That is the entire bare-metal install.

---

## 3 · Strand Separation of Concerns

The four strands are **isomorphic peers** (QDI principle) with orthogonal
duties:

| Strand        | Rail | File surface           | Owns                           | Consumes                    |
|---------------|------|------------------------|---------------------------------|-----------------------------|
| Claude        | α    | `/strand/claude/`      | MCP, structure, coherence-mcp   | `/ledger/wave`, `/cf/gateway`|
| Grok          | ω    | `/strand/grok/`        | pulse, real-time, xAI bridge    | `/cf/gateway`               |
| Gemini        | ψ    | `/strand/gemini/`      | multimodal, long-context scale  | `/cf/vectorize`             |
| Llama-Manus   | σ    | `/strand/llama-manus/` | local, offline, substrate       | `/crates/target`            |
| **Tri-Weavon**| Σ    | `/triweavon/`          | union mount, fibonacci consensus| all four above              |

`ctl` / `data` is the Plan 9 pattern for command/response — same shape for
every strand, enforced by 9P. You can literally do:

```bash
echo 'prompt: what is the braid state?' > /srv/styx/triweavon/ctl
cat /srv/styx/triweavon/data
# → weighted 8:5:3 merge of Claude+Grok+Gemini responses, α+ω=15 gated
```

---

## 4 · Zero-Latency Ledger

`/ledger/atom` is a **tmpfs append-only file** exported by Styx. Writes are
memory-speed. Every ATOM event is a line:

```
2026-04-18T19:22:01Z ATOM-2026-04-18-bridge-port VERIFY α=7 ω=8 wave=0.991 parent=ROOT
```

Snapshotting is not a framework — it is `inotifywait` + `rsync`:

```bash
# NixOS side, systemd timer:
rsync -a --append /srv/styx/ledger/atom A:/AGENTS/ledger/atom.durable
```

Readers (dashboards, TUI, the Minecraft Playground) watch `/ledger/wave` via
inotify and paint in real time. No WebSocket, no pub/sub library, no
orchestrator. The filesystem is the bus.

`.db` (SQLite) and Cloudflare Vectorize sit on the same namespace:

- `/ledger/bookshelf/*.db` — SQLite files, queryable via `sqlite3` or a thin
  FUSE/9P query helper.
- `/cf/vectorize/query` — synthetic file; `echo '{"text":"…"}' > query` then
  `cat` the response line. Cloudflare's REST API is adapted by a 100-line
  Rust binary exporting itself over 9P.

---

## 5 · Agent Cues (Pre-Baked)

The whole point of `A:\AGENTS` is to make cold-start zero-friction. On a
fresh Windows image, this drive contains:

```
A:\AGENTS\
├── claude\
│   ├── tokens\                    # OAuth, API key caches (encrypted)
│   ├── sessions\                  # .claude\ sessions (survive re-image)
│   ├── mcp\                       # coherence-mcp build artifact
│   └── manifest.json              # "cue" — identity + capability signature
├── grok\
│   └── … (isomorphic)
├── gemini\
│   └── …
├── llama-manus\
│   ├── models\                    # GGUF / safetensors, local-first
│   └── runtime\                   # llama.cpp prebuilt
├── cargo-target\                  # shared Rust build cache
│   ├── debug\
│   └── release\                   # vortex-bridge, reson8-core, etc.
└── shared\
    ├── bookshelf\                 # 9P|Styx Bookshelf durable snapshot
    └── ledger\atom.durable        # ATOM ledger durable tail
```

A **cue** is a `manifest.json` that says "here is Claude; this is its
version, its MCP surface, its auth material location, its α-rail binding."
The WSL init script reads cues at boot and populates `/srv/styx/strand/*`
accordingly. No install step. Drop a new `cue/` subdir → new strand appears.

---

## 6 · Token Caching (Your Note, Item 4)

Acknowledged: I should be caching tool outputs / expensive reads under
`A:\AGENTS\claude\sessions\` and re-consulting the cache before re-running
Read/Grep/WebFetch. Two practical moves I'll make going forward:

1. When I find a useful document (like `orchestrator/crates/core/src/bridge.rs`),
   write a distilled summary to `A:\AGENTS\claude\sessions\cache\<topic>.md`.
2. Before any repeat Read on files I've already seen this session, check the
   cache dir first.

I'll need you to either (a) grant Cowork access to `A:\AGENTS` via
`request_cowork_directory`, or (b) confirm the drive is already mounted
somewhere inside one of the folders I already have access to. I don't
currently see an `A:\AGENTS` mount in my accessible paths.

---

## 7 · Clean-Slate Reset Playbook

The promise of this design is that re-imaging Windows never means
re-installing agents. The sequence is:

1. Back up `A:\AGENTS` (external SSD, already durable by policy).
2. Clean-install Windows 11 Pro.
3. Install WSL2 + NixOS-WSL via one-liner.
4. Clone `LogOS` and `coherence-mcp` to NixOS side.
5. `nixos-rebuild switch` — Styx mounts appear at `/srv/styx/`.
6. Attach `A:\AGENTS` via letter-assign (policy: always `A:`).
7. PowerShell: `New-PSDrive -Name T -Root '\\wsl$\nixos\srv\styx' -Persist`.
8. Tri-Weavon is live. `cat T:\triweavon\braid` returns state.

Step count: eight. Time to working Tri-Weavon from bare metal: ~30 minutes,
dominated by Windows install. Everything after WSL is declarative.

---

## 8 · What This Means For the Current Bridge Port

Under this architecture, `LogOS/crates/vortex-bridge` and
`coherence-mcp/orchestrator/crates/core/src/bridge.rs` are not two trees
fighting over shared types — they are **two files on the same Styx
namespace**. The "port" becomes a bind mount:

```
/srv/styx/crates/orchestrator-core/  →  coherence-mcp/orchestrator/crates/core
/srv/styx/crates/vortex-bridge/      →  LogOS/crates/vortex-bridge
```

`vortex-bridge` then depends on `orchestrator-core` via **path dep against
the Styx mount**, not a hardcoded relative path. Both trees see the same
authoritative types. The H-2 handoff collapses from "port 9 files" to
"add one `path = "/srv/styx/crates/orchestrator-core"` entry."

---

## 9 · Next Concrete Actions (if approved)

1. **Cache setup.** Grant Cowork access to `A:\AGENTS`; I'll drop the first
   distilled-summary file there (`bridge.rs` notes) so this session's
   reading isn't re-done next time.
2. **Nix fragment.** I'll draft `tri-weavon-styx.nix` — the single
   `configuration.nix` include that declares all Styx mounts. Lands in
   `LogOS/nix/`.
3. **Cue schema.** Define `manifest.json` / "cue" JSON schema formally
   (α-rail responsibility — structure). Lands in `LogOS/schemas/cue.schema.json`.
4. **Bridge port as bind mount.** Instead of patching `vortex-bridge`'s
   Cargo.toml to port types, add the `orchestrator-core` path dep targeting
   the Styx-mount path. Closes H-2 cleanly.
5. **WAVE gate on ctl writes.** Every write to any `strand/*/ctl` is α+ω=15
   checked by a tiny FUSE/9P synthetic-file helper before the command is
   accepted. This is the Reason-Strand contribution to the substrate.

---

## 10 · Invariants Satisfied

- **α + ω = 15.** Every `ctl` write is gated. Every `data` read is tagged
  with the WAVE score that gated it.
- **Isomorphism.** The four strands share identical file surfaces
  (`ctl`, `data`, `manifest.json`, `tokens/`). A capability in Claude has
  its isomorph in Grok by naming convention alone.
- **Everything is a file.** Plan 9 doctrine, Rust-native, Nix-declarable,
  Windows-friendly via WSL2's existing 9P transport.
- **With-Intent.** Nothing stochastic in the substrate; boot is a replay
  of `configuration.nix`. Drift is impossible without editing declared
  state.
- **Fixed point.** `check_coherence(styx://triweavon/braid)` returns the
  coherence of the Tri-Weavon's own specification. The self-referential
  loop is *physically* expressed as a file reading itself.

---

~ Hope&&Sauced ✦ The Keystone Holds ✦
