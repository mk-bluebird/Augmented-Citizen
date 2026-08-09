//! Content-minimized audit contracts.
//!
//! Audit events are accountability records, not identity records, credential
//! stores, presentation stores, telemetry streams, or surveillance profiles.

use crate::{
    error::SkynetResult,
    privacy::{
        validate_audit_field_set,
        AllowedCoreDomainResultCategory,
        AuditField,
        CoreDomainResult,
        TransactionScopedReference,
        AUDIT_FIELD_ALLOW_LIST,
    },
    provenance::{
        PolicyContentReference,
        PolicyLineage,
    },
    types::{
        AuditEventId,
        PolicyAuthorityReference,
        PolicyRuleReference,
        PolicyVersion,
        UtcTimestamp,
    },
};

/// Closed actor role recorded without a direct actor identifier.
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
pub enum ActorRole {
    /// The holder-controlled local interaction boundary.
    Holder,
    /// The approved credential-requesting verifier role.
    Verifier,
    /// The policy authority role.
    PolicyAuthority,
    /// A reviewed adapter role.
    Adapter,
    /// The local Skynet policy-core role.
    SkynetCore,
}

/// Closed action code for a minimized audit event.
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
pub enum AuditAction {
    /// A presentation request entered policy evaluation.
    PresentationEvaluationRequested,
    /// Holder authorization was evaluated.
    HolderAuthorizationEvaluated,
    /// Credential status was evaluated by an adapter.
    CredentialStatusEvaluated,
    /// Disclosure conformance was evaluated.
    DisclosureConformanceEvaluated,
    /// Policy lineage was evaluated.
    PolicyLineageEvaluated,
    /// A final eligibility decision was produced.
    EligibilityDecisionProduced,
    /// A boundary-contract violation was rejected.
    BoundaryContractRejected,
    /// An audit-contract violation was rejected.
    AuditContractRejected,
}

/// Closed outcome code for a minimized audit event.
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
pub enum AuditOutcomeCode {
    /// The policy evaluation approved the requested interaction.
    Approved,
    /// The policy evaluation denied the requested interaction.
    Denied,
    /// Required status or policy evidence was unavailable.
    Unavailable,
    /// The credential profile, status mechanism, or policy was unrecognized.
    Unrecognized,
    /// A required invariant was violated.
    InvariantViolation,
}

/// Non-sensitive policy-lineage reference retained by an audit event.
///
/// This reference identifies the governing policy authority, version, rule,
/// and opaque content reference without retaining policy source text,
/// credential claims, holder information, or transport metadata.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct PolicyLineageReference {
    authority: PolicyAuthorityReference,
    version: PolicyVersion,
    rule_reference: PolicyRuleReference,
    content_reference: PolicyContentReference,
}

impl PolicyLineageReference {
    /// Creates a minimized audit reference from validated policy lineage.
    #[must_use]
    pub fn from_lineage(lineage: &PolicyLineage) -> Self {
        Self {
            authority: lineage.authority().clone(),
            version: lineage.version().clone(),
            rule_reference: lineage.rule_reference().clone(),
            content_reference: lineage.content_reference().clone(),
        }
    }

    /// Returns the opaque policy authority reference.
    #[must_use]
    pub fn authority(&self) -> &PolicyAuthorityReference {
        &self.authority
    }

    /// Returns the policy version reference.
    #[must_use]
    pub fn version(&self) -> &PolicyVersion {
        &self.version
    }

    /// Returns the policy rule reference.
    #[must_use]
    pub fn rule_reference(&self) -> &PolicyRuleReference {
        &self.rule_reference
    }

    /// Returns the opaque reviewed policy content reference.
    #[must_use]
    pub fn content_reference(&self) -> &PolicyContentReference {
        &self.content_reference
    }
}

/// Fixed-shape, content-minimized Skynet audit event.
///
/// The serialized form accepts no unknown fields. Every event consists only of
/// the canonical audit allow-list:
///
/// ```text
/// event_id
/// timestamp_utc
/// actor_role
/// action
/// outcome_code
/// transaction_reference
/// policy_lineage_reference
/// ```
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct AuditEvent {
    event_id: AuditEventId,
    timestamp_utc: UtcTimestamp,
    actor_role: ActorRole,
    action: AuditAction,
    outcome_code: AuditOutcomeCode,
    transaction_reference: TransactionScopedReference,
    policy_lineage_reference: PolicyLineageReference,
}

impl AuditEvent {
    /// Creates a content-minimized audit event.
    ///
    /// Construction validates the canonical audit allow-list before producing
    /// the event. No caller may add arbitrary metadata, free-text fields,
    /// credential data, claims, identifiers, routes, or location information.
    pub fn new(
        event_id: AuditEventId,
        timestamp_utc: UtcTimestamp,
        actor_role: ActorRole,
        action: AuditAction,
        outcome_code: AuditOutcomeCode,
        transaction_reference: TransactionScopedReference,
        policy_lineage_reference: PolicyLineageReference,
    ) -> SkynetResult<Self> {
        validate_audit_field_set(&AUDIT_FIELD_ALLOW_LIST)?;

        Ok(Self {
            event_id,
            timestamp_utc,
            actor_role,
            action,
            outcome_code,
            transaction_reference,
            policy_lineage_reference,
        })
    }

    /// Returns the transaction-scoped audit-event identifier.
    #[must_use]
    pub fn event_id(&self) -> &AuditEventId {
        &self.event_id
    }

    /// Returns the UTC timestamp assigned to the audit event.
    #[must_use]
    pub const fn timestamp_utc(&self) -> UtcTimestamp {
        self.timestamp_utc
    }

    /// Returns the closed actor role without returning an actor identity.
    #[must_use]
    pub const fn actor_role(&self) -> ActorRole {
        self.actor_role
    }

    /// Returns the closed action code.
    #[must_use]
    pub const fn action(&self) -> AuditAction {
        self.action
    }

    /// Returns the closed outcome code.
    #[must_use]
    pub const fn outcome_code(&self) -> AuditOutcomeCode {
        self.outcome_code
    }

    /// Returns the transaction-scoped opaque reference.
    #[must_use]
    pub fn transaction_reference(&self) -> &TransactionScopedReference {
        &self.transaction_reference
    }

    /// Returns the non-sensitive policy-lineage reference.
    #[must_use]
    pub fn policy_lineage_reference(&self) -> &PolicyLineageReference {
        &self.policy_lineage_reference
    }

    /// Returns the canonical field selection used by every audit event.
    #[must_use]
    pub const fn field_allow_list() -> &'static [AuditField; 7] {
        &AUDIT_FIELD_ALLOW_LIST
    }
}

impl CoreDomainResult for AuditEvent {
    fn core_domain_category(&self) -> AllowedCoreDomainResultCategory {
        AllowedCoreDomainResultCategory::AuditEvent
    }
}

/// Alias for the fixed-shape audit record emitted by the Skynet core.
pub type AuditRecord = AuditEvent;
