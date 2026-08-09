# P3, F6, and Audit Concurrency Framework

## Status

```text
P3_GOVERNANCE_DISCOVERY = ACTIVE
F6_COSE_PROFILE = CANDIDATE_RESEARCH
S6_STATUS_SELECTION = CANDIDATE_COMPARISON
H5_BINDING_RESEARCH = ACTIVE
AUDIT_DELIVERY_MODEL = PROPOSED
NO_GATE_IS_SELECTED_OR_ADOPTED_BY_THIS_DOCUMENT
```

## Purpose

This document coordinates concurrent research for:

```text
P3 governance discovery and proposal preparation
F6 CWT/COSE application-profile research
S6 status-mechanism comparison
H5 semantic holder-binding research
audit-delivery assurance research
```

It does not authorize P3, pipeline, transport, status-adapter, wallet-adapter,
ALN, fixture, or deployment implementation.

## P3 Discovery Lifecycle

### Pass 1: Verified active source tree

Retrieve and classify:

```text
Cargo.toml
README.md
LICENSE*
GOVERNANCE*
SECURITY*
CODEOWNERS
docs/
policy/
aln/
.github/
crates/skynet/
```

Search:

```text
P3
P3_POLICY
EligibilityDecision
PolicyProgram
PolicyLineage
decision_reason
denial_reason
architecture decision
governance
```

### Pass 2: Verified repository history

Run:

```bash
git rev-parse HEAD
git log --all --oneline -- crates/skynet docs policy aln
git log --all -S'P3' -- .
git log --all -S'EligibilityDecision' -- .
git log --all -S'PolicyProgram' -- .
git grep -n -I -e 'P3' -e 'PolicyLineage' -e 'EligibilityDecision' HEAD -- .
```

### Pass 3: Explicitly delegated authority

Investigate Cybercore or ALN artifacts only if a verified Skynet governance
artifact explicitly delegates authority to them.

### Stop condition

```text
If no adopted definition is found:
P3_DEFINITION_SOURCE = NOT_FOUND_IN_VERIFIED_SOURCES
P3_STATUS = PROPOSAL_REQUIRED
```

A negative discovery result permits proposal preparation. It does not permit
policy implementation.

## F6 Boundary Rule

```text
CWT/COSE wire data -> F6 adapter validation -> minimized typed evidence -> P3
```

P3 receives no CWT, COSE, CBOR, raw credential, claim value, proof, key, nonce,
wallet handle, endpoint, route, or presentation payload.

### F6 Candidate Security Floor

```text
F6-N-001: Raw credential payload never reaches core.
F6-N-002: Claim values never reach core.
F6-N-003: Unknown critical semantics fail closed at adapter boundary.
F6-N-004: Unsupported algorithms fail closed at adapter boundary.
F6-N-005: Duplicate labels and protected/unprotected collisions fail closed.
F6-N-006: Parser limits are explicit and bounded.
F6-N-007: Core trust resolution causes no implicit network access.
F6-N-008: Proof, key, nonce, wallet, and route material remain adapter-local.
```

The selected COSE structure, tagging rule, algorithm policy, AAD rule,
header allow-lists, key-resolution mechanism, and detached-payload policy
remain open until F6 governance selection.

## Audit Delivery Separation

```text
P3:
  produces EligibilityDecision and prepares AuditEvent.

Pipeline:
  invokes AuditSink according to deployment-selected audit delivery policy.

Audit observability:
  records only bounded delivery outcome metadata.
```

### Proposed audit modes

```text
RequiredSynchronous
RequiredDurableAsynchronous
BestEffort
DisabledByApprovedPolicy
```

### Disabled mode rule

```text
P3 still prepares AuditEvent.
Pipeline does not invoke an external AuditSink.
Pipeline emits DeliverySuppressedByPolicy.
Observability includes only closed delivery result, timestamp, transaction-
scoped reference, and policy-lineage reference.
```

### Prohibited observability data

```text
credential claim
credential payload
presentation payload
holder identifier
actor identifier
wallet identifier
key
proof
signature
nonce
endpoint
route
IP address
location
device identifier
neural data
physiological data
free-text error detail
```

## Adoption Preconditions

```text
P3 adoption requires:
- adopted governance record
- F6 input compatibility
- S6 input compatibility
- H5 input compatibility
- closed outcome model
- reviewed policy IR
- complete negative matrix
- complete formal invariant catalog

Pipeline implementation requires:
- F6 selected
- S6 selected
- H5 selected
- P3 adopted
- audit delivery model selected
- combined-stack threat model updated
```
