#![forbid(unsafe_code)]

pub mod charter_inputs;
pub mod lifeforce_ota_guard;

#[cfg(kani)]
mod kani_harness;

pub use charter_inputs::{
    evaluate_charter_requirements,
    CharterInputs,
    EcoImpactShard,
    LyapunovShard,
    RohShard,
    SovereignVerdict,
    SovereigntyShard,
};
pub use lifeforce_ota_guard::{
    evaluate_ota_eligibility,
    HostBioState,
    HostDutyState,
    LifeforceOtaThresholds,
    OtaEligibilityVerdict,
};
