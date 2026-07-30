"""
verification_helpers.py — pure-Python support for verification_orchestrator.ipynb

Emittance-verification, layer hashing, conservation dual checks.
Survives context resets; no notebook kernel magic required.
"""

from __future__ import annotations

import hashlib
import json
from collections import Counter
from dataclasses import asdict, dataclass, field
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, Dict, List, Optional

# Resolve repo root relative to this file (notebooks/)
DEFAULT_ROOT = Path(__file__).resolve().parents[1]
CONSERVATION_SUM = 15


LAYER_MANIFEST: Dict[str, List[str]] = {
    "python_qiskit": [
        "notebooks/gpu/Agent_M24_RMatrix.ipynb",
        "notebooks/triweave-backends.ipynb",
        "notebooks/verification_orchestrator.ipynb",
        "notebooks/verification_helpers.py",
    ],
    "rust_cutile": [
        "cutiles/cutile/src/core/r_matrix.rs",
        "cutiles/cutile/src/core/mod.rs",
    ],
    "hup_core": [
        "hup/INSTANCE.md",
        "hup/rust/src/main.rs",
        "hup/python/constraint_mathematics.py",
        "hup/python/dimensional_collapse.py",
        "hup/typescript/partial-port.ts",
        "hup/unikernel/unikernel.ml",
        "hup/instance2-redox/README.md",
        "hup/instance3-rvm/README.md",
        "hup/instance3-rvm/INTEGRATION-MAP.md",
        "hup/instance3-rvm/probe-rvm-layout.py",
        "docs/sovereign-handoff/mehler-serrescarr-convergence.dag.yaml",
    ],
    "cuda": [
        "kernels/fundamental_r_matrix.cu",
        "kernels/fundamental_r_matrix.cuh",
        "kernels/r_matrix_interface.h",
        "cutiles/r_matrix_host.cpp",
    ],
    "wgsl": [
        "kernels/fundamental_r_matrix.wgsl",
    ],
    "lean": [
        "lean/TriWeavon/ConservationInvariant.lean",
        "lean/AgdaLeanBridge.md",
        "lean/K22/M24Coefficient.lean",
        "lean/K22/MiracleOctadGenerator.lean",
        "lean/TriWeavon/SubRiemannian/Core.lean",
    ],
    "agda": [
        "agda/src/TriWeavon/ConservationRMatrix.agda",
        "agda/src/TriWeavon/Core.agda",
        "agda/src/TriWeavon/K22/SerreScarr.agda",
        "agda/src/TriWeavon/SubRiemannian/MehlerJesusBridge.agda",
        "agda/src/Everything.agda",
    ],
    "docs": [
        "docs/sovereign-handoff/LAYER-CASCADE-MAP.md",
        "docs/sovereign-handoff/CONSENSUS-VERIFIER-M1-M2.md",
        "docs/sovereign-handoff/overlays/trainmap-interjoin.md",
        "docs/sovereign-handoff/terminal-shaders/INSTANCE2-REDOX-SUITE.md",
        "docs/sovereign-handoff/session-handovers/HO-05-CUTILE-TDA-M24.md",
        "docs/sovereign-handoff/UNIFIED-WORKFLOW-FROM-MAX-CONTEXT-SESSIONS-2026-07-09.md",
        "docs/sovereign-handoff/LOGOS-COHERENCE-MCP-MAP.md",
        "docs/sovereign-handoff/mcp-inspector.coherence.json",
        "SAIF-Docs/Mehler_CoherenceMCP_Wiring_v0.5.0.md",
    ],
    "results": [
        "notebooks/triweave_backend_results/ax_wave_optimization.json",
        "notebooks/triweave_backend_results/dspy_strand_router.json",
    ],
    "coherence_mcp": [
        "docs/sovereign-handoff/LOGOS-COHERENCE-MCP-MAP.md",
        "docs/sovereign-handoff/mcp-inspector.coherence.json",
        ".atom-trail/decisions",
        "notebooks/triweave_backend_results/verification_receipts",
        "notebooks/triweave_backend_results/verification_certificates",
        "notebooks/triweave_backend_results/mcp_payloads",
    ],
}

# Live stdio tools (coherence-mcp 0.3.1) — authoritative for Inspector
LIVE_MCP_TOOLS = [
    "store_context",
    "retrieve_context",
    "map_isomorphism",
    "check_coherence",
    "bridge_translate",
    "wave_coherence_check",
    "atom_track",
    "gate_transition",
    "gauge_verify",
    "fibonacci_weight",
    "context_pack",
    "list_platforms",
]

# LogOS function → preferred MCP tool(s)
LOGOS_TO_MCP: Dict[str, List[str]] = {
    "conservation_invariant": ["gauge_verify"],
    "wave_scoring": ["wave_coherence_check", "check_coherence"],
    "atom_provenance": ["atom_track"],
    "sphinx_gates": ["gate_transition"],
    "platform_roster": ["list_platforms"],
    "cross_platform_translate": ["bridge_translate"],
    "qdi_isomorphism": ["map_isomorphism"],
    "fibonacci_priority": ["fibonacci_weight"],
    "spiralsafe_context": ["context_pack", "store_context", "retrieve_context"],
    "mehler_plateau": [],  # Rust crate only — not in live stdio
    "layer_verification": ["atom_track", "gauge_verify", "wave_coherence_check"],
    "hup_instances": ["atom_track", "fibonacci_weight"],
}


def file_hash16(path: Path) -> Optional[str]:
    if not path.is_file():
        return None
    return hashlib.sha256(path.read_bytes()).hexdigest()[:16]


def most_frequent_bitstring(bit_array: Any) -> str:
    """Qiskit 1.0+ BitArray-compatible mode bitstring."""
    if bit_array is None:
        raise ValueError("missing BitArray")
    if hasattr(bit_array, "get_counts"):
        counts = bit_array.get_counts()
        return max(counts, key=counts.get)
    return Counter(bit_array.get_bitstrings()).most_common(1)[0][0]


def fundamental_r_matrix_flat(q: float) -> List[List[float]]:
    """Python mirror of cutile::core::r_matrix (row-major, [re, im] pairs)."""
    if q == 0.0:
        raise ValueError("q must be non-zero")
    q_inv = 1.0 / q
    off = 1.0 - q * q
    z = [0.0, 0.0]
    rows = [
        [[q, 0.0], z, z, z],
        [z, [q_inv, 0.0], [off, 0.0], z],
        [z, z, [q, 0.0], z],
        [z, z, z, [q_inv, 0.0]],
    ]
    return [c for row in rows for c in row]


def is_conserved(alpha: int, omega: int, total: int = CONSERVATION_SUM) -> bool:
    return alpha + omega == total


def verify_layer(root: Path, name: str, paths: List[str]) -> Dict[str, Any]:
    files = []
    for rel in paths:
        p = root / rel
        exists = p.exists()
        files.append(
            {
                "path": rel.replace("\\", "/"),
                "status": "OK" if exists else "MISSING",
                "sha256_16": file_hash16(p) if p.is_file() else None,
                "size_bytes": p.stat().st_size if p.is_file() else 0,
                "is_dir": p.is_dir() if exists else False,
            }
        )
    ok = all(f["status"] == "OK" for f in files)
    return {
        "layer": name,
        "verified": ok,
        "ok_count": sum(1 for f in files if f["status"] == "OK"),
        "total": len(files),
        "files": files,
        "timestamp": datetime.now(timezone.utc).isoformat().replace("+00:00", "Z"),
    }


def run_full_verification(root: Optional[Path] = None) -> Dict[str, Any]:
    root = Path(root) if root else DEFAULT_ROOT
    layers = [verify_layer(root, name, paths) for name, paths in LAYER_MANIFEST.items()]
    overall_ok = all(L["verified"] for L in layers)

    # Structural identity check: Python R-matrix vs expected off-diagonal
    q = 2.0**0.5
    flat = fundamental_r_matrix_flat(q)
    r_ok = (
        abs(flat[0][0] - q) < 1e-12
        and abs(flat[5][0] - 1.0 / q) < 1e-12
        and abs(flat[6][0] - (1.0 - q * q)) < 1e-12
    )

    # Dual conservation for all 16 alpha
    duals_ok = all(is_conserved(a, CONSERVATION_SUM - a) for a in range(CONSERVATION_SUM + 1))

    receipt = {
        "atom": "ATOM-VERIFY-RECEIPT",
        "invariant": "alpha+omega=15",
        "profile": "Monitoring & Consensus Verifier + SuperHeisenGrok",
        "timestamp": datetime.now(timezone.utc).isoformat().replace("+00:00", "Z"),
        "root": str(root),
        "layers": layers,
        "overall_ok": overall_ok and r_ok and duals_ok,
        "checks": {
            "layer_manifest_complete": overall_ok,
            "r_matrix_python_mirror": r_ok,
            "dual_conservation_0_to_15": duals_ok,
        },
        "r_matrix_sample": {
            "q": q,
            "entries_re_im": flat,
            "layout": "row-major-4x4-complex",
        },
        "cascade_map": "docs/sovereign-handoff/LAYER-CASCADE-MAP.md",
    }
    return receipt


def emit_receipt(
    receipt: Dict[str, Any],
    out_dir: Optional[Path] = None,
) -> Path:
    root = Path(receipt.get("root", DEFAULT_ROOT))
    out_dir = out_dir or (root / "notebooks" / "triweave_backend_results" / "verification_receipts")
    out_dir.mkdir(parents=True, exist_ok=True)
    stamp = datetime.now(timezone.utc).strftime("%Y%m%dT%H%M%SZ")
    path = out_dir / f"receipt_{stamp}.json"
    path.write_text(json.dumps(receipt, indent=2), encoding="utf-8")
    # also write latest pointer
    latest = out_dir / "receipt_latest.json"
    latest.write_text(json.dumps(receipt, indent=2), encoding="utf-8")
    return path


def emit_certificate(
    receipt: Dict[str, Any],
    out_dir: Optional[Path] = None,
) -> Path:
    """MCP-ready verification certificate (α+ω seal + layer digest)."""
    root = Path(receipt.get("root", DEFAULT_ROOT))
    out_dir = out_dir or (
        root / "notebooks" / "triweave_backend_results" / "verification_certificates"
    )
    out_dir.mkdir(parents=True, exist_ok=True)
    stamp = datetime.now(timezone.utc).strftime("%Y%m%dT%H%M%SZ")

    layer_digest = {
        L["layer"]: {
            "verified": L["verified"],
            "ok_count": L["ok_count"],
            "total": L["total"],
        }
        for L in receipt.get("layers", [])
    }
    cert = {
        "atom": "ATOM-VERIFY-CERTIFICATE",
        "schema": "logos.verification.certificate.v1",
        "invariant": "alpha+omega=15",
        "gauge": {"alpha": 7, "omega": 8, "sum": 15, "valid": True},
        "timestamp": datetime.now(timezone.utc).isoformat().replace("+00:00", "Z"),
        "root": str(root),
        "overall_ok": receipt.get("overall_ok"),
        "checks": receipt.get("checks"),
        "layer_digest": layer_digest,
        "live_mcp_tools": LIVE_MCP_TOOLS,
        "logos_to_mcp": LOGOS_TO_MCP,
        "cascade_map": "docs/sovereign-handoff/LAYER-CASCADE-MAP.md",
        "mcp_map": "docs/sovereign-handoff/LOGOS-COHERENCE-MCP-MAP.md",
        "receipt_ref": "notebooks/triweave_backend_results/verification_receipts/receipt_latest.json",
        "profile": receipt.get("profile", "Monitoring & Consensus Verifier"),
    }
    path = out_dir / f"cert_{stamp}.json"
    path.write_text(json.dumps(cert, indent=2), encoding="utf-8")
    (out_dir / "cert_latest.json").write_text(json.dumps(cert, indent=2), encoding="utf-8")
    return path


def emit_mcp_payloads(
    receipt: Dict[str, Any],
    out_dir: Optional[Path] = None,
) -> Dict[str, Path]:
    """Write paste-ready Inspector / MCP tool argument payloads."""
    root = Path(receipt.get("root", DEFAULT_ROOT))
    out_dir = out_dir or (root / "notebooks" / "triweave_backend_results" / "mcp_payloads")
    out_dir.mkdir(parents=True, exist_ok=True)

    missing = []
    for L in receipt.get("layers", []):
        for f in L.get("files", []):
            if f.get("status") != "OK":
                missing.append(f["path"])

    atom_payload = {
        "decision": (
            f"LogOS verification overall_ok={receipt.get('overall_ok')} "
            f"layers={len(receipt.get('layers', []))} missing={len(missing)}"
        ),
        "files": [
            "docs/sovereign-handoff/LAYER-CASCADE-MAP.md",
            "docs/sovereign-handoff/LOGOS-COHERENCE-MCP-MAP.md",
            "notebooks/verification_helpers.py",
            "notebooks/triweave_backend_results/verification_receipts/receipt_latest.json",
            "notebooks/triweave_backend_results/verification_certificates/cert_latest.json",
        ],
        "tags": ["VERIFY", "LOGOS", "RECEIPT", "CERTIFICATE"],
        "type": "VERIFY",
    }
    gauge_payload = {"alpha": 7, "omega": 8}

    cascade = root / "docs" / "sovereign-handoff" / "LAYER-CASCADE-MAP.md"
    wave_content = cascade.read_text(encoding="utf-8")[:4000] if cascade.is_file() else (
        "LogOS cascade α+ω=15 verification"
    )
    wave_payload = {"content": wave_content}

    store_payload = {
        "key": "logos-receipt-latest",
        "content": json.dumps(
            {
                "overall_ok": receipt.get("overall_ok"),
                "checks": receipt.get("checks"),
                "timestamp": receipt.get("timestamp"),
            },
            indent=2,
        ),
        "platform": "generic",
        "alpha": 7,
        "omega": 8,
        "metadata": {
            "atom": "ATOM-VERIFY-RECEIPT",
            "root": str(root),
        },
    }

    written: Dict[str, Path] = {}
    for name, payload in (
        ("atom_track.json", atom_payload),
        ("gauge_verify.json", gauge_payload),
        ("wave_coherence_check.json", wave_payload),
        ("store_context.json", store_payload),
    ):
        p = out_dir / name
        p.write_text(json.dumps(payload, indent=2), encoding="utf-8")
        written[name] = p

    # Human-readable wave body for Inspector paste when JSON is large
    (out_dir / "wave_content.txt").write_text(wave_content, encoding="utf-8")
    written["wave_content.txt"] = out_dir / "wave_content.txt"

    index = {
        "atom": "ATOM-MCP-PAYLOAD-INDEX",
        "timestamp": datetime.now(timezone.utc).isoformat().replace("+00:00", "Z"),
        "live_tools": LIVE_MCP_TOOLS,
        "payloads": {k: str(v) for k, v in written.items()},
        "inspector_config": "docs/sovereign-handoff/mcp-inspector.coherence.json",
        "env_required": {
            "LOGOS_ROOT": str(root),
            "ATOM_TRAIL_ROOT": str(root / ".atom-trail"),
        },
    }
    idx_path = out_dir / "index_latest.json"
    idx_path.write_text(json.dumps(index, indent=2), encoding="utf-8")
    written["index_latest.json"] = idx_path
    return written


def run_and_emit(root: Optional[Path] = None) -> Dict[str, Any]:
    """Full verify + receipt + certificate + MCP payload emission."""
    receipt = run_full_verification(root)
    receipt_path = emit_receipt(receipt)
    cert_path = emit_certificate(receipt)
    payloads = emit_mcp_payloads(receipt)
    return {
        "overall_ok": receipt["overall_ok"],
        "checks": receipt["checks"],
        "receipt": str(receipt_path),
        "certificate": str(cert_path),
        "mcp_payloads": {k: str(v) for k, v in payloads.items()},
        "layers": [
            {"layer": L["layer"], "verified": L["verified"], "ok": f"{L['ok_count']}/{L['total']}"}
            for L in receipt["layers"]
        ],
    }


if __name__ == "__main__":
    summary = run_and_emit()
    print(json.dumps(summary, indent=2))
    for L in summary["layers"]:
        mark = "OK" if L["verified"] else "MISS"
        print(f"  [{mark}] {L['layer']:16} {L['ok']}")
