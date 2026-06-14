// crates/cybercore-brain/src/lib.rs

use ac_aln_core::AlnRow;
use parking_lot::Mutex;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use thiserror::Error;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankVector {
    pub safety: f64,
    pub legal: f64,
    pub biomech: f64,
    pub psych_risk: f64,
    pub eco: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankedModule {
    pub row: AlnRow,
    pub rank_vector: RankVector,
    pub rank_scalar: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CyberMetrics {
    pub safety: f64,
    pub legal: f64,
    pub bio: f64,
    pub psych: f64,
    pub eco: f64,
    pub sovereignty: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CyberRankWeights {
    pub w_safety: f64,
    pub w_legal: f64,
    pub w_bio: f64,
    pub w_psych: f64,
    pub w_eco: f64,
    pub w_sovereignty: f64,
}

impl CyberRankWeights {
    pub fn neurorights_focused() -> Self {
        CyberRankWeights {
            w_safety: 0.30,
            w_legal: 0.25,
            w_bio: 0.20,
            w_psych: 0.10,
            w_eco: 0.10,
            w_sovereignty: 0.05,
        }
    }
}

pub fn compute_cyber_rank(weights: &CyberRankWeights, metrics: &CyberMetrics) -> f64 {
    (weights.w_safety * metrics.safety)
        + (weights.w_legal * metrics.legal)
        + (weights.w_bio * metrics.bio)
        + (weights.w_psych * metrics.psych)
        + (weights.w_eco * metrics.eco)
        + (weights.w_sovereignty * metrics.sovereignty)
}

pub fn derive_metrics_from_row(row: &AlnRow, sovereignty_score: f64) -> CyberMetrics {
    let comp = row.compliance.to_lowercase();

    let safety = if comp.contains("euai") || comp.contains("iso26262") {
        0.9
    } else {
        0.5
    };

    let legal = if comp.contains("gdpr") || comp.contains("hipaa") || comp.contains("ccpa") {
        0.9
    } else {
        0.4
    };

    let bio = if comp.contains("hipaa") || comp.contains("iec62304") {
        0.85
    } else {
        0.3
    };

    let role_lower = row.role.to_lowercase();
    let psych = if role_lower.contains("governance") || role_lower.contains("gov") {
        0.8
    } else {
        0.4
    };

    let eco = if row.edge_analytics.to_lowercase().contains("snn") {
        0.9
    } else {
        0.5
    };

    let sov_norm = ((sovereignty_score + 100.0) / 200.0).clamp(0.0, 1.0);

    CyberMetrics {
        safety,
        legal,
        bio,
        psych,
        eco,
        sovereignty: sov_norm,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditEventType {
    SensorState,
    ZkpProof,
    BciState,
    PolicyDecision,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditRecord {
    pub id: Uuid,
    pub ts_utc: OffsetDateTime,
    pub node_id: Uuid,
    pub event_type: AuditEventType,
    pub payload: serde_json::Value,
    pub hash_hex: String,
}

pub fn init_audit_schema(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute_batch(
        r#"
CREATE TABLE IF NOT EXISTS audit_records (
    id TEXT PRIMARY KEY,
    ts_utc TEXT NOT NULL,
    node_id TEXT NOT NULL,
    event_type TEXT NOT NULL,
    payload_json TEXT NOT NULL,
    hash_hex TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS zkp_proofs (
    id TEXT PRIMARY KEY,
    audit_id TEXT NOT NULL,
    circuit_name TEXT NOT NULL,
    public_inputs_json TEXT NOT NULL,
    proof_blob BLOB NOT NULL,
    FOREIGN KEY(audit_id) REFERENCES audit_records(id)
);

CREATE TABLE IF NOT EXISTS bci_states (
    id TEXT PRIMARY KEY,
    audit_id TEXT NOT NULL,
    mode TEXT NOT NULL,
    safety_rank REAL NOT NULL,
    FOREIGN KEY(audit_id) REFERENCES audit_records(id)
);

CREATE TABLE IF NOT EXISTS log_anchors (
    id TEXT PRIMARY KEY,
    audit_id TEXT NOT NULL,
    ledger TEXT NOT NULL,
    tx_hash TEXT NOT NULL,
    ts_utc TEXT NOT NULL,
    FOREIGN KEY(audit_id) REFERENCES audit_records(id)
);
"#,
    )?;
    Ok(())
}

#[derive(Debug, Error)]
pub enum AuditError {
    #[error("db error: {0}")]
    Db(#[from] rusqlite::Error),
    #[error("serialization error: {0}")]
    Serde(#[from] serde_json::Error),
}

pub fn insert_audit_record(conn: &Connection, record: &AuditRecord) -> Result<(), AuditError> {
    let payload_json = serde_json::to_string(&record.payload)?;
    conn.execute(
        r#"
INSERT INTO audit_records (id, ts_utc, node_id, event_type, payload_json, hash_hex)
VALUES (?1, ?2, ?3, ?4, ?5, ?6)
"#,
        params![
            record.id.to_string(),
            record.ts_utc.to_string(),
            record.node_id.to_string(),
            format!("{:?}", record.event_type),
            payload_json,
            record.hash_hex
        ],
    )?;
    Ok(())
}

pub fn uptime_from_failure_rate(failure_rate: f64) -> f64 {
    (1.0 - failure_rate) * 100.0
}

#[derive(Clone)]
pub struct CybercoreBrain {
    modules: Arc<Mutex<Vec<RankedModule>>>,
    weights: Arc<Mutex<CyberRankWeights>>,
}

impl CybercoreBrain {
    pub fn new() -> Self {
        Self {
            modules: Arc::new(Mutex::new(Vec::new())),
            weights: Arc::new(Mutex::new(CyberRankWeights::neurorights_focused())),
        }
    }

    pub fn with_weights(weights: CyberRankWeights) -> Self {
        Self {
            modules: Arc::new(Mutex::new(Vec::new())),
            weights: Arc::new(Mutex::new(weights)),
        }
    }

    pub fn ingest_rows_with_sovereignty(
        &self,
        rows: Vec<AlnRow>,
        sovereignty_score: f64,
    ) {
        let mut modules = self.modules.lock();
        modules.clear();
        let weights = self.weights.lock().clone();

        for row in rows {
            let metrics = derive_metrics_from_row(&row, sovereignty_score);
            let rank_scalar = compute_cyber_rank(&weights, &metrics);
            let rank_vector = RankVector {
                safety: metrics.safety,
                legal: metrics.legal,
                biomech: metrics.bio,
                psych_risk: metrics.psych,
                eco: metrics.eco,
            };
            modules.push(RankedModule {
                row,
                rank_vector,
                rank_scalar,
            });
        }
    }

    pub fn ingest_rows(&self, rows: Vec<AlnRow>) {
        self.ingest_rows_with_sovereignty(rows, 0.0);
    }

    pub fn update_weights(&self, new_weights: CyberRankWeights) {
        let mut w = self.weights.lock();
        *w = new_weights;
        let modules_snapshot = self.modules.lock().clone();
        let mut modules = self.modules.lock();
        modules.clear();
        for ranked in modules_snapshot {
            let metrics = derive_metrics_from_row(&ranked.row, ranked.rank_vector.biomech);
            let rank_scalar = compute_cyber_rank(&new_weights, &metrics);
            let rank_vector = RankVector {
                safety: metrics.safety,
                legal: metrics.legal,
                biomech: metrics.bio,
                psych_risk: metrics.psych,
                eco: metrics.eco,
            };
            modules.push(RankedModule {
                row: ranked.row.clone(),
                rank_vector,
                rank_scalar,
            });
        }
    }

    pub fn best_modules(&self, top_n: usize) -> Vec<RankedModule> {
        let mut modules = self.modules.lock().clone();
        modules.sort_by(|a, b| b.rank_scalar.partial_cmp(&a.rank_scalar).unwrap());
        modules.into_iter().take(top_n).collect()
    }
}
