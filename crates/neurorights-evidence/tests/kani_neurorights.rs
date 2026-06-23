// filename: crates/neurorights-evidence/tests/kani_neurorights.rs
// License: MIT OR Apache-2.0
#![forbid(unsafe_code)]

use neurorights_evidence::{validate_envelope, NEURORIGHTS_ENVELOPE_CITIZEN_V1};

#[cfg(kani)]
mod kani_harness {
    use super::*;

    #[kani::proof]
    fn citizen_envelope_is_valid() {
        assert!(validate_envelope(&NEURORIGHTS_ENVELOPE_CITIZEN_V1));
    }
}
