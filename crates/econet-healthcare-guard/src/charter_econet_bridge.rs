// filename: crates/econet-healthcare-guard/src/charter_econet_bridge.rs
// edition: 2024
// rust-version = "1.85"

#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

use sovereign_guards_core::{CharterInputs, EcoImpactShard, LyapunovShard, RohShard, SovereignShard, SovereignVerdict, decide_charter_verdict}; // from previous core crate.[file:25]
use crate::{EconetHealthcareGuardInputs, EconetHealthcareInputs, HealthcareGuardVerdict, DefaultEconetHealthcareGuard};

/// EvolutionProposal carries all shards needed for the AND-gate:
///   - CharterInputs (RoH, Lyapunov, EcoImpact, Sovereignty)
///   - Econet healthcare limb (earned-care corridor)
///   - Domain-specific shards (detox, XR, etc.).[file:25][file:21]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EvolutionProposal {
    /// Core sovereignty + eco invariants.
    pub charter: CharterInputs,

    /// Econet limb: eco contributions, tokens.[file:21]
    pub econet: EconetHealthcareInputs,

    /// Domain-specific detox / XR payload (opaque here).
    pub domain_payload: serde_json::Value,
}

/// Map CharterInputs + EconetHealthcareInputs into the existing econet guard inputs.[file:21][file:25]
fn to_econet_guard_inputs(proposal: &EvolutionProposal) -> EconetHealthcareGuardInputs {
    let sovereignty = crate::SovereigntyInputs {
        roh_scalar: proposal.charter.roh.roh_scalar,
        roh_ceiling: sovereign_guards_core::ROH_CEILING,
        consent_token: proposal.charter.sov.consent_token_present,
        neurorights_ok: proposal.charter.sov.neurorights_ok,
        evolution_interval_ok: true, // interval policy already enforced upstream.[file:24]
    };

    let eco = crate::EcoInputs {
        eco_score_prev: proposal.charter.eco.eco_score_prev,
        eco_score_next: proposal.charter.eco.eco_score_next_pred,
    };

    let econet = proposal.econet.clone();

    EconetHealthcareGuardInputs {
        sovereignty,
        eco,
        econet,
    }
}

/// Bridge verdict for healthcare EvolutionProposal:
///   - First, core charter invariants (CharterInputs) must pass.
///   - Second, econet healthcare limb must pass.
/// Only then may scheduler route to nanoswarm / XR backends.[file:25][file:21]
pub fn evaluate_healthcare_evolution(
    guard: &DefaultEconetHealthcareGuard,
    proposal: &EvolutionProposal,
) -> HealthcareGuardVerdict {
    // Core charter verdict: RoH, Lyapunov, eco, neurorights.[file:25][file:24]
    let charter_verdict = decide_charter_verdict(&proposal.charter);

    if charter_verdict != SovereignVerdict::AutoAllowed {
        return HealthcareGuardVerdict::AutoDenied;
    }

    // Econet limb verdict: earned-care corridor via eco/work/research.[file:21]
    let econet_inputs = to_econet_guard_inputs(proposal);
    guard.evaluate_econet_healthcare(&econet_inputs)
}
