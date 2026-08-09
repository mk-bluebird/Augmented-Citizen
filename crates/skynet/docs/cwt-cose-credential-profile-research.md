# F6 CWT/COSE Constrained Credential Profile Research

## Status

```text
artifact_type = research_gate
format_family = F6
family_description = CWT/COSE constrained credential profile
selection_state = CONDITIONAL_SELECTION_PENDING_EVIDENCE
format_selected = false
credential_schema_selected = false
COSE_profile_selected = false
issuer_selected = false
verifier_selected = false
cryptographic_dependency_selected = false
source_generation = prohibited
adapter_generation = prohibited
deployment_activation = prohibited
```

## Purpose

This document researches whether a constrained CWT/COSE credential profile can
satisfy Skynet's opaque-reference, holder-controlled, purpose-limited, and
content-minimized design.

It does not select:

```text
credential schema
claim vocabulary
COSE profile
issuer
verifier
proof format
wallet
transport
cryptographic dependency
status mechanism
selective-disclosure mechanism
```

CWT is a compact CBOR claims container. A future CWT credential profile may use
a COSE security profile, but CWT alone is not a complete VC interoperability
profile. An exact profile identifier must be evidenced before any future adapter
decision.

## Primary Research Sources

```text
RFC 8392 — CBOR Web Token
RFC 8747 — Proof-of-Possession Key Semantics for CWTs
RFC 9052 — CBOR Object Signing and Encryption
W3C Verifiable Credentials Data Model v2.0
IETF SD-CWT tracking record
```

These sources are research inputs only. They do not constitute format selection,
issuer authorization, verifier authorization, or interoperability evidence.

## Fixed Core Boundary

Skynet core never receives:

```text
CWT CBOR bytes
credential payload
credential claims
credential identifiers
holder identifiers
subject identifiers
issuer identifiers
status locator values
proof material
public key material
signature material
COSE headers
network endpoint
transport payload
device data
location data
neural data
physiological data
clinical data
free-text narrative
unbounded payload
```

Skynet core may receive only:

```text
PolicyAuthority
PolicyVersion
PolicyLineageReference
CredentialTypeReference
CredentialFormatReference
CredentialReference
CredentialStatus
ClaimDescriptorId
DisclosureProfileId
HolderAuthorization
PresentationCommitmentReference
```

Every core-facing reference is opaque, validated, non-empty, bounded, and
non-semantic.

## Research Questions

### 1. Exact F6 profile identity

**Status:** `OPEN`

Determine:

```text
Which CWT credential profile is proposed?
Which COSE security profile, if any, is required?
Which claim vocabulary is formally defined?
Which credential-type representation is interoperable?
Which primary source defines the profile?
Which profile version is being evaluated?
```

Candidate research directions:

```text
CWT with proof-of-possession confirmation semantics
CWT with externally governed credential-type mapping
CWT with normalized validity claims
CWT with adapter-private status locator
CWT with independently specified selective-disclosure mechanism
```

No direction is selected.

### 2. Adapter-private CWT inputs

**Status:** `OPEN`

A future F6 adapter may need to process these adapter-private categories:

```text
sealed CWT CBOR input
credential claim set
credential type claim
issuer claim
subject claim
confirmation material
validity claims
credential identifier
adapter-private status locator
security header material
presentation proof material
```

The future adapter must demonstrate that none of these categories cross into:

```text
Skynet core types
Skynet ports
Skynet fixtures
Skynet provenance
Skynet audit events
Skynet error values
```

### 3. Core-facing adapter outputs

**Status:** `OPEN`

The future adapter may emit only:

```text
CredentialFormatReference
CredentialTypeReference
CredentialReference
CredentialStatus
ClaimDescriptorId set
DisclosureProfileId
HolderAuthorization
PresentationCommitmentReference
```

Rules:

```text
CredentialReference must not equal a raw credential identifier.
CredentialReference must not be derived in Skynet core.
CredentialTypeReference must not reveal claim values.
PresentationCommitmentReference must not contain presentation bytes.
PresentationCommitmentReference must not contain a transport payload.
```

The wallet adapter retains sealed presentation material. The transport adapter
resolves the commitment reference only after policy approval.

### 4. Credential type representation

**Status:** `OPEN`

Research options:

```text
externally governed type registry
adapter-private claim-to-reference mapping
COSE profile metadata mapping
private claim namespace with collision analysis
```

Required evidence:

```text
type source
type-version governance
collision behavior
issuer interoperability
verifier interoperability
mapping to CredentialTypeReference
privacy impact
adapter-only processing proof
```

Credential type must not be derived from:

```text
holder identifier
issuer identifier
network endpoint
device state
location
clinical state
biophysical state
```

### 5. Validity interval normalization

**Status:** `OPEN`

The adapter may evaluate CWT validity information and normalize it for Skynet
policy evaluation.

Research questions:

```text
How are not-before and expiry semantics represented?
How does the adapter use Clock-provided time?
How is a malformed validity interval handled?
How is an absent validity interval handled?
How does the adapter avoid passing raw CWT claim values into core?
```

Required normalization rule:

```text
Expired means validated current time is after normalized expiry.

Not-before failure means credential is not currently usable.
Its closed-status mapping remains OPEN pending data-contract approval.

Not-before failure must never be classified as Expired.
```

### 6. Status boundary

**Status:** `DEFERRED_TO_S6`

Skynet core never:

```text
dereferences a status locator
retrieves a status artifact
parses a status artifact
parses revocation material
parses suspension material
receives witness material
receives epoch material
```

A future `CredentialStatusProvider` adapter may resolve an adapter-private status
locator under approved privacy, freshness, and failure policy.

The adapter returns only:

```text
Active
Expired
Suspended
Unavailable
Unrecognized
```

Status-family research remains in:

```text
accumulator-status-research.md
```

### 7. Eligibility without claim-value exposure

**Status:** `OPEN`

Research question:

```text
Can the adapter evaluate a requested ClaimDescriptorId set against an approved
disclosure profile and produce a policy-compatible result without exposing
credential claim values to Skynet core or audit?
```

Required evidence:

```text
adapter-side evaluation model
claim-to-descriptor mapping
descriptor vocabulary governance
claim-value non-disclosure proof
audit non-disclosure proof
error non-disclosure proof
cross-verifier privacy analysis
```

If F6 cannot support this boundary, it must be marked:

```text
REJECTED_FOR_SKYNET
```

### 8. Selective disclosure

**Status:** `OPEN`

Research candidate:

```text
Independently specified CWT selective-disclosure mechanism.
```

No generic COSE behavior is presumed to provide selective disclosure.

Required evidence:

```text
primary specification and version
issuer support
holder wallet support
verifier support
disclosure minimization behavior
holder-binding interaction
linkability analysis
cross-verifier correlation analysis
offline behavior
failure behavior
adapter complexity
```

A full credential may be processed only inside a future adapter. Full credential
material must never enter Skynet core, audit, fixtures, provenance, or errors.

### 9. Holder binding

**Status:** `OPEN`

Proof-of-possession confirmation semantics may be relevant to F6 research, but
they do not by themselves establish the selected self-issued holder-authentication
profile.

The H5 research gate must determine how holder authorization is bound to:

```text
holder_authorization_id
presentation_request_id
verifier_reference
purpose
consent_scope_id
not_before
expires_at
policy_authority
policy_version
```

No DID, key, proof, challenge, credential, route, endpoint, or message content
may enter the resulting `HolderAuthorization`.

### 10. Unlinkability and correlation

**Status:** `OPEN`

Required question:

```text
Can F6 support verifier-specific and purpose-specific presentation without
disclosing a stable holder identifier, credential identifier, issuer identifier,
subject identifier, or reusable presentation identifier?
```

Research must assess whether any adapter-private material becomes a:

```text
StablePseudonymousIdentifier
CrossSessionCorrelationIdentifier
IssuerControlledIdentifier
VerifierControlledIdentifier
CredentialProofMaterial
PublicKeyMaterial
SignatureMaterial
```

If correlation cannot be mitigated, F6 must be:

```text
REJECTED_FOR_SKYNET
```

or require a separately approved privacy-impact record and independent holder
authorization.

### 11. Interoperability evidence

**Status:** `EVIDENCE_REQUIRED`

Before any F6 adapter design may be proposed, evidence must establish:

```text
at least one documented issuer-capable ecosystem
at least one documented verifier-capable ecosystem
documented CWT and COSE interoperability behavior
documented holder-binding behavior
documented failure behavior
documented offline verification constraints
test vector or equivalent reproducible evidence
```

No issuer or verifier is currently selected or supported.

### 12. Offline constraints

**Status:** `OPEN`

Research questions:

```text
How can an adapter validate issuer material without core network access?
How can an adapter validate status freshness without core network access?
What versioned policy artifact is required?
How is integrity and provenance independently evidenced?
What is the permitted freshness window?
How are stale artifacts handled?
How are unavailable artifacts handled?
```

Rules:

```text
Skynet core makes no network call.
Skynet core does not receive endpoint or routing data.
Offline behavior must not imply network reachability.
Unavailable evidence must never produce Active.
```

## Required Evidence Matrix

| Topic | Evidence source | Status | Privacy impact | Core-boundary impact | Open question |
|---|---|---|---|---|---|
| Exact CWT profile | Primary specification | OPEN | EVIDENCE_REQUIRED | EVIDENCE_REQUIRED | Which profile identifier is interoperable? |
| Type representation | Type-governance source | OPEN | EVIDENCE_REQUIRED | EVIDENCE_REQUIRED | How is type mapped opaquely? |
| Validity behavior | CWT and policy evidence | OPEN | EVIDENCE_REQUIRED | EVIDENCE_REQUIRED | How is not-before normalized? |
| Status integration | S6 research | DEFERRED | EVIDENCE_REQUIRED | EVIDENCE_REQUIRED | How is status located privately? |
| Eligibility isolation | Adapter-boundary analysis | EVIDENCE_REQUIRED | EVIDENCE_REQUIRED | EVIDENCE_REQUIRED | Can core avoid claim values entirely? |
| Selective disclosure | Primary mechanism source | OPEN | EVIDENCE_REQUIRED | EVIDENCE_REQUIRED | Does the mechanism preserve unlinkability? |
| Holder binding | H5 research | OPEN | EVIDENCE_REQUIRED | EVIDENCE_REQUIRED | How is replay prevented? |
| Correlation resistance | Privacy analysis | OPEN | EVIDENCE_REQUIRED | EVIDENCE_REQUIRED | Are repeat presentations linkable? |
| Interoperability | Reproducible ecosystem evidence | EVIDENCE_REQUIRED | EVIDENCE_REQUIRED | EVIDENCE_REQUIRED | Are issuer and verifier implementations available? |
| Offline verification | Policy and artifact evidence | OPEN | EVIDENCE_REQUIRED | EVIDENCE_REQUIRED | What freshness constraints apply? |

## Required Conclusion Format

```text
F6_PROFILE_IDENTIFIER = OPEN
F6_INTEROPERABILITY = OPEN
F6_PRIVACY_COMPATIBILITY = OPEN
F6_CORE_BOUNDARY_COMPATIBILITY = OPEN
F6_SELECTIVE_DISCLOSURE = OPEN
F6_HOLDER_BINDING = OPEN
F6_UNLINKABILITY = OPEN
F6_OFFLINE_BEHAVIOR = OPEN
F6_EVIDENCE_STATUS = EVIDENCE_REQUIRED
```

## Decision Preconditions

| Future decision | Required predecessor evidence |
|---|---|
| F6 adapter design | Exact profile identity, interoperability evidence, privacy analysis, core-boundary analysis, selective-disclosure analysis, H5 analysis, unlinkability analysis, offline and failure analysis, threat-model review |
| Combined F6 + S6 + P3 + H5 profile | F6 evidence collected, S6 freshness and witness lifecycle defined, P3 adapter-only transport evidence, H5 request-bound authorization evidence, no prohibited core data flow |

## Prohibited Actions

```text
No credential schema selection
No claim vocabulary selection
No COSE profile selection
No issuer selection
No verifier selection
No cryptographic dependency selection
No source generation
No adapter generation
No fixture generation
No ALN generation
No municipal or infrastructure claim
```

## Fixed Bindings

```text
host_did = didalnorganic-host
bostrom_address = bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7
aln_authority = ALN.MIGRATION.CYBERCORE_AUTHORITY.v1
```

The fixed host DID and Bostrom address are external ALN-governance bindings only.
Skynet core policy lineage contains only:

```text
PolicyAuthority
PolicyVersion
PolicyLineageReference
```
