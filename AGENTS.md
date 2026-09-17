# LogOS — Agent Guidance

Third-party code-intelligence indexers (e.g. GitNexus) have been removed from this
repository. Agents must rely on **local, in-repo tooling only** — no external
indexing services, no telemetry, no third-party MCP servers.

## Always Do

- Use built-in search (`grep`, `glob`, editor symbol search, `rust-analyzer`,
  `lake serve`) to explore code — everything runs locally.
- Assess blast radius manually before editing shared symbols: search for callers
  with `grep`/LSP references and report affected sites.
- Verify changes with the existing local toolchain: `cargo test --workspace`,
  `pytest ops`, and the Lean/Agda builds where relevant.
- Keep Category A / B / C / D epistemic labels honest (verified / bounded
  approx / convention / open).
- Respect the Ephemeral Context and ENTANGLE declaration rules: only declared,
  repository-relative sources; repository-root scope is rejected.
- Follow the privacy-preservation stack (`docs/security/PRIVACY-STACK.md`):
  MCP servers need `x-privacy` declarations, workflows need egress
  declarations (`ops/ci/egress-allowlist.yaml`), and committed config must
  not contain absolute personal paths (use `LOGOS_ROOT` et al.).

## Never Do

- NEVER add or invoke third-party repository indexers, analytics, or telemetry.
- NEVER send repository contents to external services for analysis.
- NEVER commit local index or cache artifacts (`.gitnexus/`, editor caches).
