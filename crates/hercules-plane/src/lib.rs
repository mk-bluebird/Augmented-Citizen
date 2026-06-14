// crates/hercules-plane/src/lib.rs
use ac_aln_core::AlnRow;

pub fn is_hercules_kernel(row: &AlnRow) -> bool {
    row.module == "HerculesKernel" && row.role == "QuantumKernel"
}
