// crates/xr-telemetry-client/src/lib.rs
#![feature(rust_2024_preview)]
#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

/// ====== COMMON TYPES ======

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub device_id: String,
    pub device_kind: String,        // "smart_glasses", "vr_headset", "ar_headset"
    pub model: String,
    pub firmware_version: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TimeWindow {
    pub t_start_utc: String,       // ISO8601
    pub t_end_utc: String,         // ISO8601
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModeContext {
    pub context: String,           // "medical" | "security" | "mixed" | "other"
    pub xr_role: String,           // e.g. "AD_MONITORING", "XR_REHAB", "SECURITY_OVERLAY"
    pub user_state: String,        // "awake" | "rest" | "sleep" | "unknown"
}

/// ====== TELEMETRY ======

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BandPowers {
    pub delta: f64,
    pub theta: f64,
    pub alpha: f64,
    pub beta: f64,
    pub gamma: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MicrostateFeatures {
    pub entropy: f64,
    pub transition_rate_hz: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConnectivityFeatures {
    pub sigma_sw_est: f64,
    pub global_coherence: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EegBlock {
    pub has_eeg: bool,
    pub sampling_rate_hz: Option<u32>,
    pub band_powers: Option<BandPowers>,
    pub microstate_features: Option<MicrostateFeatures>,
    pub connectivity: Option<ConnectivityFeatures>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TaskPerformance {
    pub reaction_time_ms: Option<u32>,
    pub error_rate: Option<f64>,       // 0–1
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct XrTask {
    pub task_id: String,
    pub task_type: String,            // "cognitive_test" | "navigation" | ...
    pub difficulty_level: f64,        // 0–1
    pub duration_sec: u32,
    pub performance: TaskPerformance,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PhysioBlock {
    pub heart_rate_bpm: Option<u32>,
    pub hrv_norm: Option<f64>,        // 0–1
    pub resp_rate_bpm: Option<u32>,
    pub temp_c: Option<f64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubjectiveBlock {
    pub fatigue_score: Option<f64>,   // 0–1
    pub pain_score: Option<f64>,      // 0–1
    pub stress_score: Option<f64>,    // 0–1
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SafetyProxies {
    pub local_roh_est: Option<f64>,
    pub biomechscore_est: Option<f64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClientMeta {
    pub app_version: String,
    pub network_rtt_ms: Option<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TelemetryMessage {
    pub r#type: String,               // must be "telemetry"
    pub version: String,
    pub hostdid: String,
    pub session_id: String,
    pub device: DeviceInfo,
    pub time_window: TimeWindow,
    pub mode: ModeContext,
    pub eeg: EegBlock,
    pub xr_task: XrTask,
    pub physio: PhysioBlock,
    pub subjective: SubjectiveBlock,
    pub safety_proxies: SafetyProxies,
    pub client_meta: ClientMeta,
}

/// ====== DECISION ======

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DecisionResult {
    pub allowed: bool,
    pub reason: String,               // "ok" | "roh_violation" | "vad_increase" | ...
    pub severity: String,             // "info" | "warning" | "critical"
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdSafetyBlock {
    pub csi: f64,
    pub sigma_sw: f64,
    pub xbar_tau_est: f64,
    pub v_ad_prev: f64,
    pub v_ad_next: f64,
    pub v_ad_max: f64,
    pub roh_step: f64,
    pub roh_max: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContinuityBlock {
    pub continuity_ok: bool,
    pub identity_index: f64,
    pub narrative_index: f64,
    pub agency_index: f64,
    pub v_cont_prev: f64,
    pub v_cont_next: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HealthcareModeBlock {
    pub mode: String,                 // "baseline" | "care_only"
    pub p_care: f64,
    pub p_care_crit: f64,
    pub p_care_exit: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct XrActionsBlock {
    pub action: String,               // "proceed" | "throttle" | "switch_to_care" | "halt_overlay"
    pub recommended_intensity_scale: f64,
    pub ui_hints: Vec<String>,
    pub blocked_features: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuditBlock {
    pub evidence_bundle_id: String,
    pub guard_version: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DecisionMessage {
    pub r#type: String,               // must be "decision"
    pub version: String,
    pub hostdid: String,
    pub session_id: String,
    pub device_id: String,
    pub time_utc: String,
    pub decision_id: String,
    pub result: DecisionResult,
    pub ad_safety: Option<AdSafetyBlock>,
    pub continuity: Option<ContinuityBlock>,
    pub healthcare_mode: Option<HealthcareModeBlock>,
    pub xr_actions: Option<XrActionsBlock>,
    pub audit: Option<AuditBlock>,
}

/// ====== CONFIG ======

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdSafetyConfig {
    pub csi_min: f64,
    pub sigma_min: f64,
    pub xbar_tau_max: f64,
    pub v_ad_max: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RohConfig {
    pub roh_max: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TelemetryConfig {
    pub window_seconds: u32,
    pub max_missed_windows: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct XrPoliciesConfig {
    pub max_overlay_complexity: f64,
    pub allowed_roles_medical: Vec<String>,
    pub allowed_roles_security: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConfigMessage {
    pub r#type: String,               // must be "config"
    pub version: String,
    pub hostdid: String,
    pub device_id: String,
    pub time_utc: String,
    pub ad_safety: AdSafetyConfig,
    pub roh: RohConfig,
    pub telemetry: TelemetryConfig,
    pub xr_policies: XrPoliciesConfig,
}

/// ====== ACK ======

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppliedActionsBlock {
    pub xr_action: String,
    pub intensity_scale_applied: f64,
    pub features_disabled: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AckMessage {
    pub r#type: String,               // must be "ack"
    pub version: String,
    pub hostdid: String,
    pub session_id: String,
    pub device_id: String,
    pub ref_decision_id: String,
    pub applied: bool,
    pub applied_actions: Option<AppliedActionsBlock>,
    pub time_utc: String,
}

/// ====== WRAPPER ENUM ======

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WireMessage {
    #[serde(rename = "telemetry")]
    Telemetry {
        version: String,
        hostdid: String,
        session_id: String,
        device: DeviceInfo,
        time_window: TimeWindow,
        mode: ModeContext,
        eeg: EegBlock,
        xr_task: XrTask,
        physio: PhysioBlock,
        subjective: SubjectiveBlock,
        safety_proxies: SafetyProxies,
        client_meta: ClientMeta,
    },
    #[serde(rename = "decision")]
    Decision(DecisionMessage),
    #[serde(rename = "config")]
    Config(ConfigMessage),
    #[serde(rename = "ack")]
    Ack(AckMessage),
}
