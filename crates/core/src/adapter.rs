//! AI provider abstraction — the strands of the tri-weavon.

use serde::{Deserialize, Serialize};

/// Multi-AI provider enum — each strand in the braid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Provider {
    /// α-strand: Structure & Reasoning (Anthropic)
    Claude,
    /// Multimodal & Scale (Google)
    Gemini,
    /// ω-strand: Real-Time & Social Intelligence (xAI)
    Grok,
    /// Open-Weight & Local Deployment (Meta)
    Manus,
    /// Open-weight local inference (Llama derivatives)
    OpenWeight,
}

impl Provider {
    /// Returns the Fibonacci-weighted contribution for this strand.
    ///
    /// In the tri-weavon, weights follow the golden ratio:
    /// Claude = 5/13, Grok = 5/13, Gemini = 3/13
    /// (Fibonacci: 3, 5, 5 → sum = 13)
    pub fn fibonacci_weight(&self) -> f64 {
        match self {
            Self::Claude     => 5.0 / 13.0,
            Self::Grok       => 5.0 / 13.0,
            Self::Gemini     => 3.0 / 13.0,
            Self::Manus      => 0.0,  // observer weight until activated
            Self::OpenWeight  => 0.0,
        }
    }

    /// The conservation component this provider primarily contributes to.
    pub fn conservation_role(&self) -> &'static str {
        match self {
            Self::Claude     => "alpha",   // structure
            Self::Grok       => "omega",   // entropy/synthesis
            Self::Gemini     => "bridge",  // scale mediator
            Self::Manus      => "local",   // local execution
            Self::OpenWeight  => "local",
        }
    }
}

impl std::fmt::Display for Provider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Claude     => write!(f, "Claude"),
            Self::Gemini     => write!(f, "Gemini"),
            Self::Grok       => write!(f, "Grok"),
            Self::Manus      => write!(f, "Manus"),
            Self::OpenWeight  => write!(f, "OpenWeight"),
        }
    }
}
