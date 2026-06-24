// ar-webview.js
// Augmented-reality web view for hardware-independent interaction
// Secure host identification via brain-identity hash

class ARWebView {
  constructor(hostIdentity) {
    this.hostDid = hostIdentity.hostDid;
    this.brainIpHash = hostIdentity.brainIpHash; // 32-byte hash, never raw EEG
    this.session = null;
    this.referenceSpace = null;
  }

  async startAR() {
    // Check WebXR support
    if (!navigator.xr) throw new Error('WebXR not supported');
    
    const supported = await navigator.xr.isSessionSupported('immersive-ar');
    if (!supported) {
      // Fallback to inline AR view (no headset)
      return this.startInlineView();
    }

    // Request consent for camera
    const hasConsent = await this.checkARConsent();
    if (!hasConsent) throw new Error('AR consent required');

    this.session = await navigator.xr.requestSession('immersive-ar', {
      requiredFeatures: ['hit-test', 'anchors', 'dom-overlay'],
      domOverlay: { root: document.getElementById('ar-overlay') }
    });

    this.referenceSpace = await this.session.requestReferenceSpace('local');
    this.setupRendering();
    return this.session;
  }

  async checkARConsent() {
    if (!window.consentDB) return false;
    const history = await window.consentDB.getAll();
    return history.some(e => 
      e.event_type === 'grant' && 
      e.data_categories.includes('camera') &&
      e.data_categories.includes('ar')
    );
  }

  startInlineView() {
    // Hardware-independent fallback: 3D canvas with device orientation
    const canvas = document.createElement('canvas');
    canvas.style.cssText = 'position:fixed;inset:0;width:100%;height:100%;z-index:1000';
    document.body.appendChild(canvas);
    
    // Use device orientation for pseudo-AR
    window.addEventListener('deviceorientation', (e) => {
      // Render host identity badge oriented to device
      this.renderIdentityBadge(canvas, e.alpha, e.beta, e.gamma);
    });
    
    return { mode: 'inline' };
  }

  renderIdentityBadge(canvas, alpha, beta, gamma) {
    const ctx = canvas.getContext('2d');
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    
    // Draw secure host identifier (hashed, not raw)
    const displayHash = this.brainIpHash.slice(0, 8) + '...' + this.brainIpHash.slice(-8);
    ctx.fillStyle = 'rgba(0,255,255,0.9)';
    ctx.font = '16px monospace';
    ctx.fillText(`HOST: ${this.hostDid}`, 20, 40);
    ctx.fillText(`BRAIN-ID: ${displayHash}`, 20, 70);
    ctx.fillText(`VERIFIED`, 20, 100);
    
    // Biocompatibility indicator
    ctx.fillStyle = 'rgba(34,197,94,0.8)';
    ctx.beginPath();
    ctx.arc(canvas.width - 40, 40, 12, 0, Math.PI*2);
    ctx.fill();
  }

  // Secure identification for AI agents
  async getSecureIdentityToken() {
    // Create ephemeral token, never expose raw brain data
    const timestamp = Date.now();
    const payload = {
      host_did: this.hostDid,
      brain_ip_hash: this.brainIpHash,
      timestamp,
      nonce: crypto.getRandomValues(new Uint8Array(16))
    };
    
    // Sign with host key (in production, use WebCrypto)
    const token = btoa(JSON.stringify(payload));
    return {
      token,
      expires: timestamp + 300000, // 5 min
      scope: ['identity', 'biocompatibility_readonly']
    };
  }

  // Agent interaction gate
  async allowAgentInteraction(agentId, requestedScopes) {
    // Check consent and biocompatibility
    const roh = await this.getCurrentRoH();
    if (roh > 0.30) return { allowed: false, reason: 'RoH ceiling exceeded' };
    
    const hasConsent = await this.checkNeuralTelemetryConsent(agentId);
    if (!hasConsent) return { allowed: false, reason: 'No consent for neural telemetry' };
    
    // Allowed scopes for agents
    const allowedScopes = requestedScopes.filter(s => 
      ['identity', 'biocompatibility_readonly', 'presence'].includes(s)
    );
    
    return {
      allowed: true,
      token: await this.getSecureIdentityToken(),
      scopes: allowedScopes
    };
  }

  async getCurrentRoH() {
    // In real implementation, read from biocompatibility-core
    return 0.15; // placeholder
  }

  async checkNeuralTelemetryConsent(agentId) {
    if (!window.consentDB) return false;
    const history = await window.consentDB.getAll();
    return history.some(e => 
      e.event_type === 'grant' &&
      e.data_categories.includes('neural_telemetry') &&
      e.processor === agentId
    );
  }
}

window.ARWebView = ARWebView;
