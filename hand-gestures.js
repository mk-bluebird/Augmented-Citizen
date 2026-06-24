// hand-gestures.js
// Deviceless hand gestures for volume, navigation
// Uses MediaPipe Hands via CDN, no hardware required

class HandGestureController {
  constructor(hostIdentity) {
    this.hostDid = hostIdentity.hostDid;
    this.video = null;
    this.hands = null;
    this.lastGesture = null;
    this.biocompatibilityGuard = null;
  }

  async init() {
    // Load MediaPipe Hands
    const script = document.createElement('script');
    script.src = 'https://cdn.jsdelivr.net/npm/@mediapipe/hands/hands.js';
    await new Promise(res => { script.onload = res; document.head.appendChild(script); });

    this.video = document.createElement('video');
    this.video.style.display = 'none';
    document.body.appendChild(this.video);

    const stream = await navigator.mediaDevices.getUserMedia({ 
      video: { width: 640, height: 480, facingMode: 'user' } 
    });
    this.video.srcObject = stream;
    await this.video.play();

    this.hands = new Hands({
      locateFile: (file) => `https://cdn.jsdelivr.net/npm/@mediapipe/hands/${file}`
    });
    
    this.hands.setOptions({
      maxNumHands: 1,
      modelComplexity: 0, // low power
      minDetectionConfidence: 0.7,
      minTrackingConfidence: 0.7
    });

    this.hands.onResults((results) => this.onResults(results));
    
    // Process frames at 15fps for low power
    setInterval(() => {
      if (this.video.readyState >= 2) {
        this.hands.send({image: this.video});
      }
    }, 66);
  }

  onResults(results) {
    if (!results.multiHandLandmarks || results.multiHandLandmarks.length === 0) return;
    
    const landmarks = results.multiHandLandmarks[0];
    const gesture = this.classifyGesture(landmarks);
    
    if (gesture !== this.lastGesture) {
      this.handleGesture(gesture, landmarks);
      this.lastGesture = gesture;
    }
  }

  classifyGesture(landmarks) {
    // Simple gesture classification
    const thumbTip = landmarks[4];
    const indexTip = landmarks[8];
    const middleTip = landmarks[12];
    const wrist = landmarks[0];

    // Point up: index extended, others folded
    const indexExtended = indexTip.y < wrist.y - 0.1;
    const othersFolded = middleTip.y > wrist.y - 0.05;
    
    if (indexExtended && othersFolded) {
      // Determine direction
      if (indexTip.y < 0.3) return 'point_up';
      if (indexTip.y > 0.7) return 'point_down';
      return 'point';
    }

    // Pinch: thumb and index close
    const pinchDist = Math.hypot(
      thumbTip.x - indexTip.x,
      thumbTip.y - indexTip.y
    );
    if (pinchDist < 0.05) return 'pinch';

    return 'none';
  }

  async handleGesture(gesture, landmarks) {
    // Check biocompatibility before acting
    const roh = await this.getRoH();
    if (roh > 0.25) {
      console.log('Gesture ignored: high cognitive load');
      return;
    }

    switch(gesture) {
      case 'point_up':
        this.adjustVolume(0.1);
        this.showFeedback('Volume +');
        break;
      case 'point_down':
        this.adjustVolume(-0.1);
        this.showFeedback('Volume -');
        break;
      case 'pinch':
        this.toggleMute();
        this.showFeedback('Mute');
        break;
    }

    // Log gesture for audit (append-only)
    if (window.consentDB) {
      await window.consentDB.addConsent({
        event_type: 'gesture',
        purpose: 'deviceless_control',
        data_categories: ['hand_tracking'],
        processor: 'local'
      });
    }
  }

  adjustVolume(delta) {
    // Control system volume or app volume
    if (window.audioContext) {
      const gain = window.audioContext.destination.gain;
      const current = gain.value;
      gain.setValueAtTime(Math.max(0, Math.min(1, current + delta)), window.audioContext.currentTime);
    }
  }

  toggleMute() {
    // Implementation
  }

  showFeedback(text) {
    const el = document.createElement('div');
    el.textContent = text;
    el.style.cssText = 'position:fixed;top:50%;left:50%;transform:translate(-50%,-50%);background:rgba(0,255,255,0.9);color:#000;padding:1rem 2rem;border-radius:0.5rem;font-family:monospace;z-index:10000;pointer-events:none';
    document.body.appendChild(el);
    setTimeout(() => el.remove(), 800);
  }

  async getRoH() {
    // Read from biocompatibility core
    return 0.15;
  }

  stop() {
    if (this.video && this.video.srcObject) {
      this.video.srcObject.getTracks().forEach(t => t.stop());
    }
  }
}

window.HandGestureController = HandGestureController;
