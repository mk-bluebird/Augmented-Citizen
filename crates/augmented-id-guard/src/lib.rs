// filename: crates/augmented-id-guard/src/lib.rs
// edition: 2024
// rust-version = "1.85"

#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum AugmentedIdVerdict {
    AutoAllowed,
    AutoDenied,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AgeBand {
    Over13,
    Over16,
    Over18,
    Over21,
    Over25,
}

impl AgeBand {
    pub fn level(&self) -> u8 {
        match self {
            AgeBand::Over13 => 1,
            AgeBand::Over16 => 2,
            AgeBand::Over18 => 3,
            AgeBand::Over21 => 4,
            AgeBand::Over25 => 5,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AugmentedIdShard {
    pub credential_id: String,
    pub subject_did: String,
    pub issuer_did: String,
    pub age_band: AgeBand,
    pub expiry_timestamp: u64,
    pub revocation_state: RevocationState,
    pub exported_fields: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum RevocationState {
    Active,
    Revoked,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NeurorightsShard {
    pub telemetry_forbidden: Vec<String>,
    pub control_mode: ControlMode,
    pub host_did: String,
    pub bostrom_address: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ControlMode {
    HostLocal,
    Remote,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EcoShard {
    pub eco_score_prev: f32,
    pub eco_score_next: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AugmentedIdGuardInputs {
    pub requested_age_band: AgeBand,
    pub augmented_id: AugmentedIdShard,
    pub neurorights: NeurorightsShard,
    pub eco: EcoShard,
    pub now_timestamp: u64,
}

pub struct AugmentedIdGuard {
    trusted_issuers: Vec<String>,
    host_did: String,
    bostrom_address: String,
}

impl AugmentedIdGuard {
    pub fn new(trusted_issuers: Vec<String>, host_did: String, bostrom_address: String) -> Self {
        Self {
            trusted_issuers,
            host_did,
            bostrom_address,
        }
    }

    pub fn evaluate(&self, inputs: &AugmentedIdGuardInputs) -> AugmentedIdVerdict {
        if !self.age_band_ok(&inputs.requested_age_band, &inputs.augmented_id.age_band) {
            return AugmentedIdVerdict::AutoDenied;
        }

        if !self.issuer_trusted(&inputs.augmented_id.issuer_did) {
            return AugmentedIdVerdict::AutoDenied;
        }

        if !self.credential_fresh(
            inputs.now_timestamp,
            inputs.augmented_id.expiry_timestamp,
            &inputs.augmented_id.revocation_state,
        ) {
            return AugmentedIdVerdict::AutoDenied;
        }

        if !self.selective_disclosure_ok(
            &inputs.augmented_id.exported_fields,
            &inputs.neurorights.telemetry_forbidden,
        ) {
            return AugmentedIdVerdict::AutoDenied;
        }

        if !self.neurorights_safe(&inputs.augmented_id, &inputs.neurorights) {
            return AugmentedIdVerdict::AutoDenied;
        }

        if !self.eco_non_regression(&inputs.eco) {
            return AugmentedIdVerdict::AutoDenied;
        }

        AugmentedIdVerdict::AutoAllowed
    }

    fn age_band_ok(&self, requested: &AgeBand, credential: &AgeBand) -> bool {
        credential.level() >= requested.level()
    }

    fn issuer_trusted(&self, issuer_did: &str) -> bool {
        self.trusted_issuers.iter().any(|i| i == issuer_did)
    }

    fn credential_fresh(
        &self,
        now: u64,
        expiry: u64,
        state: &RevocationState,
    ) -> bool {
        let not_expired = now <= expiry;
        let not_revoked = matches!(state, RevocationState::Active);
        not_expired && not_revoked
    }

    fn selective_disclosure_ok(
        &self,
        exported: &[String],
        forbidden: &[String],
    ) -> bool {
        for field in exported {
            if forbidden.iter().any(|f| f == field) {
                return false;
            }
        }
        true
    }

    fn neurorights_safe(
        &self,
        shard: &AugmentedIdShard,
        neu: &NeurorightsShard,
    ) -> bool {
        if neu.host_did != self.host_did {
            return false;
        }

        if neu.bostrom_address != self.bostrom_address {
            return false;
        }

        match neu.control_mode {
            ControlMode::HostLocal => {}
            ControlMode::Remote => return false,
        }

        self.selective_disclosure_ok(
            &shard.exported_fields,
            &neu.telemetry_forbidden,
        )
    }

    fn eco_non_regression(&self, eco: &EcoShard) -> bool {
        eco.eco_score_next >= eco.eco_score_prev
    }
}
