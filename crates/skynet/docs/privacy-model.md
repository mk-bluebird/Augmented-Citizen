# Skynet Privacy Model

## Status

```text
artifact_type = research_gate
decision_status = privacy_requirements_defined_no_retention_selected
implementation_authority = none
raw_data_collection = prohibited
audit_content = minimized_only
```

## Purpose

This document defines the prohibited-data boundary, disclosure-minimization
rules, audit-minimization rules, and retention-accountability questions for
future Skynet implementation.

It does not select a credential format, status mechanism, protocol, retention
duration, or audit sink implementation. It does not authorize collection of any
prohibited data class.

## Fixed Skynet Privacy Boundary

The following data classes are prohibited in all Skynet public contracts,
including types, ports, fixtures, audit events, and provenance records:

```text
RawNeuralData
RawEegData
RawBciData
PhysiologicalTelemetry
ClinicalData
CredentialClaimValue
RawCredentialPayload
DirectIdentity (name, free-text identifier, government ID number in routine presentation)
ContinuousLocation
StreetAddress
RealTimeLocation
NetworkPayload (IP address, MAC address, packet capture, radio trace, wireless scan)
DeviceInternalState (serial number, hardware identifiers, internal telemetry)
SubjectiveContent (free-text narrative, inferred state)
```

Allowed in Skynet core only:

```text
opaque references (CitizenIdentityReference, CredentialReference, VerifierReference, etc.)
closed enums (DeploymentRegion, CredentialStatus, PresentationStatus, ProcessingPurpose, ClaimDisclosureClass, AuditReasonCode, ConsentState)
policy lineage (PolicyAuthority, PolicyVersion, host_did = didalnorganic-host, bostrom_address, aln_authority = ALN.MIGRATION.CYBERCORE_AUTHORITY.v1)
timestamps (UtcTimestamp)
descriptor identifiers (ClaimDescriptorId, DisclosureProfileId) - not descriptor values
reason codes and status results
provenance links (ProvenanceId, BuildId, ConsentScopeId)
```

## Disclosure Minimization

### Principle

Every presentation request must be evaluated as:

```text
requested_claim_descriptors ⊆ allowed_descriptors for purpose AND verifier
required_descriptors ⊆ requested_claim_descriptors
no descriptor with ClaimDisclosureClass = Prohibited may be requested
```

Skynet core validates descriptor IDs only. It never inspects claim values.

### Purpose limitation

| Purpose | Allowed to authorize disclosure? | Can authorize research export? | Evidence status |
|---|---|---|---|
| CivicIdentityVerification | OPEN - requires purpose matrix evidence | No - segregated | EVIDENCE_REQUIRED |
| InfrastructureAccessVerification | OPEN - requires purpose matrix evidence | No - segregated | EVIDENCE_REQUIRED |
| CredentialStatusCheck | No disclosure, status only | No | EVIDENCE_REQUIRED |
| HolderLocalReview | Local review only, no transport | No | EVIDENCE_REQUIRED |
| ResearchDerivedMetadataExport | Requires independent consent scope | Yes, but only derived minimized metadata, never raw claims | EVIDENCE_REQUIRED |

Infrastructure verification may not authorize research export. Civic verification may not authorize research export. Research export requires independent scope with explicit holder authorization.

### Holder authorization binding

Future holder authorization must be bound to:

```text
presentation_request_id
verifier_reference
purpose
validity interval (not_before, expires_at)
policy_authority
policy_version
consent_scope_id
```

No biometric, neural, behavioral, device, or inferred-state method may be used as
Skynet core authorization input. Authorization is explicit, not inferred.

## Audit Minimization

### Permitted audit fields (closed schema)

Per wiring-plan.md and skynet-crate-specification.md:

```text
audit_event_id
event_time
presentation_request_id
presentation_outcome_id
holder_reference (opaque)
verifier_reference (opaque)
deployment_profile_id (opaque)
purpose (closed enum)
presentation_status (closed enum)
credential_status (closed enum)
reason_code (closed enum)
policy_authority
policy_version
provenance_id
```

### Prohibited audit fields

```text
credential_claim_value
raw_credential
neural_data, eeg_data, bci_data
physiological_telemetry
continuous_location
network_payload
device_internal_state
free_text
direct_identity
clinical_data
```

Every audit event must have policy authority, policy version, and provenance.
An approved presentation and a declined presentation both produce a content-minimized
audit event when minimum lineage exists.

### Research questions for audit

```text
what is audit sink governance
what is retention basis and duration
what is deletion or expiry action
what is holder access process
what is correction process
what is incident-response process
what are verifier audit duties
how is audit integrity evidenced without claim values
```

All questions remain EVIDENCE_REQUIRED until signed retention policy source is evidenced.

## Retention and Accountability

Research questions (no duration selected):

```text
audit retention basis (legal, policy, contractual)
audit retention duration
deletion or expiry action
holder access process for their audit events
correction process
incident-response process
audit sink governance
verifier audit duties
```

No retention duration may be asserted until documented authority evidences it.
Retention basis must link to NIST Privacy Framework https://www.nist.gov/privacy-framework
or signed organizational policy.

## Required Evidence Matrix

| Research topic | Evidence source | Authority | Evidence status | Privacy impact | Open question |
|---|---|---|---|---|---|
| Prohibited data boundary | wiring-plan.md, data-contracts.md (to be approved) | Skynet spec | EVIDENCE_COLLECTED for prohibition list, OPEN for enforcement proof | EVIDENCE_REQUIRED - need Kani proofs that no public type can encode prohibited class | How to prove type-level prohibition with Kani after contracts exist? |
| Disclosure minimization | NIST SP 800-63-4 https://pages.nist.gov/800-63-4/, W3C VC 2.0 | Primary standards | OPEN | EVIDENCE_REQUIRED - purpose matrix not yet approved | Which ClaimDescriptorIds allowed per purpose per verifier? |
| Audit minimization | Skynet audit schema, NIST Privacy Framework | Skynet spec | EVIDENCE_COLLECTED for permitted field set, OPEN for retention governance | EVIDENCE_REQUIRED - retention basis not documented | What signed source governs audit sink? |
| Holder authorization | Skynet consent and policy contracts | Organizational policy required | EVIDENCE_REQUIRED | EVIDENCE_REQUIRED - binding requirements not yet evidenced with signed source | How is replay to different verifier prevented? |
| Retention | Organizational retention policy | Evidence source required | EVIDENCE_REQUIRED | EVIDENCE_REQUIRED | No retention policy evidenced in repository |

Allowed statuses: OPEN, EVIDENCE_REQUIRED, EVIDENCE_COLLECTED, DEFERRED, REJECTED_FOR_SKYNET

## Completion Criteria

This privacy model gate is complete only when:

- Prohibited-data matrix is documented with type-level enforcement plan.
- Disclosure-minimization rule (subset check) is documented.
- Purpose-limitation segregation is documented (infrastructure ≠ research export).
- Holder-authorization binding requirements are documented (request, verifier, purpose, interval, policy lineage).
- Audit permitted vs prohibited field sets are documented.
- Retention and accountability questions are documented with evidence source requirements.
- All unresolved privacy decisions are marked OPEN or EVIDENCE_REQUIRED with primary source linkage requirement.

Completion does not authorize retention duration, audit sink activation, or source generation.

## Decision Preconditions

| Future decision | Required predecessor evidence |
|---|---|
| Credential format | Privacy analysis per candidate, selective-disclosure analysis, holder-binding analysis, linkability analysis |
| Status mechanism | Verifier privacy impact, holder privacy impact, offline behavior, failure semantics |
| OpenID4VP use | Request integrity, verifier metadata privacy, DCQL mapping privacy impact |
| Verifier activation | Purpose matrix approved, allowed descriptors per verifier approved, audit duty documented, enrollment agreement signed |
| PHX_AZ_US activation | Retention policy signed, audit sink governance documented, prohibited field enforcement proved, policy authority evidenced |

No verifier or deployment activation may occur until privacy impact for allowed descriptors and audit minimization are EVIDENCE_COLLECTED.

## Fixed Bindings

```text
host_did = didalnorganic-host
bostrom_address = bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7
aln_authority = ALN.MIGRATION.CYBERCORE_AUTHORITY.v1
```

Policy-lineage inputs only. Not audit fields, not presentation fields.
