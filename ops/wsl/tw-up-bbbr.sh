#!/usr/bin/env bash
# tw up bbbr — hermetic bbbr-verifier on :8081
# Prefer python unix SC (always in tree). Nix flake binary is optional fallback.
# Portable: no hard-coded person names — LOGOS_ROOT · $HOME/LogOS · /mnt/*/Users/*/LogOS
# ATOM: ATOM-TW-UP-BBBR-20260810
set -euo pipefail

_resolve_root() {
  if [[ -n "${LOGOS_HOME:-}" && -f "${LOGOS_HOME}/hup/unikernel/bbbr-verifier/bbbr_unix.py" ]]; then
    printf '%s' "$LOGOS_HOME"; return
  fi
  if [[ -n "${LOGOS_ROOT:-}" && -f "${LOGOS_ROOT}/hup/unikernel/bbbr-verifier/bbbr_unix.py" ]]; then
    printf '%s' "$LOGOS_ROOT"; return
  fi
  local here
  here="$(CDPATH= cd -- "$(dirname -- "${BASH_SOURCE[0]:-$0}")/../.." && pwd)"
  if [[ -f "$here/hup/unikernel/bbbr-verifier/bbbr_unix.py" ]]; then
    printf '%s' "$here"; return
  fi
  if [[ -f "$HOME/LogOS/hup/unikernel/bbbr-verifier/bbbr_unix.py" ]]; then
    printf '%s' "$HOME/LogOS"; return
  fi
  local d
  for d in /mnt/*/Users/*/LogOS; do
    if [[ -f "$d/hup/unikernel/bbbr-verifier/bbbr_unix.py" ]]; then
      printf '%s' "$d"; return
    fi
  done
  return 1
}

ROOT="$(_resolve_root)" || {
  echo "cannot resolve LogOS root (set LOGOS_ROOT or clone to \$HOME/LogOS)"
  exit 1
}
cd "$ROOT"
export LOGOS_ROOT="$ROOT"
export LOGOS_HOME="$ROOT"

PIDF=/tmp/bbbr.pid
LOGF=/tmp/bbbr.log
PORT="${BBBR_PORT:-8081}"

if [[ -f "$PIDF" ]]; then
  old="$(cat "$PIDF" 2>/dev/null || true)"
  if [[ -n "${old}" ]] && kill -0 "$old" 2>/dev/null; then
    if curl -sf "http://127.0.0.1:${PORT}/health" >/dev/null 2>&1; then
      echo "already_up pid=$old"
      exit 0
    fi
    kill "$old" 2>/dev/null || true
    sleep 0.2
  fi
fi

start_py() {
  local py=hup/unikernel/bbbr-verifier/bbbr_unix.py
  if [[ ! -f "$py" ]]; then
    echo "MISSING $py — restore hup/unikernel/bbbr-verifier/"
    return 1
  fi
  export BBBR_PORT="$PORT"
  nohup python3 "$py" >"$LOGF" 2>&1 &
  echo $! >"$PIDF"
  return 0
}

start_nix() {
  if [[ -x result-bbbr/bin/bbbr-verifier ]]; then
    nohup ./result-bbbr/bin/bbbr-verifier >"$LOGF" 2>&1 &
    echo $! >"$PIDF"
    return 0
  fi
  if [[ -f flake.nix ]] && command -v nix >/dev/null 2>&1; then
    . /nix/var/nix/profiles/default/etc/profile.d/nix-daemon.sh 2>/dev/null || true
    if nix build .#bbbr-verifier --out-link result-bbbr 2>"$LOGF"; then
      nohup ./result-bbbr/bin/bbbr-verifier >"$LOGF" 2>&1 &
      echo $! >"$PIDF"
      return 0
    fi
  fi
  return 1
}

if start_py || start_nix; then
  sleep 0.6
  if curl -sf "http://127.0.0.1:${PORT}/health"; then
    echo
    echo "OK bbbr :${PORT} pid=$(cat "$PIDF") root=$ROOT"
    exit 0
  fi
  echo "FAIL health :${PORT}"
  tail -20 "$LOGF" 2>/dev/null || true
  exit 1
fi

echo "FAIL: no bbbr backend (need python3 + hup/unikernel/bbbr-verifier/bbbr_unix.py)"
echo "  note: nix flake .#bbbr-verifier is optional and often absent from slim trees"
exit 1
