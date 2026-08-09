# H5 Self-Issued Holder Binding Research

## Status

```text
artifact_type = research_gate
holder_binding_family = H5
family_description = self-issued holder authentication research
selection_state = CONDITIONAL_SELECTION_PENDING_EVIDENCE
holder_binding_selected = false
self_issued_protocol_selected = false
DID_method_selected = false
proof_format_selected = false
cryptographic_dependency_selected = false
source_generation = prohibited
adapter_generation = prohibited
deployment_activation = prohibited
```

## Purpose

This document researches whether a self-issued holder-authentication profile can
produce a replay-resistant, request-bound, verifier-bound, purpose-bound, and
time-bounded `HolderAuthorization` result for Skynet.

It does not select:

```text
self-issued protocol
identity provider
DID method
key format
proof format
wallet
credential format
presentation protocol
transport
issuer
verifier
cryptographic dependency
```

A self-issued authentication artifact is adapter-private. It must never become a
Skynet credential, policy input, audit field, provenance field, fixture value,
or public Rust type.

## Fixed Core Boundary

Skynet core never receives:

```text
DID value
self-issued identity token
self-issued claim
holder identifier
subject identifier
key material
proof material
challenge value
nonce value
request object
audience value
message header
message thread identifier
routing metadata
endpoint
transport payload
wallet handle
credential payload
credential claim
device information
biometric information
neural data
physiological data
clinical data
location data
free-text authorization narrative
```

Skynet core receives only a typed `HolderAuthorization` result:

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

Each field is required. No additional field may be added without data-contract,
privacy-model, threat-model, fixture, and proof approval.

## Mandatory Authorization Semantics

A future self-issued adapter must establish all of the following before it
returns `HolderAuthorization` to Skynet core:

```text
the authorization applies to exactly one presentation_request_id
the authorization applies to exactly one verifier_reference
the authorization applies to exactly one ProcessingPurpose
the authorization is linked to exactly one ConsentScopeId
the authorization has a bounded validity interval
the authorization is governed by PolicyAuthority and PolicyVersion
the authorization cannot be replayed for another request
the authorization cannot be redirected to another verifier
the authorization cannot be reused for another purpose
```

A successful self-issued authentication event alone is insufficient.

## Research Questions

### 1. Which self-issued profile is under consideration?

**Status:** `OPEN`

Research candidates:

```text
Self-Issued OpenID Provider v2 profile
self-issued identity interaction defined by a documented wallet ecosystem
self-issued authentication profile defined by a documented credential ecosystem
another primary-specification-based self-issued profile
```

No candidate is selected.

Required evidence:

```text
primary specification and version
profile maturity
holder-control model
identity assertion model
verifier trust assumptions
request-binding support
verifier-binding support
purpose-binding support
freshness behavior
expiry behavior
withdrawal behavior
offline behavior
interoperability evidence
privacy and correlation analysis
```

### 2. How is holder control demonstrated?

**Status:** `OPEN`

Determine:

```text
what adapter-private evidence demonstrates holder control
how the future profile distinguishes holder control from issuer authority
how proof material is validated without entering Skynet core
how the adapter avoids exposing direct holder identity
how the adapter avoids creating a cross-verifier correlation handle
```

The core must not infer authorization from:

```text
credential presence
wallet presence
device presence
network context
historical behavior
biometric input
neural input
physiological state
clinical state
inferred mental state
```

### 3. How is request binding established?

**Status:** `OPEN`

Determine:

```text
how adapter-private authorization evidence is bound to presentation_request_id
how the adapter rejects a mismatched request
how a duplicate request is identified
how a stale request is identified
how a substituted request is identified
how request binding is represented without exposing a request artifact to core
```

Required core rule:

```text
HolderAuthorization.presentation_request_id must equal
PresentationRequest.presentation_request_id.
```

### 4. How is verifier binding established?

**Status:** `OPEN`

Determine:

```text
how self-issued authorization is bound to verifier_reference
how an adapter validates verifier identity or registry correspondence
how cross-verifier redirection is prevented
how verifier substitution is detected
how verifier-specific presentation correlation is evaluated
```

Required core rule:

```text
HolderAuthorization.verifier_reference must equal
PresentationRequest.verifier_reference.
```

A direct verifier DID, endpoint, key, audience value, or message recipient is
adapter-private and prohibited from Skynet core.

### 5. How is purpose binding established?

**Status:** `OPEN`

Determine:

```text
how self-issued authorization is bound to ProcessingPurpose
how a purpose substitution attempt is detected
how the adapter prevents civic verification authorization from enabling
infrastructure access verification
how the adapter prevents civic or infrastructure verification from enabling
research export
```

Required core rule:

```text
HolderAuthorization.purpose must equal PresentationRequest.purpose.
```

Research export remains independently scoped.

### 6. How are freshness and replay resistance established?

**Status:** `OPEN`

Determine:

```text
what adapter-private freshness mechanism is used
how authorization reuse is detected
how duplicate delivery is detected
how expired authorization is detected
how replay to another verifier is detected
how replay to another purpose is detected
how replay is prevented offline
how replay evidence is retained without creating a correlation database
```

Required rule:

```text
An authorization artifact valid for one request must not authorize any other
request, verifier, purpose, or validity interval.
```

The future adapter must fail closed when freshness or replay evidence cannot be
validated.

### 7. How are validity intervals handled?

**Status:** `OPEN`

Determine:

```text
how not_before is established
how expires_at is established
how Clock input is used
how clock disagreement is handled
how malformed or missing interval data is handled
how a future policy bounds maximum authorization lifetime
```

Required rule:

```text
Authorization outside its validity interval cannot permit presentation.
```

### 8. How is consent scope bound?

**Status:** `OPEN`

Determine:

```text
how self-issued authorization references ConsentScopeId
how the adapter confirms scope is active
how withdrawn consent is detected
how completed, expired, suspended, or unavailable scope is handled
how scope is prevented from being reused across unrelated purposes
```

Required core rule:

```text
HolderAuthorization.consent_scope_id must identify an active, matching,
purpose-compatible scope.
```

### 9. How is withdrawal or revocation handled?

**Status:** `OPEN`

Determine:

```text
how a holder withdraws authorization
how a future adapter recognizes withdrawal
how pending presentations are affected
how already-approved but not yet transported presentations are affected
how withdrawal interacts with presentation expiry
how withdrawal is represented without exposing direct identity or proof material
```

Required rule:

```text
Withdrawal or unavailable withdrawal evidence must never permit a new
presentation.
```

### 10. How is cross-verifier correlation prevented?

**Status:** `OPEN`

Research questions:

```text
Does the self-issued profile expose a stable holder identifier?
Does it expose a stable subject identifier?
Does it expose a reusable holder key identifier?
Does it expose a reusable authorization identifier?
Can two verifiers correlate presentations?
Can a verifier correlate multiple purposes?
Can a transport intermediary correlate holder activity?
```

The research must assess whether any adapter-private artifact becomes:

```text
StablePseudonymousIdentifier
CrossSessionCorrelationIdentifier
IssuerControlledIdentifier
VerifierControlledIdentifier
PublicKeyMaterial
SignatureMaterial
CredentialProofMaterial
WalletHandle
```

Unmitigated correlation requires:

```text
REJECTED_FOR_SKYNET
```

or a separately approved privacy-impact record and independently authorized
holder consent.

### 11. How is the result normalized?

**Status:** `OPEN`

The future adapter may return only one of these outcomes:

```text
HolderAuthorization returned
authorization unavailable
authorization declined
authorization expired
authorization withdrawn
authorization provenance incomplete
```

The exact mapping to Skynet error and audit reason codes remains subject to the
data-contract and threat-model gates.

The core must never receive:

```text
why a holder declined
how a holder authenticated
what proof was evaluated
what message was exchanged
what key was used
what identity was asserted
```

### 12. How does H5 compose with F6, S6, and P3?

**Status:** `OPEN`

Determine:

```text
how self-issued authorization relates to F6 holder confirmation material
how S6 status failure affects holder authorization
how P3 messages carry adapter-private authorization exchange
how P3 routing and thread metadata remains adapter-private
how F6, S6, P3, and H5 failures are normalized without prohibited data leakage
```

Rules:

```text
H5 does not make F6 credential format selection.
H5 does not make S6 status mechanism selection.
H5 does not make P3 transport selection.
H5 does not authorize external transport by itself.
```

High-risk external transport remains subject to an explicit non-LLM approval
path outside Skynet core.

## Failure Rules

```text
missing request binding -> decline
mismatched verifier binding -> decline
mismatched purpose binding -> decline
inactive consent scope -> decline
expired authorization -> decline
withdrawn authorization -> decline
unavailable freshness evidence -> decline
replay evidence unavailable -> decline
authorization provenance incomplete -> decline
```

No failure may be upgraded to an approved authorization through fallback,
implicit consent, credential presence, network context, or cached identity.

## Adapter Boundary

Adapter-private only:

```text
self-issued identity artifact
holder DID
subject identifier
key material
proof material
challenge
nonce
audience
request object
response object
message headers
message thread
message route
transport receipt
wallet state
credential state
```

Allowed core result:

```text
HolderAuthorization
```

Allowed audit facts:

```text
presentation_request_id
presentation_outcome_id
verifier_reference
purpose
presentation_status
reason_code
policy_authority
policy_version
provenance_id
event_time
```

## Required Evidence Matrix

| Topic | Evidence source | Status | Privacy impact | Core-boundary impact | Open question |
|---|---|---|---|---|---|
| Self-issued profile | Primary specification | OPEN | EVIDENCE_REQUIRED | EVIDENCE_REQUIRED | Which profile supports all nine bindings? |
| Holder control | Profile and wallet evidence | OPEN | EVIDENCE_REQUIRED | EVIDENCE_REQUIRED | Can proof remain adapter-private? |
| Request binding | Replay-resistance evidence | OPEN | EVIDENCE_REQUIRED | EVIDENCE_REQUIRED | How is replay blocked? |
| Verifier binding | Verifier-binding evidence | OPEN | EVIDENCE_REQUIRED | EVIDENCE_REQUIRED | How is redirection blocked? |
| Purpose binding | Purpose-matrix evidence | OPEN | EVIDENCE_REQUIRED | EVIDENCE_REQUIRED | How is purpose substitution blocked? |
| Consent scope | Consent-policy evidence | OPEN | EVIDENCE_REQUIRED | EVIDENCE_REQUIRED | How is withdrawal propagated? |
| Validity interval | Profile and Clock-policy evidence | OPEN | EVIDENCE_REQUIRED | EVIDENCE_REQUIRED | How is time disagreement handled? |
| Correlation resistance | Privacy analysis | OPEN | EVIDENCE_REQUIRED | EVIDENCE_REQUIRED | Can two verifiers link presentations? |
| Failure mapping | Threat-model and contract evidence | OPEN | EVIDENCE_REQUIRED | EVIDENCE_REQUIRED | Which closed reason codes apply? |
| F6/S6/P3 composition | Cross-profile evidence | OPEN | EVIDENCE_REQUIRED | EVIDENCE_REQUIRED | Can all boundaries remain opaque? |

## Required Conclusion Format

```text
H5_PROFILE_IDENTIFIER = OPEN
H5_HOLDER_CONTROL = OPEN
H5_REQUEST_BINDING = OPEN
H5_VERIFIER_BINDING = OPEN
H5_PURPOSE_BINDING = OPEN
H5_CONSENT_SCOPE_BINDING = OPEN
H5_REPLAY_RESISTANCE = OPEN
H5_VALIDITY_INTERVAL = OPEN
H5_WITHDRAWAL_BEHAVIOR = OPEN
H5_UNLINKABILITY = OPEN
H5_F6_S6_P3_COMPOSITION = OPEN
H5_CORE_BOUNDARY_COMPATIBILITY = OPEN
H5_EVIDENCE_STATUS = EVIDENCE_REQUIRED
```

## Decision Preconditions

| Future decision | Required predecessor evidence |
|---|---|
| H5 adapter design | Exact profile, holder-control evidence, nine binding proofs, replay analysis, correlation analysis, withdrawal behavior, failure semantics, core-boundary analysis, threat-model review |
| Combined F6 + S6 + P3 + H5 profile | H5 evidence collected, request/verifier/purpose/interval binding proven, independent consent scope proven, no direct identity in core, F6/S6/P3 evidence collected |

## Prohibited Actions

```text
No self-issued profile selection
No DID method selection
No key selection
No proof selection
No wallet selection
No presentation-protocol implementation
No credential issuance
No source generation
No adapter generation
No fixture generation
No ALN generation
No verifier activation
No deployment activation
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
