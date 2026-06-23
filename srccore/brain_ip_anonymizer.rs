// srccore/brain_ip_anonymizer.rs

/// Public projection matrix P, baked from ALN / registry.
/// This projects raw feature vector x into a small safety subspace.
pub struct SafetyProjector {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<f32>, // row-major P
}

impl SafetyProjector {
    pub fn project(&self, x: &[f32]) -> Vec<f32> {
        assert_eq!(x.len(), self.cols);
        let mut z = vec![0.0f32; self.rows];
        for r in 0..self.rows {
            let mut acc = 0.0;
            for c in 0..self.cols {
                acc += self.data[r * self.cols + c] * x[c];
            }
            z[r] = acc;
        }
        z
    }
}

/// Host-local mask derived from BrainIdentityToken + time.
/// In practice, key derivation stays inside enclave; this API just takes a seed.
pub struct HostMask {
    factors: Vec<f32>,
}

impl HostMask {
    pub fn from_seed(seed: &[u8], dim: usize) -> Self {
        use blake3::Hasher; // NOTE: replace with a non-blacklisted, allowed hash if needed.
        let mut factors = Vec::with_capacity(dim);
        let mut h = blake3::Hasher::new(); // If blake is blacklisted, replace with another allowed primitive.
        h.update(seed);
        let base = h.finalize();

        for i in 0..dim {
            let mut bytes = [0u8; 16];
            for j in 0..16 {
                bytes[j] = base.as_bytes()[(i + j) % 32];
            }
            let val = f32::from_le_bytes(bytes[0..4].try_into().unwrap());
            // Map to { -1.0, +1.0 } style factors for orthant flips.
            let f = if val.is_sign_negative() { -1.0 } else { 1.0 };
            factors.push(f);
        }

        Self { factors }
    }

    pub fn apply(&self, z: &mut [f32]) {
        assert_eq!(z.len(), self.factors.len());
        for (zi, fi) in z.iter_mut().zip(self.factors.iter()) {
            *zi *= *fi;
        }
    }
}

/// Quantize to coarse bands [0..Q-1] for identity-unlearnable export.
pub fn quantize(z: &[f32], q_levels: u8) -> Vec<u8> {
    let q = q_levels.max(2);
    z.iter()
        .map(|v| {
            // Clamp to [0,1] then map to discrete bins.
            let clamped = v.clamp(0.0, 1.0);
            let idx = (clamped * (q as f32 - 1.0)).round() as u8;
            idx
        })
        .collect()
}

/// High-level anonymizer entry point used by the BLE stack.
pub fn anonymize_eeg_features(
    projector: &SafetyProjector,
    host_mask: &HostMask,
    raw_features: &[f32],
    q_levels: u8,
) -> Vec<u8> {
    let mut z = projector.project(raw_features);
    host_mask.apply(&mut z);
    quantize(&z, q_levels)
}
