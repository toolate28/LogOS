# Security Policy

## Supported Versions

Security attention currently applies to the following surfaces on the
default branch (`main`):

| Surface | Supported | Notes |
|------------------------------------------------------------------------|-----|--------------------------------------|
| `main` (workspace as a whole)                                          | ✅ | Active development line               |
| Pinned Lean toolchain (`leanprover/lean4:v4.8.0`) + mathlib4 @ v4.8.0  | ✅ | Formal layer pin                      |
| Published / path-dependent crates used by apps (cutile, core, wave, …) | ✅ | Runtime surface                       |
| `@toolated/coherence-mcp` (current package version on npm)             | ✅ | MCP tool surface                      |
| Historical tags / older commits                                        | ❌ | No backport commitment                |
| Prototype kernels under `kernels/` with explicit TODOs                 | ❌ | Not a supported security surface      |
| Modules carrying `sorry` / `axiom` / `{!!}`                            | ❌ | Placeholders are not security defects |

Only the active `main` line and the explicitly pinned toolchains above
are in scope for security updates.

## Reporting a Vulnerability

**Do not** open public issues, discussions, or pull requests for
security-sensitive findings.

Prefer, in order:

1. GitHub private vulnerability reporting for this repository
   (Security → Advisories / Report a vulnerability), when enabled.
2. Direct contact with the repository owner via the address published
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
days. After acknowledgement we will assess the report and indicate
whether it is accepted, declined, or needs more information.

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
- Issues that allow silent promotion of unverified claims into
  trusted runtime state

**Out of scope (examples)**

- Presence of `sorry`, `axiom`, or `{!!}` in formal modules
  (these are explicit proof obligations, not vulnerabilities)
- Documentation drift or capability-map version skew
- Theoretical gaps in formal models that do not affect executable
  behaviour
- Prototype / TODO kernels that are not on a supported release path
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
