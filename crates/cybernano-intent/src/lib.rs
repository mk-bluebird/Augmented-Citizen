#![forbid(unsafe_code)]
#![cfg_attr(not(test), deny(warnings))]
#![doc = "CyberNano intent stability and sovereign intent filtering primitives."]

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Risk class for an intent token, aligned with ALN `risk_class` field.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RiskClass {
    Low,
    Medium,
    High,
}

/// Representation of a single token entry from `neural-skill-catalogue-v2.aln`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NeuralSkillToken {
    pub token_id: String,
    pub name: String,
    pub group: String,
    pub risk_class: RiskClass,
    pub max_roh_delta: f32,
    pub requires_consent: bool,
}

/// In‑memory view of the neural skill catalogue.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NeuralSkillCatalogue {
    pub tokens: Vec<NeuralSkillToken>,
}

/// Policy thresholds from `intent-stability-policy-v1.aln`.
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct IntentStabilityPolicy {
    /// Minimum acceptable Sintent value.
    pub s_intent_min: f32,
    /// Maximum allowed high-risk fraction.
    pub max_high_risk_fraction: f32,
    /// Weight for Sintent in eco-Lyapunov kernel.
    pub w_intent: f32,
    /// Minimum allowed Sintent delta at evolution time.
    pub s_intent_evolution_floor: f32,
}

/// Computed Sintent score for the current catalogue snapshot.
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct IntentStabilityScore {
    pub s_intent: f32,
    pub high_risk_fraction: f32,
    pub n_total: u32,
    pub n_high: u32,
}

#[derive(Debug, Error)]
pub enum IntentStabilityError {
    #[error("neural skill catalogue is empty")]
    EmptyCatalogue,
    #[error("policy violation: Sintent {s_intent:.3} < minimum {min:.3}")]
    SintentBelowMin { s_intent: f32, min: f32 },
    #[error(
        "policy violation: high-risk fraction {fraction:.3} > maximum {max:.3}"
    )]
    HighRiskFractionExceeded { fraction: f32, max: f32 },
}

impl NeuralSkillCatalogue {
    /// Compute Sintent according to:
    /// S_intent = 1 - n_high / n_total,
    /// where high-risk tokens include explicit High class and
    /// tokens with positive max_roh_delta or requires_consent=true.
    pub fn compute_stability(&self) -> Result<IntentStabilityScore, IntentStabilityError> {
        let n_total = self.tokens.len() as u32;
        if n_total == 0 {
            return Err(IntentStabilityError::EmptyCatalogue);
        }

        let n_high = self
            .tokens
            .iter()
            .filter(|t| match t.risk_class {
                RiskClass::High => true,
                RiskClass::Medium | RiskClass::Low => {
                    t.max_roh_delta > 0.0 || t.requires_consent
                }
            })
            .count() as u32;

        let high_risk_fraction = n_high as f32 / n_total as f32;
        let s_intent = 1.0 - high_risk_fraction;

        Ok(IntentStabilityScore {
            s_intent,
            high_risk_fraction,
            n_total,
            n_high,
        })
    }
}

impl IntentStabilityPolicy {
    /// Check that the given Sintent score satisfies the policy.
    pub fn is_policy_compliant(
        &self,
        score: &IntentStabilityScore,
    ) -> Result<(), IntentStabilityError> {
        if score.s_intent < self.s_intent_min {
            return Err(IntentStabilityError::SintentBelowMin {
                s_intent: score.s_intent,
                min: self.s_intent_min,
            });
        }

        if score.high_risk_fraction > self.max_high_risk_fraction {
            return Err(IntentStabilityError::HighRiskFractionExceeded {
                fraction: score.high_risk_fraction,
                max: self.max_high_risk_fraction,
            });
        }

        Ok(())
    }
}

/// Psych‑risk input stub; extend with your existing psych‑risk model.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PsychRiskInput {
    pub psych_risk_index: f32,
}

/// Decoded neural intent token, as produced by your decoder.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NeuralIntentToken {
    pub token_id: String,
    // Additional fields (e.g., semantic group, parameters) can be added here.
}

/// Approved, post‑guard request into the CyberNano core.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApprovedRequest {
    pub token: NeuralIntentToken,
    // Additional routing / actuation fields go here.
}

/// Reasons for denial at the SovereignIntentFilter chokepoint.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum DenialReason {
    SintentBelowThreshold,
    IntentAlphabetTooRisky,
    PsychRiskViolation,
    RoHViolation,
    NeurorightsViolation,
    Other(String),
}

/// Trait for the sovereign intent guard chokepoint.
pub trait SovereignIntentFilter {
    fn filter_intent(
        &self,
        token: &NeuralIntentToken,
        psych: &PsychRiskInput,
    ) -> Result<ApprovedRequest, DenialReason>;
}

/// Guard implementation that enforces Sintent as a precondition.
pub struct NeuralIntentGuard {
    catalogue: NeuralSkillCatalogue,
    policy: IntentStabilityPolicy,
    // Add RoH, neurorights, and other guard state here.
}

impl NeuralIntentGuard {
    pub fn new(
        catalogue: NeuralSkillCatalogue,
        policy: IntentStabilityPolicy,
    ) -> Self {
        Self { catalogue, policy }
    }

    fn compute_stability(&self) -> Result<IntentStabilityScore, IntentStabilityError> {
        self.catalogue.compute_stability()
    }

    fn enforce_sintent(&self) -> Result<(), DenialReason> {
        let score = self
            .compute_stability()
            .map_err(|_| DenialReason::IntentAlphabetTooRisky)?;

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
        _psych: &PsychRiskInput,
    ) -> Result<ApprovedRequest, DenialReason> {
        // 1. Sintent precondition: fail fast if vocabulary is unstable.
        self.enforce_sintent()?;

        // 2. TODO: plug in neurorights, RoH, and psych‑risk checks.

        // 3. Approve if all guards pass.
        Ok(ApprovedRequest {
            token: token.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sintent_computation_and_policy() {
        let tokens = vec![
            NeuralSkillToken {
                token_id: "t_safe".into(),
                name: "safe-intent".into(),
                group: "therapy".into(),
                risk_class: RiskClass::Low,
                max_roh_delta: 0.0,
                requires_consent: false,
            },
            NeuralSkillToken {
                token_id: "t_risky".into(),
                name: "risky-intent".into(),
                group: "coercive".into(),
                risk_class: RiskClass::High,
                max_roh_delta: 0.1,
                requires_consent: true,
            },
        ];
        let catalogue = NeuralSkillCatalogue { tokens };
        let score = catalogue.compute_stability().unwrap();
        assert_eq!(score.n_total, 2);
        assert_eq!(score.n_high, 1);
        assert!((score.s_intent - 0.5).abs() < 1e-6);

        let policy = IntentStabilityPolicy {
            s_intent_min: 0.4,
            max_high_risk_fraction: 0.75,
            w_intent: 0.1,
            s_intent_evolution_floor: 0.0,
        };
        policy.is_policy_compliant(&score).unwrap();
    }
}
