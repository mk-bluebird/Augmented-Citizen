// Rust 2024 edition, no unsafe, library-ready.
#![forbid(unsafe_code)]
#![cfg_attr(not(test), deny(warnings))]

use std::fmt;
use std::time::SystemTime;

use crate::psych::{PsychRiskInput, PsychRiskIndex};
use crate::roh::RiskOfHarmScalar;
use crate::sko::SkoId;
use crate::sovereign::NeurorightsProfile;
use crate::vault::VaultHandle;

/// Closed token set bound to aln.neural.intent_token_set.v1
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum NeuralIntentToken {
    Rest,
    LogHostile,
    AskState,
    SwitchModeConservative,
    SwitchModeNormal,
    SwitchModeExploratory,
    PauseDuty,
}

impl NeuralIntentToken {
    pub fn from_str(raw: &str) -> Option<Self> {
        match raw {
            "REST" => Some(Self::Rest),
            "LOG_HOSTILE" => Some(Self::LogHostile),
            "ASK_STATE" => Some(Self::AskState),
            "SWITCH_MODE(CONSERVATIVE)" => Some(Self::SwitchModeConservative),
            "SWITCH_MODE(NORMAL)" => Some(Self::SwitchModeNormal),
            "SWITCH_MODE(EXPLORATORY)" => Some(Self::SwitchModeExploratory),
            "PAUSE_DUTY" => Some(Self::PauseDuty),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Rest => "REST",
            Self::LogHostile => "LOG_HOSTILE",
            Self::AskState => "ASK_STATE",
            Self::SwitchModeConservative => "SWITCH_MODE(CONSERVATIVE)",
            Self::SwitchModeNormal => "SWITCH_MODE(NORMAL)",
            Self::SwitchModeExploratory => "SWITCH_MODE(EXPLORATORY)",
            Self::PauseDuty => "PAUSE_DUTY",
        }
    }
}

/// Canonical denial reasons for auditability.
#[derive(Clone, Debug)]
pub enum DenialReason {
    NeuroRightsViolation(&'static str),
    PsychRiskSoftCeiling(&'static str),
    PsychRiskHardCeiling(&'static str),
    RoHCeilingExceeded(&'static str),
    SkillNotPermitted(&'static str),
    EvolutionWindowDenied(&'static str),
    VaultAccessDenied(&'static str),
    UnknownToken(&'static str),
}

impl fmt::Display for DenialReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use DenialReason::*;
        match self {
            NeuroRightsViolation(m) => write!(f, "neurorights_violation: {}", m),
            PsychRiskSoftCeiling(m) => write!(f, "psychrisk_soft_ceiling: {}", m),
            PsychRiskHardCeiling(m) => write!(f, "psychrisk_hard_ceiling: {}", m),
            RoHCeilingExceeded(m) => write!(f, "roh_ceiling_exceeded: {}", m),
            SkillNotPermitted(m) => write!(f, "skill_not_permitted: {}", m),
            EvolutionWindowDenied(m) => write!(f, "evolution_window_denied: {}", m),
            VaultAccessDenied(m) => write!(f, "vault_access_denied: {}", m),
            UnknownToken(m) => write!(f, "unknown_token: {}", m),
        }
    }
}

/// The only shape that may escape the neural boundary.
#[derive(Clone, Debug)]
pub struct ApprovedRequest {
    pub token: NeuralIntentToken,
    pub issued_at: SystemTime,
    pub psych: PsychRiskIndex,
    pub roh: RiskOfHarmScalar,
    pub neurorights: NeurorightsProfile,
    pub consent_sko: Option<SkoId>,
}

/// Guard trait: deterministic mapping f: T × R → A ∪ {DENY}.
pub trait SovereignIntentFilter {
    fn filter_intent(
        &self,
        intent: NeuralIntentToken,
        psych: &PsychRiskInput,
    ) -> Result<ApprovedRequest, DenialReason>;
}
