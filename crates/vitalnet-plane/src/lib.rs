// crates/vitalnet-plane/src/lib.rs
use ac_aln_core::{load_aln_csv, AlnRow};
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::path::Path;
use thiserror::Error;
use tracing::warn;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VitalNetConfig {
    pub core_rows: Vec<AlnRow>,
    pub sentinel_enabled: bool,
    pub safety_kernel_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ComplianceTag {
    EuAiActHighRisk,
    GdprArt9,
    Hipaa,
    Ieee2410BiometricPrivacy,
    FccPart15,
    TsnIso26262,
    Iso42001,
    EidAs,
    Ccpa,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeviceClass {
    DroneCamera,
    BciImplant,
    BciWearable,
    CityCctv,
    XrHeadset,
    GenericEdge,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyProfile {
    pub device: DeviceClass,
    pub compliance: Vec<ComplianceTag>,
    pub no_passive_biometrics: bool,
    pub human_in_the_loop_required: bool,
    pub biometric_storage_allowed: bool,
}

#[derive(Debug, Error)]
pub enum SentinelError {
    #[error("forbidden pattern detected: {0}")]
    ForbiddenPattern(String),
    #[error("missing compliance tag: {0:?}")]
    MissingCompliance(ComplianceTag),
}

#[derive(Debug, Clone)]
pub struct VitalNetSentinelConfig {
    pub forbidden_keywords: Vec<&'static str>,
    pub forbidden_regexes: Vec<Regex>,
}

lazy_static! {
    static ref DEFAULT_SENTINEL_CONFIG: VitalNetSentinelConfig = VitalNetSentinelConfig {
        forbidden_keywords: vec![
            "death-network",
            "deathnet",
            "neuroforcepattern",
            "ghostinject",
            "passive_facial_recognition",
            "passive_biometric_scan",
            "dncheat",
            "noderootescalate",
            "noderfjam",
        ],
        forbidden_regexes: vec![
            Regex::new("(?i)death[-_ ]network").expect("valid regex"),
            Regex::new("(?i)passive.*biometric").expect("valid regex"),
        ],
    };
}

pub fn sentinel_scan_source(source: &str) -> Result<(), SentinelError> {
    let lower = source.to_ascii_lowercase();
    for kw in &DEFAULT_SENTINEL_CONFIG.forbidden_keywords {
        if lower.contains(kw) {
            return Err(SentinelError::ForbiddenPattern(kw.to_string()));
        }
    }
    for re in &DEFAULT_SENTINEL_CONFIG.forbidden_regexes {
        if re.is_match(&lower) {
            return Err(SentinelError::ForbiddenPattern(re.as_str().to_string()));
        }
    }
    Ok(())
}

pub fn validate_safety_profile(profile: &SafetyProfile) -> Result<(), SentinelError> {
    match profile.device {
        DeviceClass::DroneCamera | DeviceClass::CityCctv => {
            if !profile.no_passive_biometrics {
                return Err(SentinelError::ForbiddenPattern(
                    "passive biometric scanning disabled by policy".to_string(),
                ));
            }
            if !profile.compliance.contains(&ComplianceTag::EuAiActHighRisk) {
                return Err(SentinelError::MissingCompliance(
                    ComplianceTag::EuAiActHighRisk,
                ));
            }
            if !profile.compliance.contains(&ComplianceTag::GdprArt9) {
                return Err(SentinelError::MissingCompliance(ComplianceTag::GdprArt9));
            }
        }
        DeviceClass::BciImplant | DeviceClass::BciWearable => {
            if !profile.compliance.contains(&ComplianceTag::Hipaa) {
                return Err(SentinelError::MissingCompliance(ComplianceTag::Hipaa));
            }
            if !profile.compliance.contains(&ComplianceTag::FccPart15) {
                return Err(SentinelError::MissingCompliance(ComplianceTag::FccPart15));
            }
            if !profile.human_in_the_loop_required {
                return Err(SentinelError::ForbiddenPattern(
                    "BCI requires human-in-the-loop oversight".to_string(),
                ));
            }
        }
        DeviceClass::XrHeadset => {
            if !profile.compliance.contains(&ComplianceTag::TsnIso26262) {
                return Err(SentinelError::MissingCompliance(ComplianceTag::TsnIso26262));
            }
        }
        DeviceClass::GenericEdge => {}
    }
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeAnalyticsConstraint {
    pub no_passive_biometric_scanning: bool,
    pub allowed_modalities: Vec<String>,
    pub max_frame_rate_hz: f64,
}

impl EdgeAnalyticsConstraint {
    pub fn for_drone_camera() -> Self {
        EdgeAnalyticsConstraint {
            no_passive_biometric_scanning: true,
            allowed_modalities: vec![
                "depth".to_string(),
                "optical_flow".to_string(),
                "segmentation".to_string(),
            ],
            max_frame_rate_hz: 15.0,
        }
    }
}

pub fn tsn_latency_ms(distance_ms: f64, speed_factor: f64, processing_ms: f64) -> f64 {
    (distance_ms / speed_factor) + processing_ms
}

impl VitalNetConfig {
    pub fn load_from_repo(root: &Path) -> anyhow::Result<Self> {
        let core_path = root.join("qpudatashards/qpudatashardshybridstack.aln.csv");
        let rows = load_aln_csv(&core_path)?;
        Ok(Self {
            core_rows: rows,
            sentinel_enabled: true,
            safety_kernel_enabled: true,
        })
    }

    pub fn validate_no_deathnet_symbols(&self, source_text: &str) -> bool {
        match sentinel_scan_source(source_text) {
            Ok(_) => true,
            Err(err) => {
                warn!("VitalNet Sentinel blocked source: {err}");
                false
            }
        }
    }
}
