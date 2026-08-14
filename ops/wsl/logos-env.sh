# logos-env.sh — LogOS toolchain for WSL2 (Ubuntu / Kali)
# Sourced from ~/.bashrc via Install-LogOSShell.ps1 -Wsl
# Invariant: alpha+omega=15
# Portable: no hard-coded person names or home folders.

# Prevent double-init
if [ -n "${LOGOS_SHELL_INIT:-}" ]; then
  return 0 2>/dev/null || exit 0
fi
export LOGOS_SHELL_INIT=1

# Resolve LogOS root: LOGOS_ROOT → script parent → $HOME/LogOS → /mnt/*/Users/*/LogOS
_logos_guess_root() {
  if [ -n "${LOGOS_ROOT:-}" ] && [ -d "$LOGOS_ROOT" ] && [ -f "$LOGOS_ROOT/Cargo.toml" ]; then
    printf '%s' "$LOGOS_ROOT"
    return
  fi

  # Relative to this script: ops/wsl -> repo root
  local here
  here="$(CDPATH= cd -- "$(dirname -- "${BASH_SOURCE[0]:-$0}")" && pwd)"
  if [ -f "$here/../../Cargo.toml" ]; then
    CDPATH= cd -- "$here/../.." && pwd
    return
  fi

  if [ -d "$HOME/LogOS" ] && [ -f "$HOME/LogOS/Cargo.toml" ]; then
    printf '%s' "$HOME/LogOS"
    return
  fi

  # Windows mounts: /mnt/<drive>/Users/<any>/LogOS — prefer non-c when multiple
  local cand preferred=""
  # shellcheck disable=SC2044
  for cand in /mnt/*/Users/*/LogOS; do
    if [ -f "$cand/Cargo.toml" ]; then
      case "$cand" in
        /mnt/c/*|/mnt/C/*)
          if [ -z "$preferred" ]; then preferred="$cand"; fi
          ;;
        *)
          printf '%s' "$cand"
          return
          ;;
      esac
    fi
  done
  if [ -n "$preferred" ]; then
    printf '%s' "$preferred"
    return
  fi

  # Optional lab roots (no personal names)
  for cand in /mnt/g/Reson8-Labs/LogOS /mnt/g/LogOS; do
    if [ -f "$cand/Cargo.toml" ]; then
      printf '%s' "$cand"
      return
    fi
  done

  printf '%s' ""
}

export LOGOS_ROOT="$(_logos_guess_root)"
unset -f _logos_guess_root

if [ -z "$LOGOS_ROOT" ] || [ ! -d "$LOGOS_ROOT" ]; then
  echo "logos-env.sh: LOGOS_ROOT not found (set LOGOS_ROOT or clone to \$HOME/LogOS)" >&2
  return 0 2>/dev/null || true
fi

export RESON8_LOGOS_ROOT="$LOGOS_ROOT"
export CUTILE_ROOT="${CUTILE_ROOT:-$LOGOS_ROOT/cutiles/cutile}"
export AGDA_ROOT="${AGDA_ROOT:-$LOGOS_ROOT/agda}"
export LEAN_ROOT="${LEAN_ROOT:-$LOGOS_ROOT/lean}"
export KERNELS_ROOT="${KERNELS_ROOT:-$LOGOS_ROOT/kernels}"
export FORGE_WS_URL="${FORGE_WS_URL:-ws://127.0.0.1:8088}"
export CTWFI_INVARIANT="${CTWFI_INVARIANT:-alpha+omega=15}"

# PATH: local toolchains under $HOME only
case ":${PATH}:" in
  *":$HOME/.cargo/bin:"*) ;;
  *) export PATH="$HOME/.cargo/bin:$PATH" ;;
esac
case ":${PATH}:" in
  *":$HOME/.elan/bin:"*) ;;
  *) export PATH="$HOME/.elan/bin:$PATH" ;;
esac
case ":${PATH}:" in
  *":$LOGOS_ROOT/ops:"*) ;;
  *) export PATH="$LOGOS_ROOT/ops:$PATH" ;;
esac

# Convenience
logos() { cd "$LOGOS_ROOT" || return 1; }
