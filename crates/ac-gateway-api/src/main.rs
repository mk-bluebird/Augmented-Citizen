// crates/ac-gateway-api/src/main.rs
use ac_aln_core::load_aln_csv;
use axum::{routing::get, Router};
use cybercore_brain::CybercoreBrain;
use std::{net::SocketAddr, path::PathBuf};
use tracing::info;
use tracing_subscriber::EnvFilter;
use vitalnet_plane::VitalNetConfig;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");

    let vital_cfg = VitalNetConfig::load_from_repo(&repo_root)?;
    let shard_path = repo_root.join("qpudatashards/qpudatashardshybridstack.aln.csv");
    let rows = load_aln_csv(&shard_path)?;
    let cybercore = CybercoreBrain::new();
    cybercore.ingest_rows(rows);

    let app = Router::new().route(
        "/health",
        get(|| async { "ac-gateway-api: ok" }),
    );

    let addr: SocketAddr = "0.0.0.0:8080".parse().unwrap();
    info!(?addr, "starting ac-gateway-api");
    axum::Server::bind(&addr).serve(app.into_make_service()).await?;
    let _ = vital_cfg;

    Ok(())
}
