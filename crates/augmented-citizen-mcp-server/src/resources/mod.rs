// augmented-citizen-mcp-server/src/resources/mod.rs

#![forbid(unsafe_code)]

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PolicyResource {
    pub id: String,
    pub title: String,
    pub version: String,
    pub description: String,
    pub invariants: Vec<String>,
    pub policy_hash_hex: String,
}

pub fn canonical_non_reversal_policy_json() -> &'static str {
    // Canonical JSON: stable ordering and text.
    // This exact string must be used when computing policy_hash in the ALN shard.
    r#"{
  "id": "resource://augmented-citizen/policies/non_reversal",
  "version": "2026-06-18",
  "invariants": [
    "No module with negative capability_delta is allowed.",
    "No module ID containing 'downgrade', 'rollback', or 'reverse' is allowed.",
    "All viability-kernel checks must pass before actuation.",
    "All high-risk operations require explicit host consent via MCP host UX."
  ]
}"#
}

pub fn non_reversal_policy_resource(policy_hash_hex: String) -> PolicyResource {
    PolicyResource {
        id: "resource://augmented-citizen/policies/non_reversal".to_string(),
        title: "Non-Reversal Capability Policy".to_string(),
        version: "2026-06-18".to_string(),
        description: "This policy enforces that no cybernetic or neuromorphic upgrade applied \
                      through the Augmented-Citizen MCP server may reduce the host's functional \
                      capabilities, rights, or sovereignty."
            .to_string(),
        invariants: vec![
            "No module with negative capability_delta is allowed.".to_string(),
            "No module ID containing 'downgrade', 'rollback', or 'reverse' is allowed."
                .to_string(),
            "All viability-kernel checks must pass before actuation.".to_string(),
            "All high-risk operations require explicit host consent via MCP host UX."
                .to_string(),
        ],
        policy_hash_hex,
    }
}
