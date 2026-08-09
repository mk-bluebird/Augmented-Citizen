# F6/S6/H5 Synthesis Correction Record

## Protocol Corrections

```text
CWT identifier claim: cti
JWT identifier claim: jti
```

CWT claims are profile-defined. F6 must explicitly classify each claim as:

```text
required
optional
prohibited
adapter-local
core-derived
```

`sub` must not be admitted by default because it may create a holder
correlator.

## F6 Evidence Ownership

```text
F6 adapter:
- CWT/COSE parsing and protection validation
- profile identifier/version validation
- issuer and audience validation, if selected
- expiry/not-before validation
- disclosure conformance

S6 adapter:
- normalized CredentialStatus

H5 adapter:
- request binding, proof verification, replay detection, holder authorization

Policy-authority adapter:
- PolicyLineage

P3 core:
- EligibilityDecision
- AuditEvent
```

## S6 Terminology

```text
Use: Bitstring Status List v1.0
Do not use: BSL as a normative profile label
```

A status bit has no fixed universal Skynet meaning. Its interpretation depends
on the selected status-purpose and S6 profile.

## P3 Outcomes

```text
Approved
Denied
Unavailable
Unrecognized
InvariantViolation
```

## Threat-Model Discipline

```text
Layer 1: Current implementation coverage, verified by tests and proofs.
Layer 2: Documented future risks, promoted only by a concrete trigger.
```

Skynet does not assume a decentralized identity, blockchain, DID, accumulator,
or municipal integration architecture unless a governance-selected profile
adopts one.
