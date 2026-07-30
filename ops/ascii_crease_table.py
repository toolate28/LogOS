#!/usr/bin/env python3
"""
Mutation-resilient ASCII crease tables for LogOS / Tri-Weavon docs.

Reidemeister-protected: row reordering and cell width drift do not tear
structure — borders are recomputed from content (Music conserved).

Crease families (origami bridge → code mapping):
  miura     — alternating mountain/valley grid (Miura-ori)
  kresling  — diagonal helical lattice (Kresling bellows)
  waterbomb — diamond tessellation (Waterbomb base)
  reidemeister — I/II/III move markers on edges (isotopy class labels)

Usage:
  from ascii_crease_table import crease_table, md_tables_to_crease
  print(crease_table(["A","B"], [["1","2"]], style="miura"))
  python ops/ascii_crease_table.py --file docs/foo.md --in-place
"""

from __future__ import annotations

import argparse
import hashlib
import re
import sys
from pathlib import Path
from typing import Iterable, List, Optional, Sequence

STYLES = ("miura", "kresling", "waterbomb", "reidemeister", "etch")

# Shade ramps (etch-a-sketch / density)
SHADE = " ░▒▓█"
SHADE_INV = "█▓▒░ "


def _cell(s: str, width: int) -> str:
    s = " ".join(str(s).replace("\n", " ").split())
    if len(s) > width:
        s = s[: max(0, width - 1)] + "…"
    return s.ljust(width)


def _widths(headers: Sequence[str], rows: Sequence[Sequence[str]], min_w: int = 3) -> List[int]:
    cols = len(headers)
    w = [max(min_w, len(str(h))) for h in headers]
    for row in rows:
        for i in range(cols):
            cell = str(row[i]) if i < len(row) else ""
            w[i] = max(w[i], min(48, len(cell)))
    return w


def _shade_bar(frac: float, width: int = 8) -> str:
    frac = max(0.0, min(1.0, frac))
    filled = int(round(frac * width))
    return SHADE[-1] * filled + SHADE[1] * (width - filled)


def crease_table(
    headers: Sequence[str],
    rows: Sequence[Sequence[str]],
    *,
    style: str = "miura",
    title: Optional[str] = None,
    conserving: bool = True,
) -> str:
    """Render a mutation-resilient crease table (pure function of cells)."""
    if style not in STYLES:
        style = "miura"
    headers = [str(h) for h in headers]
    rows = [[str(c) for c in r] for r in rows]
    # Pad ragged rows (Reidemeister II cancel of missing cells)
    cols = len(headers)
    norm = []
    for r in rows:
        rr = list(r) + [""] * max(0, cols - len(r))
        norm.append(rr[:cols])
    rows = norm
    w = _widths(headers, rows)

    # Corner / edge glyphs by crease family
    if style == "miura":
        tl, tr, bl, br = "╭", "╮", "╰", "╯"
        h, v, jn, js, jw, je, jx = "─", "│", "┬", "┴", "├", "┤", "┼"
        valley, mountain = "╲", "╱"
    elif style == "kresling":
        tl, tr, bl, br = "╔", "╗", "╚", "╝"
        h, v, jn, js, jw, je, jx = "═", "║", "╦", "╩", "╠", "╣", "╬"
        valley, mountain = "◇", "◆"
    elif style == "waterbomb":
        tl, tr, bl, br = "◢", "◣", "◥", "◤"
        h, v, jn, js, jw, je, jx = "─", "│", "┬", "┴", "├", "┤", "┼"
        valley, mountain = "▽", "△"
    elif style == "reidemeister":
        tl, tr, bl, br = "┌", "┐", "└", "┘"
        h, v, jn, js, jw, je, jx = "─", "│", "┬", "┴", "├", "┤", "┼"
        valley, mountain = "I", "II"  # type markers in footer
    else:  # etch
        tl, tr, bl, br = "+", "+", "+", "+"
        h, v, jn, js, jw, je, jx = "-", "|", "+", "+", "+", "+", "+"
        valley, mountain = ".", "#"

    def hline(left: str, mid: str, right: str) -> str:
        parts = [h * (wi + 2) for wi in w]
        return left + mid.join(parts) + right

    def row_line(cells: Sequence[str]) -> str:
        body = v.join(f" {_cell(c, wi)} " for c, wi in zip(cells, w))
        return f"{v}{body}{v}"

    lines: List[str] = []
    fingerprint = hashlib.sha256(
        ("|".join(headers) + "||" + "||".join("|".join(r) for r in rows)).encode()
    ).hexdigest()[:12]

    if title:
        lines.append(f"  {mountain} {title} {valley}")
    lines.append(hline(tl, jn, tr))
    lines.append(row_line(headers))
    lines.append(hline(jw, jx, je))
    for i, r in enumerate(rows):
        lines.append(row_line(r))
        # Miura alternation: subtle separator every other row for fold memory
        if style == "miura" and i < len(rows) - 1 and i % 2 == 1:
            lines.append(hline(jw, jx, je).replace(h, "·"))
    lines.append(hline(bl, js, br))
    if conserving:
        lines.append(
            f"  α+ω=15 · crease={style} · reidemeister-protected · σ={fingerprint}"
        )
    return "\n".join(lines)


def parse_md_table(block: str) -> Optional[tuple]:
    lines = [ln.rstrip() for ln in block.strip().splitlines() if ln.strip()]
    if len(lines) < 2:
        return None
    if not all("|" in ln for ln in lines[:2]):
        return None

    def split_row(ln: str) -> List[str]:
        ln = ln.strip()
        if ln.startswith("|"):
            ln = ln[1:]
        if ln.endswith("|"):
            ln = ln[:-1]
        return [c.strip() for c in ln.split("|")]

    headers = split_row(lines[0])
    # separator line
    sep = lines[1].replace("|", "").replace("-", "").replace(":", "").strip()
    if sep != "":
        # might not be a standard md table
        data_start = 1
    else:
        data_start = 2
    rows = [split_row(ln) for ln in lines[data_start:]]
    return headers, rows


_MD_TABLE_RE = re.compile(
    r"(?:^|\n)((?:[ \t]*\|.+\|[ \t]*\n){2,})",
    re.MULTILINE,
)


def md_tables_to_crease(text: str, style: str = "miura") -> str:
    """Replace GitHub-flavored markdown tables with crease tables in fenced code."""

    def repl(m: re.Match) -> str:
        block = m.group(1)
        parsed = parse_md_table(block)
        if not parsed:
            return m.group(0)
        headers, rows = parsed
        # skip separator-only second row if present in rows
        if rows and all(re.fullmatch(r":?-{3,}:?", c or "") for c in rows[0]):
            rows = rows[1:]
        table = crease_table(headers, rows, style=style)
        return "\n\n```crease\n" + table + "\n```\n\n"

    return _MD_TABLE_RE.sub(repl, text)


def main(argv: Optional[Sequence[str]] = None) -> int:
    p = argparse.ArgumentParser(description="Crease-table transform for markdown")
    p.add_argument("--file", type=Path, help="Markdown file to transform")
    p.add_argument("--in-place", action="store_true")
    p.add_argument("--style", default="miura", choices=STYLES)
    p.add_argument("--demo", action="store_true")
    args = p.parse_args(argv)

    if args.demo or not args.file:
        print(
            crease_table(
                ["Surface", "Port", "SC", "Shade"],
                [
                    ["waist", "8080", "PASS", _shade_bar(1.0)],
                    ["bbbr", "8081", "PASS", _shade_bar(0.92)],
                    ["styx", "5640", "TCP", _shade_bar(0.85)],
                    ["kind", "37601", "UP", _shade_bar(0.78)],
                ],
                style=args.style,
                title="UNITARY RELEASE · deploy waist",
            )
        )
        return 0

    text = args.file.read_text(encoding="utf-8")
    out = md_tables_to_crease(text, style=args.style)
    if args.in_place:
        args.file.write_text(out, encoding="utf-8")
        print(f"wrote {args.file}", file=sys.stderr)
    else:
        sys.stdout.write(out)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
