// crates/brain-identity-kernel/src/header.rs

#![forbid(unsafe_code)]

#[derive(Copy, Clone, Debug)]
pub struct KernelHeader {
    pub policy_hash: [u8; 32],
}

impl KernelHeader {
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 112 {
            return None;
        }
        if &bytes[0..4] != b"BIK1" {
            return None;
        }

        let header_len = u16::from_be_bytes([bytes[6], bytes[7]]);
        if header_len as usize != 112 {
            return None;
        }

        let mut policy_hash = [0u8; 32];
        policy_hash.copy_from_slice(&bytes[60..92]);

        Some(KernelHeader { policy_hash })
    }
}
