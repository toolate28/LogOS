#!/usr/bin/env bash
# Apply an entangle slice zip onto the current worktree.
# ATOM: ATOM-ENTANGLE-MANIFEST-20260809
set -euo pipefail

ZIP="${1:-}"
if [[ -z "$ZIP" || ! -f "$ZIP" ]]; then
  echo "usage: $0 <slice.zip>" >&2
  exit 2
fi

ROOT="$(git rev-parse --show-toplevel)"
cd "$ROOT"

TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

unzip -q "$ZIP" -d "$TMP"

# Never allow target/ or VCS into the tree from a slice
if find "$TMP" -type d \( -name target -o -name .git -o -name node_modules \) | grep -q .; then
  echo "apply-slice: refusing slice that contains target/.git/node_modules" >&2
  exit 1
fi

# Copy preserving structure
if command -v rsync >/dev/null 2>&1; then
  rsync -a --exclude 'target/' --exclude '.git/' "$TMP"/ "$ROOT"/
else
  (cd "$TMP" && tar cf - .) | (cd "$ROOT" && tar xf -)
fi

echo "apply-slice: applied $ZIP"
git status -sb | head -n 40
