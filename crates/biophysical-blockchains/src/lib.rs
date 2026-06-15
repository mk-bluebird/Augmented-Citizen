#![forbid(unsafe_code)]
#![cfg_attr(not(test), deny(warnings))]
#![doc = "Biophysical blockchain guards, evolution windows, and Sintent-aware eco-invariants."]

use cybernano_intent::{IntentStabilityPolicy, IntentStabilityScore, NeuralSkillCatalogue};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Evolution window policy, extended with Sintent constraints.
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct EvolutionWindowPolicy {
    /// Minimum allowed Sintent delta (typically 0 for non-reduction).
    pub s_intent_delta_min: f32,
    /// Other evolution parameters (KF, RoH ceilings, etc.) go here.
}

/// Captures catalogues and policy around a proposed evolution step.
#[derive(Debug, Clone)]
pub struct IntentEvolutionContext {
    pub catalogue_before: NeuralSkillCatalogue,
    pub catalogue_after: NeuralSkillCatalogue,
    pub policy: IntentStabilityPolicy,
}

#[derive(Debug, Error)]
pub enum EvolutionError {
    #[error("Sintent non-reduction violated: ΔSintent={delta_s:.4} < min {min:.4}")]
    SintentDeltaViolation { delta_s: f32, min: f32 },
    #[error("Sintent policy violation after evolution: {0}")]
    SintentPolicyViolation(String),
}

/// Decision outcome for an evolution window evaluation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvolutionWindowDecision {
    Allow,
    Deny,
}

impl IntentEvolutionContext {
    pub fn compute_scores(
        &self,
    ) -> (IntentStabilityScore, IntentStabilityScore) {
        let before = self
            .catalogue_before
            .compute_stability()
            .expect("catalogue_before must be non-empty");
        let after = self
            .catalogue_after
            .compute_stability()
            .expect("catalogue_after must be non-empty");
        (before, after)
    }

    /// Enforce Sintent non-reduction and absolute floor for a proposed evolution.
    pub fn enforce_non_reduction(
        &self,
        evolution_policy: &EvolutionWindowPolicy,
    ) -> Result<(), EvolutionError> {
        let (before, after) = self.compute_scores();
        let delta_s = after.s_intent - before.s_intent;

        if delta_s < evolution_policy.s_intent_delta_min {
            return Err(EvolutionError::SintentDeltaViolation {
                delta_s,
                min: evolution_policy.s_intent_delta_min,
            });
        }

        self.policy
            .is_policy_compliant(&after)
            .map_err(|e| EvolutionError::SintentPolicyViolation(e.to_string()))
    }
}

/// Evaluation entry point used by your EvolutionWindowGuard.
pub fn evaluate_intent_evolution(
    ctx: &IntentEvolutionContext,
    policy: &EvolutionWindowPolicy,
) -> EvolutionWindowDecision {
    match ctx.enforce_non_reduction(policy) {
        Ok(()) => EvolutionWindowDecision::Allow,
        Err(_) => EvolutionWindowDecision::Deny,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cybernano_intent::{NeuralSkillToken, RiskClass};

    fn make_token(id: &str, risk: RiskClass) -> NeuralSkillToken {
        NeuralSkillToken {
            token_id: id.to_string(),
            name: id.to_string(),
            group: "test".to_string(),
            risk_class: risk,
            max_roh_delta: match risk {
                RiskClass::High => 0.1,
                RiskClass::Medium => 0.02,
                RiskClass::Low => 0.0,
            },
            requires_consent: matches!(risk, RiskClass::High | RiskClass::Medium),
        }
    }

    #[test]
    fn evolution_non_reduction_passes_when_sintent_increases() {
        let before = NeuralSkillCatalogue {
            tokens: vec![make_token("t1", RiskClass::High)],
        };
        let after = NeuralSkillCatalogue {
            tokens: vec![
                make_token("t1", RiskClass::High),
                make_token("t2", RiskClass::Low),
            ],
        };

        let sintent_policy = IntentStabilityPolicy {
            s_intent_min: 0.4,
            max_high_risk_fraction: 0.75,
            w_intent: 0.1,
            s_intent_evolution_floor: 0.0,
        };
        let evo_policy = EvolutionWindowPolicy {
            s_intent_delta_min: 0.0,
        };

        let ctx = IntentEvolutionContext {
            catalogue_before: before,
            catalogue_after: after,
            policy: sintent_policy,
        };

        let decision = evaluate_intent_evolution(&ctx, &evo_policy);
        assert_eq!(decision, EvolutionWindowDecision::Allow);
    }

    #[test]
    fn evolution_non_reduction_denies_when_sintent_drops() {
        let before = NeuralSkillCatalogue {
            tokens: vec![make_token("t1", RiskClass::Low)],
        };
        let after = NeuralSkillCatalogue {
            tokens: vec![
                make_token("t1", RiskClass::Low),
                make_token("t2", RiskClass::High),
            ],
        };

        let sintent_policy = IntentStabilityPolicy {
            s_intent_min: 0.0,
            max_high_risk_fraction: 1.0,
            w_intent: 0.1,
            s_intent_evolution_floor: 0.0,
        };
        let evo_policy = EvolutionWindowPolicy {
            s_intent_delta_min: 0.0,
        };

        let ctx = IntentEvolutionContext {
            catalogue_before: before,
            catalogue_after: after,
            policy: sintent_policy,
        };

        let decision = evaluate_intent_evolution(&ctx, &evo_policy);
        assert_eq!(decision, EvolutionWindowDecision::Deny);
    }
}
