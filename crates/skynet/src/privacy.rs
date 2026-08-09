//! Compile-time-facing privacy classifications and boundary contracts.
//!
//! This module defines the data categories that are prohibited from the Skynet
//! core, the result categories that may participate in core policy evaluation,
//! audit-field allow-lists, transaction-scoped reference rules, and no-retention
//! classifications.

use core::fmt;

use crate::{
    error::{
        AuditContractRule,
        SkynetError,
        SkynetResult,
    },
    types::{
        AuditEventId,
        DecisionReceiptId,
        PresentationRequestId,
    },
};

/// Categories of data prohibited from the Skynet core.
///
/// No public core-domain type, policy input, audit event, fixture, error
/// payload, or adapter-to-core value may contain these categories.
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum ProhibitedCoreDataCategory {
    /// Raw credential bytes, serialized credential objects, or format payloads.
    RawCredential,
    /// Credential claim values or disclosed claim-value maps.
    CredentialClaimValue,
    /// Raw credential-presentation bytes or presentation objects.
    RawPresentation,
    /// Direct holder identity, name, address, account, or subject identifier.
    DirectHolderIdentifier,
    /// Holder public key, wallet key, or key-agreement material.
    HolderKeyMaterial,
    /// Raw cryptographic proof, signature, or proof-of-possession artifact.
    CryptographicProofMaterial,
    /// Protocol challenge, nonce, session secret, or verifier challenge value.
    ProtocolChallengeMaterial,
    /// Network route, endpoint, packet payload, or transport metadata.
    NetworkOrTransportData,
    /// Real-time location, address, residence, or physical-presence data.
    LocationOrResidenceData,
    /// Device serial number, device-internal state, or hardware identifier.
    DeviceInternalData,
    /// Raw neural, EEG, BCI, or cognitive-state information.
    RawNeuralData,
    /// Raw biometric, physiological, clinical, or subjective information.
    RawPhysiologicalData,
    /// Free-text narrative capable of becoming an uncontrolled data channel.
    FreeTextNarrative,
}

/// Closed categories of minimized results permitted in the Skynet core domain.
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum AllowedCoreDomainResultCategory {
    /// Adapter-normalized credential lifecycle result.
    CredentialStatus,
    /// Request-bound holder authorization result.
    HolderAuthorization,
    /// Versioned authority and policy provenance result.
    PolicyLineage,
    /// Content-minimized disclosure conformance result.
    DisclosureReceipt,
    /// Core-generated policy outcome.
    EligibilityDecision,
    /// Core-generated content-minimized accountability event.
    AuditEvent,
}

/// Retention classification applied to a data category.
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum RetentionClassification {
    /// Data must not enter, persist in, or be emitted by the Skynet core.
    NoRetention,
    /// Data may exist only as a bounded transaction-scoped opaque reference.
    TransactionScopedOnly,
    /// Data may be retained only through the approved minimized audit contract.
    ContentMinimizedAuditOnly,
}

/// Returns the retention classification for prohibited core data.
///
/// Every prohibited category is classified as `NoRetention`.
#[must_use]
pub const fn classify_prohibited_data(
    _category: ProhibitedCoreDataCategory,
) -> RetentionClassification {
    RetentionClassification::NoRetention
}

/// Returns whether a data category is prohibited from the Skynet core.
#[must_use]
pub const fn is_prohibited_core_data(
    _category: ProhibitedCoreDataCategory,
) -> bool {
    true
}

/// Describes the approved fields that may occur in a content-minimized audit
/// event.
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum AuditField {
    /// Transaction-scoped audit-event identifier.
    EventId,
    /// UTC timestamp assigned to the event.
    TimestampUtc,
    /// Closed actor role without a direct actor identity.
    ActorRole,
    /// Closed action code.
    ActionCode,
    /// Closed outcome code.
    OutcomeCode,
    /// Transaction-scoped opaque reference.
    TransactionReference,
    /// Opaque policy-lineage reference.
    PolicyLineageReference,
}

/// Canonical allow-list of audit fields.
///
/// Audit events must contain no fields outside this list.
pub const AUDIT_FIELD_ALLOW_LIST: [AuditField; 7] = [
    AuditField::EventId,
    AuditField::TimestampUtc,
    AuditField::ActorRole,
    AuditField::ActionCode,
    AuditField::OutcomeCode,
    AuditField::TransactionReference,
    AuditField::PolicyLineageReference,
];

/// Returns whether an audit field belongs to the canonical allow-list.
#[must_use]
pub const fn is_allowed_audit_field(field: AuditField) -> bool {
    matches!(
        field,
        AuditField::EventId
            | AuditField::TimestampUtc
            | AuditField::ActorRole
            | AuditField::ActionCode
            | AuditField::OutcomeCode
            | AuditField::TransactionReference
            | AuditField::PolicyLineageReference
    )
}

/// Validates a selected set of audit fields against the canonical allow-list.
///
/// This function rejects duplicate field selections and rejects any field that
/// is not approved for content-minimized audit records.
pub fn validate_audit_field_set(fields: &[AuditField]) -> SkynetResult<()> {
    let mut event_id_seen = false;
    let mut timestamp_seen = false;
    let mut actor_role_seen = false;
    let mut action_seen = false;
    let mut outcome_seen = false;
    let mut transaction_reference_seen = false;
    let mut policy_lineage_seen = false;

    for field in fields {
        if !is_allowed_audit_field(*field) {
            return Err(SkynetError::AuditContractViolation {
                rule: AuditContractRule::FreeTextNarrativeProhibited,
            });
        }

        let already_seen = match field {
            AuditField::EventId => {
                let value = event_id_seen;
                event_id_seen = true;
                value
            }
            AuditField::TimestampUtc => {
                let value = timestamp_seen;
                timestamp_seen = true;
                value
            }
            AuditField::ActorRole => {
                let value = actor_role_seen;
                actor_role_seen = true;
                value
            }
            AuditField::ActionCode => {
                let value = action_seen;
                action_seen = true;
                value
            }
            AuditField::OutcomeCode => {
                let value = outcome_seen;
                outcome_seen = true;
                value
            }
            AuditField::TransactionReference => {
                let value = transaction_reference_seen;
                transaction_reference_seen = true;
                value
            }
            AuditField::PolicyLineageReference => {
                let value = policy_lineage_seen;
                policy_lineage_seen = true;
                value
            }
        };

        if already_seen {
            return Err(SkynetError::AuditContractViolation {
                rule: AuditContractRule::FreeTextNarrativeProhibited,
            });
        }
    }

    if !event_id_seen {
        return Err(SkynetError::AuditContractViolation {
            rule: AuditContractRule::MissingTransactionReference,
        });
    }

    if !timestamp_seen || !actor_role_seen || !action_seen || !outcome_seen {
        return Err(SkynetError::AuditContractViolation {
            rule: AuditContractRule::MissingOutcomeCode,
        });
    }

    if !transaction_reference_seen {
        return Err(SkynetError::AuditContractViolation {
            rule: AuditContractRule::MissingTransactionReference,
        });
    }

    if !policy_lineage_seen {
        return Err(SkynetError::AuditContractViolation {
            rule: AuditContractRule::MissingPolicyLineageReference,
        });
    }

    Ok(())
}

/// Opaque references permitted only within one presentation transaction.
///
/// These references must not be converted into stable holder, verifier,
/// credential, device, network, or location correlators.
#[derive(Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum TransactionScopedReference {
    /// Reference to the presentation request for this transaction.
    PresentationRequest(PresentationRequestId),
    /// Reference to the resulting eligibility decision.
    DecisionReceipt(DecisionReceiptId),
    /// Reference to the resulting audit event.
    AuditEvent(AuditEventId),
}

impl TransactionScopedReference {
    /// Returns the retention classification for every transaction-scoped
    /// reference.
    #[must_use]
    pub const fn retention_classification(
        &self,
    ) -> RetentionClassification {
        RetentionClassification::TransactionScopedOnly
    }

    /// Returns whether this reference may appear in a minimized audit event.
    #[must_use]
    pub const fn permitted_in_audit(&self) -> bool {
        true
    }
}

impl fmt::Debug for TransactionScopedReference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let kind = match self {
            Self::PresentationRequest(_) => "PresentationRequest",
            Self::DecisionReceipt(_) => "DecisionReceipt",
            Self::AuditEvent(_) => "AuditEvent",
        };

        formatter
            .debug_tuple("TransactionScopedReference")
            .field(&kind)
            .finish()
    }
}

/// Marker trait for a minimized type intentionally permitted in the core
/// policy domain.
///
/// Implementations belong only to reviewed Skynet core types. The marker does
/// not permit the implementing type to contain prohibited data.
pub trait CoreDomainResult {
    /// Returns the closed category represented by this minimized result.
    fn core_domain_category(&self) -> AllowedCoreDomainResultCategory;
}

/// Marker trait for types that may be used as a transaction-scoped reference.
///
/// Implementations must not expose stable holder, credential, verifier,
/// device, location, or transport correlators.
pub trait TransactionScoped {
    /// Returns the lifetime classification for the reference.
    fn retention_classification(&self) -> RetentionClassification;
}

impl TransactionScoped for TransactionScopedReference {
    fn retention_classification(&self) -> RetentionClassification {
        self.retention_classification()
    }
}
