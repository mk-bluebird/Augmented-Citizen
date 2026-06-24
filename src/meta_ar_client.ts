// src/meta_ar_client.ts
import { JSONRPCClient } from "json-rpc-2.0";

type HostIdentity = {
  host_did: string;
  brain_ip_hash: string;
  bostrom_address: string;
};

type SafeTelemetryResponse = {
  allowed: boolean;
  hrv?: { rmssd: number };
  eeg?: { theta_beta: number };
  gait?: { score: number };
  presence?: { p: number };
};

export class MetaArClient {
  private ws: WebSocket;
  private rpc: JSONRPCClient;

  constructor(
    private hostIdentity: HostIdentity,
    wsUrl: string = "wss://host-interface.example/augmented-chat"
  ) {
    this.ws = new WebSocket(wsUrl);
    this.rpc = new JSONRPCClient((req) => {
      if (this.ws.readyState === WebSocket.OPEN) {
        this.ws.send(JSON.stringify(req));
        return Promise.resolve();
      }
      return new Promise<void>((resolve, reject) => {
        this.ws.addEventListener("open", () => {
          this.ws.send(JSON.stringify(req));
          resolve();
        });
        this.ws.addEventListener("error", () => reject());
      });
    });

    this.ws.addEventListener("message", (event) => {
      try {
        const msg = JSON.parse(event.data);
        this.rpc.receive(msg);
      } catch {
        // ignore malformed frames
      }
    });
  }

  async requestSafeTelemetry(scopes: string[]): Promise<SafeTelemetryResponse> {
    return this.rpc.request("request_neural_telemetry", {
      host_identity: this.hostIdentity,
      scopes,
    }) as Promise<SafeTelemetryResponse>;
  }

  async requestPresenceMode(): Promise<{ p: number; continuity_ok: boolean }> {
    return this.rpc.request("request_presence_confidence", {
      host_identity: this.hostIdentity,
    }) as Promise<{ p: number; continuity_ok: boolean }>;
  }

  async registerArPreferences(preferences: any): Promise<void> {
    await this.rpc.request("register_ar_preferences", {
      host_identity: this.hostIdentity,
      preferences,
    });
  }
}
