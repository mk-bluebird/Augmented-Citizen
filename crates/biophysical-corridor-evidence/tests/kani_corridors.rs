// filename: crates/biophysical-corridor-evidence/tests/kani_corridors.rs
// License: MIT OR Apache-2.0
#![forbid(unsafe_code)]

use biophysical_corridor_evidence::{classify, CorridorState, BIOPHYSICAL_METRICS_DEFAULT};

#[cfg(kani)]
mod kani_harness {
    use super::*;

    #[kani::proof]
    fn spo2_never_inside_below_warn_min() {
        let spo2_metric = BIOPHYSICAL_METRICS_DEFAULT
            .iter()
            .find(|m| m.metricid == "spo2.percent")
            .unwrap();
        let v: f32 = kani::any();
        kani::assume(v < spo2_metric.bounds.warn_min);
        let state = classify(v, &spo2_metric.bounds);
        assert!(state != CorridorState::Inside);
    }
}
