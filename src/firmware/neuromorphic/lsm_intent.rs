#![forbid(unsafe_code)]
#![no_std]

use core::marker::PhantomData;

use crate::neural_intent::NeuralIntentToken;
use crate::psych::PsychRiskIndex;

/// Fixed-size reservoir for embedded firmware.
pub struct LiquidStateMachine<const N: usize> {
    v: [f32; N],
    v_rest: [f32; N],
    w_recurrent: [[f32; N]; N],
    w_in: [[f32; N]; N],
    alpha: f32,
    beta: f32,
}

impl<const N: usize> LiquidStateMachine<N> {
    pub fn new(
        v_rest: [f32; N],
        w_recurrent: [[f32; N]; N],
        w_in: [[f32; N]; N],
        alpha: f32,
        beta: f32,
    ) -> Self {
        Self {
            v: v_rest,
            v_rest,
            w_recurrent,
            w_in,
            alpha,
            beta,
        }
    }

    /// One integration step over spikes, returns (stable, V_next).
    pub fn step(
        &mut self,
        spikes: &[f32; N],
        psych: PsychRiskIndex,
        dt: f32,
    ) -> (bool, f32) {
        let v_prev = self.v;
        let v_next = self.integrate(&v_prev, spikes, dt);
        let v_prev_dev = Self::deviation(&v_prev, &self.v_rest);
        let v_next_dev = Self::deviation(&v_next, &self.v_rest);
        let v_prev_norm = dot(&v_prev_dev, &v_prev_dev);
        let v_next_norm = dot(&v_next_dev, &v_next_dev);
        let lhs = v_next_norm - v_prev_norm;
        let rhs = -self.alpha * v_prev_norm * dt + self.beta * psych.value * dt;
        let stable = lhs <= rhs + 1e-6;
        if stable {
            self.v = v_next;
        }
        (stable, v_next_norm)
    }

    fn integrate(&self, v_prev: &[f32; N], spikes: &[f32; N], dt: f32) -> [f32; N] {
        let mut v_next = *v_prev;
        for i in 0..N {
            let mut recurrent_sum = 0.0;
            let mut input_sum = 0.0;
            for j in 0..N {
                recurrent_sum += self.w_recurrent[i][j] * v_prev[j];
                input_sum += self.w_in[i][j] * spikes[j];
            }
            let dv = -self.alpha * (v_prev[i] - self.v_rest[i]) + recurrent_sum + input_sum;
            v_next[i] = v_prev[i] + dt * dv;
        }
        v_next
    }

    fn deviation(v: &[f32; N], v_rest: &[f32; N]) -> [f32; N] {
        let mut out = [0.0; N];
        for i in 0..N {
            out[i] = v[i] - v_rest[i];
        }
        out
    }

    pub fn readout(&self, weights: &[[f32; N; 3]]) -> [f32; 3] {
        // Example: 3-class output; this can be expanded and mapped to tokens.
        let mut y = [0.0f32; 3];
        for c in 0..3 {
            for i in 0..N {
                y[c] += weights[c][i] * self.v[i];
            }
        }
        y
    }
}

fn dot<const N: usize>(a: &[f32; N], b: &[f32; N]) -> f32 {
    let mut acc = 0.0;
    for i in 0..N {
        acc += a[i] * b[i];
    }
    acc
}

/// High-level recognizer wiring LSM to logits and tokens.
pub struct SpikingIntentRecognizer<const N: usize> {
    lsm: LiquidStateMachine<N>,
    readout_weights: [[f32; N]; 3],
    _marker: PhantomData<[u8; N]>,
}

impl<const N: usize> SpikingIntentRecognizer<N> {
    pub fn new(
        lsm: LiquidStateMachine<N>,
        readout_weights: [[f32; N]; 3],
    ) -> Self {
        Self {
            lsm,
            readout_weights,
            _marker: PhantomData,
        }
    }

    /// Returns Option<(token, stable, V)> so host can log instability.
    pub fn classify(
        &mut self,
        spikes: &[f32; N],
        psych: PsychRiskIndex,
        dt: f32,
    ) -> Option<(NeuralIntentToken, bool, f32)> {
        let (stable, v_norm) = self.lsm.step(spikes, psych, dt);
        if !stable {
            return None;
        }
        let logits = self.lsm.readout(&self.readout_weights);
        let (idx, _) = logits
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            ?;
        let token = match idx {
            0 => NeuralIntentToken::Rest,
            1 => NeuralIntentToken::AskState,
            2 => NeuralIntentToken::PauseDuty,
            _ => return None,
        };
        Some((token, true, v_norm))
    }
}
