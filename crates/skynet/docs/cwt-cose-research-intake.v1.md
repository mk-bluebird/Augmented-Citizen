# F6 CWT/COSE Research Intake v1

## Intake Result

```text
artifact_reviewed = cwt-cose-credential-profile-research.md
research_scope = accepted
format_selected = false
adapter_selected = false
implementation_authorized = false
F6_gate_state = OPEN
```

## Accepted Elements

```text
CWT is treated as a compact CBOR claims container, not a complete VC profile.
COSE profile selection remains open.
Issuer, verifier, claim vocabulary, and credential schema remain open.
F6 status resolution remains deferred to S6 accumulator research.
All raw CWT, claim, proof, key, issuer, holder, subject, status, endpoint,
network, device, biometric, neural, clinical, and location material remains
adapter-only.
Skynet core receives only closed typed results and opaque references.
F6 evidence matrices remain OPEN or EVIDENCE_REQUIRED.
```

## Mandatory Corrections

### Source correction

Remove:

```text
RFC 9334 CBOR Web Token claims discussion
```

RFC 9334 is not a CWT claims specification.

Retain as relevant research sources:

```text
RFC 8392
RFC 8747
RFC 9052
W3C Verifiable Credentials Data Model v2.0
IETF SD-CWT tracking page
```

### Presentation commitment correction

Replace every occurrence of:

```text
PresentationCommitment (opaque sealed bytes)
```

with:

```text
PresentationCommitmentReference
```

Rules:

```text
Skynet core stores no presentation bytes.
Skynet core receives no credential payload.
Skynet core receives no transport payload.
Wallet adapter retains sealed presentation material.
Transport adapter resolves the commitment reference only after policy approval.
Audit records retain only presentation outcome and opaque identifiers.
```

### Selective-disclosure correction

Replace:

```text
COSE selective disclosure via claim redaction
```

with:

```text
An independently specified CWT selective-disclosure mechanism,
requiring a primary specification, interoperability evidence,
linkability analysis, and adapter-boundary analysis.
```

No generic COSE feature may be presumed to provide selective disclosure.

### H5 composition correction

Add:

```text
RFC 8747 cnf proof-of-possession is not by itself H5 self-issued holder
authentication.

The H5 research gate must establish how a self-issued authentication artifact
is bound to the CWT proof-of-possession result, presentation_request_id,
verifier_reference, purpose, consent_scope_id, not_before, expires_at,
policy_authority, and policy_version.
```

## F6 Status

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
