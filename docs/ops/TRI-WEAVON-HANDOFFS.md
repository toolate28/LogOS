# Tri-Weavon Hand-Offs — Workspace Build Reducibility

**Origin strand:** Claude / Reason
**Generated:** 2026-04-18
**Invariant:** α + ω = 15 ✓ (all upstream verification passing)
**Audience:** Grok (Real-Time / Forge), Gemini (Multimodal / Scale), Llama-Manus (Local / Substrate), Matthew

## Purpose

`cqk-kitty-rips-verify` compiles and passes 24 property tests (2048–4096
randomised cases each) on rustc 1.94.0. Five sibling crates in the root
workspace still block `cargo check --workspace --all-targets`. This document
captures the reducible space around each — what Reason has already verified,
what remains ambiguous, and which strand is the natural owner.

Each block has a **probability-space reduction**: a narrowing of plausible
fixes based on static evidence. If a strand can answer the residual question,
append a `### Reduction — <strand>` section with the chosen branch.

## Summary table

| ID | Crate | Residual ambiguity | Deterministic? | Natural owner |
|----|-------|--------------------|----------------|---------------|
| H-1 | `reson8-tui` | Does `reson8-forge-core` exist, or is it an alias? | No — needs intent | Grok (Forge) |
| H-2 | `reson8-vortex-bridge` | `[lib] name` override vs rename bin imports | Yes — two crisp patches | Reason can apply |
| H-3 | `reson8-sysctl` | Orphan backend vs land missing substrate module | No — design call | Llama-Manus (Substrate) |
| H-4 | `cqk-microlocal` | serde for `[[f64; D]; N]` const-generic arrays | Yes — one pattern | Reason can apply |
| H-5 | `near/conservation-verifier` | Root workspace vs separate build lane | Yes — workspace config | Reason can apply (needs sign-off) |

Three of the five are deterministic from where Reason stands. Two require
another strand to collapse the ambiguity.

---

## H-1 — `reson8-tui` (Forge wiring)

### Surface

```
crates/tui/src/app.rs:5:5   E0433: unresolved module `reson8_forge_core`
crates/tui/src/ui.rs:18:5   E0433: unresolved module `orchestrator_core`
crates/tui/src/main.rs:98   E0433: unresolved module `serde_json`
```

### Static evidence (Reason)

- `crates/tui/Cargo.toml` declares deps: `reson8-core`, `reson8-wave`,
  `reson8-topology`, `ratatui`, `crossterm`, `tokio`, `serde`. No
  `reson8-forge-core`. No `orchestrator-core`. No `serde_json`.
- Glob search finds no `crates/reson8-forge-core/` directory. The name only
  appears under `crates/coherence-mcp/.../worktrees/*/orchestrator/crates/core/`
  — i.e. inside a *different* nested workspace.
- `use reson8_forge_core::{adapter, bridge, protocol, superskill, task}`
  references a non-trivial API surface (Provider, BridgeEvent, TelemetryPayload,
  LogEntry, LogLevel, LogSink, MemoryLogSink, PipelineStatus, SuperskillEvent,
  Task, TaskPhase). This is substantial — not a typo.

### Probability reduction

| Branch | Plausibility | Evidence |
|--------|--------------|----------|
| (a) `reson8-forge-core` is a planned crate not yet in root workspace | **High** | API surface is coherent; TUI is its consumer-front; coherence-mcp's nested worktree hints it exists elsewhere |
| (b) `reson8-forge-core` was renamed from a shipped crate | Low | None of the existing crates expose adapter/bridge/protocol/superskill/task symbols |
| (c) `reson8_forge_core` refers to an external crate | Very low | No external crate of that name exists |

### Residual question → Grok (Forge strand)

**Where does `reson8-forge-core` live, and what's the plan to register it in
the root workspace?** If branch (a) is correct: is there a path we should add
to `[workspace] members`, or is Forge still under development and the TUI
should gate its Forge panels behind a `#[cfg(feature = "forge")]`?

Secondary: `serde_json` is almost certainly a trivial missing dep — add
`serde_json = "1"` to `crates/tui/Cargo.toml`. Reason can apply that
independently once Forge's status is clarified.

---

## H-2 — `reson8-vortex-bridge` (naming + deps)

### Surface

```
crates/vortex-bridge/src/main.rs:1   E0432: unresolved import `tracing`
crates/vortex-bridge/src/main.rs:2   E0432: unresolved import `tracing_subscriber`
crates/vortex-bridge/src/main.rs:3   E0432: unresolved import `vortex_bridge`
```

### Static evidence (Reason)

- Package name: `reson8-vortex-bridge` → default lib name: `reson8_vortex_bridge`
- The bin writes `use vortex_bridge::server;` — stripping the `reson8_` prefix.
- Cargo.toml has no `tracing` or `tracing-subscriber` entries.

### Probability reduction

| Branch | Plausibility | Evidence |
|--------|--------------|----------|
| (a) `[lib] name = "vortex_bridge"` override missing | **High** | Cleaner ergonomic import; package name uniqueness preserved for registry |
| (b) Bin should use `reson8_vortex_bridge::server` | Medium | Less invasive but cross-strand naming inconsistency (elsewhere use cases drop `reson8_` prefix) |

### Deterministic patch (Reason-ready)

**Proposed — add to `crates/vortex-bridge/Cargo.toml`:**

```toml
[lib]
name = "vortex_bridge"

[dependencies]
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["fmt"] }
```

Matches Gemini's Scale-strand naming discipline (package names prefixed,
lib/bin names short). No cross-strand blockers; Reason can apply.

---

## H-3 — `reson8-sysctl` (orphan backend)

### Surface

```
crates/sysctl/src/backends/winget.rs:3    unresolved `async_trait`
crates/sysctl/src/backends/winget.rs:5    unresolved `crate::package`
crates/sysctl/src/backends/winget.rs:6    unresolved `crate::ExecResult`, `crate::Substrate`
crates/sysctl/src/backends/winget.rs:16+  unresolved `anyhow` (×8)
```

### Static evidence (Reason)

- `crates/sysctl/src/lib.rs` contains literally:
  ```rust
  //! reson8-sysctl — System control and hardware grounding
  pub mod backends;
  ```
  No `package` module, no `ExecResult`, no `Substrate`.
- `winget.rs` references `Package`, `PackageManager`, `PackageSource` (enum
  with `search_cmd(&str)`, `install_cmd(&str)` methods), plus crate-root
  `ExecResult { stdout, … }` and `Substrate`.
- `async_trait`, `anyhow` not declared in Cargo.toml.

### Probability reduction

| Branch | Plausibility | Evidence |
|--------|--------------|----------|
| (a) winget.rs is orphaned from a fork — park it with `#[cfg(disabled)]` | Medium | Zero-effort unblock; keeps substrate pure |
| (b) The substrate abstraction is on the roadmap — land `package.rs` + root types now | **High** | winget.rs's shape is coherent enough to *specify* the missing module; LogOS has Substrate hardware-grounding ambitions |

### Residual question → Llama-Manus (Substrate strand)

**Is the `Substrate` + `PackageManager` abstraction part of the current
LogOS plan, or is winget.rs a scout-ahead from a future iteration?**

If landing now: Reason can author the spec:

```rust
// crates/sysctl/src/lib.rs
pub mod backends;
pub mod package;

pub struct ExecResult {
    pub stdout: String,
    pub stderr: String,
    pub status: i32,
}

pub trait Substrate { /* … TBD with Llama-Manus */ }
```

```rust
// crates/sysctl/src/package.rs
#[async_trait::async_trait]
pub trait PackageManager {
    fn source(&self) -> PackageSource;
    async fn search(&self, query: &str) -> anyhow::Result<Vec<Package>>;
    async fn install(&self, pkg_id: &str) -> anyhow::Result<crate::ExecResult>;
    async fn uninstall(&self, pkg_id: &str) -> anyhow::Result<crate::ExecResult>;
}

pub struct Package { /* … */ }

pub enum PackageSource { WinGet, Apt, Homebrew, Pacman, Flatpak, Snap, Nix }
impl PackageSource {
    pub fn search_cmd(&self, query: &str) -> Vec<String> { /* … */ }
    pub fn install_cmd(&self, pkg_id: &str) -> Vec<String> { /* … */ }
}
```

If parking: one-line patch to `crates/sysctl/src/backends/mod.rs` (or equivalent)
gating `winget` behind a disabled feature.

---

## H-4 — `cqk-microlocal` (serde const-generic arrays)

### Surface

```
crates/cqk-microlocal/src/lib.rs:80  E0277: [[f64; D]; N]: serde::Serialize not satisfied
crates/cqk-microlocal/src/lib.rs:86  E0277: [[f64; D]; N]: serde::Deserialize not satisfied
```

### Static evidence (Reason)

- Field `pub fibre_angles: [[f64; D]; N]` on `CosphereLift<const D: usize, const N: usize>`.
- serde stdlib derives only implement Serialize/Deserialize for fixed arrays
  up to length 32 (const `impl` blocks). A const-generic `[T; N]` is not
  covered unless the user opts in.

### Probability reduction

**High confidence — one canonical fix.** Two sub-variants:

| Variant | Tradeoff |
|---------|----------|
| `serde-big-array` crate with `#[serde(with = "BigArray")]` | Third-party dep, well-established |
| Manual `impl Serialize/Deserialize` for `[[f64; D]; N]` wrapper | No new dep, more code |

### Deterministic patch (Reason-ready)

Preferred — add to `crates/cqk-microlocal/Cargo.toml`:

```toml
serde-big-array = "0.5"
```

Then in `lib.rs`:

```rust
use serde_big_array::BigArray;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CosphereLift<const D: usize, const N: usize> {
    pub base_mesh: BaseMeshRef,
    #[serde(with = "BigArray")]
    pub fibre_angles: [[f64; D]; N],
    pub omega_tilde: Vec<f64>,
}
```

No cross-strand blockers. Reason can apply.

---

## H-5 — `near/conservation-verifier` (near-sdk tripwire)

### Surface

```
near-sdk 5.24.1 emits a compile error on host-target cargo check/build,
directing to `cargo near build` (wasm32-unknown-unknown) only.
```

### Static evidence (Reason)

- `near/conservation-verifier/Cargo.toml` has `[lib] crate-type = ["cdylib"]`
  and `near-sdk = "5"` — a NEAR smart contract.
- Root `Cargo.toml` lists `"near/conservation-verifier"` as a workspace member.
- near-sdk 5.x is *intentionally* hostile to `cargo build` without the
  `cargo near` wrapper; only the following cfgs are valid: `target_family = "wasm"`,
  `feature = "non-contract-usage"`, `feature = "unit-testing"`, `feature = "__abi-generate"`, `test`, `doctest`, `clippy`.

### Probability reduction

**High confidence — known NEAR ergonomics pattern.** Three variants:

| Variant | Tradeoff |
|---------|----------|
| Root `[workspace] default-members` excludes `near/conservation-verifier` | `cargo check --workspace` still breaks; `cargo check` alone OK; need explicit `-p` to touch NEAR |
| Remove NEAR crate from workspace entirely; it becomes a standalone sub-crate | Cleaner isolation; CI gets a separate lane |
| Add `near-sdk` feature `non-contract-usage` to suppress tripwire | Hacky; not what near-sdk wants |

### Proposed patch (Reason-ready, pending sign-off)

Preferred — convert the NEAR crate to its own isolated workspace:

```toml
# Root Cargo.toml
[workspace]
members = [
    # ... (remove "near/conservation-verifier")
]
exclude = [
    "near/conservation-verifier",
]
```

Then give `near/conservation-verifier/` its own top-level `[workspace]` table
(or leave it standalone with path-deps resolved). CI gets:

- Host lane: `cargo check --workspace` from repo root
- Contract lane: `cd near/conservation-verifier && cargo near build` (or
  `cargo check --target wasm32-unknown-unknown`)

Matches standard NEAR project template. No cross-strand blockers, but this
touches CI topology — Matthew's call on timing.

---

## Verification cadence

Once any hand-off reduces, Reason re-runs:

```powershell
cargo check --workspace --all-targets --message-format short
cargo test -p cqk-kitty-rips-verify
```

and appends a **### Verification stamp** to the corresponding section with
the result. The universal invariant α + ω = 15 is re-asserted as the final
line of each stamp.

## Reductions appended below this line

<!-- Grok and Gemini: append `### Reduction — <your strand>` under the
     relevant section, naming the chosen branch and any auxiliary evidence
     you bring in. Reason will then re-verify and stamp. -->

With-Intent.
