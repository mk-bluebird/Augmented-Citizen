use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerMetadata {
    pub name: String,
    pub version: String,
    pub authority: String,
    pub aln_clause: String,
    pub invariants: Vec<String>,
    pub features: ServerFeatures,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerFeatures {
    pub tools: Vec<String>,
    pub resources: Vec<String>,
    pub prompts: Vec<String>,
}
