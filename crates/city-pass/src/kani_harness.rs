#![cfg(kani)]
#![forbid(unsafe_code)]

use kani::any;
use crate::domain::{BindingId, CityPassBinding, CityPassCapability, CityPassDomain};
use crate::verify::{issue_offline_pass, verify_tap, CityPassError};

/// Helper: construct a capability with symbolic taps and fixed, valid times.
fn symbolic_capability(max_taps: u32) -> CityPassCapability {
    // We fix times to a known-good RFC3339 window for bounded proofs.
    CityPassCapability {
        cap_id: "cap-1".to_string(),
        domain: CityPassDomain::OfflineTransit,
        owner_did: "didalnorganic-host".to_string(),
        owner_bostrom: "bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7".to_string(),
        eco_contract_id: "ecocontract.city-pass.v1".to_string(),
        neurorights_contract_id: "neurorights.city-pass.v1".to_string(),
        city_id: "PHX".to_string(),
        route_corridor_id: "PHX-LRT-01".to_string(),
        validity_start_utc: "2026-01-01T00:00:00Z".to_string(),
        validity_end_utc: "2026-12-31T23:59:59Z".to_string(),
        max_taps,
    }
}

/// Property 1:
/// After max_taps successful verify_tap calls, further calls fail with NoRemainingTaps.
#[kani::proof]
fn verify_tap_never_succeeds_past_max_taps() {
    // Bound max_taps and extra attempts for tractable state space.
    let max_taps: u32 = any();
    kani::assume(max_taps > 0 && max_taps <= 5);

    let cap = symbolic_capability(max_taps);
    let binding_id = BindingId(1);
    let now = "2026-06-01T12:00:00Z";

    // Issue a sealed binding.
    let mut binding = issue_offline_pass(binding_id, &cap, now, None).unwrap();

    // Perform exactly max_taps successful taps.
    let mut i = 0;
    while i < max_taps {
        let res = verify_tap(binding_id, &cap, &mut binding, now);
        assert!(res.is_ok(), "tap {i} should succeed");
        i += 1;
    }

    // State invariant: remaining_taps must be zero and absorbing.
    assert_eq!(binding.remaining_taps, 0);

    // Any further attempts (bounded to 3 for proof) must fail with NoRemainingTaps.
    let mut j = 0;
    while j < 3 {
        let res = verify_tap(binding_id, &cap, &mut binding, now);
        assert!(matches!(res, Err(CityPassError::NoRemainingTaps)));
        assert_eq!(binding.remaining_taps, 0, "remaining_taps must stay at 0");
        j += 1;
    }
}

/// Property 2:
/// A sealed binding cannot be rebound to a different host.
#[kani::proof]
fn sealed_binding_cannot_be_reassigned_to_other_host() {
    let max_taps: u32 = any();
    kani::assume(max_taps > 0 && max_taps <= 5);

    let cap = symbolic_capability(max_taps);
    let host_a = BindingId(any());
    let host_b = BindingId(any());
    // Ensure two distinct hosts.
    kani::assume(host_a != host_b);

    let now = "2026-06-01T12:00:00Z";

    // Issue binding for host A, sealed by construction.
    let binding_a = issue_offline_pass(host_a, &cap, now, None).unwrap();
    assert!(binding_a.sealed);

    // Attempt to "reissue" using the existing binding as prior state, but for host B.
    let res = issue_offline_pass(host_b, &cap, now, Some(binding_a.clone()));
    assert!(
        matches!(res, Err(CityPassError::AlreadySealed)),
        "reissuing with existing sealed binding must fail"
    );
}

/// Property 3 (optional but useful):
/// Expired passes can never yield a successful tap.
#[kani::proof]
fn expired_pass_cannot_be_used() {
    let max_taps: u32 = any();
    kani::assume(max_taps > 0 && max_taps <= 5);

    let mut cap = symbolic_capability(max_taps);
    let binding_id = BindingId(42);

    // Issue the pass in a valid window first.
    let issue_time = "2026-01-02T12:00:00Z";
    let mut binding = issue_offline_pass(binding_id, &cap, issue_time, None).unwrap();

    // Now choose a time strictly after validity_end and prove all taps fail.
    let late_time = "2027-01-01T00:00:00Z";
    // Sanity: this is after validity_end in symbolic_capability.
    // We do not change cap; we only change "now".

    let mut k = 0;
    while k < 3 {
        let res = verify_tap(binding_id, &cap, &mut binding, late_time);
        assert!(matches!(res, Err(CityPassError::Expired)));
        k += 1;
    }
}
