use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::protocol::jsonrpc::JsonRpcResponse;
use crate::security::policies::{enforce_non_reversal, UpgradeModule};

#[derive(Debug, Serialize, Deserialize)]
pub struct PlanOtaParams {
    pub host_did: String,
    pub bostrom_address: String,
    pub target_window_start: String,
    pub target_window_end: String,
    pub modules: Vec<UpgradeModule>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlannedBundle {
    pub bundle_id: String,
    pub modules: Vec<UpgradeModule>,
    pub safety_checks: Vec<String>,
    pub requires_manual_approval: bool,
}

pub fn handle_upgrade_plan_ota_bundle(
    id: Option<JsonValue>,
    params: JsonValue,
) -> JsonRpcResponse {
    let parsed: Result<PlanOtaParams, _> = serde_json::from_value(params);
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

    let violations = enforce_non_reversal(&params.modules);
    if !violations.is_empty() {
        let msgs: Vec<String> = violations
            .into_iter()
            .map(|v| format!("{}: {}", v.code, v.message))
            .collect();

        return JsonRpcResponse::error(
            id,
            -32010,
            format!("Policy violations: {}", msgs.join("; ")),
        );
    }

    let bundle_id = format!(
        "bundle:{}:{}",
        params.host_did,
        params.target_window_start
    );

    let safety_checks = vec![
        "non_reversal_enforced".to_string(),
        "no_downgrade_modules_detected".to_string(),
    ];

    let bundle = PlannedBundle {
        bundle_id,
        modules: params.modules,
        safety_checks,
        requires_manual_approval: true,
    };

    JsonRpcResponse::success(id, serde_json::to_value(bundle).unwrap())
}
