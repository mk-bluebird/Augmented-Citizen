// crates/vitalnet-plane/src/sensor_gating.rs
// Design: D High, NR Medium, EE Medium
use serde::{Deserialize, Serialize};

/// Hardware ROI mask configuration sent into ISP / secure element.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoiMask {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub opaque_outside: bool,
}

/// Abstract hardware trait; one impl binds to secure ISP/TEE.
pub trait SensorGatingHardware {
    fn apply_roi_mask(&self, mask: &RoiMask) -> Result<(), String>;
    fn clear_roi_mask(&self) -> Result<(), String>;
}

/// High-level gating state including a hardware-bound window.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SensorGatingState {
    Blocked,
    Released {
        roi: RoiMask,
        expires_at_ms: u64,
    },
}

/// Enforce that Released state always corresponds to a hardware ROI mask
/// tightly bound to the consented subject, preventing pivot to bystanders.
pub fn enter_released_with_roi<H: SensorGatingHardware>(
    hw: &H,
    subject_bbox: (u16, u16, u16, u16),
    now_ms: u64,
    window_ms: u64,
) -> Result<SensorGatingState, String> {
    let (x, y, w, h) = subject_bbox;
    let mask = RoiMask {
        x,
        y,
        width: w,
        height: h,
        opaque_outside: true,
    };
    hw.apply_roi_mask(&mask)?;
    Ok(SensorGatingState::Released {
        roi: mask,
        expires_at_ms: now_ms + window_ms,
    })
}

/// Periodic enforcement: once the gating window expires, mask is cleared and state returns Blocked.
pub fn enforce_gating_window<H: SensorGatingHardware>(
    hw: &H,
    state: &SensorGatingState,
    now_ms: u64,
) -> Result<SensorGatingState, String> {
    match state {
        SensorGatingState::Blocked => Ok(SensorGatingState::Blocked),
        SensorGatingState::Released { roi: _, expires_at_ms } => {
            if now_ms >= *expires_at_ms {
                hw.clear_roi_mask()?;
                Ok(SensorGatingState::Blocked)
            } else {
                Ok(state.clone())
            }
        }
    }
}
