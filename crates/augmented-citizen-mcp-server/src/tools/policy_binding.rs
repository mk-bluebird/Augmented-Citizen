// augmented-citizen-mcp-server/src/tools/policy_binding.rs

#![forbid(unsafe_code)]

use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use brain_identity_kernel::header::KernelHeader;

use crate::policy_hash::compute_canonical_policy_hash;
use crate::protocol::jsonrpc::JsonRpcResponse;

#[derive(Debug, Serialize, Deserialize)]
pub struct PolicyBindingResult {
    pub binding_ok: bool,
    pub attestation_tag: String,
}

fn derive_attestation_tag(kernel_header: &KernelHeader) -> String {
    // Short, non-invertible tag:
    // combine current UTC minute + last 4 bytes of policy_hash, run through a small mixing function.
    let now = Utc::now();
    let minute = now.format("%Y%m%d%H%M").to_string();

    let mut mix: u32 = 0;
    let h = &kernel_header.policy_hash;
    let len = h.len();
    if len >= 4 {
        mix = (h[len - 4] as u32) << 24
            | (h[len - 3] as u32) << 16
            | (h[len - 2] as u32) << 8
            | (h[len - 1] as u32);
    }

    let mut acc: u32 = 0xA5A5_5A5A;
    for b in minute.as_bytes() {
        acc = acc.rotate_left(3) ^ (*b as u32);
        acc = acc.wrapping_add(mix);
    }

    // encode as 8 hex chars
    let hex = {
        const HEX: &[u8; 16] = b"0123456789abcdef";
        let mut out = [0u8; 8];
        let mut v = acc;
        let mut i = 0;
        while i < 8 {
            let nib = (v & 0x0F) as u8;
            out[7 - i] = HEX[nib as usize];
            v >>= 4;
            i += 1;
        }
        core::str::from_utf8(&out).unwrap().to_string()
    };

    format!("AC-BIND-{}", hex)
}

pub fn handle_policy_verify_binding(
    kernel_header: &KernelHeader,
    id: Option<JsonValue>,
) -> JsonRpcResponse {
    let canonical = compute_canonical_policy_hash();
    let canonical_bytes = canonical.0;

    // Compare without leaking either hash value directly.
    let mut equal = true;
    let mut i = 0usize;
    while i < 32 {
        if kernel_header.policy_hash[i] != canonical_bytes[i] {
            equal = false;
        }
        i += 1;
    }

    let attestation_tag = derive_attestation_tag(kernel_header);

    let result = PolicyBindingResult {
        binding_ok: equal,
        attestation_tag,
    };

    JsonRpcResponse::success(id, serde_json::to_value(result).unwrap())
}
