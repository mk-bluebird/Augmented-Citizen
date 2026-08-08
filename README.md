# Augmented-Citizen

Augmented-Citizen is a rights-oriented engineering repository for
privacy-preserving identity, consent, and governance systems used by
augmented citizens.

The repository treats the person as the primary sovereign stakeholder.
It provides implementable contracts for cognitive liberty, mental privacy,
identity continuity, explicit consent, and accountable infrastructure
interactions. It does not collect, infer from, or expose raw neural,
physiological, credential-claim, device-internal, network-payload, or
continuous-location data.

`crates/skynet` is the repository's civic identity and
credential-routing core. It evaluates narrowly scoped, holder-authorized
credential presentations without retaining credential claims or creating
a surveillance profile.

> **Important:** This repository is not evidence of authorization by any
> government, municipality, device manufacturer, credential issuer, or
> infrastructure provider. Deployment labels and policy references are
> application-defined until a documented, independently authorized
> integration exists.

## Mission

- Preserve cognitive sovereignty and mental privacy for augmented citizens.
- Implement holder-controlled identity and credential presentation boundaries.
- Require explicit, purpose-specific, time-bounded, and revocable consent.
- Prevent non-consensual surveillance, manipulation, data export, and
  capability reduction by design.
- Provide auditable Rust, ALN, policy, and documentation artifacts for
  rights-preserving augmented-citizen infrastructure.
- Support monotone evolution: future changes may add safety and freedom but
  must not silently reduce a host's protected capabilities or rights.

## Identity Model

Augmented-Citizen distinguishes four identity layers:

| Layer | Purpose | Excluded content |
|---|---|---|
| `CitizenIdentityReference` | Opaque local reference to a holder-controlled identity | Name, raw biometrics, neural data, direct identifiers |
| `CredentialReference` | Opaque reference to a credential retained by the holder wallet | Credential claims and raw credential payload |
| `VerifierReference` | Reference to an approved requesting service | Unverified verifier metadata |
| `DeploymentProfile` | Versioned policy-bound infrastructure profile | Real-time location, network traces, device serials |

Identity is not a behavioral score, a diagnosis, a surveillance record, or a
claim of municipal authorization.

Credential claims remain within a holder-controlled wallet or a
format-specific adapter. The core policy layer processes only opaque
references, declared claim descriptors, policy lineage, status results,
holder authorization results, and content-minimized audit facts.

## Skynet Civic Identity Core

`crates/skynet` is a privacy-preserving civic identity and
credential-routing core. It coordinates a credential interaction only after
all required checks succeed:

1. Validate the named deployment profile.
2. Validate the requesting verifier against an approved registry.
3. Load an ALN-derived policy projection with complete authority and version.
4. Validate the declared processing purpose.
5. Validate active, purpose-matched consent.
6. Obtain explicit holder authorization from a local trusted interface.
7. Evaluate credential status through a status-provider port.
8. Verify that requested claim descriptors are within the approved disclosure profile.
9. Construct a sealed verifier-addressed presentation through a wallet adapter.
10. Transport only the approved sealed presentation.
11. Record a content-minimized audit event.

The core crate does not parse a raw credential, issue credentials, perform
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

## Rights and Safety Principles

### Cognitive sovereignty

The augmented citizen retains authority over their thoughts, memories,
decision-making, identity bindings, and AI-assisted reasoning. No repository
component may treat cognitive state as a commodity, a credential, or an
infrastructure access condition.

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

This repository must not be used to build systems for covert cognitive
manipulation, non-consensual behavioral influence, population control,
surveillance scoring, or discrimination against augmented citizens.

## Governance Bindings

When ALN- or BioPay-governed artifacts are enabled, host-critical policy
projections must bind to the following governance constants:

```text
host_did = didalnorganic-host
bostrom_address = bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7
aln_authority = ALN.MIGRATION.CYBERCORE_AUTHORITY.v1
```

These values establish policy lineage for governed artifacts. They are not
public credential claims, they do not authorize third-party access, and they
must not be copied into credential presentations or content-minimized audit
events unless an approved policy explicitly requires their opaque reference.

`mk-bluebird/Cybercore` is the intended authority for cybernetic evolution
artifacts and ALN policy lineage, subject to verification against the active
repository governance documents.

## Deployment Profiles

The initial `skynet` deployment label is:

```text
PHX_AZ_US
```

This means only:

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
│   └── Additional rights-preserving cybernetic and policy crates
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

## Standards Position

Augmented-Citizen uses open credential standards as interoperability references,
not as a reason to centralize personal data.

- W3C Verifiable Credentials Data Model 2.0 informs the issuer-holder-verifier
  credential model and credential data representation.
- OpenID for Verifiable Presentations informs presentation-request and
  wallet-to-verifier exchange adapters.
- NIST SP 800-63-4 informs assurance, privacy, identity-proofing, and
  authentication planning.

The `skynet` core remains format-neutral. W3C VC, ISO mdoc, SD-JWT VC, or
other credential formats must be introduced through separately reviewed
adapters after the repository's credential-profile research gate is complete.

## Installation

Before building, inspect the active workspace manifest, crate membership, and
repository-specific build instructions.

Typical Rust workspace workflow:

```bash
git clone https://github.com/mk-bluebird/Augmented-Citizen.git
cd Augmented-Citizen
cargo build --workspace
cargo test --workspace
```

Do not connect a wallet, verifier, municipal service, device, BCI, telemetry
source, or external transport merely by building this repository.

## Contribution Rules

Contributions must:

- Preserve holder control, mental privacy, and explicit consent.
- Use opaque references instead of sensitive identifiers or payloads.
- Keep raw credential claims outside core policy, provenance, and audit types.
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
- Add direct network access to the `skynet` core.
- Add raw credential parsing or generic payload fields to core types.
- Add silent capability reductions, hidden control paths, or non-consensual
  behavior-control features.
- Represent an application deployment label as verified real-world authority.

## Security Posture

The primary threats considered by this repository include:

- Unauthorized disclosure of credential claims.
- Unauthorized presentation of a holder credential.
- Verifier impersonation or verifier-policy mismatch.
- Overbroad claim requests.
- Stale, suspended, expired, unavailable, or unrecognized credential status.
- Incomplete policy lineage or unrecognized deployment profiles.
- Audit data becoming a secondary surveillance store.
- Coercive, deceptive, or non-consensual interaction flows.

The principal mitigations are holder authorization, narrow disclosure profiles,
typed policy evaluation, verifier registries, status evaluation, sealed
presentation transport, closed audit schemas, and independently testable
invariants.

## Limitations

This repository is an engineering and governance framework. It is not legal
advice, medical advice, clinical-device validation, an identity-proofing
service, a credential issuer, or an authorization from a municipality or
infrastructure operator.

Actual protection depends on correct implementation, independent review,
holder-controlled deployment, trustworthy adapters, applicable law, and
documented agreements with issuers, verifiers, device makers, and service
operators.

## License

See the repository license files for the authoritative licensing terms. No crate
manifest should declare a license value until the repository license has been
verified.

## References

- [W3C Verifiable Credentials Data Model v2.0](https://www.w3.org/TR/vc-data-model-2.0/)
- [OpenID for Verifiable Presentations 1.0](https://openid.net/specs/openid-4-verifiable-presentations-1_0.html)
- [NIST SP 800-63-4 Digital Identity Guidelines](https://pages.nist.gov/800-63-4/)
