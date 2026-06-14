// crates/cyberorganic-os/src/lib.rs
use ac_aln_core::AlnRow;

pub fn is_bci_lane(row: &AlnRow) -> bool {
    row.is_bci_edge()
        && row.role.to_ascii_lowercase().contains("biosensor")
        && row.security_protocol.to_ascii_lowercase().contains("aes")
}

pub fn requires_hipaa(row: &AlnRow) -> bool {
    row.is_bci_edge() && row.compliance.contains("HIPAA")
}
