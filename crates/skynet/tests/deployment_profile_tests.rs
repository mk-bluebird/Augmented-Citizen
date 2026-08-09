use core::num::NonZeroU32;

use skynet::{
    deployment::{
        is_phx_az_us,
        phx_az_us,
        validate_recognized_profile,
        DeploymentProfileBinding,
        DeploymentProfileVersion,
    },
    error::{
        DeploymentProfileFailure,
        SkynetError,
    },
    types::{
        DeploymentProfile,
        PolicyAuthorityReference,
        PolicyVersion,
        UtcTimestamp,
    },
};

const POLICY_AUTHORITY: &str = "polauth:skynet-policy-authority";
const POLICY_VERSION: &str = "polver:v1.0";

fn policy_authority() -> PolicyAuthorityReference {
    PolicyAuthorityReference::parse(POLICY_AUTHORITY)
        .expect("test policy authority reference must be valid")
}

fn policy_version() -> PolicyVersion {
    PolicyVersion::parse(POLICY_VERSION)
        .expect("test policy version reference must be valid")
}

fn deployment_version() -> DeploymentProfileVersion {
    DeploymentProfileVersion::new(
        NonZeroU32::new(1).expect("deployment profile version must be non-zero"),
    )
}

fn valid_phx_binding() -> DeploymentProfileBinding {
    DeploymentProfileBinding::new(
        phx_az_us().expect("PHX_AZ_US profile must be constructible"),
        deployment_version(),
        policy_authority(),
        policy_version(),
        UtcTimestamp::from_unix_seconds(1_700_000_000),
        UtcTimestamp::from_unix_seconds(1_800_000_000),
    )
    .expect("valid PHX_AZ_US policy binding must construct")
}

#[test]
fn phx_az_us_is_the_initial_recognized_application_profile() {
    let profile = phx_az_us().expect("PHX_AZ_US must be a valid deployment profile");

    assert_eq!(profile.as_str(), "PHX_AZ_US");
    assert!(is_phx_az_us(&profile));
    assert!(validate_recognized_profile(&profile).is_ok());
}

#[test]
fn phx_az_us_binding_is_explicitly_versioned() {
    let binding = valid_phx_binding();

    assert_eq!(binding.profile().as_str(), "PHX_AZ_US");
    assert_eq!(binding.version().get(), 1);
    assert_eq!(
        binding.policy_authority().as_str(),
        POLICY_AUTHORITY
    );
    assert_eq!(binding.policy_version().as_str(), POLICY_VERSION);
}

#[test]
fn phx_az_us_binding_is_valid_only_inside_its_policy_effective_interval() {
    let binding = valid_phx_binding();

    assert!(
        binding
            .validate_at(UtcTimestamp::from_unix_seconds(1_700_000_000))
            .is_ok()
    );
    assert!(
        binding
            .validate_at(UtcTimestamp::from_unix_seconds(1_799_999_999))
            .is_ok()
    );
    assert!(
        binding
            .validate_at(UtcTimestamp::from_unix_seconds(1_800_000_000))
            .is_err()
    );
}

#[test]
fn deployment_profile_binding_rejects_invalid_effective_interval() {
    let error = DeploymentProfileBinding::new(
        phx_az_us().expect("PHX_AZ_US profile must be constructible"),
        deployment_version(),
        policy_authority(),
        policy_version(),
        UtcTimestamp::from_unix_seconds(1_800_000_000),
        UtcTimestamp::from_unix_seconds(1_700_000_000),
    )
    .expect_err("deployment profile binding with reversed interval must fail");

    assert!(matches!(
        error,
        SkynetError::InvalidTemporalWindow { .. }
    ));
}

#[test]
fn unknown_deployment_profile_is_rejected_without_location_inference() {
    let profile = DeploymentProfile::parse("TUCSON_AZ_US")
        .expect("syntactically valid profile labels may still be unrecognized");

    let error = validate_recognized_profile(&profile)
        .expect_err("unrecognized profile must not be inferred from a regional label");

    assert_eq!(
        error,
        SkynetError::UnknownDeploymentProfile {
            reason: DeploymentProfileFailure::Unrecognized,
        }
    );
}

#[test]
fn phx_az_us_profile_serializes_as_a_label_not_location_evidence() {
    let profile = phx_az_us().expect("PHX_AZ_US profile must be constructible");

    let serialized = serde_json::to_value(&profile)
        .expect("deployment profile must serialize");

    assert_eq!(serialized.as_str(), Some("PHX_AZ_US"));
    assert!(!serialized.is_object());
    assert!(!serialized.is_array());
}

#[test]
fn deployment_profile_binding_has_only_policy_and_temporal_fields() {
    let binding = valid_phx_binding();

    let serialized = serde_json::to_value(&binding)
        .expect("deployment profile binding must serialize");

    let object = serialized
        .as_object()
        .expect("deployment profile binding must serialize as an object");

    let expected_fields = [
        "profile",
        "version",
        "policy_authority",
        "policy_version",
        "effective_from",
        "effective_to",
    ];

    assert_eq!(object.len(), expected_fields.len());

    for field in expected_fields {
        assert!(
            object.contains_key(field),
            "missing expected deployment binding field: {field}"
        );
    }
}

#[test]
fn deployment_profile_binding_contains_no_location_residency_or_connectivity_fields() {
    let binding = valid_phx_binding();

    let serialized = serde_json::to_string(&binding)
        .expect("deployment profile binding must serialize");

    for prohibited_marker in [
        "latitude",
        "longitude",
        "coordinate",
        "geolocation",
        "location",
        "address",
        "street",
        "residency",
        "resident",
        "municipality",
        "municipal",
        "government",
        "authorization",
        "service_access",
        "network",
        "endpoint",
        "connection",
        "telemetry",
        "device",
    ] {
        assert!(
            !serialized.contains(prohibited_marker),
            "deployment profile binding must not expose prohibited marker: {prohibited_marker}"
        );
    }
}

#[test]
fn expired_profile_binding_cannot_be_used_as_authorization() {
    let binding = valid_phx_binding();

    let error = binding
        .validate_at(UtcTimestamp::from_unix_seconds(1_800_000_001))
        .expect_err("expired deployment binding must not validate");

    assert_eq!(
        error,
        SkynetError::UnknownDeploymentProfile {
            reason: DeploymentProfileFailure::OutsideEffectiveInterval,
        }
    );
}
