// SPDX-License-Identifier: MIT OR Apache-2.0
// edition = 2024

#![forbid(unsafe_code)]

/// Minimal safety envelope exported from the neurorights kernel for agent gating.
#[derive(Clone, Debug)]
pub struct SafetyEnvelope {
    /// Risk of Harm, 0.0..=1.0, global ceiling 0.30.
    pub roh: f32,
    /// Anti-coercion score, 0.0..=1.0, higher is better.
    pub anticoercion: f32,
    /// Psych debt, 0.0..=1.0, higher is more cumulative load.
    pub psych_debt: f32,
    /// Neurorights floor already checked by the kernel (catalog flags).
    pub neurorights_ok: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GuardVerdict {
    Allow,
    Deny,
}

impl GuardVerdict {
    pub fn is_allow(&self) -> bool {
        matches!(self, GuardVerdict::Allow)
    }

    pub fn is_deny(&self) -> bool {
        matches!(self, GuardVerdict::Deny)
    }
}

/// Verify whether the current safety envelope satisfies global corridors.
pub fn verify_safety(s: &SafetyEnvelope) -> GuardVerdict {
    if s.roh <= 0.30 && s.anticoercion >= 0.8 && s.psych_debt < 0.7 && s.neurorights_ok {
        GuardVerdict::Allow
    } else {
        GuardVerdict::Deny
    }
}

/// Verify that two identical intents are separated by an acceptable time window.
///
/// `dt_ms` is the time delta in milliseconds between the first and second emission.
pub fn verify_shadowed(dt_ms: u64) -> bool {
    (300..=700).contains(&dt_ms)
}

/// Verify a high-stakes intent using two shadowed emissions and their safety envelopes.
pub fn verify_shadowed_intent(
    first: &SafetyEnvelope,
    second: &SafetyEnvelope,
    dt_ms: u64,
) -> GuardVerdict {
    if !verify_shadowed(dt_ms) {
        return GuardVerdict::Deny;
    }

    match (verify_safety(first), verify_safety(second)) {
        (GuardVerdict::Allow, GuardVerdict::Allow) => GuardVerdict::Allow,
        _ => GuardVerdict::Deny,
    }
}
