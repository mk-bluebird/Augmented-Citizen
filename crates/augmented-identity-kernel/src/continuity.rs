// Identity Lyapunov kernel V_id(x) and per-step continuity checks.

use crate::fixed::Fx;
use crate::state::{IdentityState, IdentityBaseline, ID_STATE_DIM};
use crate::distance::{IdentitySigmaDiag, IdentityDistanceResult, identity_distance_sq};

#[derive(Copy, Clone, Debug)]
pub struct IdentityLyapunovWeights {
    pub w: [Fx; ID_STATE_DIM],
}

impl IdentityLyapunovWeights {
    pub fn from_f32_slice(vals: [f32; ID_STATE_DIM]) -> Self {
        let mut w = [Fx::ZERO; ID_STATE_DIM];
        let mut i = 0;
        while i < ID_STATE_DIM {
            w[i] = Fx::from_f32(vals[i]);
            i += 1;
        }
        IdentityLyapunovWeights { w }
    }

    pub fn unit() -> Self {
        IdentityLyapunovWeights { w: [Fx::ONE; ID_STATE_DIM] }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct IdentityContinuityConfig {
    pub sigma: IdentitySigmaDiag,
    pub lyap_weights: IdentityLyapunovWeights,
    // ε_cont in Q16.16 over distance, applied to D^2 for kernel checks
    pub eps_cont_sq: Fx,
    // Allowed Lyapunov increase δ in Q16.16
    pub delta_v: Fx,
}

impl IdentityContinuityConfig {
    pub fn from_f32(eps_cont: f32, delta_v: f32, sigma: IdentitySigmaDiag, lyap: IdentityLyapunovWeights) -> Self {
        let eps_sq = Fx::from_f32(eps_cont * eps_cont);
        let dv = Fx::from_f32(delta_v);
        IdentityContinuityConfig {
            sigma,
            lyap_weights: lyap,
            eps_cont_sq: eps_sq,
            delta_v: dv,
        }
    }

    pub fn strict_defaults() -> Self {
        let sigma = IdentitySigmaDiag::unit();
        let lyap = IdentityLyapunovWeights::unit();
        IdentityContinuityConfig {
            sigma,
            lyap_weights: lyap,
            eps_cont_sq: Fx::from_f32(0.05 * 0.05),
            delta_v: Fx::from_f32(0.01),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct IdentityLyapunovValue {
    pub v: Fx,
}

pub fn identity_lyapunov(
    state: &IdentityState,
    baseline: &IdentityBaseline,
    weights: &IdentityLyapunovWeights,
) -> IdentityLyapunovValue {
    let mut acc = Fx::ZERO;
    let mut i = 0;
    while i < ID_STATE_DIM {
        let dx = Fx(state.x[i].0.wrapping_sub(baseline.x_bar[i].0));
        let term = weights.w[i] * dx.sq();
        acc = acc + term;
        i += 1;
    }
    IdentityLyapunovValue { v: acc }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ContinuityVerdict {
    Ok,
    DistanceExceeded,
    LyapunovExceeded,
}

#[derive(Copy, Clone, Debug)]
pub struct ContinuityCheckResult {
    pub distance: IdentityDistanceResult,
    pub v_prev: IdentityLyapunovValue,
    pub v_next: IdentityLyapunovValue,
    pub verdict: ContinuityVerdict,
}

pub fn check_identity_continuity(
    prev: &IdentityState,
    next: &IdentityState,
    baseline: &IdentityBaseline,
    cfg: &IdentityContinuityConfig,
) -> ContinuityCheckResult {
    let d_sq = identity_distance_sq(prev, next, &cfg.sigma);
    let v_prev = identity_lyapunov(prev, baseline, &cfg.lyap_weights);
    let v_next = identity_lyapunov(next, baseline, &cfg.lyap_weights);

    let mut verdict = ContinuityVerdict::Ok;

    if d_sq.d_sq.0 > cfg.eps_cont_sq.0 {
        verdict = ContinuityVerdict::DistanceExceeded;
    } else {
        let dv = Fx(v_next.v.0.wrapping_sub(v_prev.v.0));
        if dv.0 > cfg.delta_v.0 {
            verdict = ContinuityVerdict::LyapunovExceeded;
        }
    }

    ContinuityCheckResult {
        distance: d_sq,
        v_prev,
        v_next,
        verdict,
    }
}
