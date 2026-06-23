// crates/brain-identity-core/src/lib.rs
// SPDX-License-Identifier: MIT OR Apache-2.0

#![forbid(unsafe_code)]
#![allow(clippy::needless_return)]

//! Neurorights-safe brain identity core primitives.
//!
//! This crate provides:
//! - Minimal enclave-local brain/biometric snapshot types that must never be serialized.
//! - Exportable `BrainStateCommit` envelopes respecting neurorights corridors.
//! - `BrainIdentityToken` lifecycle rules (Suspended ↔ ActiveStable) with RoH and psych-debt floors.
//! - A `StateOnlyChannel` trait that forbids exposure of raw EEG/sEMG/fNIRS time-series.
//! - A BLE-centric `BrainChannelGuard` that enforces per-organ duty cycle using only local telemetry.
//! - A `citypass_invariants` module that derives sealed CityPass binding invariants with no unseal API.
//!
//! All designs are grounded in your existing neurorights kernel and ALN specs. [file:22][file:23][file:28]

use serde::{Deserialize, Serialize};

/// -------------------------------------------------------------------------
/// 1. Minimal host-local brain/biometric data (enclave-only)
/// -------------------------------------------------------------------------

/// Raw neurosensing snapshot that MUST remain enclave-local and non-serializable.
/// This type is intentionally not `Serialize`/`Deserialize`.
#[derive(Debug)]
pub struct EnclaveBrainSnapshot {
    pub eeg_theta_rel: f32,
    pub eeg_alpha_rel: f32,
    pub eeg_beta_rel: f32,
    pub hrv_rmssd_ms: f32,
    pub core_temp_c: f32,
    pub cortex_delta_c: f32,
    pub duty_fraction: f32,
    pub session_index: u32,
    pub pain_vas_0_10: f32,
    pub fatigue_0_10: f32,
}

/// Exportable state-only brain identity envelope. [file:22][file:23]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainStateCommit {
    pub kernel_margin: f32,
    pub anticoercion_score: f32,
    pub psych_debt: f32,
    pub roh_delta: f32,
    pub roh_after: f32,
    pub state_label: BrainStateLabel,
}

/// High-level state labels used in corridors and guards. [file:22]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum BrainStateLabel {
    ValidStable,
    ValidStressed,
    CoercionSuspect,
    Invalid,
}

/// Neurorights viability kernel A x ≤ b, with RoH ceiling. [file:22][file:28]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeurorightsKernel {
    pub a_rows: Vec<[f32; 7]>,
    pub b_rows: Vec<f32>,
    pub roh_max: f32,
}

/// -------------------------------------------------------------------------
/// 2. BrainIdentityToken and CityPass binding invariants
/// -------------------------------------------------------------------------

/// Exportable BrainIdentityToken signed by an enclave in production. [file:22][file:23]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainIdentityToken {
    pub host_did: String,
    pub bostrom_address: String,
    pub kernel_version: String,
    pub neurostate_commit: BrainStateCommit,
    pub lifecycle: BrainIdentityLifecycle,
    pub issued_at_utc: String,
    pub valid_until_utc: String,
    pub enclave_measurement: String,
    pub signature_pq: Vec<u8>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum BrainIdentityLifecycle {
    ActiveStable,
    Suspended,
}

/// Internal sealed binding handle; crate-private to block external construction. [file:22]
#[derive(Debug, Clone)]
struct SealedCityPassBinding {
    pub binding_id: i64,
    pub host_did: String,
    pub bostrom_address: String,
    pub binding_hex: String,
    pub sealed: bool,
}

/// Exportable, invariant view of a sealed binding. [file:22]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CityPassBindingInvariant {
    pub binding_id: i64,
    pub host_did: String,
    pub bostrom_address: String,
    pub binding_hex: String,
    pub sealed: bool,
    pub valid_from_utc: String,
    pub valid_until_utc: String,
}

/// Type-state wrapper with no API to unseal. [file:22]
#[derive(Debug, Clone)]
pub struct SealedBindingInvariant {
    inner: CityPassBindingInvariant,
}

/// Module that derives ALN shard invariants from a canonical neurorights envelope. [file:22]
pub mod citypass_invariants {
    use super::*;

    /// Canonical neurorights envelope used to seal bindings. [file:22][file:28]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct NeurorightsEnvelope {
        pub host_did: String,
        pub bostrom_address: String,
        pub kernel: NeurorightsKernel,
        pub corridor_floor: f32,
        pub roh_ceiling: f32,
        pub valid_from_utc: String,
        pub valid_until_utc: String,
    }

    /// Derive sealed binding invariants from the neurorights envelope and binding contexts. [file:22]
    pub fn derive_sealed_invariants(
        env: &NeurorightsEnvelope,
        binding_hex: &str,
        binding_id: i64,
    ) -> SealedBindingInvariant {
        let sealed = true;
        let inv = CityPassBindingInvariant {
            binding_id,
            host_did: env.host_did.clone(),
            bostrom_address: env.bostrom_address.clone(),
            binding_hex: binding_hex.to_owned(),
            sealed,
            valid_from_utc: env.valid_from_utc.clone(),
            valid_until_utc: env.valid_until_utc.clone(),
        };
        SealedBindingInvariant { inner: inv }
    }

    /// Export immutable invariants; no function exists to unseal or mutate `sealed`. [file:22]
    pub fn export_invariants(sealed: &SealedBindingInvariant) -> CityPassBindingInvariant {
        sealed.inner.clone()
    }
}

/// -------------------------------------------------------------------------
/// 3. Lifecycle transition rules (Suspended → ActiveStable)
/// -------------------------------------------------------------------------

/// Context needed to reactivate a suspended BrainIdentityToken. [file:22][file:23]
#[derive(Debug, Clone)]
pub struct ReactivationContext {
    pub last_known_roh: f32,
    pub psych_debt: f32,
    pub anticoercion_score: f32,
    pub manual_neuro_rights_review: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifecycleTransitionResult {
    Rejected,
    Reactivated,
}

impl BrainIdentityToken {
    /// Transition Suspended → ActiveStable without weakening neurorights floors. [file:22][file:23][file:28]
    pub fn try_reactivate(
        &self,
        kernel: &NeurorightsKernel,
        ctx: &ReactivationContext,
    ) -> (BrainIdentityToken, LifecycleTransitionResult) {
        if self.lifecycle != BrainIdentityLifecycle::Suspended {
            return (self.clone(), LifecycleTransitionResult::Rejected);
        }
        if ctx.last_known_roh > kernel.roh_max || ctx.last_known_roh > 0.30 {
            return (self.clone(), LifecycleTransitionResult::Rejected);
        }
        if ctx.psych_debt > 0.7 {
            return (self.clone(), LifecycleTransitionResult::Rejected);
        }
        if ctx.anticoercion_score < 0.8 {
            return (self.clone(), LifecycleTransitionResult::Rejected);
        }
        if !ctx.manual_neuro_rights_review {
            return (self.clone(), LifecycleTransitionResult::Rejected);
        }

        let mut next = self.clone();
        next.lifecycle = BrainIdentityLifecycle::ActiveStable;
        next.neurostate_commit.psych_debt = ctx.psych_debt;
        next.neurostate_commit.anticoercion_score = ctx.anticoercion_score;
        next.neurostate_commit.roh_after = ctx.last_known_roh;
        next.neurostate_commit.state_label = BrainStateLabel::ValidStable;

        (next, LifecycleTransitionResult::Reactivated)
    }
}

/// -------------------------------------------------------------------------
/// 4. State-only channel trait
/// -------------------------------------------------------------------------

/// Trait ensuring only scalar state leaves the enclave; no raw buffers. [file:23]
pub trait StateOnlyChannel {
    fn update_from_snapshot(&mut self, snapshot: &EnclaveBrainSnapshot);
    fn export_state(&self) -> BrainStateCommit;
    fn precision_bounds(&self) -> StatePrecisionBounds {
        StatePrecisionBounds {
            kernel_margin_eps: 1e-3,
            anticoercion_eps: 1e-3,
            psych_debt_eps: 1e-3,
            roh_eps: 1e-4,
        }
    }
}

/// Numeric precision bounds considered neurorights-safe. [file:22]
#[derive(Debug, Clone, Copy)]
pub struct StatePrecisionBounds {
    pub kernel_margin_eps: f32,
    pub anticoercion_eps: f32,
    pub psych_debt_eps: f32,
    pub roh_eps: f32,
}

/// Example state-only implementation backed by a neurorights kernel. [file:22][file:23][file:28]
#[derive(Debug, Default)]
pub struct KernelStateChannel {
    last_commit: Option<BrainStateCommit>,
}

impl KernelStateChannel {
    pub fn new() -> Self {
        Self { last_commit: None }
    }

    fn compute_kernel_margin(&self, s: &EnclaveBrainSnapshot, kernel: &NeurorightsKernel) -> f32 {
        let x = [
            s.eeg_theta_rel,
            s.duty_fraction,
            s.cortex_delta_c,
            s.pain_vas_0_10 / 10.0,
            s.fatigue_0_10 / 10.0,
            s.hrv_rmssd_ms / 100.0,
            1.0,
        ];
        let mut worst_ratio = 0.0_f32;
        for (row, &b) in kernel.a_rows.iter().zip(kernel.b_rows.iter()) {
            let dot = row[0] * x[0]
                + row[1] * x[1]
                + row[2] * x[2]
                + row[3] * x[3]
                + row[4] * x[4]
                + row[5] * x[5]
                + row[6] * x[6];
            let ratio = if b.abs() < f32::EPSILON {
                0.0
            } else {
                dot / b
            };
            if ratio > worst_ratio {
                worst_ratio = ratio;
            }
        }
        worst_ratio
    }

    fn classify_label(&self, k_margin: f32, I: f32, D: f32) -> BrainStateLabel {
        if k_margin > 1.0 {
            return BrainStateLabel::Invalid;
        }
        if I < 0.5 {
            return BrainStateLabel::CoercionSuspect;
        }
        if D > 0.7 {
            return BrainStateLabel::ValidStressed;
        }
        BrainStateLabel::ValidStable
    }
}

impl StateOnlyChannel for KernelStateChannel {
    fn update_from_snapshot(&mut self, snapshot: &EnclaveBrainSnapshot) {
        let i_brain = 1.0_f32 - snapshot.pain_vas_0_10 / 10.0;
        let i_biometric = (snapshot.hrv_rmssd_ms / 50.0).clamp(0.0, 1.0);
        let I = 0.7 * i_brain + 0.3 * i_biometric;

        let D = (snapshot.fatigue_0_10 / 10.0).clamp(0.0, 1.0);

        let dummy_kernel = NeurorightsKernel {
            a_rows: vec![[0.2, 0.3, 0.2, 0.1, 0.1, 0.1, 0.0]],
            b_rows: vec![1.0],
            roh_max: 0.30,
        };
        let k_margin = self.compute_kernel_margin(snapshot, &dummy_kernel);

        let label = self.classify_label(k_margin, I, D);
        let roh_delta = 0.01_f32;
        let roh_after = self
            .last_commit
            .map(|c| c.roh_after + roh_delta)
            .unwrap_or(roh_delta)
            .min(dummy_kernel.roh_max);

        let commit = BrainStateCommit {
            kernel_margin: k_margin,
            anticoercion_score: I,
            psych_debt: D,
            roh_delta,
            roh_after,
            state_label: label,
        };
        self.last_commit = Some(commit);
    }

    fn export_state(&self) -> BrainStateCommit {
        self.last_commit
            .clone()
            .unwrap_or(BrainStateCommit {
                kernel_margin: 0.0,
                anticoercion_score: 0.0,
                psych_debt: 0.0,
                roh_delta: 0.0,
                roh_after: 0.0,
                state_label: BrainStateLabel::Invalid,
            })
    }
}

/// -------------------------------------------------------------------------
/// 5. BLE-centric BrainChannelGuard (local duty-cycle enforcement)
/// -------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum OrganSystem {
    Respiratory,
    Cardiac,
    Neurovascular,
}

#[derive(Debug, Clone, Copy)]
pub struct OrganDutySample {
    pub respiratory_load: f32,
    pub cardiac_load: f32,
    pub neurovascular_load: f32,
}

#[derive(Debug, Clone)]
pub struct BrainChannelGuard {
    samples_seen: u64,
    sum_resp: f64,
    sum_card: f64,
    sum_neuro: f64,
    max_avg_resp: f32,
    max_avg_card: f32,
    max_avg_neuro: f32,
}

impl BrainChannelGuard {
    pub fn new(max_avg_resp: f32, max_avg_card: f32, max_avg_neuro: f32) -> Self {
        Self {
            samples_seen: 0,
            sum_resp: 0.0,
            sum_card: 0.0,
            sum_neuro: 0.0,
            max_avg_resp,
            max_avg_card,
            max_avg_neuro,
        }
    }

    pub fn ingest(&mut self, sample: OrganDutySample) {
        self.samples_seen = self.samples_seen.saturating_add(1);
        self.sum_resp += sample.respiratory_load as f64;
        self.sum_card += sample.cardiac_load as f64;
        self.sum_neuro += sample.neurovascular_load as f64;
    }

    pub fn average(&self, organ: OrganSystem) -> f32 {
        if self.samples_seen == 0 {
            return 0.0;
        }
        let n = self.samples_seen as f64;
        match organ {
            OrganSystem::Respiratory => (self.sum_resp / n) as f32,
            OrganSystem::Cardiac => (self.sum_card / n) as f32,
            OrganSystem::Neurovascular => (self.sum_neuro / n) as f32,
        }
    }

    pub fn allow_next(&self, organ: OrganSystem) -> bool {
        let avg = self.average(organ);
        match organ {
            OrganSystem::Respiratory => avg <= self.max_avg_resp,
            OrganSystem::Cardiac => avg <= self.max_avg_card,
            OrganSystem::Neurovascular => avg <= self.max_avg_neuro,
        }
    }
}
