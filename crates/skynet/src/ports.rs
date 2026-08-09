//! Narrow local ports for reviewed Skynet adapters.
//!
//! Ports define typed boundaries between the network-free Skynet core and
//! separately reviewed adapter implementations. A port implementation may be
//! backed by local storage, an offline cache, a user interface, or an external
//! service, but the core neither assumes nor performs network access.
//!
//! No port request or result may contain raw credential bytes, credential claim
//! values, raw presentations, holder identifiers, public keys, proof material,
//! nonce values, routing data, location data, device telemetry, neural data,
//! biometric data, physiological data, or free-text narratives.

use core::num::NonZeroU32;

use crate::{
    audit::AuditEvent,
    consent::ConsentPurpose,
    credential::{
        CredentialProfileIdentifier,
        CredentialProfileVersion,
    },
    error::{
        HolderAuthorizationBinding,
        HolderAuthorizationFailure,
        SkynetError,
        SkynetResult,
        TemporalWindowKind,
        TemporalWindowViolation,
    },
    privacy::{
        AllowedCoreDomainResultCategory,
        CoreDomainResult,
    },
    provenance::PolicyLineage,
    status::CredentialStatus,
    types::{
        ConsentScopeId,
        CredentialReference,
        DeploymentProfile,
        PolicyAuthorityReference,
        PolicyVersion,
        PresentationRequestId,
        UtcTimestamp,
        VerifierReference,
    },
};

/// Maximum freshness duration accepted by the local Skynet core.
///
/// The value is a policy input, not a status-resolution or network timeout.
const MAX_FRESHNESS_SECONDS: u32 = 86_400;

/// Bounded freshness requirement used in holder authorization.
///
/// This value states the maximum permitted age of adapter-observed evidence.
/// It is not a raw timestamp, network trace, challenge, or proof.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
)]
#[serde(transparent)]
pub struct FreshnessRequirement(NonZeroU32);

impl FreshnessRequirement {
    /// Creates a bounded non-zero freshness requirement in seconds.
    pub fn new(seconds: NonZeroU32) -> SkynetResult<Self> {
        if seconds.get() > MAX_FRESHNESS_SECONDS {
            return Err(SkynetError::InvalidHolderAuthorizationBinding {
                binding: HolderAuthorizationBinding::Freshness,
                reason: HolderAuthorizationFailure::Invalid,
            });
        }

        Ok(Self(seconds))
    }

    /// Returns the maximum permitted age in seconds.
    #[must_use]
    pub const fn seconds(self) -> u32 {
        self.0.get()
    }
}

/// Closed result of adapter-local request-binding verification.
///
/// Raw verifier challenges, nonce values, proof material, and session routes
/// remain entirely inside the adapter that produces this result.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
)]
pub enum RequestBinding {
    /// The adapter verified current binding to the requested interaction.
    Bound,
    /// Required request-binding evidence was absent.
    Missing,
    /// Request-binding evidence was malformed or did not validate.
    Invalid,
    /// The adapter identified reuse of already-consumed authorization evidence.
    Replayed,
}

impl RequestBinding {
    /// Returns whether the request-binding result is valid.
    #[must_use]
    pub const fn is_bound(self) -> bool {
        matches!(self, Self::Bound)
    }
}

/// Request metadata required to obtain holder authorization.
///
/// This request contains no credential, claim, proof, key, nonce, route, or
/// holder identifier. A local trusted interface may use it to obtain explicit,
/// purpose-specific, time-bounded holder authorization.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct HolderAuthorizationRequest {
    presentation_request_id: PresentationRequestId,
    verifier_reference: VerifierReference,
    purpose: ConsentPurpose,
    consent_scope_id: ConsentScopeId,
    policy_authority: PolicyAuthorityReference,
    policy_version: PolicyVersion,
    not_before: UtcTimestamp,
    expires_at: UtcTimestamp,
    freshness_requirement: FreshnessRequirement,
}

impl HolderAuthorizationRequest {
    /// Creates a context-bound holder authorization request.
    pub fn new(
        presentation_request_id: PresentationRequestId,
        verifier_reference: VerifierReference,
        purpose: ConsentPurpose,
        consent_scope_id: ConsentScopeId,
        policy_authority: PolicyAuthorityReference,
        policy_version: PolicyVersion,
        not_before: UtcTimestamp,
        expires_at: UtcTimestamp,
        freshness_requirement: FreshnessRequirement,
    ) -> SkynetResult<Self> {
        if !not_before.is_before(expires_at) {
            return Err(SkynetError::InvalidTemporalWindow {
                kind: TemporalWindowKind::HolderAuthorization,
                violation: TemporalWindowViolation::InvalidOrdering,
            });
        }

        Ok(Self {
            presentation_request_id,
            verifier_reference,
            purpose,
            consent_scope_id,
            policy_authority,
            policy_version,
            not_before,
            expires_at,
            freshness_requirement,
        })
    }

    /// Returns the presentation request identifier.
    #[must_use]
    pub fn presentation_request_id(&self) -> &PresentationRequestId {
        &self.presentation_request_id
    }

    /// Returns the verifier reference.
    #[must_use]
    pub fn verifier_reference(&self) -> &VerifierReference {
        &self.verifier_reference
    }

    /// Returns the declared processing purpose.
    #[must_use]
    pub fn purpose(&self) -> &ConsentPurpose {
        &self.purpose
    }

    /// Returns the required consent scope identifier.
    #[must_use]
    pub fn consent_scope_id(&self) -> &ConsentScopeId {
        &self.consent_scope_id
    }

    /// Returns the governing policy authority reference.
    #[must_use]
    pub fn policy_authority(&self) -> &PolicyAuthorityReference {
        &self.policy_authority
    }

    /// Returns the governing policy version.
    #[must_use]
    pub fn policy_version(&self) -> &PolicyVersion {
        &self.policy_version
    }

    /// Returns the inclusive authorization start time.
    #[must_use]
    pub const fn not_before(&self) -> UtcTimestamp {
        self.not_before
    }

    /// Returns the exclusive authorization expiry time.
    #[must_use]
    pub const fn expires_at(&self) -> UtcTimestamp {
        self.expires_at
    }

    /// Returns the maximum permitted evidence age.
    #[must_use]
    pub const fn freshness_requirement(&self) -> FreshnessRequirement {
        self.freshness_requirement
    }
}

/// Minimized, adapter-verified holder authorization evidence.
///
/// The record represents semantic authorization findings only. It contains no
/// holder identity, public key, credential, proof, nonce, challenge, route, or
/// wallet material.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct HolderAuthorization {
    presentation_request_id: PresentationRequestId,
    verifier_reference: VerifierReference,
    purpose: ConsentPurpose,
    consent_scope_id: ConsentScopeId,
    policy_authority: PolicyAuthorityReference,
    policy_version: PolicyVersion,
    not_before: UtcTimestamp,
    expires_at: UtcTimestamp,
    freshness_requirement: FreshnessRequirement,
    observed_at: UtcTimestamp,
    request_binding: RequestBinding,
}

impl HolderAuthorization {
    /// Creates minimized holder authorization evidence.
    ///
    /// `observed_at` is the local adapter observation time. It does not identify
    /// a device, user, network, route, wallet, or external authority.
    pub fn new(
        request: HolderAuthorizationRequest,
        observed_at: UtcTimestamp,
        request_binding: RequestBinding,
    ) -> SkynetResult<Self> {
        if observed_at.is_before(request.not_before)
            || !observed_at.is_before(request.expires_at)
        {
            return Err(SkynetError::InvalidHolderAuthorizationBinding {
                binding: HolderAuthorizationBinding::ExpiresAt,
                reason: HolderAuthorizationFailure::OutsideValidityInterval,
            });
        }

        Ok(Self {
            presentation_request_id: request.presentation_request_id,
            verifier_reference: request.verifier_reference,
            purpose: request.purpose,
            consent_scope_id: request.consent_scope_id,
            policy_authority: request.policy_authority,
            policy_version: request.policy_version,
            not_before: request.not_before,
            expires_at: request.expires_at,
            freshness_requirement: request.freshness_requirement,
            observed_at,
            request_binding,
        })
    }

    /// Returns the presentation request identifier.
    #[must_use]
    pub fn presentation_request_id(&self) -> &PresentationRequestId {
        &self.presentation_request_id
    }

    /// Returns the verifier reference.
    #[must_use]
    pub fn verifier_reference(&self) -> &VerifierReference {
        &self.verifier_reference
    }

    /// Returns the declared processing purpose.
    #[must_use]
    pub fn purpose(&self) -> &ConsentPurpose {
        &self.purpose
    }

    /// Returns the consent scope identifier.
    #[must_use]
    pub fn consent_scope_id(&self) -> &ConsentScopeId {
        &self.consent_scope_id
    }

    /// Returns the policy authority reference.
    #[must_use]
    pub fn policy_authority(&self) -> &PolicyAuthorityReference {
        &self.policy_authority
    }

    /// Returns the policy version.
    #[must_use]
    pub fn policy_version(&self) -> &PolicyVersion {
        &self.policy_version
    }

    /// Returns the inclusive start time.
    #[must_use]
    pub const fn not_before(&self) -> UtcTimestamp {
        self.not_before
    }

    /// Returns the exclusive expiry time.
    #[must_use]
    pub const fn expires_at(&self) -> UtcTimestamp {
        self.expires_at
    }

    /// Returns the bounded freshness requirement.
    #[must_use]
    pub const fn freshness_requirement(&self) -> FreshnessRequirement {
        self.freshness_requirement
    }

    /// Returns the adapter-local observation time.
    #[must_use]
    pub const fn observed_at(&self) -> UtcTimestamp {
        self.observed_at
    }

    /// Returns the closed request-binding result.
    #[must_use]
    pub const fn request_binding(&self) -> RequestBinding {
        self.request_binding
    }

    /// Validates authorization freshness and context at policy-evaluation time.
    pub fn validate_for(
        &self,
        request: &HolderAuthorizationRequest,
        evaluation_time: UtcTimestamp,
    ) -> SkynetResult<()> {
        if self.presentation_request_id != request.presentation_request_id {
            return Err(SkynetError::InvalidHolderAuthorizationBinding {
                binding: HolderAuthorizationBinding::PresentationRequest,
                reason: HolderAuthorizationFailure::Mismatch,
            });
        }

        if self.verifier_reference != request.verifier_reference {
            return Err(SkynetError::InvalidHolderAuthorizationBinding {
                binding: HolderAuthorizationBinding::Verifier,
                reason: HolderAuthorizationFailure::Mismatch,
            });
        }

        if self.purpose != request.purpose {
            return Err(SkynetError::InvalidHolderAuthorizationBinding {
                binding: HolderAuthorizationBinding::Purpose,
                reason: HolderAuthorizationFailure::Mismatch,
            });
        }

        if self.consent_scope_id != request.consent_scope_id {
            return Err(SkynetError::InvalidHolderAuthorizationBinding {
                binding: HolderAuthorizationBinding::ConsentScope,
                reason: HolderAuthorizationFailure::Mismatch,
            });
        }

        if self.policy_authority != request.policy_authority {
            return Err(SkynetError::InvalidHolderAuthorizationBinding {
                binding: HolderAuthorizationBinding::PolicyAuthority,
                reason: HolderAuthorizationFailure::Mismatch,
            });
        }

        if self.policy_version != request.policy_version {
            return Err(SkynetError::InvalidHolderAuthorizationBinding {
                binding: HolderAuthorizationBinding::PolicyVersion,
                reason: HolderAuthorizationFailure::Mismatch,
            });
        }

        if !self.request_binding.is_bound() {
            let reason = match self.request_binding {
                RequestBinding::Bound => HolderAuthorizationFailure::Invalid,
                RequestBinding::Missing => HolderAuthorizationFailure::Missing,
                RequestBinding::Invalid => HolderAuthorizationFailure::Invalid,
                RequestBinding::Replayed => HolderAuthorizationFailure::Replayed,
            };

            return Err(SkynetError::InvalidHolderAuthorizationBinding {
                binding: HolderAuthorizationBinding::RequestBinding,
                reason,
            });
        }

        if evaluation_time.is_before(self.not_before)
            || !evaluation_time.is_before(self.expires_at)
        {
            return Err(SkynetError::InvalidHolderAuthorizationBinding {
                binding: HolderAuthorizationBinding::ExpiresAt,
                reason: HolderAuthorizationFailure::OutsideValidityInterval,
            });
        }

        let age = self
            .observed_at
            .elapsed_until(evaluation_time)
            .ok_or(SkynetError::InvalidHolderAuthorizationBinding {
                binding: HolderAuthorizationBinding::Freshness,
                reason: HolderAuthorizationFailure::Invalid,
            })?;

        if age > u64::from(self.freshness_requirement.seconds()) {
            return Err(SkynetError::InvalidHolderAuthorizationBinding {
                binding: HolderAuthorizationBinding::Freshness,
                reason: HolderAuthorizationFailure::Stale,
            });
        }

        Ok(())
    }
}

impl CoreDomainResult for HolderAuthorization {
    fn core_domain_category(&self) -> AllowedCoreDomainResultCategory {
        AllowedCoreDomainResultCategory::HolderAuthorization
    }
}

/// Minimized context submitted to a status-evidence adapter.
///
/// `credential_reference` is an opaque adapter-local locator. It is not a raw
/// credential, claim value, public key, proof, presentation, or status witness.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct StatusEvidenceRequest {
    credential_reference: CredentialReference,
    profile_identifier: CredentialProfileIdentifier,
    profile_version: CredentialProfileVersion,
    evaluation_time: UtcTimestamp,
}

impl StatusEvidenceRequest {
    /// Creates a minimized status-evidence request.
    #[must_use]
    pub fn new(
        credential_reference: CredentialReference,
        profile_identifier: CredentialProfileIdentifier,
        profile_version: CredentialProfileVersion,
        evaluation_time: UtcTimestamp,
    ) -> Self {
        Self {
            credential_reference,
            profile_identifier,
            profile_version,
            evaluation_time,
        }
    }

    /// Returns the opaque credential reference.
    #[must_use]
    pub fn credential_reference(&self) -> &CredentialReference {
        &self.credential_reference
    }

    /// Returns the reviewed profile identifier.
    #[must_use]
    pub fn profile_identifier(&self) -> &CredentialProfileIdentifier {
        &self.profile_identifier
    }

    /// Returns the reviewed profile version.
    #[must_use]
    pub fn profile_version(&self) -> &CredentialProfileVersion {
        &self.profile_version
    }

    /// Returns the policy-evaluation time.
    #[must_use]
    pub const fn evaluation_time(&self) -> UtcTimestamp {
        self.evaluation_time
    }
}

/// Minimized policy-authority snapshot request.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct PolicyAuthoritySnapshotRequest {
    authority: PolicyAuthorityReference,
    version: PolicyVersion,
    evaluation_time: UtcTimestamp,
}

impl PolicyAuthoritySnapshotRequest {
    /// Creates a policy-authority snapshot request.
    #[must_use]
    pub fn new(
        authority: PolicyAuthorityReference,
        version: PolicyVersion,
        evaluation_time: UtcTimestamp,
    ) -> Self {
        Self {
            authority,
            version,
            evaluation_time,
        }
    }

    /// Returns the requested authority.
    #[must_use]
    pub fn authority(&self) -> &PolicyAuthorityReference {
        &self.authority
    }

    /// Returns the requested policy version.
    #[must_use]
    pub fn version(&self) -> &PolicyVersion {
        &self.version
    }

    /// Returns the local evaluation time.
    #[must_use]
    pub const fn evaluation_time(&self) -> UtcTimestamp {
        self.evaluation_time
    }
}

/// Closed verifier-registration result.
///
/// The core does not receive verifier metadata, endpoint information, network
/// routes, certificates, or service-discovery material.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
)]
pub enum VerifierRegistrationStatus {
    /// The verifier reference is approved by the supplied policy context.
    Approved,
    /// The verifier reference is known but not approved in the supplied context.
    NotApproved,
    /// The registry could not provide current evidence.
    Unavailable,
    /// The verifier reference or registry format is unsupported.
    Unrecognized,
}

/// Request for a minimized verifier-registration decision.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct VerifierRegistryRequest {
    verifier_reference: VerifierReference,
    deployment_profile: DeploymentProfile,
    policy_authority: PolicyAuthorityReference,
    policy_version: PolicyVersion,
    evaluation_time: UtcTimestamp,
}

impl VerifierRegistryRequest {
    /// Creates a verifier-registry request.
    #[must_use]
    pub fn new(
        verifier_reference: VerifierReference,
        deployment_profile: DeploymentProfile,
        policy_authority: PolicyAuthorityReference,
        policy_version: PolicyVersion,
        evaluation_time: UtcTimestamp,
    ) -> Self {
        Self {
            verifier_reference,
            deployment_profile,
            policy_authority,
            policy_version,
            evaluation_time,
        }
    }

    /// Returns the verifier reference.
    #[must_use]
    pub fn verifier_reference(&self) -> &VerifierReference {
        &self.verifier_reference
    }

    /// Returns the deployment-profile label.
    #[must_use]
    pub fn deployment_profile(&self) -> &DeploymentProfile {
        &self.deployment_profile
    }

    /// Returns the policy authority reference.
    #[must_use]
    pub fn policy_authority(&self) -> &PolicyAuthorityReference {
        &self.policy_authority
    }

    /// Returns the policy version.
    #[must_use]
    pub fn policy_version(&self) -> &PolicyVersion {
        &self.policy_version
    }

    /// Returns the policy-evaluation time.
    #[must_use]
    pub const fn evaluation_time(&self) -> UtcTimestamp {
        self.evaluation_time
    }
}

/// Local port for normalized credential-status evidence.
///
/// The trait is synchronous and transport-neutral. Implementations must
/// normalize all mechanism-specific outcomes to `CredentialStatus`.
pub trait StatusEvidencePort {
    /// Produces normalized status evidence for a single opaque credential
    /// reference and reviewed profile context.
    fn resolve_status(
        &self,
        request: &StatusEvidenceRequest,
    ) -> SkynetResult<CredentialStatus>;
}

/// Local port for explicit holder authorization.
///
/// Implementations may use a trusted local interface, but they must emit only
/// minimized `HolderAuthorization` evidence.
pub trait HolderAuthorizationPort {
    /// Obtains and validates holder authorization for one context-bound request.
    fn authorize(
        &self,
        request: &HolderAuthorizationRequest,
        evaluation_time: UtcTimestamp,
    ) -> SkynetResult<HolderAuthorization>;
}

/// Local port for a reviewed policy-lineage snapshot.
///
/// The returned record identifies the policy context without exposing policy
/// source text or requiring the core to retrieve or execute external policy.
pub trait PolicyAuthoritySnapshotPort {
    /// Returns the locally available policy lineage for a requested authority
    /// and version.
    fn resolve_policy_lineage(
        &self,
        request: &PolicyAuthoritySnapshotRequest,
    ) -> SkynetResult<PolicyLineage>;
}

/// Local port for writing a content-minimized audit event.
///
/// An implementation decides its own persistence mechanism outside the core,
/// subject to the `AuditEvent` fixed-shape contract.
pub trait AuditSink {
    /// Records one content-minimized audit event.
    fn record(&self, event: &AuditEvent) -> SkynetResult<()>;
}

/// Local port for verifier-registry evidence.
///
/// Implementations return a closed registration outcome and must not expose
/// verifier addresses, network routes, credentials, certificates, or metadata
/// to the core.
pub trait VerifierRegistryPort {
    /// Resolves the verifier-registration outcome for one minimized request.
    fn resolve_verifier(
        &self,
        request: &VerifierRegistryRequest,
    ) -> SkynetResult<VerifierRegistrationStatus>;
}
