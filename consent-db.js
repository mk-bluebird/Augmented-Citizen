// consent-db.js
// IndexedDB wrapper for append-only consent histories
// Designed for Augmented-Citizen neuro-rights compliance
// Host-bound, no deletions, exportable for GDPR/CCPA

class ConsentDB {
  constructor(hostDid = 'didalnorganic-host') {
    this.dbName = 'augmented-citizen-consent';
    this.version = 1;
    this.storeName = 'consent_log';
    this.hostDid = hostDid;
    this.db = null;
  }

  async open() {
    return new Promise((resolve, reject) => {
      const request = indexedDB.open(this.dbName, this.version);
      
      request.onupgradeneeded = (event) => {
        const db = event.target.result;
        if (!db.objectStoreNames.contains(this.storeName)) {
          const store = db.createObjectStore(this.storeName, {
            keyPath: 'id',
            autoIncrement: true
          });
          // Indexes for querying
          store.createIndex('timestamp', 'timestamp', { unique: false });
          store.createIndex('host_did', 'host_did', { unique: false });
          store.createIndex('event_type', 'event_type', { unique: false });
          store.createIndex('c_n', 'c_n', { unique: true }); // Enforce C_n uniqueness
        }
      };

      request.onsuccess = (event) => {
        this.db = event.target.result;
        resolve(this.db);
      };

      request.onerror = (event) => {
        reject(event.target.error);
      };
    });
  }

  // Append-only: C_n = C_{n-1} + 1
  async addConsent(eventData) {
    if (!this.db) await this.open();
    
    const count = await this.getCount();
    const c_n = count + 1; // Strictly increasing
    
    const entry = {
      c_n,
      timestamp: Date.now(),
      iso_timestamp: new Date().toISOString(),
      host_did: this.hostDid,
      bostrom_address: eventData.bostrom_address || 'bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7',
      event_type: eventData.event_type || 'grant', // grant, revoke, scope_change
      purpose: eventData.purpose || 'neural_data_access',
      data_categories: eventData.data_categories || ['eeg', 'hrv'],
      retention_days: eventData.retention_days || 365,
      processor: eventData.processor || 'local',
      consent_text_hash: eventData.consent_text_hash || null,
      revokes_c_n: eventData.revokes_c_n || null, // For revocations, point to original
      signature: eventData.signature || null, // Optional cryptographic signature
      // Immutable metadata
      user_agent: navigator.userAgent,
      platform: navigator.platform,
      // ALN compliance
      aln_clause: 'ALN.MIGRATION.CYBERCORE_AUTHORITY.v1',
      norollback: true
    };

    return new Promise((resolve, reject) => {
      const tx = this.db.transaction([this.storeName], 'readwrite');
      const store = tx.objectStore(this.storeName);
      const request = store.add(entry);

      request.onsuccess = () => {
        // Trigger background sync if available
        if ('serviceWorker' in navigator && 'SyncManager' in window) {
          navigator.serviceWorker.ready.then(reg => {
            reg.sync.register('sync-consent').catch(() => {});
          });
        }
        resolve(entry);
      };

      request.onerror = () => reject(request.error);
    });
  }

  async getAll(limit = null) {
    if (!this.db) await this.open();
    
    return new Promise((resolve, reject) => {
      const tx = this.db.transaction([this.storeName], 'readonly');
      const store = tx.objectStore(this.storeName);
      const request = store.getAll();

      request.onsuccess = () => {
        let results = request.result;
        // Sort by C_n ascending (immutable order)
        results.sort((a, b) => a.c_n - b.c_n);
        if (limit) results = results.slice(-limit);
        resolve(results);
      };

      request.onerror = () => reject(request.error);
    });
  }

  async getCount() {
    if (!this.db) await this.open();
    
    return new Promise((resolve, reject) => {
      const tx = this.db.transaction([this.storeName], 'readonly');
      const store = tx.objectStore(this.storeName);
      const request = store.count();

      request.onsuccess = () => resolve(request.result);
      request.onerror = () => reject(request.error);
    });
  }

  async getByCn(c_n) {
    if (!this.db) await this.open();
    
    return new Promise((resolve, reject) => {
      const tx = this.db.transaction([this.storeName], 'readonly');
      const store = tx.objectStore(this.storeName);
      const index = store.index('c_n');
      const request = index.get(c_n);

      request.onsuccess = () => resolve(request.result);
      request.onerror = () => reject(request.error);
    });
  }

  // Export for GDPR Article 20 / CCPA
  async exportJSON() {
    const all = await this.getAll();
    const exportData = {
      export_timestamp: new Date().toISOString(),
      host_did: this.hostDid,
      total_entries: all.length,
      aln_compliance: 'ALN.MIGRATION.CYBERCORE_AUTHORITY.v1',
      format_version: '1.0',
      consent_log: all
    };
    return JSON.stringify(exportData, null, 2);
  }

  // Export as CSV for legal review
  async exportCSV() {
    const all = await this.getAll();
    const headers = ['c_n', 'iso_timestamp', 'event_type', 'purpose', 'data_categories', 'host_did'];
    const rows = all.map(e => [
      e.c_n,
      e.iso_timestamp,
      e.event_type,
      e.purpose,
      (e.data_categories || []).join(';'),
      e.host_did
    ]);
    return [headers.join(','), ...rows.map(r => r.join(','))].join('\n');
  }

  // Verify integrity: check C_n sequence
  async verifyIntegrity() {
    const all = await this.getAll();
    for (let i = 0; i < all.length; i++) {
      if (all[i].c_n !== i + 1) {
        return { valid: false, error: `Gap at C_n=${i+1}`, expected: i+1, found: all[i].c_n };
      }
    }
    return { valid: true, count: all.length };
  }

  // NO DELETE METHOD - append-only by design
  // For testing only, not exposed in production builds
  async _clearAllForTesting() {
    if (!this.db) await this.open();
    return new Promise((resolve, reject) => {
      const tx = this.db.transaction([this.storeName], 'readwrite');
      const store = tx.objectStore(this.storeName);
      const request = store.clear();
      request.onsuccess = () => resolve();
      request.onerror = () => reject(request.error);
    });
  }
}

// Global instance for Augmented-Citizen
window.ConsentDB = ConsentDB;
window.consentDB = new ConsentDB();

// Auto-open on load for low-latency
if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', () => window.consentDB.open());
} else {
  window.consentDB.open();
}
