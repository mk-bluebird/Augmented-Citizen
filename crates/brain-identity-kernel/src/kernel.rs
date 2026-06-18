// crates/brain-identity-kernel/src/kernel.rs

#![forbid(unsafe_code)]

use crate::fixed::Fx;

pub const STATE_DIM: usize = 6;
pub const INEQUALITY_COUNT: usize = 16;

#[derive(Copy, Clone, Debug)]
pub struct ViabilityKernelState {
    inner: [Fx; STATE_DIM],
}

impl ViabilityKernelState {
    pub fn new(i: Fx, d: Fx, c: Fx, p: Fx, n: Fx, l: Fx) -> Result<Self, ()> {
        let s = ViabilityKernelState {
            inner: [i, d, c, p, n, l],
        };
        if s.is_within_bounds() {
            Ok(s)
        } else {
            Err(())
        }
    }

    pub fn as_array(&self) -> &[Fx; STATE_DIM] {
        &self.inner
    }

    fn is_within_bounds(&self) -> bool {
        for v in &self.inner {
            if v.0 < -65536 || v.0 > 65536 {
                return false;
            }
        }
        true
    }
}

pub struct ViabilityKernel {
    pub a: [[Fx; STATE_DIM]; INEQUALITY_COUNT],
    pub b: [Fx; INEQUALITY_COUNT],
}

impl ViabilityKernel {
    pub fn new(
        a: [[Fx; STATE_DIM]; INEQUALITY_COUNT],
        b: [Fx; INEQUALITY_COUNT],
    ) -> Self {
        ViabilityKernel { a, b }
    }
}

use crate::header::KernelHeader;
use crate::shard::{verify_and_decode, RawShard, ShardError};

pub struct LoadedKernel {
    pub kernel: ViabilityKernel,
    pub header: KernelHeader,
}

impl LoadedKernel {
    pub fn from_aln_bytes(
        bytes: &[u8],
        expected_host_did: &str,
    ) -> Result<Self, ShardError> {
        let header =
            KernelHeader::from_bytes(bytes).ok_or(ShardError::InvalidEncoding)?;
        let raw = RawShard { bytes };
        let verified = verify_and_decode(raw, expected_host_did)?;
        Ok(LoadedKernel {
            kernel: verified.into_kernel(),
            header,
        })
    }
}
