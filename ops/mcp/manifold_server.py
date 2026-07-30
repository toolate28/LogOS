#!/usr/bin/env python3
"""LogOS manifold projection API + static host for /manifold dashboard.

Endpoints:
  GET  /health              — server alive
  GET  /api/state           — manifold 2D projection snapshot
  GET  /api/healthcheck     — multi-surface health
  POST /api/ifdown          — remediation steps (dry_run default true)
  POST /api/ifup            — TW confidence onboarding
  GET  /                    — redirect to manifold UI
  GET  /manifold/*          — static dashboard

ATOM: ATOM-MANIFOLD-2D-20260730-sm100
WAVE scale: 0–100 · publish gate 85
Bind: 127.0.0.1 by default (least privilege)
"""
from __future__ import annotations

import argparse
import json
import math
import os
import socket
import time
import urllib.error
import urllib.request
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
from typing import Any
from urllib.parse import urlparse

ROOT = Path(__file__).resolve().parents[2]
MANIFOLD_STATIC = ROOT / "coherence-mcp" / "coherence-site" / "public" / "manifold"
TOOLS_DIR = ROOT / "mcps" / "coherence-mcp" / "tools"
REGISTRY_DEFAULT = "http://127.0.0.1:8787"
FORGE_WS_HOST, FORGE_WS_PORT = "127.0.0.1", 8088

# Viviani peak (Category C label)
ALPHA, OMEGA = 7, 8
WAVE_PUBLISH = 85
STARTED = time.time()


def cors(h: SimpleHTTPRequestHandler) -> None:
    h.send_header("Access-Control-Allow-Origin", "*")
    h.send_header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
    h.send_header("Access-Control-Allow-Headers", "Content-Type")


def json_response(h: SimpleHTTPRequestHandler, code: int, payload: Any) -> None:
    body = json.dumps(payload, indent=2).encode("utf-8")
    h.send_response(code)
    cors(h)
    h.send_header("Content-Type", "application/json; charset=utf-8")
    h.send_header("Cache-Control", "no-store")
    h.send_header("Content-Length", str(len(body)))
    h.end_headers()
    h.wfile.write(body)


def tcp_open(host: str, port: int, timeout: float = 0.8) -> dict:
    t0 = time.perf_counter()
    try:
        with socket.create_connection((host, port), timeout=timeout):
            ms = (time.perf_counter() - t0) * 1000
            return {"ok": True, "latency_ms": round(ms, 2), "error": None}
    except OSError as e:
        ms = (time.perf_counter() - t0) * 1000
        return {"ok": False, "latency_ms": round(ms, 2), "error": str(e)}


def http_get(url: str, timeout: float = 2.0) -> dict:
    t0 = time.perf_counter()
    try:
        req = urllib.request.Request(url, headers={"Accept": "application/json"})
        with urllib.request.urlopen(req, timeout=timeout) as resp:
            raw = resp.read().decode("utf-8", errors="replace")
            ms = (time.perf_counter() - t0) * 1000
            try:
                data = json.loads(raw)
            except json.JSONDecodeError:
                data = {"raw": raw[:200]}
            return {
                "ok": 200 <= resp.status < 400,
                "status": resp.status,
                "latency_ms": round(ms, 2),
                "body": data,
                "error": None,
            }
    except Exception as e:  # noqa: BLE001 — surface as health error
        ms = (time.perf_counter() - t0) * 1000
        return {
            "ok": False,
            "status": None,
            "latency_ms": round(ms, 2),
            "body": None,
            "error": str(e),
        }


def count_tools() -> dict:
    n = 0
    names: list[str] = []
    if TOOLS_DIR.is_dir():
        for f in sorted(TOOLS_DIR.glob("*.json")):
            n += 1
            names.append(f.stem)
    return {
        "logos_tool_schemas": n,
        "published_0_4_2": 58,
        "out_of_band": max(0, n - 58),
        "names_sample": names[:12],
        "has_server_healthcheck": "server_healthcheck" in names,
        "has_ifdown": "ifdown_remediate" in names,
        "has_ifup": "ifup_confidence_onboard" in names,
        "has_manifold_state": "manifold_state" in names,
    }


def formal_residuals() -> dict:
    """Lightweight Category B counts (not CVEs)."""
    sorry = axiom = 0
    lean = ROOT / "lean" / "K22"
    if lean.is_dir():
        for f in lean.rglob("*.lean"):
            try:
                t = f.read_text(encoding="utf-8", errors="ignore")
            except OSError:
                continue
            sorry += t.count("sorry")
            axiom += t.count("axiom")
    return {
        "category": "B",
        "k22_sorry": sorry,
        "k22_axiom": axiom,
        "security_defect": False,
        "note": "Formal residuals are not CVEs (SECURITY.md)",
    }


def wave_from_health(surfaces: dict) -> dict:
    """Composite WAVE 0–100 from surface health + conservation label."""
    # Fibonacci weights 8:5:3 → structural/semantic/temporal
    w_s, w_m, w_t = 0.50, 0.3125, 0.1875
    oks = [1.0 if v.get("ok") else 0.0 for v in surfaces.values()]
    structural = sum(oks) / max(1, len(oks))
    conservation = 1.0 if (ALPHA + OMEGA) == 15 else 0.0
    # temporal: uptime factor (saturates after ~10 min)
    up = min(1.0, (time.time() - STARTED) / 600.0)
    score = 100.0 * (w_s * structural + w_m * conservation + w_t * (0.5 + 0.5 * up))
    score = round(score, 2)
    if score >= 99:
        band = "CRITICAL"
    elif score >= WAVE_PUBLISH:
        band = "PUBLISH"
    elif score >= 80:
        band = "EMERGENT"
    elif score >= 60:
        band = "BASELINE"
    else:
        band = "BELOW_BASELINE"
    return {
        "score_0_100": score,
        "normalised": round(score / 100.0, 4),
        "band": band,
        "publish_gate": WAVE_PUBLISH,
        "publish_ok": score >= WAVE_PUBLISH,
        "weights": {"structural": w_s, "semantic": w_m, "temporal": w_t},
        "note": "85 and 0.85 name the same gate; SAIF 0.98 superseded as default publish",
    }


def healthcheck(registry_base: str) -> dict:
    surfaces = {
        "manifold_api": {"ok": True, "latency_ms": 0.0, "error": None},
        "registry": http_get(registry_base.rstrip("/") + "/health"),
        "forge_ws_tcp": tcp_open(FORGE_WS_HOST, FORGE_WS_PORT),
    }
    # simplify registry for ok flag
    surfaces["registry"] = {
        "ok": bool(surfaces["registry"].get("ok")),
        "latency_ms": surfaces["registry"].get("latency_ms"),
        "error": surfaces["registry"].get("error"),
        "detail": surfaces["registry"].get("body"),
    }
    wave = wave_from_health(surfaces)
    return {
        "timestamp": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
        "surfaces": surfaces,
        "wave": wave,
        "conservation": {
            "alpha": ALPHA,
            "omega": OMEGA,
            "sum": ALPHA + OMEGA,
            "valid": ALPHA + OMEGA == 15,
            "category": "C",
        },
    }


def manifold_state(registry_base: str) -> dict:
    hc = healthcheck(registry_base)
    tools = count_tools()
    formal = formal_residuals()
    # 2D cut-and-project: map registers to plane coordinates
    # x = structural rail (α-normalised), y = semantic rail (ω-normalised)
    nodes = [
        {"id": "conservation", "x": ALPHA / 15, "y": OMEGA / 15, "layer": 0, "label": "α+ω=15"},
        {"id": "k22", "x": 0.35, "y": 0.55, "layer": 1, "label": "K22 formal", "sorry": formal["k22_sorry"]},
        {"id": "mcp", "x": 0.65, "y": 0.4, "layer": 2, "label": f"MCP tools {tools['logos_tool_schemas']}"},
        {"id": "tui", "x": 0.55, "y": 0.75, "layer": 3, "label": "reson8-tui"},
        {"id": "registry", "x": 0.8, "y": 0.25, "layer": 3, "label": "MCP registry",
         "ok": hc["surfaces"]["registry"]["ok"]},
        {"id": "forge", "x": 0.75, "y": 0.7, "layer": 4, "label": "forge :8088",
         "ok": hc["surfaces"]["forge_ws_tcp"]["ok"]},
        {"id": "manifold", "x": 0.5, "y": 0.5, "layer": 4, "label": "2D projection", "ok": True},
    ]
    # phase as angle on circle for animation
    phase = (time.time() - STARTED) * 0.15
    for n in nodes:
        n["px"] = n["x"] + 0.03 * math.cos(phase + n["layer"])
        n["py"] = n["y"] + 0.03 * math.sin(phase + n["layer"] * 1.3)

    return {
        "atom": "ATOM-MANIFOLD-2D-20260730-sm100",
        "projection": "cut-and-project · higher-D → 2D",
        "uptime_s": round(time.time() - STARTED, 1),
        "conservation": hc["conservation"],
        "wave": hc["wave"],
        "health": hc["surfaces"],
        "tools": tools,
        "formal": formal,
        "pipeline_stages": [
            {"id": 0, "name": "Formal generators", "status": "scaffold"},
            {"id": 1, "name": "Witness emission", "status": "partial"},
            {"id": 2, "name": "75D→50D collapse", "status": "isolated"},
            {"id": 3, "name": "MCP gate", "status": "schemas+local_api"},
            {"id": 4, "name": "2D monitor", "status": "this_dashboard"},
        ],
        "nodes": nodes,
        "edges": [
            ["conservation", "k22"],
            ["k22", "mcp"],
            ["mcp", "tui"],
            ["mcp", "registry"],
            ["tui", "forge"],
            ["manifold", "mcp"],
            ["manifold", "tui"],
        ],
        "coherence_mcp": {
            "published_version": "0.4.2",
            "published_tools": 58,
            "tool_names": "snake_case",
            "drift_doc": "docs/security/COHERENCE-MCP-0.4.2-DRIFT.md",
        },
    }


def ifdown_plan(surface: str, dry_run: bool = True) -> dict:
    steps_all = {
        "registry": [
            "python ops/mcp/registry/serve_registry.py --host 127.0.0.1 --port 8787",
            "curl -s http://127.0.0.1:8787/health",
        ],
        "manifold": [
            "python ops/mcp/manifold_server.py --host 127.0.0.1 --port 8790",
            "curl -s http://127.0.0.1:8790/api/healthcheck",
        ],
        "forge_ws": [
            "Start-LogOSBridge  # or apps/triweave WS on :8088",
            "Test-NetConnection 127.0.0.1 -Port 8088",
        ],
        "mcp": [
            "Prefer GitHub MCP readonly + company registry only",
            "python ops/ci/validate_mcp_schemas.py",
            "node ops/verify-coherence-tools.mjs  # if COHERENCE_MCP_ROOT set",
        ],
        "tui": [
            "cargo check -p reson8-tui",
            "cargo run -p reson8-tui  # bin reson8-forge",
        ],
    }
    if surface == "all":
        steps = []
        for s, st in steps_all.items():
            steps.append({"surface": s, "commands": st})
    else:
        steps = [{"surface": surface, "commands": steps_all.get(surface, ["unknown surface"])}]

    return {
        "action": "ifdown_remediate",
        "surface": surface,
        "dry_run": dry_run,
        "executed": False if dry_run else False,  # never auto-exec in this skeleton
        "steps": steps,
        "follow_up": [
            "gauge_verify alpha=7 omega=8",
            "server_healthcheck",
            "atom_track decision=ifdown_remediate",
        ],
        "note": "dry_run default true — capability ≠ authority; no silent promotion",
    }


def ifup_onboard(alpha: float = 7, omega: float = 8, operator: str = "operator") -> dict:
    residual = abs((alpha + omega) - 15)
    coherent = residual < 0.001
    # confidence from conservation + local health
    hc = healthcheck(os.environ.get("LOGOS_MCP_REGISTRY", REGISTRY_DEFAULT))
    wave = hc["wave"]
    return {
        "action": "ifup_confidence_onboard",
        "operator": operator,
        "conservation": {
            "alpha": alpha,
            "omega": omega,
            "sum": alpha + omega,
            "residual": residual,
            "coherent": coherent,
        },
        "wave": wave,
        "confidence_band": wave["band"],
        "first_three_calls": [
            {"tool": "gauge_verify", "args": {"alpha": alpha, "omega": omega}, "pass": "valid && sum==15"},
            {
                "tool": "wave_coherence_check",
                "args": {"content": "alpha + omega = 15", "threshold": 85},
                "pass": "score >= 85 on 0-100 scale",
            },
            {
                "tool": "invariant_check",
                "args": {"alpha": alpha, "omega": omega},
                "pass": "coherent true",
            },
        ],
        "onboarding_checklist": [
            "Org MCP Registry only + readonly GitHub MCP",
            "git config core.hooksPath ops/githooks",
            "python ops/ci/validate_mcp_schemas.py",
            "Open /manifold dashboard — confirm WAVE band",
            "Do not enable tools: [\"*\"]",
        ],
        "tomczak_preserved": True,
        "auto_promoted": False,
    }


class Handler(SimpleHTTPRequestHandler):
    registry_base = REGISTRY_DEFAULT

    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=str(MANIFOLD_STATIC), **kwargs)

    def log_message(self, fmt: str, *args) -> None:
        print(f"[manifold] {fmt % args}")

    def do_OPTIONS(self) -> None:  # noqa: N802
        self.send_response(204)
        cors(self)
        self.end_headers()

    def do_GET(self) -> None:  # noqa: N802
        parsed = urlparse(self.path)
        path = parsed.path

        if path in ("/health", "/api/ping"):
            json_response(
                self,
                200,
                {
                    "ok": True,
                    "service": "logos-manifold",
                    "uptime_s": round(time.time() - STARTED, 1),
                },
            )
            return

        if path == "/api/state" or path == "/api/manifold_state":
            json_response(self, 200, manifold_state(self.registry_base))
            return

        if path == "/api/healthcheck" or path == "/api/server_healthcheck":
            json_response(self, 200, healthcheck(self.registry_base))
            return

        if path in ("/", ""):
            self.send_response(302)
            self.send_header("Location", "/index.html")
            self.end_headers()
            return

        # static files from MANIFOLD_STATIC
        super().do_GET()

    def do_POST(self) -> None:  # noqa: N802
        parsed = urlparse(self.path)
        path = parsed.path
        length = int(self.headers.get("Content-Length", "0") or 0)
        raw = self.rfile.read(length) if length else b"{}"
        try:
            body = json.loads(raw.decode("utf-8") or "{}")
        except json.JSONDecodeError:
            json_response(self, 400, {"error": "invalid JSON body"})
            return

        if path in ("/api/ifdown", "/api/ifdown_remediate"):
            surface = body.get("surface", "all")
            dry_run = body.get("dry_run", True)
            json_response(self, 200, ifdown_plan(surface, dry_run=bool(dry_run)))
            return

        if path in ("/api/ifup", "/api/ifup_confidence_onboard"):
            json_response(
                self,
                200,
                ifup_onboard(
                    alpha=float(body.get("alpha", 7)),
                    omega=float(body.get("omega", 8)),
                    operator=str(body.get("operator", "operator")),
                ),
            )
            return

        json_response(self, 404, {"error": "not found", "path": path})


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--host", default="127.0.0.1")
    ap.add_argument("--port", type=int, default=8790)
    ap.add_argument("--registry", default=os.environ.get("LOGOS_MCP_REGISTRY", REGISTRY_DEFAULT))
    args = ap.parse_args()
    MANIFOLD_STATIC.mkdir(parents=True, exist_ok=True)
    Handler.registry_base = args.registry
    httpd = ThreadingHTTPServer((args.host, args.port), Handler)
    print(f"Manifold API+UI http://{args.host}:{args.port}/")
    print(f"  state       GET  /api/state")
    print(f"  healthcheck GET  /api/healthcheck")
    print(f"  ifdown      POST /api/ifdown")
    print(f"  ifup        POST /api/ifup")
    print(f"  registry probe → {args.registry}")
    httpd.serve_forever()


if __name__ == "__main__":
    main()
