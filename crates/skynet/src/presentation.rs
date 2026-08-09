//! Policy-safe credential-presentation intent coordination.
//!
//! This module models presentation metadata and holder-authorization bindings.
//! It does not construct, parse, validate, seal, encrypt, sign, transport, or
//! persist credential presentations.
//!
//! No public type in this module contains raw credential data, claim values,
//! holder identity, wallet identifiers, keys, proofs, challenges, nonce values,
//! routes, endpoints, location data, device data, biometric data, neural data,
//! or free-text narrative fields.

use crate::{
    consent::ConsentPurpose,
    credential::{
        CredentialProfileIdentifier,
        CredentialProfileVersion,
    },
    error::{
        SkynetError,
        SkynetResult,
        TemporalWindowKind,
        TemporalWindowViolation,
    },
    ports::{
        FreshnessRequirement,
        HolderAuthorization,
        HolderAuthorizationRequest,
    },
    types::{
        ConsentScopeId,
        DeploymentProfile,
        DisclosureDescriptorSetId,
        PolicyAuthorityReference,
        PolicyVersion,
        PresentationRequestId,
        UtcTimestamp,
        VerifierReference,
    },
};

/// Metadata describing one policy-safe credential-presentation intent.
///
/// A presentation intent is not a credential presentation. It contains only
/// the context required to request, validate, and bind explicit holder
/// authorization before a separately reviewed adapter may construct a sealed
/// verifier-addressed presentation.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct PresentationIntent {
    presentation_request_id: PresentationRequestId,
    verifier_reference: VerifierReference,
    purpose: ConsentPurpose,
    consent_scope_id: ConsentScopeId,
    deployment_profile: DeploymentProfile,
    profile_identifier: CredentialProfileIdentifier,
    profile_version: CredentialProfileVersion,
    disclosure_descriptor_set_id: DisclosureDescriptorSetId,
    policy_authority: PolicyAuthorityReference,
    policy_version: PolicyVersion,
    not_before: UtcTimestamp,
    expires_at: UtcTimestamp,
}

impl PresentationIntent {
    /// Creates a time-bounded presentation intent.
    ///
    /// The constructor validates only the local temporal interval. Verifier
    /// registry approval, deployment acceptance, consent validation, holder
    /// authorization, status evidence, disclosure conformance, and final
    /// policy eligibility remain separate evaluation responsibilities.
    pub fn new(
        presentation_request_id: PresentationRequestId,
        verifier_reference: VerifierReference,
        purpose: ConsentPurpose,
        consent_scope_id: ConsentScopeId,
        deployment_profile: DeploymentProfile,
        profile_identifier: CredentialProfileIdentifier,
        profile_version: CredentialProfileVersion,
        disclosure_descriptor_set_id: DisclosureDescriptorSetId,
        policy_authority: PolicyAuthorityReference,
        policy_version: PolicyVersion,
        not_before: UtcTimestamp,
        expires_at: UtcTimestamp,
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
            deployment_profile,
            profile_identifier,
            profile_version,
            disclosure_descriptor_set_id,
            policy_authority,
            policy_version,
            not_before,
            expires_at,
        })
    }

    /// Returns the transaction-scoped presentation request identifier.
    #[must_use]
    pub fn presentation_request_id(&self) -> &PresentationRequestId {
        &self.presentation_request_id
    }

    /// Returns the approved verifier reference.
    #[must_use]
    pub fn verifier_reference(&self) -> &VerifierReference {
        &self.verifier_reference
    }

    /// Returns the declared processing purpose.
    #[must_use]
    pub fn purpose(&self) -> &ConsentPurpose {
        &self.purpose
    }

    /// Returns the purpose-specific consent-scope identifier.
    #[must_use]
    pub fn consent_scope_id(&self) -> &ConsentScopeId {
        &self.consent_scope_id
    }

    /// Returns the application-defined deployment profile label.
    ///
    /// This label is not real-time location, residence, municipal affiliation,
    /// or infrastructure connectivity evidence.
    #[must_use]
    pub fn deployment_profile(&self) -> &DeploymentProfile {
        &self.deployment_profile
    }

    /// Returns the reviewed credential-profile identifier.
    #[must_use]
    pub fn profile_identifier(&self) -> &CredentialProfileIdentifier {
        &self.profile_identifier
    }

    /// Returns the reviewed credential-profile version.
    #[must_use]
    pub fn profile_version(&self) -> &CredentialProfileVersion {
        &self.profile_version
    }

    /// Returns the opaque policy-approved disclosure descriptor-set identifier.
    #[must_use]
    pub fn disclosure_descriptor_set_id(&self) -> &DisclosureDescriptorSetId {
        &self.disclosure_descriptor_set_id
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

    /// Returns the inclusive presentation-intent validity start.
    #[must_use]
    pub const fn not_before(&self) -> UtcTimestamp {
        self.not_before
    }

    /// Returns the exclusive presentation-intent expiry time.
    #[must_use]
    pub const fn expires_at(&self) -> UtcTimestamp {
        self.expires_at
    }

    /// Returns whether this intent is active at the supplied evaluation time.
    #[must_use]
    pub const fn is_active_at(&self, evaluation_time: UtcTimestamp) -> bool {
        !evaluation_time.is_before(self.not_before)
            && evaluation_time.is_before(self.expires_at)
    }

    /// Builds the minimized request used to obtain holder authorization.
    ///
    /// This method does not contact a holder interface or verifier. A reviewed
    /// `HolderAuthorizationPort` consumes the returned request separately.
    pub fn holder_authorization_request(
        &self,
        freshness_requirement: FreshnessRequirement,
    ) -> SkynetResult<HolderAuthorizationRequest> {
        HolderAuthorizationRequest::new(
            self.presentation_request_id.clone(),
            self.verifier_reference.clone(),
            self.purpose.clone(),
            self.consent_scope_id.clone(),
            self.policy_authority.clone(),
            self.policy_version.clone(),
            self.not_before,
            self.expires_at,
            freshness_requirement,
        )
    }

    /// Validates that holder authorization is bound to this exact intent.
    ///
    /// The holder authorization itself is minimized evidence. This method does
    /// not inspect proofs, keys, challenges, nonces, credentials, wallets, or
    /// presentation bytes.
    pub fn validate_holder_authorization(
        &self,
        authorization: &HolderAuthorization,
        freshness_requirement: FreshnessRequirement,
        evaluation_time: UtcTimestamp,
    ) -> SkynetResult<()> {
        if !self.is_active_at(evaluation_time) {
            return Err(SkynetError::InvalidTemporalWindow {
                kind: TemporalWindowKind::HolderAuthorization,
                violation: TemporalWindowViolation::Expired,
            });
        }

        let request = self.holder_authorization_request(freshness_requirement)?;

        authorization.validate_for(&request, evaluation_time)
    }
}
