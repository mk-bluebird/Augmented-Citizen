-- filename: schema/master.sql
-- destination: EcoNet-CEIM-PhoenixWater/schema/master.sql
-- purpose: Enforce biocompatibility and psychological continuity for all actuation channels.

PRAGMA foreign_keys = ON;

-- 1. Registry of all known actuation channels and their continuity risk weights.
CREATE TABLE IF NOT EXISTS actuation_channel_registry (
    id                  INTEGER PRIMARY KEY AUTOINCREMENT,
    channel_key         TEXT NOT NULL UNIQUE,       -- e.g., 'nano_detox_v1', 'tES_focus_v2'
    domain_id           INTEGER NOT NULL REFERENCES domain(id),
    physical_modality   TEXT NOT NULL,              -- 'CHEMICAL', 'ELECTRICAL', 'MECHANICAL', 'THERMAL'
    continuity_risk_weight REAL NOT NULL CHECK (continuity_risk_weight BETWEEN 0.0 AND 1.0),
    max_thermal_delta_c REAL NOT NULL,
    max_protein_draw_mg REAL NOT NULL,
    requires_shadow_sim INTEGER NOT NULL DEFAULT 1, -- 1 = must pass digital twin sim before physical actuation
    description         TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_actuation_channel_key ON actuation_channel_registry(channel_key);

-- 2. The Psychological Continuity State Vector (Baseline and Current).
CREATE TABLE IF NOT EXISTS continuity_state_vector (
    id                  INTEGER PRIMARY KEY AUTOINCREMENT,
    host_did            TEXT NOT NULL,
    recorded_utc        INTEGER NOT NULL,           -- Unix timestamp
    memory_recall_latency_ms REAL NOT NULL,
    affective_baseline_variance REAL NOT NULL,
    belief_kernel_stability_score REAL NOT NULL CHECK (belief_kernel_stability_score BETWEEN 0.0 AND 1.0),
    neuroinflammation_proxy REAL NOT NULL,          -- Derived from HRV/autonomic telemetry
    is_within_basin_of_attraction INTEGER NOT NULL DEFAULT 1 -- 1 = Safe, 0 = Actuations must be halted
);

CREATE INDEX IF NOT EXISTS idx_continuity_host_time ON continuity_state_vector(host_did, recorded_utc DESC);

-- 3. Audit log of actuation attempts and their continuity impact.
CREATE TABLE IF NOT EXISTS continuity_actuation_audit (
    id                  INTEGER PRIMARY KEY AUTOINCREMENT,
    host_did            TEXT NOT NULL,
    channel_key         TEXT NOT NULL REFERENCES actuation_channel_registry(channel_key),
    pre_actuation_stability REAL NOT NULL,
    projected_post_actuation_stability REAL NOT NULL,
    delta_continuity    REAL NOT NULL,              -- Must be >= 0 (Monotonicity invariant)
    action_taken        TEXT NOT NULL,              -- 'ALLOWED', 'BLOCKED_BY_GUARD', 'BLOCKED_BY_HOST'
    audit_signature     TEXT NOT NULL               -- Cryptographic hash of the decision state
);

-- Seed baseline actuation channels with strict continuity constraints.
INSERT OR IGNORE INTO actuation_channel_registry 
    (channel_key, domain_id, physical_modality, continuity_risk_weight, max_thermal_delta_c, max_protein_draw_mg, requires_shadow_sim, description)
VALUES
    ('nano_detox_support', 1, 'CHEMICAL', 0.15, 0.2, 2.5, 1, 'Nanoswarm detox support; low continuity risk, strictly bounded protein draw.'),
    ('tes_focus_modulation', 1, 'ELECTRICAL', 0.45, 0.4, 1.0, 1, 'Transcranial electrical stimulation for focus; moderate risk, requires strict thermal ceilings.'),
    ('affective_soothing', 1, 'ELECTRICAL', 0.60, 0.3, 0.5, 1, 'Affective state modulation; high continuity risk, forbidden if belief_kernel_stability < 0.85.');
