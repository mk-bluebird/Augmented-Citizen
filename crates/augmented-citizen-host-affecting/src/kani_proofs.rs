// augmented-citizen/augmented-citizen-host-affecting/src/kani_proofs.rs

#![cfg(kani)]

use kani::any;
use crate::lease::{HostAffectingLeaseState, HostAffectingLease, ProcessId};

/// Kani proof: at most one process can hold a current HostAffecting lease at any time.
/// We model two processes attempting to acquire and then check coexistence.
#[kani::proof]
fn proof_at_most_one_current_lease() {
    let state = HostAffectingLeaseState::new();

    let p1_id = any::<u64>();
    let p2_id = any::<u64>();
    kani::assume(p1_id != u64::MAX);
    kani::assume(p2_id != u64::MAX);

    let p1 = ProcessId(p1_id);
    let p2 = ProcessId(p2_id);

    let l1 = state.try_acquire(p1);
    let l2 = state.try_acquire(p2);

    if let (Some(lease1), Some(lease2)) = (l1, l2) {
        let current1 = state.is_current(&lease1);
        let current2 = state.is_current(&lease2);

        assert!(
            !(current1 && current2),
            "Two distinct processes cannot both hold a current HostAffecting lease"
        );

        if current1 {
            assert_eq!(lease1.process_id.0, p1_id);
        }
        if current2 {
            assert_eq!(lease2.process_id.0, p2_id);
        }
    }
}

/// Kani proof: releasing a current lease returns the system to NO_OWNER.
#[kani::proof]
fn proof_release_resets_owner() {
    let state = HostAffectingLeaseState::new();

    let pid = ProcessId(1);
    if let Some(lease) = state.try_acquire(pid) {
        assert!(state.is_current(&lease));

        state.release(lease);

        let snapshot = state.snapshot();
        assert_eq!(
            snapshot.owner_id,
            super::lease::NO_OWNER,
            "Release must reset owner_id to NO_OWNER"
        );
    }
}
