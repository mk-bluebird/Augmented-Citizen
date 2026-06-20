// crates/xr-telemetry-client/src/client.rs
#![feature(rust_2024_preview)]
#![forbid(unsafe_code)]

use crate::{
    AckMessage, AppliedActionsBlock, ClientMeta, ConnectivityFeatures, DeviceInfo, EegBlock,
    ModeContext, PhysioBlock, SafetyProxies, SubjectiveBlock, TaskPerformance, TelemetryConfig,
    TelemetryMessage, TimeWindow, WireMessage, XrTask,
};
use chrono::{DateTime, Utc};
use tokio::time::{sleep, Duration};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use url::Url;

/// Minimal XR state interface that your XR/AR/VR runtime can implement.
pub trait XrRuntime {
    fn current_task(&self) -> XrTask;
    fn current_mode(&self) -> ModeContext;
    fn current_physio(&self) -> PhysioBlock;
    fn current_subjective(&self) -> SubjectiveBlock;
    fn current_eeg(&self) -> EegBlock;
    fn current_safety_proxies(&self) -> SafetyProxies;

    /// Apply XR safety decision (throttle, switch_to_care, etc.).
    fn apply_decision(&mut self, decision: crate::DecisionMessage);
}

/// Client configuration for connecting to the CyberNano gateway.
pub struct ClientConfig {
    pub hostdid: String,
    pub session_id: String,
    pub device: DeviceInfo,
    pub gateway_url: String,          // e.g. "wss://gateway.example.com/xr"
    pub telemetry: TelemetryConfig,
}

/// Asynchronous client loop: send telemetry, process decisions, send acks.
pub async fn run_xr_client<R: XrRuntime + Send>(
    mut runtime: R,
    cfg: ClientConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = Url::parse(&cfg.gateway_url)?;
    let (ws_stream, _) = connect_async(url).await?;
    let (mut ws_writer, mut ws_reader) = ws_stream.split();

    let mut last_decision_id: Option<String> = None;
    let mut interval = cfg.telemetry.window_seconds;

    loop {
        // Build telemetry message
        let now: DateTime<Utc> = Utc::now();
        let end = now;
        let start = end - chrono::Duration::seconds(interval as i64);

        let telemetry = TelemetryMessage {
            r#type: "telemetry".to_string(),
            version: "1.0".to_string(),
            hostdid: cfg.hostdid.clone(),
            session_id: cfg.session_id.clone(),
            device: cfg.device.clone(),
            time_window: TimeWindow {
                t_start_utc: start.to_rfc3339(),
                t_end_utc: end.to_rfc3339(),
            },
            mode: runtime.current_mode(),
            eeg: runtime.current_eeg(),
            xr_task: runtime.current_task(),
            physio: runtime.current_physio(),
            subjective: runtime.current_subjective(),
            safety_proxies: runtime.current_safety_proxies(),
            client_meta: ClientMeta {
                app_version: "0.1.0".to_string(),
                network_rtt_ms: None,
            },
        };

        let wire_msg = WireMessage::Telemetry {
            version: telemetry.version.clone(),
            hostdid: telemetry.hostdid.clone(),
            session_id: telemetry.session_id.clone(),
            device: telemetry.device.clone(),
            time_window: telemetry.time_window.clone(),
            mode: telemetry.mode.clone(),
            eeg: telemetry.eeg.clone(),
            xr_task: telemetry.xr_task.clone(),
            physio: telemetry.physio.clone(),
            subjective: telemetry.subjective.clone(),
            safety_proxies: telemetry.safety_proxies.clone(),
            client_meta: telemetry.client_meta.clone(),
        };

        let payload = serde_json::to_string(&wire_msg)?;
        ws_writer.send(Message::Text(payload)).await?;

        // Wait for a response (decision or config)
        if let Some(msg) = ws_reader.next().await {
            let msg = msg?;
            if msg.is_text() {
                let txt = msg.into_text()?;
                let parsed: WireMessage = serde_json::from_str(&txt)?;
                match parsed {
                    WireMessage::Decision(dec) => {
                        last_decision_id = Some(dec.decision_id.clone());
                        // Apply decision to XR runtime
                        runtime.apply_decision(dec);
                        // Send ack
                        if let Some(ref dec_id) = last_decision_id {
                            let ack = AckMessage {
                                r#type: "ack".to_string(),
                                version: "1.0".to_string(),
                                hostdid: cfg.hostdid.clone(),
                                session_id: cfg.session_id.clone(),
                                device_id: cfg.device.device_id.clone(),
                                ref_decision_id: dec_id.clone(),
                                applied: true,
                                applied_actions: None,
                                time_utc: Utc::now().to_rfc3339(),
                            };
                            let ack_msg = WireMessage::Ack(ack);
                            let ack_payload = serde_json::to_string(&ack_msg)?;
                            ws_writer.send(Message::Text(ack_payload)).await?;
                        }
                    }
                    WireMessage::Config(c) => {
                        // Update telemetry interval if changed
                        interval = c.telemetry.window_seconds;
                    }
                    WireMessage::Telemetry { .. } => {
                        // Gateway should not send telemetry to client; ignore
                    }
                    WireMessage::Ack(_) => {
                        // Acks from gateway not used in this minimal client
                    }
                }
            }
        }

        sleep(Duration::from_secs(interval as u64)).await;
    }
}
