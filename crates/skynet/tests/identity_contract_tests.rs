use skynet::{
    error::{
        OpaqueReferenceKind,
        OpaqueReferenceViolation,
        SkynetError,
    },
    identity::{
        construct_identity_reference,
        is_valid_identity_reference,
        validate_identity_reference,
    },
    types::CitizenIdentityReference,
};

const VALID_IDENTITY_REFERENCE: &str = "cit:0123456789ABCDEFGHJKMNPQRS";

#[test]
fn valid_identity_reference_is_accepted() {
    let reference = construct_identity_reference(VALID_IDENTITY_REFERENCE)
        .expect("fixed-width opaque identity reference must validate");

    assert_eq!(reference.as_str(), VALID_IDENTITY_REFERENCE);
    assert!(is_valid_identity_reference(&reference));
    assert!(validate_identity_reference(&reference).is_ok());
}

#[test]
fn identity_reference_rejects_empty_token() {
    let error = construct_identity_reference("cit:")
        .expect_err("identity reference without an opaque token must fail");

    assert_eq!(
        error,
        SkynetError::InvalidOpaqueReference {
            kind: OpaqueReferenceKind::CitizenIdentity,
            violation: OpaqueReferenceViolation::Empty,
        }
    );
}

#[test]
fn identity_reference_rejects_invalid_prefix() {
    let error = construct_identity_reference("holder:0123456789ABCDEFGHJKMNPQRS")
        .expect_err("identity reference with an unexpected prefix must fail");

    assert_eq!(
        error,
        SkynetError::InvalidOpaqueReference {
            kind: OpaqueReferenceKind::CitizenIdentity,
            violation: OpaqueReferenceViolation::InvalidPrefix,
        }
    );
}

#[test]
fn identity_reference_rejects_non_opaque_human_readable_input() {
    let error = construct_identity_reference("cit:ALICE-EXAMPLE-IDENTITY-000")
        .expect_err("human-readable identity-like input must not satisfy the opaque token grammar");

    assert_eq!(
        error,
        SkynetError::InvalidOpaqueReference {
            kind: OpaqueReferenceKind::CitizenIdentity,
            violation: OpaqueReferenceViolation::InvalidCharacter,
        }
    );
}

#[test]
fn identity_reference_rejects_invalid_fixed_token_length() {
    let error = construct_identity_reference("cit:0123456789ABCDEFGHJKMNPQR")
        .expect_err("identity reference token shorter than 26 characters must fail");

    assert_eq!(
        error,
        SkynetError::InvalidOpaqueReference {
            kind: OpaqueReferenceKind::CitizenIdentity,
            violation: OpaqueReferenceViolation::InvalidCharacter,
        }
    );
}

#[test]
fn identity_reference_rejects_disallowed_token_characters() {
    let error = construct_identity_reference("cit:0123456789ABCDEFGHJKMNPQR!")
        .expect_err("identity reference token with prohibited characters must fail");

    assert_eq!(
        error,
        SkynetError::InvalidOpaqueReference {
            kind: OpaqueReferenceKind::CitizenIdentity,
            violation: OpaqueReferenceViolation::InvalidCharacter,
        }
    );
}

#[test]
fn identity_reference_rejects_values_exceeding_global_reference_bound() {
    let oversized = format!("cit:{}", "0".repeat(129));

    let error = CitizenIdentityReference::parse(&oversized)
        .expect_err("identity reference larger than the bounded reference limit must fail");

    assert_eq!(
        error,
        SkynetError::InvalidOpaqueReference {
            kind: OpaqueReferenceKind::CitizenIdentity,
            violation: OpaqueReferenceViolation::TooLong,
        }
    );
}

#[test]
fn identity_reference_debug_output_is_redacted() {
    let reference = construct_identity_reference(VALID_IDENTITY_REFERENCE)
        .expect("valid identity reference must construct");

    let debug_output = format!("{reference:?}");

    assert!(debug_output.contains("CitizenIdentityReference"));
    assert!(debug_output.contains("<opaque>"));
    assert!(!debug_output.contains(VALID_IDENTITY_REFERENCE));
}

#[test]
fn identity_reference_serializes_as_an_opaque_scalar_only() {
    let reference = construct_identity_reference(VALID_IDENTITY_REFERENCE)
        .expect("valid identity reference must construct");

    let serialized = serde_json::to_value(&reference)
        .expect("opaque identity reference must serialize");

    let serialized_value = serialized
        .as_str()
        .expect("identity reference must serialize as a JSON string");

    assert_eq!(serialized_value, VALID_IDENTITY_REFERENCE);
    assert!(!serialized.is_object());
    assert!(!serialized.is_array());
    assert!(!serialized.is_null());
}

#[test]
fn identity_reference_serialization_has_no_structured_prohibited_data_fields() {
    let reference = construct_identity_reference(VALID_IDENTITY_REFERENCE)
        .expect("valid identity reference must construct");

    let serialized = serde_json::to_string(&reference)
        .expect("opaque identity reference must serialize");

    for prohibited_marker in [
        "name",
        "subject",
        "biometric",
        "neural",
        "eeg",
        "bci",
        "public_key",
        "wallet",
        "credential",
        "claim",
        "proof",
        "nonce",
        "route",
        "location",
        "address",
        "telemetry",
    ] {
        assert!(
            !serialized.contains(prohibited_marker),
            "serialized identity reference must not expose prohibited field marker: {prohibited_marker}"
        );
    }
}
