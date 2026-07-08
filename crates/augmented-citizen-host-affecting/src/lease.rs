// augmented-citizen/augmented-citizen-host-affecting/src/lease.rs

use core::sync::atomic::{AtomicU64, Ordering};
use serde::{Deserialize, Serialize};

/// Unique identifier for a virtual-tier process in the superintelligence runtime.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProcessId(pub u64);

/// Sentinel representing "no process currently holds HostAffecting".
const NO_OWNER: u64 = u64::MAX;

/// Atomic state encoding the unique HostAffecting lease.
/// This is a lock-free state machine: acquisition and release use CAS.
#[derive(Debug)]
pub struct HostAffectingLeaseState {
    owner_id: AtomicU64,
    epoch:    AtomicU64,
}

impl HostAffectingLeaseState {
    pub const fn new() -> Self {
        Self {
            owner_id: AtomicU64::new(NO_OWNER),
            epoch:    AtomicU64::new(0),
        }
    }

    /// Attempt to acquire the HostAffecting lease.
    /// Returns Some(lease) if successful, None if another process is the current owner.
    pub fn try_acquire(&self, pid: ProcessId) -> Option<HostAffectingLease> {
        let prev = self
            .owner_id
            .compare_exchange(
                NO_OWNER,
                pid.0,
                Ordering::AcqRel,
                Ordering::Acquire,
            )
            .ok()?;

        debug_assert_eq!(prev, NO_OWNER);
        let new_epoch = self.epoch.fetch_add(1, Ordering::AcqRel) + 1;

        Some(HostAffectingLease {
            process_id: pid,
            epoch:      new_epoch,
        })
    }

    /// Release the lease if this process still owns it at the given epoch.
    /// If the epoch or owner has changed, release is a no-op.
    pub fn release(&self, lease: HostAffectingLease) {
        let current_owner = self.owner_id.load(Ordering::Acquire);
        let current_epoch = self.epoch.load(Ordering::Acquire);

        if current_owner == lease.process_id.0 && current_epoch == lease.epoch {
            let _ = self.owner_id.compare_exchange(
                lease.process_id.0,
                NO_OWNER,
                Ordering::AcqRel,
                Ordering::Acquire,
            );
            let _ = self.epoch.fetch_add(1, Ordering::AcqRel);
        }
    }

    /// Check whether the given lease is the unique, current HostAffecting holder.
    pub fn is_current(&self, lease: &HostAffectingLease) -> bool {
        let current_owner = self.owner_id.load(Ordering::Acquire);
        let current_epoch = self.epoch.load(Ordering::Acquire);
        current_owner == lease.process_id.0 && current_epoch == lease.epoch
    }

    /// Read-only view for Kani proofs and diagnostics.
    pub fn snapshot(&self) -> HostAffectingLeaseSnapshot {
        HostAffectingLeaseSnapshot {
            owner_id: self.owner_id.load(Ordering::Acquire),
            epoch:    self.epoch.load(Ordering::Acquire),
        }
    }
}

/// Non-forkable HostAffecting lease token.
/// This must be presented to cross the host boundary.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct HostAffectingLease {
    pub process_id: ProcessId,
    pub epoch:      u64,
}

/// Simple snapshot used for proofs and logging.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct HostAffectingLeaseSnapshot {
    pub owner_id: u64,
    pub epoch:    u64,
}
