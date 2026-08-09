//! Bounded Kani proofs for opaque Skynet identity-reference construction.
//!
//! Run with:
//!
//! ```text
//! cargo kani --harness proof_valid_crockford_token_is_accepted
//! cargo kani --harness proof_invalid_prefix_is_rejected
//! cargo kani --harness proof_invalid_token_character_is_rejected
//! cargo kani --harness proof_invalid_token_length_is_rejected
//! cargo kani --harness proof_reference_larger_than_global_bound_is_rejected
//! ```

use skynet::{
    error::{
        OpaqueReferenceKind,
        OpaqueReferenceViolation,
        SkynetError,
    },
    identity::{
        construct_identity_reference,
        validate_identity_reference,
    },
    types::CitizenIdentityReference,
};

const IDENTITY_PREFIX: &str = "cit:";
const IDENTITY_TOKEN_LENGTH: usize = 26;
const MAX_REFERENCE_LENGTH: usize = 128;
const CROCKFORD_BASE32: &[u8; 32] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";

fn valid_token_from_indices(indices: [u8; IDENTITY_TOKEN_LENGTH]) -> String {
    let mut token = String::with_capacity(IDENTITY_TOKEN_LENGTH);

    for index in indices {
        token.push(char::from(CROCKFORD_BASE32[usize::from(index % 32)]));
    }

    token
}

fn valid_reference_from_indices(indices: [u8; IDENTITY_TOKEN_LENGTH]) -> String {
    let token = valid_token_from_indices(indices);
    format!("{IDENTITY_PREFIX}{token}")
}

fn invalid_character_reference_at(
    indices: [u8; IDENTITY_TOKEN_LENGTH],
    invalid_position: usize,
) -> String {
    let mut token = valid_token_from_indices(indices);

    token.replace_range(invalid_position..invalid_position + 1, "!");

    format!("{IDENTITY_PREFIX}{token}")
}

#[kani::proof]
fn proof_valid_crockford_token_is_accepted() {
    let indices: [u8; IDENTITY_TOKEN_LENGTH] = kani::any();
    let reference_text = valid_reference_from_indices(indices);

    let reference = construct_identity_reference(&reference_text)
        .expect("every generated Crockford Base32 token must be accepted");

    kani::assert(
        reference.as_str() == reference_text,
        "validated identity reference must preserve its opaque representation",
    );

    kani::assert(
        validate_identity_reference(&reference).is_ok(),
        "constructed identity reference must satisfy the identity contract",
    );
}

#[kani::proof]
fn proof_invalid_prefix_is_rejected() {
    let indices: [u8; IDENTITY_TOKEN_LENGTH] = kani::any();
    let token = valid_token_from_indices(indices);
    let reference_text = format!("id:{token}");

    let result = construct_identity_reference(&reference_text);

    kani::assert(
        matches!(
            result,
            Err(SkynetError::InvalidOpaqueReference {
                kind: OpaqueReferenceKind::CitizenIdentity,
                violation: OpaqueReferenceViolation::InvalidPrefix,
            })
        ),
        "identity references without the cit: prefix must be rejected",
    );
}

#[kani::proof]
fn proof_invalid_token_character_is_rejected() {
    let indices: [u8; IDENTITY_TOKEN_LENGTH] = kani::any();
    let invalid_position: usize = kani::any();

    kani::assume(invalid_position < IDENTITY_TOKEN_LENGTH);

    let reference_text =
        invalid_character_reference_at(indices, invalid_position);

    let result = construct_identity_reference(&reference_text);

    kani::assert(
        matches!(
            result,
            Err(SkynetError::InvalidOpaqueReference {
                kind: OpaqueReferenceKind::CitizenIdentity,
                violation: OpaqueReferenceViolation::InvalidCharacter,
            })
        ),
        "a token containing a non-Crockford character must be rejected",
    );
}

#[kani::proof]
fn proof_invalid_token_length_is_rejected() {
    let short_reference = format!(
        "{IDENTITY_PREFIX}{}",
        "0".repeat(IDENTITY_TOKEN_LENGTH - 1)
    );
    let long_reference = format!(
        "{IDENTITY_PREFIX}{}",
        "0".repeat(IDENTITY_TOKEN_LENGTH + 1)
    );

    let short_result = construct_identity_reference(&short_reference);
    let long_result = construct_identity_reference(&long_reference);

    kani::assert(
        matches!(
            short_result,
            Err(SkynetError::InvalidOpaqueReference {
                kind: OpaqueReferenceKind::CitizenIdentity,
                violation: OpaqueReferenceViolation::InvalidCharacter,
            })
        ),
        "a token shorter than 26 characters must be rejected",
    );

    kani::assert(
        matches!(
            long_result,
            Err(SkynetError::InvalidOpaqueReference {
                kind: OpaqueReferenceKind::CitizenIdentity,
                violation: OpaqueReferenceViolation::InvalidCharacter,
            })
        ),
        "a token longer than 26 characters must be rejected",
    );
}

#[kani::proof]
fn proof_reference_larger_than_global_bound_is_rejected() {
    let oversized_reference =
        format!("{IDENTITY_PREFIX}{}", "0".repeat(MAX_REFERENCE_LENGTH));

    let result = CitizenIdentityReference::parse(&oversized_reference);

    kani::assert(
        matches!(
            result,
            Err(SkynetError::InvalidOpaqueReference {
                kind: OpaqueReferenceKind::CitizenIdentity,
                violation: OpaqueReferenceViolation::TooLong,
            })
        ),
        "an identity reference exceeding the global opaque-reference bound must be rejected",
    );
}
