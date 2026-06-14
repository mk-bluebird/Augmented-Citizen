// crates/vitalnet-plane/src/lib.rs
use ac_aln_core::{load_aln_csv, AlnRow};
use serde::{Deserialize, Serialize};
use std::path::Path;
use tracing::warn;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VitalNetConfig {
    pub core_rows: Vec<AlnRow>,
    pub sentinel_enabled: bool,
    pub safety_kernel_enabled: bool,
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
        let lower = source_text.to_ascii_lowercase();
        let forbidden = [
            "death-network",
            "deathnet",
            "dncheat",
            "neuroforcepattern",
            "noderootescalate",
            "noderfjam",
        ];
        let mut clean = true;
        for token in forbidden {
            if lower.contains(token) {
                warn!("VitalNet Sentinel: blocked forbidden symbol: {token}");
                clean = false;
            }
        }
        clean
    }
}
