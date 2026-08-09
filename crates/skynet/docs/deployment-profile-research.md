[deployment-profile-research.md](https://github.com/user-attachments/files/30867289/deployment-profile-research.md)
# Skynet Deployment Profile Research

## Status

```text
artifact_type = research_gate
deployment_profile = PHX_AZ_US
decision_status = governance_evidence_required
municipal_authorization = not asserted
location_tracking = prohibited
```

## Meaning of PHX_AZ_US

`PHX_AZ_US` is an application-defined, versioned deployment-profile label.

It is not:

```text
municipal authorization
City of Phoenix affiliation
municipal service credential
proof of residency
street address
physical-presence assertion
real-time location signal
continuous location history
infrastructure access permission
```

## Purpose

This document defines the evidence required to determine whether a future
deployment profile can authorize a verifier interaction.

It does not activate a deployment, grant infrastructure access, establish a
government relationship, select a network, or authorize a real-world verifier.

## Required Research Questions

### Governance authority

Determine:

```text
who may publish a deployment-profile version
how policy authority is identified
how policy versions are approved
how policy versions expire
how a profile is suspended
how a successor profile is approved
how policy lineage is independently reviewed
```

### Verifier registry

Determine:

```text
registry owner
registry publication method
verifier reference format
verifier enrollment requirements
verifier removal process
verifier expiry behavior
allowed purposes per verifier
approved credential descriptors per verifier
audit obligations per verifier
incident-response obligations per verifier
```

### Network-parameter boundary

Determine which configuration facts are permitted:

```text
network_profile_id
parameter_version
policy_authority
policy_version
approved_verifier_registry_reference
created_at
expires_at
```

Confirm that the following are prohibited:

```text
IP address
MAC address
packet payload
radio trace
wireless scan
device serial
network session payload
continuous location
street address
credential claim value
neural or physiological data
```

### Retention and accountability

Determine:

```text
audit retention basis
audit retention duration
deletion or expiry action
holder access process
correction process
incident-response process
audit sink governance
verifier audit duties
```

The future Skynet audit schema remains content-minimized and must not acquire
direct host identity, credential claims, network data, or location data.

### Jurisdiction and agreements

Determine:

```text
whether a documented verifier agreement exists
whether a documented infrastructure agreement exists
whether a documented jurisdiction-specific requirement applies
which evidence is needed before municipal references are permitted
which statements must remain non-assertive until evidence exists
```

## Required Evidence Matrix

| Research topic | Evidence source | Authority | Evidence status | Effect on PHX_AZ_US | Open question |
|---|---|---|---|---|---|
| Governance authority | Repository policy source, ALN authority record | ALN.MIGRATION.CYBERCORE_AUTHORITY.v1, didalnorganic-host | EVIDENCE_REQUIRED | Cannot activate profile until publisher and version approval evidenced | Who signs PHX_AZ_US version and how is lineage reviewed? |
| Verifier registry | Documented verifier enrollment agreement | Evidence source required - no agreement in repo yet | EVIDENCE_REQUIRED | No verifier may be activated | Registry owner, publication method, reference format still OPEN |
| Network-parameter boundary | Wiring-plan and privacy-model | Skynet wiring-plan.md | EVIDENCE_COLLECTED for prohibited field set, OPEN for permitted set confirmation | Permitted fields limited to network_profile_id, parameter_version, policy_authority, policy_version, approved_verifier_registry_reference, created_at, expires_at | Confirm parameter_version governance |
| Retention and accountability | NIST Privacy Framework https://www.nist.gov/privacy-framework, NIST SP 800-63-4 | Evidence source required | OPEN | Audit retention basis and duration not yet documented | Deletion, holder access, correction processes require signed policy |
| Jurisdiction and agreements | Arizona and federal policy review | Evidence source required | OPEN | No municipal or infrastructure claim may be asserted | What evidence required before municipal reference permitted? |

Allowed evidence statuses:

```text
OPEN
EVIDENCE_REQUIRED
EVIDENCE_COLLECTED
DEFERRED
REJECTED_FOR_SKYNET
```

## Completion Criteria

This research gate is complete only when it documents:

- The exact non-geographic meaning of `PHX_AZ_US`.
- The deployment-profile governance and versioning process.
- The verifier-registry governance process.
- The allowed configuration-field set.
- The prohibited network, location, device, and telemetry field set.
- Retention and audit evidence requirements.
- The evidence required before any municipal or infrastructure integration claim.
- Every unresolved governance decision.

Completion of this document does not authorize deployment, transport, verifier
activation, or source generation.

## Decision Preconditions

| Future decision | Required predecessor evidence |
|---|---|
| Credential format | Interoperability evidence, privacy analysis, selective-disclosure analysis, holder-binding analysis |
| Status mechanism | Failure semantics, freshness policy, privacy impact, offline behavior, revocation governance |
| OpenID4VP use | Verifier metadata model, request integrity, holder-binding requirements, DCQL mapping |
| Verifier activation | Enrollment agreement, registry entry, purpose matrix, audit duty, removal process, documented authority |
| PHX_AZ_US activation | Policy authority, versioned profile, approved registry, retention policy, documented authority, independent lineage review, no municipal affiliation claim until evidenced |

No activation may occur until EVIDENCE_COLLECTED status for policy authority, versioned profile, approved registry, and retention policy, all linked to signed policy source or documented agreement.

## Fixed Bindings

```text
host_did = didalnorganic-host
bostrom_address = bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7
aln_authority = ALN.MIGRATION.CYBERCORE_AUTHORITY.v1
```

These are policy-lineage inputs only. Not credential claims, not presentation fields, not audit fields.
