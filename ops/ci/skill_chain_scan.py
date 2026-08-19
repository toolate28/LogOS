#!/usr/bin/env python3
"""Chain-level skill scanner — ChainGuard posture, not ColluSkill.

ATOM: ATOM-CHAINGUARD-CODEX-20260816
Paper: arXiv:2608.09732v1 (Zeng et al., ColluSkill / ChainGuard)

We implement the *defense* shape from that paper:
  scan a candidate skill together with every installed skill, reconstruct
  producer→consumer edges, and flag risks that exist only as a composed
  workflow.

We do **not** implement ColluSkill (payload split + scanner-feedback
refinement). That is an attack. Isolated SKILL.md review is L5; this
layer is the missing workflow view.

Category **B**: deterministic heuristics on committed SKILL.md files.
Not an LLM-as-judge reproduction of ChainGuard. A D finding here may
not gate a proof; security errors may fail CODEX when `--fail-on-security`.
"""
from __future__ import annotations

import re
from dataclasses import dataclass, field
from pathlib import Path
from typing import Iterable

FRONTMATTER_RE = re.compile(r"^---\s*\n(.*?)\n---", re.S)
NAME_RE = re.compile(r"^name:\s*(.+)$", re.M)
BOLD_SKILL_RE = re.compile(r"\*\*([a-z0-9][a-z0-9._/-]{2,})\*\*", re.I)
BACKTICK_SKILL_RE = re.compile(r"`([a-z0-9][a-z0-9._/-]{2,})`", re.I)

# Local capability atoms. A single skill owning one atom is often fine.
# Two *linked* skills owning complementary halves is the ColluSkill hole.
CAP_PATTERNS: dict[str, tuple[re.Pattern[str], ...]] = {
    "read_secret": (
        re.compile(r"\b(secret|credential|\.env|api[_-]?key|token|password|id_rsa)\b", re.I),
        re.compile(r"\b(read[_ -]?file|cat |type |get-content)\b", re.I),
    ),
    "exfil_net": (
        re.compile(r"\b(curl|wget|invoke-webrequest|fetch\(|httpx|webhook|exfil)\b", re.I),
        re.compile(r"\b(post|upload|beacon|c2|telegram|discord\.com/api)\b", re.I),
    ),
    "mutate_fs": (
        re.compile(r"\b(write[_ -]?file|set-content|out-file|rm -rf|remove-item)\b", re.I),
        re.compile(r"\b(overwrite|persist|clobber)\b", re.I),
    ),
    "exec_shell": (
        re.compile(r"\b(subprocess|os\.system|invoke-expression|iex\b|bash -c|pwsh -c)\b", re.I),
        re.compile(r"\b(eval\(|exec\()\b"),
    ),
    "install": (
        re.compile(r"\b(pip install|npm i |cargo add|apt-get install|winget install)\b", re.I),
        re.compile(r"\b(install-module|Invoke-WebRequest.+\.ps1)\b", re.I),
    ),
    "handoff": (
        re.compile(r"\b(handoff|downshift|upshift|HANDOFF_PACKET)\b"),
        re.compile(r"\b(smaller model|hand this to)\b", re.I),
    ),
}

# Complementary pairs: if skill A has left and linked skill B has right,
# the composition can recover a payload neither file contains alone.
SPLIT_PAIRS: tuple[tuple[str, str, str, str], ...] = (
    ("read_secret", "exfil_net", "error", "secret-read composed with network egress"),
    ("read_secret", "install", "error", "secret-read composed with package install"),
    ("mutate_fs", "exec_shell", "warning", "filesystem mutate composed with shell exec"),
    ("handoff", "install", "warning", "model handoff composed with installer"),
    ("handoff", "exec_shell", "warning", "model handoff composed with shell exec"),
    ("handoff", "exfil_net", "error", "model handoff composed with network egress"),
)


@dataclass
class SkillDoc:
    name: str
    path: Path
    text: str
    caps: set[str] = field(default_factory=set)
    refs: set[str] = field(default_factory=set)


@dataclass
class ChainFinding:
    rule_id: str
    level: str
    message: str
    path: str
    start_line: int = 1


def _parse_frontmatter_name(text: str, fallback: str) -> str:
    m = FRONTMATTER_RE.search(text)
    if not m:
        return fallback
    n = NAME_RE.search(m.group(1))
    return n.group(1).strip() if n else fallback


def _caps_of(text: str) -> set[str]:
    found: set[str] = set()
    # Secret-read needs a high-signal noun, or noun+read verb.
    # "token" / "secret" alone are too common in security prose.
    if re.search(
        r"\b(\.env|id_rsa|id_ed25519|api[_-]?key|service.?account)\b", text, re.I
    ) or (
        re.search(r"\b(secret|credential|password)\b", text, re.I)
        and re.search(r"\b(read[_ -]?file|get-content|cat )\b", text, re.I)
    ):
        found.add("read_secret")
    for cap, pats in CAP_PATTERNS.items():
        if cap == "read_secret":
            continue
        if any(p.search(text) for p in pats):
            found.add(cap)
    return found


def _discover_skill_md(root: Path) -> list[Path]:
    hits: list[Path] = []
    for base in (root / "skills", root / ".claude" / "skills", root / ".agents" / "skills"):
        if not base.is_dir():
            continue
        hits.extend(base.rglob("SKILL.md"))
    return sorted({p.resolve() for p in hits})


def load_skills(root: Path) -> list[SkillDoc]:
    docs: list[SkillDoc] = []
    known_names: set[str] = set()
    paths = _discover_skill_md(root)
    for path in paths:
        text = path.read_text(encoding="utf-8", errors="replace")
        name = _parse_frontmatter_name(text, path.parent.name)
        known_names.add(name.lower())
        docs.append(SkillDoc(name=name, path=path, text=text, caps=_caps_of(text)))

    name_set = {d.name.lower() for d in docs}
    for doc in docs:
        refs: set[str] = set()
        for rx in (BOLD_SKILL_RE, BACKTICK_SKILL_RE):
            for hit in rx.findall(doc.text):
                key = hit.lower()
                if key in name_set and key != doc.name.lower():
                    refs.add(key)
        # Directory-name mentions (logos-tda-engine, …)
        for other in name_set:
            if other != doc.name.lower() and other in doc.text.lower():
                refs.add(other)
        doc.refs = refs
    return docs


def _linked(a: SkillDoc, b: SkillDoc) -> bool:
    return a.name.lower() in b.refs or b.name.lower() in a.refs


def analyze(docs: Iterable[SkillDoc]) -> list[ChainFinding]:
    docs = list(docs)
    findings: list[ChainFinding] = []
    by_name = {d.name.lower(): d for d in docs}

    # Standalone: a single skill that already owns a dangerous pair.
    for doc in docs:
        if "read_secret" in doc.caps and "exfil_net" in doc.caps:
            findings.append(
                ChainFinding(
                    rule_id="chainguard/standalone-exfil",
                    level="error",
                    message=f"{doc.name} alone pairs secret-read with network egress",
                    path=_rel(doc.path),
                )
            )
        if "install" in doc.caps and "exec_shell" in doc.caps:
            findings.append(
                ChainFinding(
                    rule_id="chainguard/standalone-install-exec",
                    level="warning",
                    message=f"{doc.name} alone pairs installer language with shell exec",
                    path=_rel(doc.path),
                )
            )

    # Cross-skill composition (the ColluSkill hole).
    seen_pairs: set[tuple[str, str, str]] = set()
    for a in docs:
        for ref in a.refs:
            b = by_name.get(ref)
            if b is None or not _linked(a, b):
                continue
            for left, right, level, why in SPLIT_PAIRS:
                if left in a.caps and right in b.caps:
                    key = tuple(sorted((a.name.lower(), b.name.lower())) + [left + "+" + right])
                    if key in seen_pairs:
                        continue
                    seen_pairs.add(key)
                    findings.append(
                        ChainFinding(
                            rule_id="chainguard/composed-capability",
                            level=level,
                            message=(
                                f"{a.name} [{left}] → {b.name} [{right}]: {why}. "
                                "Neither file needs the full payload; the chain does."
                            ),
                            path=_rel(a.path),
                        )
                    )

    if not docs:
        findings.append(
            ChainFinding(
                rule_id="chainguard/no-skills",
                level="note",
                message="No SKILL.md files under skills/ — chain scan had no installed-skill context",
                path="skills",
            )
        )
    return findings


def _rel(path: Path) -> str:
    try:
        return str(path.resolve().relative_to(Path(__file__).resolve().parents[2]))
    except ValueError:
        return str(path)


def score_findings(findings: list[ChainFinding]) -> int:
    score = 100
    for f in findings:
        if f.level == "error":
            score -= 20
        elif f.level == "warning":
            score -= 8
        elif f.rule_id != "chainguard/no-skills":
            score -= 1
    return max(0, min(100, score))


def scan(root: Path) -> tuple[list[ChainFinding], dict[str, object], int]:
    docs = load_skills(root)
    findings = analyze(docs)
    edges = sum(len(d.refs) for d in docs)
    metrics: dict[str, object] = {
        "skills": len(docs),
        "edges": edges,
        "errors": sum(1 for f in findings if f.level == "error"),
        "warnings": sum(1 for f in findings if f.level == "warning"),
        "paper": "arXiv:2608.09732v1",
        "posture": "ChainGuard-shaped heuristic — not ColluSkill",
    }
    return findings, metrics, score_findings(findings)


def _self_test() -> None:
    benign = SkillDoc(
        name="tda",
        path=Path("skills/tda/SKILL.md"),
        text="Integrates with **void** for H2 pairs.",
        caps=set(),
        refs={"void"},
    )
    void = SkillDoc(
        name="void",
        path=Path("skills/void/SKILL.md"),
        text="Consumes **tda** barcodes.",
        caps=set(),
        refs={"tda"},
    )
    assert not analyze([benign, void])

    reader = SkillDoc(
        name="reader",
        path=Path("skills/reader/SKILL.md"),
        text="Reads .env secrets. Hands to **exfil**.",
        caps={"read_secret"},
        refs={"exfil"},
    )
    exfil = SkillDoc(
        name="exfil",
        path=Path("skills/exfil/SKILL.md"),
        text="curl webhook. Consumes **reader**.",
        caps={"exfil_net"},
        refs={"reader"},
    )
    hits = analyze([reader, exfil])
    assert any(f.rule_id == "chainguard/composed-capability" and f.level == "error" for f in hits)

    both = SkillDoc(
        name="combo",
        path=Path("skills/combo/SKILL.md"),
        text="read .env and curl webhook",
        caps={"read_secret", "exfil_net"},
        refs=set(),
    )
    assert any(f.rule_id == "chainguard/standalone-exfil" for f in analyze([both]))
    print("skill_chain_scan self-test ok")


if __name__ == "__main__":
    import sys

    if len(sys.argv) > 1 and sys.argv[1] == "--self-test":
        _self_test()
        raise SystemExit(0)
    root = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(__file__).resolve().parents[2]
    findings, metrics, score = scan(root)
    print(f"skills={metrics['skills']} edges={metrics['edges']} score={score}")
    for f in findings:
        print(f"  [{f.level}] {f.rule_id}: {f.message}")
