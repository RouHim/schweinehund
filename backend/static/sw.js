const CACHE_NAME = 'schweinehund-v1';
const STATIC_CACHE = [
  '/',
  '/index.html',
  '/manifest.json',
  '/css/main.css',
  '/js/main.js',
  '/assets/logo.svg',
  '/assets/icon-192.png',
  '/assets/icon-512.png',
  '/assets/favicon.ico'
];

self.addEventListener('install', (event) => {
  event.waitUntil(
    caches.open(CACHE_NAME)
      .then((cache) => cache.addAll(STATIC_CACHE))
      .then(() => self.skipWaiting())
  );
});

self.addEventListener('activate', (event) => {
  event.waitUntil(
    caches.keys().then((cacheNames) => {
      return Promise.all(
        cacheNames.filter((name) => name !== CACHE_NAME)
          .map((name) => caches.delete(name))
      );
    }).then(() => self.clients.claim())
  );
});

self.addEventListener('fetch', (event) => {
  const url = new URL(event.request.url);
  
  if (event.request.method !== 'GET') {
    return;
  }

  if (url.pathname.includes('/api/realtime')) {
    return;
  }

  const isApiRequest = url.pathname.startsWith('/api/') || url.pathname.startsWith('/_/');
  
  if (isApiRequest) {
    event.respondWith(
      fetch(event.request)
        .catch(() => {
          return new Response(
            JSON.stringify({ error: 'Offline - API unavailable' }),
            { 
              status: 503,
              headers: { 'Content-Type': 'application/json' }
            }
          );
        })
    );
    return;
  }

  event.respondWith(
    caches.match(event.request)
      .then((cachedResponse) => {
        if (cachedResponse) {
          return cachedResponse;
        }
        
        return fetch(event.request).then((response) => {
          if (!response || response.status !== 200 || response.type === 'error') {
            return response;
          }

          const shouldCache = response.type === 'basic' && 
                              (event.request.destination === 'document' ||
                               event.request.destination === 'script' ||
                               event.request.destination === 'style' ||
                               event.request.destination === 'image');
          
          if (shouldCache) {
            const responseToCache = response.clone();
            caches.open(CACHE_NAME)
              .then((cache) => cache.put(event.request, responseToCache));
          }
          
          return response;
        });
      })
      .catch(() => caches.match('/index.html'))
  );
});
