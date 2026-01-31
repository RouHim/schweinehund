# Learnings - Schweinehund

## Wave 1: Project Setup & Docker Structure

### ✅ Successful Patterns

**Directory-First Approach**: Created complete subdirectory tree before docker-compose configuration. This prevented file reorganization later and made `.gitignore` patterns obvious.

**Certificate Handling**: mkcert generates local HTTPS certs without requiring system trust installation. Valid until May 2028. Correctly ignored in `.gitignore` prevents accidental secret leaks.

**Compose v3.9 + Health Checks**: Used `condition: service_healthy` for Caddy→PocketBase dependency. All services have curl-based health checks. Named network `schweinehund-net` handles inter-service communication cleanly.

**Caddy Reverse Proxy**: HTTPS on non-standard port 8080 (avoids sudo locally). Routes for `/api/*` and `/_/*` to PocketBase. HTTP→HTTPS redirect configured. Fallback reverse proxy catches unmapped requests.

**Frontend Scaffold**: HTMX + Alpine.js via CDN (no build required initially). Service Worker implements offline caching. Manifest.json prepared for PWA. CSS uses custom properties for theming. Main.js provides Service Worker registration + basic `apiCall()` helper.

**Git Strategy**: Single atomic commit for entire setup. Conventional semantic style (feat:, fix:, chore:). Detailed commit body explains all components. Clean git history from start.

### ⚠️ Gotchas Avoided

- ❌ Hardcoding PocketBase to external port 8080 (would block reverse proxy)
- ❌ Complex build pipelines (frontend stays simple until needed)
- ❌ Starting containers without validation (only config checked)
- ❌ Cloud/external certificates (local-only for development)
- ❌ Unnecessary code comments ("Basic stylesheet" removed)

### 📋 Configuration Decisions Reference

| Component | Choice | Why |
|-----------|--------|-----|
| PocketBase Image | `ghcr.io/pocketbase/pocketbase:latest` | Official, regular updates |
| Caddy Image | `caddy:2-alpine` | Lightweight, built-in HTTPS, Alpine |
| ntfy Image | `binwiederhier/ntfy:latest` | Standard container |
| Compose Version | 3.9 | Supports service health conditions |
| Network | bridge | Safe default for single-host |
| HTTPS Port | 8080 | Non-standard avoids sudo locally |

### 🔍 Architecture Notes

**Service Dependencies**: PocketBase→Caddy (Caddy waits for PocketBase health). ntfy independent (separate notification service).

**Volume Strategy**: Bind mounts for code (`./frontend`, `./pocketbase/pb_hooks`). Named volumes for container state (`caddy_data`, `caddy_config`). Data directory (`pb_data`) will be created by PocketBase on first run.

**Frontend Without Build Step**: HTMX for server-side rendering. Alpine.js for lightweight interactivity. Service Worker for offline support. API layer via simple `apiCall()` function.

**Port Mapping**:
- PocketBase: 8090 (internal only, via reverse proxy)
- Caddy: 80, 443, 8080 (external)
- ntfy: 8091 (external, separate)

### 🚀 Next Wave Preparation

1. **PocketBase Collections**: Currently none. Need schema + migration scripts in `pb_migrations/`
2. **Frontend Assets**: Icon placeholders at `frontend/assets/icon-*.png` (PWA requires 192x192 + 512x512)
3. **ntfy Configuration**: Using defaults. Custom `ntfy/server.yml` for production setup
4. **Environment Variables**: No `.env` yet. TrueNAS deployment will inject via Docker app config

### 🧪 Validation Results

✅ `docker compose config` - Valid YAML syntax  
✅ All directories created correctly  
✅ mkcert generated valid certificates (expires 2028-05-01)  
✅ Git initialized with single clean commit (28576a5)  
✅ No secrets in version control  
✅ 437 lines total addition (lean)

## Conventions & Patterns

- **Commit Style**: Semantic (feat:, fix:, chore:) in English
- **File Organization**: Feature-based directories (frontend/, pocketbase/, ntfy/)
- **Certificate Management**: Local mkcert for dev, ignored in git
- **Relative Paths**: All volume paths relative (./...) for TrueNAS portability
- **Health Checks**: curl-based for HTTP services, standard 10s intervals

## Architecture Decisions

- **Single Compose File**: All services in one docker-compose.yml (monorepo pattern)
- **Named Network**: Use bridge network for service discovery (DNS)
- **HTTPS on 8080**: Non-standard port for local development (avoids root requirements)
- **Caddy for Reverse Proxy**: Superior to nginx for cert handling + HTTP redirect
- **No Build Pipeline**: Frontend runs vanilla HTML/CSS/JS + HTMX/Alpine CDN (zero build overhead)
- **Service Worker**: Offline-first PWA strategy (cache then network)

## Task 4: Service Worker & PWA Manifest

### ✅ Implementation Decisions

**PWA Manifest Configuration**:
- Theme color: `#FF7F50` (warm orange) - matches logo primary color
- Background color: `#FFF5E6` (cream) - soft, readable background
- Display mode: `standalone` - full-screen app experience
- Icon purpose: `any maskable` - ensures icons work on all Android launchers
- Categories: `productivity`, `lifestyle` - improves discoverability in app stores

**Service Worker Caching Strategy**:
- **Static Cache on Install**: Pre-caches essential assets (HTML, CSS, JS, icons) during service worker installation for immediate offline availability
- **Network-First for API**: `/api/*` and `/_/*` endpoints fetch from network, graceful 503 JSON error when offline
- **Skip Realtime Endpoints**: `/api/realtime` explicitly bypassed (PocketBase SSE requires live connection)
- **Cache-First for Assets**: Static resources served from cache, fallback to network if missing
- **Selective Caching**: Only caches `basic` type responses with valid destinations (document/script/style/image)

**Offline Fallback Pattern**:
- Failed fetch requests fall back to cached `/index.html`
- API requests return structured JSON error: `{"error": "Offline - API unavailable"}`
- Service Worker activates immediately on install (`skipWaiting()` + `clients.claim()`)

### 🔍 Technical Details

**Static Assets Cached**:
```javascript
[
  '/', '/index.html', '/manifest.json',
  '/css/main.css', '/js/main.js',
  '/assets/logo.svg', '/assets/icon-192.png', 
  '/assets/icon-512.png', '/assets/favicon.ico'
]
```

**Realtime Endpoint Detection**:
```javascript
if (url.pathname.includes('/api/realtime')) {
  return; // Skip service worker, let browser handle
}
```

**Cache Invalidation**:
- Old caches automatically deleted on service worker activation
- Version bump in `CACHE_NAME` triggers full cache refresh
- Active service worker claims all clients immediately

### ⚠️ Gotchas Avoided

- ❌ Caching SSE/realtime endpoints (would break PocketBase subscriptions)
- ❌ Overly aggressive caching (API requests always try network first)
- ❌ Missing `purpose: "any maskable"` (icons would clip on some Android devices)
- ❌ Generic offline fallback (API returns structured JSON errors)
- ❌ Stale cache retention (old caches deleted on activation)

### 📋 Integration Points

- **index.html**: Must register service worker in main.js
- **PocketBase Realtime**: Service worker skips `/api/realtime` to preserve SSE connections
- **Icon Assets**: manifest.json references icons created in Task 12
- **Theme Colors**: Matches design palette from logo (orange/cream)

### 🧪 Validation Results

✅ `cat frontend/manifest.json | jq '.name'` → "Schweinehund"  
✅ `node -c frontend/sw.js` → Syntax valid  
✅ Theme color: `#FF7F50`, Background: `#FFF5E6`  
✅ Icons: 192px and 512px PNGs with maskable purpose  
✅ Realtime endpoint skip logic confirmed
