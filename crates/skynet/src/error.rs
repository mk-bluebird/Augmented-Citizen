//! Closed, content-minimized error taxonomy for the Skynet core.

use core::fmt;

/// Result type returned by Skynet core operations.
pub type SkynetResult<T> = Result<T, SkynetError>;

/// Closed error taxonomy for deterministic Skynet core failures.
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum SkynetError {
    /// An opaque reference violates its representation contract.
    InvalidOpaqueReference {
        /// Category of opaque reference that failed validation.
        kind: OpaqueReferenceKind,
        /// Closed reason describing the validation failure.
        violation: OpaqueReferenceViolation,
    },

    /// A time-bounded object has an invalid validity interval.
    InvalidTemporalWindow {
        /// Category of interval that failed validation.
        kind: TemporalWindowKind,
        /// Closed reason describing the temporal violation.
        violation: TemporalWindowViolation,
    },

    /// The requested deployment profile is absent, expired, or not accepted.
    UnknownDeploymentProfile {
        /// Closed reason describing why the deployment profile is unavailable.
        reason: DeploymentProfileFailure,
    },

    /// A consent scope is invalid, inactive, withdrawn, or mismatched.
    InvalidConsentScope {
        /// Closed reason describing the consent-scope failure.
        reason: ConsentScopeFailure,
    },

    /// Holder authorization is not valid for the requested interaction.
    InvalidHolderAuthorizationBinding {
        /// Binding that failed validation.
        binding: HolderAuthorizationBinding,
        /// Closed reason describing the binding failure.
        reason: HolderAuthorizationFailure,
    },

    /// The credential profile is unsupported or not accepted by policy.
    UnsupportedCredentialProfile {
        /// Closed reason describing why the profile is unsupported.
        reason: CredentialProfileFailure,
    },

    /// Current credential-status evidence is insufficient to continue.
    StatusUnavailable {
        /// Closed reason describing the unavailable status evidence.
        reason: StatusUnavailableReason,
    },

    /// A required policy program or policy authority snapshot is unavailable.
    PolicyUnavailable {
        /// Closed reason describing the unavailable policy state.
        reason: PolicyUnavailableReason,
    },

    /// Policy lineage is incomplete, mismatched, expired, or unrecognized.
    PolicyLineageMismatch {
        /// Lineage field or relationship that failed validation.
        field: PolicyLineageField,
        /// Closed reason describing the lineage mismatch.
        reason: PolicyLineageFailure,
    },

    /// An adapter or internal caller attempted to cross the core boundary with
    /// prohibited data or an invalid minimized result.
    BoundaryContractViolation {
        /// Boundary rule that was violated.
        rule: BoundaryContractRule,
    },

    /// An audit record violates the content-minimized audit contract.
    AuditContractViolation {
        /// Audit rule that was violated.
        rule: AuditContractRule,
    },
}

impl SkynetError {
    /// Returns a stable, closed error code suitable for minimized audit output.
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::InvalidOpaqueReference { .. } => "SKY-E-OPAQUE-REFERENCE",
            Self::InvalidTemporalWindow { .. } => "SKY-E-TEMPORAL-WINDOW",
            Self::UnknownDeploymentProfile { .. } => "SKY-E-DEPLOYMENT-PROFILE",
            Self::InvalidConsentScope { .. } => "SKY-E-CONSENT-SCOPE",
            Self::InvalidHolderAuthorizationBinding { .. } => {
                "SKY-E-HOLDER-AUTHORIZATION"
            }
            Self::UnsupportedCredentialProfile { .. } => {
                "SKY-E-CREDENTIAL-PROFILE"
            }
            Self::StatusUnavailable { .. } => "SKY-E-STATUS-UNAVAILABLE",
            Self::PolicyUnavailable { .. } => "SKY-E-POLICY-UNAVAILABLE",
            Self::PolicyLineageMismatch { .. } => "SKY-E-POLICY-LINEAGE",
            Self::BoundaryContractViolation { .. } => "SKY-E-BOUNDARY-CONTRACT",
            Self::AuditContractViolation { .. } => "SKY-E-AUDIT-CONTRACT",
        }
    }
}

impl fmt::Display for SkynetError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.code())
    }
}

impl std::error::Error for SkynetError {}

/// Categories of opaque references accepted by the Skynet core.
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum OpaqueReferenceKind {
    /// Holder-controlled local identity reference.
    CitizenIdentity,
    /// Holder-controlled credential reference.
    Credential,
    /// Approved verifier reference.
    Verifier,
    /// Presentation request reference.
    PresentationRequest,
    /// Consent-scope reference.
    ConsentScope,
    /// Policy authority reference.
    PolicyAuthority,
    /// Policy rule reference.
    PolicyRule,
    /// Disclosure descriptor-set reference.
    DisclosureDescriptorSet,
    /// Transaction-scoped decision receipt reference.
    DecisionReceipt,
    /// Transaction-scoped audit event reference.
    AuditEvent,
}

/// Closed reasons for opaque-reference validation failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum OpaqueReferenceViolation {
    /// The reference was empty.
    Empty,
    /// The reference exceeded its maximum allowed length.
    TooLong,
    /// The reference used a prohibited character.
    InvalidCharacter,
    /// The reference used an invalid prefix.
    InvalidPrefix,
    /// The reference contained an unsupported format version.
    UnsupportedVersion,
}

/// Categories of temporal windows evaluated by the Skynet core.
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum TemporalWindowKind {
    /// Credential validity interval.
    CredentialValidity,
    /// Holder-authorization validity interval.
    HolderAuthorization,
    /// Consent-scope validity interval.
    ConsentScope,
    /// Policy-lineage effective interval.
    PolicyLineage,
    /// Deployment-profile effective interval.
    DeploymentProfile,
    /// Status-evidence freshness interval.
    StatusEvidence,
}

/// Closed reasons for temporal-window validation failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum TemporalWindowViolation {
    /// The start time is not earlier than the end time.
    InvalidOrdering,
    /// The evaluated time is earlier than the allowed start time.
    NotYetValid,
    /// The evaluated time is later than the allowed end time.
    Expired,
    /// The required freshness bound has elapsed.
    Stale,
    /// A required temporal value is absent.
    Missing,
}

/// Closed reasons why a deployment profile cannot be accepted.
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum DeploymentProfileFailure {
    /// The profile identifier is not recognized.
    Unrecognized,
    /// The profile is not accepted by the active policy.
    NotAllowedByPolicy,
    /// The profile's policy-lineage reference is incomplete.
    MissingPolicyLineage,
    /// The profile is outside its effective interval.
    OutsideEffectiveInterval,
    /// The profile does not match the request context.
    RequestMismatch,
}

/// Closed reasons why a consent scope is invalid.
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum ConsentScopeFailure {
    /// The consent scope is not active.
    Inactive,
    /// The consent scope has been withdrawn.
    Withdrawn,
    /// The request purpose is outside the consent scope.
    PurposeMismatch,
    /// The request verifier is outside the consent scope.
    VerifierMismatch,
    /// The scope is outside its validity interval.
    OutsideValidityInterval,
    /// The consent scope lacks a required binding.
    MissingBinding,
}

/// Bindings required for holder authorization.
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum HolderAuthorizationBinding {
    /// Binding to the presentation request identifier.
    PresentationRequest,
    /// Binding to the approved verifier reference.
    Verifier,
    /// Binding to the declared processing purpose.
    Purpose,
    /// Binding to the consent-scope identifier.
    ConsentScope,
    /// Binding to the validity start time.
    NotBefore,
    /// Binding to the validity end time.
    ExpiresAt,
    /// Binding to the policy authority.
    PolicyAuthority,
    /// Binding to the policy version.
    PolicyVersion,
    /// Binding to the evidence freshness requirement.
    Freshness,
    /// Adapter-local request-binding result.
    RequestBinding,
}

/// Closed reasons why holder authorization fails.
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum HolderAuthorizationFailure {
    /// The required binding is absent.
    Missing,
    /// The binding does not match the presentation request.
    Mismatch,
    /// The authorization is outside its validity interval.
    OutsideValidityInterval,
    /// The authorization has been withdrawn.
    Withdrawn,
    /// The authorization freshness requirement is not met.
    Stale,
    /// The adapter reports that the request binding was replayed.
    Replayed,
    /// The adapter reports that the request binding is invalid.
    Invalid,
}

/// Closed reasons why a credential profile cannot be used.
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum CredentialProfileFailure {
    /// The profile identifier is unknown.
    Unrecognized,
    /// The profile version is unsupported.
    UnsupportedVersion,
    /// The profile is not accepted by the active policy.
    NotAllowedByPolicy,
    /// The profile does not satisfy the core boundary contract.
    BoundaryIncompatible,
    /// The profile's required adapter evidence is incomplete.
    IncompleteEvidence,
}

/// Closed reasons why status evidence is unavailable.
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum StatusUnavailableReason {
    /// The status authority is unavailable.
    AuthorityUnavailable,
    /// Status evidence is older than the policy freshness bound.
    EvidenceStale,
    /// Status evidence cannot be verified by the adapter.
    VerificationFailed,
    /// The status authority produced conflicting valid evidence.
    AuthorityEquivocation,
    /// Required status evidence is absent.
    MissingEvidence,
}

/// Closed reasons why a policy cannot be evaluated.
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum PolicyUnavailableReason {
    /// The policy identifier is not recognized.
    Unrecognized,
    /// The policy version is not locally available.
    VersionUnavailable,
    /// The policy is outside its effective interval.
    OutsideEffectiveInterval,
    /// The policy-authority snapshot is unavailable.
    AuthoritySnapshotUnavailable,
    /// The policy program cannot be evaluated deterministically.
    EvaluationUnavailable,
}

/// Fields and relationships protected by policy-lineage validation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum PolicyLineageField {
    /// Policy authority reference.
    Authority,
    /// Policy version.
    Version,
    /// Policy rule reference.
    RuleReference,
    /// Policy effective start time.
    EffectiveFrom,
    /// Policy effective end time.
    EffectiveTo,
    /// Non-sensitive policy content reference.
    ContentReference,
    /// Relationship between authorization and policy lineage.
    HolderAuthorizationBinding,
    /// Relationship between deployment profile and policy lineage.
    DeploymentBinding,
}

/// Closed reasons for policy-lineage mismatch.
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum PolicyLineageFailure {
    /// The required lineage field is absent.
    Missing,
    /// The lineage field does not match the required policy.
    Mismatch,
    /// The lineage is outside its effective interval.
    OutsideEffectiveInterval,
    /// The lineage source is not recognized.
    UnrecognizedAuthority,
    /// The lineage version is not accepted.
    UnsupportedVersion,
}

/// Core-boundary rules that cannot be violated.
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum BoundaryContractRule {
    /// A raw credential payload was offered to the core.
    RawCredentialProhibited,
    /// A credential claim value was offered to the core.
    CredentialClaimValueProhibited,
    /// A direct holder identifier was offered to the core.
    HolderIdentifierProhibited,
    /// A holder public key was offered to the core.
    HolderPublicKeyProhibited,
    /// Cryptographic proof material was offered to the core.
    CryptographicProofProhibited,
    /// A protocol challenge or nonce was offered to the core.
    ProtocolChallengeProhibited,
    /// Transport route information was offered to the core.
    TransportRouteProhibited,
    /// Raw neural information was offered to the core.
    RawNeuralDataProhibited,
    /// Raw physiological information was offered to the core.
    RawPhysiologicalDataProhibited,
    /// An adapter supplied an unsupported minimized evidence type.
    UnsupportedEvidenceType,
}

/// Content-minimization rules for audit records.
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum AuditContractRule {
    /// The audit event lacks a required transaction-scoped reference.
    MissingTransactionReference,
    /// The audit event lacks a required policy-lineage reference.
    MissingPolicyLineageReference,
    /// The audit event lacks a closed outcome code.
    MissingOutcomeCode,
    /// The audit event contains a direct actor identifier.
    ActorIdentifierProhibited,
    /// The audit event contains credential claim data.
    CredentialClaimProhibited,
    /// The audit event contains a raw credential or presentation payload.
    CredentialPayloadProhibited,
    /// The audit event contains transport or routing data.
    TransportRouteProhibited,
    /// The audit event contains continuous location data.
    ContinuousLocationProhibited,
    /// The audit event contains free-text narrative content.
    FreeTextNarrativeProhibited,
}
