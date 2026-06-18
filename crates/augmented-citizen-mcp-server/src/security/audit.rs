use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditEntry {
    pub timestamp: DateTime<Utc>,
    pub host_did: String,
    pub bostrom_address: String,
    pub tool_name: String,
    pub status: String,
    pub invariants_satisfied: Vec<String>,
}

#[derive(Default)]
pub struct AuditLog {
    entries: Vec<AuditEntry>,
}

impl AuditLog {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    pub fn record(
        &mut self,
        host_did: String,
        bostrom_address: String,
        tool_name: String,
        status: String,
        invariants_satisfied: Vec<String>,
    ) {
        self.entries.push(AuditEntry {
            timestamp: Utc::now(),
            host_did,
            bostrom_address,
            tool_name,
            status,
            invariants_satisfied,
        });
    }

    pub fn query(
        &self,
        host_did: &str,
        bostrom_address: &str,
    ) -> Vec<AuditEntry> {
        self.entries
            .iter()
            .filter(|e| e.host_did == host_did && e.bostrom_address == bostrom_address)
            .cloned()
            .collect()
    }
}
