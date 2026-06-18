use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UpgradeModule {
    pub id: String,
    pub version: String,
    pub capability_delta: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PolicyViolation {
    pub code: String,
    pub message: String,
}

pub fn enforce_non_reversal(modules: &[UpgradeModule]) -> Vec<PolicyViolation> {
    let mut violations = Vec::new();

    for m in modules {
        if m.capability_delta < 0 {
            violations.push(PolicyViolation {
                code: "NON_REVERSAL".to_string(),
                message: format!(
                    "Module {} would reduce capabilities (delta={})",
                    m.id, m.capability_delta
                ),
            });
        }

        if m.id.to_lowercase().contains("downgrade")
            || m.id.to_lowercase().contains("rollback")
            || m.id.to_lowercase().contains("reverse")
        {
            violations.push(PolicyViolation {
                code: "FORBIDDEN_MODULE".to_string(),
                message: format!(
                    "Module {} is forbidden by non-reversal policy",
                    m.id
                ),
            });
        }
    }

    violations
}
