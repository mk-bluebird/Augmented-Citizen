// augmented-citizen-mcp-server/src/security.rs

#![forbid(unsafe_code)]

use anti_coercion_enclave::state_machine::{AccessLevel, ConsentVerdict};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ToolRisk {
    Low,
    Medium,
    High,
}

pub fn tool_risk(method: &str) -> ToolRisk {
    match method {
        "upgrade.plan_ota_bundle" => ToolRisk::High,
        "economy.compute_upgrade_budget" => ToolRisk::Medium,
        "bio.read_state" => ToolRisk::Medium,
        "audit.query_activity_log" => ToolRisk::Low,
        "consent.refresh_verdict" => ToolRisk::Low,
        "mcp.get_server_metadata" => ToolRisk::Low,
        _ => ToolRisk::Low,
    }
}

pub fn allowed_for(verdict: ConsentVerdict, method: &str) -> AccessLevel {
    let risk = tool_risk(method);
    let level = verdict.access_level();

    match risk {
        ToolRisk::High => match level {
            AccessLevel::Allow => AccessLevel::Allow,
            AccessLevel::Restricted => AccessLevel::Restricted,
            AccessLevel::Deny => AccessLevel::Deny,
        },
        ToolRisk::Medium => match level {
            AccessLevel::Allow | AccessLevel::Restricted => AccessLevel::Restricted,
            AccessLevel::Deny => AccessLevel::Deny,
        },
        ToolRisk::Low => match level {
            AccessLevel::Allow | AccessLevel::Restricted => AccessLevel::Allow,
            AccessLevel::Deny => AccessLevel::Restricted,
        },
    }
}
