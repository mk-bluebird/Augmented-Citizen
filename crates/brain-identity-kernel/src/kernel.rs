// crates/brain-identity-kernel/src/kernel.rs (bottom)

use crate::shard::{verify_and_decode, RawShard, ShardError};

impl ViabilityKernel {
    pub fn from_aln_bytes(bytes: &[u8], expected_host_did: &str) -> Result<Self, ShardError> {
        let raw = RawShard { bytes };
        let verified = verify_and_decode(raw, expected_host_did)?;
        Ok(verified.into_kernel())
    }
}
