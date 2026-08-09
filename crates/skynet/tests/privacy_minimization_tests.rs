use serde_json::{
    json,
    Value,
};

use skynet::{
    audit::{
        ActorRole,
        AuditAction,
        AuditEvent,
        AuditOutcomeCode,
        PolicyLineageReference,
    },
    consent::{
        ConsentPurpose,
        ConsentScope,
    },
    identity::construct_identity_reference,
    privacy::{
        has_minimized_audit_fields,
        AuditField,
        AUDIT_FIELD_ALLOW_LIST,
    },
    provenance::{
        PolicyContentReference,
        PolicyLineage,
    },
    status::CredentialStatus,
    types::{
        AuditEventId,
        ConsentScopeId,
        PolicyAuthorityReference,
        PolicyRuleReference,
        PolicyVersion,
        PresentationRequestId,
        UtcTimestamp,
        VerifierReference,
    },
    TransactionScopedReference,
};

const VALID_IDENTITY_REFERENCE: &str = "cit:0123456789ABCDEFGHJKMNPQRS";
const POLICY_AUTHORITY: &str = "polauth:skynet-policy-authority";
const POLICY_VERSION: &str = "polver:v1.0";
const POLICY_RULE: &str = "rule:credential-eligibility";
const POLICY_CONTENT: &str = "content:0123456789ABCDEFGHJKMNPQRS";
const CONSENT_SCOPE: &str = "consent:scope-01";
const VERIFIER: &str = "ver:approved-verifier-01";
const PURPOSE: &str = "purpose:infrastructure-access";
const PRESENTATION_REQUEST: &str = "req:presentation-01";
const AUDIT_EVENT: &str = "audit:event-01";

fn policy_authority() -> PolicyAuthorityReference {
    PolicyAuthorityReference::parse(POLICY_AUTHORITY)
        .expect("test policy authority reference must be valid")
}

fn policy_version() -> PolicyVersion {
    PolicyVersion::parse(POLICY_VERSION)
        .expect("test policy version reference must be valid")
}

fn policy_rule_reference() -> PolicyRuleReference {
    PolicyRuleReference::parse(POLICY_RULE)
        .expect("test policy rule reference must be valid")
}

fn policy_content_reference() -> PolicyContentReference {
    PolicyContentReference::parse(POLICY_CONTENT)
        .expect("test policy content reference must be valid")
}

fn consent_scope_id() -> ConsentScopeId {
    ConsentScopeId::parse(CONSENT_SCOPE)
        .expect("test consent scope identifier must be valid")
}

fn verifier_reference() -> VerifierReference {
    VerifierReference::parse(VERIFIER)
        .expect("test verifier reference must be valid")
}

fn consent_purpose() -> ConsentPurpose {
    ConsentPurpose::parse(PURPOSE)
        .expect("test consent purpose must be valid")
}

fn presentation_request_id() -> PresentationRequestId {
    PresentationRequestId::parse(PRESENTATION_REQUEST)
        .expect("test presentation request identifier must be valid")
}

fn audit_event_id() -> AuditEventId {
    AuditEventId::parse(AUDIT_EVENT)
        .expect("test audit event identifier must be valid")
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
    .expect("test policy lineage must be valid")
}

fn consent_scope() -> ConsentScope {
    ConsentScope::new(
        consent_scope_id(),
        verifier_reference(),
        consent_purpose(),
        policy_authority(),
        policy_version(),
        UtcTimestamp::from_unix_seconds(1_700_000_000),
        UtcTimestamp::from_unix_seconds(1_800_000_000),
    )
    .expect("test consent scope must be valid")
}

fn audit_event() -> AuditEvent {
    AuditEvent::new(
        audit_event_id(),
        UtcTimestamp::from_unix_seconds(1_750_000_000),
        ActorRole::SkynetCore,
        AuditAction::EligibilityDecisionProduced,
        AuditOutcomeCode::Approved,
        TransactionScopedReference::PresentationRequest(
            presentation_request_id(),
        ),
        PolicyLineageReference::from_lineage(&policy_lineage()),
    )
    .expect("test audit event must be valid")
}

fn assert_no_prohibited_object_keys(value: &Value) {
    const PROHIBITED_FIELD_MARKERS: [&str; 20] = [
        "name",
        "subject",
        "identity",
        "credential",
        "claim",
        "presentation",
        "public_key",
        "private_key",
        "wallet",
        "proof",
        "signature",
        "nonce",
        "route",
        "transport",
        "network",
        "location",
        "address",
        "biometric",
        "neural",
        "physiological",
    ];

    match value {
        Value::Object(fields) => {
            for (field_name, field_value) in fields {
                for prohibited_marker in PROHIBITED_FIELD_MARKERS {
                    assert!(
                        !field_name.contains(prohibited_marker),
                        "prohibited field marker found in serialized field name: {field_name}"
                    );
                }

                assert_no_prohibited_object_keys(field_value);
            }
        }
        Value::Array(values) => {
            for nested_value in values {
                assert_no_prohibited_object_keys(nested_value);
            }
        }
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => {}
    }
}

#[test]
fn identity_reference_serializes_as_a_scalar_not_an_identity_record() {
    let reference = construct_identity_reference(VALID_IDENTITY_REFERENCE)
        .expect("valid identity reference must construct");

    let serialized = serde_json::to_value(&reference)
        .expect("identity reference must serialize");

    assert_eq!(serialized.as_str(), Some(VALID_IDENTITY_REFERENCE));
    assert!(!serialized.is_object());
    assert!(!serialized.is_array());

    assert_no_prohibited_object_keys(&serialized);
}

#[test]
fn identity_reference_rejects_structured_identity_field_injection() {
    let injected = json!({
        "name": "prohibited",
        "subject": "prohibited",
        "neural_data": "prohibited"
    });

    let decoded = serde_json::from_value::<skynet::types::CitizenIdentityReference>(
        injected,
    );

    assert!(
        decoded.is_err(),
        "opaque identity references must not deserialize from structured identity records"
    );
}

#[test]
fn consent_scope_serialization_has_only_minimized_contract_fields() {
    let scope = consent_scope();

    let serialized = serde_json::to_value(&scope)
        .expect("consent scope must serialize");

    let fields = serialized
        .as_object()
        .expect("consent scope must serialize as an object");

    let expected_fields = [
        "id",
        "verifier",
        "purpose",
        "policy_authority",
        "policy_version",
        "not_before",
        "expires_at",
        "state",
    ];

    assert_eq!(fields.len(), expected_fields.len());

    for field in expected_fields {
        assert!(
            fields.contains_key(field),
            "missing expected minimized consent field: {field}"
        );
    }

    assert_no_prohibited_object_keys(&serialized);
}

#[test]
fn consent_scope_rejects_prohibited_field_injection() {
    let scope = consent_scope();

    let mut serialized = serde_json::to_value(&scope)
        .expect("consent scope must serialize");

    let fields = serialized
        .as_object_mut()
        .expect("consent scope must serialize as an object");

    fields.insert(
        "credential_claim".to_owned(),
        json!("prohibited"),
    );
    fields.insert(
        "holder_public_key".to_owned(),
        json!("prohibited"),
    );
    fields.insert(
        "neural_data".to_owned(),
        json!("prohibited"),
    );

    let decoded = serde_json::from_value::<ConsentScope>(serialized);

    assert!(
        decoded.is_err(),
        "consent scope deserialization must reject unrecognized prohibited fields"
    );
}

#[test]
fn closed_status_serialization_cannot_contain_claim_or_identity_fields() {
    for status in [
        CredentialStatus::Active,
        CredentialStatus::Expired,
        CredentialStatus::Suspended,
        CredentialStatus::Unavailable,
        CredentialStatus::Unrecognized,
    ] {
        let serialized = serde_json::to_value(status)
            .expect("closed status value must serialize");

        assert!(
            serialized.is_string(),
            "credential status must serialize as a closed scalar enum"
        );

        assert_no_prohibited_object_keys(&serialized);
    }
}

#[test]
fn credential_status_rejects_unrecognized_structured_input() {
    let decoded = serde_json::from_value::<CredentialStatus>(json!({
        "status": "Active",
        "credential_claim": "prohibited"
    }));

    assert!(
        decoded.is_err(),
        "credential status must not deserialize from a structured record"
    );
}

#[test]
fn audit_event_serialization_uses_only_the_canonical_allow_list() {
    let event = audit_event();

    assert!(has_minimized_audit_fields(&AUDIT_FIELD_ALLOW_LIST));

    let serialized = serde_json::to_value(&event)
        .expect("audit event must serialize");

    let fields = serialized
        .as_object()
        .expect("audit event must serialize as an object");

    let expected_fields = [
        "event_id",
        "timestamp_utc",
        "actor_role",
        "action",
        "outcome_code",
        "transaction_reference",
        "policy_lineage_reference",
    ];

    assert_eq!(fields.len(), expected_fields.len());

    for field in expected_fields {
        assert!(
            fields.contains_key(field),
            "missing expected minimized audit field: {field}"
        );
    }

    assert_no_prohibited_object_keys(&serialized);
}

#[test]
fn audit_event_rejects_prohibited_field_injection() {
    let event = audit_event();

    let mut serialized = serde_json::to_value(&event)
        .expect("audit event must serialize");

    let fields = serialized
        .as_object_mut()
        .expect("audit event must serialize as an object");

    fields.insert(
        "actor_identity".to_owned(),
        json!("prohibited"),
    );
    fields.insert(
        "credential_payload".to_owned(),
        json!("prohibited"),
    );
    fields.insert(
        "presentation_bytes".to_owned(),
        json!("prohibited"),
    );
    fields.insert(
        "network_route".to_owned(),
        json!("prohibited"),
    );
    fields.insert(
        "physical_location".to_owned(),
        json!("prohibited"),
    );

    let decoded = serde_json::from_value::<AuditEvent>(serialized);

    assert!(
        decoded.is_err(),
        "audit event deserialization must reject fields outside its minimized contract"
    );
}

#[test]
fn arbitrary_audit_field_selection_is_not_accepted_as_minimized() {
    let incomplete_field_set = [
        AuditField::EventId,
        AuditField::TimestampUtc,
        AuditField::OutcomeCode,
    ];

    assert!(
        !has_minimized_audit_fields(&incomplete_field_set),
        "audit records must include the complete canonical minimized field set"
    );
}
