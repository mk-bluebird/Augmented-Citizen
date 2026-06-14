// filename: crates/detox-interval-guard/src/lib.rs
// edition: 2024
// rust-version = "1.85"

#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

use sovereign_guards_core::{CharterInputs, EcoImpactShard, LyapunovShard, RohShard, SovereignShard, SovereignVerdict, decide_charter_verdict}; // core AND-gate.[file:25]
use econet_healthcare_guard::{DefaultEconetHealthcareGuard, EconetHealthcareInputs, EvolutionProposal, evaluate_healthcare_evolution}; // econet limb.[file:21][file:25]

/// Typed nanoswarm and thermo shards from existing stack.[file:25]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NanoswarmComplianceFieldV1 {
    pub corridor_safe: bool,
    pub roh_scalar: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DraculaWaveThermoV1 {
    pub v_prev: f32,
    pub v_next_pred: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PersonalEcoShardV1 {
    pub eco_score_prev: f32,
    pub eco_score_next_pred: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DetoxIntervalPolicyV1 {
    pub requires_explicit_consent: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DetoxIntervalGuardInputs {
    pub nanoswarm: NanoswarmComplianceFieldV1,
    pub thermo: DraculaWaveThermoV1,
    pub eco: PersonalEcoShardV1,
    pub policy: DetoxIntervalPolicyV1,
    pub consent_present: bool,
    pub neurorights_ok: bool,
    pub econet: EconetHealthcareInputs,
}

/// DetoxIntervalGuard verdict piggybacks SovereignVerdict but may tighten behaviour.[file:25]
pub fn evaluate_detox_interval(
    econet_guard: &DefaultEconetHealthcareGuard,
    inputs: &DetoxIntervalGuardInputs,
) -> SovereignVerdict {
    // Corridor safety: detox only in pre-verified nanoswarm corridors.[file:25]
    if !inputs.nanoswarm.corridor_safe {
        return SovereignVerdict::AutoDenied;
    }

    // Build CharterInputs from nanoswarm / thermo / eco / consent shards.[file:25][file:24]
    let charter = CharterInputs {
        roh: RohShard {
            roh_scalar: inputs.nanoswarm.roh_scalar,
        },
        lyap: LyapunovShard {
            v_prev: inputs.thermo.v_prev,
            v_next_pred: inputs.thermo.v_next_pred,
        },
        eco: EcoImpactShard {
            eco_score_prev: inputs.eco.eco_score_prev,
            eco_score_next_pred: inputs.eco.eco_score_next_pred,
        },
        sov: SovereigntyShard {
            consent_token_present: inputs.consent_present,
            neurorights_ok: inputs.neurorights_ok,
        },
    };

    // Wrap into EvolutionProposal tagged as detox healthcare.[file:25]
    let proposal = EvolutionProposal {
        charter,
        econet: inputs.econet.clone(),
        domain_payload: serde_json::json!({
            "domain": "detox.interval",
            "policy": {
                "requires_explicit_consent": inputs.policy.requires_explicit_consent
            }
        }),
    };

    // Evaluate AND-gated invariants via econet healthcare bridge.[file:25][file:21]
    let healthcare_verdict = evaluate_healthcare_evolution(econet_guard, &proposal);

    match healthcare_verdict {
        HealthcareGuardVerdict::AutoDenied => SovereignVerdict::AutoDenied,
        HealthcareGuardVerdict::AutoAllowed => {
            // Respect detox policy: may downgrade to hosted approval even if core invariants pass.[file:25]
            if inputs.policy.requires_explicit_consent && !inputs.consent_present {
                SovereignVerdict::RequiresHostedApproval
            } else {
                SovereignVerdict::AutoAllowed
            }
        }
    }
}
