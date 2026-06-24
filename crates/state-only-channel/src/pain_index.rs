// crates/state-only-channel/src/pain_index.rs

#![forbid(unsafe_code)]

use crate::state_only_vec::StateOnlyVec16;

/// PainIndexState encodes a normalized pain scalar in [0.0, 1.0]
/// plus a small set of quality scores derived from StateOnlyVec16.
///
/// This type is state-only: it is constructed exclusively from
/// BrainStateCommit / StateOnlyVec16 and never from EnclaveBrainSnapshot.
#[derive(Debug, Clone, Copy)]
pub struct PainIndexState {
    /// Normalized pain scalar in [0.0, 1.0].
    pain_index: f32,
    /// Quality score in [0.0, 1.0] for overall data fidelity.
    data_quality: f32,
    /// HRV-specific quality score in [0.0, 1.0].
    hrv_quality: f32,
    /// EEG-specific quality score in [0.0, 1.0].
    eeg_quality: f32,
}

impl PainIndexState {
    /// Construct a PainIndexState from a StateOnlyVec16 projection.
    ///
    /// The mapping must be registry-driven; here we assume:
    /// - One dimension encodes a normalized pain scalar.
    /// - Additional dimensions encode data quality and HRV/EEG quality indices.
    pub fn from_state_only_vec(vec: &StateOnlyVec16) -> Self {
        let pain_index = vec.get_axis_normalized("pain_index");
        let data_quality = vec.get_axis_normalized("data_quality");
        let hrv_quality = vec.get_axis_normalized("hrv_quality");
        let eeg_quality = vec.get_axis_normalized("eeg_quality");

        Self {
            pain_index: clamp01(pain_index),
            data_quality: clamp01(data_quality),
            hrv_quality: clamp01(hrv_quality),
            eeg_quality: clamp01(eeg_quality),
        }
    }

    pub fn pain_index(&self) -> f32 {
        self.pain_index
    }

    pub fn data_quality(&self) -> f32 {
        self.data_quality
    }

    pub fn hrv_quality(&self) -> f32 {
        self.hrv_quality
    }

    pub fn eeg_quality(&self) -> f32 {
        self.eeg_quality
    }

    /// Compute a federated weight from quality scores.
    ///
    /// This is a normalized scalar in [0.0, 1.0] suitable for use
    /// as w_i in P_global = Σ_i w_i · P_local_i.
    pub fn federated_weight(&self) -> f32 {
        let q = 0.5 * self.data_quality + 0.25 * self.hrv_quality + 0.25 * self.eeg_quality;
        clamp01(q)
    }
}

fn clamp01(x: f32) -> f32 {
    if x.is_nan() {
        0.0
    } else if x < 0.0 {
        0.0
    } else if x > 1.0 {
        1.0
    } else {
        x
    }
}
