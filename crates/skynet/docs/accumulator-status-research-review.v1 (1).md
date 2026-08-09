# S6 Accumulator Status Research Review v1

## Review Result

```text
artifact_reviewed = accumulator-status-research.md
research_scope = accepted_with_corrections
status_mechanism_selected = false
accumulator_family_selected = false
cryptographic_dependency_selected = false
implementation_authorized = false
S6_gate_state = OPEN
```

## Required S6 Corrections

### Integrity wording

Replace all instances of:

```text
signed policy source
signed offline status artifact
```

with:

```text
authorized, versioned artifact with independently evidenced integrity and provenance
```

### Construction-neutral candidate wording

Replace:

```text
Merkle-tree based accumulator
sparse Merkle tree
hash-based accumulator
```

with:

```text
tree-based commitment construction — DEFERRED pending approved primitive,
privacy analysis, witness analysis, and dependency review
```

### Status-locator wording

Replace:

```text
status URL
status resource
```

with:

```text
adapter-private status locator or status artifact reference
```

### Validity normalization

Replace:

```text
Expired = exp/nbf check fails
```

with:

```text
Expired = current validated time is after normalized expiry.

Not-before failure = credential is not currently usable.
Its closed-status mapping remains OPEN pending data-contract approval.
Presentation must decline with a closed reason code.
```

### Fail-closed status rule

Add:

```text
Missing witness, stale epoch, unavailable authority, unavailable status artifact,
unverifiable provenance, or unresolved freshness policy must never result in
Active.

The adapter returns Unavailable or another separately approved closed result.
The core declines presentation and records only a content-minimized reason code.
```

### Suspension and revocation rule

Add:

```text
No accumulator family is presumed to encode both suspension and permanent
revocation.

Research must determine whether separate accumulator domains, independent
artifacts, or another governed representation is required to preserve the
closed Skynet distinction between Suspended and permanent non-usability.
```

## F6 Dependency Correction

S6 may not be committed as part of a combined F6 + S6 profile until
`cwt-cose-credential-profile-research.md` applies all prior corrections:

```text
remove RFC 9334 as a CWT source
replace payload-bearing PresentationCommitment with PresentationCommitmentReference
remove generic COSE claim-redaction selective-disclosure assertion
```

## S6 Exit State

```text
S6_ACCUMULATOR_FAMILY = OPEN
S6_REVOCATION_AUTHORITY = OPEN
S6_EPOCH_MODEL = OPEN
S6_FRESHNESS_POLICY = OPEN
S6_WITNESS_LIFECYCLE = OPEN
S6_SUSPENSION_VS_REVOCATION = OPEN
S6_COMPROMISE_RESPONSE = OPEN
S6_CORRELATION_RISK = OPEN
S6_OFFLINE_BEHAVIOR = OPEN
S6_CORE_BOUNDARY_COMPATIBILITY = OPEN
S6_EVIDENCE_STATUS = EVIDENCE_REQUIRED
```
