
## Task 4: Database Layer (2025-02-01)

### Issue: Module Not Declared in main.rs
**Severity:** Critical  
**Time Lost:** ~30 minutes of debugging

**Problem:**
- Created `src/db.rs` with complete implementation and tests
- Tests weren't running: `cargo test` showed "0 tests"
- No compilation errors, no warnings
- Even intentional syntax errors in db.rs were silently ignored

**Root Cause:**
- Forgot to add `mod db;` declaration in `src/main.rs`
- Without module declaration, Rust compiler completely ignores the file
- The file exists on disk but isn't part of the compilation unit

**Solution:**
```rust
// src/main.rs
mod assets;
mod db;  // ← Must declare the module!
```

**Prevention:**
- Always verify module declaration when creating new module files
- If tests aren't running, first check if module is declared in parent
- Use `cargo check` to verify module is being compiled (will show dead code warnings)

**Lesson:**
Rust's module system requires explicit declaration. Unlike some languages that auto-discover files, Rust needs the `mod` statement. This is by design for explicit dependency management.
