//! Purpose-bound, time-bounded, and revocable consent-scope contracts.

use core::fmt;

use crate::{
    error::{
        ConsentScopeFailure,
        PolicyLineageFailure,
        PolicyLineageField,
        SkynetError,
        SkynetResult,
        TemporalWindowKind,
        TemporalWindowViolation,
    },
    types::{
        ConsentScopeId,
        PolicyAuthorityReference,
        PolicyVersion,
        UtcTimestamp,
        VerifierReference,
    },
};

const MAX_PURPOSE_LENGTH: usize = 96;
const PURPOSE_PREFIX: &str = "purpose:";

/// Opaque, declared processing-purpose token.
///
/// A purpose token identifies the policy-declared reason for one credential
/// interaction. It must not contain credential claims, holder identifiers,
/// biometric data, neural data, device identifiers, wallet identifiers, routes,
/// addresses, or free-text narratives.
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[serde(transparent)]
pub struct ConsentPurpose(String);

impl ConsentPurpose {
    /// Maximum serialized length for a purpose token.
    pub const MAX_LENGTH: usize = MAX_PURPOSE_LENGTH;

    /// Required purpose-token prefix.
    pub const PREFIX: &'static str = PURPOSE_PREFIX;

    /// Parses a bounded, declared processing-purpose token.
    ///
    /// Valid values use the form:
    ///
    /// ```text
    /// purpose:<lowercase-policy-token>
    /// ```
    ///
    /// The suffix may contain lowercase ASCII letters, digits, hyphens,
    /// underscores, and dots.
    pub fn parse(value: impl AsRef<str>) -> SkynetResult<Self> {
        let value = value.as_ref();

        if value.is_empty()
            || value.len() > Self::MAX_LENGTH
            || !value.starts_with(Self::PREFIX)
        {
            return Err(SkynetError::InvalidConsentScope {
                reason: ConsentScopeFailure::MissingBinding,
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
            return Err(SkynetError::InvalidConsentScope {
                reason: ConsentScopeFailure::MissingBinding,
            });
        }

        Ok(Self(value.to_owned()))
    }

    /// Returns the validated purpose token.
    ///
    /// This value is a policy token, not a description of personal data.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<&str> for ConsentPurpose {
    type Error = SkynetError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl TryFrom<String> for ConsentPurpose {
    type Error = SkynetError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl fmt::Debug for ConsentPurpose {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("ConsentPurpose")
            .field(&"<policy-token>")
            .finish()
    }
}

/// Current lifecycle state for a consent scope.
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum ConsentState {
    /// The scope is available for validation within its declared time window.
    Active,
    /// The scope has been revoked and cannot authorize future use.
    Withdrawn,
}

/// Immutable record that revokes one consent scope.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ConsentWithdrawal {
    consent_scope_id: ConsentScopeId,
    withdrawn_at: UtcTimestamp,
}

impl ConsentWithdrawal {
    /// Creates a withdrawal record for one consent scope.
    #[must_use]
    pub fn new(
        consent_scope_id: ConsentScopeId,
        withdrawn_at: UtcTimestamp,
    ) -> Self {
        Self {
            consent_scope_id,
            withdrawn_at,
        }
    }

    /// Returns the opaque reference to the withdrawn consent scope.
    #[must_use]
    pub fn consent_scope_id(&self) -> &ConsentScopeId {
        &self.consent_scope_id
    }

    /// Returns the time at which withdrawal was recorded.
    #[must_use]
    pub const fn withdrawn_at(&self) -> UtcTimestamp {
        self.withdrawn_at
    }
}

/// Purpose-bound, time-bounded, revocable holder consent scope.
///
/// A consent scope does not prove holder identity, validate a credential,
/// validate a cryptographic presentation, reveal credential claims, or provide
/// a generic authorization for unrelated verifier interactions.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ConsentScope {
    id: ConsentScopeId,
    verifier: VerifierReference,
    purpose: ConsentPurpose,
    policy_authority: PolicyAuthorityReference,
    policy_version: PolicyVersion,
    not_before: UtcTimestamp,
    expires_at: UtcTimestamp,
    state: ConsentState,
}

impl ConsentScope {
    /// Creates an active, purpose-specific consent scope.
    ///
    /// The validity interval must have a strictly increasing start and end
    /// timestamp. The scope begins in the active state and may later be
    /// withdrawn through [`ConsentScope::withdraw`].
    pub fn new(
        id: ConsentScopeId,
        verifier: VerifierReference,
        purpose: ConsentPurpose,
        policy_authority: PolicyAuthorityReference,
        policy_version: PolicyVersion,
        not_before: UtcTimestamp,
        expires_at: UtcTimestamp,
    ) -> SkynetResult<Self> {
        if !not_before.is_before(expires_at) {
            return Err(SkynetError::InvalidTemporalWindow {
                kind: TemporalWindowKind::ConsentScope,
                violation: TemporalWindowViolation::InvalidOrdering,
            });
        }

        Ok(Self {
            id,
            verifier,
            purpose,
            policy_authority,
            policy_version,
            not_before,
            expires_at,
            state: ConsentState::Active,
        })
    }

    /// Returns the opaque consent-scope identifier.
    #[must_use]
    pub fn id(&self) -> &ConsentScopeId {
        &self.id
    }

    /// Returns the verifier reference bound to this scope.
    #[must_use]
    pub fn verifier(&self) -> &VerifierReference {
        &self.verifier
    }

    /// Returns the declared processing-purpose token.
    #[must_use]
    pub fn purpose(&self) -> &ConsentPurpose {
        &self.purpose
    }

    /// Returns the policy authority bound to this scope.
    #[must_use]
    pub fn policy_authority(&self) -> &PolicyAuthorityReference {
        &self.policy_authority
    }

    /// Returns the policy version bound to this scope.
    #[must_use]
    pub fn policy_version(&self) -> &PolicyVersion {
        &self.policy_version
    }

    /// Returns the inclusive start of the consent validity interval.
    #[must_use]
    pub const fn not_before(&self) -> UtcTimestamp {
        self.not_before
    }

    /// Returns the exclusive end of the consent validity interval.
    #[must_use]
    pub const fn expires_at(&self) -> UtcTimestamp {
        self.expires_at
    }

    /// Returns the current consent lifecycle state.
    #[must_use]
    pub const fn state(&self) -> ConsentState {
        self.state
    }

    /// Returns whether the consent scope has been withdrawn.
    #[must_use]
    pub const fn is_withdrawn(&self) -> bool {
        matches!(self.state, ConsentState::Withdrawn)
    }

    /// Returns a withdrawn copy of this consent scope.
    ///
    /// The withdrawal record must refer to this exact consent-scope identifier.
    /// Once withdrawn, the scope remains withdrawn and cannot be reactivated.
    pub fn withdraw(
        mut self,
        withdrawal: &ConsentWithdrawal,
    ) -> SkynetResult<Self> {
        if withdrawal.consent_scope_id() != &self.id {
            return Err(SkynetError::InvalidConsentScope {
                reason: ConsentScopeFailure::MissingBinding,
            });
        }

        self.state = ConsentState::Withdrawn;
        Ok(self)
    }

    /// Validates this consent scope for one policy evaluation context.
    ///
    /// Validation checks only opaque references, declared purpose, policy
    /// lineage references, lifecycle state, and temporal bounds.
    pub fn validate_for(
        &self,
        expected_scope_id: &ConsentScopeId,
        expected_verifier: &VerifierReference,
        expected_purpose: &ConsentPurpose,
        expected_policy_authority: &PolicyAuthorityReference,
        expected_policy_version: &PolicyVersion,
        evaluation_time: UtcTimestamp,
    ) -> SkynetResult<()> {
        if self.id != *expected_scope_id {
            return Err(SkynetError::InvalidConsentScope {
                reason: ConsentScopeFailure::MissingBinding,
            });
        }

        if self.state != ConsentState::Active {
            return Err(SkynetError::InvalidConsentScope {
                reason: ConsentScopeFailure::Withdrawn,
            });
        }

        if self.verifier != *expected_verifier {
            return Err(SkynetError::InvalidConsentScope {
                reason: ConsentScopeFailure::VerifierMismatch,
            });
        }

        if self.purpose != *expected_purpose {
            return Err(SkynetError::InvalidConsentScope {
                reason: ConsentScopeFailure::PurposeMismatch,
            });
        }

        if self.policy_authority != *expected_policy_authority {
            return Err(SkynetError::PolicyLineageMismatch {
                field: PolicyLineageField::Authority,
                reason: PolicyLineageFailure::Mismatch,
            });
        }

        if self.policy_version != *expected_policy_version {
            return Err(SkynetError::PolicyLineageMismatch {
                field: PolicyLineageField::Version,
                reason: PolicyLineageFailure::Mismatch,
            });
        }

        if evaluation_time.is_before(self.not_before) {
            return Err(SkynetError::InvalidTemporalWindow {
                kind: TemporalWindowKind::ConsentScope,
                violation: TemporalWindowViolation::NotYetValid,
            });
        }

        if !evaluation_time.is_before(self.expires_at) {
            return Err(SkynetError::InvalidTemporalWindow {
                kind: TemporalWindowKind::ConsentScope,
                violation: TemporalWindowViolation::Expired,
            });
        }

        Ok(())
    }
}
