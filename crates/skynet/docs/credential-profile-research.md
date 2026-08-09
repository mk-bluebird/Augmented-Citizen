# Skynet Credential Profile Research

## Status

```text
artifact_type = research_gate
decision_status = no_profile_selected
implementation_authority = none
credential_parsing_in_core = prohibited
credential_issuance_in_core = prohibited
```

## Purpose

This document defines the evidence required to choose a credential profile for
future Skynet adapters.

It does not select a credential format, wallet protocol, issuer, status
mechanism, transport, cryptographic suite, or credential provider.

## Fixed Skynet Boundary

The future credential profile must preserve these core rules:

```text
holder-controlled presentation
explicit holder authorization
purpose-limited disclosure
opaque core references
content-minimized audit
no raw credential in Skynet core
no credential claims in Skynet core
no direct holder identifier in routine presentation
no raw neural, physiological, clinical, device, network, or location data
```

## Research Questions

### Credential representation

Investigate, without selecting:

```text
W3C Verifiable Credentials Data Model 2.0
SD-JWT VC
ISO mdoc
other documented credential representations
```

For each candidate, collect:

```text
specification version
primary specification source
holder storage model
issuer signature model
credential type expression
expiry expression
status expression
selective disclosure capability
holder-binding capability
presentation proof behavior
offline verification behavior
adapter complexity
interoperability evidence
known privacy limitations
```

### Holder authorization

Determine:

```text
what proof binds a presentation to the current holder
whether holder authorization can be request-bound
whether holder authorization can be verifier-bound
whether holder authorization can be purpose-bound
whether holder authorization can be time-bounded
how replay to a different verifier is prevented
how revocation affects a pending presentation
```

No biometric, neural, behavioral, device, or inferred-state method may be used
as a Skynet core authorization input.

### Credential status

Investigate candidate status approaches without choosing one:

```text
status source type
offline availability
freshness behavior
expiry behavior
suspension behavior
unrecognized-status behavior
revocation propagation
verifier privacy impact
holder privacy impact
failure behavior
adapter trust assumptions
```

The future core result must normalize only to:

```text
Active
Expired
Suspended
Unavailable
Unrecognized
```

### Disclosure profile

Determine how a future adapter can map a protocol-specific request into:

```text
ClaimDescriptorId
CredentialTypeReference
CredentialFormatReference
DisclosureProfileId
ClaimDisclosureClass
```

The research must establish whether the adapter can produce a derived
eligibility result without passing claim values into Skynet core.

### Issuer and verifier trust

Determine:

```text
issuer trust-framework source
issuer enrollment process
issuer removal process
verifier enrollment process
verifier removal process
policy-version governance
trust-list publication process
trust-list expiry behavior
incident and compromise process
```

No static issuer or verifier allowlist may be embedded in Skynet source.

## Required Evidence Matrix

| Research topic | Candidate | Primary source | Evidence status | Privacy impact | Adapter impact | Open question |
|---|---|---|---|---|---|---|

Allowed evidence statuses:

```text
OPEN
EVIDENCE_REQUIRED
EVIDENCE_COLLECTED
DEFERRED
REJECTED_FOR_SKYNET
```

## Completion Criteria

This research gate is complete only when all of the following are documented:

- Candidate credential representations and primary standards sources.
- Holder-binding and replay-resistance requirements.
- Credential-status behavior and failure semantics.
- Disclosure-profile mapping requirements.
- Issuer and verifier trust-governance requirements.
- Adapter-only boundary for all format-specific processing.
- Explicit list of unresolved decisions.

Completion of this document does not authorize source generation. It enables the
next decision: whether a profile-selection record may be proposed.
