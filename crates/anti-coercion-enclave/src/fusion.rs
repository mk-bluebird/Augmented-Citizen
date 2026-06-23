// crates/anti-coercion-enclave/src/fusion.rs
// SPDX-License-Identifier: MIT OR Apache-2.0

#![forbid(unsafe_code)]

//! Intent fusion for anti-coercion verification.
//!
//! This module provides intent fusion using brain-identity-kernel types.

pub use brain_identity_kernel::intent::{IntentScores, IntentWeights, fuse_intent, IntentFusionResult};
