// destination-path: cybercore-psych-guards/src/aln_manifest_integration.rs

#![forbid(unsafe_code)]

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct NeuroZkCommitments {
    pub commit_psych_risk:        Option<String>,
    pub commit_slavery_risk:      Option<String>,
    pub commit_job_class:         Option<String>,
    pub commit_jurisdiction:      Option<String>,

    /// Opaque commitment over active ALN shard IDs+versions at decision time.
    /// The underlying scheme is host-defined; this crate treats it as an inert blob.
    pub commit_aln_shard_set:     Option<String>,
}
