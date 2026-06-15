// edition: 2024

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RiskClass {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NeuralSkillToken {
    pub token_id: String,
    pub name: String,
    pub group: String,
    pub risk_class: RiskClass,
    pub max_roh_delta: f32,
    pub requires_consent: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NeuralSkillCatalogue {
    pub tokens: Vec<NeuralSkillToken>,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct IntentStabilityPolicy {
    pub s_intent_min: f32,
    pub max_high_risk_fraction: f32,
    pub w_intent: f32,
    pub s_intent_evolution_floor: f32,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct IntentStabilityScore {
    pub s_intent: f32,
    pub high_risk_fraction: f32,
    pub n_total: u32,
    pub n_high: u32,
}

#[derive(Debug, Error)]
pub enum IntentStabilityError {
    #[error("no tokens available in neural skill catalogue")]
    EmptyCatalogue,
    #[error("policy violation: Sintent {s_intent:.3} < min {min:.3}")]
    SintentBelowMin { s_intent: f32, min: f32 },
    #[error(
        "policy violation: high-risk fraction {fraction:.3} > max {max:.3}"
    )]
    HighRiskFractionExceeded { fraction: f32, max: f32 },
}

impl NeuralSkillCatalogue {
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
