# Skynet

Skynet is a rights-oriented engineering repository for privacy-preserving
identity, consent, policy lineage, and accountable infrastructure interactions
for sovereign persons using integrated or assistive technology.

The repository treats the person as the primary sovereign stakeholder. It
provides implementable contracts for cognitive liberty, mental privacy,
identity continuity, explicit consent, and narrowly scoped credential
presentation.

Skynet does not collect, infer from, retain, or expose raw neural,
physiological, credential-claim, device-internal, network-payload, or
continuous-location data.

`crates/skynet` is the repository's civic identity and credential-routing
core. It evaluates holder-authorized credential presentations without retaining
credential claims or creating a surveillance profile.

> **Important:** This repository does not establish authorization by a
> government, municipality, device manufacturer, credential issuer, verifier,
> or infrastructure provider. Deployment labels and policy references remain
> application-defined until a documented, independently authorized integration
> exists.

## Mission

- Preserve cognitive sovereignty and mental privacy.
- Implement holder-controlled identity and credential-presentation boundaries.
- Require explicit, purpose-specific, time-bounded, and revocable consent.
- Prevent non-consensual surveillance, manipulation, data export, and
  capability reduction by design.
- Provide auditable Rust, ALN, policy, and documentation artifacts for
  rights-preserving infrastructure.
- Support monotone evolution: future changes may add safety and freedom but
  must not silently reduce a host's protected capabilities or rights.

## Identity Model

Skynet distinguishes four identity layers:

| Layer | Purpose | Excluded content |
|---|---|---|
| `CitizenIdentityReference` | Opaque local reference to a holder-controlled identity | Names, raw biometrics, neural data, direct identifiers |
| `CredentialReference` | Opaque reference to a credential retained by the holder wallet | Credential claims and raw credential payloads |
| `VerifierReference` | Reference to an approved requesting service | Unverified verifier metadata |
| `DeploymentProfile` | Versioned, policy-bound infrastructure profile | Real-time location, network traces, device serials |

Identity is not a behavioral score, clinical diagnosis, surveillance record, or
claim of municipal authorization.

Credential claims remain within a holder-controlled wallet or a
format-specific adapter. The Skynet policy core processes only opaque
references, declared claim descriptors, policy lineage, normalized status
results, holder-authorization results, and content-minimized audit facts.

## Civic Identity Core

`crates/skynet` is a privacy-preserving civic identity and credential-routing
core. It coordinates a credential interaction only after all required checks
succeed:

1. Validate the named deployment profile.
2. Validate the requesting verifier against an approved registry.
3. Load an ALN-derived policy projection with complete authority and version.
4. Validate the declared processing purpose.
5. Validate active, purpose-matched consent.
6. Obtain explicit holder authorization through a local trusted interface.
7. Evaluate credential status through a status-provider port.
8. Verify that requested claim descriptors are within the approved disclosure profile.
9. Construct a sealed verifier-addressed presentation through a wallet adapter.
10. Transport only the approved sealed presentation.
11. Record a content-minimized audit event.

The core crate does not parse raw credentials, issue credentials, perform
identity proofing, connect automatically to civic systems, access external
networks, or store credential claims.

### Presentation Boundary

```text
Credential issuer
        |
        v
Holder-controlled wallet
        |
        | holder authorization + sealed presentation commitment
        v
Skynet policy core
        |
        +--> deployment validation
        +--> verifier authorization
        +--> consent evaluation
        +--> credential-status evaluation
        +--> disclosure minimization
        +--> invariant validation
        |
        v
Approved transport adapter
        |
        v
Verifier

Skynet audit sink receives only opaque references,
closed reason codes, policy lineage, and timestamps.
```

## Rights and Safety

### Cognitive sovereignty

The holder retains authority over thoughts, memories, decision-making, identity
bindings, and AI-assisted reasoning. No component may treat cognitive state as
a commodity, credential, behavioral score, or infrastructure-access condition.

### Explicit holder authorization

Credential presence, network context, device state, historical behavior, or
status checks do not constitute consent. A presentation requires current,
request-bound, verifier-bound, purpose-specific holder authorization.

### Data minimization

Public contracts, audit records, and policy inputs must not contain:

- Raw neural, EEG, BCI, physiological, clinical, or subjective data.
- Credential claim values, raw credentials, or raw presentations.
- Continuous location, network payloads, packet captures, or radio traces.
- Device serial numbers, device-internal state, or generic payload fields.
- Free-text narratives in audit records.

### Capability-preserving evolution

Host-critical changes must be transparent, auditable, and governed by explicit
host authorization. Silent downgrades, covert restrictions, hidden kill
switches, and non-consensual behavior-control mechanisms are prohibited.

### Non-weaponization

Skynet must not be used to build systems for covert cognitive manipulation,
non-consensual behavioral influence, population control, surveillance scoring,
or discrimination.

## Governance Bindings

When ALN- or BioPay-governed artifacts are enabled, host-critical policy
projections bind to the following governance constants:

```text
host_did = didalnorganic-host
bostrom_address = bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7
aln_authority = ALN.MIGRATION.CYBERCORE_AUTHORITY.v1
```

These values establish policy lineage for governed artifacts. They are not
public credential claims, do not authorize third-party access, and must not be
copied into credential presentations or content-minimized audit events unless
an approved policy requires an opaque reference.

Cybercore is the intended authority for cybernetic-evolution artifacts and ALN
policy lineage, subject to verification against active governance documents.

## Deployment Profiles

The initial Skynet deployment label is:

```text
PHX_AZ_US
```

This label means only:

```text
Application-defined Phoenix, Arizona, United States deployment profile.
```

It does not establish:

- City of Phoenix affiliation, approval, or service access.
- Residency, street address, or real-time physical location.
- A municipal credential or city-service account.
- A live infrastructure connection.

A deployment profile may be used only when it has a documented policy
authority, policy version, verifier-registry reference, expiry behavior, and
change-management process.

## Repository Structure

```text
.
├── aln/
│   └── Rights, policy-lineage, and governance artifacts
├── crates/
│   ├── skynet/
│   │   ├── Civic identity, consent, disclosure, and credential-routing core
│   │   ├── Typed ports for wallet, status, transport, registry, and audit adapters
│   │   └── Tests, fixtures, and formal-verification harnesses
│   └── Additional rights-preserving policy and infrastructure crates
├── docs/
│   └── Architecture, threat models, research gates, and implementation guidance
├── policy/
│   └── Machine-readable consent, disclosure, audit, and governance policies
└── ota/
    └── Governed upgrade manifests and capability-preservation policies
```

The exact workspace layout is authoritative only after review of the root Cargo
workspace manifest and repository governance documents.

## Engineering Requirements

Host-critical Rust crates are expected to use:

```text
edition = "2024"
rust-version = "1.85"
```

Each crate must define:

- A narrow, typed public API.
- Explicit prohibited-data boundaries.
- Deterministic policy and invariant checks.
- Content-minimized audit contracts.
- Test fixtures containing only opaque references and closed enum values.
- Formal-verification targets where state-machine or privacy invariants are
  safety-critical.
- Adapter ports for external systems instead of embedded network, wallet, or
  credential-format dependencies.

No source file should be generated until its upstream contracts, required
ports, invariants, fixtures, and test obligations are defined.

## Credential Profile Position

Skynet uses open credential standards as interoperability references, not as a
reason to centralize personal data.

The core remains credential-format-neutral. Credential profiles must be
introduced through separately reviewed adapters after their profile research
gate is complete.

A credential adapter must provide only:

- An `EligibilityDecision`.
- A normalized `CredentialStatus`.
- A `HolderAuthorization` result.
- A `PolicyLineage` reference.
- A content-minimized `DisclosureReceipt`.
- A closed, content-minimized `AuditEvent`.

The core must not receive credential claims, raw credential payloads, holder
keys, direct identifiers, cryptographic proof material, request challenges, or
transport-routing metadata.

## Formal Invariants

Skynet implementation work must preserve these core properties:

```text
SKY-I-001: Core policy evaluation receives no raw credential or claim values.
SKY-I-002: Holder authorization is bound to request, verifier, purpose,
           consent scope, policy version, and validity interval.
SKY-I-003: Stale, unavailable, or unrecognized status evidence never becomes Active.
SKY-I-004: Audit records cannot become a secondary credential or surveillance store.
SKY-I-005: Every policy decision contains reproducible authority and version lineage.
SKY-I-006: Deployment labels cannot be treated as real-time location evidence.
SKY-I-007: Capability changes require transparent, host-authorized governance.
```

## Installation

Before building, inspect the active workspace manifest, crate membership, and
repository-specific build instructions.

Typical Rust workspace workflow:

```bash
cargo build --workspace
cargo test --workspace
```

Do not connect a wallet, verifier, municipal service, device, BCI, telemetry
source, or external transport merely by building this repository.

## Contribution Rules

Contributions must:

- Preserve holder control, mental privacy, and explicit consent.
- Use opaque references instead of sensitive identifiers or payloads.
- Keep raw credential claims outside policy, provenance, and audit types.
- Keep neural and physiological data outside public contracts.
- Define consent, disclosure, deployment, verifier, and audit implications.
- Add deterministic tests and applicable formal-proof targets.
- Document adapter trust assumptions, failure modes, retention behavior, and
  revocation behavior.
- Avoid claims of affiliation, authorization, clinical efficacy, or legal
  enforceability that cannot be independently documented.

Contributions must not:

- Add covert collection, inference, manipulation, or tracking features.
- Treat biosignal-derived output as consent.
- Add direct network access to the Skynet policy core.
- Add raw credential parsing or generic payload fields to core types.
- Add silent capability reductions, hidden control paths, or non-consensual
  behavior-control features.
- Represent an application deployment label as verified real-world authority.

## Security Posture

Primary threats considered by Skynet include:

- Unauthorized disclosure of credential claims.
- Unauthorized presentation of a holder credential.
- Verifier impersonation or verifier-policy mismatch.
- Overbroad claim requests.
- Stale, suspended, expired, unavailable, or unrecognized credential status.
- Incomplete policy lineage or unrecognized deployment profiles.
- Audit data becoming a secondary surveillance store.
- Coercive, deceptive, or non-consensual interaction flows.
- Replay, purpose substitution, cross-verifier substitution, and expiry bypass.
- Adapter-to-core leakage of credential, identity, or routing information.

Principal mitigations include holder authorization, narrow disclosure profiles,
typed policy evaluation, verifier registries, status evaluation, sealed
presentation transport, closed audit schemas, and independently testable
invariants.

## Limitations

Skynet is an engineering and governance framework. It is not legal advice,
medical advice, clinical-device validation, an identity-proofing service, a
credential issuer, or authorization from a municipality or infrastructure
operator.

Actual protection depends on correct implementation, independent review,
holder-controlled deployment, trustworthy adapters, applicable law, and
documented agreements with issuers, verifiers, device makers, and service
operators.

## License

See repository license files for authoritative licensing terms. No crate
manifest should declare a license value until repository licensing has been
verified.

## References

- [W3C Verifiable Credentials Data Model v2.0](https://www.w3.org/TR/vc-data-model-2.0/)
- [OpenID for Verifiable Presentations 1.0](https://openid.net/specs/openid-4-verifiable-presentations-1_0.html)
- [NIST SP 800-63-4 Digital Identity Guidelines](https://pages.nist.gov/800-63-4/)
