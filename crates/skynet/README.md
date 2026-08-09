# Skynet

Skynet is a rights-oriented engineering crate for privacy-preserving identity,
consent, policy lineage, and accountable infrastructure interactions.

The crate treats the person as the primary sovereign stakeholder. It provides
typed contracts for cognitive liberty, mental privacy, identity continuity,
explicit consent, and narrowly scoped credential presentation.

Skynet does not collect, infer from, retain, or expose raw neural,
physiological, credential-claim, device-internal, network-payload, or
continuous-location data.

> **Important:** Skynet does not establish authorization by a government,
> municipality, device manufacturer, credential issuer, verifier, wallet, or
> infrastructure provider. Deployment labels and policy references remain
> application-defined until a documented, independently authorized integration
> exists.

## Purpose

`skynet` is a privacy-preserving civic identity and credential-routing core.

It evaluates whether a holder-authorized credential interaction may proceed
under a named deployment profile, a versioned policy, an approved verifier,
declared purpose, active consent scope, and credential-status result.

The core does not parse raw credentials, issue credentials, perform identity
proofing, validate raw cryptographic presentations, connect automatically to
external networks, or retain credential claims.

Credential formats, status mechanisms, wallet interactions, verifier
transport, registry lookup, and audit delivery are external adapter concerns.

## Design Principles

- Preserve cognitive sovereignty and mental privacy.
- Require explicit, purpose-specific, time-bounded, and revocable consent.
- Keep credential claims within a holder-controlled wallet or reviewed adapter.
- Use opaque references rather than direct personal identifiers.
- Minimize disclosure to the stated verifier purpose.
- Require complete policy authority and version lineage.
- Treat deployment labels as policy configuration, never as location evidence.
- Keep the policy core free from direct network, wallet, credential-format, and
  infrastructure dependencies.
- Permit safety and freedom improvements without silent capability reduction.
- Reject covert collection, behavioral manipulation, surveillance scoring, and
  non-consensual control mechanisms.

## Core Boundary

The Skynet policy core accepts only minimized, typed evidence from reviewed
adapters.

### Adapter-to-core evidence

```text
CredentialStatus
HolderAuthorization
PolicyLineage
DisclosureReceipt
```

### Core-generated outputs

```text
EligibilityDecision
AuditEvent
```

### Prohibited core data

The policy core must not receive, retain, serialize, or emit:

- Raw credential payloads.
- Credential claim values.
- Raw credential presentations.
- Holder names or direct identifiers.
- Holder public keys or wallet keys.
- Cryptographic proofs or challenges.
- Request nonce values.
- Verifier network routes or transport metadata.
- Raw neural, EEG, BCI, biometric, physiological, clinical, or subjective data.
- Continuous location, device serial numbers, network payloads, or telemetry.
- Free-text narratives in audit records.

## Identity Model

Skynet distinguishes four identity layers:

| Layer | Purpose | Excluded content |
|---|---|---|
| `CitizenIdentityReference` | Opaque local reference to a holder-controlled identity | Names, raw biometrics, neural data, direct identifiers |
| `CredentialReference` | Opaque reference to a holder-controlled credential | Credential claims and raw credential payloads |
| `VerifierReference` | Reference to an approved requesting service | Unverified verifier metadata |
| `DeploymentProfile` | Versioned policy-bound infrastructure profile | Real-time location, network traces, device serials |

Identity is not a behavioral score, diagnosis, surveillance record, or claim of
municipal authorization.

## Credential Interaction Model

A credential interaction may proceed only after required checks succeed:

1. Validate the named deployment profile.
2. Validate the requesting verifier against an approved registry.
3. Load a policy projection with complete authority and version lineage.
4. Validate the declared processing purpose.
5. Validate active, purpose-matched consent.
6. Obtain explicit holder authorization from a local trusted interface.
7. Evaluate normalized credential status through a status adapter.
8. Validate that the requested disclosure descriptors are policy-permitted.
9. Produce a core eligibility decision.
10. Authorize an external adapter to construct and transport an approved sealed presentation.
11. Record a content-minimized audit event.

```text
Credential issuer
        |
        v
Holder-controlled wallet
        |
        | holder authorization + sealed presentation commitment
        v
Skynet policy core
        |
        +--> deployment validation
        +--> verifier authorization
        +--> consent evaluation
        +--> credential-status evaluation
        +--> disclosure minimization
        +--> policy-lineage validation
        +--> invariant validation
        |
        v
Approved external transport adapter
        |
        v
Verifier
```

The audit sink receives only opaque transaction-scoped references, closed reason
codes, policy lineage, and timestamps.

## Status Semantics

External status adapters must normalize all outcomes to:

```text
Active
Expired
Suspended
Unavailable
Unrecognized
```

The policy core must not convert `Unavailable` or `Unrecognized` into `Active`.

A stale status result, unavailable status authority, invalid status evidence,
or conflicting valid authority publication produces `Unavailable`. An
unsupported credential profile or unsupported status mechanism produces
`Unrecognized`.

## Holder Authorization

A valid holder authorization is bound to:

```text
presentation_request_id
verifier_reference
purpose
consent_scope_id
not_before
expires_at
policy_authority
policy_version
freshness
```

Credential presence, device state, historical behavior, network context, or
status evidence alone do not constitute holder consent.

Raw challenges and nonce values are verified only by the appropriate adapter.
The policy core receives a minimized request-binding result, never protocol
challenge material.

## Policy Lineage

Every policy decision requires reproducible lineage:

```text
policy_authority
policy_version
policy_rule_reference
effective_from
effective_to
content_reference
```

A policy decision cannot be approved when policy lineage is missing, expired,
unrecognized, or inconsistent with holder authorization.

## Deployment Profiles

The initial deployment label is:

```text
PHX_AZ_US
```

This means only:

```text
Application-defined Phoenix, Arizona, United States deployment profile.
```

It does not establish City of Phoenix affiliation, municipal approval, service
access, residency, street address, real-time physical location, a city-service
account, or a live infrastructure connection.

A deployment profile may be accepted only when it has documented policy
authority, policy version, verifier-registry reference, expiry behavior, and
change-management requirements.

## Formal Invariants

```text
SKY-I-001: Policy evaluation receives no raw credential or claim values.
SKY-I-002: Policy evaluation receives no holder identifier, public key, proof,
           nonce, or transport route.
SKY-I-003: Unavailable and Unrecognized status evidence cannot be approved.
SKY-I-004: Holder authorization must match request, verifier, purpose, consent
           scope, policy version, freshness requirement, and validity interval.
SKY-I-005: A policy decision is reproducible from policy authority, version,
           rule reference, and effective interval.
SKY-I-006: Audit records do not become a secondary credential or surveillance store.
SKY-I-007: Deployment labels are not evidence of physical location or authority.
SKY-I-008: The core performs no implicit network access or hidden policy retrieval.
```

## Repository Role

This crate is intended to contain:

```text
src/
├── error.rs
├── types.rs
├── identity.rs
├── deployment.rs
├── credential.rs
├── consent.rs
├── privacy.rs
├── status.rs
├── policy.rs
├── presentation.rs
├── provenance.rs
├── audit.rs
├── ports.rs
├── pipeline.rs
└── invariants.rs
```

Not every listed module is immediately eligible for generation.

The initial safe implementation set consists of opaque types, error types,
identity references, deployment-profile validation, consent-scope validation,
privacy constraints, normalized status types, policy-lineage records,
content-minimized audit records, invariant checks, and their associated tests.

Credential-wire parsing, cryptographic presentation verification, external
status retrieval, transport, final policy evaluation, and pipeline orchestration
remain blocked until their upstream F6, S6, H5, and P3 contracts are selected.

## Engineering Requirements

Skynet uses:

```text
edition = "2024"
rust-version = "1.85"
```

Every host-critical module must provide:

- Narrow typed public APIs.
- Explicit prohibited-data boundaries.
- Deterministic validation and failure mapping.
- Closed enums for security-sensitive outcomes.
- Content-minimized audit contracts.
- Fixtures containing only opaque references and closed enum values.
- Formal-verification targets where privacy or state-machine invariants are
  safety-critical.
- External ports rather than embedded network, wallet, or credential-format
  dependencies.

No source artifact may be generated as a placeholder credential parser, mock
policy engine, simulated authorization authority, or hidden network client.

## Security Posture

Primary threats include:

- Unauthorized disclosure of credential claims.
- Unauthorized presentation of holder credentials.
- Verifier impersonation or verifier-policy mismatch.
- Overbroad disclosure requests.
- Replayed holder authorization.
- Cross-verifier or purpose substitution.
- Expiry bypass.
- Stale, suspended, expired, unavailable, or unrecognized credential status.
- Incomplete policy lineage.
- Unsupported deployment profiles.
- Adapter-to-core leakage.
- Audit records becoming a secondary surveillance store.
- Coercive, deceptive, or non-consensual interaction flows.

Skynet mitigates these threats through explicit authorization, minimized
disclosure, closed status semantics, typed policy evaluation, policy lineage,
external adapter ports, transaction-scoped references, and independently
testable invariants.

## Limitations

Skynet is an engineering framework. It is not legal advice, medical advice,
clinical-device validation, an identity-proofing service, a credential issuer,
or authorization from a municipality or infrastructure operator.

Actual protection depends on correct implementation, independent review,
holder-controlled deployment, trustworthy adapters, applicable law, and
documented agreements with credential issuers, verifiers, device makers, and
service operators.

## License

Licensed under either of:

- Apache License, Version 2.0.
- MIT License.

at the holder's option.
