# AGENTS.md - Schweinehund

Guidelines for AI agents working in this codebase.

## Project Overview

Schweinehund is a minimalist household cleaning task manager. PWA with Rust backend (warp + sqlx), Vanilla JS frontend,
and SQLite database. All static assets are embedded into the release binary via rust-embed.

## Tech Stack

- **Backend**: Rust (warp web framework, sqlx for SQLite, tokio async runtime)
- **Frontend**: Vanilla JavaScript (no framework), HTML, CSS
- **Database**: SQLite with WAL mode
- **Testing**: Playwright E2E tests (TypeScript)
- **Static embedding**: rust-embed (assets compiled into binary)

## Build & Run Commands

```bash
# Development
cargo run                          # Run server at localhost:3000
cargo build                        # Debug build
cargo build --release              # Release build (< 15MB self-contained)

# Database
sqlx database create               # Create SQLite database
sqlx migrate run                   # Run migrations

# Code Quality
cargo fmt                          # Format Rust code
cargo clippy -- -D warnings        # Lint with warnings as errors
cargo test                         # Run unit tests
```

## Test Commands

### Rust Unit Tests

```bash
cargo test                                    # Run all tests
cargo test test_toggle_task                   # Run single test by name
cargo test db::tests::                        # Run tests in module
cargo test -- --nocapture                     # Show println! output
```

### Playwright E2E Tests

```bash
cd e2e
npx playwright install                        # First-time browser setup
npx playwright test                           # Run all E2E tests
npx playwright test tests/crud.spec.ts        # Run single test file
npx playwright test -g "creates a new"        # Run tests matching pattern
npx playwright test --headed                  # Run with visible browser
npx playwright test --ui                      # Interactive UI mode
npx playwright test --debug                   # Debug mode with inspector
```

E2E tests auto-start the Rust server via `cargo run` (configured in `e2e/playwright.config.ts`).

## Project Structure

```
src/
  main.rs           # Entry point, server setup, graceful shutdown
  db.rs             # Database queries, models (DailyTask, DeepCleaningTask)
  routes.rs         # HTTP handlers, warp filters, request/response types
  notifications.rs  # ntfy.sh push notification client
  scheduler.rs      # Weekly reset scheduler (Monday 00:00)
  assets.rs         # Static file serving via rust-embed
static/
  index.html        # Main HTML
  app.js            # All frontend logic (vanilla JS)
  styles.css        # CSS with dark/light theme support
  sw.js             # Service worker for PWA
  manifest.json     # PWA manifest
migrations/         # SQLx migrations (SQL files)
e2e/tests/          # Playwright test specs
```

## Rust Code Style

### Imports

Group imports in order: std, external crates, local modules. Use explicit paths.

```rust
use std::net::SocketAddr;
use anyhow::Result;
use sqlx::SqlitePool;
use crate::db;
```

### Error Handling

- Use `anyhow::Result<T>` for functions that can fail
- Use `?` operator for error propagation
- Custom rejection types for warp: `impl reject::Reject for CustomError {}`
- Fire-and-forget pattern for non-critical operations (log error, return Ok)

### Naming Conventions

- Functions: `snake_case` (e.g., `get_today_tasks`, `toggle_task`)
- Types/Structs: `PascalCase` (e.g., `DailyTask`, `AppSettings`)
- Constants: `SCREAMING_SNAKE_CASE`
- Database columns: `snake_case`

### Patterns

- Async handlers with `async fn handle_*(...)` naming
- `#[derive(Debug, Clone, FromRow, serde::Serialize)]` on data models
- Use `sqlx::query_as::<_, Type>()` for typed queries
- Raw SQL strings with `r#"..."#` for multi-line queries

### Type Safety

- No `unwrap()` in production paths - use `?` or explicit error handling
- `Option<T>` for nullable database fields
- Explicit type annotations on `sqlx::query_as` for clarity

## Frontend Code Style (JavaScript)

### Patterns

- Global `state` object for application state
- Functions prefixed by purpose: `fetch*`, `render*`, `handle*`, `init*`
- Event delegation via `attachListeners()` pattern
- `escapeHtml()` for all user-provided content in templates

### DOM Conventions

- IDs: `kebab-case` (e.g., `tasks-list`, `task-modal`)
- Data attributes: `data-task-id`, `data-testid` for Playwright selectors
- `data-testid` attributes required for E2E testable elements

## E2E Test Style (TypeScript)

### Patterns

```typescript
test.describe('Feature Name', () => {
    test.beforeEach(async ({page}) => { /* setup */
    });
    test.afterEach(async ({page}) => { /* cleanup */
    });

    test('action produces expected result', async ({page}) => {
        // Arrange
        await page.goto('/');
        await page.waitForSelector('#element', {state: 'visible'});

        // Act
        await page.locator('[data-testid="btn"]').click();

        // Assert
        await expect(page.locator('.result')).toBeVisible();
    });
});
```

### Selectors (priority order)

1. `[data-testid="..."]` - preferred for test stability
2. `#id` - for unique elements
3. `.class` with context - for lists/groups

## Database

### Schema

- `daily_tasks`: id, name, description, zone, day_of_week, completed, completed_at
- `deep_cleaning_tasks`: id, name, description, zone, queue_position, completed_at
- `app_state`: key-value store for settings

### day_of_week values

- `-1`: Mini-routine (shows every day)
- `1-7`: Monday through Sunday

### Migrations

Located in `migrations/`. Create new: `sqlx migrate add <name>`

## Environment Variables

```env
DATABASE_URL=sqlite:data/schweinehund.db  # Required
NTFY_TOPIC=your-topic                      # Optional: push notifications
NTFY_SERVER=https://ntfy.sh                # Optional: custom ntfy server
RUST_LOG=info                              # Logging level
```

## Common Tasks

### Adding a new API endpoint

1. Add route filter in `routes.rs` (e.g., `fn new_endpoint()`)
2. Add handler function (`async fn handle_new_endpoint()`)
3. Wire into `api_routes()` chain with `.or(new_endpoint(pool.clone()))`
4. Add corresponding db function in `db.rs` if needed

### Adding a new test

- Unit test: Add `#[tokio::test]` function in `db.rs` tests module
- E2E test: Create/extend spec file in `e2e/tests/`

### General Conventions

* Keep functions small and single-purpose
* Use BDD style for tests
* Use TDD when adding new features
* Clean code is prioritized over clever code
  ** KISS principle
  ** DRY principle
  ** YAGNI principle
  ** Readability over brevity
* SOLID principles where applicable
  ** Favor composition over inheritance
  ** Interface segregation
  ** Dependency inversion
  ** Single responsibility
  ** Open/closed principle
  ** Liskov substitution principle
  ** Avoid premature optimization
* Never use #unwrap() in, usage of anyhow or proper error handling
* Never use #dead_code lints, remove unused code instead
* Humor is encouraged in comments, commit messages, human interactions, etc., but not in code structure
  or naming to avoid confusion.
* You are not allowed to use sudo or root privileges on the system. Always work within the confines of your user
  permissions. If you need
  elevated permissions for a task, please request assistance from a human operator.
* Before shipping a feature, make sure to run Full test suite before commiting, this might take some time but ensure consistency
