//! Bounded Kani proofs for Skynet deployment-profile baseline invariants.
//!
//! This harness does not implement or assume P3 deployment-policy predicates.
//! It proves only local deployment-profile recognition and effective-interval
//! behavior.
//!
//! Run with:
//!
//! ```text
//! cargo kani --harness proof_recognized_profile_is_valid_inside_effective_interval
//! cargo kani --harness proof_recognized_profile_is_invalid_at_expiry_boundary
//! cargo kani --harness proof_unknown_profile_is_rejected_without_inference
//! cargo kani --harness proof_malformed_deployment_interval_is_rejected
//! ```

use core::num::NonZeroU32;

use skynet::{
    deployment::{
        phx_az_us,
        validate_recognized_profile,
        DeploymentProfileBinding,
        DeploymentProfileVersion,
    },
    error::{
        DeploymentProfileFailure,
        SkynetError,
        TemporalWindowKind,
        TemporalWindowViolation,
    },
    invariants::is_deployment_profile_valid,
    types::{
        DeploymentProfile,
        PolicyAuthorityReference,
        PolicyVersion,
        UtcTimestamp,
    },
};

const POLICY_AUTHORITY: &str = "polauth:skynet-policy-authority";
const POLICY_VERSION: &str = "polver:v1.0";

const EFFECTIVE_FROM: i64 = 1_700_000_000;
const EFFECTIVE_TO: i64 = 1_800_000_000;

fn policy_authority() -> PolicyAuthorityReference {
    PolicyAuthorityReference::parse(POLICY_AUTHORITY)
        .expect("fixed policy authority reference must be valid")
}

fn policy_version() -> PolicyVersion {
    PolicyVersion::parse(POLICY_VERSION)
        .expect("fixed policy version reference must be valid")
}

fn deployment_version() -> DeploymentProfileVersion {
    DeploymentProfileVersion::new(
        NonZeroU32::new(1).expect("one must be a non-zero profile version"),
    )
}

fn phx_binding() -> DeploymentProfileBinding {
    DeploymentProfileBinding::new(
        phx_az_us().expect("PHX_AZ_US must be a recognized profile"),
        deployment_version(),
        policy_authority(),
        policy_version(),
        UtcTimestamp::from_unix_seconds(EFFECTIVE_FROM),
        UtcTimestamp::from_unix_seconds(EFFECTIVE_TO),
    )
    .expect("recognized profile with increasing interval must construct")
}

#[kani::proof]
fn proof_recognized_profile_is_valid_inside_effective_interval() {
    let offset: u32 = kani::any();

    kani::assume(i64::from(offset) < EFFECTIVE_TO - EFFECTIVE_FROM);

    let evaluation_time =
        UtcTimestamp::from_unix_seconds(EFFECTIVE_FROM + i64::from(offset));
    let binding = phx_binding();

    kani::assert(
        binding.validate_at(evaluation_time).is_ok(),
        "recognized deployment binding must validate inside its effective interval",
    );

    kani::assert(
        is_deployment_profile_valid(&binding, evaluation_time),
        "deployment invariant must hold inside the effective interval",
    );
}

#[kani::proof]
fn proof_recognized_profile_is_invalid_at_expiry_boundary() {
    let binding = phx_binding();
    let expiry_boundary = UtcTimestamp::from_unix_seconds(EFFECTIVE_TO);

    let result = binding.validate_at(expiry_boundary);

    kani::assert(
        matches!(
            result,
            Err(SkynetError::UnknownDeploymentProfile {
                reason: DeploymentProfileFailure::OutsideEffectiveInterval,
            })
        ),
        "a deployment binding must not validate at its exclusive expiry boundary",
    );

    kani::assert(
        !is_deployment_profile_valid(&binding, expiry_boundary),
        "deployment invariant must reject an expired binding",
    );
}

#[kani::proof]
fn proof_unknown_profile_is_rejected_without_inference() {
    let profile = DeploymentProfile::parse("TUCSON_AZ_US")
        .expect("syntactically valid profile label must construct");

    let result = validate_recognized_profile(&profile);

    kani::assert(
        matches!(
            result,
            Err(SkynetError::UnknownDeploymentProfile {
                reason: DeploymentProfileFailure::Unrecognized,
            })
        ),
        "an unknown label must be rejected rather than inferred from its text",
    );
}

#[kani::proof]
fn proof_malformed_deployment_interval_is_rejected() {
    let first: u8 = kani::any();
    let second: u8 = kani::any();

    kani::assume(first >= second);

    let result = DeploymentProfileBinding::new(
        phx_az_us().expect("PHX_AZ_US must be a recognized profile"),
        deployment_version(),
        policy_authority(),
        policy_version(),
        UtcTimestamp::from_unix_seconds(i64::from(first)),
        UtcTimestamp::from_unix_seconds(i64::from(second)),
    );

    kani::assert(
        matches!(
            result,
            Err(SkynetError::InvalidTemporalWindow {
                kind: TemporalWindowKind::DeploymentProfile,
                violation: TemporalWindowViolation::InvalidOrdering,
            })
        ),
        "a deployment binding with an invalid interval must be rejected",
    );
}
