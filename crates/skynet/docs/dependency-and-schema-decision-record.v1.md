# Skynet Dependency and Schema Decision Record v1

## Decision

`crates/skynet` SHALL be a standalone domain crate.

It SHALL NOT depend on:

```text
ac-aln-core
augmented-id-guard
city-pass
brain-identity-core
brain-identity-kernel
city-eco-ledger
city-eco-ledger-sqlite
```

until a later adapter review approves a strictly typed, privacy-preserving
integration boundary.

## Rationale

The reviewed candidate crates expose one or more prohibited Skynet data classes:

```text
raw credential subject
credential claim value
credential identifier
subject DID
issuer DID
host DID
Bostrom address
proof content
public-key material
endpoint
network configuration
device state
clinical or neural context
free-text audit material
```

Skynet core contracts may contain only opaque references, closed enums,
timestamps, policy lineage, consent results, status results, and declared claim
descriptor identifiers.

## Approved Core Dependencies

The first Skynet manifest may inherit only these workspace dependencies after
license and verification-toolchain gates are resolved:

```text
serde
serde_json
thiserror
```

`serde_json` is permitted only for fixture serialization and deserialization.
No public core type may contain `serde_json::Value`.

No transport, HTTP, database, credential-format, wallet, ledger, device,
telemetry, browser, cloud, or ALN-parser dependency is permitted in the core
crate.

## AgeEligibility.v1 Projection

The existing age-policy shard is replaced conceptually by this internal model:

```text
AgeThreshold
  Over13
  Over16
  Over18
  Over21
  Over25

AgeEligibilityRequest
  presentation_request_id
  holder_reference
  verifier_reference
  deployment_profile_id
  purpose
  requested_threshold
  consent_scope_id
  disclosure_profile_id
  policy_authority
  policy_version
  requested_at
  expires_at

AgeEligibilityResult
  presentation_outcome_id
  presentation_request_id
  requested_threshold
  age_over_threshold
  credential_status
  policy_decision_id
  provenance_id
  completed_at
```

The sole approved semantic disclosure is:

```text
age_over_threshold = true
```

The following source-policy fields are excluded:

```text
credential_id
subject_did
issuer_did
expiry_timestamp
revocation_state
full_name
date_of_birth
address
photo_id_image
raw_eeg
raw_bci
raw_biometric_template
full_medical_record
```

Credential validity and status remain adapter-provider results. They are not
credential claims in the Skynet core.

## Trust Projection

Static issuer lists are prohibited in Skynet source and checked-in ALN policy.

Issuer and verifier eligibility require:

```text
approved verifier registry reference
approved issuer trust policy reference
policy authority
policy version
validity interval
deployment profile compatibility
```

The core retains opaque references to these results and never retains a direct
issuer or verifier credential payload.

## Consent Projection

An age presentation is permitted only when all independent checks succeed:

```text
deployment profile accepted
verifier authorized
purpose permitted
consent scope active
holder authorization current
credential status active
disclosure profile permits requested descriptor
policy lineage complete
provenance complete
```

A local safety mechanism may decline or suspend a request. It cannot create
holder authorization.

## Audit Projection

Skynet audit events use only:

```text
audit_event_id
event_time
presentation_request_id
presentation_outcome_id
holder_reference
verifier_reference
deployment_profile_id
processing_purpose
presentation_status
credential_status
reason_code
policy_authority
policy_version
provenance_id
```

Skynet audit events exclude:

```text
host_did
bostrom_address
credential_id
subject_did
issuer_did
client_ip_or_node_id
notes
channel_ids
clinical_indication
modulation_parameters
responsible_clinician_id
raw_credential
credential_claim_value
neural_data
physiological_data
continuous_location
network_payload
device_internal_state
free_text
```

## Blockers

No Skynet source file may be generated until:

- Root workspace manifest defects are corrected or explicitly isolated.
- Repository license terms are reconciled.
- The Kani toolchain version is reconciled.
- A Rust CI workflow is added or approved.
- Existing Skynet specification and wiring-plan documents are reconciled.
- City-pass contracts are reviewed.
- Consent-envelope and rights-scope ALN documents are reviewed.
