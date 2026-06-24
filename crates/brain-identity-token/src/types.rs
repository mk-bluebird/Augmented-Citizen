// crates/brain-identity-token/src/types.rs

use chrono::{DateTime, Utc};

use state_only_channel::pain_index::PainIndexState;

/// Core BrainIdentityToken type.
///
/// This struct is extended with a PainIndexState that captures
/// the most recent state-only pain projection used for federated training.
#[derive(Debug, Clone)]
pub struct BrainIdentityToken {
    pub brainidentitytokenid: String,
    pub hostdid: String,
    pub bostromaddress: String,

    pub status: BrainIdentityStatus,
    pub created_at_utc: DateTime<Utc>,
    pub updated_at_utc: DateTime<Utc>,

    pub state_only_vec: state_only_channel::state_only_vec::StateOnlyVec16,
    pub pain_index_state: Option<PainIndexState>,

    pub neurorights_noscorefrominnerstate: bool,
    pub neurorights_noneurocoercion: bool,
    pub neurorights_noexclusionbasicservices: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BrainIdentityStatus {
    ActiveStable,
    ActiveElevatedRisk,
    Suspended,
    Revoked,
}

impl BrainIdentityToken {
    pub fn is_active_stable(&self) -> bool {
        matches!(self.status, BrainIdentityStatus::ActiveStable)
    }

    pub fn with_pain_index(mut self, pain_index_state: PainIndexState) -> Self {
        self.pain_index_state = Some(pain_index_state);
        self
    }
}
