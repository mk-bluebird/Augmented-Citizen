// geospatial-audio.js
// 3D sound and AR geospatial audio for Augmented-Citizen
// Host-bound, consent-gated, low-power friendly

class GeospatialAudio {
  constructor(hostIdentity) {
    this.hostDid = hostIdentity.hostDid;
    this.audioContext = null;
    this.pannerNodes = new Map();
    this.consentRequired = true;
  }

  async init() {
    // Check consent via IndexedDB
    const hasConsent = await this.checkAudioConsent();
    if (!hasConsent && this.consentRequired) {
      throw new Error('Audio consent required: C_n not found');
    }

    this.audioContext = new (window.AudioContext || window.webkitAudioContext)({
      latencyHint: 'interactive',
      sampleRate: 48000
    });

    // Resume on user gesture for mobile
    if (this.audioContext.state === 'suspended') {
      await this.audioContext.resume();
    }
  }

  async checkAudioConsent() {
    if (!window.consentDB) return false;
    const history = await window.consentDB.getAll();
    return history.some(e => 
      e.event_type === 'grant' && 
      e.data_categories.includes('audio') &&
      e.host_did === this.hostDid
    );
  }

  // Create a spatial audio source at geographic coordinates
  createGeoSource(id, lat, lon, audioUrl) {
    if (!this.audioContext) throw new Error('Not initialized');

    const panner = this.audioContext.createPanner();
    panner.panningModel = 'HRTF';
    panner.distanceModel = 'inverse';
    panner.refDistance = 1;
    panner.maxDistance = 1000;
    panner.rolloffFactor = 1;
    
    // Convert lat/lon to 3D position relative to host
    const pos = this.geoToCartesian(lat, lon);
    panner.positionX.value = pos.x;
    panner.positionY.value = pos.y;
    panner.positionZ.value = pos.z;

    // Load and play
    fetch(audioUrl)
      .then(r => r.arrayBuffer())
      .then(buf => this.audioContext.decodeAudioData(buf))
      .then(audioBuf => {
        const source = this.audioContext.createBufferSource();
        source.buffer = audioBuf;
        source.loop = true;
        source.connect(panner);
        panner.connect(this.audioContext.destination);
        source.start();
        this.pannerNodes.set(id, {panner, source});
      });

    return panner;
  }

  geoToCartesian(lat, lon, hostLat = 33.4484, hostLon = -112.0740) {
    // Phoenix default, convert to meters offset
    const R = 6371000;
    const dLat = (lat - hostLat) * Math.PI / 180;
    const dLon = (lon - hostLon) * Math.PI / 180;
    const x = R * dLon * Math.cos(hostLat * Math.PI/180);
    const z = R * dLat;
    return {x, y: 0, z: -z}; // WebAudio: -z is forward
  }

  // Update listener position from device orientation
  updateListener(orientation) {
    if (!this.audioContext) return;
    const listener = this.audioContext.listener;
    if (listener.positionX) {
      listener.positionX.value = 0;
      listener.positionY.value = 1.6; // ear height
      listener.positionZ.value = 0;
      // Forward vector from device orientation
      const q = orientation;
      listener.forwardX.value = 2*(q.x*q.z + q.w*q.y);
      listener.forwardY.value = 2*(q.y*q.z - q.w*q.x);
      listener.forwardZ.value = 1 - 2*(q.x*q.x + q.y*q.y);
    }
  }

  // Biocompatibility guard: reduce volume if RoH high
  applyBiocompatibilityGuard(rohScalar) {
    const volume = rohScalar > 0.25 ? 0.3 : 1.0;
    this.audioContext.destination.gain?.setValueAtTime(volume, this.audioContext.currentTime);
  }
}

// Usage for AI-chat platforms:
// const geoAudio = new GeospatialAudio({hostDid: 'didalnorganic-host'});
// await geoAudio.init();
// geoAudio.createGeoSource('alert1', 33.45, -112.07, '/sounds/beacon.ogg');

window.GeospatialAudio = GeospatialAudio;
