// crates/drone-kernel/src/lib.rs
// Design: D High, NR Medium, EE High
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

use rusqlite::{params, Connection};
use thiserror::Error;

use regex::Regex;
use lazy_static::lazy_static;

// --- 1. ZkpPresentation replay defence with GNSS+secure clock+Bostrom anchor ---
// Grounded in Tier-1 trust/audit design and Bostrom anchoring from the Augmented-Citizen doc [file:1].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkpPresentation {
    pub proof_id: Uuid,
    pub holder_did: String,
    pub circuit_name: String,
    pub not_before: OffsetDateTime,
    pub not_after: OffsetDateTime,
    pub ts_claimed: OffsetDateTime,
    pub ledger_anchor_height: i64,
    pub ledger_anchor_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeTrustSnapshot {
    pub ts_local_secure_clock: OffsetDateTime,
    pub ts_tsn_network: OffsetDateTime,
    pub ts_gnss: OffsetDateTime,
    pub ts_ledger_last_anchor: OffsetDateTime,
    pub max_skew_ms: i64,
    pub max_age_ms: i64,
}

#[derive(Debug, Error)]
pub enum TimeTrustError {
    #[error("time source skew too large: {0}")]
    SkewTooLarge(String),
    #[error("presentation expired or not yet valid")]
    WindowInvalid,
    #[error("ledger anchor stale for replay protection")]
    LedgerAnchorStale,
}

pub fn validate_time_trust(
    pres: &ZkpPresentation,
    snap: &TimeTrustSnapshot,
) -> Result<(), TimeTrustError> {
    let t_local = snap.ts_local_secure_clock;
    let t_net = snap.ts_tsn_network;
    let t_gnss = snap.ts_gnss;
    let t_anchor = snap.ts_ledger_last_anchor;
    let skew_ln = (t_local - t_net).whole_milliseconds().abs();
    let skew_lg = (t_local - t_gnss).whole_milliseconds().abs();
    if skew_ln > snap.max_skew_ms || skew_lg > snap.max_skew_ms {
        return Err(TimeTrustError::SkewTooLarge(format!(
            "local-net={}ms local-gnss={}ms",
            skew_ln, skew_lg
        )));
    }
    if t_local < pres.not_before || t_local > pres.not_after {
        return Err(TimeTrustError::WindowInvalid);
    }
    let age_ms = (t_local - pres.ts_claimed).whole_milliseconds().abs();
    if age_ms > snap.max_age_ms {
        return Err(TimeTrustError::WindowInvalid);
    }
    let anchor_delta_ms = (t_local - t_anchor).whole_milliseconds();
    if anchor_delta_ms > snap.max_age_ms {
        return Err(TimeTrustError::LedgerAnchorStale);
    }
    Ok(())
}

// Mathematical solution example for skew bound:
// Let t_l, t_n be local and TSN times in ms; skew = |t_l - t_n| must satisfy skew <= S_max.
// For t_l = 1_000_000, t_n = 999_500, S_max = 1_000, skew = 500 <= 1_000, so the check passes.
// To recompute: subtract, take absolute value, compare to bound.

// Scientific grounding: Time-Sensitive Networking and IEEE 802.1-based synchronization can hold sub-ms offsets, allowing a secure clock plus network time cross-check to detect GNSS spoofing while Bostrom-anchored hashes give a tamper-evident replay ceiling.[file:1]

// Legal terms (>=100 chars):
// Time validation logic must comply with aviation and critical-infrastructure law, ensuring logs of all rejected proofs are retained for regulator review, with explicit rationale codes for skew, window, and anchor failures, and that no override exists for unauthorized operators or vendors.

// Geographical evidence (5 locations):
// Phoenix AZ drone corridors; Rotterdam port drone ops; Singapore urban air mobility trials; Oslo smart-mobility pilots; Tokyo autonomous vehicle TSN labs.

// --- 2. ZK proving system enum + PQ-strength layout hint ---
// ZKP catalog requirement and Groth16/Plonk grounding from Zcash spec / OpenFHE mapping.[file:1][file:2]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ZkSystem {
    Groth16,
    Plonk,
    Spartan,
    FRIStark,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptographicProofLayout {
    pub system: ZkSystem,
    pub commitment_len: u16,
    pub proof_len: u16,
    pub public_input_len: u16,
    pub transcript_len: u16,
}

pub fn mt6883_cache_constrained_layout(system: ZkSystem) -> CryptographicProofLayout {
    match system {
        ZkSystem::Groth16 => CryptographicProofLayout {
            system,
            commitment_len: 96,
            proof_len: 192,
            public_input_len: 128,
            transcript_len: 256,
        },
        ZkSystem::Plonk => CryptographicProofLayout {
            system,
            commitment_len: 128,
            proof_len: 512,
            public_input_len: 192,
            transcript_len: 384,
        },
        ZkSystem::Spartan => CryptographicProofLayout {
            system,
            commitment_len: 64,
            proof_len: 256,
            public_input_len: 192,
            transcript_len: 320,
        },
        ZkSystem::FRIStark => CryptographicProofLayout {
            system,
            commitment_len: 160,
            proof_len: 1024,
            public_input_len: 256,
            transcript_len: 640,
        },
    }
}

// Mathematical solution:
// Given an L1 cache budget C and layout fields (c, p, i, t) in bytes, total footprint F = c + p + i + t.
// For Spartan layout above, F = 64 + 256 + 192 + 320 = 832 bytes, which easily fits into a 64 KB L1.
// To recompute: sum all field sizes; compare F <= 65_536 for MT6883.

// Scientific grounding: Small-circuit SNARKs and Spartan-like schemes can achieve sub-kilobyte proof objects; combined with OpenFHE-style edge cryptography and Zcash-tier constructions they support low-memory proving while the post-quantum 128-bit level is maintained through Kyber/Dilithium outer layers and ledger anchoring rather than relying solely on SNARK assumptions.[file:1][file:2]

// Legal terms (>=100 chars):
// Any chosen proving system must be documented with security parameters, curve or lattice settings, and proof sizes, and subjected to independent cryptographic review before deployment in high-risk urban airspace, with contract clauses prohibiting unvetted parameter downgrades or opaque vendor forks.

// Geographical evidence:
// Phoenix MT6883 edge lab; Berlin privacy-tech hubs; Brussels EU AI Act enforcement; Singapore crypto research groups; Cambridge UK cryptography labs.

// --- 3. SensorGatingHardware trait and ISP-TEE interface ---
// Based on SensorGatingHardware and GlobalPlatform/TEE patterns.[file:1][file:2]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IspAttestationReport {
    pub isp_firmware_hash: String,
    pub tee_measurement: String,
    pub frames_total: u64,
    pub frames_blocked: u64,
    pub pixels_to_main_mem: u64,
    pub pixels_to_secure_mem: u64,
    pub nonce: String,
    pub signature: Vec<u8>,
}

#[derive(Debug, Error)]
pub enum SensorGatingError {
    #[error("TEE communication failed: {0}")]
    TeeComm(String),
    #[error("ISP attestation invalid: {0}")]
    Attestation(String),
}

pub trait SensorGatingHardware {
    fn assert_hard_block(&mut self) -> Result<(), SensorGatingError>;
    fn clear_hard_block(&mut self) -> Result<(), SensorGatingError>;
    fn install_frame_drop_fence(&mut self) -> Result<(), SensorGatingError>;
    fn query_isp_attestation(&self, nonce: &str) -> Result<IspAttestationReport, SensorGatingError>;
}

#[derive(Debug)]
pub struct TeeIspGating {
    pub isp_device_id: String,
}

impl SensorGatingHardware for TeeIspGating {
    fn assert_hard_block(&mut self) -> Result<(), SensorGatingError> {
        // Here we would call into a TEE driver / GlobalPlatform-compliant API.
        // For functional code without external dependencies, we simulate success.
        Ok(())
    }

    fn clear_hard_block(&mut self) -> Result<(), SensorGatingError> {
        Ok(())
    }

    fn install_frame_drop_fence(&mut self) -> Result<(), SensorGatingError> {
        Ok(())
    }

    fn query_isp_attestation(&self, nonce: &str) -> Result<IspAttestationReport, SensorGatingError> {
        let report = IspAttestationReport {
            isp_firmware_hash: "isp-fw-sha2-placeholder".to_string(),
            tee_measurement: "tee-mr-enclave-hash".to_string(),
            frames_total: 10_000,
            frames_blocked: 10_000,
            pixels_to_main_mem: 0,
            pixels_to_secure_mem: 1920 * 1080 * 10_000,
            nonce: nonce.to_string(),
            signature: vec![0u8; 64],
        };
        Ok(report)
    }
}

pub fn verify_no_pixels_escaped(report: &IspAttestationReport) -> Result<(), SensorGatingError> {
    if report.pixels_to_main_mem != 0 {
        return Err(SensorGatingError::Attestation(
            "non-zero pixels_to_main_mem".to_string(),
        ));
    }
    Ok(())
}

// Mathematical solution:
// Let P_main be the pixel count to main memory, P_total the total captured.
// Safety requires P_main = 0 and P_secure = P_total, with P_total = width * height * frames.
// For 1920x1080 and 10 frames, P_total = 1920 * 1080 * 10 = 20_736_000.
// To recompute: multiply resolution and frame count; enforce main-mem count = 0.

// Scientific grounding: ISP-level gating with hardware interrupts and secure elements is consistent with IEEE 2410 biometric privacy guidance and GlobalPlatform TEE patterns; an attestation report that proves zero pixels in main memory fulfills NOPASSIVEBIOMETRICSCANNING constraints and LEDGER-anchored audit invariants.[file:1][file:2]

// Legal terms (>=100 chars):
// The sensor-gating interface must be specified as part of the drone’s safety case, including verifiable attestations that no biometric pixels reach general-purpose RAM, with periodic audits and certifiable reports under aviation, data-protection, and biometric-privacy regulations, and no vendor backdoor to bypass HARD_BLOCK or frame-drop fences.

// Geographical evidence:
// Phoenix drone privacy corridors; Barcelona biometric policy pilots; Geneva healthtech ethics; Tokyo ISP/TEE research; Singapore drone trials.

// --- 4. SQLite drone_verification_audit table and write amplification ---
// Schema aligned with audit sink in cybercore-brain / Augmented-Citizen spec.[file:1]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DroneVerificationAuditRow {
    pub id: Uuid,
    pub ts_utc: OffsetDateTime,
    pub drone_id: String,
    pub citizen_did: String,
    pub zk_system: String,
    pub proof_bytes: Vec<u8>,
    pub decision: String,
    pub reason_code: String,
}

pub fn init_drone_verification_audit_schema(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS drone_verification_audit(
            id TEXT PRIMARY KEY,
            ts_utc TEXT NOT NULL,
            drone_id TEXT NOT NULL,
            citizen_did TEXT NOT NULL,
            zk_system TEXT NOT NULL,
            proof_bytes BLOB NOT NULL,
            decision TEXT NOT NULL,
            reason_code TEXT NOT NULL
        );
        "#,
    )?;
    Ok(())
}

pub fn insert_drone_verification_audit(
    conn: &Connection,
    row: &DroneVerificationAuditRow,
) -> rusqlite::Result<()> {
    conn.execute(
        r#"
        INSERT INTO drone_verification_audit
        (id, ts_utc, drone_id, citizen_did, zk_system, proof_bytes, decision, reason_code)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8);
        "#,
        params![
            row.id.to_string(),
            row.ts_utc.to_string(),
            row.drone_id,
            row.citizen_did,
            row.zk_system,
            row.proof_bytes,
            row.decision,
            row.reason_code
        ],
    )?;
    Ok(())
}

// Per-row storage estimation for 10_000 records on 128 MB RAM edge device:
//
// Assume:
// - id: TEXT, 36 bytes UUID string + ~2 bytes SQLite overhead ≈ 40 B
// - ts_utc: 32-char RFC3339 string + 2 B ≈ 34 B
// - drone_id: 16-char string + 2 B ≈ 18 B
// - citizen_did: 64-char DID + 2 B ≈ 66 B
// - zk_system: up to 16-char label ≈ 18 B
// - proof_bytes: 512 B typical SNARK/STARK proof
// - decision: 8-char string ≈ 10 B
// - reason_code: 32-char string ≈ 34 B
//
// Sum payload ≈ 40 + 34 + 18 + 66 + 18 + 512 + 10 + 34 = 732 bytes.
// With SQLite page and index overhead (approx 30%), per-row ≈ 952 bytes.
// For 10_000 rows, table size ≈ 9.52 MB.
//
// Indexing overhead:
// - PRIMARY KEY on id uses B-tree; index entry ~ (36 B key + pointer) ≈ 48 B.
// - 10_000 entries ≈ 480 kB, plus fragmentation ≈ 650 kB total.
// Total storage ≈ 9.5 MB + 0.65 MB ≈ 10.15 MB.
//
// Write amplification on eMMC at 50 verifications/hour:
// - Each insert writes ~1 KB logical.
// - SQLite uses 4 KB pages; assume 1 page per insert on average -> 4 KB physical.
// - WA factor ≈ 4x (4 KB / 1 KB).
// - Per hour: 50 * 4 KB = 200 KB writes.
// - Per day: 4.8 MB. Over 5 years (~1_825 days): ≈ 8.76 GB, below typical eMMC TBW.
// To recompute: estimate logical row size, multiply by WA factor and events; compare cumulative writes to device endurance.

// Scientific grounding: SQLite’s page-based storage and B-tree indices are stable and well-documented, and embedded use with append-only audit patterns is common in IoT/edge; the above sizing aligns with Tier 3 audit schema guidance in Augmented-Citizen, leaving ample headroom on 128 MB RAM and moderate eMMC endurance.[file:1]

// Legal terms (>=100 chars):
// Audit tables must be append-only from the application’s perspective, with retention windows, export tools for regulators, and clear documentation that deletion or compaction never silently erases safety-critical verification decisions or their reasons within mandated retention periods.

// Geographical evidence:
// Phoenix smart-city drones; Brussels data-retention guidance; Washington DC FAA/UAS policy; Singapore smart-nation pilots; Tokyo embedded systems labs.

// --- 5. Multi-citizen kernel extensions: per-resident gating ---
// Extends single-handshake design to many concurrent citizens, with per-view gating.[file:1]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CitizenClaimContext {
    pub citizen_did: String,
    pub claim_type: String,
    pub claim_scope: String,
    pub zk_system: ZkSystem,
    pub proof_layout: CryptographicProofLayout,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldOfViewGate {
    pub citizen_did: String,
    pub allowed: bool,
    pub hard_block: bool,
}

#[derive(Debug)]
pub struct MultiCitizenKernelState {
    pub active_claims: Vec<CitizenClaimContext>,
    pub fov_gates: Vec<FieldOfViewGate>,
}

impl MultiCitizenKernelState {
    pub fn new() -> Self {
        Self {
            active_claims: Vec::new(),
            fov_gates: Vec::new(),
        }
    }

    pub fn upsert_claim(&mut self, ctx: CitizenClaimContext) {
        if let Some(slot) = self
            .active_claims
            .iter_mut()
            .find(|c| c.citizen_did == ctx.citizen_did)
        {
            *slot = ctx;
        } else {
            self.active_claims.push(ctx);
        }
    }

    pub fn set_fov_gate(&mut self, citizen_did: &str, allowed: bool, hard_block: bool) {
        if let Some(slot) = self
            .fov_gates
            .iter_mut()
            .find(|g| g.citizen_did == citizen_did)
        {
            slot.allowed = allowed;
            slot.hard_block = hard_block;
        } else {
            self.fov_gates.push(FieldOfViewGate {
                citizen_did: citizen_did.to_string(),
                allowed,
                hard_block,
            });
        }
    }

    pub fn fov_policy_for(&self, citizen_did: &str) -> Option<&FieldOfViewGate> {
        self.fov_gates.iter().find(|g| g.citizen_did == citizen_did)
    }
}

// Mathematical solution:
// Let N be the number of citizens in the plaza (e.g., N = 50), and state size per citizen S.
// Total kernel state K = N * S.
// If S ≈ 512 bytes (claim + gate), then K ≈ 50 * 512 = 25_600 bytes (~25 KB), well within MT6883 L1/L2 cache.
// To recompute: estimate structure sizes, multiply by citizen count, compare with cache budget.

// Scientific grounding: Kernel-level bookkeeping of per-identity gates aligns with the multi-agent, per-node governance described in cybercore-brain and VitalNet planes; a drone’s view can treat each identified citizen as a separate policy axis, similar to viability kernels in Cyberswarm.[file:1][file:4]

// Legal terms (>=100 chars):
// Multi-citizen handling must ensure that each person’s consent, claims, and gating state are independent, forbidding cross-contamination of biometric or credential data between residents and codifying non-discrimination constraints so that field-of-view restrictions are driven strictly by verifiable claims and law, not by protected attributes or opaque profiling.

// Geographical evidence:
// Phoenix public plazas; Barcelona drone consent rules; New York pedestrian density trials; Seoul AR mobility projects; Singapore multi-user XR deployments.

// --- Trivia-mode global hex answer >=50 chars ---
// This hex encodes a statement about multi-citizen ZKP, TSN time, Bostrom anchoring, and per-resident sensor gating.
pub const INFRA_HEX_ANSWER: &str =
    "0x4d756c74692d636974697a656e205a4b502b54534e2b426f7374726f6d2d616e63686f7273203d207265706c61792d736166652064726f6e6520766572696669636174696f6e206b65726e656c";
