// augmented-citizen-mcp-server/src/tools/consent.rs

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use anti_coercion_enclave::fusion::fuse_intent;
use anti_coercion_enclave::state_machine::ConsentVerdict;
use brain_identity_kernel::fixed::Fx;
use brain_identity_kernel::guard::KernelGuard;
use brain_identity_kernel::intent::{IntentScores, IntentWeights};
use brain_identity_kernel::kernel::ViabilityKernelState;

use crate::context::SessionContext;
use crate::protocol::jsonrpc::JsonRpcResponse;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConsentRefreshParams {
    pub host_did: String,
    pub imin: f32,
    pub i_brain: f32,
    pub i_bio: f32,
    pub alpha: f32,
    pub beta: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConsentRefreshResult {
    pub verdict: ConsentVerdict,
    pub combined_intent: Fx,
}

pub fn handle_consent_refresh(
    ctx: &mut SessionContext,
    kernel_guard: &KernelGuard<'_>,
    id: Option<JsonValue>,
    params: JsonValue,
) -> JsonRpcResponse {
    let parsed: Result<ConsentRefreshParams, _> = serde_json::from_value(params);
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

    // Here you could also recompute ViabilityKernelState from BCIIngressKernel.
    let dummy_state = ViabilityKernelState::new(
        Fx::from_f32(0.0),
        Fx::from_f32(0.0),
        Fx::from_f32(0.0),
        Fx::from_f32(0.0),
        Fx::from_f32(0.0),
        Fx::from_f32(0.0),
    )
    .unwrap();

    let inside = kernel_guard.is_inside_viability_kernel(&dummy_state);

    let alpha = Fx::from_f32(params.alpha);
    let beta = Fx::from_f32(params.beta);

    let weights = match IntentWeights::from_governance(alpha, beta) {
        Some(w) => w,
        None => {
            return JsonRpcResponse::error(
                id,
                -32020,
                "Invalid intent weights from governance shard",
            );
        }
    };

    let scores = IntentScores {
        brain: Fx::from_f32(params.i_brain),
        bio: Fx::from_f32(params.i_bio),
    };
    let imin = Fx::from_f32(params.imin);

    let fusion = fuse_intent(weights, scores, imin);

    let verdict = if !inside {
        ConsentVerdict::Invalid
    } else if !fusion.above_threshold {
        ConsentVerdict::CoercionSuspect
    } else {
        ConsentVerdict::Valid
    };

    ctx.update(verdict, fusion.combined_intent);

    let result = ConsentRefreshResult {
        verdict,
        combined_intent: fusion.combined_intent,
    };

    JsonRpcResponse::success(id, serde_json::to_value(result).unwrap())
}
