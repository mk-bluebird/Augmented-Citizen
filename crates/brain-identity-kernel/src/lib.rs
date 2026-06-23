// crates/brain-identity-kernel/src/lib.rs
// SPDX-License-Identifier: MIT OR Apache-2.0

#![forbid(unsafe_code)]
#![allow(clippy::needless_return)]

//! Brain identity kernel primitives for neurorights-safe BCI operations.
//!
//! This crate provides:
//! - Fixed-point arithmetic types (`Fx`) for deterministic neural computations
//! - Viability kernel state and constraint definitions
//! - Kernel guard for enforcing neurorights corridors
//! - Intent scoring and weighting for consent verification
//! - ALN shard parsing and verification

pub mod fixed;
pub mod header;
pub mod kernel;
pub mod shard;
pub mod guard;
pub mod intent;

pub use fixed::Fx;
pub use header::KernelHeader;
pub use kernel::{ViabilityKernel, ViabilityKernelState, STATE_DIM, INEQUALITY_COUNT};
pub use shard::{ShardError, VerifiedKernelShard, RawShard};
pub use guard::KernelGuard;
pub use intent::{IntentScores, IntentWeights, IntentFusionResult};
