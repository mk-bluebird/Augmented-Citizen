// crates/cybercore-brain/src/lib.rs
use ac_aln_core::AlnRow;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

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

#[derive(Clone)]
pub struct CybercoreBrain {
    modules: Arc<Mutex<Vec<RankedModule>>>,
}

impl CybercoreBrain {
    pub fn new() -> Self {
        Self {
            modules: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn ingest_rows(&self, rows: Vec<AlnRow>) {
        let mut modules = self.modules.lock();
        modules.clear();
        for row in rows {
            let rv = self.derive_rank_vector(&row);
            let rs = self.compute_rank_scalar(&rv);
            modules.push(RankedModule {
                row,
                rank_vector: rv,
                rank_scalar: rs,
            });
        }
    }

    fn derive_rank_vector(&self, row: &AlnRow) -> RankVector {
        let safety = if row.compliance.contains("HIPAA") || row.compliance.contains("EUAI") {
            0.9
        } else {
            0.6
        };
        let legal = if row.compliance.contains("GDPR") {
            0.9
        } else {
            0.5
        };
        let biomech = if row.device_type.eq_ignore_ascii_case("BCIEdge") {
            0.9
        } else {
            0.7
        };
        let psych_risk = if row.role.to_ascii_lowercase().contains("game") {
            0.4
        } else {
            0.6
        };
        let eco = if row.edge_analytics.to_ascii_lowercase().contains("snn") {
            0.9
        } else {
            0.6
        };
        RankVector {
            safety,
            legal,
            biomech,
            psych_risk,
            eco,
        }
    }

    fn compute_rank_scalar(&self, v: &RankVector) -> f64 {
        let w_s = 0.3;
        let w_l = 0.2;
        let w_b = 0.2;
        let w_p = -0.1;
        let w_e = 0.4;
        w_s * v.safety
            + w_l * v.legal
            + w_b * v.biomech
            + w_p * v.psych_risk
            + w_e * v.eco
    }

    pub fn best_modules(&self, top_n: usize) -> Vec<RankedModule> {
        let mut modules = self.modules.lock().clone();
        modules.sort_by(|a, b| b.rank_scalar.partial_cmp(&a.rank_scalar).unwrap());
        modules.into_iter().take(top_n).collect()
    }
}
