# H5, P3, and Threat-Model Corrections

## H5 Scope

H5 establishes context-bound holder authorization. It does not establish
identity proofing, issuer attestation, legal consent validity, or clinical
authorization.

## Mandatory H5 Bindings

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

## Adapter/Core Separation

```text
Adapter-local:
- raw nonce and challenge
- proof and signature material
- holder key material
- credential and presentation payloads
- replay cache and consumed-request state
- wallet and transport details

Core-visible:
- RequestBinding::Bound | Missing | Invalid | Replayed
- HolderAuthorization with the nine mandatory context bindings
- normalized status
- policy lineage
- disclosure receipt
```

## Withdrawal Rule

Holder authorization is not non-revocable. It is context-bound, time-bounded,
and subject to withdrawal through the governing consent and authorization
lifecycle.

## P3 Closed Outcomes

```text
Approved
Denied
Unavailable
Unrecognized
InvariantViolation
```

## P3 Policy Material

Policy material must be authenticated and lineage-verifiable by the selected
governance mechanism before conversion to a bounded immutable typed Rust IR.

P3 itself must remain:

```text
local
deterministic
network-free
dynamic-code-free
raw-data-free
```

## Layer 2 Promotion Rule

Every theoretical threat requires:

```text
asset
assumption
mitigation
residual risk
promotion trigger
future test or proof obligation
```

No threat becomes an implementation requirement until it maps to a concrete
Skynet port, adapter, type, state transition, policy input, or output.
