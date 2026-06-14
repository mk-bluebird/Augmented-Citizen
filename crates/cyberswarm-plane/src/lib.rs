// crates/cyberswarm-plane/src/lib.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Polytope {
    pub a: Vec<Vec<f64>>,
    pub b: Vec<f64>,
}

impl Polytope {
    pub fn violation(&self, x: &[f64]) -> f64 {
        let mut max_violation = f64::NEGATIVE_INFINITY;
        for (row, b_i) in self.a.iter().zip(self.b.iter()) {
            let dot: f64 = row.iter().zip(x.iter()).map(|(a, xi)| a * xi).sum();
            let v = dot - b_i;
            if v > max_violation {
                max_violation = v;
            }
        }
        max_violation.max(0.0)
    }
}
