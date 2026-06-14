// filename: crates/bioscale-upgrade-store/src/continuity_guard.rs
// destination: EcoNet-CEIM-PhoenixWater/crates/bioscale-upgrade-store/src/continuity_guard.rs

use serde::{Deserialize, Serialize};

/// Represents the host's psychological and biophysical continuity state.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ContinuityStateVector {
    pub memory_recall_latency_ms: f32,
    pub affective_baseline_variance: f32,
    pub belief_kernel_stability_score: f32,
    pub neuroinflammation_proxy: f32,
}

impl ContinuityStateVector {
    /// Calculates a unified continuity score (0.0 to 1.0, where 1.0 is perfect stability).
    pub fn continuity_score(&self) -> f32 {
        // Inverse of latency and variance, direct weight on stability.
        let latency_factor = 1.0 - (self.memory_recall_latency_ms / 1000.0).clamp(0.0, 1.0);
        let variance_factor = 1.0 - self.affective_baseline_variance.clamp(0.0, 1.0);
        let inflammation_penalty = 1.0 - self.neuroinflammation_proxy.clamp(0.0, 1.0);
        
        (0.3 * latency_factor) + 
        (0.3 * variance_factor) + 
        (0.4 * self.belief_kernel_stability_score) * inflammation_penalty
    }
}

/// Defines the projected impact of an actuation channel on the host.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ActuationImpactProjection {
    pub delta_latency_ms: f32,       // Expected change in memory recall latency
    pub delta_variance: f32,         // Expected change in affective variance
    pub delta_inflammation: f32,     // Expected change in neuroinflammation proxy
}

impl ActuationImpactProjection {
    /// Applies the projection to a baseline state to estimate the post-actuation state.
    pub fn apply_to(&self, baseline: &ContinuityStateVector) -> ContinuityStateVector {
        ContinuityStateVector {
            memory_recall_latency_ms: (baseline.memory_recall_latency_ms + self.delta_latency_ms).max(0.0),
            affective_baseline_variance: (baseline.affective_baseline_variance + self.delta_variance).clamp(0.0, 1.0),
            belief_kernel_stability_score: baseline.belief_kernel_stability_score, // Actuations cannot directly alter core belief kernels
            neuroinflammation_proxy: (baseline.neuroinflammation_proxy + self.delta_inflammation).clamp(0.0, 1.0),
        }
    }
}

/// The core guardrail. Evaluates if an actuation is safe regarding psychological continuity.
pub fn evaluate_continuity_safety(
    baseline: &ContinuityStateVector,
    impact: &ActuationImpactProjection,
    min_allowed_score: f32,
) -> ContinuityDecision {
    let pre_score = baseline.continuity_score();
    let post_state = impact.apply_to(baseline);
    let post_score = post_state.continuity_score();
    
    let delta = post_score - pre_score;

    // INVARIANT 1: Non-degradation. The actuation must not reduce continuity below the minimum threshold.
    if post_score < min_allowed_score {
        return ContinuityDecision::Blocked {
            reason: "Post-actuation continuity score falls below minimum allowed threshold.",
            pre_score,
            post_score,
        };
    }

    // INVARIANT 2: Monotonicity. The actuation must not cause a net negative shift in continuity.
    // (A small epsilon is allowed for sensor noise, but structural degradation is forbidden).
    if delta < -0.01 {
        return ContinuityDecision::Blocked {
            reason: "Actuation violates monotonicity; projected continuity degradation exceeds epsilon.",
            pre_score,
            post_score,
        };
    }

    // INVARIANT 3: Inflammation ceiling. Biocompatibility hard stop.
    if post_state.neuroinflammation_proxy > 0.4 {
        return ContinuityDecision::Blocked {
            reason: "Projected neuroinflammation exceeds biocompatibility ceiling, risking dissociation.",
            pre_score,
            post_score,
        };
    }

    ContinuityDecision::Allowed {
        pre_score,
        post_score,
        delta,
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContinuityDecision {
    Allowed {
        pre_score: f32,
        post_score: f32,
        delta: f32,
    },
    Blocked {
        reason: &'static str,
        pre_score: f32,
        post_score: f32,
    },
}

// --- Kani Proof Harnesses (Conceptual) ---
// These functions are used by Kani to formally verify the invariants hold for ALL possible inputs.

#[cfg(kani)]
#[kani::proof]
fn verify_continuity_monotonicity() {
    let baseline: ContinuityStateVector = kani::any();
    let impact: ActuationImpactProjection = kani::any();
    
    // Assume valid input ranges
    kani::assume(baseline.belief_kernel_stability_score >= 0.0 && baseline.belief_kernel_stability_score <= 1.0);
    kani::assume(impact.delta_inflammation >= 0.0); // Actuations generally add biological load
    
    let decision = evaluate_continuity_safety(&baseline, &impact, 0.5);
    
    if let ContinuityDecision::Allowed { delta, .. } = decision {
        // Kani will assert this. If an actuation is allowed, delta MUST be >= -0.01.
        assert!(delta >= -0.01, "Monotonicity invariant violated: Allowed actuation caused unacceptable degradation.");
    }
}
