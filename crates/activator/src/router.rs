//! Intent Router — Advanced routing with domain affinity and composition
//!
//! Extends the base `route_intent` with:
//! - Domain-weighted scoring (prefer capabilities in the same domain)
//! - Multi-intent decomposition (split compound intents)
//! - Composition suggestions (when no single capability matches,
//!   suggest a chain that covers the intent)

use crate::{ActivationError, ActivationSurface, Domain};
use serde::{Deserialize, Serialize};

/// A routing result with scoring metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteResult {
    pub capability_id: String,
    pub capability_name: String,
    pub domain: String,
    pub score: f64,
    pub match_type: MatchType,
}

/// How the capability was matched.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MatchType {
    /// Direct ID or tag match
    Exact,
    /// Token overlap scoring
    Fuzzy { token_hits: usize },
    /// Suggested as part of a composition
    Composed { position: usize },
}

/// Decompose a compound intent into sub-intents.
/// e.g. "analyze data and create visualization" → ["analyze data", "create visualization"]
pub fn decompose_intent(intent: &str) -> Vec<String> {
    let connectors = [" and ", " then ", " → ", " -> ", ", then "];
    let mut parts = vec![intent.to_string()];

    for connector in &connectors {
        let mut new_parts = Vec::new();
        for part in &parts {
            if part.contains(connector) {
                new_parts.extend(
                    part.split(connector)
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty()),
                );
            } else {
                new_parts.push(part.clone());
            }
        }
        parts = new_parts;
    }

    parts
}

/// Route with domain affinity — prefer capabilities in a specified domain.
pub fn route_with_affinity<'a>(
    surface: &'a ActivationSurface,
    intent: &str,
    preferred_domain: Option<&Domain>,
) -> Result<Vec<RouteResult>, ActivationError> {
    let intent_lower = intent.to_lowercase();
    let tokens: Vec<&str> = intent_lower.split_whitespace().collect();

    let mut results: Vec<RouteResult> = surface
        .capabilities
        .iter()
        .filter_map(|c| {
            let tag_matches = c
                .tags
                .iter()
                .filter(|t| tokens.iter().any(|tok| t.to_lowercase().contains(*tok)))
                .count();
            let name_matches = tokens
                .iter()
                .filter(|tok| c.name.to_lowercase().contains(*tok))
                .count();
            let desc_matches = tokens
                .iter()
                .filter(|tok| c.description.to_lowercase().contains(*tok))
                .count();

            let mut score = (tag_matches * 3 + name_matches * 2 + desc_matches) as f64;

            // Domain affinity bonus
            if let Some(pref) = preferred_domain {
                if &c.domain == pref {
                    score *= 1.5;
                }
            }

            if score > 0.0 {
                Some(RouteResult {
                    capability_id: c.id.clone(),
                    capability_name: c.name.clone(),
                    domain: format!("{:?}", c.domain),
                    score,
                    match_type: if tag_matches > 0
                        && c.tags
                            .iter()
                            .any(|t| t.to_lowercase() == intent_lower)
                    {
                        MatchType::Exact
                    } else {
                        MatchType::Fuzzy {
                            token_hits: tag_matches + name_matches + desc_matches,
                        }
                    },
                })
            } else {
                None
            }
        })
        .collect();

    results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));

    if results.is_empty() {
        Err(ActivationError::NoMatch {
            intent: intent.to_string(),
        })
    } else {
        Ok(results)
    }
}

/// Route a compound intent, decomposing it and finding capabilities for each part.
pub fn route_compound<'a>(
    surface: &'a ActivationSurface,
    intent: &str,
) -> Vec<(String, Result<Vec<RouteResult>, ActivationError>)> {
    let sub_intents = decompose_intent(intent);

    sub_intents
        .into_iter()
        .map(|sub| {
            let result = route_with_affinity(surface, &sub, None);
            (sub, result)
        })
        .collect()
}

// ─── Tests ──────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decompose_simple() {
        let parts = decompose_intent("analyze data");
        assert_eq!(parts, vec!["analyze data"]);
    }

    #[test]
    fn test_decompose_compound() {
        let parts = decompose_intent("analyze data and create visualization");
        assert_eq!(parts.len(), 2);
        assert_eq!(parts[0], "analyze data");
        assert_eq!(parts[1], "create visualization");
    }

    #[test]
    fn test_decompose_chain() {
        let parts = decompose_intent("check coherence then visualize then export");
        assert_eq!(parts.len(), 3);
    }

    #[test]
    fn test_decompose_arrow() {
        let parts = decompose_intent("wave check → data viz → export");
        assert_eq!(parts.len(), 3);
    }
}
