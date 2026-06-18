// augmented-citizen-mcp-server/src/main.rs

#![forbid(unsafe_code)]

use std::io::{self, BufRead, Write};
use std::time::SystemTime;

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
#[serde(rename_all = "lowercase")]
enum JsonRpcVersion {
    V2_0,
}

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
                "bio.read_state".to_string(),
                "bio.project_state_window".to_string(),
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

    JsonRpcResponse {
        jsonrpc: "2.0".to_string(),
        id,
        result: Some(serde_json::to_value(metadata).unwrap()),
        error: None,
    }
}

fn handle_bio_read_state(id: Option<JsonValue>, params: JsonValue) -> JsonRpcResponse {
    let parsed: Result<BioStateParams, _> = serde_json::from_value(params);
    let params = match parsed {
        Ok(p) => p,
        Err(e) => {
            return JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id,
                result: None,
                error: Some(JsonRpcError {
                    code: -32602,
                    message: format!("Invalid params: {}", e),
                    data: None,
                }),
            };
        }
    };

    if !is_allowed_bostrom_address(&params.bostrom_address) {
        return JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id,
            result: None,
            error: Some(JsonRpcError {
                code: -32001,
                message: "Unauthorized bostrom_address for this server".to_string(),
                data: None,
            }),
        };
    }

    // Placeholder for real bio-signal integration: keep strictly read-only and non-reversing.
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

    JsonRpcResponse {
        jsonrpc: "2.0".to_string(),
        id,
        result: Some(serde_json::to_value(state).unwrap()),
        error: None,
    }
}

fn handle_unknown_method(id: Option<JsonValue>, method: &str) -> JsonRpcResponse {
    JsonRpcResponse {
        jsonrpc: "2.0".to_string(),
        id,
        result: None,
        error: Some(JsonRpcError {
            code: -32601,
            message: format!("Method not found: {}", method),
            data: None,
        }),
    }
}

fn dispatch(request: JsonRpcRequest) -> JsonRpcResponse {
    match request.method.as_str() {
        "mcp.get_server_metadata" => handle_get_server_metadata(request.id),
        "bio.read_state" => handle_bio_read_state(request.id, request.params),
        other => handle_unknown_method(request.id, other),
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut reader = stdin.lock();

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
                let error_response = JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: None,
                    result: None,
                    error: Some(JsonRpcError {
                        code: -32700,
                        message: format!("Parse error: {}", e),
                        data: None,
                    }),
                };
                let serialized = serde_json::to_string(&error_response).unwrap();
                writeln!(stdout, "{}", serialized).unwrap();
                stdout.flush().unwrap();
                continue;
            }
        };

        let resp = dispatch(req);
        let serialized = serde_json::to_string(&resp).unwrap();
        writeln!(stdout, "{}", serialized).unwrap();
        stdout.flush().unwrap();
    }
}
