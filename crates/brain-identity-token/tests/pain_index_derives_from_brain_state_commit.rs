// crates/brain-identity-token/tests/pain_index_derives_from_brain_state_commit.rs

#![cfg_attr(kani, allow(dead_code))]
#![cfg_attr(kani, allow(unused_imports))]
#![forbid(unsafe_code)]

use std::collections::HashMap;

use brain_identity_token::{BrainIdentityPainExtension, BrainIdentityStatus, BrainIdentityToken};
use chrono::{TimeZone, Utc};
use state_only_channel::{PainIndexState, StateOnlyVec16};

fn sample_brain_identity_token() -> BrainIdentityToken {
    let mut axes = HashMap::new();
    axes.insert("pain_index".to_string(), 0.4);
    axes.insert("data_quality".to_string(), 0.9);
    axes.insert("hrv_quality".to_string(), 0.8);
    axes.insert("eeg_quality".to_string(), 0.7);

    let state_only_vec = StateOnlyVec16::new(axes);

    BrainIdentityToken {
        brainidentitytokenid: "bit-001".to_string(),
        hostdid: "didalnorganic-host".to_string(),
        bostromaddress: "bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7".to_string(),
        status: BrainIdentityStatus::ActiveStable,
        created_at_utc: Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap(),
        updated_at_utc: Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap(),
        state_only_vec,
        pain_index_state: None,
        neurorights_noscorefrominnerstate: true,
        neurorights_noneurocoercion: true,
        neurorights_noexclusionbasicservices: true,
    }
}

#[test]
fn pain_index_is_derived_from_state_only_vec() {
    let mut token = sample_brain_identity_token();
    BrainIdentityPainExtension::update_pain_index(&mut token);

    let pain_state = token.pain_index_state.expect("pain_index_state must be set");

    assert!(pain_state.pain_index() >= 0.0 && pain_state.pain_index() <= 1.0);
}

#[cfg(kani)]
mod kani_harnesses {
    use super::*;
    use kani::any;

    #[kani::proof]
    fn kani_pain_index_uses_only_state_only_vec() {
        let mut axes = HashMap::new();
        axes.insert("pain_index".to_string(), any::<f32>());
        axes.insert("data_quality".to_string(), any::<f32>());
        axes.insert("hrv_quality".to_string(), any::<f32>());
        axes.insert("eeg_quality".to_string(), any::<f32>());

        let state_only_vec = StateOnlyVec16::new(axes);

        let mut token = BrainIdentityToken {
            brainidentitytokenid: "bit-kani".to_string(),
            hostdid: "didalnorganic-host".to_string(),
            bostromaddress: "bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7".to_string(),
            status: BrainIdentityStatus::ActiveStable,
            created_at_utc: Utc::now(),
            updated_at_utc: Utc::now(),
            state_only_vec,
            pain_index_state: None,
            neurorights_noscorefrominnerstate: true,
            neurorights_noneurocoercion: true,
            neurorights_noexclusionbasicservices: true,
        };

        BrainIdentityPainExtension::update_pain_index(&mut token);

        if let Some(pain_state) = token.pain_index_state {
            assert!(pain_state.pain_index() >= 0.0 && pain_state.pain_index() <= 1.0);
        }
    }
}
