use crate::{BridgeError, BridgeCommand, BridgeEvent, BridgeResponse, WaveEngine, AtomTracker};
use crate::xai_client::XaiClient;

pub async fn route_and_braid(
    strand: &str,
    request: BridgeCommand,
    wave_engine: &WaveEngine,
    atom_tracker: &AtomTracker,
) -> Result<BridgeEvent, BridgeError> {
    match strand {
        "grok" => {
            let xai = XaiClient::new()?;
            let response = xai
                .send_with_braid(request.content, request.parent_knot, &wave_engine, &atom_tracker)
                .await?;

            Ok(BridgeEvent {
                from: "grok".into(),
                content: response.content,
                wave_score: response.wave_score,
                knot_id: response.knot_id,
            })
        }
        // ... other strands
        _ => Err(BridgeError::UnknownStrand(strand.to_string()))
    }
}
