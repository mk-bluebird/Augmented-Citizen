// crates/state-only-channel/src/state_only_vec.rs

#![forbid(unsafe_code)]

use std::collections::HashMap;

/// StateOnlyVec16 is a 16-dimensional, normalized, state-only export
/// used to project inner-state features without exposing raw signals.
#[derive(Debug, Clone)]
pub struct StateOnlyVec16 {
    axes: HashMap<String, f32>,
}

impl StateOnlyVec16 {
    pub fn new(axes: HashMap<String, f32>) -> Self {
        Self { axes }
    }

    /// Retrieve an axis by name and clamp to [0.0, 1.0].
    ///
    /// Missing axes default to 0.0.
    pub fn get_axis_normalized(&self, name: &str) -> f32 {
        let value = self.axes.get(name).copied().unwrap_or(0.0);
        if value.is_nan() {
            0.0
        } else if value < 0.0 {
            0.0
        } else if value > 1.0 {
            1.0
        } else {
            value
        }
    }
}
