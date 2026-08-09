//! Opaque identity-reference construction and validation.
//!
//! This module does not create identity references from personal data. A
//! reviewed holder-controlled system must supply an already opaque reference.
//! Skynet validates only its bounded representation and never receives names,
//! biometric data, neural data, public keys, wallet identifiers, or direct
//! subject identifiers.

use crate::{
    error::{
        OpaqueReferenceKind,
        OpaqueReferenceViolation,
        SkynetError,
        SkynetResult,
    },
    types::CitizenIdentityReference,
};

const IDENTITY_TOKEN_LENGTH: usize = 26;

/// Validates and constructs an opaque citizen identity reference.
///
/// The accepted representation is:
///
/// ```text
/// cit:<26-character-crockford-base32-token>
/// ```
///
/// The token must use uppercase Crockford Base32 characters:
///
/// ```text
/// 0-9 A-H J-K M-N P-T V-Z
/// ```
///
/// This restricted representation reduces accidental use of human-readable
/// names, email addresses, wallet identifiers, public keys, or direct subject
/// identifiers as core identity references.
pub fn construct_identity_reference(
    value: impl AsRef<str>,
) -> SkynetResult<CitizenIdentityReference> {
    let reference = CitizenIdentityReference::parse(value)?;
    validate_identity_reference(&reference)?;
    Ok(reference)
}

/// Validates an existing opaque citizen identity reference.
///
/// This function validates the type-specific prefix and the fixed opaque-token
/// grammar. It performs no identity proofing, wallet lookup, key validation,
/// biometric matching, neural-data processing, or external network access.
pub fn validate_identity_reference(
    reference: &CitizenIdentityReference,
) -> SkynetResult<()> {
    let value = reference.as_str();

    if !value.starts_with(CitizenIdentityReference::PREFIX) {
        return Err(SkynetError::InvalidOpaqueReference {
            kind: OpaqueReferenceKind::CitizenIdentity,
            violation: OpaqueReferenceViolation::InvalidPrefix,
        });
    }

    let token = &value[CitizenIdentityReference::PREFIX.len()..];

    if token.is_empty() {
        return Err(SkynetError::InvalidOpaqueReference {
            kind: OpaqueReferenceKind::CitizenIdentity,
            violation: OpaqueReferenceViolation::Empty,
        });
    }

    if token.len() != IDENTITY_TOKEN_LENGTH {
        return Err(SkynetError::InvalidOpaqueReference {
            kind: OpaqueReferenceKind::CitizenIdentity,
            violation: OpaqueReferenceViolation::InvalidCharacter,
        });
    }

    if !token.chars().all(is_crockford_base32_character) {
        return Err(SkynetError::InvalidOpaqueReference {
            kind: OpaqueReferenceKind::CitizenIdentity,
            violation: OpaqueReferenceViolation::InvalidCharacter,
        });
    }

    Ok(())
}

/// Returns whether a reference satisfies the Skynet opaque identity contract.
#[must_use]
pub fn is_valid_identity_reference(
    reference: &CitizenIdentityReference,
) -> bool {
    validate_identity_reference(reference).is_ok()
}

const fn is_crockford_base32_character(character: char) -> bool {
    matches!(
        character,
        '0'..='9'
            | 'A'..='H'
            | 'J'..='K'
            | 'M'..='N'
            | 'P'..='T'
            | 'V'..='Z'
    )
}
