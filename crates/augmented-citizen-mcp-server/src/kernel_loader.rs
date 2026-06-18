// augmented-citizen-mcp-server/src/kernel_loader.rs

#![forbid(unsafe_code)]

use std::fs;
use std::path::Path;

use brain_identity_kernel::kernel::LoadedKernel;
use brain_identity_kernel::shard::ShardError;

const HOST_DID: &str = "bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7";

pub fn load_viability_kernel_from_aln<P: AsRef<Path>>(path: P) -> Result<LoadedKernel, ShardError> {
    let data = fs::read(path).map_err(|_| ShardError::InvalidEncoding)?;
    LoadedKernel::from_aln_bytes(&data, HOST_DID)
}
