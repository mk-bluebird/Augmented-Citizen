// identity-manager.js
// Autonomous credential management for augmented citizens
// No traditional user accounts, identity derived from host

class AutonomousIdentityManager {
  constructor() {
    this.hostDid = null;
    this.bostromAddress = null;
    this.brainIpHash = null;
  }

  // Create identity from device signals, no signup form
  async createEphemeralIdentity() {
    // 1. Generate or retrieve host DID
    let hostDid = localStorage.getItem('host_did');
    if (!hostDid) {
      hostDid = 'did:aln:' + this.generateId();
      localStorage.setItem('host_did', hostDid);
    }

    // 2. Derive brain-IP hash from stable device characteristics
    // (In production, use WebAuthn or secure enclave)
    const brainIpHash = await this.deriveBrainIpHash();

    // 3. Bind to Bostrom address if available
    const bostrom = localStorage.getItem('bostrom_address') || 
                    'bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7';

    this.hostDid = hostDid;
    this.brainIpHash = brainIpHash;
    this.bostromAddress = bostrom;

    // 4. Create identity record in consent DB
    if (window.consentDB) {
      await window.consentDB.addConsent({
        event_type: 'identity_created',
        purpose: 'autonomous_identity',
        data_categories: ['identity'],
        processor: 'local'
      });
    }

    return {
      host_did: hostDid,
      brain_ip_hash: brainIpHash,
      bostrom_address: bostrom,
      created: Date.now()
    };
  }

  async deriveBrainIpHash() {
    // Combine stable signals to create consistent hash
    // Never store raw biometric data
    const signals = [
      navigator.userAgent,
      navigator.language,
      screen.width + 'x' + screen.height,
      Intl.DateTimeFormat().resolvedOptions().timeZone,
      await this.getStableHardwareId()
    ].join('|');

    const encoder = new TextEncoder();
    const data = encoder.encode(signals);
    const hashBuffer = await crypto.subtle.digest('SHA-256', data);
    const hashArray = Array.from(new Uint8Array(hashBuffer));
    return hashArray.map(b => b.toString(16).padStart(2, '0')).join('');
  }

  async getStableHardwareId() {
    // Use WebAuthn credential as stable identifier
    try {
      const cred = await navigator.credentials.get({
        publicKey: {
          challenge: new Uint8Array(32),
          allowCredentials: [],
          userVerification: 'discouraged'
        }
      });
      return cred ? cred.id : 'fallback-' + Math.random();
    } catch {
      return 'webauthn-unavailable';
    }
  }

  generateId() {
    return Array.from(crypto.getRandomValues(new Uint8Array(16)))
      .map(b => b.toString(16).padStart(2, '0')).join('');
  }

  // Get identity for any platform, no login required
  async getIdentityForPlatform(platform) {
    if (!this.hostDid) await this.createEphemeralIdentity();

    // Create platform-specific derived identity
    const platformHash = await crypto.subtle.digest('SHA-256', 
      new TextEncoder().encode(this.hostDid + '|' + platform)
    );
    const platformId = Array.from(new Uint8Array(platformHash))
      .slice(0, 16).map(b => b.toString(16).padStart(2,'0')).join('');

    return {
      platform_id: platformId,
      host_did: this.hostDid,
      brain_ip_hash: this.brainIpHash,
      bostrom_address: this.bostromAddress,
      // Proof of identity without revealing secrets
      proof: await this.createProof(platform)
    };
  }

  async createProof(platform) {
    const message = this.hostDid + '|' + platform + '|' + Math.floor(Date.now()/300000);
    const encoder = new TextEncoder();
    const data = encoder.encode(message);
    const hash = await crypto.subtle.digest('SHA-256', data);
    return Array.from(new Uint8Array(hash)).map(b => b.toString(16).padStart(2,'0')).join('').slice(0, 32);
  }

  // Verify identity across devices
  async verifyIdentity(identity) {
    // Check brain-IP hash matches
    const currentHash = await this.deriveBrainIpHash();
    return currentHash === identity.brain_ip_hash;
  }
}

window.AutonomousIdentityManager = AutonomousIdentityManager;
window.identityManager = new AutonomousIdentityManager();
