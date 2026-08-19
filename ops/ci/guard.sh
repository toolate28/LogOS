#!/usr/bin/env bash
# Tree guards — secret-like paths + lake products must not be committed.
# Mirrors ops/githooks/pre-commit for CI.
# ATOM: ATOM-SEC-ADVISORY-20260730-sm100
set -euo pipefail

ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
cd "$ROOT"

FAIL=0
note() { echo "$*"; }
fail() { echo "::error::$*"; FAIL=1; }

note "== secret-path guard =="
# Staged or tracked paths that look like secrets
SECRET_RE='(^|/)\.env($|\.)|(^|/)id_rsa|(^|/)id_ed25519|\.pem$|\.p12$|\.pfx$|(^|/)credentials\.json$|(^|/)service.account.*\.json$|(^|/)secrets?/|auth\.token|api[_-]?key\.txt'

while IFS= read -r path; do
  [[ -z "$path" ]] && continue
  # allow examples and docs
  if [[ "$path" == *".example"* ]] || [[ "$path" == *".sample"* ]] || [[ "$path" == *"docs/"* ]]; then
    continue
  fi
  if echo "$path" | grep -Eiq "$SECRET_RE"; then
    fail "secret-like path tracked: $path"
  fi
done < <(git ls-files)

note "== gitlink without .gitmodules =="
# Ghost gitlinks (mode 160000) with no .gitmodules url break
# GitHub's legacy Pages recursive checkout:
#   fatal: No url found for submodule path 'crates/coherence-mcp'
while IFS= read -r mode _sha _stage path; do
  [[ -z "${path:-}" ]] && continue
  [[ "$mode" != "160000" ]] && continue
  if [[ ! -f .gitmodules ]] || ! grep -q "path = ${path}" .gitmodules; then
    fail "gitlink without .gitmodules url: $path (breaks Pages recursive checkout)"
  fi
done < <(git ls-files -s)

note "== lake / build artefact guard =="
LAKE_RE='\.(olean|ilean|trace)$|\.lake/build/|\.c\.hash$|\.olean\.hash$'
while IFS= read -r path; do
  [[ -z "$path" ]] && continue
  # lean/.lake is gitignored; if it appears in the index, fail
  if echo "$path" | grep -Eiq "$LAKE_RE"; then
    fail "lake/build artefact tracked: $path"
  fi
done < <(git ls-files)

note "== embedded secret pattern scan (sample of text files) =="
# Lightweight content scan on committed small configs only
CONTENT_RE='ghp_[A-Za-z0-9]{20,}|github_pat_[A-Za-z0-9_]{20,}|sk-[A-Za-z0-9]{20,}|xox[baprs]-[A-Za-z0-9-]{10,}|-----BEGIN (RSA |OPENSSH |EC )?PRIVATE KEY-----'
while IFS= read -r path; do
  [[ -z "$path" ]] && continue
  case "$path" in
    *.png|*.jpg|*.jpeg|*.gif|*.webp|*.pdf|*.zip|*.wasm|*.o|*.a|*.so|*.dll|*.exe|*.olean|*.ilean)
      continue ;;
  esac
  # skip large binary-ish
  if [[ -f "$path" ]] && [[ $(wc -c <"$path" 2>/dev/null || echo 0) -gt 500000 ]]; then
    continue
  fi
  if [[ -f "$path" ]] && grep -EIq "$CONTENT_RE" "$path" 2>/dev/null; then
    fail "possible embedded secret material in: $path"
  fi
done < <(git ls-files '*.yml' '*.yaml' '*.json' '*.toml' '*.env' '*.md' '*.ts' '*.js' '*.mjs' '*.rs' '*.py' 2>/dev/null || true)

if [[ "$FAIL" -ne 0 ]]; then
  echo "guard: FAILED"
  exit 1
fi
echo "guard: OK"
