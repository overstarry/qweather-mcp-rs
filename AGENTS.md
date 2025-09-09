# Repository Guidelines

## Project Structure & Module Organization
- `src/main.rs`: Binary entry (`qweather-mcp-server`). Starts the MCP service over stdio using Tokio. Optional HTTP streaming server mode via `--http`.
- `src/server.rs`: Server implementation with rmcp macros (`#[tool]`, `#[tool_router]`, `#[tool_handler]`).
- `Cargo.toml`: Package metadata, dependencies, and bin target.
- `target/`: Build artifacts (ignored). Editor folders like `.idea/` and `.zed` are local-only; don’t commit changes to them.

## Build, Test, and Development Commands
- Build debug: `cargo build`
- Build release: `cargo build --release`
- Run locally: `cargo run --bin qweather-mcp-server`
- Quick JSON-RPC check (stdin/stdout):
  `echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}' | cargo run --quiet --bin qweather-mcp-server`
- Run HTTP streaming mode (axum + rmcp Streamable HTTP):
  - `cargo run --features http --bin qweather-mcp-server -- --http`
  - or `QWEATHER_MCP_HTTP=1 cargo run --features http --bin qweather-mcp-server`
  - Bind address: `QWEATHER_MCP_HTTP_ADDR` (default `127.0.0.1:8000`)
- Lint: `cargo clippy --all-targets --all-features -- -D warnings`
- Format: `cargo fmt --all`

## Coding Style & Naming Conventions
- Rust edition: 2024. Use rustfmt defaults (4-space indentation, no tabs).
- Naming: `snake_case` for modules/functions, `UpperCamelCase` for types/traits, `SCREAMING_SNAKE_CASE` for consts.
- Error handling: prefer `?` with `anyhow::Result` for fallible ops; avoid `unwrap()` in non-test code.
- Keep functions small and focused; document non-obvious behavior with concise comments.

## Testing Guidelines
- Run all tests: `cargo test`
- Unit tests live next to code using `#[cfg(test)] mod tests { ... }`.
- For broader scenarios, add integration tests in `tests/` (e.g., `tests/server_integration.rs`).
- Tests should be deterministic and not rely on external network; mock or stub where needed.

## Commit & Pull Request Guidelines
- Commits: imperative mood, concise summary (≤72 chars). Examples: “Add counter tool”, “Refactor server routing”, “Update README”.
- PRs should include: purpose/motivation, overview of changes, how to run/verify (commands), and linked issues (e.g., “Closes #123”).
- Before opening a PR: run `cargo fmt`, `cargo clippy -D warnings`, and `cargo test` locally.

## Security & Configuration Tips
- Do not hardcode credentials. If/when API access is added, prefer env vars (e.g., `QWEATHER_API_KEY`).
- Avoid logging secrets. Use structured errors (`anyhow`) and return MCP-compliant error responses via rmcp types.
