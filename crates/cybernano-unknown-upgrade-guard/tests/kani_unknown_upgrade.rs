#![cfg(kani)]
use kani::any;
use cybernano_unknown_upgrade_guard::*;

#[kani::proof]
fn upgrade_never_raises_roh_ceiling() {
    let before: f32 = any();
    let after: f32 = any();
    let global: f32 = 0.30;
    let unk: f32 = 0.28;

    kani::assume(before >= 0.0 && before <= global);
    kani::assume(after >= 0.0 && after <= global);

    let proposal = UnknownRegionUpgradeProposalV1 {
        proposal_id: "p".into(),
        evidence_id: "e".into(),
        host_did: HOST_DID.into(),
        bostrom_address: BOSTROM_ADDRESS.into(),
        roh_ceiling_before: before,
        roh_ceiling_after: after,
    };

    let evidence = UnknownRegionEvidenceV1 {
        evidence_id: "e".into(),
        host_did: HOST_DID.into(),
        bostrom_address: BOSTROM_ADDRESS.into(),
        organ_evidence: alloc::vec::Vec::new(),
    };

    let vc = UpgradeApprovalVcV1 {
        vc_id: "vc".into(),
        proposal_id: "p".into(),
        evidence_id: "e".into(),
        host_did: HOST_DID.into(),
        bostrom_address: BOSTROM_ADDRESS.into(),
        signers_did: alloc::vec!["didalnorganic-host".into()],
        neurorights_clause_ids: alloc::vec![
            "neurorights.mental.integrity".into(),
            "neurorights.cognitive.liberty".into(),
        ],
        roh_ceiling_at_approval: global,
        no_rollback_flag: true,
        no_downgrade_flag: true,
        monotone_tightening_only_flag: true,
        approved_at_unix_days: 0,
    };

    let thr = GlobalThresholds {
        roh_global_ceiling: global,
        roh_unknown_corridor_ceiling: unk,
        vc_min_signers: 1,
        cooling_off_days_min: 0,
    };

    let verdict = evaluate_unknown_region_upgrade(
        &proposal,
        &evidence,
        &vc,
        &[],
        thr,
        10,
    );

    // If verdict is Admissible, then after must not exceed before or ceilings.
    if verdict == UpgradeVerdict::Admissible {
        assert!(after <= before);
        assert!(after <= global);
        assert!(after <= unk);
    }
}
