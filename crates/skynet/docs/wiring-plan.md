# Skynet Wiring Plan

## Status

- Lifecycle: pre-implementation specification
- Source repository: `mk-bluebird/Augmented-Citizen`
- Crate path: `crates/skynet`
- Initial deployment-profile label: `PHX_AZ_US`
- External municipal integration: prohibited until separately authorized
- Network behavior in core crate: none
- Credential parsing in core crate: none
- Credential issuance in core crate: none
- Identity proofing in core crate: none
- Raw neural, EEG, BCI, physiological, device, credential-claim, location, and packet data: prohibited

## 1. System Boundary

`skynet` is a privacy-preserving civic identity-policy core. It evaluates whether a
holder-controlled credential presentation may proceed for a stated purpose in a
named deployment profile.

It SHALL coordinate typed decisions among:

1. A local holder-controlled wallet adapter.
2. A credential-status adapter.
3. A verifier-registry adapter.
4. A deployment-profile adapter.
5. An ALN-policy projection adapter.
6. A presentation transport adapter.
7. A content-minimized audit sink.

It SHALL NOT own, parse, persist, inspect, derive, infer, transmit, or audit:

- Credential claims or credential payloads.
- Raw verifiable presentations.
- Neural, EEG, BCI, physiological, clinical, or subjective data.
- Device serials, device-internal state, IP addresses, MAC addresses, radio data,
  packet payloads, wireless scans, or continuous location.
- Free-text narratives.
- Municipal accounts, municipal authority assertions, or infrastructure credentials.

## 2. Trust Model

```text
Issuer
  signs a credential
      |
Holder wallet
  retains credential payload and private material
  evaluates holder authorization locally
      |
Skynet core
  evaluates only opaque references, policy facts, consent result,
  status result, verifier authorization, and disclosure descriptors
      |
Transport adapter
  transmits a sealed presentation only after core approval
      |
Verifier
  receives a presentation addressed to that verifier
```

`skynet` is not a trust anchor for issuers, verifiers, or municipalities.
Trust decisions are supplied through validated policy and registry ports.

## 3. Binding Constants

The policy-projection layer SHALL require the following exact bindings when
BioPay or ALN-governed operations are enabled:

```text
host_did = didalnorganic-host
bostrom_address = bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7
aln_authority = ALN.MIGRATION.CYBERCORE_AUTHORITY.v1
```

These bindings identify governance lineage. They SHALL NOT be emitted into a
presentation, used as a public citizen identifier, or treated as a credential
claim.

## 4. Deployment Profile

`PHX_AZ_US` means only:

```text
Application-defined Phoenix, Arizona, United States deployment profile.
```

It SHALL NOT mean:

- City of Phoenix approval or partnership.
- Connection to a City of Phoenix system.
- Residency, physical presence, address, or real-time location.
- Authority to access civic infrastructure.
- A public or externally resolvable identity.

A deployment profile is accepted only if it provides:

```text
deployment_profile_id
deployment_region
network_profile_id
policy_authority
policy_version
approved_verifier_registry_reference
created_at
expires_at
```

No deployment-profile type may include a geographic coordinate, street address,
device identifier, network identifier, packet field, or telemetry field.

## 5. Repository Assembly

```text
crates/skynet/
├── Cargo.toml
├── README.md
├── aln/
│   └── skynet-civic-identity.v1.aln
├── docs/
│   ├── skynet-crate-specification.md
│   ├── wiring-plan.md
│   ├── architecture.md
│   ├── data-contracts.md
│   ├── privacy-model.md
│   ├── credential-profile-research.md
│   ├── deployment-profile-research.md
│   ├── verifier-trust-research.md
│   └── threat-model.md
├── src/
│   ├── lib.rs
│   ├── error.rs
│   ├── types.rs
│   ├── identity.rs
│   ├── deployment.rs
│   ├── network.rs
│   ├── credential.rs
│   ├── consent.rs
│   ├── privacy.rs
│   ├── status.rs
│   ├── policy.rs
│   ├── presentation.rs
│   ├── provenance.rs
│   ├── audit.rs
│   ├── ports.rs
│   ├── pipeline.rs
│   └── invariants.rs
├── tests/
│   ├── identity_contract_tests.rs
│   ├── deployment_profile_tests.rs
│   ├── credential_presentation_tests.rs
│   ├── consent_policy_tests.rs
│   ├── privacy_minimization_tests.rs
│   └── pipeline_tests.rs
├── kani/
│   ├── identity_reference_proofs.rs
│   ├── claim_minimization_proofs.rs
│   ├── consent_scope_proofs.rs
│   ├── credential_status_proofs.rs
│   ├── audit_minimization_proofs.rs
│   └── deployment_policy_proofs.rs
└── fixtures/
    ├── policy/
    ├── deployment/
    ├── credential/
    ├── presentation/
    └── audit/
```

## 6. Generation Order

No implementation file may be generated before its direct dependencies are
specified and approved.

| Order | Deliverable | Required before generation |
|---:|---|---|
| 1 | `docs/*-research.md` | Repository inspection, standards profile decisions, trust-model decisions |
| 2 | `docs/data-contracts.md` | Closed identifier grammar, time model, reason-code registry |
| 3 | `docs/privacy-model.md` | Prohibited-data matrix, retention policy, audit minimization rules |
| 4 | `aln/skynet-civic-identity.v1.aln` | Canonical repository ALN syntax confirmed |
| 5 | `Cargo.toml`, `README.md` | Workspace, license, package metadata, Kani convention confirmed |
| 6 | `src/error.rs`, `src/types.rs` | Data contracts approved |
| 7 | Pure validators | Types and policy semantics approved |
| 8 | `src/ports.rs` | All pure input and output contracts approved |
| 9 | `src/pipeline.rs` | Ports, state machine, audit rules approved |
| 10 | Fixtures | All public types and expected outcomes approved |
| 11 | Tests | Fixtures and invariants approved |
| 12 | Kani harnesses | Bounded representations and proof assumptions approved |

## 7. Core Types

Every identifier is a validated, non-empty opaque value. Identifiers SHALL NOT
be parsed for meaning after construction.

```text
CitizenIdentityReference
CredentialReference
CredentialFormatReference
CredentialTypeReference
IssuerReference
VerifierReference
VerifierPolicyReference
VerifierRegistryReference
PresentationRequestId
PresentationOutcomeId
PresentationCommitment
ConsentScopeId
HolderAuthorizationId
PolicyDecisionId
DeploymentProfileId
NetworkProfileId
ClaimDescriptorId
DisclosureProfileId
CredentialStatusReference
AuditEventId
ProvenanceId
BuildId
PolicyAuthority
PolicyVersion
UtcTimestamp
Milliseconds
```

Closed enums:

```text
DeploymentRegion:
  PHX_AZ_US
  Custom

CredentialStatus:
  Active
  Expired
  Suspended
  Unavailable
  Unrecognized

PresentationStatus:
  Approved
  Declined
  Unavailable
  Completed

ProcessingPurpose:
  CivicIdentityVerification
  InfrastructureAccessVerification
  CredentialStatusCheck
  HolderLocalReview
  ResearchDerivedMetadataExport

ConsentState:
  Active
  Expired
  Withdrawn
  Suspended
  Completed
  Unavailable

ClaimDisclosureClass:
  Required
  Optional
  Prohibited

AuditReasonCode:
  OperationAuthorized
  ConsentScopeInactive
  PurposeNotAuthorized
  ClaimNotAuthorized
  CredentialStatusUnavailable
  CredentialNotUsable
  VerifierNotAuthorized
  DeploymentProfileUnavailable
  PolicyVersionUnavailable
  ProvenanceIncomplete
  AuditValidationFailed
  InvalidRecordStructure
  TransportFailure
```

## 8. Module Contracts

### `src/lib.rs`

- Enables `#![forbid(unsafe_code)]`.
- Exposes stable modules only.
- Contains no networking, storage, credential parsing, cryptography, or transport logic.

### `src/error.rs`

Defines `SkynetError` with only typed, non-sensitive diagnostics:

```text
InvalidIdentifier
InvalidTimestamp
InvalidDeploymentProfile
InvalidNetworkParameter
InvalidConsentState
InvalidCredentialReference
CredentialStatusUnavailable
CredentialNotUsable
PresentationNotAuthorized
ClaimDisclosureNotAuthorized
VerifierNotAuthorized
PolicyLineageMissing
ProvenanceIncomplete
ProhibitedDataClass
AuditValidationFailed
PortFailure
InvariantFailure
```

### `src/identity.rs`

Defines:

```text
CitizenIdentity
IdentityBinding
IdentityBindingStatus
IdentityReferenceValidation
```

Enforces opaque identity references, deployment lineage, and policy lineage.
Identity types cannot carry direct identity, claims, biometric data, neural data,
or a city-service-account assertion.

### `src/deployment.rs`

Defines:

```text
DeploymentProfile
NetworkParameterProfile
NetworkProfileStatus
VerifierRegistryReference
```

Validates deployment profile structure and policy lineage. It does not look up
location, scan networks, or activate infrastructure access.

### `src/network.rs`

Defines:

```text
NetworkAccessClass
NetworkSessionReference
NetworkEligibility
```

Permitted fields are profile references, verifier reference, policy authority,
policy version, parameter version, and expiry. Network data fields are absent by
type design.

### `src/credential.rs`

Defines:

```text
CredentialDescriptor
CredentialStatusResult
CredentialUsageProfile
```

A descriptor contains only credential reference, format reference, type
reference, issuer reference, holder reference, status reference, expiry, and
policy lineage. It never contains credential claims or a raw credential.

### `src/consent.rs`

Defines:

```text
ConsentScope
EffectiveScopeStatus
ConsentTemporalEvaluation
ConsentEvaluationRequest
ConsentEvaluation
HolderAuthorization
```

A `HolderAuthorization` is an explicit local outcome. It contains a scope ID,
purpose, verifier reference, request ID, authorization ID, validity interval,
and policy lineage. It contains neither a biometric method nor a raw interaction
record.

### `src/privacy.rs`

Defines:

```text
DisclosureProfile
ClaimDisclosureRule
PrivacyAssessment
ProhibitedDataClass
```

Prohibited classes:

```text
RawNeuralData
RawEegData
RawBciData
PhysiologicalTelemetry
CredentialClaimValue
DirectIdentity
ContinuousLocation
NetworkPayload
DeviceInternalState
SubjectiveContent
ClinicalData
```

It proves that requested descriptor IDs are a permitted subset of the applicable
disclosure profile. It does not inspect claim values.

### `src/status.rs`

Defines:

```text
StatusEvaluation
```

Only `Active` can permit presentation. `Expired`, `Suspended`, `Unavailable`,
and `Unrecognized` cannot permit presentation.

### `src/policy.rs`

Defines:

```text
SkynetPolicy
PolicyRequirement
PolicyEvaluationContext
PolicyEvaluationResult
PolicyDisposition
```

The policy projection requires authority, version, approved deployment profiles,
approved verifier registry, permitted processing purposes, permitted claim
descriptors, content-minimized audit behavior, and prohibited-data declarations.

### `src/presentation.rs`

Defines:

```text
PresentationRequest
PresentationRequestItem
ClaimDescriptor
PresentationCommitment
PresentationOutcome
PresentationDisclosure
```

`PresentationCommitment` is opaque and verifier-bound. It is generated by a
wallet adapter and passed unchanged to a transport adapter. It is never decoded
by the core crate or copied to audit records.

### `src/provenance.rs`

Defines:

```text
SkynetProvenance
PresentationProvenanceLink
ReproducibilityManifest
```

Provenance records only opaque references, policy lineage, build ID, scope ID,
and timestamps.

### `src/audit.rs`

Defines:

```text
SkynetAuditEvent
AuditEventBuilder
AuditValidationReport
AuditReceipt
```

An audit event contains only:

```text
audit_event_id
event_time
presentation_request_id
presentation_outcome_id
holder_reference
verifier_reference
deployment_profile_id
purpose
presentation_status
credential_status
reason_code
policy_authority
policy_version
provenance_id
```

### `src/ports.rs`

Defines integration traits:

```text
CredentialWallet
CredentialStatusProvider
PresentationTransport
PolicyProvider
VerifierRegistryProvider
DeploymentProfileProvider
AuditSink
LocalRecordStore
ResearchMetadataExportSink
Clock
```

Port requirements:

| Port | Core input | Core output |
|---|---|---|
| `CredentialWallet` | Approved request context | Holder authorization and sealed presentation commitment |
| `CredentialStatusProvider` | Status reference | Typed status result |
| `PresentationTransport` | Approved sealed commitment | Transport receipt |
| `PolicyProvider` | Purpose, scope, verifier, deployment | Policy projection |
| `VerifierRegistryProvider` | Verifier reference | Typed authorization result |
| `DeploymentProfileProvider` | Deployment profile ID | Deployment profile |
| `AuditSink` | Closed audit event | Audit receipt |
| `LocalRecordStore` | Permitted derived local record | Local receipt |
| `ResearchMetadataExportSink` | Independently authorized derived metadata | Export receipt |
| `Clock` | Time request | Validated timestamp |

### `src/pipeline.rs`

Implements the sole orchestration sequence:

```text
1. Obtain validated time.
2. Resolve deployment profile.
3. Resolve verifier authorization.
4. Resolve ALN-derived policy projection.
5. Evaluate declared purpose.
6. Evaluate consent scope.
7. Request local holder authorization.
8. Resolve credential status.
9. Validate disclosure descriptor subset.
10. Validate all invariants.
11. Transport sealed presentation only if approved.
12. Construct closed audit event.
13. Write audit event.
```

A denial before step 11 emits an audit event when the minimum lineage necessary
to build one is available. It never invokes presentation transport.

### `src/invariants.rs`

Defines pure, deterministic checks:

```text
check_identity_invariants
check_deployment_invariants
check_network_invariants
check_credential_invariants
check_credential_status_invariants
check_consent_invariants
check_disclosure_invariants
check_presentation_invariants
check_provenance_invariants
check_audit_invariants
check_research_export_invariants
all_invariants_pass
```

## 9. Presentation State Machine

```text
Received
  -> DeploymentValidated
  -> VerifierValidated
  -> PolicyValidated
  -> ConsentValidated
  -> HolderAuthorized
  -> CredentialStatusValidated
  -> DisclosureValidated
  -> ApprovedForTransport
  -> Transported
  -> Audited

Any state
  -> Declined
  -> Audited
```

Approval requires every one of these facts:

```text
deployment accepted
verifier authorized
policy lineage complete
purpose allowed
consent active and purpose-matched
holder authorization current and request-bound
credential status active
requested descriptors allowed
provenance complete
audit event valid
```

## 10. ALN Projection Requirements

**File:** `aln/skynet-civic-identity.v1.aln`

The repository’s canonical ALN grammar must be confirmed before this file is
written. The resulting shard SHALL include non-empty sections that express:

```text
META
EPOCH
NEURORIGHTS
LEDGER
```

Required policy semantics:

```text
mental_privacy = true
cognitive_liberty = true
mental_integrity = true
raw_neural_data_prohibited = true
raw_credential_claim_export_prohibited = true
audit_content_minimized = true
continuous_location_prohibited = true
```

Required event semantics:

```text
SkynetPresentationRequested
SkynetPresentationApproved
SkynetPresentationDeclined
SkynetCredentialStatusUnavailable
SkynetDisclosureProfileViolation
SkynetVerifierNotAuthorized
SkynetDeploymentProfileUnavailable
SkynetAuditRecorded
```

Each event contains only opaque references, closed reason codes, policy lineage,
and timestamps.

## 11. Test and Proof Obligations

Tests must establish:

- Empty opaque references are rejected.
- Identity binding requires deployment and policy lineage.
- No public model admits prohibited data classes.
- Only Active status permits transport.
- Expired, Suspended, Unavailable, and Unrecognized statuses decline.
- Consent must be active, current, purpose-matched, verifier-bound, and request-bound.
- Requested descriptors must be within the approved disclosure profile.
- An infrastructure-verification scope cannot authorize research export.
- Every audit event omits claims, presentations, neural data, location, and network data.
- A declined request never invokes presentation transport.
- Every approved transport yields a content-minimized audit receipt.

Kani harnesses shall prove bounded forms of:

```text
empty-reference rejection
policy-lineage completeness
inactive-consent denial
purpose mismatch denial
status denial unless Active
descriptor-subset enforcement
prohibited-class denial
unrecognized-deployment denial
unapproved-verifier denial
audit-schema closure
```

## 12. Completion Gate

Implementation may begin only after all items are complete:

- [ ] Target repository workspace and crate membership inspected.
- [ ] Repository license confirmed.
- [ ] Cargo dependency and Kani convention confirmed.
- [ ] Canonical ALN grammar and directory confirmed.
- [ ] VC format profile selected.
- [ ] OpenID4VP profile selected or explicitly deferred.
- [ ] Credential-status mechanism selected.
- [ ] Verifier enrollment and removal process documented.
- [ ] `PHX_AZ_US` governance source and update authority documented.
- [ ] Claim-descriptor vocabulary approved.
- [ ] Consent-purpose matrix approved.
- [ ] Audit retention and deletion policy approved.
- [ ] Adapter threat model approved.
- [ ] Fixtures contain opaque references only.
- [ ] No contract type can encode prohibited data.
