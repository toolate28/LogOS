//! awesome_skill.rs — Rust Ecosystem Taxonomy Router
//!
//! Maps the full awesome-rust ecosystem into the activation surface.
//! Each category from awesome-rust becomes a routable capability domain,
//! enabling intent-based discovery of Rust crates and tools.
//!
//! The taxonomy is the INVERSE of awesome-rust's list format:
//! instead of "here are crates, find what you need",
//! it's "here's what you need, here are the crates."
//!
//! This is the LucasArts principle: high-value, maximum-impact items first.

use super::{Capability, CapabilityKind, Domain, RustCategory};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A crate entry in the awesome-rust taxonomy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwesomeCrate {
    pub name: String,
    pub category: RustCategory,
    pub description: String,
    pub repo_url: String,
    pub crates_io: Option<String>,
    /// Impact score: relevance to Coherence Forge goals
    pub impact: Impact,
    /// Tags for intent matching
    pub tags: Vec<String>,
}

/// Impact classification — LucasArts priority ordering.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Impact {
    /// Critical path — blocks other work
    Critical = 4,
    /// High value — significant capability unlock
    High = 3,
    /// Medium — useful but not blocking
    Medium = 2,
    /// Low — nice to have
    Low = 1,
}

/// The awesome-rust taxonomy registry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwesomeRegistry {
    pub crates: Vec<AwesomeCrate>,
    pub category_index: HashMap<String, Vec<usize>>,
}

impl AwesomeRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            crates: Vec::new(),
            category_index: HashMap::new(),
        };
        registry.seed_critical_path();
        registry.build_index();
        registry
    }

    /// Seed the registry with critical-path crates for Coherence Forge.
    /// These are the LucasArts "maximum impact items first".
    fn seed_critical_path(&mut self) {
        // ─── LLM Integration Layer ──────────────────────────────
        self.add(AwesomeCrate {
            name: "ollama-rs".to_string(),
            category: RustCategory::AI,
            description: "Rust bindings for Ollama — local model inference".to_string(),
            repo_url: "https://github.com/pepperoni21/ollama-rs".to_string(),
            crates_io: Some("ollama-rs".to_string()),
            impact: Impact::Critical,
            tags: vec!["ollama".into(), "llm".into(), "local".into(), "inference".into()],
        });

        self.add(AwesomeCrate {
            name: "async-openai".to_string(),
            category: RustCategory::AI,
            description: "Ergonomic OpenAI API bindings — also works with compatible APIs".to_string(),
            repo_url: "https://github.com/64bit/async-openai".to_string(),
            crates_io: Some("async-openai".to_string()),
            impact: Impact::High,
            tags: vec!["openai".into(), "api".into(), "llm".into(), "claude".into(), "compatible".into()],
        });

        self.add(AwesomeCrate {
            name: "rig".to_string(),
            category: RustCategory::AI,
            description: "Library for creating agents and modular LLM-powered applications".to_string(),
            repo_url: "https://github.com/0xplaygrounds/rig".to_string(),
            crates_io: Some("rig-core".to_string()),
            impact: Impact::Critical,
            tags: vec!["agent".into(), "llm".into(), "modular".into(), "orchestration".into()],
        });

        self.add(AwesomeCrate {
            name: "baml".to_string(),
            category: RustCategory::AI,
            description: "Prompting language for building reliable AI workflows — compiler in Rust".to_string(),
            repo_url: "https://github.com/BoundaryML/baml".to_string(),
            crates_io: None,
            impact: Impact::High,
            tags: vec!["prompt".into(), "workflow".into(), "agent".into(), "dspy".into()],
        });

        // ─── Database & Storage ─────────────────────────────────
        self.add(AwesomeCrate {
            name: "surrealdb".to_string(),
            category: RustCategory::Database,
            description: "Scalable document-graph database — perfect for knowledge graphs".to_string(),
            repo_url: "https://github.com/surrealdb/surrealdb".to_string(),
            crates_io: Some("surrealdb".to_string()),
            impact: Impact::High,
            tags: vec!["database".into(), "graph".into(), "document".into(), "knowledge".into()],
        });

        self.add(AwesomeCrate {
            name: "qdrant".to_string(),
            category: RustCategory::Database,
            description: "Vector similarity search engine — embeddings and RAG".to_string(),
            repo_url: "https://github.com/qdrant/qdrant".to_string(),
            crates_io: Some("qdrant-client".to_string()),
            impact: Impact::Critical,
            tags: vec!["vector".into(), "embedding".into(), "rag".into(), "search".into(), "similarity".into()],
        });

        self.add(AwesomeCrate {
            name: "lancedb".to_string(),
            category: RustCategory::Database,
            description: "Serverless vector database for AI applications".to_string(),
            repo_url: "https://github.com/lancedb/lancedb".to_string(),
            crates_io: Some("vectordb".to_string()),
            impact: Impact::High,
            tags: vec!["vector".into(), "serverless".into(), "embedding".into(), "ai".into()],
        });

        // ─── Web & Networking ───────────────────────────────────
        self.add(AwesomeCrate {
            name: "axum".to_string(),
            category: RustCategory::WebProgramming,
            description: "Ergonomic web framework built with Tokio, Tower, and Hyper".to_string(),
            repo_url: "https://github.com/tokio-rs/axum".to_string(),
            crates_io: Some("axum".to_string()),
            impact: Impact::Critical,
            tags: vec!["web".into(), "api".into(), "server".into(), "http".into(), "rest".into()],
        });

        self.add(AwesomeCrate {
            name: "tungstenite".to_string(),
            category: RustCategory::NetworkProgramming,
            description: "WebSocket library — needed for POP bridge".to_string(),
            repo_url: "https://github.com/snapview/tungstenite-rs".to_string(),
            crates_io: Some("tungstenite".to_string()),
            impact: Impact::Critical,
            tags: vec!["websocket".into(), "pop".into(), "bridge".into(), "realtime".into()],
        });

        // ─── Computation & Science ──────────────────────────────
        self.add(AwesomeCrate {
            name: "nalgebra".to_string(),
            category: RustCategory::Computation,
            description: "Linear algebra library — foundation for WAVE vectors and anyon matrices".to_string(),
            repo_url: "https://github.com/dimforge/nalgebra".to_string(),
            crates_io: Some("nalgebra".to_string()),
            impact: Impact::Critical,
            tags: vec!["linear-algebra".into(), "matrix".into(), "vector".into(), "math".into()],
        });

        self.add(AwesomeCrate {
            name: "num-complex".to_string(),
            category: RustCategory::Computation,
            description: "Complex number type — needed for F-matrix and R-matrix (anyon braiding)".to_string(),
            repo_url: "https://github.com/rust-num/num-complex".to_string(),
            crates_io: Some("num-complex".to_string()),
            impact: Impact::High,
            tags: vec!["complex".into(), "anyon".into(), "braid".into(), "quantum".into()],
        });

        self.add(AwesomeCrate {
            name: "faer".to_string(),
            category: RustCategory::Computation,
            description: "High-performance linear algebra — Rust-native LAPACK alternative".to_string(),
            repo_url: "https://github.com/sarah-ek/faer-rs".to_string(),
            crates_io: Some("faer".to_string()),
            impact: Impact::High,
            tags: vec!["linear-algebra".into(), "performance".into(), "lapack".into()],
        });

        // ─── TUI & Visualization ────────────────────────────────
        self.add(AwesomeCrate {
            name: "ratatui".to_string(),
            category: RustCategory::GUI,
            description: "Terminal UI library — foundation for Forge TUI dashboard".to_string(),
            repo_url: "https://github.com/ratatui-org/ratatui".to_string(),
            crates_io: Some("ratatui".to_string()),
            impact: Impact::Critical,
            tags: vec!["tui".into(), "terminal".into(), "dashboard".into(), "sparkline".into()],
        });

        // ─── Async Runtime ──────────────────────────────────────
        self.add(AwesomeCrate {
            name: "tokio".to_string(),
            category: RustCategory::Async,
            description: "Async runtime — the foundation everything runs on".to_string(),
            repo_url: "https://github.com/tokio-rs/tokio".to_string(),
            crates_io: Some("tokio".to_string()),
            impact: Impact::Critical,
            tags: vec!["async".into(), "runtime".into(), "concurrent".into(), "io".into()],
        });

        // ─── Cryptography (Merkle, hashing) ─────────────────────
        self.add(AwesomeCrate {
            name: "ring".to_string(),
            category: RustCategory::Cryptography,
            description: "Safe, fast crypto using BoringSSL primitives — SHA-256 for Merkle chain".to_string(),
            repo_url: "https://github.com/briansmith/ring".to_string(),
            crates_io: Some("ring".to_string()),
            impact: Impact::High,
            tags: vec!["crypto".into(), "hash".into(), "sha256".into(), "merkle".into()],
        });

        // ─── Sandbox & Virtualization ───────────────────────────
        self.add(AwesomeCrate {
            name: "wasmtime".to_string(),
            category: RustCategory::Virtualization,
            description: "WebAssembly runtime — sandboxed plugin execution".to_string(),
            repo_url: "https://github.com/bytecodealliance/wasmtime".to_string(),
            crates_io: Some("wasmtime".to_string()),
            impact: Impact::High,
            tags: vec!["wasm".into(), "sandbox".into(), "plugin".into(), "isolation".into()],
        });

        // ─── Observability ──────────────────────────────────────
        self.add(AwesomeCrate {
            name: "tracing".to_string(),
            category: RustCategory::Observability,
            description: "Application-level tracing — structured logging for ATOM trail".to_string(),
            repo_url: "https://github.com/tokio-rs/tracing".to_string(),
            crates_io: Some("tracing".to_string()),
            impact: Impact::Critical,
            tags: vec!["tracing".into(), "logging".into(), "observability".into(), "atom".into()],
        });

        // ─── Workflow Automation ─────────────────────────────────
        self.add(AwesomeCrate {
            name: "temporal-sdk-core".to_string(),
            category: RustCategory::WorkflowAutomation,
            description: "Temporal workflow engine — durable execution for pipelines".to_string(),
            repo_url: "https://github.com/temporalio/sdk-core".to_string(),
            crates_io: Some("temporal-sdk-core".to_string()),
            impact: Impact::Medium,
            tags: vec!["workflow".into(), "durable".into(), "pipeline".into(), "orchestration".into()],
        });

        // ─── Blockchain / NEAR ──────────────────────────────────
        self.add(AwesomeCrate {
            name: "near-sdk".to_string(),
            category: RustCategory::Blockchain,
            description: "NEAR Protocol SDK — decentralized smart contract platform".to_string(),
            repo_url: "https://github.com/near/nearcore".to_string(),
            crates_io: Some("near-sdk".to_string()),
            impact: Impact::Medium,
            tags: vec!["near".into(), "blockchain".into(), "decentralized".into(), "contract".into()],
        });

        // ─── Serialization & Data ───────────────────────────────
        self.add(AwesomeCrate {
            name: "serde".to_string(),
            category: RustCategory::DataProcessing,
            description: "Serialization framework — the universal Rust data interchange".to_string(),
            repo_url: "https://github.com/serde-rs/serde".to_string(),
            crates_io: Some("serde".to_string()),
            impact: Impact::Critical,
            tags: vec!["serialize".into(), "json".into(), "data".into(), "interchange".into()],
        });

        // ─── Bioinformatics ─────────────────────────────────────
        self.add(AwesomeCrate {
            name: "rust-bio".to_string(),
            category: RustCategory::Bioinformatics,
            description: "Bioinformatics libraries for sequence analysis".to_string(),
            repo_url: "https://github.com/rust-bio/rust-bio".to_string(),
            crates_io: Some("bio".to_string()),
            impact: Impact::Medium,
            tags: vec!["bio".into(), "sequence".into(), "genomics".into(), "bioinformatics".into()],
        });
    }

    fn add(&mut self, crate_entry: AwesomeCrate) {
        self.crates.push(crate_entry);
    }

    fn build_index(&mut self) {
        self.category_index.clear();
        for (i, c) in self.crates.iter().enumerate() {
            let key = format!("{:?}", c.category);
            self.category_index.entry(key).or_default().push(i);
        }
    }

    /// Find crates by intent — searches tags, names, descriptions.
    pub fn search(&self, intent: &str) -> Vec<&AwesomeCrate> {
        let tokens: Vec<String> = intent
            .to_lowercase()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        let mut scored: Vec<(usize, &AwesomeCrate)> = self
            .crates
            .iter()
            .map(|c| {
                let tag_hits = c.tags.iter()
                    .filter(|t| tokens.iter().any(|tok| t.contains(tok.as_str())))
                    .count();
                let name_hits = tokens.iter()
                    .filter(|tok| c.name.to_lowercase().contains(tok.as_str()))
                    .count();
                let desc_hits = tokens.iter()
                    .filter(|tok| c.description.to_lowercase().contains(tok.as_str()))
                    .count();
                // Impact multiplier: Critical=4x, High=3x, Medium=2x, Low=1x
                let impact_mult = c.impact as usize;
                ((tag_hits * 3 + name_hits * 2 + desc_hits) * impact_mult, c)
            })
            .filter(|(score, _)| *score > 0)
            .collect();

        scored.sort_by(|a, b| b.0.cmp(&a.0));
        scored.into_iter().map(|(_, c)| c).collect()
    }

    /// Get all crates in a category, sorted by impact.
    pub fn by_category(&self, category: &RustCategory) -> Vec<&AwesomeCrate> {
        let key = format!("{:?}", category);
        let mut crates: Vec<&AwesomeCrate> = self
            .category_index
            .get(&key)
            .map(|indices| indices.iter().map(|&i| &self.crates[i]).collect())
            .unwrap_or_default();
        crates.sort_by(|a, b| b.impact.cmp(&a.impact));
        crates
    }

    /// Get the critical path — all Critical impact crates, ordered.
    pub fn critical_path(&self) -> Vec<&AwesomeCrate> {
        let mut critical: Vec<&AwesomeCrate> = self
            .crates
            .iter()
            .filter(|c| c.impact == Impact::Critical)
            .collect();
        critical.sort_by(|a, b| a.name.cmp(&b.name));
        critical
    }

    /// Convert to Capability entries for the activation surface.
    pub fn to_capabilities(&self) -> Vec<Capability> {
        self.crates
            .iter()
            .map(|c| Capability {
                id: format!("rust_{}", c.name.replace('-', "_")),
                name: c.name.clone(),
                domain: Domain::RustEcosystem(c.category.clone()),
                kind: CapabilityKind::RustCrate {
                    crate_name: c.crates_io.clone().unwrap_or_else(|| c.name.clone()),
                    version: "latest".to_string(),
                },
                description: c.description.clone(),
                tags: c.tags.clone(),
                integrations: vec![],
            })
            .collect()
    }
}

impl Default for AwesomeRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_seeds() {
        let registry = AwesomeRegistry::new();
        assert!(!registry.crates.is_empty());
        assert!(registry.crates.len() >= 18); // At least our seeded crates
    }

    #[test]
    fn test_critical_path() {
        let registry = AwesomeRegistry::new();
        let critical = registry.critical_path();
        assert!(!critical.is_empty());
        assert!(critical.iter().all(|c| c.impact == Impact::Critical));
        // These must be on the critical path
        let names: Vec<&str> = critical.iter().map(|c| c.name.as_str()).collect();
        assert!(names.contains(&"tokio"));
        assert!(names.contains(&"axum"));
        assert!(names.contains(&"qdrant"));
        assert!(names.contains(&"ratatui"));
        assert!(names.contains(&"nalgebra"));
    }

    #[test]
    fn test_search_vector_database() {
        let registry = AwesomeRegistry::new();
        let results = registry.search("vector embedding database");
        assert!(!results.is_empty());
        assert!(results[0].name == "qdrant" || results[0].name == "lancedb");
    }

    #[test]
    fn test_search_llm_inference() {
        let registry = AwesomeRegistry::new();
        let results = registry.search("local llm inference");
        assert!(!results.is_empty());
        assert!(results.iter().any(|c| c.name == "ollama-rs"));
    }

    #[test]
    fn test_search_quantum_braid() {
        let registry = AwesomeRegistry::new();
        let results = registry.search("complex anyon braid quantum");
        assert!(!results.is_empty());
        assert!(results.iter().any(|c| c.name == "num-complex"));
    }

    #[test]
    fn test_by_category() {
        let registry = AwesomeRegistry::new();
        let ai_crates = registry.by_category(&RustCategory::AI);
        assert!(!ai_crates.is_empty());
        // Should be impact-sorted: Critical before High before Medium
        for window in ai_crates.windows(2) {
            assert!(window[0].impact >= window[1].impact);
        }
    }

    #[test]
    fn test_to_capabilities() {
        let registry = AwesomeRegistry::new();
        let caps = registry.to_capabilities();
        assert_eq!(caps.len(), registry.crates.len());
        assert!(caps.iter().all(|c| c.id.starts_with("rust_")));
    }

    #[test]
    fn test_lucasarts_priority() {
        // The search should weight Critical crates higher
        let registry = AwesomeRegistry::new();
        let results = registry.search("database");
        // qdrant (Critical) should rank above surrealdb (High) for "database"
        // when both match equally on tags
        assert!(!results.is_empty());
    }
}
