// filename: crates/sovereign-guards-core/src/charter_inputs.rs
// edition: 2024
// rust-version = "1.85"

#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

/// Unified verdict returned by all host-local guards.[file:25]
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SovereignVerdict {
    AutoAllowed,
    RequiresHostedApproval,
    AutoDenied,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RohShard {
    pub roh_scalar: f32, // RoH(state_vector), hard ceiling 0.3.[file:25]
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LyapunovShard {
    pub v_prev: f32,      // V(t).[file:25]
    pub v_next_pred: f32, // predicted V(t+1).[file:25]
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EcoImpactShard {
    pub eco_score_prev: f32,
    pub eco_score_next_pred: f32,
}

#[derive(Clone, Debug, Serialize, Serialize, Deserialize)]
pub struct SovereigntyShard {
    pub consent_token_present: bool,
    pub neurorights_ok: bool, // cognitive liberty, mental privacy, continuity.[file:24]
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CharterInputs {
    pub roh: RohShard,
    pub lyap: LyapunovShard,
    pub eco: EcoImpactShard,
    pub sov: SovereigntyShard,
}

pub const ROH_CEILING: f32 = 0.30;
pub const LYAP_TOLERANCE: f32 = 1e-5;

/// Core AND-gate: RoH ≤ 0.3, Lyapunov non-increase, eco-monotone, neurorights + consent.[file:25][file:24]
pub fn and_gate_satisfied(input: &CharterInputs) -> bool {
    let roh_ok = input.roh.roh_scalar <= ROH_CEILING;
    let lyap_ok = input.lyap.v_next_pred <= input.lyap.v_prev + LYAP_TOLERANCE;
    let eco_ok = input.eco.eco_score_next_pred >= input.eco.eco_score_prev;
    let sov_ok = input.sov.consent_token_present && input.sov.neurorights_ok;

    roh_ok && lyap_ok && eco_ok && sov_ok
}

/// Default verdict mapping for core charter invariants.[file:25]
pub fn decide_charter_verdict(input: &CharterInputs) -> SovereignVerdict {
    if !and_gate_satisfied(input) {
        SovereignVerdict::AutoDenied
    } else {
        SovereignVerdict::AutoAllowed
    }
}
