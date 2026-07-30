# Security Policy — `@toolated/coherence-mcp`

> **"From the constraints, gifts. From the spiral, safety."**

Package: `@toolated/coherence-mcp` · Repo: `github.com/toolate28/coherence-mcp`  
*(scope and org differ — package scope `@toolated` vs GitHub `toolate28`)*

This document is the **package threat surface** for the MCP server. The monorepo
root [`SECURITY.md`](../../SECURITY.md) covers LogOS as a whole. Tool schemas in
`mcps/coherence-mcp/tools/` are LogOS descriptors; the published npm package is
the executable authority when versions disagree.

---

## Supported versions

| Version | Supported | Status |
|---------|-----------|--------|
| **0.4.x** | ✅ | current — shipped `0.4.2`, security fixes land here |
| 0.3.x | ⚠️ | previous minor — upgrade; fixes only if trivially backportable |
| < 0.3 | ❌ | development-era, unsupported |

Check what you are actually running — the published version and the string the server
reports have drifted:

```bash
npm view @toolated/coherence-mcp version   # 0.4.2 — authoritative
```

> **Known drift:** `src/index.ts` reports `server.version` as `0.3.2`. This is a
> cosmetic string defect, not a version-skew vulnerability, and is tracked as P0
> upstream. Do not use the reported string to determine patch level.
> See also [COHERENCE-MCP-0.4.2-DRIFT.md](COHERENCE-MCP-0.4.2-DRIFT.md).

---

## Reporting a vulnerability

**Do not open a public GitHub issue for a vulnerability.**

- [GitHub Security Advisories](https://github.com/toolate28/coherence-mcp/security/advisories/new) — preferred for the package
- LogOS monorepo: [LogOS advisories](https://github.com/toolate28/LogOS/security/advisories/new) when the finding is monorepo-local
- Email `security@safespiral.org`, or reach `@toolated` / the owner profile on GitHub

> **Verify the channel before relying on it.** The advisory email domain
> (`safespiral.org`) differs from the project homepage (`spiralsafe.org`). If you do not
> receive an acknowledgement within 48 hours, use the GitHub advisory path — that one is
> tied to the repo and cannot be misrouted.

### Include

1. Description of the vulnerability  
2. Steps to reproduce, or a proof of concept  
3. Impact assessment  
4. Affected versions, if known  
5. A suggested fix, if you have one  

### Response

| Stage | Target |
|-------|--------|
| Acknowledgement | 48 hours |
| Severity assessment | 1 week |
| Fix — critical | 1–3 days |
| Fix — high | 1–2 weeks |
| Fix — medium | 2–4 weeks |
| Fix — low | next release cycle |

Process: report → acknowledge → classify → fix in a private branch → test → coordinated
disclosure with the reporter → patch release → public advisory.

---

## Threat surface — read this before you trust it

This server exists to give an AI agent reach. That reach is the risk, and it is
deliberate. **SafeSkill scans have reported ≈ 65/100 on the npm package** — read that as an
accurate description of surface area, not as a verdict.

| Surface | Risk | Mitigation |
|---------|------|------------|
| **MCP tool execution** | tools run with your permissions and touch the filesystem | schema validation on every argument; writes confined to `.atom-trail/` under the configured mount; path sanitisation against traversal; all invocations logged |
| **Script runner** | arbitrary execution | **allow-list only** (`SCRIPT_ALLOW_LIST`); no free-form exec |
| **Connectors** (`github_*`, `jira_*`, `postgres_*`, `fetch_url`, `slack_notify`) | credentialled outbound reach | least-privilege tokens; per-connector allow-lists; timeouts and byte caps |
| **`ops_deploy`** | production side effects | guarded and disabled by default; dry-run first; never in a smoke test |
| **`mc_exec`** | RCON against a live server | treat as production; do not point at a survival world |
| **External binary** (`WAVE_TOOLKIT_BIN`) | executes a path you supply | optional; falls back to internal heuristics; path validated; timeout and resource limits |
| **Strand APIs** (`grok_*`, `gemini_*`, `openweight_*`) | your content leaves the machine | opt-in per key; no key, no call |
| **Dependencies** | upstream vulnerabilities | `npm audit` in the release path; Dependabot; minimal footprint |

**No telemetry.** Local processing by default; no data collection. What leaves the machine
leaves because you configured a token that sends it.

### MCP vs agent firewall (LogOS)

The Copilot agent Bash firewall does **not** constrain MCP traffic. Complementary
controls in this monorepo:

- Org/enterprise MCP registry + **Registry only** — [MCP-REGISTRY.md](MCP-REGISTRY.md)
- CI `mcp-validation.yml` — no `tools: ["*"]`, no embedded secrets
- Prefer GitHub MCP **readonly** endpoint
- [MCP-RISK-MITIGATIONS.md](MCP-RISK-MITIGATIONS.md)

---

## Hardening checklist

- [ ] Set `ATOM_AUTH_TOKEN` (or `ATOM_AUTH_HMAC_SECRET`) — do not run tokenless in a shared environment  
- [ ] Populate allow-lists explicitly; an empty allow-list should mean *deny*, so verify it does  
- [ ] Bound external calls: `WAVE_TIMEOUT_MS`, `WAVE_MAX_BYTES`  
- [ ] Keep `ops_deploy` disabled until a human has reviewed the target  
- [ ] Scope every connector token to the minimum repo / project / database it needs  
- [ ] Confirm the audit trail carries `requestId` and caller context  
- [ ] Protect the MCP client config file — it holds the tokens  
- [ ] `npm audit` and `npm audit signatures` before upgrading  
- [ ] Prefer snake_case tool names (`wave_analyze`, not `wave.analyze`) against 0.4.2  

---

## Package integrity

### npm provenance

```bash
npm audit signatures @toolated/coherence-mcp
```

### GPG-signed releases

```bash
# 1. import the signing key
curl -s https://spiralsafe.org/.well-known/pgp-key.txt | gpg --import
# or from the package repo
curl -s https://raw.githubusercontent.com/toolate28/coherence-mcp/main/.well-known/pgp-key.txt | gpg --import

# 2. fetch checksums + signature for the version you are installing
VERSION="0.4.2"
curl -LO "https://github.com/toolate28/coherence-mcp/releases/download/v${VERSION}/SHA256SUMS.txt"
curl -LO "https://github.com/toolate28/coherence-mcp/releases/download/v${VERSION}/SHA256SUMS.txt.asc"

# 3. verify the signature, then the tarball
gpg --verify SHA256SUMS.txt.asc SHA256SUMS.txt
npm pack @toolated/coherence-mcp@${VERSION}
sha256sum -c SHA256SUMS.txt
```

Upstream helper: `./scripts/verify-release.sh 0.4.2` in `toolate28/coherence-mcp`.

**Always verify the key fingerprint through more than one channel** before trusting it.
If `spiralsafe.org` and the repo `.well-known/pgp-key.txt` disagree, stop.

---

## For contributors

1. **Never commit secrets.** No keys, tokens, or `.env` files. Review the diff.  
2. **Validate every input.** Sanitise paths, check for injection, validate tool parameters against the schema — including the reject path, in a test.  
3. **Least privilege.** Request only what the tool needs; document the permission and why.  
4. **Audit dependencies.** `npm audit` before adding one; justify it in the PR.  
5. **Fail loudly and safely.** Return a structured error. A crashed server is a denial of service.  

LogOS CI mirrors: `ops/ci/validate_mcp_schemas.py`, `ops/ci/validate_mcp_config.py`,
`.github/workflows/mcp-validation.yml`.

---

## Built-in controls

- **Input validation** — Ajv schemas on tool parameters; path sanitisation; argument validation before any execution  
- **Audit logging** — invocations logged with request ID, timestamp, and caller context  
- **Rate limiting** — configurable per-tool invocation limits  
- **Scope checks** — tools restricted by scope; sensitive operations require explicit approval  
- **Bounds** — size and time limits on every external call  

Gaps, stated plainly: **ATOM-AUTH scope issuance is not implemented** (tokens are accepted;
fine-grained scope issuance is upstream P1), and there is no queue/retry/backoff layer yet,
so a hostile or flaky upstream degrades the call rather than the server.

---

## Incident response

Contain → assess scope and impact → notify `security@safespiral.org` (or GitHub advisory
if email is unconfirmed) → remediate → document what was learned → coordinate public disclosure.

---

## WAVE scale (canonical)

Publish / handoff gate for LogOS + package docs: **85 on 0–100** (= 0.85).  
See [WAVE-SCALE.md](WAVE-SCALE.md). SAIF-era 0.98 is superseded as the default publish gate.

---

α + ω = 15 (Category C) · tomczak_preserved · capability ≠ authority · Music conserved
