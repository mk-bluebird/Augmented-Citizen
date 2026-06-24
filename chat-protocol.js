// chat-protocol.js
// Chat-based interaction protocol for AI agents
// JSON-RPC over WebSocket with host identity binding

class AugmentedChatProtocol {
  constructor(hostIdentity) {
    this.hostDid = hostIdentity.hostDid;
    this.brainIpHash = hostIdentity.brainIpHash;
    this.ws = null;
    this.pendingRequests = new Map();
  }

  async connect(endpoint) {
    this.ws = new WebSocket(endpoint);
    
    return new Promise((resolve, reject) => {
      this.ws.onopen = async () => {
        // Authenticate with ephemeral identity token
        const token = await this.createIdentityToken();
        this.send({
          jsonrpc: '2.0',
          method: 'auth',
          params: {
            host_did: this.hostDid,
            token: token,
            capabilities: ['neural_telemetry_readonly', 'biocompatibility', 'presence']
          },
          id: 1
        });
        resolve();
      };
      
      this.ws.onmessage = (event) => this.handleMessage(JSON.parse(event.data));
      this.ws.onerror = reject;
    });
  }

  async createIdentityToken() {
    // Create short-lived token, never expose raw brain data
    const payload = {
      host_did: this.hostDid,
      brain_ip_hash: this.brainIpHash,
      iat: Math.floor(Date.now() / 1000),
      exp: Math.floor(Date.now() / 1000) + 300, // 5 min
      nonce: Array.from(crypto.getRandomValues(new Uint8Array(16))).map(b => b.toString(16).padStart(2,'0')).join('')
    };
    return btoa(JSON.stringify(payload));
  }

  send(message) {
    this.ws.send(JSON.stringify(message));
  }

  async handleMessage(msg) {
    if (msg.method === 'request_neural_telemetry') {
      // AI agent requesting neural data
      const allowed = await this.checkTelemetryPermission(msg.params.agent_id, msg.params.scopes);
      
      if (!allowed.allowed) {
        this.send({
          jsonrpc: '2.0',
          id: msg.id,
          error: { code: 403, message: allowed.reason }
        });
        return;
      }

      // Return only allowed, anonymized data
      const telemetry = await this.getSafeTelemetry(msg.params.scopes);
      
      this.send({
        jsonrpc: '2.0',
        id: msg.id,
        result: {
          host_did: this.hostDid,
          timestamp: Date.now(),
          data: telemetry,
          consent_c_n: allowed.consent_c_n
        }
      });
    }
  }

  async checkTelemetryPermission(agentId, scopes) {
    // 1. Check consent in IndexedDB
    if (!window.consentDB) return { allowed: false, reason: 'No consent DB' };
    
    const history = await window.consentDB.getAll();
    const grant = history.find(e => 
      e.event_type === 'grant' &&
      e.data_categories.includes('neural_telemetry') &&
      e.processor === agentId
    );
    
    if (!grant) return { allowed: false, reason: 'No consent grant found' };

    // 2. Check biocompatibility
    const roh = await this.getCurrentRoH();
    if (roh > 0.30) return { allowed: false, reason: 'RoH ceiling exceeded' };

    // 3. Check scopes
    const allowedScopes = scopes.filter(s => 
      ['hrv', 'eeg_theta_beta', 'presence'].includes(s)
    );

    return {
      allowed: true,
      consent_c_n: grant.c_n,
      scopes: allowedScopes
    };
  }

  async getSafeTelemetry(scopes) {
    const data = {};
    
    if (scopes.includes('hrv')) {
      // Return only processed HRV, not raw ECG
      data.hrv = { rmssd: 42, timestamp: Date.now() };
    }
    
    if (scopes.includes('eeg_theta_beta')) {
      // Return ratio, not raw waves
      data.eeg_theta_beta_ratio = 1.2;
    }
    
    if (scopes.includes('presence')) {
      data.presence_confidence = 0.94;
    }

    return data;
  }

  async getCurrentRoH() {
    return 0.18;
  }

  // Chat command handler
  async handleChatCommand(text) {
    const lower = text.toLowerCase();
    
    if (lower.includes('volume up') || lower.includes('louder')) {
      // Check gesture consent
      const allowed = await this.checkTelemetryPermission('chat-agent', ['audio_control']);
      if (allowed.allowed) {
        window.audioContext?.destination.gain.setValueAtTime(0.8, window.audioContext.currentTime);
        return 'Volume increased (host-verified)';
      }
    }
    
    if (lower.includes('show identity')) {
      const token = await this.createIdentityToken();
      return `Host: ${this.hostDid.slice(0,12)}... Token: ${token.slice(0,20)}...`;
    }

    return null;
  }
}

window.AugmentedChatProtocol = AugmentedChatProtocol;
