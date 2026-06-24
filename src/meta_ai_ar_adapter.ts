// src/meta_ai_ar_adapter.ts
import { MetaArClient } from "./meta_ar_client";

export class MetaAiArAdapter {
  constructor(private arClient: MetaArClient) {}

  async chooseEngagementMode() {
    const presence = await this.arClient.requestPresenceMode();
    // Default conservative behavior if anything is unclear
    if (!presence.continuity_ok) {
      return "minimal";
    }
    if (presence.p >= 0.8) return "deep";
    if (presence.p >= 0.5) return "normal";
    return "light";
  }

  async fetchTelemetryForCoaching() {
    const resp = await this.arClient.requestSafeTelemetry(["hrv", "eeg_bands"]);
    if (!resp.allowed) {
      return null;
    }
    return resp;
  }
}
