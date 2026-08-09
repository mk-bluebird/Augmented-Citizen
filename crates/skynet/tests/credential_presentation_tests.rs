//! Format-neutral credential-presentation policy tests.
//!
//! These tests do not construct or parse credentials, CWTs, COSE objects,
//! presentations, proofs, wallets, status lists, accumulators, or transports.
//!
//! Full F6/H5 presentation tests remain blocked until the credential profile,
//! holder-authorization serialization, disclosure receipt schema, and negative
//! test matrix are formally adopted.

use skynet::{
    invariants::{
        is_closed_credential_status,
        status_satisfies_active_requirement,
    },
    status::CredentialStatus,
};

#[test]
fn unavailable_status_cannot_satisfy_an_active_required_policy_rule() {
    let status = CredentialStatus::Unavailable;

    assert!(!status.is_active());
    assert!(status.prevents_active_approval());
    assert!(!status_satisfies_active_requirement(status));
}

#[test]
fn unrecognized_status_cannot_satisfy_an_active_required_policy_rule() {
    let status = CredentialStatus::Unrecognized;

    assert!(!status.is_active());
    assert!(status.prevents_active_approval());
    assert!(!status_satisfies_active_requirement(status));
}

#[test]
fn expired_status_cannot_satisfy_an_active_required_policy_rule() {
    let status = CredentialStatus::Expired;

    assert!(!status.is_active());
    assert!(status.prevents_active_approval());
    assert!(!status_satisfies_active_requirement(status));
}

#[test]
fn suspended_status_cannot_satisfy_an_active_required_policy_rule() {
    let status = CredentialStatus::Suspended;

    assert!(!status.is_active());
    assert!(status.prevents_active_approval());
    assert!(!status_satisfies_active_requirement(status));
}

#[test]
fn only_active_status_satisfies_an_active_required_policy_rule() {
    let status = CredentialStatus::Active;

    assert!(status.is_active());
    assert!(!status.prevents_active_approval());
    assert!(status_satisfies_active_requirement(status));
}

#[test]
fn normalized_status_model_remains_closed() {
    let all_statuses = [
        CredentialStatus::Active,
        CredentialStatus::Expired,
        CredentialStatus::Suspended,
        CredentialStatus::Unavailable,
        CredentialStatus::Unrecognized,
    ];

    for status in all_statuses {
        assert!(
            is_closed_credential_status(status),
            "every supported status must belong to the closed status model"
        );
    }
}

#[test]
fn exactly_one_status_satisfies_an_active_required_policy_rule() {
    let all_statuses = [
        CredentialStatus::Active,
        CredentialStatus::Expired,
        CredentialStatus::Suspended,
        CredentialStatus::Unavailable,
        CredentialStatus::Unrecognized,
    ];

    let satisfying_status_count = all_statuses
        .into_iter()
        .filter(|status| status_satisfies_active_requirement(*status))
        .count();

    assert_eq!(
        satisfying_status_count,
        1,
        "only CredentialStatus::Active may satisfy an active-required policy rule"
    );
}

#[test]
fn non_active_statuses_are_not_lifecycle_approval_equivalents() {
    let non_active_statuses = [
        CredentialStatus::Expired,
        CredentialStatus::Suspended,
        CredentialStatus::Unavailable,
        CredentialStatus::Unrecognized,
    ];

    for status in non_active_statuses {
        assert!(
            status.prevents_active_approval(),
            "{status:?} must prevent an approval requiring Active status"
        );
    }
}
