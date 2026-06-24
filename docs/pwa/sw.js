// Augmented-Citizen Service Worker v1.0
// Designed for low-connectivity, low-power, offline-first operation
// Host-bound to didalnorganic-host

const CACHE_NAME = 'augcitizen-v1-2026-06-23';
const OFFLINE_URL = '/';
const CORE_ASSETS = [
  '/',
  '/manifest.json',
  // Core app shell - in production, add your built assets here
];

// Install: cache core shell immediately
self.addEventListener('install', (event) => {
  event.waitUntil(
    caches.open(CACHE_NAME).then((cache) => {
      return cache.addAll(CORE_ASSETS);
    }).then(() => self.skipWaiting())
  );
});

// Activate: clean old caches, claim clients
self.addEventListener('activate', (event) => {
  event.waitUntil(
    caches.keys().then((keys) => {
      return Promise.all(
        keys.filter(k => k !== CACHE_NAME).map(k => caches.delete(k))
      );
    }).then(() => self.clients.claim())
  );
});

// Fetch strategy: Network-first for API, Cache-first for shell, Stale-while-revalidate for assets
self.addEventListener('fetch', (event) => {
  const req = event.request;
  const url = new URL(req.url);

  // Only handle GET
  if (req.method !== 'GET') return;

  // 1. Host-critical data: always try network, fallback to cache, then offline page
  if (url.pathname.startsWith('/api/') || url.pathname.includes('consent')) {
    event.respondWith(
      fetch(req).then((res) => {
        // Update cache in background for offline replay
        const copy = res.clone();
        caches.open(CACHE_NAME).then(c => c.put(req, copy));
        return res;
      }).catch(() => caches.match(req).then(r => r || caches.match(OFFLINE_URL)))
    );
    return;
  }

  // 2. App shell and static assets: cache-first for low-power
  if (CORE_ASSETS.includes(url.pathname) || req.destination === 'document') {
    event.respondWith(
      caches.match(req).then(cached => {
        const fetchPromise = fetch(req).then(networkRes => {
          caches.open(CACHE_NAME).then(c => c.put(req, networkRes.clone()));
          return networkRes;
        }).catch(() => cached);
        return cached || fetchPromise;
      })
    );
    return;
  }

  // 3. Everything else: stale-while-revalidate
  event.respondWith(
    caches.match(req).then(cached => {
      const network = fetch(req).then(res => {
        caches.open(CACHE_NAME).then(c => c.put(req, res.clone()));
        return res;
      }).catch(() => null);
      return cached || network || new Response('Offline', {status: 503});
    })
  );
});

// Background sync for consent logs when back online
self.addEventListener('sync', (event) => {
  if (event.tag === 'sync-consent') {
    event.waitUntil(
      // In real implementation, read IndexedDB/localStorage queue and push to server
      self.clients.matchAll().then(clients => {
        clients.forEach(c => c.postMessage({type: 'SYNC_CONSENT'}));
      })
    );
  }
});

// Periodic sync for OTA checks (low-power friendly: max 1/day)
self.addEventListener('periodicsync', (event) => {
  if (event.tag === 'check-ota') {
    event.waitUntil(
      // Check for non-rollback updates respecting host sovereignty
      fetch('/ota/manifest.json', {cache: 'no-store'}).catch(() => null)
    );
  }
});

// Message handler for host commands
self.addEventListener('message', (event) => {
  if (event.data && event.data.type === 'SKIP_WAITING') {
    self.skipWaiting();
  }
  if (event.data && event.data.type === 'GET_CACHE_STATUS') {
    event.ports[0].postMessage({cache: CACHE_NAME, offlineReady: true});
  }
});
