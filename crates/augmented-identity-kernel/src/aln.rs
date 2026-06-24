// ALN shard schema for IdentityContinuityConfig bound to Augmented-Citizen host.

use crate::fixed::Fx;
use crate::distance::IdentitySigmaDiag;
use crate::continuity::{IdentityLyapunovWeights, IdentityContinuityConfig};

pub const IDENTITY_CONTINUITY_CONFIG_ID: &str = "IdentityContinuityConfig2026v1";

#[derive(Clone, Debug)]
pub struct IdentityContinuityShard {
    pub shard_id: &'static str,
    pub host_did: &'static str,
    pub bostrom_address: &'static str,
    pub eps_cont_sq: Fx,
    pub delta_v: Fx,
    pub sigma_inv_var: [Fx; super::state::ID_STATE_DIM],
    pub lyap_weights: [Fx; super::state::ID_STATE_DIM],
    pub aln_migration_clause: &'static str,
}

impl IdentityContinuityShard {
    pub fn default_for_augmented_citizen() -> Self {
        let cfg = IdentityContinuityConfig::strict_defaults();
        IdentityContinuityShard {
            shard_id: IDENTITY_CONTINUITY_CONFIG_ID,
            host_did: "did.cybercore.organic-host",
            bostrom_address: "bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7",
            eps_cont_sq: cfg.eps_cont_sq,
            delta_v: cfg.delta_v,
            sigma_inv_var: cfg.sigma.inv_var,
            lyap_weights: cfg.lyap_weights.w,
            aln_migration_clause: "ALN.MIGRATION.CYBERCORE_AUTHORITY.v1",
        }
    }

    pub fn to_runtime_config(&self) -> IdentityContinuityConfig {
        let sigma = IdentitySigmaDiag { inv_var: self.sigma_inv_var };
        let lyap = IdentityLyapunovWeights { w: self.lyap_weights };
        IdentityContinuityConfig {
            sigma,
            lyap_weights: lyap,
            eps_cont_sq: self.eps_cont_sq,
            delta_v: self.delta_v,
        }
    }

    pub fn verify_invariants(&self) -> bool {
        if self.shard_id != IDENTITY_CONTINUITY_CONFIG_ID {
            return false;
        }
        if self.bostrom_address != "bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7" {
            return false;
        }
        if self.aln_migration_clause != "ALN.MIGRATION.CYBERCORE_AUTHORITY.v1" {
            return false;
        }
        if self.eps_cont_sq.0 <= 0 {
            return false;
        }
        if self.delta_v.0 <= 0 {
            return false;
        }
        true
    }
}
