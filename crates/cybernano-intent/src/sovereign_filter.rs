// edition: 2024

use crate::intent_risk::{
    IntentStabilityError, IntentStabilityPolicy, IntentStabilityScore, NeuralSkillCatalogue,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PsychRiskInput {
    // existing psych-risk fields
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NeuralIntentToken {
    pub token_id: String,
    // other semantic fields...
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApprovedRequest {
    pub token: NeuralIntentToken,
    // other derived fields...
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum DenialReason {
    SintentBelowThreshold,
    IntentAlphabetTooRisky,
    PsychRiskViolation,
    RoHViolation,
    NeurorightsViolation,
    // ...
}

pub trait SovereignIntentFilter {
    fn filter_intent(
        &self,
        token: &NeuralIntentToken,
        psych: &PsychRiskInput,
    ) -> Result<ApprovedRequest, DenialReason>;
}

pub struct NeuralIntentGuard {
    catalogue: NeuralSkillCatalogue,
    policy: IntentStabilityPolicy,
    // other guard state: neurorights, RoH, etc.
}

impl NeuralIntentGuard {
    pub fn new(
        catalogue: NeuralSkillCatalogue,
        policy: IntentStabilityPolicy,
        // other deps...
    ) -> Self {
        Self {
            catalogue,
            policy,
        }
    }

    fn compute_stability(&self) -> Result<IntentStabilityScore, IntentStabilityError> {
        self.catalogue.compute_stability()
    }

    fn enforce_sintent(&self) -> Result<(), DenialReason> {
        let score = self.compute_stability().map_err(|_| DenialReason::IntentAlphabetTooRisky)?;
        self.policy
            .is_policy_compliant(&score)
            .map_err(|err| match err {
                IntentStabilityError::EmptyCatalogue => DenialReason::IntentAlphabetTooRisky,
                IntentStabilityError::SintentBelowMin { .. } => DenialReason::SintentBelowThreshold,
                IntentStabilityError::HighRiskFractionExceeded { .. } => {
                    DenialReason::IntentAlphabetTooRisky
                }
            })
    }
}

impl SovereignIntentFilter for NeuralIntentGuard {
    fn filter_intent(
        &self,
        token: &NeuralIntentToken,
        psych: &PsychRiskInput,
    ) -> Result<ApprovedRequest, DenialReason> {
        // 1. Sintent precondition: fail fast if vocabulary is unstable
        self.enforce_sintent()?;

        // 2. Existing neurorights, RoH, and psych-risk checks go here,
        //    reusing your current guard spine.

        // 3. If all guards pass, approve
        Ok(ApprovedRequest {
            token: token.clone(),
        })
    }
}
