# P3 Policy Evaluation Gate

## Status

```text
P3_DEFINITION_SOURCE = PROPOSED
P3_STATUS = BLOCKED_PENDING_GOVERNANCE_ADOPTION
P3_RUNTIME_MODE = LOCAL_DETERMINISTIC_EVALUATION
P3_EXTERNAL_NETWORK_DEPENDENCY = PROHIBITED
P3_RAW_CREDENTIAL_ACCESS = PROHIBITED
P3_RAW_CLAIM_ACCESS = PROHIBITED
```

## Purpose

P3 defines the Skynet policy evaluation gate. It evaluates minimized,
typed evidence produced by reviewed adapters and returns a deterministic
eligibility outcome.

P3 does not parse credential formats, verify cryptographic presentations,
resolve status lists, contact policy authorities, transport presentations,
or retain sensitive identity material.

## Inputs

```text
PresentationRequestMetadata
CredentialStatus
HolderAuthorization
PolicyLineage
DisclosureReceipt
PolicyProgram
EvaluationTime
```

## Outputs

```text
EligibilityDecision
AuditEvent
```

## Approval Rule

\[
\operatorname{Approved} \iff
\operatorname{PolicyAvailable}
\land
\operatorname{RequestAccepted}
\land
\operatorname{StatusActive}
\land
\operatorname{HolderAuthorizationValid}
\land
\operatorname{DisclosureWithinScope}
\land
\operatorname{DeploymentAccepted}
\]

## Required Invariants

```text
SKY-I-001: P3 receives no raw credential or credential claim values.
SKY-I-002: P3 receives no holder identifiers, keys, proofs, nonces, or routes.
SKY-I-003: P3 cannot approve Unavailable or Unrecognized status evidence.
SKY-I-004: P3 rejects missing, expired, replayed, or context-mismatched authorization.
SKY-I-005: P3 decisions are reproducible from policy authority, version,
           rule reference, and effective interval.
SKY-I-006: P3 audit events contain no credential, claim, identity, or route data.
SKY-I-007: P3 cannot perform implicit network access or hidden policy retrieval.
```

## Policy Program Contract

```text
policy_id
policy_version
policy_authority
effective_from
effective_to
deployment_profile_set
verifier_class_set
purpose_set
credential_profile_set
required_statuses
required_holder_bindings
permitted_disclosure_descriptor_sets
freshness_constraints
decision_rules
denial_reason_catalog
audit_profile
```

## Closed Outcomes

```text
Approved
Denied
Unavailable
Unrecognized
InvariantViolation
```

## Required Failure Mapping

| Condition | Result |
|---|---|
| Policy not present or outside effective interval | Unrecognized |
| Status authority unavailable or evidence stale | Unavailable |
| Conflicting valid status publications | Unavailable |
| Holder authorization missing, invalid, replayed, or expired | Denied |
| Requested descriptor set exceeds policy scope | Denied |
| Deployment profile is not accepted by policy | Denied |
| Core boundary contract is violated | InvariantViolation |
