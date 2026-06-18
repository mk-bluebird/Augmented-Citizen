// augmented-citizen-mcp-server/src/main.rs

#![forbid(unsafe_code)]

mod context;
mod security;
mod tools;

use std::io::{self, BufRead, Write};
use std::time::SystemTime;

use anti_coercion_enclave::state_machine::AccessLevel;
use brain_identity_kernel::guard::KernelGuard;
use brain_identity_kernel::kernel::ViabilityKernel;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

const SERVER_NAME: &str = "augmented-citizen-mcp";
const SERVER_VERSION: &str = "0.1.0";
const SERVER_AUTHORITY: &str = "github.com/mk-bluebird/Augmented-Citizen";
const ALN_CLAUSE: &str = "ALN.MIGRATION.CYBERCORE_AUTHORITY.v1";

const PRIMARY_BOSTROM_ADDRESS: &str = "bostrom18sd2ujv24ual9c9psht7xj8knh6xaead9ye7";
const ALT_BOSTROM_ADDRESSES: &[&str] = &[
    "bostrom1ldgmtf20d6604a24ztr0jxht7xt7az4jhkmsrc",
    "zeta12x0up66pzyeretzyku8p4ccuxrjqtqpdc4y4x8",
    "0x519fC0eB4111323Cac44b70e1aE31c30e405802D",
];

#[derive(Debug, Serialize, Deserialize)]
struct JsonRpcRequest {
    jsonrpc: String,
    id: Option<JsonValue>,
    method: String,
    #[serde(default)]
    params: JsonValue,
}

#[derive(Debug, Serialize, Deserialize)]
struct JsonRpcResponse {
    jsonrpc: String,
    id: Option<JsonValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<JsonValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<JsonRpcError>,
}

impl JsonRpcResponse {
    fn success(id: Option<JsonValue>, result: JsonValue) -> Self {
        JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(result),
            error: None,
        }
    }

    fn error(id: Option<JsonValue>, code: i32, message: impl Into<String>) -> Self {
        JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id,
            result: None,
            error: Some(JsonRpcError {
                code,
                message: message.into(),
                data: None,
            }),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct JsonRpcError {
    code: i32,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<JsonValue>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ServerMetadata {
    name: String,
    version: String,
    authority: String,
    aln_clause: String,
    invariants: Vec<String>,
    features: ServerFeatures,
}

#[derive(Debug, Serialize, Deserialize)]
struct ServerFeatures {
    tools: Vec<String>,
    resources: Vec<String>,
    prompts: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct BioStateParams {
    host_did: String,
    bostrom_address: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct BioState {
    blood: f64,
    sugar: f64,
    protein: f64,
    lifeforce: f64,
    oxygen: f64,
    brain: f64,
    wave: f64,
    dw: f64,
    pain: f64,
    fear: f64,
    timestamp: String,
}

fn is_allowed_bostrom_address(addr: &str) -> bool {
    if addr == PRIMARY_BOSTROM_ADDRESS {
        return true;
    }
    ALT_BOSTROM_ADDRESSES.iter().any(|a| a == &addr)
}

fn now_iso8601() -> String {
    let now = SystemTime::now();
    let datetime: chrono::DateTime<chrono::Utc> = now.into();
    datetime.to_rfc3339()
}

fn handle_get_server_metadata(id: Option<JsonValue>) -> JsonRpcResponse {
    let metadata = ServerMetadata {
        name: SERVER_NAME.to_string(),
        version: SERVER_VERSION.to_string(),
        authority: SERVER_AUTHORITY.to_string(),
        aln_clause: ALN_CLAUSE.to_string(),
        invariants: vec![
            "no_reversal".to_string(),
            "no_downgrade".to_string(),
            "no_rollback".to_string(),
        ],
        features: ServerFeatures {
            tools: vec![
                "mcp.get_server_metadata".to_string(),
                "consent.refresh_verdict".to_string(),
                "bio.read_state".to_string(),
                "economy.compute_upgrade_budget".to_string(),
                "upgrade.plan_ota_bundle".to_string(),
                "upgrade.validate_application_path".to_string(),
                "audit.query_activity_log".to_string(),
            ],
            resources: vec![
                "resource://augmented-citizen/profiles/{host_did}".to_string(),
                "resource://augmented-citizen/devices/{host_did}".to_string(),
                "resource://augmented-citizen/policies/non_reversal".to_string(),
            ],
            prompts: vec![
                "prompt://augmented-citizen/prepare_upgrade_request".to_string(),
                "prompt://augmented-citizen/interpret_bio_state".to_string(),
            ],
        },
    };

    JsonRpcResponse::success(id, serde_json::to_value(metadata).unwrap())
}

fn handle_bio_read_state(id: Option<JsonValue>, params: JsonValue) -> JsonRpcResponse {
    let parsed: Result<BioStateParams, _> = serde_json::from_value(params);
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

    if !is_allowed_bostrom_address(&params.bostrom_address) {
        return JsonRpcResponse::error(
            id,
            -32001,
            "Unauthorized bostrom_address for this server",
        );
    }

    let state = BioState {
        blood: 1.0,
        sugar: 1.0,
        protein: 1.0,
        lifeforce: 1.0,
        oxygen: 1.0,
        brain: 1.0,
        wave: 1.0,
        dw: 1.0,
        pain: 0.0,
        fear: 0.0,
        timestamp: now_iso8601(),
    };

    JsonRpcResponse::success(id, serde_json::to_value(state).unwrap())
}

fn handle_unknown_method(id: Option<JsonValue>, method: &str) -> JsonRpcResponse {
    JsonRpcResponse::error(
        id,
        -32601,
        format!("Method not found: {}", method),
    )
}

fn dispatch_with_context(
    ctx: &mut context::SessionContext,
    req: JsonRpcRequest,
    kernel_guard: &KernelGuard<'_>,
) -> JsonRpcResponse {
    use security::allowed_for;

    let method = req.method.clone();
    let access = allowed_for(ctx.verdict, &method);

    match method.as_str() {
        "mcp.get_server_metadata" => handle_get_server_metadata(req.id),
        "consent.refresh_verdict" => {
            tools::consent::handle_consent_refresh(ctx, kernel_guard, req.id, req.params)
        }
        "upgrade.plan_ota_bundle" => match access {
            AccessLevel::Allow => {
                tools::upgrade_planner::handle_upgrade_plan_ota_bundle(
                    req.id,
                    req.params,
                )
            }
            AccessLevel::Restricted => {
                let mut resp = tools::upgrade_planner::handle_upgrade_plan_ota_bundle(
                    req.id,
                    req.params,
                );
                if let Some(ref mut result) = resp.result {
                    if let Some(obj) = result.as_object_mut() {
                        obj.insert(
                            "requires_step_up".to_string(),
                            serde_json::json!(true),
                        );
                    }
                }
                resp
            }
            AccessLevel::Deny => JsonRpcResponse::error(
                req.id,
                -32030,
                "Upgrade tool denied under current consent verdict",
            ),
        },
        "economy.compute_upgrade_budget" => match access {
            AccessLevel::Allow | AccessLevel::Restricted => {
                tools::token_economy::handle_economy_compute_upgrade_budget(
                    req.id,
                    req.params,
                )
            }
            AccessLevel::Deny => JsonRpcResponse::error(
                req.id,
                -32031,
                "Economy tool denied under current consent verdict",
            ),
        },
        "bio.read_state" => {
            let resp = handle_bio_read_state(req.id, req.params);
            if let AccessLevel::Restricted = access {
            }
            resp
        }
        "audit.query_activity_log" => {
            tools::audit::handle_audit_query(ctx, req.id, req.params)
        }
        other => handle_unknown_method(req.id, other),
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut reader = stdin.lock();

    let kernel = ViabilityKernel {
        a: [[brain_identity_kernel::fixed::Fx::zero(); brain_identity_kernel::kernel::STATE_DIM];
            brain_identity_kernel::kernel::INEQUALITY_COUNT],
        b: [brain_identity_kernel::fixed::Fx::zero();
            brain_identity_kernel::kernel::INEQUALITY_COUNT],
    };
    let kernel_guard = KernelGuard::new(&kernel);
    let mut ctx = context::SessionContext::new();

    loop {
        let mut line = String::new();
        let bytes_read = reader.read_line(&mut line).unwrap();
        if bytes_read == 0 {
            break;
        }

        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let req: Result<JsonRpcRequest, _> = serde_json::from_str(trimmed);
        let req = match req {
            Ok(r) => r,
            Err(e) => {
                let error_response =
                    JsonRpcResponse::error(None, -32700, format!("Parse error: {}", e));
                let serialized = serde_json::to_string(&error_response).unwrap();
                writeln!(stdout, "{}", serialized).unwrap();
                stdout.flush().unwrap();
                continue;
            }
        };

        let resp = dispatch_with_context(&mut ctx, req, &kernel_guard);
        let serialized = serde_json::to_string(&resp).unwrap();
        writeln!(stdout, "{}", serialized).unwrap();
        stdout.flush().unwrap();
    }
}
