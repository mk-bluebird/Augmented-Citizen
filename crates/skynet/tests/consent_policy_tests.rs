//! Consent-binding regression tests.
//!
//! This file intentionally tests only policy-independent consent invariants.
//! It does not implement, assume, or simulate a P3 policy engine.
//!
//! When P3 is governance-adopted, add P3-specific decision and reason-code
//! assertions to this file without weakening these baseline tests.

use skynet::{
    consent::{
        ConsentPurpose,
        ConsentScope,
        ConsentWithdrawal,
    },
    error::{
        ConsentScopeFailure,
        PolicyLineageFailure,
        PolicyLineageField,
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
const APPROVED_VERIFIER: &str = "ver:approved-verifier-01";
const OTHER_VERIFIER: &str = "ver:other-verifier-01";
const APPROVED_PURPOSE: &str = "purpose:infrastructure-access";
const OTHER_PURPOSE: &str = "purpose:alternate-access";
const POLICY_AUTHORITY: &str = "polauth:skynet-policy-authority";
const OTHER_POLICY_AUTHORITY: &str = "polauth:other-policy-authority";
const POLICY_VERSION: &str = "polver:v1.0";
const OTHER_POLICY_VERSION: &str = "polver:v2.0";

const NOT_BEFORE: i64 = 1_700_000_000;
const EXPIRES_AT: i64 = 1_800_000_000;
const VALID_EVALUATION_TIME: i64 = 1_750_000_000;

fn consent_scope_id() -> ConsentScopeId {
    ConsentScopeId::parse(CONSENT_SCOPE_ID)
        .expect("test consent scope identifier must be valid")
}

fn approved_verifier() -> VerifierReference {
    VerifierReference::parse(APPROVED_VERIFIER)
        .expect("test approved verifier reference must be valid")
}

fn other_verifier() -> VerifierReference {
    VerifierReference::parse(OTHER_VERIFIER)
        .expect("test alternate verifier reference must be valid")
}

fn approved_purpose() -> ConsentPurpose {
    ConsentPurpose::parse(APPROVED_PURPOSE)
        .expect("test approved purpose must be valid")
}

fn other_purpose() -> ConsentPurpose {
    ConsentPurpose::parse(OTHER_PURPOSE)
        .expect("test alternate purpose must be valid")
}

fn policy_authority() -> PolicyAuthorityReference {
    PolicyAuthorityReference::parse(POLICY_AUTHORITY)
        .expect("test policy authority must be valid")
}

fn other_policy_authority() -> PolicyAuthorityReference {
    PolicyAuthorityReference::parse(OTHER_POLICY_AUTHORITY)
        .expect("test alternate policy authority must be valid")
}

fn policy_version() -> PolicyVersion {
    PolicyVersion::parse(POLICY_VERSION)
        .expect("test policy version must be valid")
}

fn other_policy_version() -> PolicyVersion {
    PolicyVersion::parse(OTHER_POLICY_VERSION)
        .expect("test alternate policy version must be valid")
}

fn valid_consent_scope() -> ConsentScope {
    ConsentScope::new(
        consent_scope_id(),
        approved_verifier(),
        approved_purpose(),
        policy_authority(),
        policy_version(),
        UtcTimestamp::from_unix_seconds(NOT_BEFORE),
        UtcTimestamp::from_unix_seconds(EXPIRES_AT),
    )
    .expect("valid test consent scope must construct")
}

fn valid_evaluation_time() -> UtcTimestamp {
    UtcTimestamp::from_unix_seconds(VALID_EVALUATION_TIME)
}

#[test]
fn active_consent_scope_validates_for_its_exact_bound_context() {
    let scope = valid_consent_scope();

    let result = scope.validate_for(
        &consent_scope_id(),
        &approved_verifier(),
        &approved_purpose(),
        &policy_authority(),
        &policy_version(),
        valid_evaluation_time(),
    );

    assert!(
        result.is_ok(),
        "active consent must validate only for its exact bound context"
    );
}

#[test]
fn consent_scope_rejects_a_different_verifier() {
    let scope = valid_consent_scope();

    let error = scope
        .validate_for(
            &consent_scope_id(),
            &other_verifier(),
            &approved_purpose(),
            &policy_authority(),
            &policy_version(),
            valid_evaluation_time(),
        )
        .expect_err("consent must not transfer across verifiers");

    assert_eq!(
        error,
        SkynetError::InvalidConsentScope {
            reason: ConsentScopeFailure::VerifierMismatch,
        }
    );
}

#[test]
fn consent_scope_rejects_a_different_purpose() {
    let scope = valid_consent_scope();

    let error = scope
        .validate_for(
            &consent_scope_id(),
            &approved_verifier(),
            &other_purpose(),
            &policy_authority(),
            &policy_version(),
            valid_evaluation_time(),
        )
        .expect_err("consent must not transfer across purposes");

    assert_eq!(
        error,
        SkynetError::InvalidConsentScope {
            reason: ConsentScopeFailure::PurposeMismatch,
        }
    );
}

#[test]
fn consent_scope_rejects_a_different_policy_authority() {
    let scope = valid_consent_scope();

    let error = scope
        .validate_for(
            &consent_scope_id(),
            &approved_verifier(),
            &approved_purpose(),
            &other_policy_authority(),
            &policy_version(),
            valid_evaluation_time(),
        )
        .expect_err("consent must not transfer across policy authorities");

    assert_eq!(
        error,
        SkynetError::PolicyLineageMismatch {
            field: PolicyLineageField::Authority,
            reason: PolicyLineageFailure::Mismatch,
        }
    );
}

#[test]
fn consent_scope_rejects_a_different_policy_version() {
    let scope = valid_consent_scope();

    let error = scope
        .validate_for(
            &consent_scope_id(),
            &approved_verifier(),
            &approved_purpose(),
            &policy_authority(),
            &other_policy_version(),
            valid_evaluation_time(),
        )
        .expect_err("consent must not transfer across policy versions");

    assert_eq!(
        error,
        SkynetError::PolicyLineageMismatch {
            field: PolicyLineageField::Version,
            reason: PolicyLineageFailure::Mismatch,
        }
    );
}

#[test]
fn consent_scope_rejects_evaluation_at_expiry_boundary() {
    let scope = valid_consent_scope();

    let error = scope
        .validate_for(
            &consent_scope_id(),
            &approved_verifier(),
            &approved_purpose(),
            &policy_authority(),
            &policy_version(),
            UtcTimestamp::from_unix_seconds(EXPIRES_AT),
        )
        .expect_err("consent must expire at the exclusive expiry boundary");

    assert_eq!(
        error,
        SkynetError::InvalidTemporalWindow {
            kind: TemporalWindowKind::ConsentScope,
            violation: TemporalWindowViolation::Expired,
        }
    );
}

#[test]
fn withdrawn_consent_scope_cannot_validate() {
    let withdrawal = ConsentWithdrawal::new(
        consent_scope_id(),
        valid_evaluation_time(),
    );

    let withdrawn_scope = valid_consent_scope()
        .withdraw(&withdrawal)
        .expect("matching scope withdrawal must succeed");

    let error = withdrawn_scope
        .validate_for(
            &consent_scope_id(),
            &approved_verifier(),
            &approved_purpose(),
            &policy_authority(),
            &policy_version(),
            valid_evaluation_time(),
        )
        .expect_err("withdrawn consent must never validate");

    assert_eq!(
        error,
        SkynetError::InvalidConsentScope {
            reason: ConsentScopeFailure::Withdrawn,
        }
    );
}

#[test]
fn withdrawal_for_a_different_scope_is_rejected() {
    let mismatched_withdrawal = ConsentWithdrawal::new(
        ConsentScopeId::parse("consent:other-scope-01")
            .expect("alternate consent scope identifier must be valid"),
        valid_evaluation_time(),
    );

    let error = valid_consent_scope()
        .withdraw(&mismatched_withdrawal)
        .expect_err("withdrawal must bind to the exact consent scope");

    assert_eq!(
        error,
        SkynetError::InvalidConsentScope {
            reason: ConsentScopeFailure::MissingBinding,
        }
    );
}
