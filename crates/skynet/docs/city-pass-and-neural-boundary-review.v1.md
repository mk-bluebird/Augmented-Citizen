# CityPass and Neural Boundary Review v1

## Decision

`skynet` SHALL NOT depend on `city-pass` or on BCI-oriented policy modules.

`city-pass` may become a separately reviewed verifier integration after:

- Verifier enrollment is complete.
- A deployment profile authorizes the verifier.
- A presentation purpose is approved.
- A disclosure profile permits the requested eligibility descriptor.
- A holder authorization is current and request-bound.
- A transport adapter has been approved.

## CityPass Exclusions

The following CityPass concepts are excluded from Skynet core types:

```text
owner_did
owner_bostrom
city_id
route_corridor_id
pass_cap_id
tap_device_id
tap_location_bucket
max_taps
remaining_taps
eco_kwh_baseline_per_trip
eco_kwh_transit_per_trip
eco_savings_kwh
neurorights_ok
roh_ok
psychload_ok
abuse_suspected
abuse_event_id
```

Skynet may retain only an opaque verifier reference and an opaque
service-policy reference for a CityPass integration.

## CityPass Adapter Contract

A future downstream adapter may receive:

```text
ApprovedPresentationReference
  presentation_outcome_id
  verifier_reference
  transport_receipt_reference
  policy_decision_id
  provenance_id
```

The adapter may not request:

```text
host_did
bostrom_address
credential_id
subject_did
issuer_did
birth_date
age_band_history
transit_route
tap_location
tap_device
neural_data
physiological_data
clinical_data
```

## BCI Boundary

All BCI and neural-consent schemas are external to Skynet.

Skynet does not process:

```text
neural channel
cognitive channel
affective state
memory-adjacent state
meta-cognitive state
diagnostic data
therapeutic data
clinical indication
neural signal summary
pattern-level inference
host-provided cognitive report
```

A local host interface may create a typed `HolderAuthorization` after applying
its own approved consent and safety policy. Skynet receives only that typed
authorization result.

## Reusable Consent Semantics

The following semantic requirements are approved for Skynet:

```text
default posture = deny
purpose = explicit and enumerated
authorization = affirmative and current
renewal = explicit
expiry = deny after expiration
revocation = blocks future presentation
retention = policy-bounded and minimized
sharing = no sharing by default
```

## Security Projection

Skynet security controls protect:

```text
policy-lineage integrity
holder-authorization binding
verifier authorization
deployment-profile authorization
credential-status outcome
disclosure descriptor minimization
presentation transport authorization
content-minimized audit construction
```

Skynet does not protect or process raw neural, biometric, clinical, device, or
network data because those classes are prohibited from its contracts.

Cryptographic selection is delegated to approved wallet, transport, and audit
adapter profiles. The core crate stores only a typed profile reference when
required for provenance.

## Recovery Rule

Skynet recovery is forward-only.

When a policy, adapter, verifier, deployment profile, or status source fails:

1. Decline the presentation.
2. Emit a content-minimized audit event when lineage is available.
3. Isolate the affected adapter or configuration reference.
4. Preserve host rights and protected capability floors.
5. Require an explicit, versioned successor policy or adapter approval before
   future use.

No recovery path may silently reduce host protections or create a weaker
authorization requirement.
