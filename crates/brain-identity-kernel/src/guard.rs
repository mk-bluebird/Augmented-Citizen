// crates/brain-identity-kernel/src/guard.rs
// SPDX-License-Identifier: MIT OR Apache-2.0

#![forbid(unsafe_code)]

//! Kernel guard for enforcing neurorights viability corridors.
//!
//! The `KernelGuard` ensures that all brain state transitions remain
//! within the viability kernel defined by A x ≤ b constraints.

use crate::fixed::Fx;
use crate::kernel::{ViabilityKernel, ViabilityKernelState, INEQUALITY_COUNT, STATE_DIM};

/// Guard that enforces viability kernel constraints.
#[derive(Debug, Clone)]
pub struct KernelGuard<'a> {
    kernel: &'a ViabilityKernel,
}

impl<'a> KernelGuard<'a> {
    pub fn new(kernel: &'a ViabilityKernel) -> Self {
        KernelGuard { kernel }
    }

    /// Check if a state is inside the viability kernel (A x ≤ b).
    pub fn is_inside_viability_kernel(&self, state: &ViabilityKernelState) -> bool {
        let x = state.as_array();
        
        for i in 0..INEQUALITY_COUNT {
            let mut lhs = Fx::ZERO;
            for j in 0..STATE_DIM {
                if let Some(prod) = self.kernel.a[i][j].checked_mul(x[j]) {
                    lhs = lhs.saturating_add(prod);
                } else {
                    return false;
                }
            }
            if lhs > self.kernel.b[i] {
                return false;
            }
        }
        true
    }

    /// Get the margin to the nearest constraint boundary.
    /// Returns None if outside the kernel.
    pub fn margin_to_boundary(&self, state: &ViabilityKernelState) -> Option<Fx> {
        if !self.is_inside_viability_kernel(state) {
            return None;
        }
        
        let x = state.as_array();
        let mut min_margin: Option<Fx> = None;
        
        for i in 0..INEQUALITY_COUNT {
            let mut lhs = Fx::ZERO;
            for j in 0..STATE_DIM {
                if let Some(prod) = self.kernel.a[i][j].checked_mul(x[j]) {
                    lhs = lhs.saturating_add(prod);
                }
            }
            
            if let Some(margin) = self.kernel.b[i].checked_sub(lhs) {
                if margin >= Fx::ZERO {
                    min_margin = Some(match min_margin {
                        Some(current) => if margin < current { margin } else { current },
                        None => margin,
                    });
                }
            }
        }
        
        min_margin
    }

    /// Get reference to the underlying kernel.
    pub fn kernel(&self) -> &ViabilityKernel {
        self.kernel
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guard_zero_state() {
        let kernel = ViabilityKernel::new(
            [[Fx::ZERO; STATE_DIM]; INEQUALITY_COUNT],
            [Fx::ONE; INEQUALITY_COUNT],
        );
        let guard = KernelGuard::new(&kernel);
        
        let state = ViabilityKernelState::new(
            Fx::ZERO, Fx::ZERO, Fx::ZERO,
            Fx::ZERO, Fx::ZERO, Fx::ZERO,
        ).unwrap();
        
        assert!(guard.is_inside_viability_kernel(&state));
    }
}
