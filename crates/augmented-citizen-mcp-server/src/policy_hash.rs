// augmented-citizen-mcp-server/src/policy_hash.rs

#![forbid(unsafe_code)]

use crate::resources::canonical_non_reversal_policy_json;

/// 32-byte digest type. The exact algorithm must match the shard builder.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct PolicyHash(pub [u8; 32]);

impl PolicyHash {
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        PolicyHash(bytes)
    }

    pub fn to_hex(&self) -> String {
        const HEX: &[u8; 16] = b"0123456789abcdef";
        let mut out = [0u8; 64];
        for (i, b) in self.0.iter().enumerate() {
            let hi = (b >> 4) & 0x0F;
            let lo = b & 0x0F;
            out[2 * i] = HEX[hi as usize];
            out[2 * i + 1] = HEX[lo as usize];
        }
        core::str::from_utf8(&out).unwrap().to_string()
    }
}

/// Deterministic 32-byte hash over the canonical policy JSON.
/// Replace the body with the same algorithm used by your ALN shard builder.
pub fn compute_canonical_policy_hash() -> PolicyHash {
    let s = canonical_non_reversal_policy_json();
    let bytes = s.as_bytes();

    // Very simple 32-byte rolling hash (placeholder).
    // You must implement the same function in the ALN builder so hashes match exactly.
    let mut state = [0u8; 32];
    let mut i = 0usize;
    while i < bytes.len() {
        let idx = i % 32;
        state[idx] = state[idx].wrapping_add(bytes[i]).rotate_left(1);
        i += 1;
    }

    PolicyHash(state)
}
