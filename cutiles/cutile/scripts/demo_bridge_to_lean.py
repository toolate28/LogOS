#!/usr/bin/env python3
"""Validate cutile existence_certificate.json and print Lean bridge status."""

from __future__ import annotations

import json
import sys
from pathlib import Path

REQUIRED = {
    "bettiProxyBelowThreshold",
    "tomczakPreserved",
    "maxErrorBound",
    "reliable",
    "waveScore",
    "alphaOmegaSum",
    "coherenceDelta",
    "atomTrailId",
    "kernelVersion",
    "certificateHash",
    "timestampNs",
}


def main() -> int:
    cert_path = Path("existence_certificate.json")
    if not cert_path.exists():
        print(f"Missing {cert_path}. Run: cargo run --bin demo_existence_certificate_emission", file=sys.stderr)
        return 1

    data = json.loads(cert_path.read_text(encoding="utf-8"))
    missing = REQUIRED - set(data.keys())
    if missing:
        print(f"Certificate missing fields: {sorted(missing)}", file=sys.stderr)
        return 1

    alpha = float(data["alphaOmegaSum"])
    if abs(alpha - 15.0) >= 0.05:
        print(f"alphaOmegaSum out of basin tolerance: {alpha}", file=sys.stderr)
        return 1

    if not (data["bettiProxyBelowThreshold"] and data["tomczakPreserved"] and data["reliable"]):
        print("Certificate failed Tomczak/reliability gate", file=sys.stderr)
        return 1

    out = Path("existence_certificate.validated.json")
    out.write_text(json.dumps(data, indent=2), encoding="utf-8")

    print("Certificate validated for Lean bridge")
    print(f"  atomTrailId: {data['atomTrailId']}")
    print(f"  certificateHash: {data['certificateHash']}")
    print(f"  alphaOmegaSum: {alpha}")
    print(f"Wrote {out}")
    print("Lean: lake build K22  (see lean/K22/Existence.lean example)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())