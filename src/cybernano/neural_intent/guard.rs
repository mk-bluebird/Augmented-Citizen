use std::time::SystemTime;

use super::{ApprovedRequest, DenialReason, NeuralIntentToken, SovereignIntentFilter};
use crate::aln::NeuralSkillCatalogue;
use crate::evolution::EvolutionWindowClient;
use crate::psych::{PsychRiskInput, PsychRiskIndex};
use crate::roh::{RiskOfHarmKernel, RiskOfHarmScalar};
use crate::sovereign::NeurorightsProfile;
use crate::vault::VaultHandle;

/// Policy bundle injected from ALN at startup.
pub struct SovereignIntentPolicy {
    pub psych_soft_ceiling: f32,
    pub psych_hard_ceiling: f32,
    pub roh_global_ceiling: RiskOfHarmScalar,
}

/// Host-local guard state.
pub struct NeuralIntentGuard<'a> {
    policy: SovereignIntentPolicy,
    skills: &'a NeuralSkillCatalogue,
    roh_kernel: &'a RiskOfHarmKernel,
    evo_client: &'a EvolutionWindowClient,
    neurorights: &'a NeurorightsProfile,
    vault: &'a VaultHandle,
}

impl<'a> NeuralIntentGuard<'a> {
    pub fn new(
        policy: SovereignIntentPolicy,
        skills: &'a NeuralSkillCatalogue,
        roh_kernel: &'a RiskOfHarmKernel,
        evo_client: &'a EvolutionWindowClient,
        neurorights: &'a NeurorightsProfile,
        vault: &'a VaultHandle,
    ) -> Self {
        Self {
            policy,
            skills,
            roh_kernel,
            evo_client,
            neurorights,
            vault,
        }
    }

    fn current_psych_index(&self, input: &PsychRiskInput) -> PsychRiskIndex {
        self.roh_kernel.psych_index(input)
    }

    fn current_roh(&self, input: &PsychRiskInput) -> RiskOfHarmScalar {
        self.roh_kernel.estimate_roh(input)
    }

    fn check_neurorights(&self, token: &NeuralIntentToken) -> Result<(), DenialReason> {
        if !self.neurorights.permits_token(token.as_str()) {
            return Err(DenialReason::NeuroRightsViolation(
                "token not permitted by neurorights profile",
            ));
        }
        Ok(())
    }
}

impl<'a> SovereignIntentFilter for NeuralIntentGuard<'a> {
    fn filter_intent(
        &self,
        intent: NeuralIntentToken,
        psych_in: &PsychRiskInput,
    ) -> Result<ApprovedRequest, DenialReason> {
        let psych = self.current_psych_index(psych_in);
        if psych.value >= self.policy.psych_hard_ceiling {
            return Err(DenialReason::PsychRiskHardCeiling(
                "intent denied: psychrisk >= hard ceiling",
            ));
        }

        let roh = self.current_roh(psych_in);
        if roh.value > self.policy.roh_global_ceiling.value {
            return Err(DenialReason::RoHCeilingExceeded(
                "intent denied: RoH above global ceiling",
            ));
        }

        self.check_neurorights(&intent)?;

        let skill = self
            .skills
            .lookup(intent.as_str())
            .ok_or(DenialReason::SkillNotPermitted("token not in skill catalogue"))?;

        if psych.value >= self.policy.psych_soft_ceiling && skill.is_high_risk() {
            return Err(DenialReason::PsychRiskSoftCeiling(
                "high-risk neural skill blocked at or above psych soft ceiling",
            ));
        }

        if skill.requires_evolution_window() {
            let allowed = self
                .evo_client
                .check_neural_skill_window(&intent, psych_in, &roh);
            if !allowed {
                return Err(DenialReason::EvolutionWindowDenied(
                    "evolution window guard denied this neural skill request",
                ));
            }
        }

        let consent_sko = self.vault.current_consent_sko_for(intent.as_str())?;

        Ok(ApprovedRequest {
            token: intent,
            issued_at: SystemTime::now(),
            psych,
            roh,
            neurorights: self.neurorights.clone(),
            consent_sko,
        })
    }
}
