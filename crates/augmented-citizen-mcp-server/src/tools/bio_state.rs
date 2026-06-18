use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::identity::host_binding::is_allowed_bostrom_address;
use crate::protocol::jsonrpc::JsonRpcResponse;

#[derive(Debug, Serialize, Deserialize)]
pub struct BioStateParams {
    pub host_did: String,
    pub bostrom_address: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BioState {
    pub blood: f64,
    pub sugar: f64,
    pub protein: f64,
    pub lifeforce: f64,
    pub oxygen: f64,
    pub brain: f64,
    pub wave: f64,
    pub dw: f64,
    pub pain: f64,
    pub fear: f64,
    pub timestamp: String,
}

pub fn handle_bio_read_state(
    id: Option<JsonValue>,
    params: JsonValue,
) -> JsonRpcResponse {
    let parsed: Result<BioStateParams, _> = serde_json::from_value(params);
    let params = match parsed {
        Ok(p) => p,
        Err(e) => {
            return JsonRpcResponse::error(
                id,
                -32602,
                format!("Invalid params: {}", e),
            );
        }
    };

    if !is_allowed_bostrom_address(&params.bostrom_address) {
        return JsonRpcResponse::error(
            id,
            -32001,
            "Unauthorized bostrom_address for this server",
        );
    }

    let timestamp = Utc::now().to_rfc3339();

    let state = BioState {
        blood: 1.0,
        sugar: 1.0,
        protein: 1.0,
        lifeforce: 1.0,
        oxygen: 1.0,
        brain: 1.0,
        wave: 1.0,
        dw: 1.0,
        pain: 0.0,
        fear: 0.0,
        timestamp,
    };

    JsonRpcResponse::success(id, serde_json::to_value(state).unwrap())
}
