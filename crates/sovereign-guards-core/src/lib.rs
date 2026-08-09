#![forbid(unsafe_code)]

pub mod charter_inputs;
pub mod lifeforce_ota_guard;

pub use charter_inputs::{
    evaluate_charter_requirements,
    CharterInputs,
    SovereignVerdict,
};
pub use lifeforce_ota_guard::{
    evaluate_ota_eligibility,
    HostBioState,
    HostDutyState,
    LifeforceOtaThresholds,
    OtaEligibilityVerdict,
};
