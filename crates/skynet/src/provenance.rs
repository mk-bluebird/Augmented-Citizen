//! Local, typed policy-lineage records.
//!
//! Policy lineage records identify which reviewed policy authority, version,
//! rule, effective interval, and opaque content reference governed a decision.
//! They do not contain policy source text, credential claims, holder identity,
//! public keys, cryptographic proof material, routing information, or raw
//! physiological data.

use core::fmt;

use crate::{
    error::{
        PolicyLineageFailure,
        PolicyLineageField,
        SkynetError,
        SkynetResult,
        TemporalWindowKind,
        TemporalWindowViolation,
    },
    privacy::{
        AllowedCoreDomainResultCategory,
        CoreDomainResult,
    },
    types::{
        PolicyAuthorityReference,
        PolicyRuleReference,
        PolicyVersion,
        UtcTimestamp,
    },
};

const CONTENT_REFERENCE_PREFIX: &str = "content:";
const CONTENT_REFERENCE_TOKEN_LENGTH: usize = 26;
const CONTENT_REFERENCE_MAX_LENGTH: usize =
    CONTENT_REFERENCE_PREFIX.len() + CONTENT_REFERENCE_TOKEN_LENGTH;

/// Opaque reference to reviewed policy content.
///
/// This type identifies policy content without storing the policy text,
/// executable source, credential data, or any sensitive material in the
/// Skynet core.
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[serde(transparent)]
pub struct PolicyContentReference(String);

impl PolicyContentReference {
    /// Required prefix for a policy content reference.
    pub const PREFIX: &'static str = CONTENT_REFERENCE_PREFIX;

    /// Maximum serialized length for a policy content reference.
    pub const MAX_LENGTH: usize = CONTENT_REFERENCE_MAX_LENGTH;

    /// Parses a fixed-width opaque policy content reference.
    ///
    /// Valid references use the form:
    ///
    /// ```text
    /// content:<26-character-crockford-base32-token>
    /// ```
    pub fn parse(value: impl AsRef<str>) -> SkynetResult<Self> {
        let value = value.as_ref();

        if !value.starts_with(Self::PREFIX) {
            return Err(SkynetError::PolicyLineageMismatch {
                field: PolicyLineageField::ContentReference,
                reason: PolicyLineageFailure::Mismatch,
            });
        }

        if value.len() != Self::MAX_LENGTH {
            return Err(SkynetError::PolicyLineageMismatch {
                field: PolicyLineageField::ContentReference,
                reason: PolicyLineageFailure::Missing,
            });
        }

        let token = &value[Self::PREFIX.len()..];

        if !token.chars().all(is_crockford_base32_character) {
            return Err(SkynetError::PolicyLineageMismatch {
                field: PolicyLineageField::ContentReference,
                reason: PolicyLineageFailure::Mismatch,
            });
        }

        Ok(Self(value.to_owned()))
    }

    /// Returns the validated opaque content reference.
    ///
    /// Callers must not interpret this value as policy source text or use it as
    /// a stable holder, credential, verifier, device, or transport correlator.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<&str> for PolicyContentReference {
    type Error = SkynetError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl TryFrom<String> for PolicyContentReference {
    type Error = SkynetError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl fmt::Debug for PolicyContentReference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("PolicyContentReference")
            .field(&"<opaque>")
            .finish()
    }
}

/// Immutable, local lineage for the policy governing a Skynet decision.
///
/// A lineage record is valid only during its declared effective interval. It
/// supports decision reproducibility without causing the core to retrieve,
/// execute, or retain policy source material.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct PolicyLineage {
    authority: PolicyAuthorityReference,
    version: PolicyVersion,
    rule_reference: PolicyRuleReference,
    effective_from: UtcTimestamp,
    effective_to: UtcTimestamp,
    content_reference: PolicyContentReference,
}

impl PolicyLineage {
    /// Creates a policy-lineage record with a strictly increasing effective
    /// interval.
    pub fn new(
        authority: PolicyAuthorityReference,
        version: PolicyVersion,
        rule_reference: PolicyRuleReference,
        effective_from: UtcTimestamp,
        effective_to: UtcTimestamp,
        content_reference: PolicyContentReference,
    ) -> SkynetResult<Self> {
        if !effective_from.is_before(effective_to) {
            return Err(SkynetError::InvalidTemporalWindow {
                kind: TemporalWindowKind::PolicyLineage,
                violation: TemporalWindowViolation::InvalidOrdering,
            });
        }

        Ok(Self {
            authority,
            version,
            rule_reference,
            effective_from,
            effective_to,
            content_reference,
        })
    }

    /// Returns the opaque reference to the governing policy authority.
    #[must_use]
    pub fn authority(&self) -> &PolicyAuthorityReference {
        &self.authority
    }

    /// Returns the governing policy version.
    #[must_use]
    pub fn version(&self) -> &PolicyVersion {
        &self.version
    }

    /// Returns the policy rule reference used for evaluation.
    #[must_use]
    pub fn rule_reference(&self) -> &PolicyRuleReference {
        &self.rule_reference
    }

    /// Returns the inclusive start of the lineage effective interval.
    #[must_use]
    pub const fn effective_from(&self) -> UtcTimestamp {
        self.effective_from
    }

    /// Returns the exclusive end of the lineage effective interval.
    #[must_use]
    pub const fn effective_to(&self) -> UtcTimestamp {
        self.effective_to
    }

    /// Returns the non-sensitive opaque content reference.
    #[must_use]
    pub fn content_reference(&self) -> &PolicyContentReference {
        &self.content_reference
    }

    /// Returns whether this policy lineage is effective at the evaluation time.
    #[must_use]
    pub const fn is_effective_at(&self, evaluation_time: UtcTimestamp) -> bool {
        !evaluation_time.is_before(self.effective_from)
            && evaluation_time.is_before(self.effective_to)
    }

    /// Validates the lineage effective interval at an evaluation time.
    pub fn validate_at(&self, evaluation_time: UtcTimestamp) -> SkynetResult<()> {
        if self.is_effective_at(evaluation_time) {
            return Ok(());
        }

        Err(SkynetError::InvalidTemporalWindow {
            kind: TemporalWindowKind::PolicyLineage,
            violation: TemporalWindowViolation::Expired,
        })
    }

    /// Returns whether this lineage matches an expected authority and version.
    #[must_use]
    pub fn matches_authority_and_version(
        &self,
        authority: &PolicyAuthorityReference,
        version: &PolicyVersion,
    ) -> bool {
        self.authority == *authority && self.version == *version
    }

    /// Validates expected authority and version bindings.
    pub fn validate_authority_and_version(
        &self,
        authority: &PolicyAuthorityReference,
        version: &PolicyVersion,
    ) -> SkynetResult<()> {
        if self.authority != *authority {
            return Err(SkynetError::PolicyLineageMismatch {
                field: PolicyLineageField::Authority,
                reason: PolicyLineageFailure::Mismatch,
            });
        }

        if self.version != *version {
            return Err(SkynetError::PolicyLineageMismatch {
                field: PolicyLineageField::Version,
                reason: PolicyLineageFailure::Mismatch,
            });
        }

        Ok(())
    }
}

impl CoreDomainResult for PolicyLineage {
    fn core_domain_category(&self) -> AllowedCoreDomainResultCategory {
        AllowedCoreDomainResultCategory::PolicyLineage
    }
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
