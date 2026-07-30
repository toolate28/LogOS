# logos-env.sh — LogOS toolchain for WSL2 (Ubuntu / Kali)
# Sourced from ~/.bashrc via Install-LogOSShell.ps1 -Wsl
# Invariant: alpha+omega=15

# Prevent double-init
if [ -n "${LOGOS_SHELL_INIT:-}" ]; then
  return 0 2>/dev/null || exit 0
fi
export LOGOS_SHELL_INIT=1

# Resolve LogOS root: env override → F: Beelink → C: fallback → walk from this file
_logos_guess_root() {
  if [ -n "${LOGOS_ROOT:-}" ] && [ -d "$LOGOS_ROOT" ]; then
    printf '%s' "$LOGOS_ROOT"
    return
  fi
  for cand in \
    "/mnt/f/Users/Matthew Ruhnau/LogOS" \
    "/mnt/c/Users/Matthew Ruhnau/LogOS" \
    "$HOME/LogOS" \
    "/mnt/g/Reson8-Labs/LogOS" \
    "/mnt/g/LogOS"
  do
    if [ -d "$cand" ]; then
      printf '%s' "$cand"
      return
    fi
  done
  # Relative to this script: ops/wsl -> repo root
  local here
  here="$(CDPATH= cd -- "$(dirname -- "${BASH_SOURCE[0]:-$0}")" && pwd)"
  if [ -d "$here/../.." ] && [ -f "$here/../../Cargo.toml" ]; then
    CDPATH= cd -- "$here/../.." && pwd
    return
  fi
  printf '%s' ""
}

export LOGOS_ROOT="$(_logos_guess_root)"
unset -f _logos_guess_root

if [ -z "$LOGOS_ROOT" ] || [ ! -d "$LOGOS_ROOT" ]; then
  echo "logos-env.sh: LOGOS_ROOT not found (mount F: or set LOGOS_ROOT)" >&2
  return 0 2>/dev/null || true
fi

export RESON8_LOGOS_ROOT="$LOGOS_ROOT"
export CUTILE_ROOT="${CUTILE_ROOT:-$LOGOS_ROOT/cutiles/cutile}"
export AGDA_ROOT="${AGDA_ROOT:-$LOGOS_ROOT/agda}"
export LEAN_ROOT="${LEAN_ROOT:-$LOGOS_ROOT/lean}"
export KERNELS_ROOT="${KERNELS_ROOT:-$LOGOS_ROOT/kernels}"
export CRATES_ROOT="${CRATES_ROOT:-$LOGOS_ROOT/crates}"
export CTWFI_INVARIANT="${CTWFI_INVARIANT:-alpha+omega=15}"
export CTWFI_STRAND="${CTWFI_STRAND:-reason}"
export ATOM_LOG="${ATOM_LOG:-$LOGOS_ROOT/ATOM/ATOM-trail.log}"

# Toolchain PATH (prepend, no duplicates)
_logos_path_prepend() {
  case ":$PATH:" in
    *":$1:"*) ;;
    *) PATH="$1${PATH:+:$PATH}" ;;
  esac
}

_logos_path_prepend "$HOME/.cargo/bin"
_logos_path_prepend "$HOME/.elan/bin"
_logos_path_prepend "$HOME/.ghcup/bin"
_logos_path_prepend "$HOME/.cabal/bin"
_logos_path_prepend "$HOME/.local/bin"
_logos_path_prepend "/usr/local/cuda/bin"
_logos_path_prepend "/usr/local/cuda/bin"  # idempotent

# Prefer LogOS Windows-side scripts only when pwsh available; native tools preferred
if [ -d "$LOGOS_ROOT/ops" ]; then
  _logos_path_prepend "$LOGOS_ROOT/ops"
fi

# Python: prefer repo venv2 (Linux) then .venv
if [ -x "$LOGOS_ROOT/venv2/bin/python" ]; then
  _logos_path_prepend "$LOGOS_ROOT/venv2/bin"
  export VIRTUAL_ENV="$LOGOS_ROOT/venv2"
elif [ -x "$LOGOS_ROOT/.venv/bin/python" ]; then
  _logos_path_prepend "$LOGOS_ROOT/.venv/bin"
  export VIRTUAL_ENV="$LOGOS_ROOT/.venv"
elif [ -x "$LOGOS_ROOT/venv-ctfwi/bin/python" ]; then
  _logos_path_prepend "$LOGOS_ROOT/venv-ctfwi/bin"
  export VIRTUAL_ENV="$LOGOS_ROOT/venv-ctfwi"
fi

# CUDA
if [ -d /usr/local/cuda ]; then
  export CUDA_HOME="${CUDA_HOME:-/usr/local/cuda}"
  export CUDA_PATH="${CUDA_PATH:-$CUDA_HOME}"
  _logos_path_prepend "$CUDA_HOME/bin"
  if [ -d "$CUDA_HOME/lib64" ]; then
    case ":${LD_LIBRARY_PATH:-}:" in
      *":$CUDA_HOME/lib64:"*) ;;
      *) export LD_LIBRARY_PATH="$CUDA_HOME/lib64${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}" ;;
    esac
  fi
fi

export PATH

# Convenience functions
logos() { cd "$LOGOS_ROOT${1:+/$1}" || return; }
logos_status() {
  echo "LOGOS_ROOT=$LOGOS_ROOT"
  for c in cargo rustc python3 agda lean lake elan nvcc node; do
    if command -v "$c" >/dev/null 2>&1; then
      printf '  [OK] %-10s %s\n' "$c" "$(command -v "$c")"
    else
      printf '  [--] %-10s\n' "$c"
    fi
  done
}
logos_cargo() { (cd "$LOGOS_ROOT" && cargo "$@"); }
logos_agda() {
  cd "$AGDA_ROOT" || return
  if command -v agda >/dev/null 2>&1; then
    agda -l TriWeavon.agda-lib src/Everything.agda "$@"
  else
    echo "agda missing — sudo apt install agda  (or cabal install agda)" >&2
    return 127
  fi
}
logos_lean() {
  cd "$LEAN_ROOT" || return
  if command -v lake >/dev/null 2>&1; then
    lake "${@:-build}"
  else
    echo "lake missing — install elan: https://github.com/leanprover/elan" >&2
    return 127
  fi
}
logos_kernels() {
  echo "kernels: $KERNELS_ROOT"
  ls -1 "$KERNELS_ROOT"/*.{cu,cuh,wgsl} 2>/dev/null || true
  if [ "${1:-}" = "build" ]; then
    if ! command -v nvcc >/dev/null 2>&1; then
      echo "nvcc missing — install CUDA toolkit in WSL" >&2
      return 127
    fi
    if [ -f "$CUTILE_ROOT/scripts/build_ptx.sh" ]; then
      bash "$CUTILE_ROOT/scripts/build_ptx.sh"
    elif [ -f "$CUTILE_ROOT/scripts/build_ptx.ps1" ] && command -v pwsh >/dev/null 2>&1; then
      pwsh -File "$CUTILE_ROOT/scripts/build_ptx.ps1"
    else
      echo "no build_ptx script found" >&2
      return 1
    fi
  fi
}

# Interactive banner (skip for non-interactive)
if [ -n "${PS1:-}" ] && [ "${LOGOS_QUIET:-0}" != "1" ]; then
  printf 'LogOS WSL | %s | α+ω=15\n' "$LOGOS_ROOT"
  printf '  cmds: logos | logos_status | logos_agda | logos_lean | logos_kernels | logos_cargo\n'
fi
