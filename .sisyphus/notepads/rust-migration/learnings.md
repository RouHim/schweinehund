# Learnings - Rust Migration

## Conventions & Patterns


## Task 7: Frontend Static Files

### Status: COMPLETE - No Changes Needed

All static files are already in place and correctly configured:

**Files Verified:**
- ✓ `backend/static/index.html` - Main entry point with HTMX navigation
- ✓ `backend/static/partials/today.html` - Today view with Alpine.js
- ✓ `backend/static/partials/tasks.html` - Tasks management view
- ✓ `backend/static/partials/zones.html` - Zones management view
- ✓ `backend/static/sw.js` - Service Worker with cache strategy
- ✓ `backend/static/manifest.json` - PWA manifest
- ✓ `backend/static/css/main.css` - Stylesheet (834 lines)
- ✓ `backend/static/js/app.js` - Alpine.js app state
- ✓ `backend/static/js/main.js` - API helper functions

**API Paths Verified:**
All fetch URLs use the same paths as Go backend:
- `/api/tasks` - Task CRUD
- `/api/zones` - Zone CRUD
- `/partials/{name}.html` - Partial templates
- `/api/health` - Health check (referenced in plan)

**Key Observations:**
1. Frontend uses HTMX for navigation (hx-get="/partials/...")
2. Alpine.js handles state and interactivity
3. Service Worker implements offline-first caching strategy
4. API paths are already compatible with Rust backend (no changes needed)
5. Static assets are embedded in `backend/static/` directory
6. No HTML structure or Alpine.js logic modifications required

**Conclusion:**
The frontend is ready for the Rust backend. No fetch URL updates needed because the Rust backend maintains API compatibility with the Go backend.

## Task 1: Initialize Rust Crate with Dependencies

### Status: COMPLETE

**Cargo.toml Structure:**
- Edition: 2021
- Package name: `schweinehund-backend`
- All 13 dependencies added successfully

**Dependencies Added:**
- `axum` (0.7) - HTTP framework with routing, middleware, extractors
- `tokio` (1, full features) - Async runtime with all utilities
- `serde` + `serde_json` - JSON serialization/deserialization
- `tracing` + `tracing-subscriber` - Structured logging
- `chrono` (with serde feature) - Timezone-aware timestamps
- `chrono-tz` (0.8) - Europe/Berlin timezone support
- `include_dir` (0.7) - Asset embedding for static files
- `reqwest` (0.11, rustls-tls, no default-features) - HTTP client for ntfy
- `thiserror` (1) - Error handling macros
- `uuid` (1, v4 + serde features) - ID generation

**Module Structure (Flat):**
- `main.rs` - Entry point with `#[tokio::main]`
- `config.rs` - Configuration
- `state.rs` - Application state
- `storage.rs` - Data storage layer
- `models.rs` - Data models
- `error.rs` - Error types
- `handlers_task.rs` - Task HTTP handlers
- `handlers_zone.rs` - Zone HTTP handlers
- `handlers_static.rs` - Static file serving
- `scheduler.rs` - Background task scheduling
- `ntfy.rs` - ntfy.sh integration

**Build Verification:**
- `cargo build` completed successfully in 38.46s
- All dependencies compiled without errors
- No warnings or issues

**Key Decisions:**
1. Used `rustls-tls` for reqwest instead of openssl (lighter, no system deps)
2. Flat module structure (all in `src/*.rs`) for simplicity
3. Full tokio features for maximum flexibility
4. UUID v4 for random ID generation
5. Chrono with serde for JSON timestamp handling

**Next Steps:**
- Task 2: Implement error handling with thiserror
- Task 3: Define data models (Task, Zone, etc.)
- Task 4: Set up application state and configuration

## Task 2: Data Models and JSON Storage

### Status: COMPLETE

**Models:**
- Added `Task` and `Zone` structs in `backend/src/models.rs` with chrono `DateTime<Utc>` timestamps and serde derives.

**Storage:**
- JSON persistence in `backend/src/storage.rs` using `tasks.json` and `zones.json`.
- Atomic write pattern: write temp file, `sync_all`, rename, then `sync_all` directory.
- Load returns empty vectors on missing files; save creates data directory if missing.

**Tests:**
- `test_save_and_load_tasks`, `test_save_and_load_zones`, `test_atomic_write_creates_file`, `test_missing_file_returns_empty`.

**Verification Notes:**
- `cargo test storage -- --nocapture` passes.
- `lsp_diagnostics` still fails to initialize despite `rustup component add rust-analyzer`.

## [2026-01-31] Task 8: ntfy Client + Config

### Status: COMPLETE

- `backend/src/config.rs` provides `ntfy_url()` reading `NTFY_URL` (default `http://ntfy:80/schweinehund`).
- `backend/src/ntfy.rs` implements best-effort `NtfyClient::send()` using reqwest (rustls) with `Title`/`Priority`/`Tags` headers.
- Tests cover: config default/override, send request headers via a minimal tokio TCP server, and connection-error best-effort behavior.

## [2026-01-31] Task 5: Health + Partials Handlers

### Status: COMPLETE

- `backend/src/handlers_static.rs` now has `health_handler()` returning `{ "status": "ok" }`.
- `partials_handler()` validates names (only `a-z0-9-`, optional `.html`, no slashes) and serves `partials/<name>.html` from `include_dir::Dir` (injected via `axum::Extension`).
- Added unit tests for partial name validation (path traversal + invalid chars).

## [2026-01-31] Task 3: Zone Handlers + AppState

### Status: COMPLETE

- `backend/src/state.rs` defines `AppState` with `tokio::RwLock<Vec<Task>>`, `tokio::RwLock<Vec<Zone>>`, and `Storage`, plus `SharedState = Arc<AppState>`.
- `backend/src/handlers_zone.rs` implements zone CRUD handlers with JSON `{ "items": [...] }` list wrapper and validation error string `name, weekday, and color are required`.
- `GET /api/zones` sorting: weekday ASC, created_at ASC.
- Zone deletion cascades by setting `task.zone_id = None` and persisting both tasks and zones.

## [2026-01-31] Task 4: Task Handlers

### Status: COMPLETE

- `backend/src/handlers_task.rs` implements task CRUD handlers with list wrapper `{ "items": [...] }`.
- API conversion matches frontend expectations:
  - `zone` field (not `zone_id`) is `null` or zone id string.
  - `completed_at` is `""` when unset, otherwise RFC3339 with `Z`.
- `PATCH` supports clearing `completed_at` via empty string and clearing zone via `null`.
- Tests cover create/list and patch `completed_at` parsing + clear.

## [2026-01-31] Task 6: Task Rotation

### Status: COMPLETE

- Implemented non-daily rotation in `backend/src/handlers_task.rs:update_task` when `completed` transitions `false -> true`.
- Rotation sets `sort_order = (max non-daily sort_order) + 1`.
- Added `test_task_rotation_on_completion`.

## [2026-01-31] Task 9: Server Wiring + Embedded Assets

### Status: COMPLETE

- `backend/src/main.rs` now runs an axum server, embeds `backend/static/**` via `include_dir`, and serves `/` + `/*path` from embedded assets with basic content-type mapping.
- Routes wired: `/api/health`, `/api/tasks`, `/api/tasks/:id`, `/api/zones`, `/api/zones/:id`, `/partials/:name`.
- Admin trigger endpoints stubbed (204): `/api/admin/cron/daily`, `/api/admin/cron/weekly`.
- Startup notification sent best-effort via `NtfyClient`.
- Smoke tested: `curl /api/health`, `/api/tasks`, `/api/zones`, and `/`.

## [2026-01-31] Task 11: Dockerfile

### Status: COMPLETE

- `backend/Dockerfile` builds a musl-targeted static binary using `ghcr.io/rust-cross/rust-musl-cross` (via build arg `MUSL_IMAGE`).
- Build target configurable via `TARGET_TRIPLE` (default `x86_64-unknown-linux-musl`).

## [2026-01-31] Task 10: Scheduler

### Status: COMPLETE

- `backend/src/scheduler.rs` schedules:
  - daily reminder at 09:00 Europe/Berlin
  - weekly reset at Monday 00:00 Europe/Berlin
- Added env flags `RUN_DAILY_ON_START` / `RUN_WEEKLY_ON_START`.
- Admin endpoints now execute the scheduler routines and return 204.

## [2026-01-31] Task 12: docker-compose

### Status: COMPLETE

- `docker-compose.yml` now runs `backend` built from `./backend` with env `DATA_DIR=/app/data` and `NTFY_URL=http://ntfy:80/schweinehund`.
- `ntfy` service left unchanged.

## [2026-01-31] Task 13: Playwright E2E

### Status: COMPLETE

- `podman-compose up -d --build` boots `backend` + `ntfy`.
- `npm test` (Playwright) passes: 47/47.

## [2026-01-31] Task 14: Remove Go Backend Artifacts

### Status: COMPLETE

- Deleted Go backend files in `backend/*.go` and `backend/go.mod` + `backend/go.sum`.
- Updated `.gitignore` to ignore Rust build outputs and runtime data directories.
- Verified: `cd backend && cargo build --release` and `cd backend && cargo test`.
