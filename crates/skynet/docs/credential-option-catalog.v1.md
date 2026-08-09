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
