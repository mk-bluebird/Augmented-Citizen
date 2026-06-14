// crates/cyberorganic-os/src/passive_consent_beacon.rs
// Design: D High, NR High, EE Medium
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

/// Thalamic attention metric decoded on-device from spikes; 0.0–1.0.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct AttentionMetric(pub f32);

/// Implant-side policy thresholds for emergency consent beacon.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassiveBeaconPolicy {
    pub emergency_threshold: f32,   // e.g. 0.85
    pub cooldown_seconds: u32,     // min interval between beacons
}

/// Internal state of the beacon logic.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassiveBeaconState {
    pub last_emitted: Option<OffsetDateTime>,
    pub policy: PassiveBeaconPolicy,
}

/// Abstract interface to ZKP generator inside the implant.
/// It consumes only derived state (attention, mode flags), never raw EEG.
pub trait ZkpEngine {
    fn generate_emergency_consent_proof(
        &self,
        attention: AttentionMetric,
    ) -> Result<Vec<u8>, String>;
}

/// Decision function: emit ProofOfEmergencyConsent when attention crosses threshold.
pub fn maybe_emit_emergency_beacon<E: ZkpEngine>(
    engine: &E,
    state: &mut PassiveBeaconState,
    attention: AttentionMetric,
    now: OffsetDateTime,
) -> Result<Option<(Uuid, Vec<u8>)>, String> {
    if attention.0 < state.policy.emergency_threshold {
        return Ok(None);
    }

    if let Some(last) = state.last_emitted {
        let elapsed = now - last;
        if elapsed.whole_seconds() < state.policy.cooldown_seconds as i64 {
            return Ok(None);
        }
    }

    let proof = engine.generate_emergency_consent_proof(attention)?;
    let beacon_id = Uuid::new_v4();
    state.last_emitted = Some(now);

    Ok(Some((beacon_id, proof)))
}
