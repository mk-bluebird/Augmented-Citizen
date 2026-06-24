// Mahalanobis-style identity distance D(x_t, x_{t+1}) with diagonal Σ.

use crate::fixed::Fx;
use crate::state::{IdentityState, ID_STATE_DIM};

#[derive(Copy, Clone, Debug)]
pub struct IdentitySigmaDiag {
    // Σ diagonal entries; we store their inverse as weights w_i = 1/σ_i^2
    pub inv_var: [Fx; ID_STATE_DIM],
}

impl IdentitySigmaDiag {
    pub fn from_f32_inv_var(vals: [f32; ID_STATE_DIM]) -> Self {
        let mut inv_var = [Fx::ZERO; ID_STATE_DIM];
        let mut i = 0;
        while i < ID_STATE_DIM {
            inv_var[i] = Fx::from_f32(vals[i]);
            i += 1;
        }
        IdentitySigmaDiag { inv_var }
    }

    pub fn unit() -> Self {
        let one = Fx::ONE;
        IdentitySigmaDiag { inv_var: [one; ID_STATE_DIM] }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct IdentityDistanceResult {
    pub d_sq: Fx,
}

impl IdentityDistanceResult {
    pub fn to_f32(self) -> f32 {
        // sqrt in host space; keep kernel deterministic and monotone in d_sq
        self.d_sq.to_f32().sqrt()
    }
}

/// Compute squared Mahalanobis-style distance:
/// D^2 = (x2 - x1)^T Σ^{-1} (x2 - x1), with diagonal Σ.
pub fn identity_distance_sq(
    x1: &IdentityState,
    x2: &IdentityState,
    sigma: &IdentitySigmaDiag,
) -> IdentityDistanceResult {
    let mut acc = Fx::ZERO;
    let mut i = 0;
    while i < ID_STATE_DIM {
        let dx = Fx(x2.x[i].0.wrapping_sub(x1.x[i].0));
        let w = sigma.inv_var[i];
        acc = acc + dx * dx * w;
        i += 1;
    }
    IdentityDistanceResult { d_sq: acc }
}
