// crates/ac-aln-core/src/lib.rs
use serde::{Deserialize, Serialize};
use std::path::Path;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlnRow {
    pub destination_path: String,
    pub module: String,
    pub version: String,
    pub role: String,
    pub security_protocol: String,
    pub interop_standard: String,
    pub identity_mgmt: String,
    pub ai_agent_integration: String,
    pub device_type: String,
    pub authentication: String,
    pub digital_twin: String,
    pub edge_analytics: String,
    pub compliance: String,
    pub log_persistence: String,
}

#[derive(Error, Debug)]
pub enum AlnError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("CSV error: {0}")]
    Csv(#[from] csv::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DidDocument {
    pub id: String,
    #[serde(default)]
    pub controller: Vec<String>,
    #[serde(default)]
    pub verification_method: Vec<VerificationMethod>,
    #[serde(default)]
    pub authentication: Vec<String>,
    #[serde(default)]
    pub service: Vec<Service>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationMethod {
    pub id: String,
    pub r#type: String,
    pub controller: String,
    #[serde(rename = "publicKeyJwk")]
    pub public_key_jwk: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
    pub id: String,
    pub r#type: String,
    #[serde(rename = "serviceEndpoint")]
    pub service_endpoint: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifiableCredential {
    #[serde(rename = "@context")]
    pub context: Vec<serde_json::Value>,
    pub id: Option<String>,
    pub r#type: Vec<String>,
    pub issuer: String,
    pub issuance_date: String,
    pub expiration_date: Option<String>,
    #[serde(rename = "credentialSubject")]
    pub credential_subject: serde_json::Value,
    #[serde(rename = "credentialSchema")]
    pub credential_schema: Option<Vec<CredentialSchemaRef>>,
    pub proof: Option<Proof>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialSchemaRef {
    pub id: String,
    pub r#type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proof {
    pub r#type: String,
    #[serde(rename = "created")]
    pub created: String,
    #[serde(rename = "verificationMethod")]
    pub verification_method: String,
    #[serde(rename = "proofPurpose")]
    pub proof_purpose: String,
    #[serde(rename = "proofValue")]
    pub proof_value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XrMobilityAssistantSubject {
    pub id: Option<String>,
    pub resident_id: String,
    pub jurisdiction: String,
    pub mobility_tier: String,
    pub emergency_consent: bool,
    pub max_assist_speed_kmh: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XrMobilityAssistantCredential {
    #[serde(flatten)]
    pub base: VerifiableCredential,
    #[serde(skip)]
    pub subject: XrMobilityAssistantSubject,
}

#[derive(Debug, Error)]
pub enum VcError {
    #[error("schema missing or invalid")]
    SchemaMissing,
    #[error("credential subject invalid: {0}")]
    SubjectInvalid(String),
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
}

pub fn vc_lint_xr_mobility(vc_json: &str) -> Result<XrMobilityAssistantCredential, VcError> {
    let vc: VerifiableCredential = serde_json::from_str(vc_json)?;
    if vc.credential_schema.is_none() {
        return Err(VcError::SchemaMissing);
    }
    let subject: XrMobilityAssistantSubject =
        serde_json::from_value(vc.credential_subject.clone()).map_err(|e| {
            VcError::SubjectInvalid(format!("parse error: {e}"))
        })?;
    if subject.max_assist_speed_kmh > 40 {
        return Err(VcError::SubjectInvalid(
            "max_assist_speed_kmh must be ≤ 40 for urban XR lanes".to_string(),
        ));
    }
    if !matches!(subject.mobility_tier.as_str(), "wheelchair" | "pedestrian" | "bicycle") {
        return Err(VcError::SubjectInvalid(
            "mobility_tier must be wheelchair|pedestrian|bicycle".to_string(),
        ));
    }
    Ok(XrMobilityAssistantCredential { base: vc, subject })
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlnNodeRow {
    pub destination_path: String,
    pub module: String,
    pub version: String,
    pub role: String,
    #[serde(rename = "security-protocol")]
    pub security_protocol: String,
    #[serde(rename = "interop-standard")]
    pub interop_standard: String,
    #[serde(rename = "identity-mgmt")]
    pub identity_mgmt: String,
    #[serde(rename = "ai-agent-integration")]
    pub ai_agent_integration: String,
    #[serde(rename = "device-type")]
    pub device_type: String,
    pub authentication: String,
    #[serde(rename = "digital-twin")]
    pub digital_twin: String,
    #[serde(rename = "edge-analytics")]
    pub edge_analytics: String,
    pub compliance: String,
    #[serde(rename = "log-persistence")]
    pub log_persistence: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeRole {
    CitizenAgent,
    Drone,
    CityService,
    BciGateway,
    AuditSink,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeIdentityConfig {
    pub node_id: Uuid,
    pub role: NodeRole,
    pub did: String,
    pub required_vc_types: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LedgerTarget {
    OrganichainBostrom,
    IotaIdentity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditAnchorConfig {
    pub ledger: LedgerTarget,
    pub endpoint: String,
    pub chain_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlnNodeConfig {
    pub node: NodeIdentityConfig,
    pub aln_row: AlnNodeRow,
    pub audit_anchor: AuditAnchorConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KubePodSpec {
    pub name: String,
    pub image: String,
    pub cpu_milli: u32,
    pub memory_mb: u32,
    pub allow_privileged: bool,
    pub host_network: bool,
    pub compliance_tags: Vec<String>,
}

impl KubePodSpec {
    pub fn from_aln_row(row: &AlnNodeRow) -> Self {
        let base_name = row
            .module
            .to_lowercase()
            .replace('.', "-")
            .replace('_', "-");
        let image = format!("ghcr.io/mk-bluebird/{}:{}", base_name, row.version);
        let (cpu_milli, memory_mb) = match row.role.as_str() {
            "QuantumKernel" => (2000, 4096),
            "BiosensorOrch" => (1000, 2048),
            "SpatialRuntime" => (1500, 3072),
            "GovernanceHub" | "SupervisoryGov" => (1000, 2048),
            _ => (500, 1024),
        };
        let allow_privileged = false;
        let host_network = row.role == "IoTGuardian";
        let compliance_tags = row
            .compliance
            .split(|c| c == ',' || c == ' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();
        KubePodSpec {
            name: base_name,
            image,
            cpu_milli,
            memory_mb,
            allow_privileged,
            host_network,
            compliance_tags,
        }
    }
}

pub fn sovereignty_score(local_control_pct: f64, vendor_lock_in_pct: f64) -> f64 {
    (local_control_pct * 0.4) - vendor_lock_in_pct
}

pub fn load_aln_csv(path: &Path) -> Result<Vec<AlnRow>, AlnError> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(path)?;
    let mut rows = Vec::new();
    for result in rdr.deserialize::<AlnRow>() {
        rows.push(result?);
    }
    Ok(rows)
}

impl AlnRow {
    pub fn is_bci_edge(&self) -> bool {
        self.device_type.eq_ignore_ascii_case("BCIEdge")
    }

    pub fn is_xr_edge(&self) -> bool {
        self.device_type.eq_ignore_ascii_case("XREdge")
    }

    pub fn is_neuromorphic_node(&self) -> bool {
        self.device_type.eq_ignore_ascii_case("NeuromorphicNode")
    }
}
