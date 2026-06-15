// edition: 2024

use crate::evolution_window::{
    EvolutionWindowGuardState, EvolutionWindowPolicy, EvolutionWindowDecision,
};
use cybernano_intent::intent_risk::{IntentStabilityPolicy, IntentStabilityScore, NeuralSkillCatalogue};

#[derive(Debug, Clone)]
pub struct IntentEvolutionContext {
    pub catalogue_before: NeuralSkillCatalogue,
    pub catalogue_after: NeuralSkillCatalogue,
    pub policy: IntentStabilityPolicy,
}

impl IntentEvolutionContext {
    pub fn compute_scores(
        &self,
    ) -> (IntentStabilityScore, IntentStabilityScore) {
        let before = self
            .catalogue_before
            .compute_stability()
            .expect("catalogue_before non-empty");
        let after = self
            .catalogue_after
            .compute_stability()
            .expect("catalogue_after non-empty");
        (before, after)
    }

    pub fn enforce_non_reduction(
        &self,
        evolution_policy: &EvolutionWindowPolicy,
    ) -> Result<(), String> {
        let (before, after) = self.compute_scores();
        let delta_s = after.s_intent - before.s_intent;
        if delta_s < evolution_policy.s_intent_delta_min {
            return Err(format!(
                "Sintent non-reduction violated: ΔSintent={:.4} < min {:.4}",
                delta_s, evolution_policy.s_intent_delta_min
            ));
        }
        // Also enforce absolute floor from runtime policy
        self.policy
            .is_policy_compliant(&after)
            .map_err(|e| e.to_string())
    }
}
