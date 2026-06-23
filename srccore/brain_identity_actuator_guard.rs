// srccore/brain_identity_actuator_guard.rs

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BrainIdentityState {
    ActiveStable,
    ActiveElevatedRisk,
    Suspended,
    Revoked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MissionMode {
    DiagnosticOnly,
    Therapeutic,
}

#[derive(Debug, Clone)]
pub struct ActuatorPolicy {
    pub mode: MissionMode,
    pub roh_ceiling: f32,
    pub requires_stable_state: bool,
    pub allowed_in_elevated: bool,
}

pub fn can_invoke_actuator(
    state: BrainIdentityState,
    roh: f32,
    policy: &ActuatorPolicy,
    kernel_roh_ceiling: f32,
) -> bool {
    if roh > kernel_roh_ceiling || roh > policy.roh_ceiling {
        return false;
    }

    match state {
        BrainIdentityState::ActiveStable => true,
        BrainIdentityState::ActiveElevatedRisk => {
            // No therapeutic actuation in elevated risk.
            if matches!(policy.mode, MissionMode::Therapeutic) {
                return false;
            }
            policy.allowed_in_elevated
        }
        BrainIdentityState::Suspended | BrainIdentityState::Revoked => false,
    }
}
