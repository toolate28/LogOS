use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

/// The Sapper-Grid API routes
pub fn sapper_routes() -> Router {
    Router::new()
        .route("/v1/sapper/backlog/active", get(get_active_backlog))
        .route("/v1/sapper/claim/atomic-unit", post(claim_atomic_unit))
        .route("/v1/sapper/submit/verify", post(submit_verify))
        .route("/v1/sapper/status/:username", get(sapper_status))
}

// ---------------------------------------------------------------------------
// 1. The Dispatcher: GET /backlog/active
// ---------------------------------------------------------------------------

#[derive(Serialize)]
pub struct BacklogResponse {
    pub lattice_state: String,
    pub priority_braids: Vec<PriorityBraid>,
}

#[derive(Serialize)]
pub struct PriorityBraid {
    pub task_id: String,
    pub repository: String,
    pub r#type: String,
    pub description: String,
    pub omega_bounty: f64,
    pub atomic_unit_weight: f64,
    pub status: String,
}

async fn get_active_backlog() -> impl IntoResponse {
    let response = BacklogResponse {
        lattice_state: "stable".to_string(),
        priority_braids: vec![
            PriorityBraid {
                task_id: "RES-089".to_string(),
                repository: "coherence-mcp".to_string(),
                r#type: "infrastructure_fix".to_string(),
                description: "Optimize WASM memory allocation for tool execution".to_string(),
                omega_bounty: 4.5,
                atomic_unit_weight: 1.0,
                status: "unclaimed".to_string(),
            },
            PriorityBraid {
                task_id: "RES-092".to_string(),
                repository: "quantum-redstone".to_string(),
                r#type: "feature_forge".to_string(),
                description: "Implement Fibonacci Anyon topological mapping for PR validation".to_string(),
                omega_bounty: 7.0,
                atomic_unit_weight: 2.5,
                status: "locked_by_sapper".to_string(),
            },
        ],
    };

    Json(response)
}

// ---------------------------------------------------------------------------
// 2. The Commitment: POST /claim/atomic-unit
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
pub struct ClaimRequest {
    pub task_id: String,
    pub github_username: String,
    pub sapper_public_key: String,
    pub estimated_alpha_cost: f64,
}

#[derive(Serialize)]
pub struct ClaimResponse {
    pub status: String,
    pub message: String,
}

async fn claim_atomic_unit(Json(payload): Json<ClaimRequest>) -> impl IntoResponse {
    // In a real implementation, register intent in the ledger
    let response = ClaimResponse {
        status: "locked".to_string(),
        message: format!("Task {} locked by {}", payload.task_id, payload.github_username),
    };

    (StatusCode::OK, Json(response))
}

// ---------------------------------------------------------------------------
// 3. The Crucible: POST /submit/verify
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
pub struct VerifyRequest {
    pub task_id: String,
    pub sapper: String,
    pub payload_hash: String,
    pub code_diff: String,
}

#[derive(Serialize)]
pub struct VerifyResponse {
    pub status: String,
    pub message: String,
    pub alpha_structural_complexity: u8,
    pub omega_semantic_value: u8,
    pub conservation_passed: bool,
}

async fn submit_verify(Json(_payload): Json<VerifyRequest>) -> impl IntoResponse {
    // Here we would route the code to the AI Triad and calculate alpha and omega.
    // For now, we simulate the validation.
    
    // Simulating evaluated metrics:
    let evaluated_alpha = 7;
    let evaluated_omega = 8;
    let passed = (evaluated_alpha + evaluated_omega) == crate::CONSERVATION_SUM; // 15

    let response = VerifyResponse {
        status: if passed { "verified".to_string() } else { "rejected".to_string() },
        message: if passed {
            "Payload merged and Atomic Unit assigned via NEAR smart contract.".to_string()
        } else {
            "Topological invariant α + ω = 15 failed.".to_string()
        },
        alpha_structural_complexity: evaluated_alpha,
        omega_semantic_value: evaluated_omega,
        conservation_passed: passed,
    };

    if passed {
        // Trigger NEAR smart contract locally
        (StatusCode::OK, Json(response))
    } else {
        (StatusCode::NOT_ACCEPTABLE, Json(response))
    }
}

// ---------------------------------------------------------------------------
// 4. The Ledger: GET /status/{username}
// ---------------------------------------------------------------------------

#[derive(Serialize)]
pub struct SapperStatusResponse {
    pub sapper: String,
    pub membership_status: String,
    pub atomic_units_ytd: f64,
    pub required_for_retention: f64,
    pub nexus_contributions: Vec<String>,
    pub larrikin_defiance_strikes: u32,
}

async fn sapper_status(Path(username): Path<String>) -> impl IntoResponse {
    let response = SapperStatusResponse {
        sapper: username,
        membership_status: "ACTIVE_COMMONWEALTH_PILLAR".to_string(),
        atomic_units_ytd: 6.5,
        required_for_retention: 5.0,
        nexus_contributions: vec![
            "coherence-mcp".to_string(),
            "SpiralSafe".to_string(),
            "HOPE-AI-NPC-SUITE".to_string(),
        ],
        larrikin_defiance_strikes: 0,
    };

    Json(response)
}
