# Skynet Research Gate Review v1

## Reviewed Artifacts

```text
privacy-model.md
verifier-trust-research.md
deployment-profile-research.md
credential-profile-research.md
```

## Review Result

```text
research_artifacts_generated = true
research_artifacts_approved_as_drafts = true
research_gates_closed = false
source_generation_authorized = false
deployment_activation_authorized = false
verifier_activation_authorized = false
```

## Required Corrections

### Core policy-lineage boundary

Skynet core permits only:

```text
PolicyAuthority
PolicyVersion
PolicyLineageReference
```

Direct host bindings remain outside Skynet public contracts.

Fixed host DID didalnorganic-host and Bostrom address remain bound in external
ALN governance records only. They must not appear in Skynet public Rust type,
port input, fixture, provenance record, presentation result, or audit event.

### Research export boundary

Research export requires:

```text
independent holder authorization
separate data contract
separate retention policy
recipient agreement
privacy impact assessment
explicit prohibited-data validation
```

ResearchDerivedMetadataExport may not export direct identity, opaque references,
credential identifiers, issuer identifiers, verifier identifiers, claim values,
biophysical data, cognitive inferences, clinical data, device data, location,
network metadata, or cross-session correlators.

### Registry integrity wording

Until an integrity mechanism is selected, use:

```text
versioned registry with independently evidenced integrity and provenance
```

Do not state a signing, cryptographic, or transport mechanism as selected.

### Deployment profile boundary

A deployment profile is configuration identity only. It contains no routing,
endpoint, address, network, device, session, location, or availability field.

It may reference a future verifier registry and policy authority, but only as
opaque, versioned references.

### Credential-status boundary

Only CredentialStatusProvider may resolve status through a future adapter.
Skynet core accepts only a closed status result:

```text
Active
Expired
Suspended
Unavailable
Unrecognized
```

Skynet core never dereferences a credential status URL, retrieves a status
resource, parses a status list, or receives a status payload.

### Prohibited classes addition required

Privacy model must add:

```text
StablePseudonymousIdentifier
CrossSessionCorrelationIdentifier
IssuerControlledIdentifier
VerifierControlledIdentifier
PublicKeyMaterial
SignatureMaterial
CredentialProofMaterial
WalletHandle
EndpointReference
NetworkAddress
RouteIdentifier
DeviceIdentifier
UnboundedJsonValue
UnrestrictedMap
UnrestrictedList
FreeTextReason
FreeTextException
```

### Verifier separation rule required

A verifier reference may identify an approved service inside a registry, but
must not be derived from a network endpoint, device identifier, location,
credential claim, holder identifier, or mutable network session property.

### Negative tests for verifier registry

Future verifier-registry plan must include tests for:

```text
unrecognized verifier declines
expired registry entry declines
suspended verifier declines
verifier-purpose mismatch declines
verifier-disclosure mismatch declines
registry provenance missing declines
registry freshness unavailable declines
```

### Credential profile amendments required

- Replace SD-JWT VC draft-08 link with version-neutral tracking URL or specific draft revision plus retrieval date
- Add status-resolution rule: core never dereferences status URLs
- Add anti-correlation question: Can candidate format produce verifier-specific, purpose-specific presentation without exposing long-lived holder, credential, issuer, or reusable presentation identifier?

## Research-Gate Exit Criteria

The four research documents may advance from OPEN only when:

- All required evidence matrices have primary-source or signed-policy evidence.
- Every unresolved item is explicitly classified.
- No direct host binding is admitted into a Skynet core contract.
- No deployment, verifier, format, status, protocol, transport, or retention
  decision is implied by document wording.
- The separate threat model is complete and approved.
- The privacy model’s prohibited-data checks are converted into approved
  fixture and Kani-proof requirements.
