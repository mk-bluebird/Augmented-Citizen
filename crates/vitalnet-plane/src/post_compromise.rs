// crates/vitalnet-plane/src/post_compromise.rs
// Design: D Medium, NR Low, EE High
use ac_aln_core::{AlnNodeConfig, AuditAnchorConfig, LedgerTarget};
use cybercore_brain::{AuditEventType, AuditRecord};
use rusqlite::Connection;
use time::OffsetDateTime;
use uuid::Uuid;

/// Key material tracked by the Rust kernel for audit signing.
#[derive(Debug, Clone)]
pub struct DroneKeyState {
    pub drone_id: Uuid,
    pub current_key_id: Uuid,
    pub current_pubkey_der: Vec<u8>,
    pub organichain_anchor: AuditAnchorConfig,
    pub compromised_at: Option<OffsetDateTime>,
}

#[derive(Debug, thiserror::Error)]
pub enum PcsError {
    #[error("database error: {0}")]
    Db(#[from] rusqlite::Error),
    #[error("anchor error: {0}")]
    Anchor(String),
}

/// State-machine transition for post-compromise security.
/// 1. Mark key as compromised in local SQLite.
/// 2. Emit a `KeyCompromised` record anchored on Organichain/Bostrom.
/// 3. Rotate to a fresh key and emit `KeyRotated`.
/// 4. Keep old logs valid by linking via `previous_key_id` and `organichain_anchor`.
pub fn handle_post_compromise(
    conn: &Connection,
    aln: &AlnNodeConfig,
    mut keystate: DroneKeyState,
    anchor_tx_fn: &dyn Fn(&AuditAnchorConfig, &AuditRecord) -> Result<String, String>,
) -> Result<DroneKeyState, PcsError> {
    let now = OffsetDateTime::now_utc();

    // Step 1: mark compromised in local state
    keystate.compromised_at = Some(now);

    // Step 2: log & anchor KeyCompromised event
    let compromised_record = AuditRecord {
        id: Uuid::new_v4(),
        ts_utc: now,
        node_id: aln.node.node_id,
        event_type: AuditEventType::PolicyDecision,
        payload: serde_json::json!({
            "kind": "KeyCompromised",
            "drone_id": keystate.drone_id,
            "key_id": keystate.current_key_id,
            "reason": "edge key exfil suspected",
            "organichain_anchor": aln.audit_anchor,
        }),
        hash_hex: "deadbeefcafebabe0011223344556677".into(), // replace with real hash fn
    };
    cybercore_brain::insert_audit_record(conn, &compromised_record)?;
    let txhash = anchor_tx_fn(&keystate.organichain_anchor, &compromised_record)
        .map_err(PcsError::Anchor)?;

    cybercore_brain::insert_log_anchor(
        conn,
        &compromised_record.id,
        &keystate.organichain_anchor,
        &txhash,
    )?;

    // Step 3: rotate signing key (done in HSM/TEE in practice)
    let new_key_id = Uuid::new_v4();
    let new_pubkey_der = generate_fresh_pubkey(); // hardware-backed, not shown here

    let rotated_record = AuditRecord {
        id: Uuid::new_v4(),
        ts_utc: now,
        node_id: aln.node.node_id,
        event_type: AuditEventType::PolicyDecision,
        payload: serde_json::json!({
            "kind": "KeyRotated",
            "drone_id": keystate.drone_id,
            "previous_key_id": keystate.current_key_id,
            "new_key_id": new_key_id,
            "new_pubkey_der": base64::encode(&new_pubkey_der),
            "organichain_anchor": aln.audit_anchor,
            "tx_hash_compromised": txhash,
        }),
        hash_hex: "feedface0123456789abcdef00112233".into(),
    };
    cybercore_brain::insert_audit_record(conn, &rotated_record)?;
    let txhash_rot = anchor_tx_fn(&keystate.organichain_anchor, &rotated_record)
        .map_err(PcsError::Anchor)?;
    cybercore_brain::insert_log_anchor(
        conn,
        &rotated_record.id,
        &keystate.organichain_anchor,
        &txhash_rot,
    )?;

    // Step 4: update in-memory state
    keystate.current_key_id = new_key_id;
    keystate.current_pubkey_der = new_pubkey_der;

    Ok(keystate)
}

/// Minimal stub – in production this is provided by HSM/TEE.
fn generate_fresh_pubkey() -> Vec<u8> {
    vec![1, 3, 5, 7, 9, 11, 13, 15]
}
