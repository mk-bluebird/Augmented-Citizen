# H5 Holder-Binding Evidence Request

## Mission

Collect read-only evidence needed to evaluate whether a self-issued holder
authentication profile can produce a Skynet-compatible `HolderAuthorization`.

Do not select a protocol. Do not write source code. Do not implement an adapter.
Do not access keys, wallet data, credential payloads, personal records, neural
data, clinical data, telemetry, or external services.

## Required Evidence per Candidate Profile

For each candidate self-issued profile, return:

```text
profile name
specification version
specification maturity
primary specification source
holder-control model
verifier trust assumptions
request-binding mechanism
verifier-binding mechanism
purpose-binding mechanism
consent-scope binding mechanism
freshness behavior
replay-resistance behavior
validity-interval behavior
withdrawal behavior
offline behavior
message or transport dependency
privacy and correlation risk
adapter-private inputs
proposed normalized HolderAuthorization output
unresolved risks
```

## Required Nine-Binding Assessment

For every candidate, answer separately:

```text
Can it bind holder_authorization_id?
Can it bind presentation_request_id?
Can it bind verifier_reference?
Can it bind purpose?
Can it bind consent_scope_id?
Can it bind not_before?
Can it bind expires_at?
Can it bind policy_authority?
Can it bind policy_version?
```

For each answer, provide one of:

```text
EVIDENCE_COLLECTED
EVIDENCE_REQUIRED
OPEN
REJECTED_FOR_SKYNET
```

## Required Threat Analysis

Evaluate:

```text
request replay
duplicate delivery
verifier substitution
purpose substitution
consent-scope substitution
expiry bypass
clock disagreement
withdrawal race
cross-verifier correlation
cross-purpose correlation
stable subject identifier exposure
stable key identifier exposure
message-thread correlation
routing-metadata correlation
adapter-to-core prohibited-data leakage
```

## Required Core-Boundary Check

Verify that the proposed adapter can return only:

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

Reject the candidate for Skynet if it requires any of these in core:

```text
DID
subject identifier
holder identifier
key
proof
challenge
nonce
audience
request object
response object
message header
thread identifier
route
endpoint
wallet handle
credential claim
credential payload
free-text authorization explanation
```

## Required Output Files

```text
docs/research/h5-profile-comparison.md
docs/research/h5-nine-binding-matrix.md
docs/research/h5-replay-and-correlation-analysis.md
docs/research/h5-core-boundary-assessment.md
```

## Final Decision Format

```text
H5_PROFILE_IDENTIFIER = OPEN | EVIDENCE_COLLECTED | REJECTED_FOR_SKYNET
H5_REQUEST_BINDING = OPEN | EVIDENCE_COLLECTED | REJECTED_FOR_SKYNET
H5_VERIFIER_BINDING = OPEN | EVIDENCE_COLLECTED | REJECTED_FOR_SKYNET
H5_PURPOSE_BINDING = OPEN | EVIDENCE_COLLECTED | REJECTED_FOR_SKYNET
H5_CONSENT_SCOPE_BINDING = OPEN | EVIDENCE_COLLECTED | REJECTED_FOR_SKYNET
H5_REPLAY_RESISTANCE = OPEN | EVIDENCE_COLLECTED | REJECTED_FOR_SKYNET
H5_UNLINKABILITY = OPEN | EVIDENCE_COLLECTED | REJECTED_FOR_SKYNET
H5_CORE_BOUNDARY_COMPATIBILITY = OPEN | EVIDENCE_COLLECTED | REJECTED_FOR_SKYNET
H5_EVIDENCE_STATUS = EVIDENCE_REQUIRED
```
