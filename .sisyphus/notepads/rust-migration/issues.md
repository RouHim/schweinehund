# Issues & Gotchas - Rust Migration

## Problems Encountered

1. `rust-analyzer` missing in toolchain; `lsp_diagnostics` failed to initialize.
2. `rustup component add rust-analyzer` reports installed, but `lsp_diagnostics` still fails with "Unknown binary 'rust-analyzer'".
2. `lsp_diagnostics` still reports missing `rust-analyzer` after installing component; tool appears to use a rustup context that cannot see user toolchain.

3. `lsp_diagnostics` tool has no Rust LSP configured in this environment (reports "No LSP server configured" and lists no rust-analyzer). For Rust tasks, rely on `cargo test`, `cargo build`, `cargo fmt --check`, and `cargo clippy -- -D warnings` as the static-analysis gate until Rust LSP support is configured.

4. `podman build` failed pulling `ghcr.io/rust-cross/rust-musl-cross:x86_64-musl` with a missing blob/layer error; `podman build --arch amd64` succeeded.
5. `podman build` pull of `docker.io/messense/rust-musl-cross:x86_64-musl` required Docker Hub auth in this environment (unauthorized). Prefer GHCR.
