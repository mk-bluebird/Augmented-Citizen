//! Bounded Kani proofs for Skynet consent-scope validity.
//!
//! Run with:
//!
//! ```text
//! cargo kani --harness proof_expired_consent_scope_cannot_validate
//! cargo kani --harness proof_withdrawn_consent_scope_cannot_validate
//! cargo kani --harness proof_malformed_consent_scope_interval_is_rejected
//! ```

use skynet::{
    consent::{
        ConsentPurpose,
        ConsentScope,
        ConsentWithdrawal,
    },
    error::{
        ConsentScopeFailure,
        SkynetError,
        TemporalWindowKind,
        TemporalWindowViolation,
    },
    types::{
        ConsentScopeId,
        PolicyAuthorityReference,
        PolicyVersion,
        UtcTimestamp,
        VerifierReference,
    },
};

const CONSENT_SCOPE_ID: &str = "consent:scope-01";
const VERIFIER_REFERENCE: &str = "ver:approved-verifier-01";
const CONSENT_PURPOSE: &str = "purpose:infrastructure-access";
const POLICY_AUTHORITY: &str = "polauth:skynet-policy-authority";
const POLICY_VERSION: &str = "polver:v1.0";

fn scope_id() -> ConsentScopeId {
    ConsentScopeId::parse(CONSENT_SCOPE_ID)
        .expect("fixed consent scope identifier must be valid")
}

fn verifier_reference() -> VerifierReference {
    VerifierReference::parse(VERIFIER_REFERENCE)
        .expect("fixed verifier reference must be valid")
}

fn consent_purpose() -> ConsentPurpose {
    ConsentPurpose::parse(CONSENT_PURPOSE)
        .expect("fixed consent purpose must be valid")
}

fn policy_authority() -> PolicyAuthorityReference {
    PolicyAuthorityReference::parse(POLICY_AUTHORITY)
        .expect("fixed policy authority reference must be valid")
}

fn policy_version() -> PolicyVersion {
    PolicyVersion::parse(POLICY_VERSION)
        .expect("fixed policy version reference must be valid")
}

fn valid_scope(
    not_before: UtcTimestamp,
    expires_at: UtcTimestamp,
) -> ConsentScope {
    ConsentScope::new(
        scope_id(),
        verifier_reference(),
        consent_purpose(),
        policy_authority(),
        policy_version(),
        not_before,
        expires_at,
    )
    .expect("strictly increasing consent interval must construct")
}

fn validate_scope_at(
    scope: &ConsentScope,
    evaluation_time: UtcTimestamp,
) -> Result<(), SkynetError> {
    scope.validate_for(
        &scope_id(),
        &verifier_reference(),
        &consent_purpose(),
        &policy_authority(),
        &policy_version(),
        evaluation_time,
    )
}

#[kani::proof]
fn proof_expired_consent_scope_cannot_validate() {
    let start_offset: u8 = kani::any();
    let duration: u8 = kani::any();

    kani::assume(duration > 0);

    let not_before = UtcTimestamp::from_unix_seconds(i64::from(start_offset));
    let expires_at = UtcTimestamp::from_unix_seconds(
        i64::from(start_offset) + i64::from(duration),
    );
    let scope = valid_scope(not_before, expires_at);

    let result = validate_scope_at(&scope, expires_at);

    kani::assert(
        matches!(
            result,
            Err(SkynetError::InvalidTemporalWindow {
                kind: TemporalWindowKind::ConsentScope,
                violation: TemporalWindowViolation::Expired,
            })
        ),
        "a consent scope must not validate at or after expires_at",
    );
}

#[kani::proof]
fn proof_withdrawn_consent_scope_cannot_validate() {
    let start_offset: u8 = kani::any();
    let duration: u8 = kani::any();
    let withdrawal_offset: u8 = kani::any();

    kani::assume(duration > 0);

    let not_before = UtcTimestamp::from_unix_seconds(i64::from(start_offset));
    let expires_at = UtcTimestamp::from_unix_seconds(
        i64::from(start_offset) + i64::from(duration),
    );
    let evaluation_time = UtcTimestamp::from_unix_seconds(
        i64::from(start_offset) + i64::from(withdrawal_offset),
    );

    let withdrawal = ConsentWithdrawal::new(scope_id(), evaluation_time);
    let withdrawn_scope = valid_scope(not_before, expires_at)
        .withdraw(&withdrawal)
        .expect("withdrawal for the matching consent scope must succeed");

    let result = validate_scope_at(&withdrawn_scope, evaluation_time);

    kani::assert(
        matches!(
            result,
            Err(SkynetError::InvalidConsentScope {
                reason: ConsentScopeFailure::Withdrawn,
            })
        ),
        "a withdrawn consent scope must never validate",
    );
}

#[kani::proof]
fn proof_malformed_consent_scope_interval_is_rejected() {
    let first: u8 = kani::any();
    let second: u8 = kani::any();

    kani::assume(first >= second);

    let result = ConsentScope::new(
        scope_id(),
        verifier_reference(),
        consent_purpose(),
        policy_authority(),
        policy_version(),
        UtcTimestamp::from_unix_seconds(i64::from(first)),
        UtcTimestamp::from_unix_seconds(i64::from(second)),
    );

    kani::assert(
        matches!(
            result,
            Err(SkynetError::InvalidTemporalWindow {
                kind: TemporalWindowKind::ConsentScope,
                violation: TemporalWindowViolation::InvalidOrdering,
            })
        ),
        "a consent scope with not_before greater than or equal to expires_at must be rejected",
    );
}
