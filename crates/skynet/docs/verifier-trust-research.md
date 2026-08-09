# Skynet Verifier Trust Research

## Status

```text
artifact_type = research_gate
decision_status = no_verifier_selected
implementation_authority = none
verifier_activation = deferred
municipal_integration = not_asserted
```

## Purpose

This document defines the evidence required to determine whether a future
verifier can be enrolled, authorized for a purpose, and later removed.

It does not select a verifier, enroll a verifier, activate a verifier,
establish a municipal relationship, authorize infrastructure access, or
implement a registry.

## Fixed Skynet Boundary

Verifier trust must preserve:

```text
holder-controlled presentation
explicit holder authorization bound to request, verifier, purpose, interval, policy lineage
purpose-limited disclosure via ClaimDescriptorId only
verifier authorization behind VerifierRegistryProvider port
policy lineage via ALN projection
content-minimized audit with reason codes only
no raw credential or claim value in Skynet core
no direct holder identifier in routine presentation
no neural, physiological, clinical, device, network, or location data
```

## Research Questions

### Verifier reference and registry

Investigate, without selecting:

```text
verifier reference format
registry owner
registry publication method
registry signature and integrity model
registry expiry and freshness model
registry removal and suspension propagation
```

### Verifier enrollment

Determine:

```text
enrollment requester
enrollment approver
required enrollment artifacts
allowed processing purposes per verifier
approved claim descriptors per verifier
credential status expectations per verifier
holder notice requirements per verifier
audit obligations per verifier
incident-response obligations per verifier
```

### Verifier removal and expiry

Determine:

```text
removal requester
removal approver
removal propagation delay
expiry behavior
suspension behavior
compromise and incident response
holder notification on removal
audit retention after removal
```

### Purpose and disclosure mapping

Determine for each candidate verifier:

```text
purpose = CivicIdentityVerification | InfrastructureAccessVerification | CredentialStatusCheck | HolderLocalReview | ResearchDerivedMetadataExport
whether purpose is allowed for this verifier
which ClaimDescriptorIds are allowed for this purpose and verifier
which ClaimDisclosureClass applies (Required, Optional, Prohibited)
whether purpose requires independent consent scope (e.g., ResearchDerivedMetadataExport)
```

No infrastructure verification purpose may authorize research export.

### Trust governance

Determine:

```text
policy authority that governs verifier registry
policy version governance
how approved_deployment_profiles list is maintained
how approved_verifier_registry_reference is versioned
how policy lineage is independently reviewed
how trust-list publication is evidenced
how compromise is evidenced and audited
```

No static allowlist may be embedded in Skynet source. All trust must be supplied
via typed ports from signed, versioned configuration.

## Required Evidence Matrix

| Research topic | Candidate / Verifier type | Primary source | Evidence status | Privacy impact | Governance impact | Open question |
|---|---|---|---|---|---|---|
| Verifier reference format | Opaque VerifierReference | Skynet wiring-plan.md, data-contracts.md (to be approved) | OPEN | EVIDENCE_REQUIRED - must be opaque, validated, non-empty | EVIDENCE_REQUIRED - must be governed by registry, not hard-coded | What is canonical grammar for VerifierReference and VerifierPolicyReference? |
| Registry publication | Versioned registry file | Organizational policy source required | EVIDENCE_REQUIRED | EVIDENCE_REQUIRED - must not leak holder data | EVIDENCE_REQUIRED - owner, publication, signature model needed | Who publishes registry and how is integrity verified? |
| Enrollment | Civic identity verification verifier | Documented enrollment agreement required | EVIDENCE_REQUIRED | EVIDENCE_REQUIRED - purpose limitation and minimization analysis | EVIDENCE_REQUIRED - enrollment approver not evidenced | No enrollment agreement evidenced in repository yet |
| Enrollment | Infrastructure access verification verifier | Documented enrollment agreement required | EVIDENCE_REQUIRED | EVIDENCE_REQUIRED | EVIDENCE_REQUIRED | Infrastructure access must not be claimed as municipal authorization |
| Enrollment | Research metadata export verifier | Documented enrollment agreement + independent consent scope evidence required | OPEN | EVIDENCE_REQUIRED - must not receive raw claim values or direct identity | EVIDENCE_REQUIRED - requires independent consent scope | How is ResearchDerivedMetadataExport segregated from CivicIdentityVerification? |
| Removal / expiry | All verifier types | Organizational incident-response policy required | OPEN | EVIDENCE_REQUIRED | EVIDENCE_REQUIRED | Removal propagation delay and holder notification still OPEN |
| Purpose mapping | Purpose matrix per verifier | NIST SP 800-63-4 https://pages.nist.gov/800-63-4/, NIST Privacy Framework | OPEN | EVIDENCE_REQUIRED | EVIDENCE_REQUIRED | Purpose matrix not yet approved |

Allowed evidence statuses:

```text
OPEN
EVIDENCE_REQUIRED
EVIDENCE_COLLECTED
DEFERRED
REJECTED_FOR_SKYNET
```

## Prohibited Claims

Until EVIDENCE_COLLECTED with signed policy source:

```text
No City of Phoenix affiliation
No municipal authorization
No infrastructure access permission
No proof of residency or physical presence
No real-time location
No municipal service credential
No deployment activation
```

## Completion Criteria

This research gate is complete only when it documents:

- Verifier reference format requirements (opaque, validated).
- Registry owner, publication, integrity, expiry, and removal governance.
- Enrollment requirements per verifier and purpose.
- Allowed purposes and approved claim descriptors per verifier type.
- Audit and incident-response obligations per verifier.
- Purpose-matrix governance and independent consent requirement for research export.
- Explicit list of unresolved decisions with evidence status OPEN or EVIDENCE_REQUIRED.

Completion does not authorize verifier activation, deployment activation, transport,
or source generation. It enables the next decision: whether a verifier-registry
governance record may be proposed.

## Decision Preconditions

| Future decision | Required predecessor evidence |
|---|---|
| Credential format | Interoperability evidence, privacy analysis, selective-disclosure analysis, holder-binding analysis |
| Status mechanism | Failure semantics, freshness policy, privacy impact, offline behavior, revocation governance |
| OpenID4VP use | Verifier metadata model, request integrity, holder-binding requirements, DCQL mapping to ClaimDescriptorId |
| Verifier activation | Enrollment agreement (signed), registry entry (versioned, signed), purpose matrix (approved), audit duty (documented), removal process (documented), policy authority and version, independent lineage review |
| PHX_AZ_US activation | Policy authority, versioned profile, approved registry reference, retention policy, documented authority, verifier activation evidence for at least one purpose |

No verifier may be activated until enrollment agreement, registry entry, purpose matrix,
and audit duty are EVIDENCE_COLLECTED and linked to signed policy source.

## Fixed Bindings

```text
host_did = didalnorganic-host
bostrom_address = bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7
aln_authority = ALN.MIGRATION.CYBERCORE_AUTHORITY.v1
```

These are policy-lineage inputs only. Not credential claims, not verifier payloads, not audit fields.
