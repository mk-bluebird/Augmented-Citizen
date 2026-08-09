//! Bounded Kani proofs for the Skynet content-minimized audit contract.
//!
//! Run with:
//!
//! ```text
//! cargo kani --harness proof_audit_constructor_accepts_only_bounded_contract
//! cargo kani --harness proof_canonical_audit_field_set_is_accepted
//! cargo kani --harness proof_incomplete_audit_field_set_is_rejected
//! cargo kani --harness proof_duplicate_audit_field_set_is_rejected
//! cargo kani --harness proof_audit_event_contains_only_transaction_scoped_reference
//! ```

use skynet::{
    audit::{
        ActorRole,
        AuditAction,
        AuditEvent,
        AuditOutcomeCode,
        PolicyLineageReference,
    },
    privacy::{
        validate_audit_field_set,
        AuditField,
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
        PresentationRequestId,
        UtcTimestamp,
    },
    SkynetResult,
};

const AUDIT_EVENT_ID: &str = "audit:event-01";
const PRESENTATION_REQUEST_ID: &str = "req:presentation-01";
const POLICY_AUTHORITY: &str = "polauth:skynet-policy-authority";
const POLICY_VERSION: &str = "polver:v1.0";
const POLICY_RULE_REFERENCE: &str = "rule:credential-eligibility";
const POLICY_CONTENT_REFERENCE: &str = "content:0123456789ABCDEFGHJKMNPQRS";

type BoundedAuditConstructor = fn(
    AuditEventId,
    UtcTimestamp,
    ActorRole,
    AuditAction,
    AuditOutcomeCode,
    TransactionScopedReference,
    PolicyLineageReference,
) -> SkynetResult<AuditEvent>;

fn audit_event_id() -> AuditEventId {
    AuditEventId::parse(AUDIT_EVENT_ID)
        .expect("fixed audit event identifier must be valid")
}

fn presentation_request_id() -> PresentationRequestId {
    PresentationRequestId::parse(PRESENTATION_REQUEST_ID)
        .expect("fixed presentation request identifier must be valid")
}

fn policy_authority() -> PolicyAuthorityReference {
    PolicyAuthorityReference::parse(POLICY_AUTHORITY)
        .expect("fixed policy authority reference must be valid")
}

fn policy_version() -> PolicyVersion {
    PolicyVersion::parse(POLICY_VERSION)
        .expect("fixed policy version reference must be valid")
}

fn policy_rule_reference() -> PolicyRuleReference {
    PolicyRuleReference::parse(POLICY_RULE_REFERENCE)
        .expect("fixed policy rule reference must be valid")
}

fn policy_content_reference() -> PolicyContentReference {
    PolicyContentReference::parse(POLICY_CONTENT_REFERENCE)
        .expect("fixed policy content reference must be valid")
}

fn policy_lineage() -> PolicyLineage {
    PolicyLineage::new(
        policy_authority(),
        policy_version(),
        policy_rule_reference(),
        UtcTimestamp::from_unix_seconds(1_700_000_000),
        UtcTimestamp::from_unix_seconds(1_800_000_000),
        policy_content_reference(),
    )
    .expect("fixed policy lineage must be valid")
}

fn policy_lineage_reference() -> PolicyLineageReference {
    PolicyLineageReference::from_lineage(&policy_lineage())
}

fn transaction_reference() -> TransactionScopedReference {
    TransactionScopedReference::PresentationRequest(presentation_request_id())
}

#[kani::proof]
fn proof_audit_constructor_accepts_only_bounded_contract() {
    let constructor: BoundedAuditConstructor = AuditEvent::new;
    let timestamp: i64 = kani::any();

    let result = constructor(
        audit_event_id(),
        UtcTimestamp::from_unix_seconds(timestamp),
        ActorRole::SkynetCore,
        AuditAction::EligibilityDecisionProduced,
        AuditOutcomeCode::Approved,
        transaction_reference(),
        policy_lineage_reference(),
    );

    kani::assert(
        result.is_ok(),
        "the constructor must accept exactly the bounded minimized audit contract",
    );
}

#[kani::proof]
fn proof_canonical_audit_field_set_is_accepted() {
    let result = validate_audit_field_set(&AUDIT_FIELD_ALLOW_LIST);

    kani::assert(
        result.is_ok(),
        "the canonical seven-field audit allow-list must validate",
    );
}

#[kani::proof]
fn proof_incomplete_audit_field_set_is_rejected() {
    let incomplete_fields = [
        AuditField::EventId,
        AuditField::TimestampUtc,
        AuditField::ActorRole,
        AuditField::ActionCode,
        AuditField::OutcomeCode,
        AuditField::TransactionReference,
    ];

    let result = validate_audit_field_set(&incomplete_fields);

    kani::assert(
        result.is_err(),
        "an audit field set without policy lineage must be rejected",
    );
}

#[kani::proof]
fn proof_duplicate_audit_field_set_is_rejected() {
    let duplicate_fields = [
        AuditField::EventId,
        AuditField::TimestampUtc,
        AuditField::ActorRole,
        AuditField::ActionCode,
        AuditField::OutcomeCode,
        AuditField::TransactionReference,
        AuditField::PolicyLineageReference,
        AuditField::OutcomeCode,
    ];

    let result = validate_audit_field_set(&duplicate_fields);

    kani::assert(
        result.is_err(),
        "an audit field set with duplicate fields must be rejected",
    );
}

#[kani::proof]
fn proof_audit_event_contains_only_transaction_scoped_reference() {
    let timestamp: i64 = kani::any();

    let event = AuditEvent::new(
        audit_event_id(),
        UtcTimestamp::from_unix_seconds(timestamp),
        ActorRole::Adapter,
        AuditAction::CredentialStatusEvaluated,
        AuditOutcomeCode::Unavailable,
        transaction_reference(),
        policy_lineage_reference(),
    )
    .expect("bounded audit contract must construct an audit event");

    kani::assert(
        event.transaction_reference().permitted_in_audit(),
        "the audit event may retain only a permitted transaction-scoped reference",
    );

    kani::assert(
        event
            .transaction_reference()
            .retention_classification()
            == skynet::privacy::RetentionClassification::TransactionScopedOnly,
        "audit transaction references must remain transaction-scoped only",
    );
}
