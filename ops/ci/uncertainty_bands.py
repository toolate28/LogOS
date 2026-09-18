#!/usr/bin/env python3
"""Deterministic execution restriction bands for ephemeral shared context.

These signals describe execution uncertainty only. They do not infer mental
state, intent, moral worth, or consciousness.
"""
from __future__ import annotations

from dataclasses import dataclass
from typing import Any

MAX_PENALTY = 19


@dataclass(frozen=True)
class UncertaintyAssessment:
    stress: float | None
    strain: float | None
    provenance_confidence: float | None
    crosscheck_confidence: float | None
    rollback_available: bool | None
    uncertainty_score: float
    execution_band: str
    restriction_reasons: tuple[str, ...]

    def as_dict(self) -> dict[str, Any]:
        return {
            "stress": self.stress,
            "strain": self.strain,
            "provenance_confidence": self.provenance_confidence,
            "crosscheck_confidence": self.crosscheck_confidence,
            "rollback_available": self.rollback_available,
            "uncertainty_score": self.uncertainty_score,
            "execution_band": self.execution_band,
            "restriction_reasons": list(self.restriction_reasons),
        }


def _bounded(name: str, value: float | None) -> float | None:
    if value is None:
        return None
    if not 0.0 <= value <= 1.0:
        raise ValueError(f"{name} must be within [0, 1]")
    return round(float(value), 6)


def _high_signal_penalty(label: str, value: float | None) -> tuple[int, list[str]]:
    if value is None:
        return 3, [f"{label} missing"]
    if value >= 0.85:
        return 4, [f"{label} high"]
    if value >= 0.65:
        return 3, [f"{label} elevated"]
    if value >= 0.45:
        return 2, [f"{label} moderate"]
    if value >= 0.25:
        return 1, [f"{label} noticeable"]
    return 0, []


def _confidence_penalty(label: str, value: float | None) -> tuple[int, list[str]]:
    if value is None:
        return 2, [f"{label} missing"]
    if value < 0.20:
        return 4, [f"{label} critically low"]
    if value < 0.40:
        return 3, [f"{label} low"]
    if value < 0.60:
        return 2, [f"{label} limited"]
    if value < 0.80:
        return 1, [f"{label} incomplete"]
    return 0, []


def assess_uncertainty(
    *,
    stress: float | None,
    strain: float | None,
    provenance_confidence: float | None,
    crosscheck_confidence: float | None,
    rollback_available: bool | None,
) -> UncertaintyAssessment:
    stress = _bounded("stress", stress)
    strain = _bounded("strain", strain)
    provenance_confidence = _bounded("provenance_confidence", provenance_confidence)
    crosscheck_confidence = _bounded("crosscheck_confidence", crosscheck_confidence)

    penalty = 0
    reasons: list[str] = []
    missing_count = 0

    for label, value, scorer in (
        ("stress", stress, _high_signal_penalty),
        ("strain", strain, _high_signal_penalty),
        ("provenance_confidence", provenance_confidence, _confidence_penalty),
        ("crosscheck_confidence", crosscheck_confidence, _confidence_penalty),
    ):
        if value is None:
            missing_count += 1
        component_penalty, component_reasons = scorer(label, value)
        penalty += component_penalty
        reasons.extend(component_reasons)

    if rollback_available is None:
        missing_count += 1
        penalty += 3
        reasons.append("rollback availability unknown")
    elif not rollback_available:
        penalty += 2
        reasons.append("rollback unavailable")

    score = round(min(1.0, penalty / MAX_PENALTY), 4)
    if missing_count == 5:
        score = 1.0
    if score >= 0.80:
        band = "fully_restricted"
    elif score >= 0.60:
        band = "quarantine"
    elif score >= 0.40:
        band = "constrained"
    elif score >= 0.20:
        band = "reversible"
    else:
        band = "analysis"

    return UncertaintyAssessment(
        stress=stress,
        strain=strain,
        provenance_confidence=provenance_confidence,
        crosscheck_confidence=crosscheck_confidence,
        rollback_available=rollback_available,
        uncertainty_score=score,
        execution_band=band,
        restriction_reasons=tuple(reasons),
    )
