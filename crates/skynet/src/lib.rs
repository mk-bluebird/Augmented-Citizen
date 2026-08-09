#![doc = r#"
# Skynet

`skynet` is a privacy-preserving civic identity, consent, and policy-lineage
core for narrowly scoped credential interactions.

## Core boundary

The core accepts only minimized, typed evidence:

- `CredentialStatus`
- `HolderAuthorization`
- `PolicyLineage`
- `DisclosureReceipt`

The core produces only:

- `EligibilityDecision`
- `AuditEvent`

This initial foundation exports only the modules needed to model opaque
references, deployment profiles, consent scopes, normalized status, policy
lineage, privacy constraints, audit records, deterministic errors, and
invariant checks.

## Prohibited data

No Skynet core-domain public type may contain:

- Raw credentials or credential claim values.
- Raw presentations or credential-format payloads.
- Holder names, direct identifiers, public keys, or wallet keys.
- Cryptographic proofs, request challenges, or nonce values.
- Verifier routing data, network payloads, or continuous location.
- Raw neural, EEG, BCI, biometric, physiological, clinical, or subjective data.
- Device serial numbers, device-internal state, or free-text audit narratives.

Credential parsing, status-list retrieval, cryptographic proof verification,
wallet access, verifier transport, and external network access belong to
separately reviewed adapter crates. They are not part of this core foundation.

## Safety posture

The crate denies unsafe Rust. Policy-critical behavior is represented by narrow
types, closed enums, deterministic validation, content-minimized audit records,
and explicit invariant functions.
"#]
#![deny(unsafe_code)]
#![forbid(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(rustdoc::bare_urls)]

/// Content-minimized audit event types and validation.
pub mod audit;

/// Purpose-specific, time-bounded, and revocable consent-scope types.
pub mod consent;

/// Versioned deployment-profile types and policy-bound validation.
pub mod deployment;

/// Closed error taxonomy for deterministic Skynet failures.
pub mod error;

/// Opaque holder-controlled identity-reference validation.
pub mod identity;

/// Pure functions that enforce Skynet privacy and policy invariants.
pub mod invariants;

/// Prohibited-data classifications and core-boundary rules.
pub mod privacy;

/// Versioned policy-authority and policy-lineage records.
pub mod provenance;

/// Normalized credential-status types.
pub mod status;

/// Opaque identifiers, bounded values, and shared core-domain primitives.
pub mod types;

pub use audit::{
    ActorRole,
    AuditAction,
    AuditEvent,
    AuditOutcomeCode,
    AuditRecord,
};
pub use consent::{
    ConsentScope,
    ConsentState,
    ConsentWithdrawal,
};
pub use deployment::{
    DeploymentProfile,
    DeploymentProfileVersion,
};
pub use error::{
    SkynetError,
    SkynetResult,
};
pub use identity::CitizenIdentityReference;
pub use provenance::{
    PolicyAuthorityReference,
    PolicyLineage,
    PolicyRuleReference,
    PolicyVersion,
};
pub use status::CredentialStatus;
pub use types::{
    AuditEventId,
    ConsentScopeId,
    CredentialReference,
    DecisionReceiptId,
    DisclosureDescriptorSetId,
    PresentationRequestId,
    UtcTimestamp,
    VerifierReference,
};
