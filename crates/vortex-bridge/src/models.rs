use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub id: Option<String>,
    pub method: String,
    pub params: Option<Value>,
    #[serde(rename = "atom_token", skip_serializing_if = "Option::is_none")]
    pub atom_token: Option<String>, // $ATOM_TOKEN_RESONANCE
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<Value>,
}

#[derive(Debug, Deserialize)]
pub struct ExecutePipelineParams {
    pub pipeline_name: String,
    pub steps: Vec<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StepCompleteParams {
    pub output: StepCompleteOutput,
    pub knot_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StepCompleteOutput {
    pub content: String,
    pub wave_score: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoherenceReportParams {
    pub wave_score: Option<f64>,
    pub curl: Option<f64>,
    pub divergence: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PipelineFailedParams {
    pub reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "method", content = "params")]
pub enum OutgoingMessage {
    #[serde(rename = "STEP_COMPLETE")]
    StepComplete(StepCompleteParams),

    #[serde(rename = "COHERENCE_REPORT")]
    CoherenceReport(CoherenceReportParams),

    #[serde(rename = "PIPELINE_FAILED")]
    PipelineFailed(PipelineFailedParams),
}
