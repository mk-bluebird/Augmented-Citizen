// augmented-citizen/augmented-citizen-host-affecting/src/brain_binding_guard.rs

use chrono::{DateTime, Utc};
use identity_core::{HostDid, BostromAddress};
use guard_core::{
    BrainBindingCapability,
    RuntimeBrainBindingContext,
    BrainBoundVerdict,
    PhysioState,
    evaluate_brain_bound_gate,
};

use crate::lease::{HostAffectingLease, HostAffectingLeaseState};

/// Host-affecting call descriptor for audit and invariants.
#[derive(Clone, Debug)]
pub struct HostAffectingCallDescriptor {
    pub host_did:          HostDid,
    pub bostrom_address:   BostromAddress,
    pub timestamp:         DateTime<Utc>,
    pub process_epoch:     u64,
    pub physio_before:     PhysioState,
    pub physio_after:      PhysioState,
    pub roh_delta:         f32,
    pub brain_capability:  BrainBindingCapability,
}

/// Guard that enforces:
/// - at most one HostAffecting lease holder at a time (no soul-fork),
/// - host binding to DID + Bostrom,
/// - non-reversal RoH / neurorights invariants via existing GuardKernel.
pub struct HostAffectingGuard<'a> {
    pub lease_state: &'a HostAffectingLeaseState,
    pub ctx:         &'a RuntimeBrainBindingContext,
}

impl<'a> HostAffectingGuard<'a> {
    pub fn new(
        lease_state: &'a HostAffectingLeaseState,
        ctx:         &'a RuntimeBrainBindingContext,
    ) -> Self {
        Self { lease_state, ctx }
    }

    /// Execute a host-affecting action, fully guarded.
    pub fn execute_host_affecting(
        &self,
        lease: &HostAffectingLease,
        descriptor: HostAffectingCallDescriptor,
    ) -> BrainBoundVerdict {
        if !self.lease_state.is_current(lease) {
            return BrainBoundVerdict::DenyIdentity;
        }

        if !self.ctx.is_authoritative_for(
            &descriptor.host_did,
            &descriptor.bostrom_address,
        ) {
            return BrainBoundVerdict::DenyIdentity;
        }

        evaluate_brain_bound_gate(
            self.ctx.clone(),
            descriptor.physio_before,
            descriptor.physio_after,
            descriptor.roh_delta,
        )
    }
}
