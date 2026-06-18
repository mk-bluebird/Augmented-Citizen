// augmented-citizen-mcp-server/src/context.rs

#![forbid(unsafe_code)]

use anti_coercion_enclave::state_machine::ConsentVerdict;
use brain_identity_kernel::fixed::Fx;

use crate::audit::AuditLog;

#[derive(Debug)]
pub struct SessionContext<'a> {
    pub verdict: ConsentVerdict,
    pub combined_intent: Fx,
    pub host_did: String,
    pub bostrom_address: String,
    pub audit_log: &'a mut AuditLog,
}

impl<'a> SessionContext<'a> {
    pub fn new(audit_log: &'a mut AuditLog) -> Self {
        SessionContext {
            verdict: ConsentVerdict::Invalid,
            combined_intent: Fx::zero(),
            host_did: String::new(),
            bostrom_address: String::new(),
            audit_log,
        }
    }

    pub fn update_verdict(&mut self, verdict: ConsentVerdict, combined_intent: Fx) {
        self.verdict = verdict;
        self.combined_intent = combined_intent;
    }

    pub fn set_identity(&mut self, host_did: String, bostrom_address: String) {
        self.host_did = host_did;
        self.bostrom_address = bostrom_address;
    }
}
