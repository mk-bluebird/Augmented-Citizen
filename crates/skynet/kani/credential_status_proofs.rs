//! Bounded Kani proofs for the normalized Skynet credential-status model.
//!
//! This harness does not resolve status evidence, retrieve a status list,
//! process an accumulator, refresh a witness, contact an authority, or define
//! S6 freshness semantics.
//!
//! Run with:
//!
//! ```text
//! cargo kani --harness proof_only_active_satisfies_active_requirement
//! cargo kani --harness proof_expired_cannot_satisfy_active_requirement
//! cargo kani --harness proof_suspended_cannot_satisfy_active_requirement
//! cargo kani --harness proof_unavailable_cannot_satisfy_active_requirement
//! cargo kani --harness proof_unrecognized_cannot_satisfy_active_requirement
//! cargo kani --harness proof_all_closed_statuses_have_deterministic_active_rule
//! ```

use skynet::{
    invariants::{
        is_closed_credential_status,
        status_satisfies_active_requirement,
    },
    status::CredentialStatus,
};

fn status_from_bounded_index(index: u8) -> CredentialStatus {
    match index % 5 {
        0 => CredentialStatus::Active,
        1 => CredentialStatus::Expired,
        2 => CredentialStatus::Suspended,
        3 => CredentialStatus::Unavailable,
        _ => CredentialStatus::Unrecognized,
    }
}

#[kani::proof]
fn proof_only_active_satisfies_active_requirement() {
    kani::assert(
        status_satisfies_active_requirement(CredentialStatus::Active),
        "Active must satisfy an active-required policy rule",
    );
}

#[kani::proof]
fn proof_expired_cannot_satisfy_active_requirement() {
    kani::assert(
        !status_satisfies_active_requirement(CredentialStatus::Expired),
        "Expired must not satisfy an active-required policy rule",
    );
}

#[kani::proof]
fn proof_suspended_cannot_satisfy_active_requirement() {
    kani::assert(
        !status_satisfies_active_requirement(CredentialStatus::Suspended),
        "Suspended must not satisfy an active-required policy rule",
    );
}

#[kani::proof]
fn proof_unavailable_cannot_satisfy_active_requirement() {
    kani::assert(
        !status_satisfies_active_requirement(CredentialStatus::Unavailable),
        "Unavailable must not satisfy an active-required policy rule",
    );
}

#[kani::proof]
fn proof_unrecognized_cannot_satisfy_active_requirement() {
    kani::assert(
        !status_satisfies_active_requirement(CredentialStatus::Unrecognized),
        "Unrecognized must not satisfy an active-required policy rule",
    );
}

#[kani::proof]
fn proof_all_closed_statuses_have_deterministic_active_rule() {
    let bounded_index: u8 = kani::any();
    let status = status_from_bounded_index(bounded_index);

    kani::assert(
        is_closed_credential_status(status),
        "every generated status must belong to the closed Skynet status model",
    );

    let satisfies_active_requirement =
        status_satisfies_active_requirement(status);

    match status {
        CredentialStatus::Active => kani::assert(
            satisfies_active_requirement,
            "Active must satisfy the active-required rule",
        ),
        CredentialStatus::Expired
        | CredentialStatus::Suspended
        | CredentialStatus::Unavailable
        | CredentialStatus::Unrecognized => kani::assert(
            !satisfies_active_requirement,
            "every non-Active status must prevent active-required approval",
        ),
    }
}
