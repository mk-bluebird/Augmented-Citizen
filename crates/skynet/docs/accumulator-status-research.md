# S6 Cryptographic Accumulator Status Research

## Status

```text
artifact_type = research_gate
status_family = S6
family_description = Cryptographic accumulator for revocation/suspension status
selection_state = CONDITIONAL_SELECTION_PENDING_EVIDENCE
status_mechanism_selected = false
cryptographic_dependency_selected = false
implementation_authorized = false
source_generation = prohibited
adapter_generation = prohibited
```

## Purpose

This document researches whether a cryptographic accumulator can provide
revocation and suspension status for CWT/COSE credentials without exposing
claim values, holder identifiers, or correlation handles to Skynet core.

It does not select an accumulator construction, witness format, revocation
authority, epoch publisher, or cryptographic library. No accumulator implementation,
fixture, or Rust code is generated.

Skynet core boundary remains:

```text
Skynet core never dereferences a credential status URL, retrieves a status resource,
parses a status list, or receives a status payload.
CredentialStatusProvider adapter may resolve status under approved privacy, freshness,
and failure policy, then returns only closed enum:
Active, Expired, Suspended, Unavailable, Unrecognized
```

## Research Questions

### 1. Which accumulator family is under consideration?

**OPEN - EVIDENCE_REQUIRED**

Candidates (none selected, research comparison only):

```text
RSA accumulator (Benaloh de Mare style) - requires trusted setup or class-group assumption analysis
Bilinear-map accumulator (Nguyen) - pairing-based, trusted setup analysis required
Merkle-tree based accumulator / sparse Merkle tree - hash-based, no trusted setup, witness size analysis
Hash-based accumulator with batch updates
```

Required evidence per candidate:

```text
primary specification source and version
trusted setup requirements
witness size
epoch update complexity
holder witness refresh complexity
verifier check complexity
privacy: does accumulator or witness leak membership or enable correlation?
offline behavior
failure behavior
interoperability evidence
```

No family may be marked SELECTED_FOR_ADAPTER_ONLY until evidence matrix is EVIDENCE_COLLECTED
with primary source, privacy analysis, linkability analysis, offline behavior, failure semantics.

### 2. Who is the revocation authority?

**OPEN - EVIDENCE_REQUIRED**

Determine:

```text
revocation authority identifier (opaque, not network endpoint)
how authority is evidenced (signed policy source required)
how authority enrollment is approved
how authority removal or rotation is handled
how authority compromise is evidenced and audited
```

No authority may be embedded in Skynet source. Authority must be supplied via
versioned registry with independently evidenced integrity and provenance.

### 3. Who publishes accumulator epochs?

**OPEN**

Determine:

```text
epoch publisher (may be same as revocation authority or separate)
epoch publication method (versioned artifact, not network endpoint)
epoch integrity and provenance model (versioned registry wording, no signing mechanism presumed selected)
epoch expiry and freshness representation
how old epochs are retired
```

Epoch is adapter-only material. Epoch bytes, accumulator value, and publication receipt
must never enter Skynet core types, fixtures, provenance, or audit.

### 4. How is accumulator freshness represented?

**OPEN**

Determine:

```text
freshness claim format (timestamp, epoch counter, validity interval)
how freshness is validated by holder wallet adapter
how freshness is validated by verifier adapter
what happens when accumulator is stale
what evidence is required before a verifier may rely on offline status
```

Freshness policy must be documented with primary source or signed policy source.
Stale epoch must result in Unavailable or Declined with reason code, not in bypass.

### 5. How does a holder obtain, refresh, and validate a witness?

**OPEN - EVIDENCE_REQUIRED**

Determine:

```text
witness acquisition method (adapter-private, not core)
witness refresh method
witness validation method inside wallet adapter
what happens when witness is unavailable
whether witness refresh leaks holder activity to authority or verifier
whether two witness refreshes are correlatable
```

Witness is adapter-only: witness bytes, proof material, and refresh receipts are
prohibited from Skynet core, fixtures, audit, provenance.

### 6. What happens when witness is unavailable?

**OPEN**

Required failure semantics:

```text
holder wallet adapter returns Unavailable to CredentialStatusProvider?
or Suspended?
or defers presentation?
How is holder notified?
How is audit recorded (content-minimized, no witness material)?
```

Failure must not leak claim values, holder identifier, or witness material to core.

### 7. What happens when accumulator is stale?

**OPEN**

Required failure semantics:

```text
adapter returns Unavailable
core declines presentation with reason CredentialStatusUnavailable or PolicyVersionUnavailable?
Does stale epoch require holder to refresh witness?
How is staleness evidenced without network fetch in core?
```

Staleness must be determined in adapter under approved freshness policy. Core only
receives closed status.

### 8. What happens when revocation authority is unavailable?

**OPEN**

Determine:

```text
does authority unavailability cause Unavailable status?
does it cause suspension to be treated as Active or as Unavailable?
how is authority unavailability evidenced?
how is holder affected?
```

Authority unavailability must not cause silent Active.

### 9. How are suspension and permanent revocation distinguished?

**OPEN - EVIDENCE_REQUIRED**

Determine:

```text
accumulator representation for suspension (temporary, reversible) vs revocation (permanent)
whether suspension and revocation use same accumulator or separate accumulators
how holder distinguishes suspension from revocation
how verifier distinguishes suspension from revocation
how suspension expiry is represented
```

Skynet core normalization must preserve distinction:

```text
Suspended = credential is temporarily not usable, may become Active again
Active = not suspended, not revoked, not expired
Expired = validity interval ended (via exp/nbf, not accumulator)
```

No accumulator construction may conflate Suspended with Expired.

### 10. How is compromise of a revocation authority handled?

**OPEN**

Threat model additions required:

```text
revocation-authority compromise
revocation-authority equivocation (publishing different accumulator values to different holders/verifiers)
witness theft
offline status replay
```

Compromise response must include:

```text
how compromise is detected
how compromised epochs are retired
how holders obtain new witnesses from new authority or new accumulator
how verifiers are notified
how audit records compromise without exposing witness or accumulator material
```

### 11. How are old epochs retired?

**OPEN**

Determine:

```text
epoch retirement policy
how holders with old witnesses are migrated
how verifiers handle presentation with old epoch
how long old epochs remain valid for offline verification
```

Retirement must be governed by signed policy source, not by network availability.

### 12. Can two verifier checks be correlated?

**OPEN - EVIDENCE_REQUIRED - privacy critical**

Research question:

```text
Can two verifier status checks for same credential be linked via accumulator value, witness, epoch, or status request?
Can accumulator value or witness serve as StablePseudonymousIdentifier or CrossSessionCorrelationIdentifier?
Does status check leak to authority that holder is presenting to a specific verifier?
```

If accumulator enables correlation, it must be marked REJECTED_FOR_SKYNET or require
explicit privacy impact assessment, holder consent, and mitigation evidence.

Reference: W3C VC Data Model 2.0 privacy considerations
https://www.w3.org/TR/vc-data-model-2.0/#privacy-considerations
Bitstring Status List correlation analysis for comparison.

### 13. Can accumulator status process work offline?

**OPEN**

Determine:

```text
can holder prove non-revocation offline without contacting authority?
can verifier check status offline without contacting authority or holder?
what offline artifact is required (signed offline status snapshot, accumulator epoch snapshot)?
what freshness policy allows offline reliance?
what evidence is required before verifier may rely on offline status?
```

Offline behavior must not assert network reachability, endpoint availability, or routing.
Offline artifact, if any, is adapter-only and must be versioned registry with independently
evidenced integrity and provenance.

### 14. What evidence is required before a verifier may rely on offline status?

**OPEN - EVIDENCE_REQUIRED**

Required:

```text
signed offline status artifact with validity interval
freshness policy (e.g., max age)
holder witness freshness policy
verifier audit duty for offline checks
incident-response duty if offline artifact is compromised
```

No verifier may rely on offline status until EVIDENCE_COLLECTED with signed policy source.

## Core Normalization

Regardless of accumulator family, adapter returns only:

```text
Active
Expired
Suspended
Unavailable
Unrecognized
```

Mapping:

```text
Active = accumulator shows not revoked/suspended, validity interval OK, witness valid, freshness OK
Expired = exp/nbf check fails (via Clock port), regardless of accumulator
Suspended = accumulator shows suspended, or suspension registry indicates suspended
Unavailable = witness unavailable, accumulator stale, authority unavailable, freshness policy fails, offline artifact unavailable
Unrecognized = credential reference not in accumulator domain, or status reference unknown
```

## Adapter Boundary

Prohibited from Skynet core, fixtures, provenance, audit, ports:

```text
accumulator value
witness bytes
epoch bytes
accumulator proof material
revocation list
suspension list
status URL
status resource
network endpoint
holder identifier
credential identifier as direct identifier
```

Allowed to core:

```text
CredentialStatus (closed enum)
PolicyAuthority, PolicyVersion, PolicyLineageReference
Audit reason code (CredentialStatusUnavailable, CredentialNotUsable, etc.)
```

Wallet adapter retains sealed CWT and witness material.
Transport adapter resolves PresentationCommitmentReference after policy approval.

## Required Evidence Matrix

| Topic | Evidence source | Evidence status | Privacy impact | Failure impact | Open question |
|---|---|---|---|---|---|
| Accumulator family | Primary spec per candidate (RSA, pairing, Merkle) | OPEN | EVIDENCE_REQUIRED - correlation risk analysis | EVIDENCE_REQUIRED - witness size, epoch update | Which family avoids trusted setup and correlation? |
| Revocation authority | Organizational policy source required | EVIDENCE_REQUIRED | EVIDENCE_REQUIRED | EVIDENCE_REQUIRED | Who is authority and how is compromise handled? |
| Epoch publication | Versioned registry model | OPEN | EVIDENCE_REQUIRED | EVIDENCE_REQUIRED | Publisher, integrity, provenance, retirement |
| Freshness | Signed policy source required | OPEN | EVIDENCE_REQUIRED | EVIDENCE_REQUIRED | Freshness representation and stale behavior |
| Witness lifecycle | Adapter design research | OPEN | EVIDENCE_REQUIRED - refresh linkability | EVIDENCE_REQUIRED - unavailable behavior | Obtain/refresh/validate without correlation |
| Suspension vs revocation | Accumulator spec | OPEN | EVIDENCE_REQUIRED | EVIDENCE_REQUIRED | Temporary vs permanent distinction |
| Authority compromise | Threat model | OPEN | EVIDENCE_REQUIRED | EVIDENCE_REQUIRED | Detection, equivocation, retirement |
| Correlation risk | W3C VC privacy considerations | OPEN | EVIDENCE_REQUIRED | EVIDENCE_REQUIRED | Can two checks be linked? |
| Offline behavior | Offline artifact policy | OPEN | EVIDENCE_REQUIRED | EVIDENCE_REQUIRED | Offline proof without network |
| Verifier reliance on offline | Signed offline policy | EVIDENCE_REQUIRED | EVIDENCE_REQUIRED | EVIDENCE_REQUIRED | What evidence required? |

## Conclusion Format

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

## Decision Preconditions

| Future decision | Required predecessor evidence |
|---|---|
| S6 adapter design | Accumulator family primary source, revocation authority evidenced, epoch model, freshness policy, witness lifecycle, suspension vs revocation, compromise response, correlation risk analysis, offline behavior, failure semantics, core boundary compatibility, threat-model review |
| Combined F6+S6+P3+H5 acceptance | S6 freshness and offline policy EVIDENCE_COLLECTED, no correlation without mitigation, failure semantics produce only closed status, no accumulator/witness material in core |

## Prohibited Actions

```text
No accumulator construction selected
No witness format selected
No revocation authority selected
No epoch publisher selected
No cryptographic library or dependency selected
No fixture, ALN, or Rust file generation
No status URL dereferencing in core
No network, device, location, or municipal claim
```

## Fixed Bindings

```text
host_did = didalnorganic-host - external ALN governance only, not core
bostrom_address = bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7 - external ALN governance only
aln_authority = ALN.MIGRATION.CYBERCORE_AUTHORITY.v1 - policy lineage reference only as PolicyAuthority
```
Core policy lineage is only PolicyAuthority, PolicyVersion, PolicyLineageReference.
