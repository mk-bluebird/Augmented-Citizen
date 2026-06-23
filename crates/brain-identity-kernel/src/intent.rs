// crates/brain-identity-kernel/src/intent.rs
// SPDX-License-Identifier: MIT OR Apache-2.0

#![forbid(unsafe_code)]

//! Intent scoring and weighting for consent verification.
//!
//! This module provides types for fusing brain-derived and biometric
//! intent signals with configurable weights from governance shards.

use crate::fixed::Fx;

/// Weights for combining brain and biometric intent scores.
#[derive(Debug, Clone, Copy)]
pub struct IntentWeights {
    /// Weight for brain-derived intent (alpha).
    pub alpha: Fx,
    /// Weight for biometric intent (beta).
    pub beta: Fx,
}

impl IntentWeights {
    /// Create weights from governance parameters.
    /// Alpha and beta should sum to approximately 1.0.
    pub fn from_governance(alpha: Fx, beta: Fx) -> Option<Self> {
        // Validate that weights are in valid range [0, 1]
        if alpha < Fx::ZERO || beta < Fx::ZERO {
            return None;
        }
        if alpha > Fx::ONE || beta > Fx::ONE {
            return None;
        }
        Some(IntentWeights { alpha, beta })
    }

    /// Default equal weighting (0.5 each).
    pub fn default() -> Self {
        IntentWeights {
            alpha: Fx::from_f32(0.5),
            beta: Fx::from_f32(0.5),
        }
    }
}

impl Default for IntentWeights {
    fn default() -> Self {
        Self::default()
    }
}

/// Scores from different modalities.
#[derive(Debug, Clone, Copy)]
pub struct IntentScores {
    /// Brain-derived intent score (e.g., from EEG patterns).
    pub brain: Fx,
    /// Biometric intent score (e.g., from HRV, temperature).
    pub bio: Fx,
}

impl IntentScores {
    pub fn new(brain: Fx, bio: Fx) -> Self {
        IntentScores { brain, bio }
    }
}

/// Result of intent fusion.
#[derive(Debug, Clone, Copy)]
pub struct IntentFusionResult {
    /// Combined intent score after weighted fusion.
    pub combined_intent: Fx,
    /// Whether the combined score exceeds the minimum threshold.
    pub above_threshold: bool,
}

/// Fuse intent scores using provided weights and a minimum threshold.
pub fn fuse_intent(
    weights: IntentWeights,
    scores: IntentScores,
    min_threshold: Fx,
) -> IntentFusionResult {
    let brain_contrib = weights.alpha.saturating_mul(scores.brain);
    let bio_contrib = weights.beta.saturating_mul(scores.bio);
    
    let combined = brain_contrib.saturating_add(bio_contrib);
    let above_threshold = combined >= min_threshold;
    
    IntentFusionResult {
        combined_intent: combined,
        above_threshold,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_weights() {
        let w = IntentWeights::default();
        assert_eq!(w.alpha.to_f32(), 0.5);
        assert_eq!(w.beta.to_f32(), 0.5);
    }

    #[test]
    fn test_fusion_above_threshold() {
        let weights = IntentWeights::default();
        let scores = IntentScores::new(Fx::from_f32(0.8), Fx::from_f32(0.7));
        let threshold = Fx::from_f32(0.5);
        
        let result = fuse_intent(weights, scores, threshold);
        assert!(result.above_threshold);
        assert!(result.combined_intent.to_f32() > 0.5);
    }

    #[test]
    fn test_fusion_below_threshold() {
        let weights = IntentWeights::default();
        let scores = IntentScores::new(Fx::from_f32(0.2), Fx::from_f32(0.3));
        let threshold = Fx::from_f32(0.5);
        
        let result = fuse_intent(weights, scores, threshold);
        assert!(!result.above_threshold);
    }
}
