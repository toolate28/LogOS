#!/usr/bin/env python3
"""Chebyshev-Lobatto differentiation matrix + Levin weights for N=8.

Basis: Chebyshev first kind at Lobatto nodes (see mehler_mma_levin_batched.cu).
Outputs JSON for host upload via mehler_init_constants().
"""

import json
import math
import sys

N = 8


def lobatto_nodes(n: int) -> list[float]:
    return [math.cos(j * math.pi / (n - 1)) for j in range(n)]


def chebyshev_D_lobatto(n: int) -> list[list[float]]:
    """Spectral differentiation matrix (Trefethen-style approximation)."""
    x = lobatto_nodes(n)
    c = [2.0 if i in (0, n - 1) else 1.0 for i in range(n)]
    D = [[0.0] * n for _ in range(n)]
    for i in range(n):
        for j in range(n):
            if i == j:
                if i == 0:
                    D[i][j] = (2 * (n - 1) ** 2 + 1) / 6.0
                elif i == n - 1:
                    D[i][j] = -(2 * (n - 1) ** 2 + 1) / 6.0
                else:
                    D[i][j] = -x[i] / (2 * (1 - x[i] ** 2))
            else:
                D[i][j] = (c[i] / c[j]) * ((-1) ** (i + j)) / (x[i] - x[j])
    return D


def levin_weights(n: int) -> list[float]:
    """Simpson-like weights on Lobatto grid (normalized)."""
    w = [1.0] * n
    w[0] = w[-1] = 0.5
    s = sum(w)
    return [v / s for v in w]


def main() -> int:
    D = chebyshev_D_lobatto(N)
    W = levin_weights(N)
    out = {"N": N, "D_flat": [v for row in D for v in row], "weights": W}
    json.dump(out, sys.stdout, indent=2)
    print(file=sys.stderr)
    print(f"Chebyshev-Lobatto D ({N}x{N}) + weights generated.", file=sys.stderr)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())