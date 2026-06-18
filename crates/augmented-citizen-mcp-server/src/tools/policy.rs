// augmented-citizen-mcp-server/src/tools/policy.rs

#![forbid(unsafe_code)]

use serde_json::Value as JsonValue;

use crate::policy_hash::compute_canonical_policy_hash;
use crate::protocol::jsonrpc::JsonRpcResponse;
use crate::resources::non_reversal_policy_resource;

pub fn handle_policy_get_non_reversal(id: Option<JsonValue>) -> JsonRpcResponse {
    let hash = compute_canonical_policy_hash();
    let policy_hash_hex = hash.to_hex();
    let res = non_reversal_policy_resource(policy_hash_hex);
    JsonRpcResponse::success(id, serde_json::to_value(res).unwrap())
}
