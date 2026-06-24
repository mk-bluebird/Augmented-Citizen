// Identity state vector x_t and baseline x̄ for Augmented-Citizen.

use crate::fixed::Fx;

pub const ID_STATE_DIM: usize = 8;

#[derive(Copy, Clone, Debug)]
pub struct IdentityState {
    pub x: [Fx; ID_STATE_DIM],
}

impl IdentityState {
    pub fn zeros() -> Self {
        IdentityState { x: [Fx::ZERO; ID_STATE_DIM] }
    }

    pub fn from_f32_slice(vals: [f32; ID_STATE_DIM]) -> Self {
        let mut x = [Fx::ZERO; ID_STATE_DIM];
        let mut i = 0;
        while i < ID_STATE_DIM {
            x[i] = Fx::from_f32(vals[i]);
            i += 1;
        }
        IdentityState { x }
    }

    pub fn as_array(&self) -> &[Fx; ID_STATE_DIM] {
        &self.x
    }
}

#[derive(Copy, Clone, Debug)]
pub struct IdentityBaseline {
    pub x_bar: [Fx; ID_STATE_DIM],
}

impl IdentityBaseline {
    pub fn new(x_bar: [Fx; ID_STATE_DIM]) -> Self {
        IdentityBaseline { x_bar }
    }

    pub fn zeros() -> Self {
        IdentityBaseline { x_bar: [Fx::ZERO; ID_STATE_DIM] }
    }
}
