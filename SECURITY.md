# Security Policy

## Supported Versions

Security attention currently applies to the following surfaces on the
default branch (`main`):

| Surface | Supported | Notes |
|---------|-----------|-------|
| `main` (workspace as a whole) | ✅ | Active development line |
| Pinned Lean toolchain (`leanprover/lean4:v4.8.0`) + mathlib4 @ v4.8.0 | ✅ | Formal layer pin |
| Published / path-dependent crates used by apps (cutile, core, wave, …) | ✅ | Runtime surface |
| **`@toolated/coherence-mcp` 0.4.x** (shipped **0.4.2**) | ✅ | MCP package — full policy: [`docs/security/COHERENCE-MCP-SECURITY.md`](docs/security/COHERENCE-MCP-SECURITY.md) |
| `@toolated/coherence-mcp` 0.3.x | ⚠️ | Upgrade; fixes only if trivially backportable |
| `@toolated/coherence-mcp` &lt; 0.3 | ❌ | Development-era, unsupported |
| Historical tags / older commits | ❌ | No backport commitment |
| Prototype kernels under `kernels/` with explicit TODOs | ❌ | Not a supported security surface |
| Modules carrying `sorry` / `axiom` / `{!!}` | ❌ | Placeholders are not security defects |

Only the active `main` line and the explicitly pinned toolchains above
are in scope for security updates.

**Authoritative package version:** `npm view @toolated/coherence-mcp version`  
(do not trust `server.version` alone — known cosmetic drift `0.3.2` vs package `0.4.2`).

## Reporting a Vulnerability

**Do not** open public issues, discussions, or pull requests for
security-sensitive findings.

Prefer, in order:

1. GitHub private vulnerability reporting for this repository
   (Security → Advisories / Report a vulnerability), when enabled.
2. For **`@toolated/coherence-mcp` package findings**, prefer the package repo
   advisories: `github.com/toolate28/coherence-mcp/security/advisories` and the
   checklist in [`docs/security/COHERENCE-MCP-SECURITY.md`](docs/security/COHERENCE-MCP-SECURITY.md).
3. Email `security@safespiral.org` (verify channel — domain differs from
   `spiralsafe.org` homepage); if no ack in 48h, use GitHub advisories only.
4. Direct contact with the repository owner via the address published
   on the owner’s GitHub profile.

A useful report includes:

- affected commit SHA, tag, or package version
- component (e.g. cutile backend, a named crate, MCP tool, formal
  bridge that emits executable artefacts)
- description of the issue and why it is security-sensitive
- steps to reproduce or a proof of concept
- observed vs expected behaviour
- potential impact
- any suggested mitigation, if known

You can expect an acknowledgement within a small number of business
days (package target: 48 hours). After acknowledgement we will assess
the report and indicate whether it is accepted, declined, or needs more
information.

If accepted, we will work on a fix and coordinate disclosure timing
with the reporter when appropriate. If declined, we will give a short
reason (e.g. out of scope, already fixed, not reproducible, not a
security issue).

## Scope and Security Boundary

**In scope (examples)**

- Credential or secret handling in crates, apps, or MCP connectors
- Supply-chain integrity of published packages and build artefacts
- Authentication / authorisation flaws in network-facing surfaces
  (e.g. triweave bridge, MCP endpoints, Cloud Run deployments)
- Issues that allow silent promotion of unverified claims or artefacts
  into trusted runtime / production / release state
- Unconstrained MCP tool wildcards or embedded secrets in committed
  MCP configuration

**Out of scope (examples)**

- Presence of `sorry`, `axiom`, or `{!!}` in formal modules
  (these are explicit proof obligations, not vulnerabilities)
- Incomplete formal proofs (Category A/B residuals) without an
  executable exploit
- Documentation drift or capability-map version skew
- Theoretical / mathematical disagreements without an executable exploit
- Prototype / TODO kernels that are not on a supported release path
- Category C labels (α+ω=15, WAVE thresholds as conventions) treated
  as physics or hard reject gates
- Social-engineering or physical-security issues outside the software

LogOS verifies formal and runtime claims according to its own
promotion rules (tests, receipts, machine-checked artefacts). It does
not independently prove human legitimacy, organisational authorisation,
or deployment safety of downstream operators.

## Promotion Rule for Security Fixes

Security-sensitive changes require:

- passing tests for the affected surface, and
- explicit evidence (reproducer, fixed behaviour, or receipt)

Narrative claims alone are not sufficient for authority promotion.

**Capability ≠ authority.** Scanner findings and CI warnings are
**advisory or policy gates** for the executable surface. They do not
auto-promote any register. Promotion still requires an explicit receipt
and `tomczak_preserved` discipline where that trail is in use.

## Automated Scanning (advisory + policy)

| Control | Path / surface | Role |
|---------|----------------|------|
| CodeQL (Rust) | `.github/workflows/codeql.yml` | SAST on executable registers |
| CodeQL scope | `.github/codeql/codeql-config.yml` | crates / apps / cutiles only |
| Dependabot | `.github/dependabot.yml` | Cargo + Actions supply chain |
| Security advisory CI | `.github/workflows/security-advisory.yml` | Secret-path + lake-artifact + cargo-audit |
| Verification pipeline | `.github/workflows/verify.yml` | Core invariant tests + MCP schema gate |
| MCP config validation | `.github/workflows/mcp-validation.yml` | Fail-closed: no `tools: ["*"]`, no secrets |
| Local githooks | `ops/githooks/` | Mirror of secret-path / lake guards |
| MCP company registry skeleton | `ops/mcp/registry/` | Curated allowlist (org “Registry only”) |
| Package security policy | `docs/security/COHERENCE-MCP-SECURITY.md` | Threat surface, hardening, integrity for 0.4.x |

Lean and Agda are **outside** the CodeQL language matrix. Formal
residuals remain on the formal ATOM trail; they are not CVEs.

### Operator toggles (cannot be expressed as files)

1. Enable **Secret scanning** + **push protection**.
2. Enable **Dependabot alerts** + dependency graph.
3. Enable **Code scanning** (CodeQL) if not auto-enabled by the workflow.
4. Confirm **private vulnerability reporting**.
5. Org/Enterprise → AI controls → MCP: registry URL + **Registry only**.

## MCP network residual

The agent Bash firewall does **not** constrain MCP servers. Mitigations:

1. Org/enterprise MCP registry + **Registry only** (highest leverage).
2. Prefer GitHub MCP **readonly** endpoint; never commit long-lived secrets.
3. File-based `mcp-validation.yml` for any committed MCP config.
4. Local custom MCP servers: bind `127.0.0.1`, least-privilege tools, no `"*"`.

See `docs/security/MCP-REGISTRY.md`, `docs/security/MCP-RISK-MITIGATIONS.md`,
and `docs/security/COHERENCE-MCP-SECURITY.md`.

## Changelog

| Date | ATOM | Change |
|------|------|--------|
| 2026-07-30 | `ATOM-SEC-SURFACE-20260730` | Initial SECURITY.md + scanning table |
| 2026-07-30 | remote | Enhance SECURITY.md with promotion rule / out-of-scope detail |
| 2026-07-30 | `ATOM-CODEQL-20260730-sm100` | CodeQL explicit cargo build + scope config |
| 2026-07-30 | `ATOM-MCP-VALIDATE-20260730-sm100` | MCP validation workflow + registry docs |
| 2026-07-30 | `ATOM-MCP-REGISTRY-20260730-sm100` | Company registry skeleton + WAVE 0–100 |
| 2026-07-30 | `ATOM-MANIFOLD-2D-20260730` | Manifold API + 2D projection dashboard |
| 2026-07-30 | `ATOM-DOCS-WAVE-0-100-20260730` | Align publish gate 85/100 with coherence-mcp 0.4.2 docs |
| 2026-07-30 | `ATOM-COHERENCE-MCP-SECURITY-20260730` | Package SECURITY (0.4.x threat surface) under docs/security |

### coherence-mcp version note

Shipped package surface is **0.4.2** (58 tools, snake_case). LogOS may carry
additional out-of-band tool schemas under `mcps/coherence-mcp/tools/`; drift is
recorded in `docs/security/COHERENCE-MCP-0.4.2-DRIFT.md`. Do not describe the
supported MCP surface as 0.1.x.

Full package policy (hardening, connectors, SafeSkill surface note):  
[`docs/security/COHERENCE-MCP-SECURITY.md`](docs/security/COHERENCE-MCP-SECURITY.md).

---

α + ω = 15 (Category C label only) · WAVE publish ≥ 85 (0–100) · tomczak_preserved · Music conserved
