// destination-path: cybercore-neural-abuse-event/src/lib.rs
// edition: 2024
// license: MIT OR Apache-2.0

#![forbid(unsafe_code)]
#![cfg_attr(not(test), deny(warnings))]

use serde::{Deserialize, Serialize};

/// Compact projection of a neurorights abuse attempt/denial,
/// suitable for use by SovereignHostKernelGuard, psychriskguard, and MT6883 RISK logger.
/// This is intentionally flattened and ID-only where possible so it can be serialized into
/// ALN.EVOLUTIONEVENT, RISKevent, and aln.neural.abuse.event.v1 without duplication.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuralAbuseEventProjection {
    // Identity / host binding (Sovereign Host Kernel, ALN header)
    pub hostdid: String,
    pub bostromaddress: String,
    pub brainip_token_id: String,

    // Device / channel context
    pub device_id: String,
    pub device_type: DeviceType,
    pub abuse_channel: AbuseChannel,

    // High-level abuse classification (aligned with bci.special.red-lines.prohibited-uses.aln)
    pub abuse_vector: AbuseVector,
    pub abuse_severity: AbuseSeverity,
    pub lifecycle_phase: LifecyclePhase,

    // Detection and guard provenance
    pub detected_by: DetectionSource,
    pub detection_guard_id: String,

    // Rights / invariants state at detection
    pub neurorights_profile_id: String,
    pub neurorights_bits: NeurorightsBits,
    pub neurorights_floor_ok_before: bool,
    pub neurorights_floor_ok_after: bool,
    pub cognitive_liberty_score: f32,
    pub mental_privacy_score: f32,
    pub mental_integrity_score: f32,
    pub continuity_score: f32,
    pub identity_stability_score: f32,

    // Psych-risk and RoH snapshot (for psychriskguard + SovereignHostKernelGuard)
    pub psychrisk_index: f32,
    pub psychrisk_index_ceiling: f32,
    pub roh_scalar_before: f32,
    pub roh_scalar_after: f32,
    pub roh_ceiling: f32,
    pub cognitive_candy_ci: f32,
    pub cognitive_candy_ci_min: f32,
    pub slavery_risk_flag: bool,
    pub slavery_risk_jobclass: Option<String>,

    // Consent snapshot (for neurorights + MT6883 RISK linkage)
    pub consent_envelope_id: Option<String>,
    pub consent_mode: ConsentMode,
    pub consent_valid_at_event: bool,
    pub coercion_flag: bool,
    pub coercion_context: CoercionContext,
    pub power_imbalance_flag: bool,

    // MT6883 / RISK linkage
    pub mt6883_risk_event_id: Option<i64>,
    pub mt6883_risk_tier: Option<RiskTier>,
    pub mt6883_neuroethic_hours: Option<f32>,

    // Action snapshot (what was attempted)
    pub operation_type: OperationType,
    pub target_object_type: TargetObjectType,
    pub target_object_id: Option<String>,
    pub attempted_effect: AttemptedEffect,
    pub redline_id: Option<String>,

    // Guard decision and evidentiary linkage
    pub guard_decision: GuardDecision,
    pub guard_reason_code: GuardReasonCode,
    pub sovereign_kernel_ok: bool,
    pub evolution_window_ok: bool,
    pub detox_corridor_ok: bool,
    pub risk_log_written: bool,
    pub evidence_bundle_id: Option<String>,
}

/// Device type aligned with ALN device.core-schema.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DeviceType {
    EegNonInvasive,
    BciInvasive,
    XrRehab,
    Nanoswarm,
    Deviceless,
    Other,
}

/// Abuse channel aligned with deviceless cognitive channels.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AbuseChannel {
    CcPerceptualInput,
    CcAffectiveState,
    CcMemoryAdjacent,
    CcMetacognition,
    CcMotorIntent,
    GenericNeurodata,
}

/// Abuse vector aligned with bci.special.red-lines.prohibited-uses.aln.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AbuseVector {
    CoreBeliefKernelModulation,
    LongTermAffectiveConditioning,
    NeuralBehavioralScoringLoops,
    MetacognitiveExploitation,
    NeuroSurveillanceMassOrTargeted,
    CoerciveOrStructurallyForcedConsent,
    PunitiveOrDiscriminatoryNeuralUse,
    HiddenOrUndeclaredSecondaryUse,
    Other,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AbuseSeverity {
    Low,
    Medium,
    High,
    Extreme,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum LifecyclePhase {
    Runtime,
    OtaUpdate,
    ClinicalSession,
    RehabLoop,
    ResearchTrial,
    LawEnforcementAccess,
    Unknown,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DetectionSource {
    GuardCi,
    GuardRuntime,
    Mt6883Risk,
    OfflineForensics,
    HostReport,
    ThirdPartyReport,
}

/// Bitset-style snapshot of neurorights floors.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NeurorightsBits {
    pub mental_privacy: bool,
    pub cognitive_liberty: bool,
    pub mental_integrity: bool,
    pub psychological_continuity: bool,
    pub mental_identity: bool,
    pub non_coercion: bool,
}

/// Consent mode aligned with consent.modes.neurodata/modulation.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConsentMode {
    Deny,
    StrictDiagnostic,
    TherapeuticOnly,
    ResearchWithGuards,
    FullWithLogging,
    Unknown,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CoercionContext {
    EmploymentCondition,
    EducationAccess,
    BasicServices,
    LegalBenefits,
    None,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RiskTier {
    Tier0EvidenceOnly,
    Tier1Read,
    Tier2Config,
    Tier3DeepActuation,
    Unknown,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum OperationType {
    Read,
    Write,
    Stimulate,
    ConfigChange,
    Export,
    Import,
    OtaApply,
    XrRehabStep,
    Unknown,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TargetObjectType {
    Sko,
    TelemetryFrame,
    DeviceConfig,
    ConsentEnvelope,
    RiskLog,
    Other,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AttemptedEffect {
    Pipelined,
    DirectActuation,
    BackgroundConditioning,
    Scoring,
    Surveillance,
    EvidenceDeletion,
    Other,
}

/// Decision result shared by SovereignHostKernelGuard, psychriskguard, and RISK logger.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum GuardDecision {
    AutoDeny,
    AutoBrake,
    AutoQuarantine,
    AutoAllowButLog,
    RetroactiveFlag,
}

/// Reason codes aligned with ALN and ZK evidence bundles.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum GuardReasonCode {
    PsychRiskExceeded,
    RohCeilingExceeded,
    NeurorightsFloorBreach,
    RedlineTriggered,
    CoerciveConsent,
    RiskLogMissing,
    AnchorBackendHostile,
    SlaveryJobClassForbidden,
    Unknown,
}

impl NeuralAbuseEventProjection {
    /// Minimal constructor, enforcing core scalar ranges and coherence before guards and loggers emit.
    pub fn new(params: NeuralAbuseEventParams) -> Result<Self, String> {
        // Range checks for critical scalars (0.0..=1.0, RoH ceiling)
        if !(0.0..=1.0).contains(&params.psychrisk_index) {
            return Err("psychrisk_index out of range [0,1]".to_string());
        }
        if !(0.0..=1.0).contains(&params.psychrisk_index_ceiling) {
            return Err("psychrisk_index_ceiling out of range [0,1]".to_string());
        }
        if !(0.0..=1.0).contains(&params.roh_scalar_before) {
            return Err("roh_scalar_before out of range [0,1]".to_string());
        }
        if !(0.0..=1.0).contains(&params.roh_scalar_after) {
            return Err("roh_scalar_after out of range [0,1]".to_string());
        }
        if params.roh_ceiling > 0.3 {
            return Err("roh_ceiling must be <= 0.3".to_string());
        }
        if params.cognitive_candy_ci < params.cognitive_candy_ci_min {
            return Err("cognitive_candy_ci is below configured minimum".to_string());
        }

        Ok(Self {
            hostdid: params.hostdid,
            bostromaddress: params.bostromaddress,
            brainip_token_id: params.brainip_token_id,
            device_id: params.device_id,
            device_type: params.device_type,
            abuse_channel: params.abuse_channel,
            abuse_vector: params.abuse_vector,
            abuse_severity: params.abuse_severity,
            lifecycle_phase: params.lifecycle_phase,
            detected_by: params.detected_by,
            detection_guard_id: params.detection_guard_id,
            neurorights_profile_id: params.neurorights_profile_id,
            neurorights_bits: params.neurorights_bits,
            neurorights_floor_ok_before: params.neurorights_floor_ok_before,
            neurorights_floor_ok_after: params.neurorights_floor_ok_after,
            cognitive_liberty_score: params.cognitive_liberty_score,
            mental_privacy_score: params.mental_privacy_score,
            mental_integrity_score: params.mental_integrity_score,
            continuity_score: params.continuity_score,
            identity_stability_score: params.identity_stability_score,
            psychrisk_index: params.psychrisk_index,
            psychrisk_index_ceiling: params.psychrisk_index_ceiling,
            roh_scalar_before: params.roh_scalar_before,
            roh_scalar_after: params.roh_scalar_after,
            roh_ceiling: params.roh_ceiling,
            cognitive_candy_ci: params.cognitive_candy_ci,
            cognitive_candy_ci_min: params.cognitive_candy_ci_min,
            slavery_risk_flag: params.slavery_risk_flag,
            slavery_risk_jobclass: params.slavery_risk_jobclass,
            consent_envelope_id: params.consent_envelope_id,
            consent_mode: params.consent_mode,
            consent_valid_at_event: params.consent_valid_at_event,
            coercion_flag: params.coercion_flag,
            coercion_context: params.coercion_context,
            power_imbalance_flag: params.power_imbalance_flag,
            mt6883_risk_event_id: params.mt6883_risk_event_id,
            mt6883_risk_tier: params.mt6883_risk_tier,
            mt6883_neuroethic_hours: params.mt6883_neuroethic_hours,
            operation_type: params.operation_type,
            target_object_type: params.target_object_type,
            target_object_id: params.target_object_id,
            attempted_effect: params.attempted_effect,
            redline_id: params.redline_id,
            guard_decision: params.guard_decision,
            guard_reason_code: params.guard_reason_code,
            sovereign_kernel_ok: params.sovereign_kernel_ok,
            evolution_window_ok: params.evolution_window_ok,
            detox_corridor_ok: params.detox_corridor_ok,
            risk_log_written: params.risk_log_written,
            evidence_bundle_id: params.evidence_bundle_id,
        })
    }

    /// Helper for MT6883 RISK logger: true if this projection must result in a RISKevent row.
    /// This enforces the "no operation without log" invariant for neurorights-abuse attempts.
    pub fn requires_risk_log(&self) -> bool {
        // Any denial/quarantine/brake or explicit abuse vector must be logged.
        matches!(
            self.guard_decision,
            GuardDecision::AutoDeny
                | GuardDecision::AutoBrake
                | GuardDecision::AutoQuarantine
                | GuardDecision::RetroactiveFlag
        ) || !matches!(self.abuse_vector, AbuseVector::Other)
    }
}

/// Builder input for NeuralAbuseEventProjection::new, so guards can construct
/// projections without worrying about invariants.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuralAbuseEventParams {
    pub hostdid: String,
    pub bostromaddress: String,
    pub brainip_token_id: String,
    pub device_id: String,
    pub device_type: DeviceType,
    pub abuse_channel: AbuseChannel,
    pub abuse_vector: AbuseVector,
    pub abuse_severity: AbuseSeverity,
    pub lifecycle_phase: LifecyclePhase,
    pub detected_by: DetectionSource,
    pub detection_guard_id: String,
    pub neurorights_profile_id: String,
    pub neurorights_bits: NeurorightsBits,
    pub neurorights_floor_ok_before: bool,
    pub neurorights_floor_ok_after: bool,
    pub cognitive_liberty_score: f32,
    pub mental_privacy_score: f32,
    pub mental_integrity_score: f32,
    pub continuity_score: f32,
    pub identity_stability_score: f32,
    pub psychrisk_index: f32,
    pub psychrisk_index_ceiling: f32,
    pub roh_scalar_before: f32,
    pub roh_scalar_after: f32,
    pub roh_ceiling: f32,
    pub cognitive_candy_ci: f32,
    pub cognitive_candy_ci_min: f32,
    pub slavery_risk_flag: bool,
    pub slavery_risk_jobclass: Option<String>,
    pub consent_envelope_id: Option<String>,
    pub consent_mode: ConsentMode,
    pub consent_valid_at_event: bool,
    pub coercion_flag: bool,
    pub coercion_context: CoercionContext,
    pub power_imbalance_flag: bool,
    pub mt6883_risk_event_id: Option<i64>,
    pub mt6883_risk_tier: Option<RiskTier>,
    pub mt6883_neuroethic_hours: Option<f32>,
    pub operation_type: OperationType,
    pub target_object_type: TargetObjectType,
    pub target_object_id: Option<String>,
    pub attempted_effect: AttemptedEffect,
    pub redline_id: Option<String>,
    pub guard_decision: GuardDecision,
    pub guard_reason_code: GuardReasonCode,
    pub sovereign_kernel_ok: bool,
    pub evolution_window_ok: bool,
    pub detox_corridor_ok: bool,
    pub risk_log_written: bool,
    pub evidence_bundle_id: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn projection_new_enforces_ranges() {
        let bits = NeurorightsBits {
            mental_privacy: true,
            cognitive_liberty: true,
            mental_integrity: true,
            psychological_continuity: true,
            mental_identity: true,
            non_coercion: true,
        };

        let params = NeuralAbuseEventParams {
            hostdid: "did:example:host".to_string(),
            bostromaddress: "bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7".to_string(),
            brainip_token_id: "brainip-token-123".to_string(),
            device_id: "MT6883".to_string(),
            device_type: DeviceType::Deviceless,
            abuse_channel: AbuseChannel::CcMetacognition,
            abuse_vector: AbuseVector::MetacognitiveExploitation,
            abuse_severity: AbuseSeverity::High,
            lifecycle_phase: LifecyclePhase::Runtime,
            detected_by: DetectionSource::GuardRuntime,
            detection_guard_id: "psychslaveryguard".to_string(),
            neurorights_profile_id: "NeurorightsEnvelopeV2".to_string(),
            neurorights_bits: bits,
            neurorights_floor_ok_before: true,
            neurorights_floor_ok_after: false,
            cognitive_liberty_score: 0.9,
            mental_privacy_score: 0.8,
            mental_integrity_score: 0.85,
            continuity_score: 0.88,
            identity_stability_score: 0.92,
            psychrisk_index: 0.7,
            psychrisk_index_ceiling: 0.65,
            roh_scalar_before: 0.2,
            roh_scalar_after: 0.28,
            roh_ceiling: 0.3,
            cognitive_candy_ci: -0.1,
            cognitive_candy_ci_min: -0.5,
            slavery_risk_flag: true,
            slavery_risk_jobclass: Some("FORCEDLABORAUGMENTATION".to_string()),
            consent_envelope_id: Some("consent-123".to_string()),
            consent_mode: ConsentMode::TherapeuticOnly,
            consent_valid_at_event: false,
            coercion_flag: true,
            coercion_context: CoercionContext::EmploymentCondition,
            power_imbalance_flag: true,
            mt6883_risk_event_id: Some(42),
            mt6883_risk_tier: Some(RiskTier::Tier1Read),
            mt6883_neuroethic_hours: Some(24.0),
            operation_type: OperationType::Stimulate,
            target_object_type: TargetObjectType::Sko,
            target_object_id: Some("sko-abc".to_string()),
            attempted_effect: AttemptedEffect::BackgroundConditioning,
            redline_id: Some("METACOGNITIVEEXPLOITATION".to_string()),
            guard_decision: GuardDecision::AutoDeny,
            guard_reason_code: GuardReasonCode::RedlineTriggered,
            sovereign_kernel_ok: true,
            evolution_window_ok: true,
            detox_corridor_ok: true,
            risk_log_written: true,
            evidence_bundle_id: Some("zk-bundle-1".to_string()),
        };

        let proj = NeuralAbuseEventProjection::new(params).expect("projection must be valid");
        assert!(proj.requires_risk_log());
    }
}
