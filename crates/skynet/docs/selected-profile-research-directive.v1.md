[selected-profile-research-directive.v1.md](https://github.com/user-attachments/files/30867534/selected-profile-research-directive.v1.md)
# Skynet Selected Profile Research Directive v1

## Status

```text
format_family = F6
status_family = S6
presentation_family = P3
holder_binding_family = H5
selection_state = CONDITIONAL_SELECTION_PENDING_EVIDENCE
source_generation = prohibited
adapter_generation = prohibited
deployment_activation = prohibited
```

## Selection Meaning

The selected items identify research families only.

They do not select:

```text
a credential schema
a claim vocabulary
a COSE profile
a credential issuer
a verifier
an accumulator construction
a witness format
a revocation authority
a DID method
a DIDComm implementation
a routing method
a mediator
a transport endpoint
a self-issued authentication protocol
a proof format
a cryptographic dependency
```

## F6 — CWT/COSE Credential Profile Research

Create:

```text
docs/cwt-cose-credential-profile-research.md
```

Answer:

```text
Which exact CWT credential profile is proposed?
Which fields are mandatory at the adapter boundary?
Which fields are prohibited from crossing into Skynet core?
How is credential type represented?
How are validity interval and credential status represented?
How can an adapter produce an eligibility result without exposing claim values?
How is selective disclosure achieved, if supported?
How is holder binding represented?
How is verifier-specific presentation unlinkability evaluated?
What is the interoperability evidence for issuer and verifier ecosystems?
What are the offline verification constraints?
```

Required conclusion format:

```text
F6_PROFILE_IDENTIFIER = OPEN
F6_INTEROPERABILITY = OPEN
F6_PRIVACY_COMPATIBILITY = OPEN
F6_CORE_BOUNDARY_COMPATIBILITY = OPEN
```

CWT is a CBOR claims container (RFC 8392) secured with COSE. It is not by itself a complete VC interoperability profile. Exact profile must be identified.

## S6 — Cryptographic Accumulator Status Research

Create:

```text
docs/accumulator-status-research.md
```

Answer:

```text
Which accumulator family is under consideration?
Who is the revocation authority?
Who publishes accumulator epochs?
How is accumulator freshness represented?
How does a holder obtain, refresh, and validate a witness?
What happens when a witness is unavailable?
What happens when the accumulator is stale?
What happens when the revocation authority is unavailable?
How are suspension and permanent revocation distinguished?
How is compromise of a revocation authority handled?
How are old epochs retired?
Can two verifier checks be correlated?
Can the accumulator status process work offline?
What evidence is required before a verifier may rely on offline status?
```

Skynet core result remains limited to:

```text
Active
Expired
Suspended
Unavailable
Unrecognized
```

The accumulator, witness, epoch material, proof material, and revocation data are
adapter-only and prohibited from Skynet public contracts.

## P3 — DIDComm Presentation Research

Create:

```text
docs/didcomm-presentation-profile-research.md
```

Answer:

```text
Which DIDComm version and message-family profile is proposed?
How are request and response messages identified?
How is message expiry represented?
How are duplicate or replayed messages rejected?
How are verifier and holder metadata prevented from entering Skynet core?
How are routing and mediator metadata isolated?
How are message failures normalized to Skynet errors?
How is a presentation request mapped to PresentationRequest?
How is a sealed response mapped to PresentationCommitment?
How is external transport approval obtained?
How are high-risk actions prevented without explicit non-LLM approval?
What offline behavior is supported without asserting network reachability?
```

Mandatory adapter rule:

```text
DIDComm message payloads, headers, DID values, recipient values, sender values,
thread identifiers, routing metadata, endpoint references, and transport
receipts must not enter Skynet core types, fixtures, provenance, or audit events.
```

DIDComm defines application-level messaging and carries sender, recipient, threading,
and routing metadata. All such metadata must remain inside transport adapter.

## H5 — Self-Issued Holder Authentication Research

Create:

```text
docs/self-issued-holder-binding-research.md
```

Answer:

```text
What self-issued authentication profile is proposed?
How is holder control demonstrated without exposing direct holder identity?
How is the authorization bound to presentation_request_id?
How is it bound to verifier_reference?
How is it bound to purpose?
How is freshness established?
How is expiry enforced?
How is replay prevented?
How is cross-verifier reuse prevented?
How is withdrawal or revocation handled?
What failure state is returned when binding evidence is unavailable?
How does the adapter emit only HolderAuthorization to Skynet core?
```

Mandatory `HolderAuthorization` semantic fields (not optional):

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

No DID, key, proof, credential, challenge, route, endpoint, or message value may
appear in this core result.

H5 alone is not sufficient. Self-issued flow must be bound to specific presentation
request; otherwise replay or redirection to another verifier is possible.

The future Skynet core still receives only a typed HolderAuthorization result. DID, key,
proof, challenge, message headers, routing information, and any self-issued artifact
remain inside adapters.

## Threat Model Additions

The future threat model must include:

```text
accumulator witness theft
stale accumulator epoch
revocation-authority compromise
revocation-authority equivocation
offline status replay
CWT claim over-disclosure
CWT credential correlation
DIDComm sender or recipient correlation
DIDComm thread correlation
DIDComm routing or mediator metadata exposure
replayed self-issued authorization
cross-verifier authorization substitution
purpose substitution
expiry bypass
presentation commitment substitution
adapter-to-core prohibited-data leakage
```

## Combined-Profile Acceptance Criteria

The combined F6 + S6 + P3 + H5 profile may advance only when:

- A complete F6 credential profile is identified (CWT alone insufficient).
- S6 status behavior has defined epoch, witness, freshness, and failure rules.
- P3 transport is adapter-only and requires explicit external-action approval.
- H5 binds authorization to request, verifier, purpose, interval, and policy lineage.
- The profile preserves opaque Skynet core contracts.
- The profile preserves content-minimized Skynet audit contracts.
- No format, status, protocol, or binding artifact enters Skynet core.
- Privacy, verifier-trust, deployment, and threat-model gates contain
  EVIDENCE_COLLECTED entries for the selected family.

## Next Required Research Order

```text
1. cwt-cose-credential-profile-research.md
2. accumulator-status-research.md
3. self-issued-holder-binding-research.md
4. didcomm-presentation-profile-research.md
5. threat-model.md update incorporating all four selections
6. Reconcile findings into credential-profile-research.md
7. Decide whether combined profile is accepted or rejected for adapter design
```

No Rust, ALN, fixture, protocol-message, wallet, or cryptographic implementation work
should begin before this research sequence is completed.
