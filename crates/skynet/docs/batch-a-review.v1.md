# Skynet Batch A Review v1

## Review Scope

Reviewed evidence:

```text
Cargo.toml
LICENSE text
.github/workflows/static.yml
docs/BUILD.md
docs/CONTRIBUTING.md
aln-core-spec/consent.core-schema.aln
aln-core-spec/host.core-schema.aln
aln-core-spec/bostrom-authority.binding.aln
aln-core-spec/rights.constraints.global-invariants.aln
aln-audit-ledger/audit.schema.events.aln
```

## Gate Status

```text
Workspace membership: conditional
License: blocked
Rust CI: blocked
ALN grammar: partially evidenced
Consent semantics: partially reusable
Audit inheritance: rejected
Skynet source generation: blocked
```

## Approved Workspace Facts

```text
workspace_edition = "2024"
workspace_rust_version = "1.85"
workspace_resolver = "2"
workspace_serde_version = "1.0.204"
workspace_serde_json_version = "1.0.120"
```

`crates/skynet` must use workspace inheritance for approved shared
dependencies. It must be added once to the root workspace member list after
workspace metadata is validated.

## Blocking Workspace Defects

1. The workspace member list contains duplicate paths.
2. Root-level package feature declarations require relocation or a dedicated
   package because the root manifest is virtual.
3. Root-level target-specific dependency declarations require relocation to
   consuming member crate manifests.
4. The stated workspace Kani version conflicts with the required Skynet
   verification-toolchain version.
5. The workspace package license declaration conflicts with the supplied
   license text, which only establishes MIT terms.
6. No Rust or formal-verification CI workflow has been evidenced.

## Skynet Consent Projection

The canonical consent schema is reused only through a narrow projection:

```text
ConsentScope
  consent_scope_id
  holder_reference
  permitted_purposes
  valid_from
  valid_until
  policy_authority
  policy_version

HolderAuthorization
  holder_authorization_id
  presentation_request_id
  consent_scope_id
  holder_reference
  verifier_reference
  purpose
  valid_from
  valid_until
  policy_authority
  policy_version
```

Excluded source-schema fields:

```text
revocation_mechanism
cryptographic_signature
reason
data_scope
```

A local consent adapter may validate those source fields. The Skynet core does
not accept raw signatures, neural-command material, free-text reasons, or
datascope internals.

## Skynet Audit Projection

Skynet does not inherit `NeurorightsAuditEvents_v1` field sets.

```text
SkynetAuditEvent
  audit_event_id
  event_time
  presentation_request_id
  presentation_outcome_id
  holder_reference
  verifier_reference
  deployment_profile_id
  processing_purpose
  presentation_status
  credential_status
  reason_code
  policy_authority
  policy_version
  provenance_id
```

Excluded audit content:

```text
direct_host_did
bostrom_address
client_ip_or_node_id
notes
clinical_data
channel_data
neural_data
physiological_data
raw_credential
credential_claim_value
device_internal_state
continuous_location
free_text
```

## Required Evidence Before Source Generation

- Validated output from `cargo metadata --no-deps --format-version 1`.
- Root manifest corrected for duplicate members and virtual-manifest section use.
- Authoritative license decision.
- Workspace Kani compatibility decision.
- Rust CI workflow covering formatting, linting, tests, and Kani harnesses.
- `audit.index.schema.aln`.
- `augmented-id.age.policy.v1.aln`.
- `crates/ac-aln-core/Cargo.toml` and `src/lib.rs`.
- `crates/augmented-id-guard/Cargo.toml` and `src/lib.rs`.
- Existing Skynet specification and wiring-plan documents.
