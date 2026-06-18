// augmented-citizen-mcp-server/src/context.rs

use anti_coercion_enclave::state_machine::ConsentVerdict;
use brain_identity_kernel::fixed::Fx;

#[derive(Copy, Clone, Debug)]
pub struct SessionContext {
    pub verdict: ConsentVerdict,
    pub combined_intent: Fx,
}

impl SessionContext {
    pub fn new() -> Self {
        SessionContext {
            verdict: ConsentVerdict::Invalid,
            combined_intent: Fx::zero(),
        }
    }

    pub fn update(&mut self, verdict: ConsentVerdict, combined_intent: Fx) {
        self.verdict = verdict;
        self.combined_intent = combined_intent;
    }
}
