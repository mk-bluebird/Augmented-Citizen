# Skynet Crate Specification

Designed for: https://github.com/mk-bluebird/Augmented-Citizen

## 1. Purpose

`skynet` is a Rust 2024 crate for privacy-preserving augmented-citizen identity,
credential presentation, and infrastructure-network parameter governance.

The crate establishes a holder-controlled identity boundary for civic and
infrastructure interactions. It supports:

- Opaque augmented-citizen identity references.
- Holder-controlled verifiable credentials.
- Purpose-limited credential presentation requests.
- Minimal claim disclosure.
- Consent-governed infrastructure access decisions.
- Deployment-profile validation.
- Credential status and expiry evaluation.
- Content-minimized audit events.
- ALN-governed policy lineage.
- Formal verification of identity and disclosure invariants.

`skynet` does not:

- Store raw neural, EEG, BCI, physiological, or device-internal measurements.
- Infer identity from neural or physiological signals.
- Perform clinical diagnosis or treatment decisions.
- Establish identity proofing by itself.
- Issue credentials by itself.
- Retain credential claim values in ledger audit records.
- Connect automatically to municipal systems.
- Assume authority over any City of Phoenix service.
- Treat a deployment-region label as real-time location data.

## 2. Deployment Posture

The initial deployment label is:

```text
PHX_AZ_US
```

This label means:

```text
Phoenix, Arizona, United States deployment profile.
```

It is an application-defined locality reference only. It is not:

- A claim of municipal authorization.
- A municipal service credential.
- A continuous geolocation record.
- A street address.
- A proof of residency.
- A public identity label.

The deployment profile must be replaced or extended only through an ALN-governed
configuration change with documented infrastructure authority, policy version,
and consent requirements.

## 3. Architecture

```text
Credential issuer
        |
        v
Holder-controlled credential wallet
        |
        v
Skynet presentation request evaluator
        |
        +--> Consent scope
        +--> ALN policy projection
        +--> Deployment profile
        +--> Credential status provider
        +--> Claim disclosure minimizer
        |
        v
Presentation outcome
        |
        +--> Holder-local response
        +--> Approved verifier transport
        +--> Content-minimized audit event
```

### 3.1 Identity model

Skynet uses four distinct identity layers:

| Layer | Purpose | Prohibited content |
|---|---|---|
| `CitizenIdentityReference` | Opaque holder reference inside the local system | Name, raw biometrics, neural data |
| `CredentialReference` | Reference to a holder-controlled credential | Credential claim values in audit records |
| `VerifierReference` | Reference to an approved requesting service | Unverified service metadata |
| `DeploymentProfile` | Versioned locality and network-parameter profile | Continuous location history |

No layer may derive or expose neural, EEG, BCI, cognitive, subjective, or
physiological data.

### 3.2 Credential model

Skynet uses an issuer-holder-verifier model:

```text
Issuer
  -> signs credential
Holder
  -> stores credential and approves presentation
Verifier
  -> requests a minimal set of claims for a declared purpose
Skynet
  -> validates policy, consent, disclosure scope, credential status, and audit
```

The crate shall support credential formats through adapters. Initial standards
research must assess:

- W3C Verifiable Credentials Data Model 2.0.
- OpenID for Verifiable Presentations.
- ISO mdoc compatibility where applicable.
- SD-JWT VC compatibility where applicable.

The core crate shall not encode format-specific parsing, signature mechanics,
or remote transport behavior.

### 3.3 Network model

Skynet represents infrastructure interaction through a bounded network profile:

```text
DeploymentProfile
  -> NetworkParameterProfile
  -> VerifierPolicy
  -> PresentationRequest
  -> ConsentDecision
  -> PresentationOutcome
```

The core crate stores only:

- Versioned network-profile identifiers.
- Approved verifier references.
- Declared requested claim descriptors.
- Disclosure decision.
- Credential status result.
- Policy authority and version.
- Content-minimized audit metadata.

The core crate does not store:

- IP addresses.
- Packet captures.
- Network traffic payloads.
- Wireless identifiers.
- Continuous location traces.
- Device hardware serial numbers.
- Raw infrastructure telemetry.

## 4. Design Principles

- Holder control is required for every credential presentation.
- Consent is explicit, purpose-specific, and time-bounded.
- Requested claims must be minimized to the stated verifier purpose.
- Credential status must be evaluated before presentation.
- A credential reference never becomes a raw credential payload in audit data.
- Deployment profile is versioned and policy-bound.
- Neural and physiological data are outside all Skynet public contracts.
- Infrastructure interactions are disabled when policy lineage is incomplete.
- ALN policy is canonical for access, consent, disclosure, and audit rules.
- Every presentation outcome is auditable without retaining claim values.
- External transports and credential formats are implemented through ports.
- Formal properties are defined before adapter implementation.

## 5. Crate Layout

```text
crates/skynet/
├── Cargo.toml
├── README.md
├── docs/
│   ├── skynet-crate-specification.md
│   ├── architecture.md
│   ├── data-contracts.md
│   ├── wiring-plan.md
│   ├── privacy-model.md
│   ├── credential-profile-research.md
│   ├── deployment-profile-research.md
│   └── verifier-trust-research.md
├── aln/
│   └── skynet-civic-identity.v1.aln
├── src/
│   ├── lib.rs
│   ├── error.rs
│   ├── types.rs
│   ├── identity.rs
│   ├── deployment.rs
│   ├── network.rs
│   ├── credential.rs
│   ├── presentation.rs
│   ├── consent.rs
│   ├── policy.rs
│   ├── privacy.rs
│   ├── status.rs
│   ├── provenance.rs
│   ├── audit.rs
│   ├── ports.rs
│   ├── pipeline.rs
│   └── invariants.rs
├── tests/
│   ├── identity_contract_tests.rs
│   ├── credential_presentation_tests.rs
│   ├── consent_policy_tests.rs
│   ├── privacy_minimization_tests.rs
│   ├── deployment_profile_tests.rs
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

No source file is generated until its upstream contracts, required ports,
invariants, fixtures, and tests are defined.

## 6. Cargo Requirements

`crates/skynet/Cargo.toml` shall include:

```text
edition = "2024"
rust-version = "1.85"
kani-verifier = "0.67"
serde = { version = "1.0.203", features = ["derive"] }
serde_json = { version = "1.0.120" }
```

The manifest shall:

- Use repository metadata for `mk-bluebird/Augmented-Citizen`.
- Use workspace-managed versioning where the workspace provides it.
- Include no license field until repository licensing is explicitly confirmed.
- Keep default features empty.
- Add no credential transport, network, database, browser, or cloud dependency
  to the core crate.

## 7. Module Definitions

### `src/lib.rs`

Exports only stable public modules:

```text
audit
consent
credential
deployment
error
identity
invariants
network
pipeline
policy
ports
presentation
privacy
provenance
status
types
```

The library root forbids unsafe Rust and must not implement transport, storage,
or credential parsing.

### `src/error.rs`

Defines `SkynetError` with typed variants:

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

Error values may contain local diagnostics but must not contain credential
claims, identifiers, raw telemetry, or subjective content.

### `src/types.rs`

Defines:

```text
CitizenIdentityReference
CredentialReference
CredentialFormatReference
CredentialTypeReference
VerifierReference
VerifierPolicyReference
PresentationRequestId
PresentationOutcomeId
ConsentScopeId
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
UnitInterval
```

All identifiers are opaque and validated.

This file also defines closed enums:

```text
DeploymentRegion
  PHX_AZ_US
  Custom

CredentialStatus
  Active
  Expired
  Suspended
  Unavailable
  Unrecognized

PresentationStatus
  Approved
  Declined
  Unavailable
  Completed

ProcessingPurpose
  CivicIdentityVerification
  InfrastructureAccessVerification
  CredentialStatusCheck
  HolderLocalReview
  ResearchDerivedMetadataExport

ClaimDisclosureClass
  Required
  Optional
  Prohibited

AuditReasonCode
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
```

No type may contain raw credential claims, raw neural data, device internals,
free-text narrative, or generic payload fields.

### `src/identity.rs`

Defines holder-controlled identity reference validation.

Required types:

```text
CitizenIdentity
IdentityBinding
IdentityBindingStatus
IdentityReferenceValidation
```

Required functions:

```text
validate_citizen_identity_reference
validate_identity_binding
validate_identity_policy_lineage
```

Rules:

- Identity reference must be opaque.
- Identity binding must retain deployment and policy lineage.
- Identity binding must not contain raw credential claims.
- Identity binding must not contain neural, EEG, BCI, or physiological data.
- Identity binding must not create a direct city-service account claim.

### `src/deployment.rs`

Defines locality and infrastructure-network profile validation.

Required types:

```text
DeploymentProfile
DeploymentRegion
NetworkParameterProfile
NetworkProfileStatus
VerifierRegistryReference
```

A `PHX_AZ_US` profile must contain:

```text
deployment_profile_id
deployment_region
network_profile_id
policy_authority
policy_version
approved_verifier_registry_reference
created_at
```

A deployment profile must not contain real-time location, street address,
network traffic, or device serial data.

Required functions:

```text
validate_deployment_profile
validate_network_parameter_profile
validate_deployment_policy_lineage
```

### `src/network.rs`

Defines bounded infrastructure-network parameter contracts.

Required types:

```text
NetworkParameterProfile
NetworkAccessClass
NetworkSessionReference
NetworkEligibility
```

Permitted fields:

```text
network_profile_id
access_class
verifier_reference
policy_authority
policy_version
parameter_version
expires_at
```

Prohibited fields:

```text
ip_address
mac_address
packet_payload
radio_trace
wireless_scan
continuous_location
device_serial
neural_data
credential_claim_value
```

Required functions:

```text
validate_network_profile
validate_verifier_network_eligibility
```

### `src/credential.rs`

Defines credential references and credential status boundaries.

Required types:

```text
CredentialDescriptor
CredentialReference
CredentialFormatReference
CredentialStatusResult
CredentialStatus
CredentialUsageProfile
```

Required descriptor fields:

```text
credential_reference
credential_format_reference
credential_type_reference
issuer_reference
holder_reference
credential_status_reference
expires_at
policy_authority
policy_version
```

The credential descriptor must not embed credential claim values.

Required functions:

```text
validate_credential_descriptor
validate_credential_status
credential_is_usable
credential_matches_profile
```

### `src/presentation.rs`

Defines request and outcome contracts for credential presentation.

Required types:

```text
PresentationRequest
PresentationRequestItem
ClaimDescriptor
PresentationOutcome
PresentationDisclosure
```

A presentation request requires:

```text
presentation_request_id
holder_reference
verifier_reference
deployment_profile_id
purpose
requested_claim_descriptors
disclosure_profile_id
credential_reference
requested_at
expires_at
policy_authority
policy_version
```

A presentation outcome requires:

```text
presentation_outcome_id
presentation_request_id
status
credential_status
disclosed_claim_descriptor_ids
policy_decision_id
provenance_id
completed_at
```

The outcome records descriptor identifiers, not disclosed claim values.

Required functions:

```text
validate_presentation_request
validate_claim_minimization
validate_presentation_outcome
construct_approved_outcome
construct_declined_outcome
```

### `src/consent.rs`

Defines purpose-specific presentation consent.

Required types:

```text
ConsentScope
EffectiveScopeStatus
ConsentTemporalEvaluation
ConsentEvaluationRequest
ConsentEvaluation
```

Required purposes:

```text
CivicIdentityVerification
InfrastructureAccessVerification
CredentialStatusCheck
HolderLocalReview
ResearchDerivedMetadataExport
```

Rules:

- Present-time consent status is supplied by `PolicyProvider`.
- Consent is never inferred from credential presence, network metadata, or
  biosignal-derived material.
- Credential status checks may not authorize credential presentation.
- Infrastructure access verification may not authorize research metadata export.
- Research metadata export may never include raw credential claim values.

### `src/policy.rs`

Defines typed ALN policy projection and operation evaluation.

Required types:

```text
SkynetPolicy
PolicyRequirement
PolicyEvaluationContext
PolicyEvaluationResult
PolicyDisposition
```

Required policy fields:

```text
host_did = didalnorganic-host
bostrom_address = bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7
authority = ALN.MIGRATION.CYBERCORE_AUTHORITY.v1
required_identity_fields
required_credential_status
required_claim_descriptors
permitted_processing_purposes
approved_deployment_profiles
approved_verifier_registry
audit_content_minimized
neural_data_prohibited
raw_credential_claim_export_prohibited
```

Required functions:

```text
validate_policy
validate_requested_purpose
validate_verifier_reference
validate_deployment_profile
evaluate_presentation_policy
evaluate_research_metadata_export
```

### `src/privacy.rs`

Defines disclosure minimization and prohibited-data enforcement.

Required types:

```text
DisclosureProfile
ClaimDisclosureRule
PrivacyAssessment
ProhibitedDataClass
```

Required prohibited data classes:

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

Required functions:

```text
validate_disclosure_profile
validate_requested_claims
validate_prohibited_data_classes
validate_audit_minimization
```

The privacy module must prove that a presentation request contains only allowed
claim descriptors for its stated purpose.

### `src/status.rs`

Defines credential-status resolution outcomes.

Required types:

```text
CredentialStatusResult
CredentialStatus
StatusEvaluation
```

Required functions:

```text
validate_status_result
status_permits_presentation
status_requires_holder_review
```

Credential status must be resolved through `CredentialStatusProvider`.
The core crate does not fetch a remote status endpoint.

### `src/provenance.rs`

Defines reproducibility and policy lineage.

Required types:

```text
SkynetProvenance
PresentationProvenanceLink
ReproducibilityManifest
```

Required provenance fields:

```text
citizen_identity_reference
credential_reference
credential_format_reference
verifier_reference
deployment_profile_id
network_profile_id
disclosure_profile_id
policy_authority
policy_version
build_id
consent_scope_id
created_at
```

No provenance type stores claim values or raw credential payloads.

### `src/audit.rs`

Defines closed, content-minimized audit construction.

Required types:

```text
SkynetAuditEvent
AuditEventBuilder
AuditValidationReport
AuditReceipt
```

Permitted audit fields:

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

Prohibited audit fields:

```text
credential_claim_value
raw_credential
neural_data
eeg_data
bci_data
physiological_telemetry
continuous_location
network_payload
device_internal_state
free_text
direct_identity
clinical_data
```

### `src/ports.rs`

Defines every external integration boundary.

Required traits:

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

Required trait responsibilities:

| Trait | Input | Output |
|---|---|---|
| `CredentialWallet` | Presentation request | Holder-approved credential response reference |
| `CredentialStatusProvider` | Credential status reference | `CredentialStatusResult` |
| `PresentationTransport` | Approved presentation outcome | Transport receipt |
| `PolicyProvider` | Scope, purpose, deployment, verifier | ALN-derived policy projection |
| `VerifierRegistryProvider` | Verifier reference | Verifier authorization result |
| `DeploymentProfileProvider` | Deployment profile reference | `DeploymentProfile` |
| `AuditSink` | Closed audit event | Audit receipt |
| `LocalRecordStore` | Permitted local derived record | Local receipt |
| `ResearchMetadataExportSink` | Approved derived metadata export | Export receipt |
| `Clock` | Clock request | Validated timestamp |

No port trait accepts raw neural data, credential claim values, or generic
payload fields.

### `src/pipeline.rs`

The pipeline coordinates only typed ports.

Required sequence:

```text
1. Clock
2. DeploymentProfileProvider
3. VerifierRegistryProvider
4. PolicyProvider
5. Consent evaluation
6. CredentialWallet
7. CredentialStatusProvider
8. DisclosureProfile validation
9. Presentation validation
10. Invariant evaluation
11. PresentationTransport
12. AuditEvent construction
13. AuditSink
```

Required outcome rules:

| Condition | Presentation | Audit |
|---|---:|---:|
| Policy declined | No presentation | Required when policy lineage exists |
| Consent inactive | No presentation | Required |
| Verifier unrecognized | No presentation | Required |
| Credential unavailable | No presentation | Required |
| Credential status not usable | No presentation | Required |
| Claim request exceeds disclosure profile | No presentation | Required |
| All checks pass | Approved presentation | Required |

### `src/invariants.rs`

Defines pure checks:

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

Required invariants:

```text
No raw neural or physiological data in public contracts.
No credential claim values in audit contracts.
No presentation without active matching consent.
No presentation without accepted deployment profile.
No presentation without approved verifier.
No presentation with unavailable, expired, or suspended credential status.
No claim descriptor outside disclosure profile.
No research metadata export without independent consent scope.
Every audit event has policy authority, policy version, and provenance.
```

## 8. ALN Contract

**Filename:** `crates/skynet/aln/skynet-civic-identity.v1.aln`  
**Destination:** `https://github.com/mk-bluebird/Augmented-Citizen/tree/main/crates/skynet/aln/skynet-civic-identity.v1.aln`  
**Designed for:** `https://github.com/mk-bluebird/Augmented-Citizen`

The shard must contain non-empty:

```text
META
EPOCH
NEURORIGHTS
LEDGER
```

Required META bindings:

```text
authority = Organichain-contracts
repo_namespace = mk-bluebird/Cybercore
host_did = didalnorganic-host
bostrom_address = bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7
aln_authority = ALN.MIGRATION.CYBERCORE_AUTHORITY.v1
deployment_profile = PHX_AZ_US
```

Required NEURORIGHTS declarations:

```text
mental_privacy = true
cognitive_liberty = true
mental_integrity = true
non_commercial_neural = true
raw_neural_data_prohibited = true
raw_credential_claim_export_prohibited = true
audit_content_minimized = true
continuous_location_prohibited = true
```

Required LEDGER events:

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

Each ledger event records only opaque references, policy lineage, closed reason
codes, and timestamps.

## 9. Test Plan

### `tests/identity_contract_tests.rs`

Required checks:

- Identity references reject empty values.
- Identity binding requires deployment and policy lineage.
- Identity record contains no prohibited field.
- Deployment reference is opaque.
- Identity reference cannot be used as claim-value storage.

### `tests/credential_presentation_tests.rs`

Required checks:

- Valid request accepts only declared claim descriptors.
- Credential status Active permits a profile-compatible presentation.
- Credential status Expired declines presentation.
- Credential status Suspended declines presentation.
- Credential status Unavailable declines presentation.
- Unsupported credential format declines presentation.
- Verifier mismatch declines presentation.
- Disclosure profile mismatch declines presentation.

### `tests/consent_policy_tests.rs`

Required checks:

- Active consent with matching purpose permits policy evaluation.
- Expired, suspended, withdrawn, completed, and unavailable effective states
  decline presentation.
- Verifier purpose mismatch declines.
- Infrastructure identity verification does not permit research metadata export.
- Research metadata export requires independent scope.
- Policy authority mismatch declines.
- Policy version mismatch declines.

### `tests/privacy_minimization_tests.rs`

Required checks:

- Requested descriptor list is a subset of disclosure profile.
- Prohibited descriptor class declines presentation.
- Audit event contains no claim value field.
- Audit event contains no raw neural or physiological field.
- Presentation outcome lists descriptor identifiers only.
- Research metadata export excludes identity and claim values.

### `tests/deployment_profile_tests.rs`

Required checks:

- `PHX_AZ_US` profile requires policy lineage.
- Deployment profile lacks continuous location field.
- Deployment profile lacks device serial field.
- Unrecognized deployment profile declines infrastructure verification.
- Verifier registry reference is required.

### `tests/pipeline_tests.rs`

Required deterministic port doubles:

```text
FixedClock
FixedDeploymentProfileProvider
FixedVerifierRegistryProvider
FixedPolicyProvider
FixedCredentialWallet
FixedCredentialStatusProvider
MemoryPresentationTransport
MemoryAuditSink
MemoryLocalRecordStore
MemoryResearchMetadataExportSink
```

Required outcomes:

- Approved presentation produces transport and audit receipts.
- Declined consent produces audit receipt only.
- Unrecognized verifier produces audit receipt only.
- Credential status unavailable produces audit receipt only.
- Overbroad disclosure request produces audit receipt only.
- Research export is unreachable without independent scope.

## 10. Kani Plan

### `kani/identity_reference_proofs.rs`

Prove:

```text
Empty opaque identity reference is rejected.
Identity binding cannot omit deployment profile.
Identity binding cannot omit policy authority or version.
```

### `kani/claim_minimization_proofs.rs`

Prove:

```text
A requested claim descriptor absent from disclosure profile cannot be approved.
A prohibited claim class cannot be approved.
A disclosure profile with no allowed descriptors cannot approve a presentation.
```

### `kani/consent_scope_proofs.rs`

Prove:

```text
Inactive effective consent state cannot permit presentation.
Mismatched purpose cannot permit presentation.
Research export scope cannot be inferred from civic verification scope.
```

### `kani/credential_status_proofs.rs`

Prove:

```text
Only Active credential status can permit a presentation.
Unavailable status cannot permit a presentation.
Expired status cannot permit a presentation.
Suspended status cannot permit a presentation.
```

### `kani/audit_minimization_proofs.rs`

Prove:

```text
Audit event uses only its closed typed schema.
Audit event has no claim-value field.
Audit event has no neural or physiological field.
Audit event requires authority, version, purpose, and provenance.
```

### `kani/deployment_policy_proofs.rs`

Prove:

```text
Unrecognized deployment profile cannot permit presentation.
Verifier outside approved registry cannot permit presentation.
Deployment profile without policy lineage cannot permit presentation.
```

## 11. Fixture Plan

### `fixtures/policy/`

```text
active-civic-verification.json
expired-civic-verification.json
suspended-civic-verification.json
withdrawn-civic-verification.json
research-export-authorized.json
research-export-not-authorized.json
policy-authority-mismatch.json
policy-version-mismatch.json
```

### `fixtures/deployment/`

```text
phx-az-us-v1.json
unrecognized-deployment.json
missing-verifier-registry.json
missing-policy-lineage.json
```

### `fixtures/credential/`

```text
active-credential-reference.json
expired-credential-reference.json
suspended-credential-reference.json
unavailable-credential-reference.json
unsupported-format-reference.json
```

### `fixtures/presentation/`

```text
minimal-civic-verification.json
overbroad-claim-request.json
verifier-not-authorized.json
disclosure-profile-violation.json
approved-presentation.json
declined-presentation.json
```

### `fixtures/audit/`

```text
approved-presentation-audit.json
declined-presentation-audit.json
credential-unavailable-audit.json
disclosure-violation-audit.json
invalid-audit-prohibited-field.json
```

Fixtures contain only opaque references, closed enums, timestamps, and expected
outcomes. They contain no raw credentials, claims, neural data, location
history, or network payloads.

## 12. Research Gates Before Code

A research agent must produce these records before source implementation:

### Credential profile research

**Filename:** `crates/skynet/docs/credential-profile-research.md`

Determine:

- Selected credential formats.
- Holder-binding requirements.
- Issuer trust model.
- Credential status mechanism.
- Credential expiry behavior.
- Disclosure-profile representation.
- Claim descriptor vocabulary.
- Format adapter requirements.
- Interoperability requirements.

### Deployment profile research

**Filename:** `crates/skynet/docs/deployment-profile-research.md`

Determine:

- Exact meaning of `PHX_AZ_US`.
- Local infrastructure participation agreements.
- Approved verifier registry source.
- Network profile governance source.
- Deployment authority.
- Deployment-profile update process.
- Data retention and audit requirements.
- Whether any jurisdiction-specific policy is required.

### Verifier trust research

**Filename:** `crates/skynet/docs/verifier-trust-research.md`

Determine:

- Verifier enrollment process.
- Verifier reference format.
- Verifier policy reference.
- Allowed presentation purposes.
- Required claim descriptors per purpose.
- Credential-status expectations.
- Verifier audit obligations.
- Verifier removal and expiry process.

### Privacy and legal research

Determine:

- Consent notices and holder review requirements.
- Credential claim minimization requirements.
- Data retention requirements.
- Data deletion and access requirements.
- Privacy assessment requirements.
- Applicable Arizona, federal, and partner-organization policies.
- Evidence needed before any connection to a municipal service.

## 13. Completion Gates

No Skynet source file may be generated until:

- [ ] The Augmented-Citizen workspace structure is inspected.
- [ ] The crate package name and workspace membership convention are confirmed.
- [ ] Repository licensing is confirmed before a license field is added.
- [ ] The canonical ALN location is confirmed.
- [ ] Credential format research is complete.
- [ ] Credential status mechanism is selected.
- [ ] `PHX_AZ_US` deployment meaning is documented.
- [ ] Verifier trust model is documented.
- [ ] Claim descriptor vocabulary is approved.
- [ ] Disclosure profile semantics are approved.
- [ ] Consent-purpose matrix is approved.
- [ ] Audit schema is approved.
- [ ] Fixture schemas are approved.
- [ ] Kani proof targets are approved.
- [ ] No source contract includes raw neural, physiological, credential-claim,
      continuous-location, or network-payload data.

## 14. Reference Sources

- NIST SP 800-63-4 Digital Identity Guidelines:
  https://pages.nist.gov/800-63-4/
- W3C Verifiable Credentials Data Model v2.0:
  https://www.w3.org/TR/vc-data-model-2.0/
- OpenID for Verifiable Presentations:
  https://openid.net/specs/openid-4-verifiable-presentations-1_0.html
- NIST Privacy Framework:
  https://www.nist.gov/privacy-framework
- City of Phoenix Smart Cities:
  https://www.phoenix.gov/administration/departments/innovation/smart-cities.html
- City of Phoenix Information Technology Services:
  https://www.phoenix.gov/administration/departments/its.html
