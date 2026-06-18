// augmented-citizen-mcp-server/src/tools/audit_tool.rs

#![forbid(unsafe_code)]

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::audit::{AuditEntry, AuditLog};
use crate::context::SessionContext;
use crate::protocol::jsonrpc::JsonRpcResponse;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditQueryParams {
    pub host_did: Option<String>,
    pub bostrom_address: Option<String>,
    pub from: Option<DateTime<Utc>>,
    pub to: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditQueryResult {
    pub entries: heapless::Vec<AuditEntry, 256>,
}

pub fn handle_audit_query_activity_log(
    ctx: &SessionContext<'_>,
    id: Option<JsonValue>,
    params: JsonValue,
) -> JsonRpcResponse {
    let parsed: Result<AuditQueryParams, _> = serde_json::from_value(params);
    let params = match parsed {
        Ok(p) => p,
        Err(e) => {
            return JsonRpcResponse::error(
                id,
                -32602,
                format!("Invalid params: {}", e),
            );
        }
    };

    let host_filter = params.host_did.as_deref().unwrap_or(&ctx.host_did);
    let addr_filter = params
        .bostrom_address
        .as_deref()
        .unwrap_or(&ctx.bostrom_address);

    let from = params.from.unwrap_or_else(|| DateTime::<Utc>::MIN_UTC);
    let to = params.to.unwrap_or_else(|| DateTime::<Utc>::MAX_UTC);

    let snapshot = ctx.audit_log.snapshot();

    let mut filtered: heapless::Vec<AuditEntry, 256> = heapless::Vec::new();
    for e in snapshot.iter() {
        if e.host_did != host_filter {
            continue;
        }
        if e.bostrom_address != addr_filter {
            continue;
        }
        if e.timestamp < from || e.timestamp > to {
            continue;
        }
        let _ = filtered.push(e.clone());
    }

    let result = AuditQueryResult { entries: filtered };

    JsonRpcResponse::success(id, serde_json::to_value(result).unwrap())
}
