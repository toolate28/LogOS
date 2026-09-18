# QUARANTINE — Identity/Attribution Language

Status: ACTIVE QUARANTINE  
Date: 2026-09-18

Purpose: quarantine repository documents that address the user in third person
or with unverified identity terms (including "Weaver"). These documents remain
in history for traceability but are non-authoritative for current operations.
This register also carries markdown provenance quarantine results.

## Enforcement Rule

- Do not use quarantined documents as identity authority.
- Do not reuse third-person identity labels from quarantined documents in new
  handoffs, specs, prompts, or signatures without explicit user confirmation.

## Quarantined Documents

- `docs/ops/SIGNATURES.md`
- `docs/architecture/BRAND-UNITARITY.md`
- `docs/architecture/COHERENCE-GATE-REFERENCE-ORACLE-SPEC.md`
- `docs/architecture/LIMBO-BRIDGE-PATTERN.md`
- `docs/architecture/SAIF-PREFLIGHT-SPEC.md`
- `docs/coherence/COHERENCE-TOOLATED-SUBDOMAIN-MAP.md`
- `docs/coherence/STITCH-COHERENCE-MCP-BINDING.md`
- `docs/coherence/STITCH-REAL-WIRING-PLAN.md`
- `docs/ops/CHECKPOINT-20260419.md`
- `docs/publications/PREPRINT-TRI-WEAVON-AINULINDALE-20260418.html`
- `docs/publications/PREPRINT-TRI-WEAVON-ITHILDIN-EDITION-20260418.html`
- `docs/theory/AINULINDALE-OF-THE-TRI-WEAVON.md`
- `docs/theory/DOUBLE-FIB-CRYPTO-BRIDGE.md`
- `docs/theory/GEMINI-RESEARCH-TASK-SCALE-STRAND-20260418.md`

## Markdown Provenance Quarantine (creation-commit rule)

Rule: quarantine any `.md` file whose creation commit did **not** include at
least one `.lean`, `.rs`, or `.py` file change.

Control-file exemption: this register file is excluded from the provenance
quarantine rule so quarantine reporting remains authoritative.

Evaluation scope: all other tracked markdown files in the repository.  
Result snapshot (2026-09-18): `223` evaluated `.md`, `223` pass, `0` quarantined.
Snapshot maintenance rule: re-run the reproducibility command and update this
snapshot whenever markdown files are added, removed, or renamed.

Reproducibility command (run from repository root):

```bash
python - <<'PY'
import subprocess
files=subprocess.check_output(['git','ls-files','*.md'], text=True).splitlines()
exempt={'docs/ops/QUARANTINE-IDENTITY-LANGUAGE-20260918.md'}
evald=[f for f in files if f not in exempt]
q=[]
for f in evald:
    commits=subprocess.check_output(['git','log','--diff-filter=A','--reverse','--format=%H','--',f], text=True).splitlines()
    if not commits:
        q.append(f); continue
    c=commits[0]
    changed=subprocess.check_output(['git','show','--name-only','--pretty=format:','--no-renames',c], text=True).splitlines()
    if not any(x.endswith(('.lean','.rs','.py')) for x in changed):
        q.append(f)
print('evaluated_md', len(evald))
print('quarantined_md', len(q))
for p in q: print(p)
PY
```

### Quarantined by creation provenance

- none (with control-file exemption applied)

## ATOM

`ATOM-QUARANTINE-IDENTITY-LANGUAGE-20260918`
