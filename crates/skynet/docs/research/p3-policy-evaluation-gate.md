# P3 Policy Evaluation Gate Research

## Status

```text
artifact_type = research_gate
policy_gate = P3
P3_DEFINITION_SOURCE = PROPOSED
P3_STATUS = BLOCKED_PENDING_GOVERNANCE_ADOPTION
P3_RUNTIME_MODE = LOCAL_DETERMINISTIC_EVALUATION
P3_EXTERNAL_NETWORK_DEPENDENCY = PROHIBITED
P3_RAW_CREDENTIAL_ACCESS = PROHIBITED
P3_RAW_CLAIM_ACCESS = PROHIBITED
P3_RAW_PROOF_ACCESS = PROHIBITED
P3_RAW_NONCE_ACCESS = PROHIBITED
P3_DYNAMIC_CODE_EXECUTION = PROHIBITED
P3_RUST_IMPLEMENTATION_GENERATION = PROHIBITED
P3_ADAPTER_IMPLEMENTATION_GENERATION = PROHIBITED
P3_FIXTURE_GENERATION = PROHIBITED
P3_DEPLOYMENT_ACTIVATION = PROHIBITED
```

## Purpose

P3 is the proposed Skynet local policy-evaluation gate. It evaluates minimized,
typed semantic evidence from reviewed adapters and produces one final
`EligibilityDecision` plus one content-minimized `AuditEvent` instruction.

P3 is not a credential parser, cryptographic verifier, status resolver, wallet
interface, transport client, policy downloader, or dynamic-code runtime.

P3 does not:

- Parse CWT, COSE, VC, mdoc, SD-JWT, or other credential formats.
- Verify raw signatures, proofs, keys, challenges, or nonce values.
- Resolve status lists, accumulators, witnesses, or external authorities.
- Contact policy authorities, verifier registries, wallets, transports, or
  audit sinks at evaluation time.
- Access system time directly.
- Retain raw credentials, claims, presentations, holder identifiers, routes,
  location, neural data, biometric data, or physiological data.
- Upgrade `Unavailable` or `Unrecognized` status evidence to `Active`.
- Execute dynamic code or interpret free-text policy programs.

P3 receives already-normalized evidence. It evaluates that evidence locally and
deterministically against a bounded typed policy representation.

## Boundary Ownership

```text
F6 credential adapter produces:
  CredentialProfileIdentifier
  CredentialProfileVersion
  DisclosureReceipt

S6 status adapter produces:
  CredentialStatus

H5 holder-authorization adapter produces:
  HolderAuthorization
  RequestBinding

PolicyAuthoritySnapshotPort produces:
  PolicyLineage
  authenticated policy material for pre-evaluation conversion to local IR

VerifierRegistryPort produces:
  VerifierRegistrationStatus

P3 core produces:
  EligibilityDecision
  AuditEvent

AuditSink persists:
  AuditEvent outside the P3 evaluation function
```

No F6, S6, or H5 adapter produces `EligibilityDecision` or `AuditEvent`.

The policy-authority snapshot adapter is the only permitted adapter role that
resolves policy lineage. F6 and H5 adapters do not resolve policy lineage.

## Inputs

P3 receives exactly eight minimized input categories under this proposed
contract. The verifier-registration outcome is explicit because P3 cannot
deterministically enforce verifier approval without receiving that evidence.

```text
R  PresentationRequestMetadata
S  CredentialStatus
H  HolderAuthorization
L  PolicyLineage
Q  DisclosureReceipt
V  VerifierRegistrationStatus
P  PolicyProgram
t  EvaluationTime
```

No additional input category may be added without governance adoption.

### R: PresentationRequestMetadata

```text
presentation_request_id
verifier_reference
deployment_profile
purpose
consent_scope_id
requested_descriptor_set_id
```

`PresentationRequestMetadata` contains no raw request object, nonce, endpoint,
transport route, credential request payload, holder identifier, or claim value.

### S: CredentialStatus

```text
Active
Expired
Suspended
Unavailable
Unrecognized
```

### H: HolderAuthorization

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
observed_at
request_binding
```

`HolderAuthorization` contains no holder authorization identifier, wallet
identifier, key, signature, proof, nonce, credential, presentation, or route.

### L: PolicyLineage

```text
authority
version
rule_reference
effective_from
effective_to
content_reference
```

### Q: DisclosureReceipt

```text
profile_identifier
profile_version
descriptor_set_id
conformance
```

### V: VerifierRegistrationStatus

```text
Approved
NotApproved
Unavailable
Unrecognized
```

### P: PolicyProgram

`PolicyProgram` is a proposed bounded, immutable typed Rust IR generated from
previously authenticated and lineage-verified policy material. It is not a raw
serialized package, dynamic script, free-text rule document, or runtime
network response.

### t: EvaluationTime

```text
UtcTimestamp supplied explicitly by the trusted evaluation caller.
```

P3 never reads system time directly. Clock-source selection, synchronization,
and trust assumptions remain outside P3 and require separate governance review.

## Outputs

### EligibilityDecision

```text
Permitted fields:
  decision_receipt_id
  outcome
  reason_codes
  profile_identifier
  policy_lineage_reference
  evaluated_at

Prohibited fields:
  claim_value
  credential_digest
  holder_identifier
  verifier_network_address
  credential_payload
  presentation_payload
  proof_material
  key_material
  nonce_value
  transport_route
```

`decision_receipt_id` must be transaction-scoped. It must not become a stable
holder, verifier, credential, device, or location correlator.

### AuditEvent

```text
Permitted fields:
  event_id
  timestamp_utc
  actor_role
  action
  outcome_code
  transaction_scoped_reference
  policy_lineage_reference

Prohibited fields:
  actor_identifier
  credential_claim
  credential_payload
  presentation_payload
  network_route
  continuous_location
  free_text_narrative
  device_identifier
  neural_data
  physiological_data
```

`actor_role` is a closed enum:

```text
Holder
Verifier
PolicyAuthority
Adapter
SkynetCore
```

P3 creates the fixed-shape audit event. `AuditSink` is responsible only for
external persistence and is not part of the pure P3 evaluation function.

## Closed Outcomes

```text
Approved
Denied
Unavailable
Unrecognized
InvariantViolation
```

### Outcome semantics

```text
Approved
  Evidence is sufficient; the selected policy conditions are met; status is
  Active; required bindings are valid; verifier registration is Approved;
  disclosure is within scope; policy lineage is complete and effective; and
  the deployment profile is accepted.

Denied
  Evidence is sufficient to determine that one or more policy conditions are
  not met.

Unavailable
  Required current evidence or local policy material is insufficient to produce
  an approval or definitive denial. This is transaction-scoped and does not
  reduce protected host capabilities.

Unrecognized
  A required credential profile, policy version, status semantic, verifier
  registry semantic, or other policy-relevant representation is unsupported.

InvariantViolation
  A core-boundary or internal contract violation occurred. The transaction
  halts and a bounded operational fault event is prepared. This outcome never
  authorizes rollback, surveillance escalation, or capability reduction.
```

## Proposed evaluation rule

\[
D = \operatorname{Evaluate}(R,S,H,L,Q,V,P,t)
\]

\[
\operatorname{Approved} \iff
\operatorname{PolicyAvailable}(P,L)
\land
\operatorname{RequestAccepted}(R,P)
\land
(S=\operatorname{Active})
\land
(V=\operatorname{Approved})
\land
\operatorname{ValidAuth}(H,R,L,t)
\land
\operatorname{DisclosureWithinScope}(Q,R,P)
\land
\operatorname{DeploymentAccepted}(R,P)
\]

No evaluation may return `Approved` when:

- `S` is `Expired`, `Suspended`, `Unavailable`, or `Unrecognized`.
- `V` is `NotApproved`, `Unavailable`, or `Unrecognized`.
- Required policy material is unavailable or unsupported.
- Authorization is missing, invalid, replayed, expired, stale, withdrawn, or
  context-mismatched.
- Disclosure is outside approved scope.
- Deployment profile is not accepted.
- Policy lineage is incomplete, mismatched, or outside its effective interval.
- An invariant violation has been detected.

## Authorization predicate

\[
\operatorname{ValidAuth}(H,R,L,t) =
B_r \land B_v \land B_p \land B_c \land B_\lambda \land B_t \land B_f \land B_b
\]

```text
B_r  H.presentation_request_id = R.presentation_request_id
B_v  H.verifier_reference = R.verifier_reference
B_p  H.purpose = R.purpose
B_c  H.consent_scope_id = R.consent_scope_id
B_λ  H.policy_authority = L.authority
     ∧ H.policy_version = L.version
B_t  H.not_before ≤ t < H.expires_at
B_f  t − H.observed_at ≤ H.freshness
B_b  H.request_binding = Bound
```

Cryptographic proof, nonce, challenge, key, wallet, and replay-cache mechanics
are adapter-local. P3 evaluates only the semantic `RequestBinding` result and
the typed context fields.

## Status mapping

```text
Fresh verified usable status evidence       -> Active
Credential validity interval ended          -> Expired
Fresh verified reversible disable evidence  -> Suspended
Missing, stale, invalid, unavailable, or
conflicting authority evidence              -> Unavailable
Unsupported status mechanism or purpose     -> Unrecognized
```

```text
Unavailable != Expired
Unavailable != Suspended
Unrecognized != Unavailable
Only Active may satisfy an active-required policy rule.
```

## Policy material boundary

```text
PolicyAuthoritySnapshot adapter
        |
        v
Authenticated and lineage-verified policy material
        |
        v
Schema validation and bounded conversion
        |
        v
Immutable typed PolicyProgram
        |
        v
Local deterministic P3 evaluator
        |
        v
EligibilityDecision + AuditEvent
```

The authentication method, signature format, ALN representation, and
policy-package serialization remain open until governance selects them.

## Proposed PolicyProgram fields

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

The proposal is subject to review. The final IR must be bounded, immutable, and
free of credential claims, holder identifiers, proofs, keys, nonce values,
routes, location, device state, raw policy source, hidden remote lookups, and
dynamic-code directives.

## Required failure mapping

| Condition | Proposed P3 result |
| :--- | :--- |
| Local policy snapshot absent or authority evidence unavailable | `Unavailable` |
| Requested policy version or required semantic unsupported | `Unrecognized` |
| Policy lineage outside its effective interval | `Denied` |
| Status evidence stale, invalid, unavailable, or conflicting | `Unavailable` |
| Credential status is `Expired` or `Suspended` | `Denied` |
| Credential status is `Unrecognized` | `Unrecognized` |
| Verifier registration is `NotApproved` | `Denied` |
| Verifier registry result is `Unavailable` | `Unavailable` |
| Verifier registry result is `Unrecognized` | `Unrecognized` |
| Authorization is missing, invalid, replayed, expired, stale, or mismatched | `Denied` |
| Consent scope is inactive, withdrawn, expired, or mismatched | `Denied` |
| Descriptor set exceeds policy scope | `Denied` |
| Deployment profile is not accepted | `Denied` |
| Core-boundary contract violation | `InvariantViolation` |
| Prohibited data detected in a P3 input | `InvariantViolation` |

## State model

```text
Received
   |
   v
EvidenceValidated
   |
   v
PolicyLocated
   |
   v
PolicyLineageValidated
   |
   v
PolicyEvaluated
   |
   +--> Denied
   +--> Unavailable
   +--> Unrecognized
   +--> InvariantViolation
   |
   v
Approved
   |
   v
AuditPrepared
   |
   v
Completed
```

`AuditPrepared` means P3 constructed a fixed-shape `AuditEvent`. Audit-sink
persistence occurs outside the pure policy evaluation function. Audit delivery
failure semantics require separate policy adoption and must not be assumed here.

## Required invariants

```text
SKY-I-001: P3 receives no raw credential or credential claim values.
SKY-I-002: P3 receives no holder identifiers, keys, proofs, nonces, or routes.
SKY-I-003: P3 cannot approve non-Active status evidence.
SKY-I-004: P3 rejects missing, expired, replayed, stale, or context-mismatched authorization.
SKY-I-005: P3 decisions are reproducible from policy authority, version,
           rule reference, content reference, and effective interval.
SKY-I-006: P3 audit events contain no credential, claim, identity, or route data.
SKY-I-007: P3 performs no implicit network access or hidden policy retrieval.
SKY-I-008: P3 performs no dynamic-code execution or unbounded policy interpretation.
SKY-I-009: InvariantViolation halts only the current transaction and does not reduce host capabilities.
SKY-I-010: Only Active may satisfy an active-required policy rule.
SKY-I-011: Unavailable and Unrecognized never silently become Active.
SKY-I-012: P3 independently validates temporal bounds and freshness using explicit evaluation time.
```

## Governance bindings

```text
host_did = didalnorganic-host
bostrom_address = bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7
aln_authority = ALN.MIGRATION.CYBERCORE_AUTHORITY.v1
```

These are external governance bindings only. They are not credential claims,
presentation fields, verifier payloads, direct policy inputs, or audit fields.

No ALN syntax, parser integration, or policy-package encoding is implied by
this document.

## Code-generation gate

```text
P3_DEFINITION_SOURCE = ADOPTED
P3_POLICY_IR_SCHEMA = REVIEWED
P3_LINEAGE_FIELDS = COMPLETE
P3_DENIAL_REASON_CATALOG = COMPLETE
P3_STATUS_MAPPING = COMPLETE
P3_H5_BINDINGS = COMPLETE
P3_F6_PROFILE_IDENTIFIER = SELECTED
P3_S6_BASELINE = SELECTED
P3_NEGATIVE_TEST_MATRIX = COMPLETE
P3_THREAT_MODEL_UPDATE = COMPLETE
P3_FORMAL_INVARIANT_CATALOG = COMPLETE
```

No `src/policy.rs`, P3 fixture, policy-schema implementation, or policy
execution code may be generated until every listed condition is satisfied.

## Conclusion

```text
P3_DEFINITION_SOURCE = PROPOSED
P3_STATUS = BLOCKED_PENDING_GOVERNANCE_ADOPTION
P3_RUNTIME_MODE = LOCAL_DETERMINISTIC_EVALUATION
P3_POLICY_IR_SCHEMA = OPEN
P3_DENIAL_REASON_CATALOG = OPEN
P3_STATE_MACHINE = PROPOSED
P3_NEGATIVE_TEST_MATRIX = OPEN
P3_FORMAL_INVARIANT_CATALOG = OPEN
P3_EVIDENCE_STATUS = EVIDENCE_REQUIRED
```
