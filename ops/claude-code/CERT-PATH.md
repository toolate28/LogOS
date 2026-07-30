# Claude Code cert path

**ATOM:** `ATOM-CC-CERT-PATH-20260722`  
**Rule:** Config is git. State is backup. Keyless.

## Paths

| Kind | Path | Tracked? |
|------|------|----------|
| Init packet | `CLAUDECODE-INIT-v0_1.md` | git |
| Settings | `.claude/settings.json` → `cold_start.cert_*` | git |
| Schema | `ops/claude-code/cert.schema.json` | git |
| Emit script | `ops/claude-code/Emit-ClaudeCodeCert.ps1` | git |
| **latest cert** | `.atom-trail/certs/claude-code/latest.json` | **state** (gitignored) |
| Timestamped copy | `.atom-trail/certs/claude-code/YYYYMMDD-HHMMSS.json` | **state** (gitignored) |
| Local README | `.atom-trail/certs/claude-code/README.md` | local only |

## Contract

```
pass: true  ⟺  surfaces.spiralsafe_wrangler.ok
            ∧  surfaces.vcs.ok
            ∧  surfaces.formal_core.ok
            ∧  keys_present = false
            ∧  no false-green formal/LSP placeholders
```

Deploy must **refuse** if `pass: false` or `keys_present: true`.

## Tree-state binding (required before countersign)

Cert JSON **must** include `head_sha` = full `git rev-parse HEAD` at survey time.

Countersign commit trailers:

```
Mark-Cert-Head: <same 40hex as cert.head_sha>
```

**Invariant (D6):** first parent of the countersign commit **equals** `Mark-Cert-Head`.

**Invariant (D7 — claim drift):** for each mark being countersigned, claimed artifacts must be unchanged between ledger `head_sha` (BUILD’s claim tree) and cert `head_sha` / `Mark-Cert-Head` (VERIFY’s survey tree):

```powershell
# Preflight before countersign (refuse if exit ≠ 0 on any claimed path)
pwsh -File ops/marks/Query-MarkDetectors.ps1 -CertHead HEAD
```

If D7 fires: **do not countersign**. BUILD issues a new `Mark-Id` over the new tree. Exclude `ops/marks/MARKS.jsonl` from the path set (stamp bookkeeping is not a claim surface).

If the branch is `behind` origin, reconcile (or re-survey after merge) **before** emitting the cert. Do not countersign a local tip that will be rewritten by an unseen remote history without re-running the survey **and** D7.

`α + ω = 15` and seat sum `16` are independent Category C conventions — countersign does not promote either to A. Countersign never promotes B→A either; VERIFY A requires a **new subject** (VERIFY’s own observation).

## Emit

```powershell
# Survey-only cert (honest B until real Claude Code init fills detail):
pwsh -File ops/claude-code/Emit-ClaudeCodeCert.ps1

# After human/Claude Code cold-start review:
pwsh -File ops/claude-code/Emit-ClaudeCodeCert.ps1 -AsInitRun
```

`α + ω = 15` is Category C label only — never a pass threshold.
