# File: crates/xr-telemetry-client/README.md
# Repo: mk-bluebird/Cybercore

# xr-telemetry-client

`xr-telemetry-client` is a Rust 2024 crate that implements a production-grade
telemetry and safety-decision client for XR/AR/VR and smart-glasses devices
integrated with the CyberNano / Cybercore stack.

It:

- Collects XR, EEG, physiological, and subjective telemetry.
- Encodes messages using a stable JSON schema.
- Streams telemetry to a CyberNano gateway over WebSocket.
- Receives safety decisions (including Alzheimer-aware Lyapunov guards).
- Applies XR actions (throttle, care-mode, halt) via a pluggable runtime trait.
- Sends acknowledgements for audit and EvidenceBundle binding.

This crate is designed for augmented citizens and cybernetic integrations,
with non-regressive, host-sovereign safety as a hard requirement.

---

## Features

- Rust edition 2024, `rust-version = "1.85"`.
- No unsafe code.
- JSON wire format with `serde` for interoperability.
- Async WebSocket client using `tokio` and `tokio-tungstenite`.
- Pluggable `XrRuntime` trait for integration with any XR/AR/VR engine.
- Pre-wired types for:

  - Telemetry (EEG, XR tasks, physio, subjective, safety proxies).
  - Safety decisions (AD Lyapunov, RoH, continuity, healthcare mode).
  - Configuration updates from the gateway.
  - Acknowledgements for guard decisions.

---

## Wire Protocol Overview

Messages are encoded as JSON and framed as WebSocket text messages.

### Telemetry (client → gateway)

Type tag: `"telemetry"`

Carries:

- Host and session identifiers.
- Device metadata (ID, model, firmware).
- Time window.
- Mode context (`medical`, `security`, `mixed`, `other`).
- Optional EEG features:

  - Band powers.
  - Microstate features (entropy, transition rate).
  - Connectivity proxy (small-world estimate, coherence).

- XR task metadata and performance.
- Physiological signals.
- Subjective state estimates (fatigue, pain, stress).
- Safety proxies (local RoH, biomechscore estimates).

### Decision (gateway → client)

Type tag: `"decision"`

Carries:

- Allow/deny verdict and reason.
- Alzheimer safety block (CSI, sigma, xbar_tau_est, V_AD and RoH).
- Continuity envelope status.
- Healthcare priority mode (baseline, care-only).
- XR actions:

  - `proceed`, `throttle`, `switch_to_care`, `halt_overlay`.
  - Recommended intensity scaling.
  - Blocked features and UI hints.

- Audit metadata (EvidenceBundle ID, guard version).

### Config (gateway → client)

Type tag: `"config"`

Carries:

- Thresholds for AD safety (csi_min, sigma_min, xbar_tau_max, v_ad_max).
- RoH limits.
- Telemetry window configuration.
- XR policy constraints (max overlay complexity, allowed roles per mode).

### Ack (client → gateway)

Type tag: `"ack"`

Carries:

- Reference to decision ID.
- Whether the decision was applied.
- Optional details on XR actions applied.

---

## Integration Steps

1. Add the crate to `mk-bluebird/Cybercore`:

   - Place this crate at `crates/xr-telemetry-client`.
   - Add it to the workspace `Cargo.toml`.

2. Implement `XrRuntime` for your device or engine:

   - Provide accessors for current task, mode, physio, subjective and EEG blocks.
   - Implement `apply_decision` to adjust overlays, scenes, and workloads.

3. Configure the client:

   - Set `hostdid`, `session_id`, `device` info, and `gateway_url`.
   - Configure `TelemetryConfig` (window duration, max missed windows).

4. Run the async loop:

   - Call `run_xr_client(runtime, client_config).await` from your XR app.

This will immediately connect your XR hardware to the CyberNano safety
gateway and enforce AD-aware, RoH-aware, and continuity-aware safety
decisions in real time.

---

## Safety and Sovereignty

This crate is designed to respect:

- Host sovereignty (`hostdid`-bound, `bostrom_address`-anchored in the wider stack).
- Psychological continuity envelopes.
- Healthcare priority envelopes (care-only mode).
- Alzheimer progression Lyapunov guards (V_AD non-increase).
- Biomechanical non-regression via biomechscore proxies.

It is intended as a core building block for host-sovereign cybernetic
XR integrations with non-derogable safety guarantees.
