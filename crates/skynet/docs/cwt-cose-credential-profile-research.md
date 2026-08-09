[cwt-cose-credential-profile-research (1).md](https://github.com/user-attachments/files/30867667/cwt-cose-credential-profile-research.1.md)
# F6 CWT/COSE Constrained Credential Profile Research

## Status

```text
artifact_type = research_gate
format_family = F6
family_description = CWT/COSE constrained credential profile
selection_state = CONDITIONAL_SELECTION_PENDING_EVIDENCE
format_selected = false
source_generation = prohibited
adapter_generation = prohibited
```

## Purpose

This document researches whether a CWT/COSE-based credential profile can satisfy
Skynet's opaque-reference, holder-controlled, purpose-limited, content-minimized
boundary.

It does not select a credential schema, claim vocabulary, COSE profile, issuer,
verifier, or cryptographic suite. CWT (RFC 8392) is a CBOR claims container
secured with COSE; it is not by itself a complete VC interoperability profile.
An exact profile must be identified before any adapter decision.

Primary sources:
- RFC 8392 CWT: https://datatracker.ietf.org/doc/rfc8392/
- RFC 9052 COSE: https://datatracker.ietf.org/doc/rfc9052/
- RFC 9334 CBOR Web Token claims discussion
- W3C VC Data Model 2.0 (data model reference, not wire format): https://www.w3.org/TR/vc-data-model-2.0/
- IETF SD-JWT VC tracking page for comparison (not selected): https://datatracker.ietf.org/doc/draft-ietf-oauth-sd-jwt-vc/

## Research Questions and Findings (OPEN)

### 1. Which exact CWT credential profile is proposed?

**OPEN - EVIDENCE_REQUIRED**

Candidates to evaluate (none selected):

```text
CWT with cnf claim for holder confirmation (RFC 8747)
CWT with custom credential-type claim under private claim namespace
CWT with exp/nbf/iat for validity interval
CWT with status reference claim pointing to external status artifact (not dereferenced by Skynet core)
CWT with selective disclosure via SD-CWT draft (IETF draft-ietf-spice-sd-cwt) - active Internet-Draft, track version-neutral URL
```

Evidence required before identifier can be proposed:

```text
primary spec source and version
private claim namespace registration or collision analysis
cnf claim structure and proof-of-possession model
interoperability evidence with at least one issuer ecosystem
interoperability evidence with at least one verifier ecosystem
offline verification evidence
```

No profile identifier may be marked SELECTED_FOR_ADAPTER_ONLY until evidence matrix
reaches EVIDENCE_COLLECTED.

### 2. Which fields are mandatory at the adapter boundary?

**OPEN**

Proposed mandatory fields for future adapter (not core) - all remain inside adapter,
never crossing into Skynet core as raw values:

```text
adapter input: sealed CWT (CBOR bytes) - never decoded by core
adapter internal: iss (issuer reference, opaque mapping only)
adapter internal: sub or cnf (holder confirmation material, never core)
adapter internal: exp, nbf, iat (validity interval, used only to derive Expired/Active for core)
adapter internal: cti or jti (credential identifier, stays in adapter)
adapter internal: status claim (status reference, not dereferenced by core, passed to CredentialStatusProvider only)
adapter internal: credential type claim (maps to CredentialTypeReference)
adapter output to core: CredentialTypeReference (opaque), CredentialFormatReference = F6, CredentialReference (opaque derived, not raw cti), expiry as UtcTimestamp for core to compare via Clock
```

Every field above must be documented with primary source and adapter impact.
No field is approved for core ingestion as raw claim value.

### 3. Which fields are prohibited from crossing into Skynet core?

**EVIDENCE_COLLECTED for prohibition list**

Prohibited from Skynet core types, ports, fixtures, provenance, audit:

```text
raw CWT CBOR bytes
claim values
cnf public key material
cnf proof material
credential proof material
public key material
signature material
credential identifier (cti/jti) as direct identifier
issuer identifier as direct identifier
holder identifier as direct identifier
subject identifier as long-lived identifier
status URL content
network endpoint
device identifier
biometric, neural, physiological, clinical, location, network metadata
UnboundedJsonValue, UnrestrictedMap, UnrestrictedList
FreeTextReason, FreeTextException
StablePseudonymousIdentifier that enables cross-session correlation
```

Core receives only:

```text
PolicyAuthority
PolicyVersion
PolicyLineageReference
CredentialTypeReference (opaque)
CredentialFormatReference (opaque, = F6 family)
CredentialReference (opaque, not raw cti)
CredentialStatus (closed enum from CredentialStatusProvider)
ClaimDescriptorId (closed set)
DisclosureProfileId
HolderAuthorization result (typed, not proof material)
PresentationCommitment (opaque sealed bytes from wallet adapter, never decoded by core)
```

### 4. How is credential type represented?

**OPEN**

Options (none selected):

```text
private claim namespace (e.g., vc_type or credential_type) mapping to CredentialTypeReference
COSE header parameter mapping
external type registry with opaque reference
```

Requirement: type representation must allow adapter to map protocol-specific type
to Skynet opaque `CredentialTypeReference` without exposing claim values to core.
Type must not be derived from network, device, location, or holder identifier.

### 5. How are validity interval and credential status represented?

**OPEN for interval, DEFERRED for status mechanism pending S6 research**

Validity interval:

```text
CWT exp (expiry), nbf (not-before), iat (issued-at) claims inside adapter
Adapter normalizes to: not_before, expires_at as UtcTimestamp for core comparison via Clock port
Core never receives exp/nbf/iat raw values, only normalized expiry result via credential_is_usable logic
```

Credential status:

```text
Skynet core never dereferences a credential status URL, retrieves a status resource,
parses a status list, or receives a status payload.
Future CredentialStatusProvider adapter may resolve status via F6-specific status claim
under approved privacy, freshness, and failure policy, then returns only closed CredentialStatus enum:
Active, Expired, Suspended, Unavailable, Unrecognized
Status resolution details deferred to accumulator-status-research.md (S6)
```

### 6. How can an adapter produce an eligibility result without exposing claim values?

**OPEN - EVIDENCE_REQUIRED**

Research question:

```text
Can adapter evaluate requested ClaimDescriptorIds against allowed descriptors
and produce PresentationOutcome with disclosed_claim_descriptor_ids only,
without passing claim values into Skynet core or audit?
```

Required evidence:

```text
eligibility logic inside adapter
mapping from CWT claim names to ClaimDescriptorId
proof that core and audit receive only descriptor IDs, not values
privacy analysis showing no claim value leakage
```

This is mandatory for Skynet compatibility. If adapter cannot prove eligibility without
claim-value exposure, F6 is REJECTED_FOR_SKYNET.

### 7. How is selective disclosure achieved, if supported?

**OPEN - EVIDENCE_REQUIRED**

Candidates (none selected):

```text
SD-CWT (IETF draft-ietf-spice-sd-cwt) - selective disclosure for CWT
COSE selective disclosure via claim redaction
No selective disclosure - full credential disclosure inside adapter, but only descriptor IDs cross to core (still requires privacy review)
```

Evaluation criteria:

```text
disclosure mechanism primary source and version
whether disclosure produces unlinkable presentations
whether disclosure requires holder key proof
whether disclosure increases adapter complexity
whether disclosure enables verifier-specific presentations without long-lived identifier
```

SD-CWT is an active Internet-Draft; must reference version-neutral tracking URL
https://datatracker.ietf.org/doc/draft-ietf-spice-sd-cwt/ plus specific revision and retrieval date when evaluating.

### 8. How is holder binding represented?

**OPEN - EVIDENCE_REQUIRED, linked to H5 research**

F6 holder binding must use:

```text
cnf claim (confirmation) per RFC 8747 - holder proof-of-possession
or COSE_Key in cnf
or external wallet-held key proof
```

Binding material must never enter Skynet core. Core receives only HolderAuthorization
result with mandatory bindings:

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

No DID, key, proof, credential, challenge, route, endpoint may appear in core result.
Holder binding evaluation and replay prevention details deferred to self-issued-holder-binding-research.md (H5).

### 9. How is verifier-specific presentation unlinkability evaluated?

**OPEN - EVIDENCE_REQUIRED**

Anti-correlation question (required):

```text
Can F6 produce a verifier-specific, purpose-specific presentation without exposing
a long-lived holder identifier, credential identifier (cti/jti), issuer identifier,
or reusable presentation identifier?
```

Evidence required:

```text
linkability analysis per W3C VC Data Model 2.0 privacy considerations https://www.w3.org/TR/vc-data-model-2.0/#privacy-considerations
whether cti/jti is stable pseudonymous identifier
whether sub is stable
whether cnf enables correlation
whether SD-CWT disclosures enable correlation
adapter mitigation (e.g., per-verifier nonce, per-presentation identifier)
```

If F6 cannot avoid long-lived identifiers, it must be marked REJECTED_FOR_SKYNET or
require explicit holder privacy impact assessment and independent authorization.

### 10. What is interoperability evidence for issuer and verifier ecosystems?

**OPEN - EVIDENCE_REQUIRED**

Required:

```text
issuer ecosystem that can issue CWT-based credential with cnf and type claim
verifier ecosystem that can verify CWT COSE signature and cnf proof
at least one documented implementation or test vector
offline verification test evidence
failure behavior evidence (expired, suspended, unavailable)
```

No issuer or verifier may be claimed as supported without primary source or test evidence.

### 11. What are offline verification constraints?

**OPEN**

Constraints to document:

```text
COSE signature verification without network (requires pre-distributed issuer keys via registry with independently evidenced integrity)
status freshness without network (requires S6 offline status snapshot or accumulator epoch policy)
validity interval check via Clock port only, no system time
no network fetch for issuer keys, status, or revocation in Skynet core - all in adapter under approved policy
```

Offline behavior must not assert network reachability, endpoint availability, or routing.

## Required Evidence Matrix

| Topic | Evidence source | Evidence status | Privacy impact | Core boundary impact | Open question |
|---|---|---|---|---|---|
| Exact CWT profile | RFC 8392, RFC 9052, SD-CWT tracking page | OPEN | EVIDENCE_REQUIRED | EVIDENCE_REQUIRED | Which private claim namespace and cnf structure? |
| Mandatory adapter fields | RFC 8392 claims registry | OPEN | EVIDENCE_REQUIRED | EVIDENCE_COLLECTED for prohibition list | Can expiry be normalized without claim exposure? |
| Type representation | Private claim or external registry | OPEN | EVIDENCE_REQUIRED | EVIDENCE_REQUIRED | How to map to CredentialTypeReference opaquely? |
| Validity and status | RFC 8392 exp/nbf/iat, S6 research | OPEN | EVIDENCE_REQUIRED | EVIDENCE_COLLECTED - core receives only closed enum | How does status claim map to CredentialStatusProvider? |
| Eligibility without claim values | Adapter logic | EVIDENCE_REQUIRED | EVIDENCE_REQUIRED | EVIDENCE_REQUIRED | Can adapter prove eligibility with only descriptor IDs? |
| Selective disclosure | SD-CWT draft tracking page https://datatracker.ietf.org/doc/draft-ietf-spice-sd-cwt/ | OPEN | EVIDENCE_REQUIRED | EVIDENCE_REQUIRED | SD-CWT vs full disclosure privacy impact |
| Holder binding | RFC 8747 cnf, H5 research | OPEN | EVIDENCE_REQUIRED | EVIDENCE_COLLECTED - proof material never core | cnf proof-of-possession model still OPEN |
| Unlinkability | W3C VC 2.0 privacy considerations https://www.w3.org/TR/vc-data-model-2.0/#privacy-considerations | OPEN | EVIDENCE_REQUIRED | EVIDENCE_REQUIRED | Long-lived cti/jti/sub correlation risk |
| Interoperability | Issuer/verifier implementation evidence | EVIDENCE_REQUIRED | EVIDENCE_REQUIRED | EVIDENCE_REQUIRED | No implementation evidenced yet |
| Offline verification | Registry integrity model | OPEN | EVIDENCE_REQUIRED | EVIDENCE_REQUIRED | Offline key distribution and status freshness |

## Conclusion Format (Required)

```text
F6_PROFILE_IDENTIFIER = OPEN
F6_INTEROPERABILITY = OPEN
F6_PRIVACY_COMPATIBILITY = OPEN
F6_CORE_BOUNDARY_COMPATIBILITY = OPEN
F6_SELECTIVE_DISCLOSURE = OPEN
F6_HOLDER_BINDING = OPEN
F6_UNLINKABILITY = OPEN
F6_OFFLINE_BEHAVIOR = OPEN
F6_EVIDENCE_STATUS = EVIDENCE_REQUIRED
```

No conclusion may be marked EVIDENCE_COLLECTED until primary source, interoperability evidence, privacy analysis, linkability analysis, and core-boundary compatibility are documented.

## Decision Preconditions

| Future decision | Required predecessor evidence |
|---|---|
| F6 adapter design | Exact profile identifier, interoperability evidence, privacy compatibility, core boundary compatibility, selective disclosure analysis, holder binding analysis, unlinkability analysis, offline behavior, failure semantics, threat-model review |
| Combined F6+S6+P3+H5 acceptance | F6 privacy compatibility EVIDENCE_COLLECTED, S6 epoch/witness/freshness defined, P3 adapter-only transport with explicit approval, H5 mandatory bindings enforced, no prohibited data in core |

## Prohibited Actions

```text
No credential schema selected
No claim vocabulary selected
No COSE profile selected
No issuer selected
No verifier selected
No cryptographic dependency selected
No source code, fixture, ALN, or Rust file generation
No municipal, location, or infrastructure claim
```

## Fixed Bindings

```text
host_did = didalnorganic-host - external ALN governance only, not in Skynet core types
bostrom_address = bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7 - external ALN governance only
aln_authority = ALN.MIGRATION.CYBERCORE_AUTHORITY.v1 - policy lineage reference only as PolicyAuthority
```

Policy lineage in core is only PolicyAuthority, PolicyVersion, PolicyLineageReference.
