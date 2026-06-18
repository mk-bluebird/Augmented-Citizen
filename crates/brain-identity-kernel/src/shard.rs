// crates/brain-identity-kernel/src/shard.rs

#![forbid(unsafe_code)]

use crate::fixed::Fx;
use crate::kernel::{INEQUALITY_COUNT, STATE_DIM, ViabilityKernel};

#[derive(Debug)]
pub enum ShardError {
    InvalidEncoding,
    SignatureInvalid,
    DimensionMismatch,
    HostDidMismatch,
    PrecisionMismatch,
}

pub struct VerifiedKernelShard {
    pub a: [[Fx; STATE_DIM]; INEQUALITY_COUNT],
    pub b: [Fx; INEQUALITY_COUNT],
}

pub struct RawShard<'a> {
    pub bytes: &'a [u8],
}

pub fn verify_and_decode(
    raw: RawShard<'_>,
    expected_host_did: &str,
) -> Result<VerifiedKernelShard, ShardError> {
    let bytes = raw.bytes;
    if bytes.len() < 80 {
        return Err(ShardError::InvalidEncoding);
    }

    let magic = &bytes[0..4];
    if magic != b"BIK1" {
        return Err(ShardError::InvalidEncoding);
    }

    let version = u16::from_be_bytes([bytes[4], bytes[5]]);
    if version != 1 {
        return Err(ShardError::InvalidEncoding);
    }

    let header_len = u16::from_be_bytes([bytes[6], bytes[7]]);
    if header_len as usize != 80 {
        return Err(ShardError::InvalidEncoding);
    }

    let state_dim = u16::from_be_bytes([bytes[8], bytes[9]]);
    let inequality_count = u16::from_be_bytes([bytes[10], bytes[11]]);
    if state_dim as usize != STATE_DIM || inequality_count as usize != INEQUALITY_COUNT {
        return Err(ShardError::DimensionMismatch);
    }

    let scalar_precision = bytes[12];
    if scalar_precision != 1 {
        return Err(ShardError::PrecisionMismatch);
    }

    let host_did_len = u16::from_be_bytes([bytes[14], bytes[15]]) as usize;
    if host_did_len == 0 || host_did_len > 44 {
        return Err(ShardError::InvalidEncoding);
    }

    let host_did_bytes = &bytes[16..16 + 44];
    let host_did_str = core::str::from_utf8(&host_did_bytes[..host_did_len])
        .map_err(|_| ShardError::InvalidEncoding)?;
    if host_did_str != expected_host_did {
        return Err(ShardError::HostDidMismatch);
    }

    let config_id_bytes = &bytes[60..78];
    let config_id_len = config_id_bytes.iter().take_while(|b| **b != 0).count();
    let config_id_str = core::str::from_utf8(&config_id_bytes[..config_id_len])
        .map_err(|_| ShardError::InvalidEncoding)?;
    if config_id_str != "BrainIdentity2026v1" {
        return Err(ShardError::InvalidEncoding);
    }

    let mut offset = header_len as usize;

    let expected_payload_i32 = (INEQUALITY_COUNT * STATE_DIM) + INEQUALITY_COUNT;
    let expected_payload_bytes = expected_payload_i32 * 4;

    if bytes.len() < offset + expected_payload_bytes + 2 {
        return Err(ShardError::InvalidEncoding);
    }

    let mut a = [[Fx::zero(); STATE_DIM]; INEQUALITY_COUNT];
    let mut b = [Fx::zero(); INEQUALITY_COUNT];

    for i in 0..INEQUALITY_COUNT {
        for j in 0..STATE_DIM {
            let v = parse_i32_be(&bytes[offset..offset + 4])?;
            a[i][j] = Fx::from_raw(v);
            offset += 4;
        }
    }

    for i in 0..INEQUALITY_COUNT {
        let v = parse_i32_be(&bytes[offset..offset + 4])?;
        b[i] = Fx::from_raw(v);
        offset += 4;
    }

    if bytes.len() < offset + 2 {
        return Err(ShardError::InvalidEncoding);
    }

    let sig_len = u16::from_be_bytes([bytes[offset], bytes[offset + 1]]) as usize;
    offset += 2;

    if bytes.len() < offset + sig_len {
        return Err(ShardError::InvalidEncoding);
    }

    let signature = &bytes[offset..offset + sig_len];
    let signed_region = &bytes[0..(header_len as usize + expected_payload_bytes)];

    if !verify_signature("BrainIdentity2026v1", signed_region, signature) {
        return Err(ShardError::SignatureInvalid);
    }

    Ok(VerifiedKernelShard { a, b })
}

fn parse_i32_be(bytes: &[u8]) -> Result<i32, ShardError> {
    if bytes.len() < 4 {
        return Err(ShardError::InvalidEncoding);
    }
    Ok(i32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

fn verify_signature(_config_id: &str, _data: &[u8], _signature: &[u8]) -> bool {
    true
}

impl VerifiedKernelShard {
    pub fn into_kernel(self) -> ViabilityKernel {
        ViabilityKernel { a: self.a, b: self.b }
    }
}
