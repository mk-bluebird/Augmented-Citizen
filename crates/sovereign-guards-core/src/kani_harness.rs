#![cfg(kani)]
#![forbid(unsafe_code)]

use crate::{
    evaluate_charter_requirements,
    CharterInputs,
    EcoImpactShard,
    LyapunovShard,
    RohShard,
    SovereignVerdict,
    SovereigntyShard,
};

fn valid_inputs() -> CharterInputs {
    CharterInputs {
        roh: RohShard { roh_scalar: 0.30 },
        lyap: LyapunovShard {
            v_prev: 0.40,
            v_next_pred: 0.40,
        },
        eco: EcoImpactShard {
            eco_score_prev: 0.60,
            eco_score_next_pred: 0.60,
        },
        sovereignty: SovereigntyShard {
            consent_token_present: true,
            neurorights_ok: true,
        },
    }
}

#[kani::proof]
fn valid_charter_inputs_require_hosted_approval() {
    let verdict = evaluate_charter_requirements(&valid_inputs());

    assert_eq!(verdict, SovereignVerdict::RequiresHostedApproval);
}

#[kani::proof]
fn risk_of_harm_above_ceiling_is_denied() {
    let mut input = valid_inputs();
    input.roh.roh_scalar = 0.31;

    let verdict = evaluate_charter_requirements(&input);

    assert_eq!(verdict, SovereignVerdict::AutoDenied);
}

#[kani::proof]
fn missing_consent_is_denied() {
    let mut input = valid_inputs();
    input.sovereignty.consent_token_present = false;

    let verdict = evaluate_charter_requirements(&input);

    assert_eq!(verdict, SovereignVerdict::AutoDenied);
}
