// SPDX-License-Identifier: MIT OR Apache-2.0
// Edition: 2024

#![forbid(unsafe_code)]
#![allow(clippy::needless_return)]

use std::env;
use std::time::{Duration, SystemTime};

use serde::{Deserialize, Serialize};

/// -------------------------------------------------------------------------
/// 1. Minimal host‑local biometric/brain data model (never leaves enclave)
/// -------------------------------------------------------------------------

/// Raw neurosensing streams that MUST remain enclave‑local and non‑exportable.
/// No instance of this type may cross an FFI boundary or be serialized.
#[derive(Debug)]
pub struct EnclaveBrainSnapshot {
    pub eeg_theta_rel: f32,      // Relative theta power [0,1] (band‑limited) [file:23]
    pub eeg_alpha_rel: f32,      // Relative alpha power [0,1] [file:23]
    pub eeg_beta_rel: f32,       // Relative beta power [0,1] [file:23]
    pub hrv_rmssd_ms: f32,       // Heart‑rate variability, RMSSD in ms [file:23]
    pub core_temp_c: f32,        // Core body temperature [file:23]
    pub cortex_delta_c: f32,     // Cortex ΔT from baseline [file:23]
    pub duty_fraction: f32,      // Current BCI duty fraction for the session [file:23]
    pub session_index: u32,      // Index of the session in the current window [file:23]
    pub pain_vas_0_10: f32,      // Subjective pain score [file:23]
    pub fatigue_0_10: f32,       // Subjective fatigue score [file:23]
}

/// Public, exportable “brain state only” envelope.
/// This is the minimal scalar summary that can leave the enclave.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainStateCommit {
    /// Normalized viability kernel distance: 0 = center, 1 = on boundary, >1 = invalid. [file:22]
    pub kernel_margin: f32,
    /// Anticoercion score I ∈ [0,1]; higher means stronger authentic intent. [file:22][file:23]
    pub anticoercion_score: f32,
    /// Psych‑debt scalar D ∈ [0,1]; higher means more cumulative load. [file:22]
    pub psych_debt: f32,
    /// Risk‑of‑harm accumulator for this decision (ΔRoH). [file:22]
    pub roh_delta: f32,
    /// Overall RoH after applying this decision, bounded by 0.30. [file:22]
    pub roh_after: f32,
    /// Discrete state label – never raw signals.
    pub state_label: BrainStateLabel,
}

/// High‑level state classification used by corridors and policies. [file:22]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum BrainStateLabel {
    ValidStable,
    ValidStressed,
    CoercionSuspect,
    Invalid,
}

/// Viability kernel constraints that NEVER relax (OTA‑proof). [file:22][file:28]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeurorightsKernel {
    /// A and b define A x ≤ b over a fixed telemetry vector x. [file:22][file:28]
    pub a_rows: Vec<[f32; 7]>,
    pub b_rows: Vec<f32>,
    /// Global RoH ceiling; must always satisfy roh_max <= 0.30. [file:22]
    pub roh_max: f32,
}

/// -------------------------------------------------------------------------
/// 2. BrainIdentityToken + typestate + Kani‑verifiable unsealing rules
/// -------------------------------------------------------------------------

/// Internal sealed binding handle – not serializable, enclave‑local.
#[derive(Debug, Clone)]
pub struct SealedCityPassBinding {
    /// Stable binding id (e.g., DB primary key). [file:22]
    pub binding_id: i64,
    /// Host DID and Bostrom address anchor. [file:22][file:23]
    pub host_did: String,
    pub bostrom_address: String,
    /// Deterministic binding hex (not a secret, but integrity‑critical). [file:22]
    pub binding_hex: String,
    /// Whether this binding has been sealed by the neurorights kernel. [file:22]
    pub sealed: bool,
}

/// Exportable, sealed view used across crates; data only, invariant proven by crate API. [file:22]
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

/// BrainIdentityToken states. Suspended and ActiveStable are externally visible; SealedOnly is internal.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum BrainIdentityLifecycle {
    ActiveStable,
    Suspended,
}

/// Exportable BrainIdentityToken – signed by enclave in the real system. [file:22][file:23]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainIdentityToken {
    pub host_did: String,
    pub bostrom_address: String,
    pub kernel_version: String,
    pub neurostate_commit: BrainStateCommit,
    pub lifecycle: BrainIdentityLifecycle,
    pub issued_at_utc: String,
    pub valid_until_utc: String,
    /// Device‑enclave attestation reference (e.g., MRENCLAVE hash). [file:22]
    pub enclave_measurement: String,
    /// PQC signature bytes over all above fields (opaque here). [file:22]
    pub signature_pq: Vec<u8>,
}

/// Type‑state wrapper that marks a binding as sealed, but never exposes unsealed internals. [file:22]
#[derive(Debug, Clone)]
pub struct SealedBindingInvariant {
    inner: CityPassBindingInvariant,
}

/// Public crate API for generating invariants from a canonical neurorights envelope.
/// No function returns SealedCityPassBinding; only CityPassBindingInvariant and SealedBindingInvariant. [file:22]
pub mod citypass_invariants {
    use super::*;

    /// Canonical neurorights envelope that drives sealing. [file:22][file:28]
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

    /// Derive shard invariants for a sealed CityPassBinding from a canonical neurorights envelope. [file:22]
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

    /// Export immutable, sealed invariants – no API exists to toggle `sealed` back to false. [file:22]
    pub fn export_invariants(sealed: &SealedBindingInvariant) -> CityPassBindingInvariant {
        sealed.inner.clone()
    }
}

/// -------------------------------------------------------------------------
/// 3. Lifecycle transitions: Suspended -> ActiveStable under neurorights floors
/// -------------------------------------------------------------------------

/// Input needed by the kernel to consider re‑activation after suspension. [file:22][file:23]
#[derive(Debug, Clone)]
pub struct ReactivationContext {
    pub last_known_roh: f32,
    pub psych_debt: f32,
    pub anticoercion_score: f32,
    pub manual_neuro_rights_review: bool,
}

/// Result of a state transition attempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifecycleTransitionResult {
    Rejected,
    Reactivated,
}

/// Core rules for state transitions that must NEVER weaken neurorights floor. [file:22][file:23][file:28]
impl BrainIdentityToken {
    /// Attempt to transition from Suspended to ActiveStable, preserving neurorights floor.
    pub fn try_reactivate(
        &self,
        kernel: &NeurorightsKernel,
        ctx: &ReactivationContext,
    ) -> (BrainIdentityToken, LifecycleTransitionResult) {
        // 1. Only Suspended tokens can be reactivated.
        if self.lifecycle != BrainIdentityLifecycle::Suspended {
            return (self.clone(), LifecycleTransitionResult::Rejected);
        }

        // 2. RoH must remain below kernel roh_max and 0.30. [file:22]
        if ctx.last_known_roh > kernel.roh_max || ctx.last_known_roh > 0.30 {
            return (self.clone(), LifecycleTransitionResult::Rejected);
        }

        // 3. Psych‑debt D must be under a safe threshold (e.g., 0.7). [file:22]
        if ctx.psych_debt > 0.7 {
            return (self.clone(), LifecycleTransitionResult::Rejected);
        }

        // 4. Anticoercion score I must be ≥ Imin (e.g., 0.8). [file:22]
        if ctx.anticoercion_score < 0.8 {
            return (self.clone(), LifecycleTransitionResult::Rejected);
        }

        // 5. Manual neurorights review flag must be true (host‑side consent review). [file:23]
        if !ctx.manual_neuro_rights_review {
            return (self.clone(), LifecycleTransitionResult::Rejected);
        }

        // All floors satisfied; construct a new ActiveStable token with updated commit.
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
/// 4. State‑only channel trait forbidding raw EEG/sEMG/fNIRS time‑series
/// -------------------------------------------------------------------------

/// State‑only channel trait: only scalar summaries, no raw time‑series exposure. [file:23]
pub trait StateOnlyChannel {
    /// Input: enclave‑local snapshot; implementors must NOT store or expose it. [file:23]
    fn update_from_snapshot(&mut self, snapshot: &EnclaveBrainSnapshot);

    /// Output: exportable scalar state only. [file:22]
    fn export_state(&self) -> BrainStateCommit;

    /// Numerical precision guarantees for exportable scalars. [file:22]
    fn precision_bounds(&self) -> StatePrecisionBounds {
        StatePrecisionBounds {
            kernel_margin_eps: 1e-3,
            anticoercion_eps: 1e-3,
            psych_debt_eps: 1e-3,
            roh_eps: 1e-4,
        }
    }
}

/// Precision bounds that are considered neurorights‑safe for export. [file:22]
#[derive(Debug, Clone, Copy)]
pub struct StatePrecisionBounds {
    pub kernel_margin_eps: f32,
    pub anticoercion_eps: f32,
    pub psych_debt_eps: f32,
    pub roh_eps: f32,
}

/// Example implementation of a state‑only channel.
/// Internally keeps only aggregated scalars; no raw buffer storage. [file:22][file:23]
#[derive(Debug, Default)]
pub struct KernelStateChannel {
    last_commit: Option<BrainStateCommit>,
}

impl KernelStateChannel {
    pub fn new() -> Self {
        Self { last_commit: None }
    }

    fn compute_kernel_margin(&self, s: &EnclaveBrainSnapshot, kernel: &NeurorightsKernel) -> f32 {
        // Example 7‑dim vector: [theta, duty, cortexΔT, pain, fatigue, HRV, 1]. [file:23][file:28]
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
        // Example anticoercion score using multimodal fusion. [file:22][file:23]
        let i_brain = 1.0_f32 - snapshot.pain_vas_0_10 / 10.0;
        let i_biometric = (snapshot.hrv_rmssd_ms / 50.0).clamp(0.0, 1.0);
        let I = 0.7 * i_brain + 0.3 * i_biometric;

        // Psych‑debt example (simple moving towards fatigue). [file:22]
        let D = (snapshot.fatigue_0_10 / 10.0).clamp(0.0, 1.0);

        // Dummy kernel – in production, injected from neurorights envelope. [file:22][file:28]
        let dummy_kernel = NeurorightsKernel {
            a_rows: vec![[0.2, 0.3, 0.2, 0.1, 0.1, 0.1, 0.0]],
            b_rows: vec![1.0],
            roh_max: 0.30,
        };
        let k_margin = self.compute_kernel_margin(snapshot, &dummy_kernel);

        let label = self.classify_label(k_margin, I, D);
        let roh_delta = 0.01_f32; // bounded per‑operation contribution. [file:22]
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
/// 5. BLE‑centric BrainChannelGuard with duty‑cycle enforcement
/// -------------------------------------------------------------------------

/// Organ systems governed by the guard. [file:23]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum OrganSystem {
    Respiratory,
    Cardiac,
    Neurovascular,
}

/// Local, clockless telemetry sample with per‑organ instantaneous load. [file:23]
#[derive(Debug, Clone, Copy)]
pub struct OrganDutySample {
    pub respiratory_load: f32,
    pub cardiac_load: f32,
    pub neurovascular_load: f32,
}

/// BLE‑centric guard that limits duty cycle using only local counters and sample‑based estimates. [file:23]
#[derive(Debug, Clone)]
pub struct BrainChannelGuard {
    /// Sliding window sample count per organ.
    samples_seen: u64,
    sum_resp: f64,
    sum_card: f64,
    sum_neuro: f64,
    /// Max allowed average duty per organ in the implicit window.
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

    /// Update duty accumulators from a new sample.
    pub fn ingest(&mut self, sample: OrganDutySample) {
        self.samples_seen = self.samples_seen.saturating_add(1);
        self.sum_resp += sample.respiratory_load as f64;
        self.sum_card += sample.cardiac_load as f64;
        self.sum_neuro += sample.neurovascular_load as f64;
    }

    /// Compute current average duty for an organ without external time reference. [file:23]
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

    /// Decide whether another BLE control packet affecting an organ is allowed.
    /// This uses only local sample statistics – no wall clock. [file:23]
    pub fn allow_next(&self, organ: OrganSystem) -> bool {
        let avg = self.average(organ);
        match organ {
            OrganSystem::Respiratory => avg <= self.max_avg_resp,
            OrganSystem::Cardiac => avg <= self.max_avg_card,
            OrganSystem::Neurovascular => avg <= self.max_avg_neuro,
        }
    }
}

/// -------------------------------------------------------------------------
/// 6. Minimal CLI skeleton wiring pieces together (non‑enclave demo)
/// -------------------------------------------------------------------------

fn now_utc_rfc3339() -> String {
    // In production, use chrono; here we just create a placeholder string
    // because real time formatting is out of scope and enclave‑specific. [file:22]
    "2026-06-23T00:00:00Z".to_string()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        eprintln!("brain-identity-core: state-only demo, no subcommand specified");
        return;
    }

    match args[1].as_str() {
        "demo-state-channel" => {
            // Demonstrate state-only channel updating and exporting.
            let snapshot = EnclaveBrainSnapshot {
                eeg_theta_rel: 0.24,
                eeg_alpha_rel: 0.31,
                eeg_beta_rel: 0.19,
                hrv_rmssd_ms: 42.0,
                core_temp_c: 36.9,
                cortex_delta_c: 0.18,
                duty_fraction: 0.33,
                session_index: 2,
                pain_vas_0_10: 1.0,
                fatigue_0_10: 3.0,
            };
            let mut chan = KernelStateChannel::new();
            chan.update_from_snapshot(&snapshot);
            let st = chan.export_state();
            println!(
                "kernel_margin={:.3}, I={:.3}, D={:.3}, roh_after={:.3}, label={:?}",
                st.kernel_margin, st.anticoercion_score, st.psych_debt, st.roh_after, st.state_label
            );
        }
        "demo-guard" => {
            // Demonstrate BLE duty guard.
            let mut guard = BrainChannelGuard::new(0.4, 0.4, 0.4);
            let samples = [
                OrganDutySample {
                    respiratory_load: 0.2,
                    cardiac_load: 0.3,
                    neurovascular_load: 0.25,
                },
                OrganDutySample {
                    respiratory_load: 0.3,
                    cardiac_load: 0.35,
                    neurovascular_load: 0.3,
                },
            ];
            for s in samples.iter() {
                guard.ingest(*s);
            }
            println!(
                "avg_resp={:.3}, allow_resp={}",
                guard.average(OrganSystem::Respiratory),
                guard.allow_next(OrganSystem::Respiratory)
            );
        }
        "demo-reactivate" => {
            // Demonstrate Suspended -> ActiveStable transition.
            let kernel = NeurorightsKernel {
                a_rows: vec![[0.2, 0.3, 0.2, 0.1, 0.1, 0.1, 0.0]],
                b_rows: vec![1.0],
                roh_max: 0.30,
            };
            let token = BrainIdentityToken {
                host_did: "did:aln-organic-host".to_string(),
                bostrom_address: "bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7".to_string(),
                kernel_version: "kernel-phx-2026.1".to_string(),
                neurostate_commit: BrainStateCommit {
                    kernel_margin: 0.8,
                    anticoercion_score: 0.6,
                    psych_debt: 0.5,
                    roh_delta: 0.01,
                    roh_after: 0.10,
                    state_label: BrainStateLabel::ValidStressed,
                },
                lifecycle: BrainIdentityLifecycle::Suspended,
                issued_at_utc: now_utc_rfc3339(),
                valid_until_utc: "2026-06-24T00:00:00Z".to_string(),
                enclave_measurement: "ENCLAVE-MEASUREMENT-HASH".to_string(),
                signature_pq: Vec::new(),
            };
            let ctx = ReactivationContext {
                last_known_roh: 0.12,
                psych_debt: 0.4,
                anticoercion_score: 0.9,
                manual_neuro_rights_review: true,
            };
            let (next, res) = token.try_reactivate(&kernel, &ctx);
            println!("reactivation={:?}, lifecycle={:?}", res, next.lifecycle);
        }
        _ => {
            eprintln!("unknown subcommand");
        }
    }
}
