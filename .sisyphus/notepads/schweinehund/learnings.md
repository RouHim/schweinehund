## Learnings - Schweinehund Project Initialization

### Project Setup
- **Rust Project Structure**: Created standard Cargo project with `src/`, `migrations/`, `static/`, and `e2e/` directories
- **Dependencies Locked**: All key dependencies added and verified to compile cleanly
  - warp 0.3.7 for HTTP server (no dedicated rustls feature, but compatible with modern TLS)
  - sqlx 0.7.4 with SQLite support for database ORM
  - tokio 1.x with full features for async runtime
  - rust-embed 8.4 for static asset embedding
  - Included proper logging stack: tracing + tracing-subscriber

### Build Configuration
- **Cargo.toml Organization**: Dependencies grouped by purpose with explanatory comments for maintainability
- **Feature Flags**: 
  - sqlx configured with `sqlite`, `runtime-tokio-rustls`, `migrate` features
  - tokio using full feature set for development convenience
  - tracing-subscriber with `env-filter` for flexible logging control
- **Profile Settings**: Added release profile with LTO for production builds

### Server Implementation
- **Warp Server Pattern**: Initial skeleton uses `bind_with_graceful_shutdown()` for clean SIGTERM handling
- **Health Check Endpoint**: `/health` returns simple JSON response for monitoring
- **Port Configuration**: Server binds to `127.0.0.1:3000` (can be made configurable later)
- **Logging Integration**: tracing initialized with env-filter support

### Git & Version Control
- **Initial Commit**: Atomic commit `9264bee` with message following conventional commits format
- **.gitignore**: Comprehensive Rust patterns including target/, data/, SQLite databases
- **Build Artifacts**: Properly excluding Cargo.lock, compiled binaries, IDE configs

### Database Configuration
- **sqlx.toml**: Minimal migration configuration pointing to migrations directory and SQLite path
- **.env.example**: Provides DATABASE_URL template for local development setup
- **Data Directory**: Configured to store SQLite database in `data/` (gitignored)

### Verification Results
✓ Cargo check passes cleanly
✓ All 4 required directories created
✓ 12+ critical dependencies resolved and locked
✓ Project builds with zero errors or warnings
✓ Git repository initialized with first commit

### Next Steps Template
When implementing database schema:
- Create migrations in `migrations/` directory
- Use sqlx-cli for migration management
- Keep migration files versioned and ordered

When adding frontend assets:
- Place static files in `static/` directory
- Use rust-embed to bundle them into binary

When adding tests:
- Playwright e2e tests go in `e2e/` 
- Integration tests in `src/` with `#[tokio::test]`

## Task 2: Database Schema and Migrations

**Completed**: ✓

### Key Learnings

1. **SQLite Schema Design**:
   - Used simple INTEGER PRIMARY KEYs (no AUTOINCREMENT, SQLite handles this automatically)
   - BOOLEAN stored as 0/1 (SQLite standard)
   - Timestamps as INTEGER (Unix epoch) for easy comparison
   - No foreign keys or complex relations (per requirements)

2. **Seed Data Organization**:
   - Daily tasks: 23 total = 5 mini-routine (day_of_week=-1) + 4 Mon + 3 Tue + 4 Wed + 3 Thu + 4 Fri
   - Day of week: 0=Sunday, 1=Monday, ..., 5=Friday (no weekend tasks)
   - day_of_week = -1 used for daily routine tasks that repeat every day
   - Zone names follow user's German naming (EG, KG, 1.OG, Buero)

3. **Deep Cleaning Queue**:
   - 4 tasks in rotation queue (queue_position 1-4)
   - Schema includes completed_at field for tracking (though v1 just rotates, no history)

4. **App State Pattern**:
   - Simple key-value store for global settings
   - last_reset_at = '0' means never reset (Unix epoch check)
   - notification_enabled and notification_time stored as strings

5. **Migration Verification**:
   - Used direct sqlite3 CLI for testing (sqlx migrate run had path issues)
   - Verified: 23 daily tasks, 4 deep cleaning tasks, 3 app state entries
   - All data matches the cleaning schedule from plan exactly

### SQL Comments Rationale

Added SQL comments to document:
- Schema intent and table purposes
- Zone mappings (EG, KG, 1.OG abbreviations)
- Day-of-week grouping for easy reading of seed data
- These are necessary for future maintenance (not verbose or redundant)

### Files Created
- `migrations/001_initial.sql` - Schema + seed data (78 lines)

### Next Steps
Task 4 (Database Layer) will implement the sqlx data access layer that reads from these tables.

## Task 3: E2E Test Infrastructure (COMPLETED)

### Setup Pattern
- **Framework**: Playwright @latest (1.58.1+ as of Task 3)
- **Config**: TypeScript-based playwright.config.ts with mobile device profiles
- **Mobile Devices**: iPhone 13 and Pixel 5 from Playwright's built-in devices
- **Server Integration**: webServer config auto-starts Rust backend (cargo run on port 3000)
- **Reports**: HTML reporter + screenshots on failure + traces on retry

### Key Configuration Details
- **Test Discovery**: Tests in `e2e/tests/` directory
- **Base URL**: http://localhost:3000 (configurable via baseURL)
- **CI Behavior**: 
  - Retries: 0 for local, 2 for CI
  - Workers: undefined (parallel) for local, 1 for CI
  - forbidOnly: true on CI to catch accidental test.only
- **Failure Artifacts**: Screenshots and traces collected automatically

### Testing Strategy
- Start with smoke test (health endpoint)
- Mobile-first: Tests run on 3 projects (iPhone 13, Pixel 5, Desktop Chrome)
- API testing: Use request fixture for backend API validation
- UI testing: Use page fixture for browser interactions

### Commands
- `npm install` - Install Playwright and dependencies
- `npx playwright --version` - Verify installation
- `npx playwright test --list` - Validate config and list all tests
- `npm run test` - Run all tests headless
- `npm run test:headed` - Run with headed browsers
- `npm run test:debug` - Run in debug mode with inspector


## Task 4: Database Layer Implementation (2025-02-01)

### Key Learnings

**SQLx Compile-Time Query Verification:**
- sqlx query macros (`query_as!`, `query!`) require DATABASE_URL to be set for compile-time verification
- Alternatively, can use `cargo sqlx prepare` to generate `.sqlx/` metadata cache for offline mode
- The `.sqlx/` directory MUST be committed to version control for CI/CD pipelines
- Regular `query_as::<_, Type>()` works without compile-time checks but loses type safety

**Module Declaration Issue:**
- Rust modules MUST be declared with `mod module_name;` in the parent module (main.rs)
- Without the `mod` declaration, the module file is completely ignored by the compiler
- No compilation errors, no warnings - the file just isn't compiled
- Tests in undeclared modules won't run (silently skipped)

**SQLx Type Handling:**
- SQLite BOOLEAN is stored as INTEGER (0/1), sqlx handles conversion automatically
- For `query_as!` macro, nullable columns are inferred as Option types even if marked as NOT NULL in migration
- Using regular `query_as::<_, Type>()` gives more control over Option handling
- INTEGER timestamps can be stored as i64 directly

**Test Structure:**
- Tests in submodules (`#[cfg(test)] mod tests`) are discovered and run automatically
- `#[tokio::test]` works seamlessly with `cargo test` (no special configuration needed)
- In-memory SQLite (":memory:") works well for unit tests with migrations

**Best Practices Validated:**
- WAL mode (`PRAGMA journal_mode = WAL`) improves concurrent read/write performance
- Connection pooling with max_connections prevents resource exhaustion
- anyhow::Result provides ergonomic error handling with ? operator
- FromRow derive trait simplifies query result mapping

### Technical Decisions

**Regular query() vs query!() macro:**
- Chose regular `query()` and `query_as()` for flexibility
- Avoids Option type inference issues with nullable columns
- Still get runtime type checking from sqlx
- Trade-off: lose compile-time SQL validation but gain simpler types

**Timestamps as i64:**
- Storing Unix timestamps as INTEGER (i64) for simplicity
- Using chrono::Utc::now().timestamp() for current time
- Avoids timezone complexity for MVP

**Queue Rotation Algorithm:**
- MAX(queue_position) + 1 for moving completed tasks to end
- Simple but effective for small queue sizes
- Could be optimized later with batch position updates if needed

## Task 5: REST API Routes with Warp (2026-02-01)

### Key Learnings

**Warp Filter Composition:**
- Filters are composable with `.and()` for sequential matching and `.or()` for alternatives
- Order matters: API routes MUST come before catch-all static file handler
- `warp::any()` is useful for dependency injection (e.g., database pool)
- Path segments can extract typed parameters: `warp::path!("api" / "tasks" / i64 / "toggle")`

**Static File + SPA Routing Conflict:**
- SPA fallback (returning index.html for 404s) interferes with API routes
- Solution: Explicitly reject `/api/*` paths in static handler before checking files
- Static handler should check path prefix FIRST, then attempt file serving
- This allows `.or()` chain to continue to API routes when path starts with /api

**Database Pool Injection:**
- Clone-based approach works well with warp: `warp::any().map(move || pool.clone())`
- SqlitePool is Arc-based internally, so cloning is cheap (just increments ref count)
- Extract with `.and(with_db(pool))` in filter chains
- Pool is moved into closure, requiring clone for each route filter

**Error Handling Pattern:**
- Custom rejection types: `struct DatabaseError; impl reject::Reject for DatabaseError {}`
- Central rejection handler converts rejections to JSON responses with appropriate status codes
- Distinguish between different error types (NotFound, DatabaseError, InvalidQuery, etc.)
- `handle_rejection()` is async and returns Result<impl Reply, Rejection>

**CORS Configuration:**
- Warp has built-in CORS support via `warp::cors()`
- For local development: `.allow_any_origin()` is acceptable
- Production should restrict origins, methods, and headers
- Apply CORS with `.with(cors)` at the end of filter chain

**Type Serialization:**
- Database types (DailyTask, DeepCleaningTask) need Serialize trait for JSON responses
- Simple addition: `#[derive(serde::Serialize)]` to existing FromRow types
- Chrono Datelike trait must be imported for `.weekday()` method
- `chrono::Local::now().weekday().num_days_from_monday()` gives 0-6 (Mon-Sun)

**Testing Strategy:**
- Manual curl tests before automated tests confirm basic functionality
- Always check raw responses first (without jq) to diagnose parsing issues
- Server logs reveal database connection issues early
- Kill processes with `pkill -9` when port binding fails

**Database Setup:**
- SQLite needs file to exist before connection (or proper permissions to create it)
- Use `touch schweinehund.db` to pre-create empty file if needed
- Migrations run automatically on startup via `db::run_migrations(&pool)`
- WAL mode files (*.db-shm, *.db-wal) should be .gitignored

### Technical Decisions

**Why Custom Rejection Types:**
- Warp's default rejections don't distinguish between "resource not found" vs "route not found"
- Custom DatabaseError and NotFoundError allow specific HTTP status codes (500 vs 404)
- Centralized error handling in `handle_rejection()` ensures consistent JSON error responses

**Why warp::path! Macro:**
- Type-safe path parameter extraction: `warp::path!("api" / "tasks" / i64)` auto-parses to i64
- Compile-time validation of path structure
- Cleaner than manual path segment extraction

**Debug Endpoints Placeholder:**
- `/api/debug/notify` returns success but doesn't send notification (Task 8)
- `/api/debug/reset` fully functional (calls db::reset_daily_tasks + set_last_reset)
- Allows frontend development to proceed without notification system

**Settings Update Pattern:**
- POST /api/settings accepts partial updates (notification_enabled, notification_time)
- Created db::update_app_settings() to persist changes to app_state table
- Returns updated settings in response for client-side state sync

### API Endpoints Implemented

**Core Task Endpoints:**
- `GET /api/health` - Health check (returns {"status": "ok"})
- `GET /api/tasks/today` - Daily tasks + mini-routine (query param: day_of_week)
- `POST /api/tasks/:id/toggle` - Toggle completion status

**Deep Cleaning Endpoints:**
- `GET /api/deep-cleaning` - Queue ordered by position
- `POST /api/deep-cleaning/:id/complete` - Mark complete and rotate to end

**Settings Endpoints:**
- `GET /api/settings` - Get notification settings
- `POST /api/settings` - Update notification settings

**Debug Endpoints:**
- `POST /api/debug/reset` - Manual reset trigger
- `POST /api/debug/notify` - Test notification (placeholder)

### Files Modified
- `src/routes.rs` - NEW: Complete REST API implementation (295 lines)
- `src/main.rs` - Initialize DB pool and wire routes
- `src/db.rs` - Add update_app_settings() + Serialize derives
- `src/assets.rs` - Reject /api paths before SPA fallback

### Verification Results
✓ Health check returns {"status": "ok"}
✓ Today's tasks returns 5 tasks (mini-routine for Saturday)
✓ Toggle task updates completion status
✓ Deep cleaning queue returns 4 tasks in order
✓ Completing deep task rotates queue (Bad gruendlich -> Kuehlschrank becomes first)
✓ Settings GET/POST works with persistence
✓ Debug reset clears completed status
✓ Debug notify returns placeholder response

### Next Steps
- Task 6: Frontend UI (Svelte + TailwindCSS)
- Task 7: Daily reset scheduler (cron-like background task)
- Task 8: Notification system (ntfy.sh integration)

## Task 8: ntfy.sh Notification Integration ✅

### Implementation Patterns
1. **Fire-and-forget pattern**: HTTP requests that log errors but don't propagate failures
2. **Trait imports**: Must import `Datelike` and `Timelike` from chrono for `.hour()`, `.weekday()` methods
3. **ureq agent**: Synchronous HTTP client works well for one-shot requests
4. **Quiet hours**: Implemented as 22:00 (10 PM) to 07:00 (7 AM) check using `.hour()` method

### Key Implementation Details
- `NtfyClient` is simple struct with `ureq::Agent`, topic, and server URL
- All configuration via environment variables with sensible defaults
- `send_reminder()` catches and logs errors without propagating
- Response status 200 indicates successful delivery
- Proper headers: Title, Priority (default=3), Tags

### Testing Approach
- Debug endpoint `/api/debug/notify` calls `send_test_notification()`
- Can verify with `curl -X POST http://localhost:3000/api/debug/notify`
- Check logs for "Notification sent to schweinehund" message
- Debug logging shows full HTTP request/response cycle

### Design Decisions
- Keep implementation minimal - fire-and-forget only, no retries
- `send_daily_reminder()` is public API for scheduler integration (Task 7)
- Unused method warnings are expected - scheduler will call these later
- Quiet hours prevent spam at night (10 PM - 7 AM)
- Topic "schweinehund" is hardcoded with env var override option

### Next Steps
- Task 9 (UI) and Task 12 (E2E tests) can use notification system
- Scheduler (Task 7) should call `send_daily_reminder()` at wake time
- Database stores `notification_enabled` and `notification_time` settings

## Task 7: Weekly Reset Scheduler ✅

### Implementation Patterns
1. **Startup Reconciliation**: Check `last_reset_at` on startup and execute missed resets
2. **Background Task**: Use `tokio::spawn()` with continuous loop and `tokio::time::sleep_until()`
3. **Monday Calculation**: Use `chrono::Datelike::weekday()` and day-of-week math to find next Monday
4. **Timezone Handling**: Use `chrono::Local` for local timezone awareness (Monday 00:00 local time)

### Key Implementation Details
- `get_last_monday_midnight()` calculates most recent Monday 00:00 from any timestamp
- `get_next_monday_midnight()` calculates next Monday 00:00 for sleep duration
- `startup_reconciliation()` compares last reset with last Monday to detect missed resets
- `execute_reset()` calls `db::reset_daily_tasks()` and `db::set_last_reset()`
- Background task spawned with `tokio::spawn()` returns JoinHandle for graceful shutdown tracking

### Time Calculation Logic
```rust
// Days since Monday: 0=Mon, 1=Tue, ..., 6=Sun
let days_since_monday = now.weekday().num_days_from_monday();
let last_monday = now.date_naive() - chrono::Duration::days(days_since_monday);
```

### Integration Points
- **main.rs**: Call `scheduler::start_scheduler(pool.clone())` after migrations
- **routes.rs**: Wire `trigger_reset_now()` to `/api/debug/reset` endpoint
- **Database**: Read `last_reset_at` from app_state, update after each reset

### Testing Strategy
1. Start server and verify startup reconciliation logs
2. Check "Next reset scheduled for..." message shows correct Monday 00:00
3. Use debug endpoint to trigger manual reset
4. Verify tasks are unchecked after reset
5. Confirm `last_reset_at` is updated in database

### Design Decisions
- **No external cron/systemd**: All scheduling logic in Rust for portability
- **No retry logic**: Simple sleep-and-execute pattern, failures logged but not retried
- **Local timezone**: Uses system local time, not UTC, for user-friendly Monday 00:00
- **Fire-and-forget task**: Spawned task runs forever, no explicit shutdown handling needed

### Gotchas & Solutions
1. **Weekday enum**: Must use `num_days_from_monday()` method, not raw enum values
2. **tokio::time::Instant**: Convert duration to `Instant::now() + Duration` for `sleep_until()`
3. **NaiveDateTime conversion**: Use `Local.from_local_datetime()` to get timezone-aware DateTime
4. **Tests with specific dates**: Use `NaiveDate::from_ymd_opt()` for deterministic test fixtures

### Files Created/Modified
- `src/scheduler.rs` - NEW: 168 lines (startup reconciliation + background task)
- `src/main.rs` - Add `mod scheduler;` and call `start_scheduler(pool.clone())`
- `src/routes.rs` - Update `handle_debug_reset()` to call `scheduler::trigger_reset_now()`
- `src/notifications.rs` - Added `Timelike` import (bug fix from Task 8)

### Verification Results
✓ Server starts with "Starting weekly reset scheduler" log
✓ Startup reconciliation runs and logs last reset timestamp
✓ Next reset scheduled for correct Monday 00:00 (e.g., "2026-02-02 00:00:00")
✓ Debug endpoint `/api/debug/reset` triggers immediate reset
✓ Reset logs show "Executing weekly reset" and "Weekly reset completed successfully"
✓ Tasks are unchecked after reset
✓ `last_reset_at` updated in database

### Next Steps
- Task 8 (notifications) already completed - can integrate scheduler with daily reminders
- Task 9 (UI) will consume the reset functionality
- Task 12 (E2E tests) should verify reset behavior
