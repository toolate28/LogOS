# Fleet Action SHA pins — toolate28

**ATOM:** `ATOM-FLEET-ACTION-PINS-20260804`  
**Tool:** `ops/ci/fleet_pin_actions.py`  
**capability ≠ authority**

## Treatment (same for every repo)

1. Rewrite every workflow `uses:` / `- uses:` to a **full 40-char commit SHA** with `# original-ref` comment.
2. Add `.github/workflows/ci-policy.yml` when missing (fail-closed pin assert).
3. Set repository Actions permission **`sha_pinning_required: true`**.

## Operator commands

```bash
# Dry-run (no writes)
python ops/ci/fleet_pin_actions.py --owner toolate28

# Apply pins + enable policy (non-forks)
python ops/ci/fleet_pin_actions.py --owner toolate28 --apply --enable-sha-pinning --add-ci-policy

# Include forks (diverges from upstream)
python ops/ci/fleet_pin_actions.py --owner toolate28 --include-forks --apply --enable-sha-pinning --add-ci-policy

# Single repo
python ops/ci/fleet_pin_actions.py --owner toolate28 --only LogOS --apply --enable-sha-pinning
```

Requires `gh` authenticated with `repo` scope (and permission to edit Actions settings).

## Aliases handled

| Floating / broken ref | Replacement |
|-----------------------|-------------|
| `zricethezav/gitleaks-action@latest` | pin `v2` SHA |
| `PowerShell/Setup-PowerShell@v2` (missing) | `milliewalky/setup-pwsh@v1` SHA |

## Notes

- Forks receive the same pin treatment when `--include-forks` is set; this **will diverge** from upstream and may complicate sync.
- Repos without workflows still can enable `sha_pinning_required` so future workflows must pin.
- LogOS also carries full CODEX / Agentic MLOps (`codex-mlops.yml`) — fleet pins are the minimum bar everywhere else.
- Dependabot `github-actions` updates work best when pins keep `# vX` comments.

## Related

- [`AGENTIC-MLOPS-CI.md`](./AGENTIC-MLOPS-CI.md)
- [`CLOUD-GEMINI-AGENT-PROMPT.md`](./CLOUD-GEMINI-AGENT-PROMPT.md)
- Local pin assert: `ops/ci/assert_action_pins.py`
