// crates/reality-os/src/lib.rs
use ac_aln_core::AlnRow;

pub fn is_spatial_runtime(row: &AlnRow) -> bool {
    row.module == "RealityOSScene" && row.role == "SpatialRuntime"
}

pub fn latency_target_ms() -> f64 {
    1.0
}
