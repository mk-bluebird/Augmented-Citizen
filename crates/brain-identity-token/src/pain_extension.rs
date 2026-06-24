// crates/brain-identity-token/src/pain_extension.rs

use chrono::Utc;

use crate::{BrainIdentityStatus, BrainIdentityToken};
use state_only_channel::pain_index::PainIndexState;

/// BrainIdentityPainExtension provides state-only pain index updates
/// bound to BrainIdentityToken and neurorights invariants.
pub struct BrainIdentityPainExtension;

impl BrainIdentityPainExtension {
    /// Update the BrainIdentityToken with a new PainIndexState derived
    /// from StateOnlyVec16. This function must be called only after
    /// state_only_vec has been refreshed from BrainStateCommit.
    pub fn update_pain_index(token: &mut BrainIdentityToken) {
        if !token.is_active_stable() {
            return;
        }

        if !token.neurorights_noscorefrominnerstate
            || !token.neurorights_noneurocoercion
            || !token.neurorights_noexclusionbasicservices
        {
            return;
        }

        let state_only_vec = token.state_only_vec.clone();
        let pain_index_state = PainIndexState::from_state_only_vec(&state_only_vec);

        token.pain_index_state = Some(pain_index_state);
        token.updated_at_utc = Utc::now();
    }
}
