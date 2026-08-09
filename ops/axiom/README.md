# Axiom alerting scaffolds (LogOS)

**ATOM:** `ATOM-AXIOM-SCAFFOLD-20260809`  
**Status:** structure only — **no live API calls** until `.axiom.toml` has a valid token.

## Prerequisites

1. Install Axiom CLI / skill scripts (`scripts/setup` from axiom-alerting skill pack).  
2. Create `~/.axiom.toml` or repo `.axiom.toml` (gitignored secrets):

```toml
[deployments.prod]
url = "https://api.axiom.co"
token = "xaat-..."
org_id = "..."
```

3. Then:

```bash
scripts/notifier-list prod
scripts/monitor-create prod ops/axiom/monitors/github-actions-failures.json
```

## Planned monitors

| File | Type | Intent |
|------|------|--------|
| `monitors/github-actions-failures.json` | MatchEvent | CI failure signal (dataset TBD) |
| `monitors/coherence-edge-errors.json` | Threshold | Edge 5xx / tool errors |
| `notifiers/oncall-email.example.json` | email | copy → fill address |

Do **not** commit real tokens or webhook secrets.
