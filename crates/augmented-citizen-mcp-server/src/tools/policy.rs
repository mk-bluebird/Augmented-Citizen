// augmented-citizen-mcp-server/src/tools/policy.rs

#![forbid(unsafe_code)]

use serde_json::Value as JsonValue;

use crate::protocol::jsonrpc::JsonRpcResponse;
use crate::resources::non_reversal_policy_resource;

pub fn handle_policy_get_non_reversal(id: Option<JsonValue>) -> JsonRpcResponse {
    let res = non_reversal_policy_resource();
    JsonRpcResponse::success(id, serde_json::to_value(res).unwrap())
}
