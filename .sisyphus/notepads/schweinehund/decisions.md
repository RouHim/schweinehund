# Decisions - Schweinehund

## Technical Choices
(To be populated by subagents)

## Task 2: ntfy.sh Container Setup

### Configuration Choices

#### ntfy Docker Image & Ports
- **Image**: `binwiederhier/ntfy:latest` (official, actively maintained)
- **Port Mapping**: 8090:80 (external:internal)
  - Rationale: Local network use, no need for HTTPS on ntfy service itself
  - HTTPS termination handled by Caddy at the reverse proxy level

#### Authentication & Access Control
- **Auth Setting**: `auth-default-access=read-write`
  - Rationale: Local network only, no external exposure
  - No user authentication needed for simple household task notifications
  - All topics can be read and written without credentials

#### Data Persistence
- **Cache Volume**: `./ntfy/cache:/var/lib/ntfy/cache`
  - Rationale: Persists notification history across container restarts
  - Cache duration: 168 hours (1 week) - sufficient for task history

#### Environment Configuration
```yaml
NTFY_BASE_URL: http://localhost:8090
NTFY_CACHE_FILE: /var/lib/ntfy/cache/cache.db
NTFY_WEB_ROOT: app (enables web UI)
```

### Docker Compose Integration
- **Network**: `schweinehund-net` (shared with PocketBase, Caddy)
- **Restart Policy**: `unless-stopped` (survives accidental stops)
- **Health Check**: Configured with wget polling
- **No Dependencies**: ntfy is independent service (PocketBase can publish to it)

### Android Client Setup
- Direct documentation: `ntfy/ANDROID_SETUP.md`
- Manual subscription to `schweinehund` topic on local server
- Supports both hostname (`schweinehund.local:8090`) and IP address

### Test Results
- ✅ Container starts successfully: `docker compose up ntfy --no-deps -d`
- ✅ Health endpoint responds: `{"healthy":true}` from `/v1/health`
- ✅ Topic publishing works: Successful POST to `http://localhost:8090/schweinehund`
- ✅ Web UI accessible: HTML response from `http://localhost:8090/`

### Future Integration Points
- **PocketBase Hooks**: Will publish notifications via curl to `http://ntfy:8090/schweinehund`
- **Scheduled Tasks**: Cron jobs for daily 09:00 reminder, weekly reset
- **Event Triggers**: Task completion, zone changes, reset cycles

### Known Limitations & Notes
- No HTTPS between PocketBase and ntfy (same Docker network)
- Local network only - firewall will block external access (intentional)
- Rate limiting: 60 burst, 5s replenish (sufficient for household use)

## Task 12: Schweinehund Logo & PWA Icons

### Design Concept
- **Character**: Tired/lazy dog ("Schweinehund") with broom, inspired by Duolingo's character design philosophy
- **Style**: Flat vector graphics, simple and friendly
- **Color Palette**:
  - Orange (#FF7F50) - Primary body color, energetic warmth
  - Pink (#FF6B9D) - Accents (tongue, cheeks), approachability
  - Turquoise (#40E0D0) - Broom bristles, contrast and cleanliness

### Asset Creation Approach
- **SVG Master File**: `frontend/assets/logo.svg` - scalable source
- **PNG Icons**: Generated from SVG using `rsvg-convert`
  - icon-192.png: PWA manifest (iOS/Android home screen)
  - icon-512.png: Splash screens and large displays
- **Favicon**: favicon.ico converted from PNG, supports multiple sizes

### Technical Implementation
- **SVG Design Elements**:
  - Ellipse-based body (organic shapes)
  - Sleepy eye marks (horizontal curves) - conveys lazy mood
  - Lazy smile and pink tongue - friendly demeanor
  - Four legs with perspective (opacity on back legs)
  - Simple curved tail
  - Broom with bristles using turquoise fill

- **Image Conversion Process**:
  - `rsvg-convert` (GNOME): Converts SVG → PNG with high fidelity
  - ImageMagick `convert`: PNG → favicon.ico with multi-resolution support
  - 192x192 and 512x512 PNGs scale well on mobile devices
  - Multi-resolution favicon ensures browser compatibility

### PWA Integration Points
- **manifest.json**: Icons linked for app installation
- **HTML head**: Favicon link for browser tab
- **Service Worker**: Can cache icons for offline access
- **Splash Screen**: 512x512 used on app launch (iOS/Android)

### Files Created
```
frontend/assets/
├── logo.svg         (2.5 KB) - Master SVG vector
├── icon-192.png     (6.1 KB) - Mobile home screen icon
├── icon-512.png     (18 KB)  - Splash screen / large display
└── favicon.ico      (298 KB) - Multi-resolution browser favicon
```

### Design Rationale
- **Lazy Dog Concept**: Represents procrastination (typical of task management users) with humor
- **Broom Symbolism**: Connects to "cleaning up" tasks, completing work
- **Flat Design**: Matches modern PWA aesthetic, scales cleanly to all sizes
- **Color Theory**: Orange energizes, pink adds personality, turquoise signals completion/progress
