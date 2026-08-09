#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SovereignVerdict {
    RequiresHostedApproval,
    AutoDenied,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RohShard {
    pub roh_scalar: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LyapunovShard {
    pub v_prev: f32,
    pub v_next_pred: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EcoImpactShard {
    pub eco_score_prev: f32,
    pub eco_score_next_pred: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SovereigntyShard {
    pub consent_token_present: bool,
    pub neurorights_ok: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CharterInputs {
    pub roh: RohShard,
    pub lyap: LyapunovShard,
    pub eco: EcoImpactShard,
    pub sovereignty: SovereigntyShard,
}

pub const ROH_CEILING: f32 = 0.30;
pub const LYAP_TOLERANCE: f32 = 1e-5;

pub fn evaluate_charter_requirements(input: &CharterInputs) -> SovereignVerdict {
    if input.roh.roh_scalar > ROH_CEILING {
        return SovereignVerdict::AutoDenied;
    }

    if input.lyap.v_next_pred > input.lyap.v_prev + LYAP_TOLERANCE {
        return SovereignVerdict::AutoDenied;
    }

    if input.eco.eco_score_next_pred < input.eco.eco_score_prev {
        return SovereignVerdict::AutoDenied;
    }

    if !input.sovereignty.consent_token_present {
        return SovereignVerdict::AutoDenied;
    }

    if !input.sovereignty.neurorights_ok {
        return SovereignVerdict::AutoDenied;
    }

    SovereignVerdict::RequiresHostedApproval
}
