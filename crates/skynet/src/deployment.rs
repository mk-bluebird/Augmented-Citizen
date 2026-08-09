//! Versioned, policy-bound deployment-profile validation.
//!
//! A deployment profile is an application-defined policy label. It is not
//! proof of municipal affiliation, government approval, physical presence,
//! residency, address, real-time location, service access, network attachment,
//! device ownership, or infrastructure connectivity.

use core::num::NonZeroU32;

use crate::{
    error::{
        DeploymentProfileFailure,
        SkynetError,
        SkynetResult,
        TemporalWindowKind,
        TemporalWindowViolation,
    },
    types::{
        DeploymentProfile,
        PolicyAuthorityReference,
        PolicyVersion,
        UtcTimestamp,
    },
};

const PHX_AZ_US_LABEL: &str = "PHX_AZ_US";

/// Monotonically increasing version for a deployment-profile definition.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
)]
#[serde(transparent)]
pub struct DeploymentProfileVersion(NonZeroU32);

impl DeploymentProfileVersion {
    /// Creates a non-zero deployment-profile version.
    #[must_use]
    pub const fn new(value: NonZeroU32) -> Self {
        Self(value)
    }

    /// Returns the numeric deployment-profile version.
    #[must_use]
    pub const fn get(self) -> u32 {
        self.0.get()
    }
}

/// Policy-bound registration for one recognized deployment profile.
///
/// This record expresses only that a profile label is available for policy
/// evaluation within a declared effective interval. It does not establish any
/// real-world affiliation, location, or infrastructure access.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DeploymentProfileBinding {
    profile: DeploymentProfile,
    version: DeploymentProfileVersion,
    policy_authority: PolicyAuthorityReference,
    policy_version: PolicyVersion,
    effective_from: UtcTimestamp,
    effective_to: UtcTimestamp,
}

impl DeploymentProfileBinding {
    /// Creates a validated, versioned deployment-profile binding.
    ///
    /// The profile must be recognized by this Skynet release and the effective
    /// interval must have a strictly increasing start and end timestamp.
    pub fn new(
        profile: DeploymentProfile,
        version: DeploymentProfileVersion,
        policy_authority: PolicyAuthorityReference,
        policy_version: PolicyVersion,
        effective_from: UtcTimestamp,
        effective_to: UtcTimestamp,
    ) -> SkynetResult<Self> {
        validate_recognized_profile(&profile)?;

        if !effective_from.is_before(effective_to) {
            return Err(SkynetError::InvalidTemporalWindow {
                kind: TemporalWindowKind::DeploymentProfile,
                violation: TemporalWindowViolation::InvalidOrdering,
            });
        }

        Ok(Self {
            profile,
            version,
            policy_authority,
            policy_version,
            effective_from,
            effective_to,
        })
    }

    /// Returns the application-defined deployment-profile label.
    #[must_use]
    pub fn profile(&self) -> &DeploymentProfile {
        &self.profile
    }

    /// Returns the version of this deployment-profile definition.
    #[must_use]
    pub const fn version(&self) -> DeploymentProfileVersion {
        self.version
    }

    /// Returns the opaque reference to the policy authority.
    #[must_use]
    pub fn policy_authority(&self) -> &PolicyAuthorityReference {
        &self.policy_authority
    }

    /// Returns the policy version governing this deployment-profile binding.
    #[must_use]
    pub fn policy_version(&self) -> &PolicyVersion {
        &self.policy_version
    }

    /// Returns the inclusive start of the binding's effective interval.
    #[must_use]
    pub const fn effective_from(&self) -> UtcTimestamp {
        self.effective_from
    }

    /// Returns the exclusive end of the binding's effective interval.
    #[must_use]
    pub const fn effective_to(&self) -> UtcTimestamp {
        self.effective_to
    }

    /// Returns whether the binding is effective at the supplied evaluation time.
    #[must_use]
    pub const fn is_effective_at(&self, evaluation_time: UtcTimestamp) -> bool {
        !evaluation_time.is_before(self.effective_from)
            && evaluation_time.is_before(self.effective_to)
    }

    /// Validates that this binding is effective at the supplied evaluation time.
    pub fn validate_at(
        &self,
        evaluation_time: UtcTimestamp,
    ) -> SkynetResult<()> {
        validate_recognized_profile(&self.profile)?;

        if !self.is_effective_at(evaluation_time) {
            return Err(SkynetError::UnknownDeploymentProfile {
                reason: DeploymentProfileFailure::OutsideEffectiveInterval,
            });
        }

        Ok(())
    }
}

/// Returns the initial application-defined Phoenix deployment profile.
///
/// This profile is a policy label only. It does not establish affiliation with
/// the City of Phoenix, a municipal service, physical residence, real-time
/// presence, street address, or network connection.
pub fn phx_az_us() -> SkynetResult<DeploymentProfile> {
    DeploymentProfile::parse(PHX_AZ_US_LABEL)
}

/// Returns whether a profile is the initial Skynet deployment label.
#[must_use]
pub fn is_phx_az_us(profile: &DeploymentProfile) -> bool {
    profile.as_str() == PHX_AZ_US_LABEL
}

/// Validates that a deployment profile is recognized by this Skynet release.
///
/// Future region labels must be introduced through a documented, versioned
/// policy update and an accompanying Skynet release. Unknown labels are never
/// inferred from address, location, network, device, or telemetry data.
pub fn validate_recognized_profile(
    profile: &DeploymentProfile,
) -> SkynetResult<()> {
    if is_phx_az_us(profile) {
        return Ok(());
    }

    Err(SkynetError::UnknownDeploymentProfile {
        reason: DeploymentProfileFailure::Unrecognized,
    })
}
