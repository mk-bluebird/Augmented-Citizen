// crates/state-only-channel/tests/pain_index_state_only.rs

#![cfg_attr(kani, allow(dead_code))]
#![cfg_attr(kani, allow(unused_imports))]
#![forbid(unsafe_code)]

use std::collections::HashMap;

use state_only_channel::{PainIndexState, StateOnlyVec16};

fn arbitrary_state_only_vec() -> StateOnlyVec16 {
    let mut axes = HashMap::new();
    axes.insert("pain_index".to_string(), 0.5);
    axes.insert("data_quality".to_string(), 0.8);
    axes.insert("hrv_quality".to_string(), 0.7);
    axes.insert("eeg_quality".to_string(), 0.6);
    StateOnlyVec16::new(axes)
}

#[test]
fn pain_index_clamped_to_unit_interval() {
    let vec = arbitrary_state_only_vec();
    let pain_state = PainIndexState::from_state_only_vec(&vec);

    assert!(pain_state.pain_index() >= 0.0);
    assert!(pain_state.pain_index() <= 1.0);

    assert!(pain_state.data_quality() >= 0.0);
    assert!(pain_state.data_quality() <= 1.0);

    assert!(pain_state.hrv_quality() >= 0.0);
    assert!(pain_state.hrv_quality() <= 1.0);

    assert!(pain_state.eeg_quality() >= 0.0);
    assert!(pain_state.eeg_quality() <= 1.0);
}

#[cfg(kani)]
mod kani_harnesses {
    use super::*;
    use kani::any;

    #[kani::proof]
    fn kani_pain_index_from_state_only_vec_is_bounded() {
        let pain_index: f32 = any();
        let data_quality: f32 = any();
        let hrv_quality: f32 = any();
        let eeg_quality: f32 = any();

        let mut axes = HashMap::new();
        axes.insert("pain_index".to_string(), pain_index);
        axes.insert("data_quality".to_string(), data_quality);
        axes.insert("hrv_quality".to_string(), hrv_quality);
        axes.insert("eeg_quality".to_string(), eeg_quality);
        let vec = StateOnlyVec16::new(axes);

        let pain_state = PainIndexState::from_state_only_vec(&vec);

        assert!(pain_state.pain_index() >= 0.0 && pain_state.pain_index() <= 1.0);
        assert!(pain_state.data_quality() >= 0.0 && pain_state.data_quality() <= 1.0);
        assert!(pain_state.hrv_quality() >= 0.0 && pain_state.hrv_quality() <= 1.0);
        assert!(pain_state.eeg_quality() >= 0.0 && pain_state.eeg_quality() <= 1.0);
    }
}
