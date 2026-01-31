# Rust Migration - Schweinehund

## TL;DR

> **Quick Summary**: Rewrite the Go backend in Rust with filesystem-only persistence (JSON files instead of SQLite), keeping the existing HTMX/Alpine web UI intact.
> 
> **Deliverables**:
> - New Rust crate replacing `backend/` Go code
> - JSON file persistence for tasks and zones
> - Embedded static assets serving
> - ntfy notification integration
> - Scheduler for daily reminders + weekly reset
> - Updated docker-compose.yml for Rust binary
> - Updated frontend fetch URLs (if API shapes change)
> - All existing Playwright E2E tests passing
> 
> **Estimated Effort**: Large (3-5 days)
> **Parallel Execution**: YES - 4 waves
> **Critical Path**: Task 1 → Task 2 → Task 3 → Task 6 → Task 9 → Task 11

---

## Context

### Original Request
Migrate the application to Rust with:
- Rust only (no other languages for runtime)
- Filesystem-only persistence (no database)
- Minimal external crates
- Embedded static files
- `tracing` for logging
- No `unwrap`/`expect` in non-test code
- RustTLS for outbound TLS
- TDD with BDD-style tests
- Flat file structure (no nested module directories)

### Interview Summary
**Key Decisions**:
- Contract: Keep existing web UI, but API endpoints/payloads can change (frontend updates allowed)
- Data migration: Fresh start (no import from existing SQLite)
- Keep ntfy integration + scheduler (daily 09:00, weekly Monday 00:00, Europe/Berlin)
- TLS: HTTP only for inbound; rustls for outbound https
- Data directory: `./data` (configurable via `DATA_DIR` env)
- Tests: TDD (cargo test) + Playwright E2E
- Quality gates: `cargo fmt` + `cargo clippy`

**Research Findings**:
- Current Go backend embeds `backend/static/**` and serves CRUD API for tasks/zones
- Oracle recommends: `axum` + `tokio`, JSON files with atomic write, `include_dir` for embedding
- Metis identified: need to match exact API response shapes, implement task rotation logic, handle zone deletion cascade

### Metis Review
**Identified Gaps** (addressed via defaults):
- Filesystem structure: Use separate files (`tasks.json`, `zones.json`) for atomic writes per collection
- ID generation: Match Go's 32-char hex format for compatibility
- Error messages: Match Go's exact error strings for API parity
- JSON strictness: Match Go's `DisallowUnknownFields` behavior
- Concurrent access: Use `RwLock<AppState>` with atomic file writes
- Zone deletion cascade: Implement `SET NULL` behavior manually (match SQLite FK)

---

## Work Objectives

### Core Objective
Replace the Go backend with a Rust HTTP server that serves the embedded web UI, provides REST API for tasks/zones, persists data to JSON files, and runs scheduled notification jobs.

### Concrete Deliverables
- `backend/Cargo.toml` - Rust crate definition
- `backend/src/main.rs` - Entry point, server setup
- `backend/src/config.rs` - Environment variable handling
- `backend/src/state.rs` - Shared application state
- `backend/src/storage.rs` - JSON file persistence with atomic writes
- `backend/src/models.rs` - Task, Zone data structures
- `backend/src/handlers_task.rs` - Task CRUD handlers
- `backend/src/handlers_zone.rs` - Zone CRUD handlers
- `backend/src/handlers_static.rs` - Static file + partials serving
- `backend/src/scheduler.rs` - Daily reminder + weekly reset
- `backend/src/ntfy.rs` - Notification client
- `backend/src/error.rs` - Unified error handling
- `backend/Dockerfile` - Multi-stage Rust build
- Updated `docker-compose.yml`
- Updated frontend fetch URLs (in `backend/static/`)

### Definition of Done
- [x] `cargo build --release` produces a working binary
- [x] `cargo test` passes all unit + integration tests
- [x] `cargo fmt --check` passes
- [x] `cargo clippy -- -D warnings` passes
- [x] `docker compose up -d` starts backend + ntfy services
- [x] `curl http://localhost:8090/api/health` returns `{"status":"ok"}`
- [x] `curl http://localhost:8090/api/tasks` returns `{"items":[...]}`
- [x] `curl http://localhost:8090/api/zones` returns `{"items":[...]}`
- [x] `curl http://localhost:8090/` returns embedded frontend HTML
- [x] `npm test` (Playwright) passes all E2E tests
- [x] Scheduler fires daily/weekly jobs at correct times
- [x] Data persists across container restarts

### Must Have
- REST API: GET/POST/PATCH/DELETE for `/api/tasks` and `/api/zones`
- Health endpoint: `GET /api/health`
- Partials serving: `GET /partials/{name}`
- Admin triggers: `POST /api/admin/cron/daily`, `POST /api/admin/cron/weekly`
- Task rotation on completion (non-daily tasks move to end of sort order)
- Weekly reset (Monday 00:00 Berlin: clear all daily task completions)
- Daily reminder (09:00 Berlin: ntfy notification with today's zone)
- JSON file persistence with atomic writes
- Embedded static file serving
- tracing-based logging

### Must NOT Have (Guardrails)
- DO NOT add SQLite or any database (filesystem only)
- DO NOT add authentication/authorization
- DO NOT add API versioning (keep same paths)
- DO NOT add OpenAPI/Swagger generation
- DO NOT add metrics/telemetry endpoints
- DO NOT add rate limiting
- DO NOT use OpenSSL (rustls only for outbound TLS)
- DO NOT add nested module directories (keep flat `src/*.rs` structure)
- DO NOT add database abstraction layers or traits for "future backends"
- DO NOT add config file parsing (env vars only)
- DO NOT change error message strings (must match Go for API parity)
- DO NOT change response field names (`is_daily`, `completed_at`, `sort_order`)

---

## Verification Strategy (MANDATORY)

### Test Decision
- **Infrastructure exists**: NO (new Rust crate)
- **User wants tests**: YES (TDD)
- **Framework**: Rust `#[cfg(test)]` + `#[tokio::test]` for async

### TDD Workflow

Each TODO follows RED-GREEN-REFACTOR:

**Task Structure:**
1. **RED**: Write failing test first
   - Test file: inline `#[cfg(test)] mod tests { ... }` in each source file
   - Test command: `cargo test`
   - Expected: FAIL (test exists, implementation doesn't)
2. **GREEN**: Implement minimum code to pass
   - Command: `cargo test`
   - Expected: PASS
3. **REFACTOR**: Clean up while keeping green
   - Command: `cargo test`
   - Expected: PASS (still)

### Automated Verification

**For API endpoints** (using Bash curl):
```bash
# Health check
curl -sf http://localhost:8090/api/health | jq -e '.status == "ok"'

# Create task
curl -sf -X POST http://localhost:8090/api/tasks \
  -H "Content-Type: application/json" \
  -d '{"name":"Test","emoji":"T","is_daily":true}' \
  | jq -e '.id != null and .name == "Test"'

# List tasks
curl -sf http://localhost:8090/api/tasks | jq -e '.items | type == "array"'
```

**For Frontend** (using Bash):
```bash
curl -sf http://localhost:8090/ | grep -q "Schweinehund"
curl -sf http://localhost:8090/partials/today | grep -q "todayView"
```

---

## Execution Strategy

### Parallel Execution Waves

```
Wave 1 (Start Immediately - Foundation):
├── Task 1: Initialize Rust crate with dependencies
├── Task 2: Implement data models and JSON persistence
└── Task 7: Copy and update frontend static files

Wave 2 (After models + storage):
├── Task 3: Implement zone handlers
├── Task 4: Implement task handlers
├── Task 5: Implement health + partials handlers
└── Task 6: Implement task rotation logic

Wave 3 (After handlers):
├── Task 8: Implement ntfy client
├── Task 9: Wire up HTTP server with embedded assets
└── Task 10: Implement scheduler

Wave 4 (After server works):
├── Task 11: Create Dockerfile
├── Task 12: Update docker-compose.yml
└── Task 13: Run Playwright E2E tests and fix issues

Wave 5 (Cleanup):
└── Task 14: Remove Go backend artifacts
```

### Dependency Matrix

| Task | Depends On | Blocks | Can Parallelize With |
|------|------------|--------|---------------------|
| 1 | None | 2, 3, 4, 5, 6, 8, 9, 10 | 7 |
| 2 | 1 | 3, 4, 5, 6, 9 | 7 |
| 3 | 2 | 6, 9 | 4, 5, 7, 8 |
| 4 | 2 | 6, 9 | 3, 5, 7, 8 |
| 5 | 2 | 9 | 3, 4, 7, 8 |
| 6 | 3, 4 | 9 | 5, 7, 8 |
| 7 | None | 9 | 1, 2, 3, 4, 5, 6, 8 |
| 8 | 1 | 10 | 2, 3, 4, 5, 6, 7 |
| 9 | 2, 3, 4, 5, 6, 7 | 10, 11, 12, 13 | 8 |
| 10 | 8, 9 | 11 | None |
| 11 | 9 | 12 | 10 |
| 12 | 11 | 13 | None |
| 13 | 12 | 14 | None |
| 14 | 13 | None | None (final) |

---

## TODOs

- [x] 1. Initialize Rust crate with dependencies

  **What to do**:
  - Create `backend/Cargo.toml` with:
    - `axum` - HTTP framework
    - `tokio` (full features) - async runtime
    - `serde`, `serde_json` - JSON serialization
    - `tracing`, `tracing-subscriber` - logging
    - `chrono` (with `serde` feature) - timezone-aware timestamps
    - `chrono-tz` - Europe/Berlin timezone
    - `include_dir` - asset embedding
    - `reqwest` (with `rustls-tls` feature, default-features = false) - HTTP client for ntfy
    - `thiserror` - error handling
    - `uuid` (v4 feature) - ID generation (or implement hex random)
  - Create `backend/src/main.rs` with minimal tokio main
  - Create empty module files: `config.rs`, `state.rs`, `storage.rs`, `models.rs`, `error.rs`, `handlers_task.rs`, `handlers_zone.rs`, `handlers_static.rs`, `scheduler.rs`, `ntfy.rs`
  - Verify: `cargo build` succeeds

  **Must NOT do**:
  - Do NOT add SQLite/diesel/sea-orm or any database crate
  - Do NOT add openssl (use rustls via reqwest feature)
  - Do NOT create nested `src/handlers/` directory

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Straightforward crate scaffolding
  - **Skills**: None required

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Task 7)
  - **Blocks**: Tasks 2, 3, 4, 5, 6, 8, 9, 10
  - **Blocked By**: None

  **References**:
  - Axum docs: https://docs.rs/axum/latest/axum/
  - Current Go dependencies in `backend/go.mod` for feature parity reference

  **Acceptance Criteria**:

  **Automated Verification:**
  ```bash
  test -f backend/Cargo.toml && echo "EXISTS"
  # Assert: Output is "EXISTS"

  cd backend && cargo build 2>&1 | tail -5
  # Assert: Build succeeds (no errors)

  grep "axum" backend/Cargo.toml && grep "tokio" backend/Cargo.toml
  # Assert: Both found
  ```

  **Commit**: YES
  - Message: `feat(backend): initialize Rust crate with dependencies`
  - Files: `backend/Cargo.toml`, `backend/src/*.rs`
  - Pre-commit: `cd backend && cargo build`

---

- [x] 2. Implement data models and JSON persistence

  **What to do**:
  - In `backend/src/models.rs`:
    - Define `Task` struct with fields: id, name, emoji, is_daily, sort_order, completed, completed_at (Option), zone_id (Option), created_at, updated_at
    - Define `Zone` struct with fields: id, name, emoji, weekday, color, created_at, updated_at
    - Derive `Serialize`, `Deserialize`, `Clone`
    - Use `chrono::DateTime<Utc>` for timestamps
  - In `backend/src/storage.rs`:
    - Define `Storage` struct holding file paths
    - Implement `load_tasks()`, `save_tasks()`, `load_zones()`, `save_zones()`
    - Implement atomic write: write to `.tmp` file, fsync, rename
    - Handle missing files gracefully (return empty vec)
    - Create data directory if missing
  - Write tests:
    - `test_save_and_load_tasks` - roundtrip
    - `test_save_and_load_zones` - roundtrip
    - `test_atomic_write_creates_file` - new file creation
    - `test_missing_file_returns_empty` - graceful handling

  **Data File Format**:
  ```json
  // data/tasks.json
  [
    {"id":"abc123...","name":"Dishes","emoji":"D","is_daily":true,"sort_order":0,"completed":false,"completed_at":null,"zone_id":null,"created_at":"2026-01-31T10:00:00Z","updated_at":"2026-01-31T10:00:00Z"}
  ]
  
  // data/zones.json
  [
    {"id":"def456...","name":"Kitchen","emoji":"K","weekday":1,"color":"#FF7F50","created_at":"2026-01-31T10:00:00Z","updated_at":"2026-01-31T10:00:00Z"}
  ]
  ```

  **Must NOT do**:
  - Do NOT use SQLite or any database
  - Do NOT add schema versioning (keep simple for now)

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: Core data layer, requires careful implementation
  - **Skills**: None required

  **Parallelization**:
  - **Can Run In Parallel**: YES (with Task 7)
  - **Parallel Group**: Wave 1
  - **Blocks**: Tasks 3, 4, 5, 6, 9
  - **Blocked By**: Task 1

  **References**:
  - `backend/db.go:13-34` - Go Task and Zone struct definitions
  - Serde JSON docs: https://docs.rs/serde_json/

  **Acceptance Criteria**:

  **TDD - RED phase:**
  ```bash
  cd backend && cargo test storage -- --nocapture 2>&1 | grep -E "FAILED|test result"
  # Assert: Tests fail initially
  ```

  **TDD - GREEN phase:**
  ```bash
  cd backend && cargo test storage -- --nocapture
  # Assert: All storage tests PASS
  ```

  **Commit**: YES
  - Message: `feat(backend): implement data models and JSON file persistence`
  - Files: `backend/src/models.rs`, `backend/src/storage.rs`
  - Pre-commit: `cd backend && cargo test`

---

- [x] 3. Implement zone handlers

  **What to do**:
  - In `backend/src/state.rs`:
    - Define `AppState` struct with `RwLock<Vec<Task>>`, `RwLock<Vec<Zone>>`, storage paths
    - Implement `new(data_dir: &str)` that loads from files
  - In `backend/src/handlers_zone.rs`:
    - `get_zones` - returns `{"items": [...]}`sorted by weekday ASC, created_at ASC
    - `create_zone` - validates required fields (name, weekday, color), generates ID, saves
    - `update_zone` - PATCH semantics (partial update), returns updated zone
    - `delete_zone` - deletes zone AND sets `zone_id = None` on any referencing tasks
  - Write tests for each handler

  **API Response Format** (must match exactly):
  ```json
  // GET /api/zones
  {"items": [{"id":"...","name":"Kitchen","emoji":"K","weekday":1,"color":"#FF7F50"}]}
  
  // POST /api/zones (request)
  {"name":"Kitchen","emoji":"K","weekday":1,"color":"#FF7F50"}
  
  // POST /api/zones (response) - same as single zone object
  {"id":"...","name":"Kitchen","emoji":"K","weekday":1,"color":"#FF7F50"}
  
  // Validation error
  {"error":"name, weekday, and color are required"}
  ```

  **Must NOT do**:
  - Do NOT change response field names
  - Do NOT change error message strings

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: Core API functionality
  - **Skills**: None required

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 4, 5)
  - **Blocks**: Tasks 6, 9
  - **Blocked By**: Task 2

  **References**:
  - `backend/handlers.go:249-353` - Go zone handler implementations
  - `backend/handlers.go:52-65` - API response structs

  **Acceptance Criteria**:

  **TDD:**
  ```bash
  cd backend && cargo test zone -- --nocapture
  # Assert: All zone handler tests PASS
  ```

  **Commit**: YES
  - Message: `feat(backend): implement zone REST API handlers`
  - Files: `backend/src/state.rs`, `backend/src/handlers_zone.rs`
  - Pre-commit: `cd backend && cargo test`

---

- [x] 4. Implement task handlers

  **What to do**:
  - In `backend/src/handlers_task.rs`:
    - `get_tasks` - returns `{"items": [...]}` sorted by sort_order ASC, created_at ASC
    - `create_task` - validates required fields (name, emoji), generates ID, saves
    - `update_task` - PATCH semantics, handles `completed_at` as RFC3339 or empty string to clear
    - `delete_task` - removes task
  - Handle strict JSON parsing (reject unknown fields)
  - Write tests for each handler

  **API Response Format**:
  ```json
  // GET /api/tasks
  {"items": [{"id":"...","name":"Dishes","emoji":"D","is_daily":true,"zone":null,"sort_order":0,"completed":false,"completed_at":""}]}
  
  // Note: "zone" field (not "zone_id") contains zone ID or null
  // Note: "completed_at" is empty string when null, RFC3339 when set
  ```

  **Must NOT do**:
  - Do NOT implement rotation here (that's Task 6)
  - Do NOT change field names (use `zone` not `zone_id` in API response)

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: Core API functionality
  - **Skills**: None required

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 3, 5)
  - **Blocks**: Tasks 6, 9
  - **Blocked By**: Task 2

  **References**:
  - `backend/handlers.go:103-232` - Go task handler implementations
  - `backend/handlers.go:379-400` - `taskToAPI` conversion (note `zone` vs `zone_id`)

  **Acceptance Criteria**:

  **TDD:**
  ```bash
  cd backend && cargo test task -- --nocapture
  # Assert: All task handler tests PASS
  ```

  **Commit**: YES
  - Message: `feat(backend): implement task REST API handlers`
  - Files: `backend/src/handlers_task.rs`
  - Pre-commit: `cd backend && cargo test`

---

- [x] 5. Implement health + partials handlers

  **What to do**:
  - In `backend/src/handlers_static.rs`:
    - `health_handler` - returns `{"status":"ok"}`
    - `partials_handler` - serves `partials/{name}.html` from embedded assets
      - Validate name with regex `^[a-z0-9-]+$` (no path traversal)
      - Accept with or without `.html` extension
      - Return 400 for invalid names, 404 if not found
      - Return `Content-Type: text/html; charset=utf-8`
  - Write tests for path validation

  **Must NOT do**:
  - Do NOT allow path traversal (e.g., `../../../etc/passwd`)

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Simple handlers with validation
  - **Skills**: None required

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 3, 4)
  - **Blocks**: Task 9
  - **Blocked By**: Task 2

  **References**:
  - `backend/handlers.go:434-463` - Go PartialsHandler
  - `backend/main.go:103-106` - healthHandler

  **Acceptance Criteria**:

  **TDD:**
  ```bash
  cd backend && cargo test partials -- --nocapture
  cd backend && cargo test health -- --nocapture
  # Assert: All tests PASS
  ```

  **Commit**: YES
  - Message: `feat(backend): implement health and partials handlers`
  - Files: `backend/src/handlers_static.rs`
  - Pre-commit: `cd backend && cargo test`

---

- [x] 6. Implement task rotation logic

  **What to do**:
  - In `backend/src/handlers_task.rs` (or separate file):
    - When `update_task` changes `completed` from `false` to `true`
    - AND task is NOT daily (`is_daily == false`)
    - Find max `sort_order` among all non-daily tasks
    - Set this task's `sort_order` to `max + 1`
    - Save updated task
  - Write test: `test_task_rotation_on_completion`
    - Create 3 non-daily tasks with sort_order 0, 1, 2
    - Complete the first one
    - Verify its sort_order is now 3

  **Business Logic** (from Go):
  ```
  When non-daily task transitions incomplete→complete:
  1. Find all non-daily tasks
  2. Get max(sort_order) from them
  3. Set completed task's sort_order = max + 1
  4. Save
  ```

  **Must NOT do**:
  - Do NOT rotate daily tasks
  - Do NOT rotate on un-completion (only false→true)

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Small logic addition
  - **Skills**: None required

  **Parallelization**:
  - **Can Run In Parallel**: YES (with Tasks 5, 7, 8)
  - **Parallel Group**: Wave 2
  - **Blocks**: Task 9
  - **Blocked By**: Tasks 3, 4

  **References**:
  - `backend/handlers.go:218-229` - Go rotation trigger in PatchTask
  - `backend/db.go:209-225` - Go RotateTaskToEnd implementation

  **Acceptance Criteria**:

  **TDD:**
  ```bash
  cd backend && cargo test rotation -- --nocapture
  # Assert: Rotation test PASS
  ```

  **Commit**: YES
  - Message: `feat(backend): implement task rotation on completion`
  - Files: `backend/src/handlers_task.rs`
  - Pre-commit: `cd backend && cargo test`

---

- [x] 7. Copy and update frontend static files

  **What to do**:
  - Copy `backend/static/` to a location that will be embedded (keep in `backend/static/`)
  - Update fetch URLs in HTML/JS files:
    - If keeping same API paths, no changes needed
    - If any paths changed, update accordingly
  - Verify partials exist: `today.html`, `tasks.html`, `zones.html`
  - Verify main files: `index.html`, `sw.js`, `manifest.json`, `css/main.css`, `js/app.js`, `js/main.js`

  **Must NOT do**:
  - Do NOT modify HTML structure
  - Do NOT modify Alpine.js logic
  - Do NOT add new assets

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: File inspection, minimal changes if any
  - **Skills**: None required

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 2)
  - **Blocks**: Task 9
  - **Blocked By**: None

  **References**:
  - `backend/static/` - Current embedded assets
  - `frontend/` - Mirror (use backend/static/ as source of truth)

  **Acceptance Criteria**:

  **Automated Verification:**
  ```bash
  test -f backend/static/index.html && echo "EXISTS"
  test -f backend/static/partials/today.html && echo "EXISTS"
  # Assert: Both output "EXISTS"
  ```

  **Commit**: YES (if changes made)
  - Message: `refactor(frontend): update static assets for Rust backend`
  - Files: `backend/static/**`
  - Pre-commit: None

---

- [x] 8. Implement ntfy client

  **What to do**:
  - In `backend/src/ntfy.rs`:
    - Define `NtfyClient` struct with `base_url` and `reqwest::Client`
    - Implement `new(base_url: &str)` - create client with rustls
    - Implement `send(message: &str, title: &str, priority: u8, tags: &str)` - sends POST
    - Log errors but don't fail (best-effort notifications)
  - In `backend/src/config.rs`:
    - Read `NTFY_URL` env var, default to `http://ntfy:80/schweinehund`
  - Write test with mock server or just verify struct creation

  **ntfy Request Format**:
  ```
  POST {base_url}
  Headers:
    Title: {title}
    Priority: {priority}
    Tags: {tags}
  Body: {message}
  ```

  **Must NOT do**:
  - Do NOT use OpenSSL (reqwest with rustls-tls feature only)
  - Do NOT block or fail requests on ntfy errors

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Simple HTTP client wrapper
  - **Skills**: None required

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 3, 4, 5, 6)
  - **Blocks**: Task 10
  - **Blocked By**: Task 1

  **References**:
  - `backend/ntfy.go` - Go ntfy client implementation
  - ntfy docs: https://docs.ntfy.sh/publish/

  **Acceptance Criteria**:

  **TDD:**
  ```bash
  cd backend && cargo test ntfy -- --nocapture
  # Assert: Tests PASS
  ```

  **Commit**: YES
  - Message: `feat(backend): implement ntfy notification client`
  - Files: `backend/src/ntfy.rs`, `backend/src/config.rs`
  - Pre-commit: `cd backend && cargo test`

---

- [x] 9. Wire up HTTP server with embedded assets

  **What to do**:
  - In `backend/src/main.rs`:
    - Use `include_dir!` to embed `static/` directory
    - Initialize tracing subscriber
    - Load config from environment
    - Create `AppState` (loads data from files)
    - Create `NtfyClient`
    - Build axum router with routes:
      - `GET /api/health` → health_handler
      - `GET /api/tasks` → get_tasks
      - `POST /api/tasks` → create_task
      - `PATCH /api/tasks/:id` → update_task
      - `DELETE /api/tasks/:id` → delete_task
      - `GET /api/zones` → get_zones
      - `POST /api/zones` → create_zone
      - `PATCH /api/zones/:id` → update_zone
      - `DELETE /api/zones/:id` → delete_zone
      - `GET /partials/:name` → partials_handler
      - `POST /api/admin/cron/daily` → trigger_daily
      - `POST /api/admin/cron/weekly` → trigger_weekly
      - `GET /*` → serve embedded static files (fallback)
    - Bind to `0.0.0.0:{PORT}` (default 8090)
    - Send startup notification via ntfy
  - In `backend/src/error.rs`:
    - Define `AppError` enum
    - Implement `IntoResponse` for axum
    - Return `{"error": "..."}` JSON for errors

  **Must NOT do**:
  - Do NOT start scheduler here (Task 10)
  - Do NOT use external router libraries

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: Central integration point
  - **Skills**: None required

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 3 (after handlers complete)
  - **Blocks**: Tasks 10, 11, 12, 13
  - **Blocked By**: Tasks 2, 3, 4, 5, 6, 7

  **References**:
  - `backend/main.go:18-82` - Go server setup and routing
  - Axum routing: https://docs.rs/axum/latest/axum/routing/

  **Acceptance Criteria**:

  **Automated Verification:**
  ```bash
  cd backend && cargo build --release
  # Assert: Build succeeds
  
  # Start server in background
  DATA_DIR=/tmp/test-data NTFY_URL=http://localhost:9999 ./target/release/schweinehund &
  PID=$!
  sleep 2
  
  curl -sf http://localhost:8090/api/health | jq -e '.status == "ok"'
  curl -sf http://localhost:8090/ | grep -q "Schweinehund"
  
  kill $PID
  # Assert: All checks pass
  ```

  **Commit**: YES
  - Message: `feat(backend): wire up HTTP server with embedded assets`
  - Files: `backend/src/main.rs`, `backend/src/error.rs`
  - Pre-commit: `cd backend && cargo build`

---

- [x] 10. Implement scheduler

  **What to do**:
  - In `backend/src/scheduler.rs`:
    - Implement `start_scheduler(state, ntfy)` that spawns background tasks
    - Daily reminder: compute next 09:00 Berlin, sleep until then, send notification, repeat
    - Weekly reset: compute next Monday 00:00 Berlin, sleep until then, reset daily tasks, repeat
    - Handle timezone correctly with `chrono-tz`
  - Daily reminder logic:
    - Get current weekday (0=Sunday, 6=Saturday)
    - Find first zone with matching weekday
    - Send ntfy: "Heute ist {emoji} {name} Tag!"
  - Weekly reset logic:
    - Find all tasks where `is_daily == true`
    - Set `completed = false`, `completed_at = None`
    - Save tasks
    - Send ntfy: "Neue Woche! Aufgaben zuruckgesetzt"
  - Support `RUN_DAILY_ON_START` and `RUN_WEEKLY_ON_START` env vars for testing

  **Must NOT do**:
  - Do NOT crash on scheduler errors (log and continue)
  - Do NOT use UTC (use Europe/Berlin)

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: Time/timezone handling is error-prone
  - **Skills**: None required

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 3
  - **Blocks**: Task 11
  - **Blocked By**: Tasks 8, 9

  **References**:
  - `backend/cron.go` - Go scheduler implementation
  - chrono-tz docs: https://docs.rs/chrono-tz/

  **Acceptance Criteria**:

  **Automated Verification:**
  ```bash
  grep "Europe/Berlin" backend/src/scheduler.rs
  # Assert: Found

  # Test manual triggers work
  curl -sf -X POST http://localhost:8090/api/admin/cron/daily
  curl -sf -X POST http://localhost:8090/api/admin/cron/weekly
  # Assert: Both return 204 No Content
  ```

  **Commit**: YES
  - Message: `feat(backend): implement scheduler for daily reminder and weekly reset`
  - Files: `backend/src/scheduler.rs`
  - Pre-commit: `cd backend && cargo build`

---

- [x] 11. Create Dockerfile

  **What to do**:
  - Create `backend/Dockerfile`:
    ```dockerfile
    # Build stage
    FROM rust:1.83-alpine AS builder
    RUN apk add --no-cache musl-dev
    WORKDIR /app
    COPY Cargo.toml Cargo.lock ./
    COPY src ./src
    COPY static ./static
    RUN cargo build --release
    
    # Runtime stage
    FROM alpine:latest
    RUN apk --no-cache add ca-certificates tzdata
    WORKDIR /app
    COPY --from=builder /app/target/release/schweinehund ./server
    ENV TZ=Europe/Berlin
    ENV DATA_DIR=/app/data
    EXPOSE 8090
    CMD ["./server"]
    ```
  - Ensure static linking (musl) for small image
  - Include tzdata for timezone support

  **Must NOT do**:
  - Do NOT use glibc (use musl for alpine)
  - Do NOT include build tools in final image

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Standard Dockerfile
  - **Skills**: None required

  **Parallelization**:
  - **Can Run In Parallel**: YES (with Task 10)
  - **Parallel Group**: Wave 4
  - **Blocks**: Task 12
  - **Blocked By**: Task 9

  **References**:
  - `backend/Dockerfile` - Current Go Dockerfile pattern
  - Rust Docker best practices

  **Acceptance Criteria**:

  **Automated Verification:**
  ```bash
  docker build -t schweinehund-backend:test backend/
  # Assert: Build succeeds

  docker run --rm schweinehund-backend:test ./server --help 2>&1 || true
  # Assert: Binary runs
  ```

  **Commit**: YES
  - Message: `build(backend): add Dockerfile for Rust backend`
  - Files: `backend/Dockerfile`
  - Pre-commit: `docker build -t test backend/`

---

- [x] 12. Update docker-compose.yml

  **What to do**:
  - Update `backend` service in `docker-compose.yml`:
    - Keep same service name, ports, healthcheck
    - Update build context if needed
    - Update environment variables:
      - Remove `DB_PATH`
      - Add `DATA_DIR=/app/data`
      - Keep `NTFY_URL`, `TZ`
    - Keep volume mount: `./data:/app/data`
  - Keep ntfy service unchanged
  - Verify healthcheck endpoint matches

  **Must NOT do**:
  - Do NOT modify ntfy service
  - Do NOT change port mappings

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: YAML configuration changes
  - **Skills**: None required

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 4
  - **Blocks**: Task 13
  - **Blocked By**: Task 11

  **References**:
  - `docker-compose.yml:4-27` - Current backend service definition

  **Acceptance Criteria**:

  **Automated Verification:**
  ```bash
  docker compose config --services
  # Assert: Output includes "backend" and "ntfy"

  docker compose config | grep "DATA_DIR"
  # Assert: Found
  ```

  **Commit**: YES
  - Message: `build(docker): update docker-compose for Rust backend`
  - Files: `docker-compose.yml`
  - Pre-commit: `docker compose config`

---

- [x] 13. Run Playwright E2E tests and fix issues

  **What to do**:
  - Start services: `docker compose up -d --build`
  - Run Playwright: `npm test`
  - If tests fail:
    - Analyze failure (API response shape? timing? selector?)
    - Fix backend or frontend as needed
    - Re-run until all pass
  - Document any API contract differences found

  **Must NOT do**:
  - Do NOT modify test expectations to match bugs
  - Do NOT skip flaky tests without investigation

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: May require debugging and fixes
  - **Skills**: `['playwright']`
    - playwright: For E2E test debugging

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 4
  - **Blocks**: Task 14
  - **Blocked By**: Task 12

  **References**:
  - `e2e/` - Playwright test files
  - `playwright.config.ts` - Test configuration

  **Acceptance Criteria**:

  **Automated Verification:**
  ```bash
  docker compose up -d --build
  sleep 10  # Wait for healthy
  npm test
  # Assert: All Playwright tests PASS
  ```

  **Commit**: YES (if fixes made)
  - Message: `fix(backend): resolve E2E test failures`
  - Files: (varies)
  - Pre-commit: `npm test`

---

- [x] 14. Remove Go backend artifacts

  **What to do**:
  - Delete Go source files: `backend/*.go`
  - Delete Go module files: `backend/go.mod`, `backend/go.sum`
  - Keep `backend/Cargo.toml`, `backend/src/`, `backend/static/`, `backend/Dockerfile`
  - Update `.gitignore` if needed (add `target/`, remove Go artifacts)
  - Verify build still works: `cargo build --release`

  **Must NOT do**:
  - Do NOT delete Rust files
  - Do NOT delete static assets
  - Do NOT delete data directory

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: File cleanup
  - **Skills**: `['git-master']`
    - git-master: Proper git rm for tracked files

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 5 (final)
  - **Blocks**: None
  - **Blocked By**: Task 13

  **References**:
  - `backend/*.go` - Files to delete
  - `backend/go.mod`, `backend/go.sum` - Go module files to delete

  **Acceptance Criteria**:

  **Automated Verification:**
  ```bash
  test -f backend/go.mod && echo "EXISTS" || echo "DELETED"
  # Assert: Output is "DELETED"

  test -f backend/Cargo.toml && echo "EXISTS"
  # Assert: Output is "EXISTS"

  cd backend && cargo build --release
  # Assert: Build succeeds
  ```

  **Commit**: YES
  - Message: `chore: remove Go backend artifacts`
  - Files: (deleted Go files)
  - Pre-commit: `cd backend && cargo build`

---

## Commit Strategy

| After Task | Message | Files | Verification |
|------------|---------|-------|--------------|
| 1 | `feat(backend): initialize Rust crate with dependencies` | Cargo.toml, src/*.rs | `cargo build` |
| 2 | `feat(backend): implement data models and JSON file persistence` | src/models.rs, src/storage.rs | `cargo test` |
| 3 | `feat(backend): implement zone REST API handlers` | src/state.rs, src/handlers_zone.rs | `cargo test` |
| 4 | `feat(backend): implement task REST API handlers` | src/handlers_task.rs | `cargo test` |
| 5 | `feat(backend): implement health and partials handlers` | src/handlers_static.rs | `cargo test` |
| 6 | `feat(backend): implement task rotation on completion` | src/handlers_task.rs | `cargo test` |
| 7 | `refactor(frontend): update static assets for Rust backend` | static/** | None |
| 8 | `feat(backend): implement ntfy notification client` | src/ntfy.rs, src/config.rs | `cargo test` |
| 9 | `feat(backend): wire up HTTP server with embedded assets` | src/main.rs, src/error.rs | `cargo build` |
| 10 | `feat(backend): implement scheduler for daily reminder and weekly reset` | src/scheduler.rs | `cargo build` |
| 11 | `build(backend): add Dockerfile for Rust backend` | Dockerfile | `docker build` |
| 12 | `build(docker): update docker-compose for Rust backend` | docker-compose.yml | `docker compose config` |
| 13 | `fix(backend): resolve E2E test failures` | (varies) | `npm test` |
| 14 | `chore: remove Go backend artifacts` | (deleted) | `cargo build` |

---

## Success Criteria

### Verification Commands
```bash
# 1. Rust build passes
cd backend && cargo build --release
# Expected: Build completes

# 2. All Rust tests pass
cd backend && cargo test
# Expected: All tests PASS

# 3. Quality gates pass
cd backend && cargo fmt --check && cargo clippy -- -D warnings
# Expected: No errors

# 4. Docker build succeeds
docker build -t test backend/
# Expected: Build completes

# 5. Services start correctly
docker compose up -d --build
docker compose ps
# Expected: backend (healthy), ntfy (healthy)

# 6. API works
curl -sf http://localhost:8090/api/health | jq '.status'
# Expected: "ok"

curl -sf http://localhost:8090/api/tasks | jq '.items | type'
# Expected: "array"

curl -sf http://localhost:8090/api/zones | jq '.items | type'
# Expected: "array"

# 7. Frontend loads
curl -sf http://localhost:8090/ | head -5
# Expected: HTML with "Schweinehund"

# 8. Partials work
curl -sf http://localhost:8090/partials/today | head -3
# Expected: HTML with "todayView"

# 9. Playwright E2E tests pass
npm test
# Expected: All tests pass

# 10. Data persists
docker compose restart backend
curl -sf http://localhost:8090/api/tasks | jq '.items | length'
# Expected: Same count as before restart

# 11. No Go files remain
test -f backend/go.mod && echo "FAIL" || echo "PASS"
# Expected: PASS
```

### Final Checklist
- [x] All "Must Have" present
  - [x] REST API for tasks and zones
  - [x] Health endpoint
  - [x] Partials serving
  - [x] Task rotation on completion
  - [x] Scheduler running
  - [x] ntfy integration
  - [x] JSON file persistence
  - [x] Embedded static files
  - [x] tracing logging
- [x] All "Must NOT Have" absent
  - [x] No SQLite/database
  - [x] No authentication
  - [x] No OpenSSL
  - [x] No nested module directories
- [x] All Rust tests pass
- [x] All Playwright tests pass
- [x] cargo fmt + clippy clean
- [x] Docker deployment working
