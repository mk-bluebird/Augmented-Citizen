//! Normalized credential-status domain model.
//!
//! This module does not parse credential formats, retrieve status information,
//! validate external status evidence, contact authorities, process status lists,
//! process accumulators, refresh witnesses, or perform network access.
//!
//! Reviewed external adapters normalize their evidence into `CredentialStatus`
//! before that evidence reaches the Skynet policy core.

use crate::privacy::{
    AllowedCoreDomainResultCategory,
    CoreDomainResult,
};

/// Normalized credential lifecycle status accepted by the Skynet policy core.
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
pub enum CredentialStatus {
    /// Current adapter-validated evidence indicates that the credential is
    /// active and within the applicable status-freshness policy.
    Active,

    /// The credential validity interval has ended.
    Expired,

    /// Current adapter-validated evidence indicates temporary suspension.
    Suspended,

    /// Status evidence is missing, stale, unverifiable, conflicting, or cannot
    /// be obtained within the applicable policy constraints.
    Unavailable,

    /// The credential profile, status mechanism, status purpose, or status
    /// evidence type is unsupported by the reviewed adapter.
    Unrecognized,
}

impl CredentialStatus {
    /// Returns whether this status is eligible to satisfy a policy rule that
    /// explicitly requires an active credential.
    #[must_use]
    pub const fn is_active(self) -> bool {
        matches!(self, Self::Active)
    }

    /// Returns whether this status must prevent an approval that requires an
    /// active credential.
    #[must_use]
    pub const fn prevents_active_approval(self) -> bool {
        !self.is_active()
    }

    /// Returns whether this status reflects a definite credential lifecycle
    /// state rather than insufficient or unsupported status evidence.
    #[must_use]
    pub const fn is_lifecycle_state(self) -> bool {
        matches!(self, Self::Active | Self::Expired | Self::Suspended)
    }

    /// Returns whether this status represents an evidence-availability issue.
    #[must_use]
    pub const fn is_evidence_unavailable(self) -> bool {
        matches!(self, Self::Unavailable)
    }

    /// Returns whether this status represents an unsupported or unknown status
    /// representation.
    #[must_use]
    pub const fn is_unrecognized(self) -> bool {
        matches!(self, Self::Unrecognized)
    }

    /// Returns a stable, closed code suitable for minimized audit output.
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::Active => "ACTIVE",
            Self::Expired => "EXPIRED",
            Self::Suspended => "SUSPENDED",
            Self::Unavailable => "UNAVAILABLE",
            Self::Unrecognized => "UNRECOGNIZED",
        }
    }
}

impl CoreDomainResult for CredentialStatus {
    fn core_domain_category(&self) -> AllowedCoreDomainResultCategory {
        AllowedCoreDomainResultCategory::CredentialStatus
    }
}
