// augmented-citizen-mcp-server/src/audit.rs

#![forbid(unsafe_code)]

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use anti_coercion_enclave::state_machine::{AccessLevel, ConsentVerdict};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub timestamp: DateTime<Utc>,
    pub host_did: String,
    pub bostrom_address: String,
    pub method: String,
    pub access_level: AccessLevel,
    pub verdict: ConsentVerdict,
    pub status_code: i32,
    pub status_message: String,
    pub params_fingerprint: Option<JsonValue>,
}

#[derive(Debug, Default)]
pub struct AuditLog {
    entries: heapless::Vec<AuditEntry, 256>,
}

impl AuditLog {
    pub fn new() -> Self {
        AuditLog {
            entries: heapless::Vec::new(),
        }
    }

    pub fn record(&mut self, entry: AuditEntry) {
        // If full, drop the oldest entry to preserve recent context.
        if self.entries.is_full() {
            let _ = self.entries.remove(0);
        }
        let _ = self.entries.push(entry);
    }

    pub fn snapshot(&self) -> heapless::Vec<AuditEntry, 256> {
        self.entries.clone()
    }
}
