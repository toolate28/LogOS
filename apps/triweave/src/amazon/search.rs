//! Vectorize embedding search — queries Cloudflare Worker for product results.
//!
//! Calls the `reson8-edge` Worker's `/api/search-products` endpoint
//! which queries the `VECTORIZE_INDEX` (reson8-embeddings) binding.
//!
//! Conservation: α + ω = 15

use anyhow::{Context, Result};
use serde::Deserialize;

use super::hologram::SearchResult;

/// Default search endpoint (local Cloudflare Worker dev or production).
const SEARCH_ENDPOINT: &str = "https://reson8-edge.toolated.workers.dev/api/search-products";

/// Search products via Vectorize embedding similarity.
pub async fn search_products(query: &str, top_k: usize) -> Result<Vec<SearchResult>> {
    let client = reqwest::Client::new();
    let resp = client
        .get(SEARCH_ENDPOINT)
        .query(&[("q", query), ("k", &top_k.to_string())])
        .send()
        .await
        .context("failed to reach search endpoint")?;

    if !resp.status().is_success() {
        anyhow::bail!(
            "search endpoint returned {}: {}",
            resp.status(),
            resp.text().await.unwrap_or_default()
        );
    }

    let body: SearchResponse = resp
        .json()
        .await
        .context("failed to parse search response")?;

    Ok(body.products)
}

#[derive(Deserialize)]
struct SearchResponse {
    products: Vec<SearchResult>,
}
