// crates/ac-aln-core/src/lib.rs
use serde::{Deserialize, Serialize};
use std::path::Path;

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

#[derive(thiserror::Error, Debug)]
pub enum AlnError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("CSV error: {0}")]
    Csv(#[from] csv::Error),
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
