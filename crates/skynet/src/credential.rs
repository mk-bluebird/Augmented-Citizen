//! Format-neutral credential evidence contracts.
//!
//! This module does not parse, decode, validate, issue, sign, encrypt, fetch,
//! transport, or store credentials. It contains no raw credential payload,
//! claim value, presentation data, key material, proof material, nonce, route,
//! wallet identifier, biometric input, or neural input.
//!
//! Reviewed adapters convert format-specific evidence into these minimized
//! contracts before Skynet policy evaluation.

use core::fmt;

use crate::{
    error::{
        CredentialProfileFailure,
        SkynetError,
        SkynetResult,
    },
    privacy::{
        AllowedCoreDomainResultCategory,
        CoreDomainResult,
    },
    types::DisclosureDescriptorSetId,
};

/// Re-export of the normalized credential lifecycle status.
///
/// Status resolution remains an adapter responsibility. This module does not
/// retrieve or validate status evidence.
pub use crate::status::CredentialStatus;

const PROFILE_IDENTIFIER_PREFIX: &str = "cprof:";
const PROFILE_VERSION_PREFIX: &str = "cver:";
const MAX_PROFILE_IDENTIFIER_LENGTH: usize = 96;
const MAX_PROFILE_VERSION_LENGTH: usize = 64;

/// Format-neutral, opaque identifier for an accepted credential profile.
///
/// A profile identifier selects a reviewed adapter contract. It is not a raw
/// credential type, claim name, issuer identifier, holder identifier, or
/// verifier route.
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[serde(transparent)]
pub struct CredentialProfileIdentifier(String);

impl CredentialProfileIdentifier {
    /// Required prefix for credential-profile identifiers.
    pub const PREFIX: &'static str = PROFILE_IDENTIFIER_PREFIX;

    /// Maximum serialized length for a credential-profile identifier.
    pub const MAX_LENGTH: usize = MAX_PROFILE_IDENTIFIER_LENGTH;

    /// Parses a bounded format-neutral credential-profile identifier.
    ///
    /// Valid identifiers use:
    ///
    /// ```text
    /// cprof:<lowercase-profile-token>
    /// ```
    pub fn parse(value: impl AsRef<str>) -> SkynetResult<Self> {
        let value = value.as_ref();

        if value.is_empty()
            || value.len() > Self::MAX_LENGTH
            || !value.starts_with(Self::PREFIX)
        {
            return Err(SkynetError::UnsupportedCredentialProfile {
                reason: CredentialProfileFailure::Unrecognized,
            });
        }

        let suffix = &value[Self::PREFIX.len()..];

        if suffix.is_empty()
            || !suffix.chars().all(|character| {
                character.is_ascii_lowercase()
                    || character.is_ascii_digit()
                    || character == '-'
                    || character == '_'
                    || character == '.'
            })
        {
            return Err(SkynetError::UnsupportedCredentialProfile {
                reason: CredentialProfileFailure::Unrecognized,
            });
        }

        Ok(Self(value.to_owned()))
    }

    /// Returns the validated profile identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<&str> for CredentialProfileIdentifier {
    type Error = SkynetError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl TryFrom<String> for CredentialProfileIdentifier {
    type Error = SkynetError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl fmt::Debug for CredentialProfileIdentifier {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("CredentialProfileIdentifier")
            .field(&"<profile>")
            .finish()
    }
}

/// Opaque version identifier for an accepted credential profile.
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[serde(transparent)]
pub struct CredentialProfileVersion(String);

impl CredentialProfileVersion {
    /// Required prefix for credential-profile versions.
    pub const PREFIX: &'static str = PROFILE_VERSION_PREFIX;

    /// Maximum serialized length for a credential-profile version.
    pub const MAX_LENGTH: usize = MAX_PROFILE_VERSION_LENGTH;

    /// Parses a bounded credential-profile version.
    ///
    /// Valid versions use:
    ///
    /// ```text
    /// cver:<lowercase-version-token>
    /// ```
    pub fn parse(value: impl AsRef<str>) -> SkynetResult<Self> {
        let value = value.as_ref();

        if value.is_empty()
            || value.len() > Self::MAX_LENGTH
            || !value.starts_with(Self::PREFIX)
        {
            return Err(SkynetError::UnsupportedCredentialProfile {
                reason: CredentialProfileFailure::UnsupportedVersion,
            });
        }

        let suffix = &value[Self::PREFIX.len()..];

        if suffix.is_empty()
            || !suffix.chars().all(|character| {
                character.is_ascii_lowercase()
                    || character.is_ascii_digit()
                    || character == '-'
                    || character == '_'
                    || character == '.'
            })
        {
            return Err(SkynetError::UnsupportedCredentialProfile {
                reason: CredentialProfileFailure::UnsupportedVersion,
            });
        }

        Ok(Self(value.to_owned()))
    }

    /// Returns the validated credential-profile version.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<&str> for CredentialProfileVersion {
    type Error = SkynetError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl TryFrom<String> for CredentialProfileVersion {
    type Error = SkynetError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl fmt::Debug for CredentialProfileVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("CredentialProfileVersion")
            .field(&"<version>")
            .finish()
    }
}

/// Closed result of adapter-side disclosure conformance evaluation.
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
pub enum DisclosureConformance {
    /// The adapter established that disclosed descriptors are within the
    /// requested and policy-permitted descriptor set.
    WithinApprovedScope,

    /// The adapter established that disclosure exceeded, omitted required
    /// descriptors from, or otherwise mismatched the approved scope.
    OutsideApprovedScope,

    /// The adapter could not establish disclosure conformance from available
    /// evidence.
    Unavailable,
}

/// Content-minimized receipt for one disclosure conformance evaluation.
///
/// The receipt contains only the reviewed profile identity, profile version,
/// opaque descriptor-set identity, and closed conformance result. It contains
/// no disclosed claim names, claim values, credential bytes, presentation
/// bytes, holder identifier, verifier route, proof material, or key material.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct DisclosureReceipt {
    profile_identifier: CredentialProfileIdentifier,
    profile_version: CredentialProfileVersion,
    descriptor_set_id: DisclosureDescriptorSetId,
    conformance: DisclosureConformance,
}

impl DisclosureReceipt {
    /// Creates a format-neutral disclosure conformance receipt.
    #[must_use]
    pub fn new(
        profile_identifier: CredentialProfileIdentifier,
        profile_version: CredentialProfileVersion,
        descriptor_set_id: DisclosureDescriptorSetId,
        conformance: DisclosureConformance,
    ) -> Self {
        Self {
            profile_identifier,
            profile_version,
            descriptor_set_id,
            conformance,
        }
    }

    /// Returns the reviewed credential-profile identifier.
    #[must_use]
    pub fn profile_identifier(&self) -> &CredentialProfileIdentifier {
        &self.profile_identifier
    }

    /// Returns the reviewed credential-profile version.
    #[must_use]
    pub fn profile_version(&self) -> &CredentialProfileVersion {
        &self.profile_version
    }

    /// Returns the opaque approved disclosure descriptor-set identifier.
    #[must_use]
    pub fn descriptor_set_id(&self) -> &DisclosureDescriptorSetId {
        &self.descriptor_set_id
    }

    /// Returns the closed disclosure conformance result.
    #[must_use]
    pub const fn conformance(&self) -> DisclosureConformance {
        self.conformance
    }

    /// Returns whether the receipt satisfies a policy rule requiring approved
    /// disclosure conformance.
    #[must_use]
    pub const fn is_within_approved_scope(&self) -> bool {
        matches!(
            self.conformance,
            DisclosureConformance::WithinApprovedScope
        )
    }
}

impl CoreDomainResult for DisclosureReceipt {
    fn core_domain_category(&self) -> AllowedCoreDomainResultCategory {
        AllowedCoreDomainResultCategory::DisclosureReceipt
    }
}

/// Format-neutral adapter evidence for a credential policy evaluation.
///
/// This type is evidence only. It does not represent an eligibility decision,
/// holder authorization, policy decision, credential, or presentation.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct EligibilityEvidence {
    profile_identifier: CredentialProfileIdentifier,
    profile_version: CredentialProfileVersion,
    status: CredentialStatus,
    disclosure_receipt: DisclosureReceipt,
}

impl EligibilityEvidence {
    /// Creates a minimized evidence bundle with internally consistent profile
    /// identifier and profile version.
    pub fn new(
        profile_identifier: CredentialProfileIdentifier,
        profile_version: CredentialProfileVersion,
        status: CredentialStatus,
        disclosure_receipt: DisclosureReceipt,
    ) -> SkynetResult<Self> {
        if disclosure_receipt.profile_identifier() != &profile_identifier
            || disclosure_receipt.profile_version() != &profile_version
        {
            return Err(SkynetError::UnsupportedCredentialProfile {
                reason: CredentialProfileFailure::IncompleteEvidence,
            });
        }

        Ok(Self {
            profile_identifier,
            profile_version,
            status,
            disclosure_receipt,
        })
    }

    /// Returns the reviewed credential-profile identifier.
    #[must_use]
    pub fn profile_identifier(&self) -> &CredentialProfileIdentifier {
        &self.profile_identifier
    }

    /// Returns the reviewed credential-profile version.
    #[must_use]
    pub fn profile_version(&self) -> &CredentialProfileVersion {
        &self.profile_version
    }

    /// Returns the normalized credential lifecycle status.
    #[must_use]
    pub const fn status(&self) -> CredentialStatus {
        self.status
    }

    /// Returns the content-minimized disclosure receipt.
    #[must_use]
    pub fn disclosure_receipt(&self) -> &DisclosureReceipt {
        &self.disclosure_receipt
    }

    /// Returns whether the evidence is suitable for a policy rule requiring
    /// active status and approved disclosure conformance.
    #[must_use]
    pub const fn satisfies_active_minimized_evidence_requirement(&self) -> bool {
        self.status.is_active()
            && self.disclosure_receipt.is_within_approved_scope()
    }
}
