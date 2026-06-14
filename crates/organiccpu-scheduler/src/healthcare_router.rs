// filename: crates/organiccpu-scheduler/src/healthcare_router.rs
// edition: 2024
// rust-version = "1.85"

#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

use detox_interval_guard::{DetoxIntervalGuardInputs, evaluate_detox_interval};
use econet_healthcare_guard::DefaultEconetHealthcareGuard;
use sovereign_guards_core::SovereignVerdict;

/// OrganicCPU scheduler entrypoint for healthcare EvolutionProposal.[file:25]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum HealthcareRoute {
    NanoswarmDetox,
    NanoswarmUvRepair,
    XrBackbone,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HealthcareEvolutionJob {
    pub route: HealthcareRoute,
    pub detox_inputs: Option<DetoxIntervalGuardInputs>,
    // UV repair / XR payloads would be added here as additional typed fields.
}

pub struct OrganicCpuScheduler {
    econet_guard: DefaultEconetHealthcareGuard,
}

impl OrganicCpuScheduler {
    pub fn new() -> Self {
        Self {
            econet_guard: DefaultEconetHealthcareGuard,
        }
    }

    /// All healthcare jobs must carry:
    ///   - CharterInputs (via domain-specific guard inputs)
    ///   - Econet shard (econet_healthcare.v1)
    ///   - Neurorights / eco shards (already encoded in guard inputs).[file:25][file:21]
    /// No job may reach nanoswarm or XR backends without passing these guards.[file:25]
    pub fn route_healthcare_job(&self, job: &HealthcareEvolutionJob) -> SovereignVerdict {
        match job.route {
            HealthcareRoute::NanoswarmDetox => {
                if let Some(inputs) = &job.detox_inputs {
                    evaluate_detox_interval(&self.econet_guard, inputs)
                } else {
                    SovereignVerdict::AutoDenied
                }
            }
            HealthcareRoute::NanoswarmUvRepair => {
                // TODO (explicitly disallowed by space rules) would be implemented as another
                // guard that builds CharterInputs + EconetHealthcareInputs and reuses
                // evaluate_healthcare_evolution exactly as detox does.[file:21][file:25]
                SovereignVerdict::AutoDenied
            }
            HealthcareRoute::XrBackbone => {
                // XR healthcare workloads must implement a similar bridge guard, not route directly.[file:25]
                SovereignVerdict::AutoDenied
            }
        }
    }
}
