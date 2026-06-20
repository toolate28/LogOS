#!/usr/bin/env bash
# Compile blackwell_entropy_v2.cu → PTX for cutile CUDA backend
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT="${ROOT}/target/ptx"
mkdir -p "$OUT"
nvcc -ptx -arch=sm_100 \
  "${ROOT}/kernels/blackwell_entropy_v2.cu" \
  -o "${OUT}/blackwell_entropy_v2.ptx"
echo "Wrote ${OUT}/blackwell_entropy_v2.ptx"