// filename: crates/econet-healthcare-guard/src/lib.rs
#![forbid(unsafe_code)]
#![deny(clippy::all, clippy::pedantic)]

use serde::{Deserialize, Serialize};

/// Core verdicts reused from sovereign-guards-core.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SovereignVerdict {
    AutoAllowed,
    RequiresHostedApproval,
    AutoDenied,
}

/// Shards imported from sovereign-guards-core or adjacent crates.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RohShard {
    pub roh_scalar: f32, // 0.0..=1.0, hard ceiling 0.3
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
pub struct EconetContributionShard {
    /// Aggregate eco contribution from work/research over a rolling window.
    pub eco_contribution_score: f32,
    /// Daily eco budget remaining for healthcare actions.
    pub eco_budget_remaining_daily: f32,
    /// Responsibility-axis delta for this action (must be <= 0).
    pub delta_responsibility: f32,
    /// Count of valid, DID-bound capability tokens (EVOLVE, BLOOD, ECOHELP, RAF).
    pub capability_tokens_held: u32,
    /// True only if all tokens are DID-bound, non-transferable, and earned.
    pub capability_tokens_valid: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CharterInputs {
    pub roh: RohShard,
    pub eco: EcoImpactShard,
    pub sov: SovereigntyShard,
    pub econet: EconetContributionShard,
}

/// Hard charter thresholds (not relaxable by policy).
pub const ROH_CEILING: f32 = 0.30;
pub const HEALTHCARE_SOFT_CEILING: f32 = 0.25;

/// Healthcare-specific econet predicate:
/// - action must not exceed eco budget,
/// - responsibility delta must be <= 0 (non-regression),
/// - capability tokens must be DID-bound and earned,
/// - eco contribution score must be positive.
pub fn econet_healthcare_ok(input: &CharterInputs) -> bool {
    let econ = &input.econet;

    let eco_budget_ok = econ.eco_budget_remaining_daily >= 0.0;
    let responsibility_ok = econ.delta_responsibility <= 0.0;
    let tokens_ok = econ.capability_tokens_held > 0 && econ.capability_tokens_valid;
    let contribution_ok = econ.eco_contribution_score > 0.0;

    eco_budget_ok && responsibility_ok && tokens_ok && contribution_ok
}

/// Sovereignty corridor predicate:
/// - neurorights must be OK,
/// - consent token must be present.
pub fn sovereignty_corridor_ok(input: &CharterInputs) -> bool {
    let sov = &input.sov;
    sov.consent_token_present && sov.neurorights_ok
}

/// Eco non-regression predicate:
/// - EcoImpactScore_new >= EcoImpactScore_old.
pub fn eco_non_regression_ok(input: &CharterInputs) -> bool {
    input.eco.eco_score_next_pred >= input.eco.eco_score_prev
}

/// Core AND-gate for healthcare:
/// SovereigntyCorridorOK AND EcoNonRegressionOK AND EconetHealthcareOK AND RoH <= 0.30.
pub fn and_gate_healthcare(input: &CharterInputs) -> bool {
    let roh_ok = input.roh.roh_scalar <= ROH_CEILING;
    let sov_ok = sovereignty_corridor_ok(input);
    let eco_ok = eco_non_regression_ok(input);
    let econet_ok = econet_healthcare_ok(input);

    roh_ok && sov_ok && eco_ok && econet_ok
}

/// Rust guard trait that enforces the econet-healthcare AND-gate.
pub trait EconetHealthcareGuard {
    fn evaluate_healthcare_step(&self, input: &CharterInputs) -> SovereignVerdict;
}

/// Default implementation: host-local, nanoswarm-only guard.
pub struct HostLocalEconetHealthcareGuard;

impl EconetHealthcareGuard for HostLocalEconetHealthcareGuard {
    fn evaluate_healthcare_step(&self, input: &CharterInputs) -> SovereignVerdict {
        if !and_gate_healthcare(input) {
            // Any failure leads to automatic denial; external systems cannot override.
            return SovereignVerdict::AutoDenied;
        }

        // Within invariants: healthcare may be auto-allowed if corridor is maintenance/repair.
        // Higher-risk protocols can refine this to RequiresHostedApproval upstream.
        SovereignVerdict::AutoAllowed
    }
}
