#!/usr/bin/env python3
"""Minimal MCP registry v0.1 HTTP server for GitHub Copilot company registry.

Bind 127.0.0.1 by default. Deploy behind HTTPS for org/enterprise URL.
ATOM: ATOM-MCP-REGISTRY-20260730-sm100
"""
from __future__ import annotations

import argparse
import json
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
from urllib.parse import unquote

CATALOG = Path(__file__).resolve().parent / "catalog" / "servers.json"


def load_servers() -> list[dict]:
    data = json.loads(CATALOG.read_text(encoding="utf-8"))
    return list(data.get("servers", []))


def cors(handler: BaseHTTPRequestHandler) -> None:
    handler.send_header("Access-Control-Allow-Origin", "*")
    handler.send_header("Access-Control-Allow-Methods", "GET, OPTIONS")
    handler.send_header("Access-Control-Allow-Headers", "Authorization, Content-Type")


class RegistryHandler(BaseHTTPRequestHandler):
    servers = load_servers()

    def log_message(self, fmt: str, *args) -> None:
        print(f"[registry] {self.address_string()} {fmt % args}")

    def do_OPTIONS(self) -> None:  # noqa: N802
        self.send_response(204)
        cors(self)
        self.end_headers()

    def _json(self, code: int, payload: object) -> None:
        body = json.dumps(payload, indent=2).encode("utf-8")
        self.send_response(code)
        cors(self)
        self.send_header("Content-Type", "application/json; charset=utf-8")
        self.send_header("Content-Length", str(len(body)))
        self.end_headers()
        self.wfile.write(body)

    def do_GET(self) -> None:  # noqa: N802
        path = unquote(self.path.split("?", 1)[0])

        if path in ("/", "/health"):
            self._json(
                200,
                {
                    "ok": True,
                    "service": "logos-mcp-registry",
                    "version": "0.1.0",
                    "servers": len(self.servers),
                    "wave_publish_gate": 85,
                    "wave_scale": "0-100",
                },
            )
            return

        if path == "/v0.1/servers":
            self._json(200, {"servers": self.servers})
            return

        # /v0.1/servers/{name}/versions/latest
        # /v0.1/servers/{name}/versions/{version}
        prefix = "/v0.1/servers/"
        if path.startswith(prefix):
            rest = path[len(prefix) :]
            parts = [p for p in rest.split("/") if p]
            if len(parts) >= 3 and parts[1] == "versions":
                name = parts[0]
                ver = parts[2]
                for s in self.servers:
                    if s.get("name") == name or s.get("name", "").endswith("/" + name):
                        if ver == "latest" or ver == s.get("version"):
                            self._json(200, s)
                            return
                self._json(404, {"error": "server or version not found", "name": name, "version": ver})
                return
            if len(parts) == 1:
                name = parts[0]
                for s in self.servers:
                    if s.get("name") == name or s.get("name", "").endswith("/" + name):
                        self._json(200, s)
                        return
                self._json(404, {"error": "server not found", "name": name})
                return

        self._json(404, {"error": "not found", "path": path})


def main() -> None:
    ap = argparse.ArgumentParser(description="LogOS MCP registry v0.1")
    ap.add_argument("--host", default="127.0.0.1")
    ap.add_argument("--port", type=int, default=8787)
    args = ap.parse_args()
    httpd = ThreadingHTTPServer((args.host, args.port), RegistryHandler)
    print(f"MCP registry listening on http://{args.host}:{args.port}")
    print("GitHub settings: use this base URL only (no /v0.1/servers suffix)")
    httpd.serve_forever()


if __name__ == "__main__":
    main()
