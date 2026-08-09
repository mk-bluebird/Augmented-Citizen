//! Bounded Kani proofs for format-neutral claim minimization.
//!
//! This harness proves the currently available evidence-model boundary. It does
//! not implement or assume a P3 policy engine, CWT parser, COSE verifier,
//! credential wallet, presentation builder, proof verifier, or transport.
//!
//! Run with:
//!
//! ```text
//! cargo kani --harness proof_disclosure_receipt_uses_only_minimized_inputs
//! cargo kani --harness proof_eligibility_evidence_uses_only_minimized_inputs
//! cargo kani --harness proof_profile_mismatch_is_rejected
//! cargo kani --harness proof_active_approved_evidence_requires_no_claim_values
//! cargo kani --harness proof_nonconforming_evidence_cannot_satisfy_active_requirement
//! ```

use skynet::{
    credential::{
        CredentialProfileIdentifier,
        CredentialProfileVersion,
        CredentialStatus,
        DisclosureConformance,
        DisclosureReceipt,
        EligibilityEvidence,
    },
    error::{
        CredentialProfileFailure,
        SkynetError,
    },
    types::DisclosureDescriptorSetId,
    SkynetResult,
};

const PROFILE_IDENTIFIER: &str = "cprof:skynet-minimized-evidence";
const PROFILE_VERSION: &str = "cver:v1.0";
const OTHER_PROFILE_VERSION: &str = "cver:v2.0";
const DISCLOSURE_DESCRIPTOR_SET: &str = "dset:approved-descriptors-v1";

type DisclosureReceiptConstructor = fn(
    CredentialProfileIdentifier,
    CredentialProfileVersion,
    DisclosureDescriptorSetId,
    DisclosureConformance,
) -> DisclosureReceipt;

type EligibilityEvidenceConstructor = fn(
    CredentialProfileIdentifier,
    CredentialProfileVersion,
    CredentialStatus,
    DisclosureReceipt,
) -> SkynetResult<EligibilityEvidence>;

fn profile_identifier() -> CredentialProfileIdentifier {
    CredentialProfileIdentifier::parse(PROFILE_IDENTIFIER)
        .expect("fixed credential profile identifier must be valid")
}

fn profile_version() -> CredentialProfileVersion {
    CredentialProfileVersion::parse(PROFILE_VERSION)
        .expect("fixed credential profile version must be valid")
}

fn other_profile_version() -> CredentialProfileVersion {
    CredentialProfileVersion::parse(OTHER_PROFILE_VERSION)
        .expect("fixed alternate credential profile version must be valid")
}

fn descriptor_set_id() -> DisclosureDescriptorSetId {
    DisclosureDescriptorSetId::parse(DISCLOSURE_DESCRIPTOR_SET)
        .expect("fixed disclosure descriptor-set identifier must be valid")
}

fn approved_disclosure_receipt() -> DisclosureReceipt {
    DisclosureReceipt::new(
        profile_identifier(),
        profile_version(),
        descriptor_set_id(),
        DisclosureConformance::WithinApprovedScope,
    )
}

#[kani::proof]
fn proof_disclosure_receipt_uses_only_minimized_inputs() {
    let constructor: DisclosureReceiptConstructor = DisclosureReceipt::new;

    let receipt = constructor(
        profile_identifier(),
        profile_version(),
        descriptor_set_id(),
        DisclosureConformance::WithinApprovedScope,
    );

    kani::assert(
        receipt.is_within_approved_scope(),
        "approved disclosure conformance must be represented without claim values",
    );

    kani::assert(
        receipt.descriptor_set_id().as_str() == DISCLOSURE_DESCRIPTOR_SET,
        "the receipt may retain only its opaque descriptor-set identifier",
    );
}

#[kani::proof]
fn proof_eligibility_evidence_uses_only_minimized_inputs() {
    let constructor: EligibilityEvidenceConstructor = EligibilityEvidence::new;

    let evidence = constructor(
        profile_identifier(),
        profile_version(),
        CredentialStatus::Active,
        approved_disclosure_receipt(),
    )
    .expect("matching minimized evidence must construct");

    kani::assert(
        evidence.status() == CredentialStatus::Active,
        "normalized status must remain a closed scalar result",
    );

    kani::assert(
        evidence
            .disclosure_receipt()
            .is_within_approved_scope(),
        "eligibility evidence must retain only disclosure conformance",
    );
}

#[kani::proof]
fn proof_profile_mismatch_is_rejected() {
    let receipt = DisclosureReceipt::new(
        profile_identifier(),
        other_profile_version(),
        descriptor_set_id(),
        DisclosureConformance::WithinApprovedScope,
    );

    let result = EligibilityEvidence::new(
        profile_identifier(),
        profile_version(),
        CredentialStatus::Active,
        receipt,
    );

    kani::assert(
        matches!(
            result,
            Err(SkynetError::UnsupportedCredentialProfile {
                reason: CredentialProfileFailure::IncompleteEvidence,
            })
        ),
        "profile-version mismatch must not create evidence eligible for P3 input",
    );
}

#[kani::proof]
fn proof_active_approved_evidence_requires_no_claim_values() {
    let evidence = EligibilityEvidence::new(
        profile_identifier(),
        profile_version(),
        CredentialStatus::Active,
        approved_disclosure_receipt(),
    )
    .expect("matching minimized evidence must construct");

    kani::assert(
        evidence.satisfies_active_minimized_evidence_requirement(),
        "active status plus approved descriptor conformance must be sufficient at the evidence layer",
    );
}

#[kani::proof]
fn proof_nonconforming_evidence_cannot_satisfy_active_requirement() {
    let receipt = DisclosureReceipt::new(
        profile_identifier(),
        profile_version(),
        descriptor_set_id(),
        DisclosureConformance::OutsideApprovedScope,
    );

    let evidence = EligibilityEvidence::new(
        profile_identifier(),
        profile_version(),
        CredentialStatus::Active,
        receipt,
    )
    .expect("nonconforming disclosure evidence remains representable for denial");

    kani::assert(
        !evidence.satisfies_active_minimized_evidence_requirement(),
        "active status cannot compensate for disclosure outside approved scope",
    );
}
