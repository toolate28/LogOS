#!/usr/bin/env python3
"""LogOS CODEX scan — topology-aware CI gates + SARIF + Agentic MLOps badges.

ATOM: ATOM-CODEX-MLOPS-20260804-sm100

Epistemic posture
-----------------
- **Security gates** (fail closed): unpinned Actions, secret-like paths/content,
  MCP fail-closed policy violations.
- **Advisory layers** (Category B): formal residuals, CI surface inventory,
  agentic surface completeness.
- **Category C telemetry only**: α / ω heuristic balance tag — never a reject
  gate and never presented as a theorem.

capability ≠ authority. No silent promotion of scan scores into proofs.
"""
from __future__ import annotations

import argparse
import ast
import datetime as dt
import json
import os
import re
import sys
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any, Iterable

ROOT = Path(__file__).resolve().parents[2]
SCAN_VERSION = "2.1.0"
ATOM_ID = "ATOM-CODEX-MLOPS-20260804-sm100"

# Full-length commit SHA (40 hex) required when repo sha_pinning_required=true
SHA_PIN_RE = re.compile(
    r"^\s*uses:\s*(?P<action>[^\s@#]+?)@(?P<ref>[^\s#]+)"
)
FULL_SHA_RE = re.compile(r"^[0-9a-fA-F]{40}$")

SECRET_PATH_RE = re.compile(
    r"(^|/)\.env($|\.)|(^|/)id_rsa|(^|/)id_ed25519|\.pem$|\.p12$|\.pfx$"
    r"|(^|/)credentials\.json$|(^|/)service.?account.*\.json$"
    r"|(^|/)secrets?/|auth\.token|api[_-]?key\.txt",
    re.I,
)
SECRET_CONTENT_RE = re.compile(
    r"ghp_[A-Za-z0-9]{20,}|github_pat_[A-Za-z0-9_]{20,}|sk-[A-Za-z0-9]{20,}"
    r"|xox[baprs]-[A-Za-z0-9-]{10,}|-----BEGIN (RSA |OPENSSH |EC )?PRIVATE KEY-----"
)
MCP_WILDCARD_RE = re.compile(r'"tools"\s*:\s*\[\s*"\*"\s*\]')

SKIP_DIRS = {
    ".git",
    "node_modules",
    "target",
    ".venv",
    "venv",
    "venv2",
    "venv-ctfwi",
    ".lake",
    "vendor",
    "__pycache__",
    ".claude",
    "MAlonzo",
}

REQUIRED_WORKFLOWS = {
    "ci-policy.yml",
    "verify.yml",
    "codeql.yml",
    "security-advisory.yml",
    "mcp-validation.yml",
    "labels.yml",
    "codex-mlops.yml",
}

AGENTIC_MARKERS = [
    "AGENTS.md",
    "Claude.md",
    "CLAUDE.md",
    "ops/ci/guard.sh",
    "ops/ci/validate_mcp_config.py",
    "mcps",
    "skills",
]


@dataclass
class Finding:
    rule_id: str
    level: str  # error | warning | note
    message: str
    path: str = ""
    start_line: int = 1
    category: str = "security"  # security | advisory | telemetry
    help_uri: str = ""


@dataclass
class LayerResult:
    id: str
    name: str
    score: int  # 0-100
    weight: float
    findings: list[Finding] = field(default_factory=list)
    metrics: dict[str, Any] = field(default_factory=dict)
    epistemic: str = "B"  # A | B | C


def _iter_text_files(base: Path, suffixes: Iterable[str]) -> Iterable[Path]:
    suf = {s.lower() for s in suffixes}
    for p in base.rglob("*"):
        if not p.is_file():
            continue
        if any(part in SKIP_DIRS for part in p.parts):
            continue
        if p.suffix.lower() in suf or p.name in suf:
            yield p


def _read(path: Path, limit: int = 500_000) -> str:
    try:
        data = path.read_bytes()
        if len(data) > limit:
            data = data[:limit]
        return data.decode("utf-8", errors="ignore")
    except OSError:
        return ""


def layer_action_pins(root: Path) -> LayerResult:
    """Repo requires full-length commit SHA pins on every uses:."""
    findings: list[Finding] = []
    total = 0
    pinned = 0
    wf_dir = root / ".github" / "workflows"
    for path in sorted(wf_dir.glob("*.yml")) + sorted(wf_dir.glob("*.yaml")):
        text = _read(path)
        for i, line in enumerate(text.splitlines(), start=1):
            m = SHA_PIN_RE.search(line)
            if not m:
                continue
            total += 1
            ref = m.group("ref").strip()
            action = m.group("action").strip()
            if FULL_SHA_RE.match(ref):
                pinned += 1
            else:
                findings.append(
                    Finding(
                        rule_id="actions/unpinned-ref",
                        level="error",
                        message=(
                            f"Action {action}@{ref} is not a full 40-char commit SHA "
                            f"(sha_pinning_required)."
                        ),
                        path=str(path.relative_to(root)).replace("\\", "/"),
                        start_line=i,
                        category="security",
                        help_uri="https://docs.github.com/en/actions/security-guides/security-hardening-for-github-actions#using-third-party-actions",
                    )
                )
    score = 100 if total == 0 else int(round(100.0 * pinned / total))
    return LayerResult(
        id="L0_action_pin",
        name="Action SHA pinning",
        score=score,
        weight=0.22,
        findings=findings,
        metrics={"uses_total": total, "uses_pinned": pinned},
        epistemic="A",  # policy is machine-checkable against repo setting
    )


def layer_secrets(root: Path) -> LayerResult:
    findings: list[Finding] = []
    # Prefer git ls-files when available for tracked-only realism
    try:
        import subprocess

        out = subprocess.check_output(
            ["git", "ls-files"], cwd=root, text=True, stderr=subprocess.DEVNULL
        )
        paths = [root / p for p in out.splitlines() if p.strip()]
    except Exception:
        paths = list(root.rglob("*"))

    scanned = 0
    for path in paths:
        if not path.is_file():
            continue
        rel = str(path.relative_to(root)).replace("\\", "/")
        if any(part in SKIP_DIRS for part in path.parts):
            continue
        if SECRET_PATH_RE.search(rel):
            if ".example" in rel or ".sample" in rel or rel.startswith("docs/"):
                continue
            findings.append(
                Finding(
                    rule_id="secrets/path",
                    level="error",
                    message=f"Secret-like path tracked: {rel}",
                    path=rel,
                    category="security",
                )
            )
        if path.suffix.lower() in {
            ".yml",
            ".yaml",
            ".json",
            ".toml",
            ".env",
            ".md",
            ".ts",
            ".js",
            ".mjs",
            ".rs",
            ".py",
        }:
            scanned += 1
            text = _read(path)
            if SECRET_CONTENT_RE.search(text):
                findings.append(
                    Finding(
                        rule_id="secrets/content",
                        level="error",
                        message=f"Possible embedded secret material in {rel}",
                        path=rel,
                        category="security",
                    )
                )
    score = 100 if not findings else max(0, 100 - 25 * len(findings))
    return LayerResult(
        id="L1_secrets",
        name="Secret path & content guard",
        score=score,
        weight=0.22,
        findings=findings,
        metrics={"files_content_scanned": scanned, "findings": len(findings)},
        epistemic="B",
    )


def layer_mcp_policy(root: Path) -> LayerResult:
    findings: list[Finding] = []
    candidates: list[Path] = []
    for name in ("mcp.json", "mcp-config.json"):
        candidates.extend(root.rglob(name))
    candidates.extend((root / ".github" / "copilot").rglob("*.json"))
    candidates.extend((root / "ops" / "mcp").rglob("*.json"))
    # Prefer the existing fail-closed validator when present
    validator = root / "ops" / "ci" / "validate_mcp_config.py"
    if validator.is_file():
        import subprocess

        proc = subprocess.run(
            [sys.executable, str(validator)],
            cwd=root,
            capture_output=True,
            text=True,
        )
        if proc.returncode != 0:
            msg = (proc.stdout or "") + (proc.stderr or "")
            findings.append(
                Finding(
                    rule_id="mcp/policy",
                    level="error",
                    message=f"MCP validation failed:\n{msg.strip()[:1500]}",
                    path="ops/ci/validate_mcp_config.py",
                    category="security",
                )
            )
            return LayerResult(
                id="L2_mcp",
                name="MCP fail-closed policy",
                score=0,
                weight=0.16,
                findings=findings,
                metrics={"validator": "ops/ci/validate_mcp_config.py", "rc": proc.returncode},
                epistemic="A",
            )
    else:
        for path in candidates:
            if any(part in SKIP_DIRS for part in path.parts):
                continue
            text = _read(path)
            rel = str(path.relative_to(root)).replace("\\", "/")
            if MCP_WILDCARD_RE.search(text):
                findings.append(
                    Finding(
                        rule_id="mcp/wildcard-tools",
                        level="error",
                        message="Wildcard MCP tools [\"*\"] rejected (fail-closed).",
                        path=rel,
                        category="security",
                    )
                )
            if SECRET_CONTENT_RE.search(text):
                findings.append(
                    Finding(
                        rule_id="mcp/embedded-secret",
                        level="error",
                        message="Embedded secret material in MCP config.",
                        path=rel,
                        category="security",
                    )
                )
    score = 100 if not findings else max(0, 100 - 30 * len(findings))
    return LayerResult(
        id="L2_mcp",
        name="MCP fail-closed policy",
        score=score,
        weight=0.16,
        findings=findings,
        metrics={"configs_considered": len(candidates)},
        epistemic="A",
    )


def layer_formal_residuals(root: Path) -> LayerResult:
    findings: list[Finding] = []
    sorry_re = re.compile(r"\bsorry\b")
    axiom_re = re.compile(r"\baxiom\b")
    hole_re = re.compile(r"\{!!\}|\bpostulate\b")
    stats = {"lean_files": 0, "agda_files": 0, "sorry": 0, "axiom": 0, "holes": 0}

    def scan(subdir: str, patterns: list[str], key: str) -> None:
        base = root / subdir
        if not base.is_dir():
            return
        for pat in patterns:
            for f in base.rglob(pat):
                if any(p in f.parts for p in SKIP_DIRS):
                    continue
                text = _read(f)
                stats[key] += 1
                s = len(sorry_re.findall(text))
                a = len(axiom_re.findall(text))
                h = len(hole_re.findall(text))
                stats["sorry"] += s
                stats["axiom"] += a
                stats["holes"] += h
                if s or a or h:
                    rel = str(f.relative_to(root)).replace("\\", "/")
                    findings.append(
                        Finding(
                            rule_id="formal/residual",
                            level="note",
                            message=f"Formal residual: sorry={s} axiom={a} hole/postulate={h}",
                            path=rel,
                            category="advisory",
                        )
                    )

    scan("lean", ["*.lean"], "lean_files")
    scan("agda", ["*.agda", "*.lagda", "*.lagda.md"], "agda_files")
    # Residuals are honesty markers — score stays high; volume only soft-dips
    residual_units = stats["sorry"] + stats["axiom"] + stats["holes"]
    score = max(70, 100 - min(30, residual_units // 10))
    return LayerResult(
        id="L3_formal",
        name="Formal residual inventory",
        score=score,
        weight=0.10,
        findings=findings[:80],
        metrics=stats,
        epistemic="B",
    )


def layer_ci_surface(root: Path) -> LayerResult:
    findings: list[Finding] = []
    wf = root / ".github" / "workflows"
    present = {p.name for p in wf.glob("*.yml")} | {p.name for p in wf.glob("*.yaml")}
    missing = sorted(REQUIRED_WORKFLOWS - present)
    for name in missing:
        findings.append(
            Finding(
                rule_id="ci/missing-workflow",
                level="warning",
                message=f"Expected workflow missing: .github/workflows/{name}",
                path=".github/workflows",
                category="advisory",
            )
        )
    score = int(round(100.0 * (len(REQUIRED_WORKFLOWS) - len(missing)) / len(REQUIRED_WORKFLOWS)))
    return LayerResult(
        id="L4_ci_surface",
        name="CI/CD surface completeness",
        score=score,
        weight=0.12,
        findings=findings,
        metrics={"required": sorted(REQUIRED_WORKFLOWS), "present": sorted(present)},
        epistemic="B",
    )


def layer_agentic_surface(root: Path) -> LayerResult:
    findings: list[Finding] = []
    hit = 0
    for marker in AGENTIC_MARKERS:
        p = root / marker
        if p.exists():
            hit += 1
        else:
            findings.append(
                Finding(
                    rule_id="agentic/missing-marker",
                    level="note",
                    message=f"Agentic MLOps marker missing: {marker}",
                    path=marker,
                    category="advisory",
                )
            )
    score = int(round(100.0 * hit / len(AGENTIC_MARKERS)))
    return LayerResult(
        id="L5_agentic",
        name="Agentic MLOps surface",
        score=score,
        weight=0.12,
        findings=findings,
        metrics={"markers_hit": hit, "markers_total": len(AGENTIC_MARKERS)},
        epistemic="B",
    )


def layer_topology_telemetry(root: Path) -> LayerResult:
    """Category C α/ω heuristic — telemetry only, never a reject gate."""
    alphas: list[float] = []
    omegas: list[float] = []
    samples = 0
    for path in _iter_text_files(root, {".py", ".ts", ".js", ".rs"}):
        text = _read(path, limit=120_000)
        if not text.strip():
            continue
        lines = text.splitlines()
        n = len(lines)
        if path.suffix == ".py":
            try:
                tree = ast.parse(text)
                classes = sum(isinstance(n, ast.ClassDef) for n in ast.walk(tree))
                funcs = sum(isinstance(n, ast.FunctionDef) for n in ast.walk(tree))
                alpha = min(7.5, classes * 1.5 + funcs * 0.5 + min(n / 100, 2.0))
            except SyntaxError:
                alpha = 1.0
        elif path.suffix == ".rs":
            fns = text.count("fn ")
            structs = text.count("struct ") + text.count("enum ") + text.count("impl ")
            alpha = min(7.5, structs * 1.2 + fns * 0.35 + min(n / 120, 2.0))
        else:
            interfaces = text.count("interface ") + text.count("type ")
            functions = text.count("function ") + text.count("=>")
            alpha = min(7.5, interfaces * 2.0 + functions * 0.2 + min(n / 100, 2.0))
        markers = text.count("#") + text.count("//") + text.count("/*")
        async_calls = text.count("await ") + text.count("async ")
        api = text.count("http") + text.count("ws://")
        omega = min(7.5, markers * 0.05 + async_calls * 0.4 + api * 0.3 + min(n / 150, 1.5))
        if alpha + omega <= 0:
            continue
        scale = 15.0 / (alpha + omega)
        alphas.append(alpha * scale)
        omegas.append(omega * scale)
        samples += 1
        if samples >= 400:
            break
    if samples == 0:
        avg_a = avg_w = 0.0
    else:
        avg_a = round(sum(alphas) / samples, 2)
        avg_w = round(sum(omegas) / samples, 2)
    inv = round(avg_a + avg_w, 2)
    # Telemetry score: closeness of convention tag to 15 (not a security claim)
    drift = abs(inv - 15.0) if samples else 15.0
    score = max(0, int(round(100 - drift * 20)))
    finding = Finding(
        rule_id="telemetry/alpha-omega",
        level="note",
        message=(
            f"Category C convention tag α={avg_a} ω={avg_w} sum={inv} "
            f"(samples={samples}). Not a constitutional reject gate."
        ),
        path=".",
        category="telemetry",
    )
    return LayerResult(
        id="L6_topology_telemetry",
        name="Topology telemetry (Category C)",
        score=score,
        weight=0.06,
        findings=[finding],
        metrics={"alpha": avg_a, "omega": avg_w, "sum": inv, "samples": samples},
        epistemic="C",
    )


def composite_score(layers: list[LayerResult]) -> int:
    wsum = sum(l.weight for l in layers) or 1.0
    raw = sum(l.score * l.weight for l in layers) / wsum
    return int(round(raw))


def security_errors(layers: list[LayerResult]) -> list[Finding]:
    out: list[Finding] = []
    for layer in layers:
        for f in layer.findings:
            if f.category == "security" and f.level == "error":
                out.append(f)
    return out


def to_sarif(layers: list[LayerResult], root: Path) -> dict[str, Any]:
    rules: dict[str, dict[str, Any]] = {}
    results: list[dict[str, Any]] = []
    for layer in layers:
        for f in layer.findings:
            if f.category == "telemetry":
                continue
            if f.rule_id not in rules:
                rules[f.rule_id] = {
                    "id": f.rule_id,
                    "name": f.rule_id,
                    "shortDescription": {"text": f.rule_id},
                    "fullDescription": {"text": f.message[:200]},
                    "defaultConfiguration": {
                        "level": "error" if f.level == "error" else "warning" if f.level == "warning" else "note"
                    },
                    "helpUri": f.help_uri or "https://github.com/toolate28/LogOS/blob/main/docs/ops/AGENTIC-MLOPS-CI.md",
                    "properties": {"category": f.category, "layer": layer.id},
                }
            level = f.level if f.level in {"error", "warning", "note"} else "warning"
            uri = f.path.replace("\\", "/") if f.path else "."
            results.append(
                {
                    "ruleId": f.rule_id,
                    "level": level,
                    "message": {"text": f.message},
                    "locations": [
                        {
                            "physicalLocation": {
                                "artifactLocation": {"uri": uri, "uriBaseId": "%SRCROOT%"},
                                "region": {"startLine": max(1, f.start_line)},
                            }
                        }
                    ],
                    "properties": {
                        "category": f.category,
                        "layer": layer.id,
                        "epistemic": layer.epistemic,
                    },
                }
            )
    return {
        "version": "2.1.0",
        "$schema": "https://json.schemastore.org/sarif-2.1.0.json",
        "runs": [
            {
                "tool": {
                    "driver": {
                        "name": "LogOS CODEX",
                        "version": SCAN_VERSION,
                        "informationUri": "https://github.com/toolate28/LogOS",
                        "rules": list(rules.values()),
                    }
                },
                "results": results,
                "originalUriBaseIds": {
                    "%SRCROOT%": {"uri": root.as_uri().rstrip("/") + "/"}
                },
                "properties": {"atom": ATOM_ID},
            }
        ],
    }


def badge(label: str, message: str, color: str) -> dict[str, Any]:
    return {
        "schemaVersion": 1,
        "label": label,
        "message": message,
        "color": color,
        "namedLogo": "githubactions",
        "cacheSeconds": 300,
    }


def color_for_score(score: int) -> str:
    if score >= 90:
        return "brightgreen"
    if score >= 85:
        return "green"
    if score >= 70:
        return "yellow"
    if score >= 50:
        return "orange"
    return "red"


def build_badges(
    score: int,
    layers: list[LayerResult],
    sec_errs: list[Finding],
) -> dict[str, dict[str, Any]]:
    pin = next(l for l in layers if l.id == "L0_action_pin")
    agentic = next(l for l in layers if l.id == "L5_agentic")
    topo = next(l for l in layers if l.id == "L6_topology_telemetry")
    mlops_ok = score >= 85 and not sec_errs
    return {
        "codex.json": badge(
            "codex",
            f"{score}/100",
            "red" if sec_errs else color_for_score(score),
        ),
        "action-pin.json": badge(
            "actions",
            "SHA-pinned" if pin.score == 100 else f"pin {pin.score}%",
            "brightgreen" if pin.score == 100 else "red",
        ),
        "agentic-mlops.json": badge(
            "agentic-mlops",
            "pass" if mlops_ok else ("blocked" if sec_errs else "amber"),
            "brightgreen" if mlops_ok else ("red" if sec_errs else "yellow"),
        ),
        "wave-posture.json": badge(
            "wave-posture",
            f"publish≥85 · score {score}",
            color_for_score(score),
        ),
        "topology-tag.json": badge(
            "α+ω",
            f"{topo.metrics.get('sum', '?')} (C)",
            "blue",
        ),
        "ci-matrix.json": badge(
            "ci-matrix",
            f"agentic {agentic.score}% · codex {score}",
            color_for_score(min(agentic.score, score)),
        ),
    }


def write_step_summary(report: dict[str, Any]) -> None:
    path = os.environ.get("GITHUB_STEP_SUMMARY")
    if not path:
        return
    layers = report["layers"]
    lines = [
        "## LogOS CODEX · Agentic MLOps",
        "",
        f"- ATOM: `{report['atom']}`",
        f"- Composite score: **{report['composite_score']}/100**",
        f"- Security errors: **{report['security_error_count']}**",
        f"- Gate: **{report['gate']}**",
        f"- Epistemic: scores are Category B/C tooling — not Category A proofs",
        "",
        "| Layer | Score | Weight | Epistemic | Findings |",
        "|-------|------:|-------:|:---------:|---------:|",
    ]
    for layer in layers:
        lines.append(
            f"| {layer['name']} | {layer['score']} | {layer['weight']:.2f} | "
            f"{layer['epistemic']} | {len(layer['findings'])} |"
        )
    lines.extend(
        [
            "",
            "### Badges (shields endpoint)",
            "",
            "Served from `docs/badges/*.json` after merge to default branch / Pages.",
            "",
            "capability ≠ authority.",
        ]
    )
    with open(path, "a", encoding="utf-8") as fh:
        fh.write("\n".join(lines) + "\n")


def run(root: Path, out_dir: Path, badge_dir: Path, fail_on_security: bool) -> int:
    layers = [
        layer_action_pins(root),
        layer_secrets(root),
        layer_mcp_policy(root),
        layer_formal_residuals(root),
        layer_ci_surface(root),
        layer_agentic_surface(root),
        layer_topology_telemetry(root),
    ]
    score = composite_score(layers)
    sec_errs = security_errors(layers)
    gate = "pass" if not sec_errs and score >= 85 else ("fail" if sec_errs else "amber")

    report = {
        "codex_version": SCAN_VERSION,
        "atom": ATOM_ID,
        "timestamp": dt.datetime.now(dt.timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
        "target": str(root),
        "composite_score": score,
        "gate": gate,
        "security_error_count": len(sec_errs),
        "publish_gate": 85,
        "layers": [
            {
                **{k: v for k, v in asdict(layer).items() if k != "findings"},
                "findings": [asdict(f) for f in layer.findings],
            }
            for layer in layers
        ],
        "notes": [
            "α+ω=15 is Category C telemetry only — not a reject gate.",
            "WAVE publish gate is 85 on 0-100 (see docs/security/WAVE-SCALE.md).",
            "capability ≠ authority.",
        ],
    }

    out_dir.mkdir(parents=True, exist_ok=True)
    badge_dir.mkdir(parents=True, exist_ok=True)

    report_path = out_dir / "report.json"
    sarif_path = out_dir / "codex.sarif"
    report_path.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    sarif_path.write_text(json.dumps(to_sarif(layers, root), indent=2) + "\n", encoding="utf-8")

    badges = build_badges(score, layers, sec_errs)
    for name, payload in badges.items():
        (badge_dir / name).write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")
    # index for humans
    (badge_dir / "README.md").write_text(
        "\n".join(
            [
                "# LogOS CI / Agentic MLOps badges",
                "",
                "Shields.io endpoint JSON (schemaVersion 1).",
                "",
                "```text",
                "https://img.shields.io/endpoint?url=https://toolate28.github.io/LogOS/badges/codex.json",
                "```",
                "",
                f"Generated by `{ATOM_ID}` / CODEX {SCAN_VERSION}.",
                "Scores are tooling posture, not Category A proofs.",
                "",
            ]
        ),
        encoding="utf-8",
    )

    write_step_summary(report)

    print(f"CODEX composite={score} gate={gate} security_errors={len(sec_errs)}")
    print(f"report: {report_path}")
    print(f"sarif:  {sarif_path}")
    print(f"badges: {badge_dir}")
    for layer in layers:
        print(f"  [{layer.epistemic}] {layer.id:24} score={layer.score:3} findings={len(layer.findings)}")
    for f in sec_errs[:20]:
        print(f"  !! {f.rule_id}: {f.message.splitlines()[0][:120]}")

    if fail_on_security and sec_errs:
        return 1
    return 0


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description="LogOS CODEX / Agentic MLOps scanner")
    ap.add_argument("--root", type=Path, default=ROOT)
    ap.add_argument(
        "--out",
        type=Path,
        default=ROOT / "artifacts" / "codex",
        help="Directory for report.json + codex.sarif",
    )
    ap.add_argument(
        "--badge-dir",
        type=Path,
        default=ROOT / "docs" / "badges",
        help="Shields endpoint JSON output directory",
    )
    ap.add_argument(
        "--fail-on-security",
        action="store_true",
        default=True,
        help="Exit 1 on security-category errors (default: true)",
    )
    ap.add_argument(
        "--no-fail-on-security",
        action="store_true",
        help="Always exit 0 (advisory mode)",
    )
    args = ap.parse_args(argv)
    fail = args.fail_on_security and not args.no_fail_on_security
    return run(args.root.resolve(), args.out.resolve(), args.badge_dir.resolve(), fail)


if __name__ == "__main__":
    sys.exit(main())
