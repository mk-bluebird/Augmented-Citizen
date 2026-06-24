// Integration example for Augmented-Citizen PWA
// Replace localStorage consent log with IndexedDB

// In your main app initialization:
async function initConsentSystem() {
  await window.consentDB.open();
  
  // Migrate from localStorage if exists (one-time)
  const oldLog = localStorage.getItem('consent_log');
  if (oldLog) {
    const entries = JSON.parse(oldLog);
    for (const entry of entries) {
      await window.consentDB.addConsent({
        event_type: entry.type,
        purpose: entry.purpose,
        timestamp: entry.ts
      });
    }
    localStorage.removeItem('consent_log');
  }
  
  // Update UI with count
  const count = await window.consentDB.getCount();
  document.getElementById('consent-count').textContent = count;
}

// Grant consent button handler
async function grantConsent(purpose, categories) {
  const entry = await window.consentDB.addConsent({
    event_type: 'grant',
    purpose: purpose,
    data_categories: categories,
    bostrom_address: document.getElementById('bostrom-addr').value
  });
  
  // Update UI: C_n = previous + 1
  console.log(`Consent C_${entry.c_n} recorded`);
  updateConsentDisplay();
}

// Revoke consent (append-only, points to original)
async function revokeConsent(originalCn, reason) {
  const entry = await window.consentDB.addConsent({
    event_type: 'revoke',
    purpose: 'revocation',
    revokes_c_n: originalCn,
    data_categories: ['revocation:' + reason]
  });
  console.log(`Revocation C_${entry.c_n} recorded, revokes C_${originalCn}`);
}

// Export for GDPR
async function exportConsentHistory() {
  const json = await window.consentDB.exportJSON();
  const blob = new Blob([json], {type: 'application/json'});
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = `consent-export-${new Date().toISOString().split('T')[0]}.json`;
  a.click();
}

// Verify integrity (for audit)
async function verifyConsentLog() {
  const result = await window.consentDB.verifyIntegrity();
  if (result.valid) {
    console.log(`✓ Consent log intact: ${result.count} entries, C_1 to C_${result.count}`);
  } else {
    console.error('✗ Integrity failure:', result.error);
  }
  return result;
}

// Background sync handler (in service worker)
self.addEventListener('sync', event => {
  if (event.tag === 'sync-consent') {
    event.waitUntil(
      // Open IndexedDB from SW context
      new Promise(async (resolve) => {
        const db = await new Promise((res, rej) => {
          const req = indexedDB.open('augmented-citizen-consent', 1);
          req.onsuccess = () => res(req.result);
          req.onerror = () => rej(req.error);
        });
        
        const tx = db.transaction('consent_log', 'readonly');
        const store = tx.objectStore('consent_log');
        const all = await new Promise(res => {
          const req = store.getAll();
          req.onsuccess = () => res(req.result);
        });
        
        // In production: POST to your server
        // fetch('/api/consent/sync', {method:'POST', body:JSON.stringify(all)})
        console.log('Would sync', all.length, 'consent entries');
        resolve();
      })
    );
  }
});
