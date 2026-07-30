//! Styx route table — maps strand queries to namespace paths.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A route entry in the Styx namespace.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route {
    pub strand: String,
    pub path: String,
}

/// The Styx route table, loaded from 9P2000.L/styx/routes.json.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteTable {
    pub version: String,
    pub fixed_point: String,
    pub routes: HashMap<String, Route>,
}

impl RouteTable {
    /// Load from a JSON file path.
    pub fn load(path: &std::path::Path) -> anyhow::Result<Self> {
        let data = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&data)?)
    }

    /// Resolve a query key (e.g. "claude.query") to a namespace path.
    pub fn resolve(&self, key: &str) -> Option<&str> {
        self.routes.get(key).map(|r| r.path.as_str())
    }
}
