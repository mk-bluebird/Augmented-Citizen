// srcrf_brain_actuation_guard.rs

use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy)]
pub enum BiomarkerBand {
    Safe,
    Warn,
    Critical,
}

#[derive(Debug, Clone)]
pub struct CardioInflammState {
    pub cardiac_strain_index: f32, // 0.0-1.0 normalized
    pub il6_band: BiomarkerBand,   // derived from ClinicalBiomarkerCorridors
}

#[derive(Debug, Clone)]
pub struct BrainRfActuationToken {
    // Identity binding (enclave-only handle, !Send/!Sync in real code)
    pub session_id: [u8; 16],
    pub host_did: String,
    pub channel_id: u8,

    // Time bounds
    pub issued_at: Instant,
    pub base_ttl: Duration,
    pub current_ttl: Duration,

    // Monotone safety flags
    pub revoked: bool,
}

impl BrainRfActuationToken {
    pub fn new(session_id: [u8; 16], host_did: String, channel_id: u8, base_ttl: Duration) -> Self {
        Self {
            session_id,
            host_did,
            channel_id,
            issued_at: Instant::now(),
            base_ttl,
            current_ttl: base_ttl,
            revoked: false,
        }
    }

    pub fn remaining(&self) -> Duration {
        if self.revoked {
            return Duration::from_millis(0);
        }
        let elapsed = self.issued_at.elapsed();
        if elapsed >= self.current_ttl {
            Duration::from_millis(0)
        } else {
            self.current_ttl - elapsed
        }
    }

    // Monotone shortening based on cardiac strain + IL-6 band.
    // Invariant: current_ttl is never increased, only decreased.
    pub fn tighten_ttl(&mut self, state: &CardioInflammState) {
        if self.revoked {
            return;
        }

        // Factor from cardiac strain index (0.0-1.0)
        // Higher strain => smaller factor <= 1.0
        let strain_factor = if state.cardiac_strain_index <= 0.3 {
            1.0
        } else if state.cardiac_strain_index <= 0.6 {
            0.5
        } else {
            0.25
        };

        // Factor from IL-6 band
        let il6_factor = match state.il6_band {
            BiomarkerBand::Safe => 1.0,
            BiomarkerBand::Warn => 0.5,
            BiomarkerBand::Critical => 0.1,
        };

        // Combined tightening factor, clamped to (0,1]
        let factor = (strain_factor * il6_factor).clamp(0.1, 1.0);

        // Proposed new TTL relative to base TTL
        let proposed = self
            .base_ttl
            .mul_f32(factor);

        // Monotone: never increase TTL
        if proposed < self.current_ttl {
            self.current_ttl = proposed;
        }

        // Auto-revoke if TTL collapses below a minimal floor
        let min_floor = Duration::from_secs(5);
        if self.current_ttl < min_floor {
            self.revoked = true;
        }
    }

    pub fn is_usable(&self) -> bool {
        !self.revoked && self.remaining() > Duration::from_millis(0)
    }
}
