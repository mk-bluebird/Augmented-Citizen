#![cfg_attr(not(test), no_std)]
#![deny(warnings)]
#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

/// Fixed bindings to host identity and Bostrom address.
pub const HOST_DID: &str = "didalnorganic-host";
pub const BOSTROM_ADDRESS: &str =
    "bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnknownRegionOrganEvidence {
    pub organ_system_id: alloc::string::String,
    pub donor_count: u32,
    pub donor_fraction: f32,
    pub ph_pvalue: f32,
    pub ssm_cumulative_variance: f32,
    pub registration_tre_mm: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnknownRegionEvidenceV1 {
    pub evidence_id: alloc::string::String,
    pub host_did: alloc::string::String,
    pub bostrom_address: alloc::string::String,
    pub organ_evidence: alloc::vec::Vec<UnknownRegionOrganEvidence>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpgradeApprovalVcV1 {
    pub vc_id: alloc::string::String,
    pub proposal_id: alloc::string::String,
    pub evidence_id: alloc::string::String,
    pub host_did: alloc::string::String,
    pub bostrom_address: alloc::string::String,
    pub signers_did: alloc::vec::Vec<alloc::string::String>,
    pub neurorights_clause_ids: alloc::vec::Vec<alloc::string::String>,
    pub roh_ceiling_at_approval: f32,
    pub no_rollback_flag: bool,
    pub no_downgrade_flag: bool,
    pub monotone_tightening_only_flag: bool,
    pub approved_at_unix_days: u64,
}

/// Minimal view of a proposal for the guard.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnknownRegionUpgradeProposalV1 {
    pub proposal_id: alloc::string::String,
    pub evidence_id: alloc::string::String,
    pub host_did: alloc::string::String,
    pub bostrom_address: alloc::string::String,
    pub roh_ceiling_before: f32,
    pub roh_ceiling_after: f32,
}

/// Thresholds loaded from ALN shard for a given organ system.
#[derive(Debug, Clone, Copy)]
pub struct OrganThreshold {
    pub donor_min: u32,
    pub donor_fraction_min: f32,
    pub ph_pvalue_max: f32,
    pub ssm_cumvar_min: f32,
    pub tre_mm_max: f32,
}

/// Values pulled once from aln-cybernano-unknown-upgrade.v1.aln
#[derive(Debug, Clone, Copy)]
pub struct GlobalThresholds {
    pub roh_global_ceiling: f32,
    pub roh_unknown_corridor_ceiling: f32,
    pub vc_min_signers: u32,
    pub cooling_off_days_min: u64,
}

/// Guard verdict.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UpgradeVerdict {
    Admissible,
    RejectedIdentity,
    RejectedThresholds,
    RejectedRoHMonotonicity,
    RejectedGovernance,
    RejectedCoolingOff,
}

/// Core guard function: pure, side-effect-free, Kani-checkable.
pub fn evaluate_unknown_region_upgrade(
    proposal: &UnknownRegionUpgradeProposalV1,
    evidence: &UnknownRegionEvidenceV1,
    vc: &UpgradeApprovalVcV1,
    organ_thresholds: &[(&str, OrganThreshold)],
    global_thr: GlobalThresholds,
    current_unix_days: u64,
) -> UpgradeVerdict {
    // Identity binding.
    if proposal.host_did != HOST_DID
        || proposal.bostrom_address != BOSTROM_ADDRESS
        || evidence.host_did != HOST_DID
        || evidence.bostrom_address != BOSTROM_ADDRESS
        || vc.host_did != HOST_DID
        || vc.bostrom_address != BOSTROM_ADDRESS
        || proposal.evidence_id != evidence.evidence_id
        || proposal.proposal_id != vc.proposal_id
        || vc.evidence_id != evidence.evidence_id
    {
        return UpgradeVerdict::RejectedIdentity;
    }

    // Global RoH monotonicity: cannot raise ceilings above previous or above global.
    if proposal.roh_ceiling_after > proposal.roh_ceiling_before
        || proposal.roh_ceiling_after > global_thr.roh_global_ceiling
        || proposal.roh_ceiling_after > global_thr.roh_unknown_corridor_ceiling
        || vc.roh_ceiling_at_approval > global_thr.roh_global_ceiling
    {
        return UpgradeVerdict::RejectedRoHMonotonicity;
    }

    // Organ evidence thresholds.
    for oe in &evidence.organ_evidence {
        if let Some((_, thr)) = organ_thresholds
            .iter()
            .find(|(id, _)| *id == oe.organ_system_id.as_str())
        {
            if oe.donor_count < thr.donor_min
                || oe.donor_fraction < thr.donor_fraction_min
                || oe.ph_pvalue > thr.ph_pvalue_max
                || oe.ssm_cumulative_variance < thr.ssm_cumvar_min
                || oe.registration_tre_mm > thr.tre_mm_max
            {
                return UpgradeVerdict::RejectedThresholds;
            }
        } else {
            // Unknown organ threshold → reject by default.
            return UpgradeVerdict::RejectedThresholds;
        }
    }

    // Governance checks: host must sign, minimum quorum, neurorights must be present.
    let host_signed = vc.signers_did.iter().any(|d| d == HOST_DID);
    if !host_signed
        || vc.signers_did.len() < global_thr.vc_min_signers as usize
        || !vc.no_rollback_flag
        || !vc.no_downgrade_flag
        || !vc.monotone_tightening_only_flag
        || !vc
            .neurorights_clause_ids
            .iter()
            .any(|c| c == "neurorights.mental.integrity")
        || !vc
            .neurorights_clause_ids
            .iter()
            .any(|c| c == "neurorights.cognitive.liberty")
    {
        return UpgradeVerdict::RejectedGovernance;
    }

    // Cooling-off check.
    if current_unix_days.saturating_sub(vc.approved_at_unix_days)
        < global_thr.cooling_off_days_min
    {
        return UpgradeVerdict::RejectedCoolingOff;
    }

    UpgradeVerdict::Admissible
}
