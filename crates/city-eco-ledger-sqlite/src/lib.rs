// crates/city-eco-ledger-sqlite/src/lib.rs
#![forbid(unsafe_code)]

use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcoLedgerEntry {
    pub id: Uuid,
    pub city_id: String,
    pub date_ymd: String,
    pub e_city_kwh: f64,
    pub measured_fraction: f64,
    pub modeled_fraction: f64,
    pub iso14064_verified: bool,
    pub ts_utc: OffsetDateTime,
    pub hashhex: String,
}

pub fn init_eco_ledger_schema(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS eco_ledger (
            id TEXT PRIMARY KEY,
            city_id TEXT NOT NULL,
            date_ymd TEXT NOT NULL,
            e_city_kwh REAL NOT NULL,
            measured_fraction REAL NOT NULL,
            modeled_fraction REAL NOT NULL,
            iso14064_verified INTEGER NOT NULL,
            ts_utc TEXT NOT NULL,
            hashhex TEXT NOT NULL
        );
        "#,
    )
}

pub fn insert_eco_ledger_entry(
    conn: &Connection,
    entry: &EcoLedgerEntry,
) -> rusqlite::Result<()> {
    conn.execute(
        r#"
        INSERT INTO eco_ledger (
            id, city_id, date_ymd, e_city_kwh,
            measured_fraction, modeled_fraction,
            iso14064_verified, ts_utc, hashhex
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
        "#,
        params![
            entry.id.to_string(),
            entry.city_id,
            entry.date_ymd,
            entry.e_city_kwh,
            entry.measured_fraction,
            entry.modeled_fraction,
            if entry.iso14064_verified { 1 } else { 0 },
            entry.ts_utc.to_string(),
            entry.hashhex,
        ],
    )?;
    Ok(())
}
