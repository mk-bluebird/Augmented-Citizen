//! Opaque, bounded, and content-minimized value types for the Skynet core.

use core::fmt;

use crate::error::{
    DeploymentProfileFailure,
    OpaqueReferenceKind,
    OpaqueReferenceViolation,
    SkynetError,
    SkynetResult,
};

const MAX_REFERENCE_LENGTH: usize = 128;
const MAX_POLICY_VERSION_LENGTH: usize = 64;
const MAX_DEPLOYMENT_PROFILE_LENGTH: usize = 64;

macro_rules! opaque_reference {
    (
        $documentation:literal,
        $name:ident,
        $prefix:literal,
        $kind:expr,
        $maximum_length:expr
    ) => {
        #[doc = $documentation]
        #[derive(Clone, Eq, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            /// Required, type-specific prefix for this reference.
            pub const PREFIX: &'static str = $prefix;

            /// Maximum serialized length for this reference.
            pub const MAX_LENGTH: usize = $maximum_length;

            /// Parses and validates a bounded opaque reference.
            ///
            /// The value must use the required prefix, contain a non-empty
            /// suffix, fit within the type's maximum length, and use only
            /// ASCII alphanumeric characters, hyphens, underscores, or dots
            /// after the prefix.
            pub fn parse(value: impl AsRef<str>) -> SkynetResult<Self> {
                let value = value.as_ref();

                if value.is_empty() {
                    return Err(SkynetError::InvalidOpaqueReference {
                        kind: $kind,
                        violation: OpaqueReferenceViolation::Empty,
                    });
                }

                if value.len() > Self::MAX_LENGTH {
                    return Err(SkynetError::InvalidOpaqueReference {
                        kind: $kind,
                        violation: OpaqueReferenceViolation::TooLong,
                    });
                }

                if !value.starts_with(Self::PREFIX) {
                    return Err(SkynetError::InvalidOpaqueReference {
                        kind: $kind,
                        violation: OpaqueReferenceViolation::InvalidPrefix,
                    });
                }

                let suffix = &value[Self::PREFIX.len()..];

                if suffix.is_empty() {
                    return Err(SkynetError::InvalidOpaqueReference {
                        kind: $kind,
                        violation: OpaqueReferenceViolation::Empty,
                    });
                }

                if !suffix.chars().all(|character| {
                    character.is_ascii_alphanumeric()
                        || character == '-'
                        || character == '_'
                        || character == '.'
                }) {
                    return Err(SkynetError::InvalidOpaqueReference {
                        kind: $kind,
                        violation: OpaqueReferenceViolation::InvalidCharacter,
                    });
                }

                Ok(Self(value.to_owned()))
            }

            /// Returns the validated opaque representation.
            ///
            /// Callers must not place this value in logs, free-text records,
            /// telemetry, or public identifiers unless an approved policy
            /// explicitly permits that use.
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl TryFrom<&str> for $name {
            type Error = SkynetError;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                Self::parse(value)
            }
        }

        impl TryFrom<String> for $name {
            type Error = SkynetError;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::parse(value)
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter
                    .debug_tuple(stringify!($name))
                    .field(&"<opaque>")
                    .finish()
            }
        }
    };
}

opaque_reference!(
    "Opaque local reference to a holder-controlled identity.",
    CitizenIdentityReference,
    "cit:",
    OpaqueReferenceKind::CitizenIdentity,
    MAX_REFERENCE_LENGTH
);

opaque_reference!(
    "Opaque reference to a credential retained outside the Skynet core.",
    CredentialReference,
    "cred:",
    OpaqueReferenceKind::Credential,
    MAX_REFERENCE_LENGTH
);

opaque_reference!(
    "Opaque reference to an approved credential-requesting verifier.",
    VerifierReference,
    "ver:",
    OpaqueReferenceKind::Verifier,
    MAX_REFERENCE_LENGTH
);

opaque_reference!(
    "Opaque reference to one presentation request.",
    PresentationRequestId,
    "req:",
    OpaqueReferenceKind::PresentationRequest,
    MAX_REFERENCE_LENGTH
);

opaque_reference!(
    "Opaque reference to a purpose-specific consent scope.",
    ConsentScopeId,
    "consent:",
    OpaqueReferenceKind::ConsentScope,
    MAX_REFERENCE_LENGTH
);

opaque_reference!(
    "Opaque reference to a recognized policy authority.",
    PolicyAuthorityReference,
    "polauth:",
    OpaqueReferenceKind::PolicyAuthority,
    MAX_REFERENCE_LENGTH
);

opaque_reference!(
    "Opaque version identifier for a policy program.",
    PolicyVersion,
    "polver:",
    OpaqueReferenceKind::PolicyAuthority,
    MAX_POLICY_VERSION_LENGTH
);

opaque_reference!(
    "Opaque reference to a policy rule within a policy version.",
    PolicyRuleReference,
    "rule:",
    OpaqueReferenceKind::PolicyRule,
    MAX_REFERENCE_LENGTH
);

opaque_reference!(
    "Opaque identifier for a policy-approved disclosure descriptor set.",
    DisclosureDescriptorSetId,
    "dset:",
    OpaqueReferenceKind::DisclosureDescriptorSet,
    MAX_REFERENCE_LENGTH
);

opaque_reference!(
    "Transaction-scoped opaque receipt identifier for one eligibility decision.",
    DecisionReceiptId,
    "decision:",
    OpaqueReferenceKind::DecisionReceipt,
    MAX_REFERENCE_LENGTH
);

opaque_reference!(
    "Transaction-scoped opaque identifier for one audit event.",
    AuditEventId,
    "audit:",
    OpaqueReferenceKind::AuditEvent,
    MAX_REFERENCE_LENGTH
);

/// Versioned, policy-bound infrastructure deployment profile.
///
/// A deployment profile is an application-defined configuration label. It is
/// not evidence of municipal authorization, physical residence, real-time
/// location, a city-service account, or live infrastructure connectivity.
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[serde(transparent)]
pub struct DeploymentProfile(String);

impl DeploymentProfile {
    /// Maximum serialized length for a deployment profile label.
    pub const MAX_LENGTH: usize = MAX_DEPLOYMENT_PROFILE_LENGTH;

    /// Parses a policy-bound deployment profile label.
    ///
    /// Labels use uppercase ASCII letters, digits, and single underscores.
    /// `PHX_AZ_US` is an example application-defined deployment label.
    pub fn parse(value: impl AsRef<str>) -> SkynetResult<Self> {
        let value = value.as_ref();

        let valid_characters = value.chars().all(|character| {
            character.is_ascii_uppercase()
                || character.is_ascii_digit()
                || character == '_'
        });

        let valid_structure = !value.is_empty()
            && value.len() <= Self::MAX_LENGTH
            && !value.starts_with('_')
            && !value.ends_with('_')
            && !value.contains("__");

        if !valid_characters || !valid_structure {
            return Err(SkynetError::UnknownDeploymentProfile {
                reason: DeploymentProfileFailure::Unrecognized,
            });
        }

        Ok(Self(value.to_owned()))
    }

    /// Returns the validated deployment-profile label.
    ///
    /// This value must not be interpreted as proof of location, residence,
    /// government affiliation, municipal approval, or service authorization.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<&str> for DeploymentProfile {
    type Error = SkynetError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl TryFrom<String> for DeploymentProfile {
    type Error = SkynetError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl fmt::Debug for DeploymentProfile {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("DeploymentProfile")
            .field(&self.0)
            .finish()
    }
}

/// UTC time represented as signed Unix seconds.
///
/// The type intentionally carries no timezone name, location, device clock
/// identifier, or telemetry source. Temporal validity rules are evaluated by
/// consent, authorization, policy, deployment, and status modules.
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
pub struct UtcTimestamp(i64);

impl UtcTimestamp {
    /// Creates a UTC timestamp from signed Unix seconds.
    #[must_use]
    pub const fn from_unix_seconds(seconds: i64) -> Self {
        Self(seconds)
    }

    /// Returns the signed Unix-second representation.
    #[must_use]
    pub const fn unix_seconds(self) -> i64 {
        self.0
    }

    /// Returns whether this timestamp occurs strictly before another timestamp.
    #[must_use]
    pub const fn is_before(self, other: Self) -> bool {
        self.0 < other.0
    }

    /// Returns whether this timestamp occurs strictly after another timestamp.
    #[must_use]
    pub const fn is_after(self, other: Self) -> bool {
        self.0 > other.0
    }

    /// Returns the elapsed seconds when `self` is not later than `other`.
    #[must_use]
    pub const fn elapsed_until(self, other: Self) -> Option<u64> {
        if self.0 > other.0 {
            return None;
        }

        let difference = i128::from(other.0) - i128::from(self.0);

        if difference > i128::from(u64::MAX) {
            return None;
        }

        Some(difference as u64)
    }
}
