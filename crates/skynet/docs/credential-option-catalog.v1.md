[credential-option-catalog.v1.md](https://github.com/user-attachments/files/30867526/credential-option-catalog.v1.md)
# Skynet Credential Option Catalog v1

## Status

```text
artifact_type = research_reference
format_selected = false
status_mechanism_selected = false
presentation_protocol_selected = false
issuance_protocol_selected = false
holder_binding_selected = false
```

## Selection Rule

No option may be selected unless every required evidence category is marked
`EVIDENCE_COLLECTED`.

```text
primary specification source
current specification version
interoperability evidence
holder-binding analysis
replay-resistance analysis
linkability analysis
selective-disclosure analysis
status and expiry behavior
offline behavior
failure behavior
verifier privacy impact
holder privacy impact
adapter complexity
retention impact
threat-model review
policy-authority review
```

## Core Boundary Rule

Regardless of the selected option:

```text
Skynet core receives no raw credential.
Skynet core receives no claim value.
Skynet core receives no protocol message.
Skynet core receives no key or proof material.
Skynet core receives no network endpoint.
Skynet core receives only opaque references, closed status results,
descriptor identifiers, holder-authorization result, policy lineage,
and content-minimized audit facts.
```

## Decision Outcome Values

```text
NOT_RESEARCHED
OPEN
EVIDENCE_REQUIRED
EVIDENCE_COLLECTED
DEFERRED
REJECTED_FOR_SKYNET
SELECTED_FOR_ADAPTER_ONLY
```

## Format Options

Choose one primary format family later, or explicitly defer selection.

| ID | Format option | Brief description | Skynet status |
|---|---|---|---|
| F0 | No format selected | Core research phase; Skynet accepts no raw credentials | Current state |
| F1 | W3C VC 2.0 + Data Integrity | W3C credential data model secured through Data Integrity proofs | Research candidate |
| F2 | W3C VC 2.0 + JOSE/COSE | W3C credential data model secured using JOSE or COSE mechanisms | Research candidate |
| F3 | SD-JWT VC | IETF credential format using selective disclosures from a signed JWT-based credential - see tracking page https://datatracker.ietf.org/doc/draft-ietf-oauth-sd-jwt-vc/ | Research candidate |
| F4 | ISO/IEC 18013-5 mdoc | Mobile document format used for mobile driving licences and related credential presentations | Research candidate |
| F5 | AnonCreds | Anonymous-credential family designed for privacy-preserving proofs and selective disclosure | Research candidate |
| F6 | CWT/COSE constrained credential profile | Compact CBOR/COSE-oriented profile for constrained or offline-capable environments - CWT is a CBOR claims container per RFC 8392, not a complete VC profile by itself | Research candidate |
| F7 | BBS-capable VC profile | VC profile using unlinkable selective-disclosure proof mechanisms | Research candidate |
| F8 | Existing issuer-specific format | A format imposed by a documented issuer or verifier ecosystem | Adapter-only; requires full privacy review |
| F9 | Proprietary or undocumented format | Non-standard or closed credential representation | Rejected unless primary specification, interoperability, and privacy evidence are provided |

Research requirement: F1 through F9 must be evaluated for linkability, selective disclosure, holder binding, expiry, status, verifier interoperability, offline behavior, failure behavior, and adapter complexity.

## Status-Mechanism Options

Skynet core never resolves a status reference itself. A future CredentialStatusProvider may use one of these mechanisms and return only the closed core result.

| ID | Status option | Brief description | Skynet status |
|---|---|---|---|
| S0 | No status mechanism selected | Core remains format-neutral | Current state |
| S1 | W3C Bitstring Status List | W3C Recommendation for publishing revocation or suspension through compressed bitstrings - https://www.w3.org/TR/vc-bitstring-status-list/ | Research candidate |
| S2 | StatusList2021 | Earlier VC status-list pattern; assess migration and interoperability limitations | Legacy research candidate |
| S3 | Issuer-operated status endpoint | Adapter retrieves status from an issuer-operated service | Research candidate; privacy and availability risk |
| S4 | Signed offline status snapshot | Adapter verifies a versioned, time-bounded, pre-fetched status artifact | Research candidate |
| S5 | Short-lived credential only | Credential expiry is relied upon; no separate revocation channel | Research candidate; limited suspension response |
| S6 | Cryptographic accumulator | Adapter checks non-revocation or revocation membership through an accumulator scheme - witness lifecycle, freshness, compromise handling are design obligations | Research candidate; high complexity |
| S7 | Registry-backed status assertion | A governed registry publishes a status result for a credential reference | Research candidate; governance-heavy |
| S8 | Issuer re-presentation requirement | Credential must be recently reissued or refreshed before use | Research candidate; issuer availability dependency |
| S9 | Proprietary status service | Non-standard provider-specific status behavior | Rejected unless fully documented and privacy-reviewed |

## Protocol Options

### Presentation protocols

| ID | Presentation protocol | Brief description | Skynet status |
|---|---|---|---|
| P0 | No presentation protocol selected | Local policy research only | Current state |
| P1 | OpenID4VP 1.0 | OpenID protocol for verifier requests and holder wallet presentations | Research candidate |
| P2 | ISO mdoc presentation flow | mdoc-specific device-to-reader or online presentation interaction | Research candidate |
| P3 | DIDComm-mediated presentation | Secure message-based presentation exchange - carries sender, recipient, threading, routing metadata that must remain inside transport adapter | Research candidate |
| P4 | Verifier-specific documented API | A documented verifier protocol outside OpenID or ISO profiles | Adapter-only research candidate |
| P5 | Offline local presentation queue | Local-only test fixture or simulated transport; no external delivery | Research candidate |
| P6 | Proprietary presentation protocol | Closed or undocumented exchange protocol | Rejected unless independently documented and reviewed |

### Issuance protocols

| ID | Issuance protocol | Brief description | Skynet status |
|---|---|---|---|
| I0 | No issuance protocol selected | Skynet does not issue credentials | Current state |
| I1 | OpenID4VCI 1.0 | OpenID protocol for issuing verifiable credentials to a holder wallet - https://openid.net/specs/openid-4-verifiable-credentials-issuance-1_0.html | Out-of-core research candidate |
| I2 | Issuer-specific documented API | Issuer-defined enrollment and credential-delivery interface | Out-of-core research candidate |
| I3 | ISO mdoc provisioning flow | Issuance/provisioning approach for mdoc ecosystems | Out-of-core research candidate |
| I4 | Proprietary issuance flow | Undocumented or closed credential issuance method | Rejected unless fully researched |

## Holder-Binding and Request-Authentication Options

| ID | Holder-binding option | Brief description | Skynet status |
|---|---|---|---|
| H0 | No binding method selected | Only the logical holder-authorization contract exists | Current state |
| H1 | Wallet-held key proof | Wallet proves control of credential-associated key material | Research candidate |
| H2 | Verifier nonce and audience binding | Presentation is bound to a verifier challenge and intended audience | Research candidate |
| H3 | Request-object integrity binding | Presentation is bound to an authenticated request object | Research candidate |
| H4 | Device-bound wallet credential | Wallet binding relies on an approved device-bound authenticator | Research candidate; device privacy review required |
| H5 | Holder self-issued authentication profile | Holder authenticates through a self-issued identity interaction - must be bound to request, verifier, purpose, expiry to prevent replay | Research candidate |
| H6 | Proprietary holder-binding method | Non-standard vendor method | Rejected unless privacy, replay, and interoperability evidence exists |

## Recommended Research-Agent Output

For every row above, require a one-page evaluation with:

- Standards maturity — final standard, active draft, legacy, or proprietary.
- Data exposure — what a holder, verifier, issuer, and adapter can observe.
- Correlation risk — whether repeat presentations are linkable.
- Offline behavior — issuance, status, verification, and expiry constraints.
- Failure semantics — exactly when Skynet must return Unavailable or decline.
- Adapter boundary — exact data entering and leaving the future adapter.
- Skynet compatibility — whether the core boundary can remain opaque and content-minimized.

No option should move beyond OPEN until those evidence records exist.

## Current Conditional Research Selection (Pending Evidence)

Per staff selection, the following families are marked CONDITIONAL_SELECTION_PENDING_EVIDENCE only - not deployed:

```text
Format: F6 CWT/COSE constrained credential profile
Status: S6 Cryptographic accumulator
Presentation: P3 DIDComm
Holder binding: H5 Self-issued holder authentication + mandatory request/verifier/purpose/expiry binding
```

These do not form a single off-the-shelf stack. Compatibility, privacy, replay-resistance, and adapter-boundary records are required before any implementation decision.
