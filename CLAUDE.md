# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Zellij is a terminal multiplexer/workspace written in Rust with a client-server architecture and a WASM plugin system. Requires `protoc` (protocol buffer compiler), `pkg-config`, and `openssl` to build and test.

## Build Commands

The build system uses `cargo xtask` (aliased to `cargo x`):

```sh
cargo xtask build              # Build all crates and plugins
cargo xtask test               # Run all tests (builds plugins first)
cargo xtask test -- --test-threads=1  # Run tests single-threaded
cargo xtask clippy             # Run clippy linter
cargo xtask format             # Run rustfmt
cargo xtask format -- --check  # Check formatting without modifying
cargo xtask make               # Full pipeline: format, build, test, clippy
cargo xtask                    # Same as 'cargo xtask make'
cargo xtask run                # Build and run zellij in debug mode
cargo xtask run --singlepass   # Run with faster plugin compilation (Winch compiler)
cargo xtask install /path      # Install zellij binary to a directory
```

To run a single test in a specific crate:
```sh
cd zellij-server && cargo test test_name -- --all-features
```

For plugin crates (under `default-plugins/`), tests need the host target:
```sh
cd default-plugins/strider && cargo test --target x86_64-unknown-linux-gnu
```

E2E tests (require docker/podman + docker-compose):
```sh
docker-compose up -d
cargo xtask ci e2e --build
cargo xtask ci e2e --test
```

## Architecture

### Workspace Crates

- **`zellij-utils/`** — Shared library: data types (`data.rs` is ~122KB with core types like `Event`, `InputMode`, `SessionInfo`), IPC definitions, error handling, config parsing, KDL layout system. Almost everything depends on this.
- **`zellij-client/`** — Client-side: terminal UI, input handling (`input_handler.rs`, `keyboard_parser.rs`), signal handling. Connects to the server via IPC.
- **`zellij-server/`** — Server-side: session management, pane/tab orchestration (`screen.rs` is ~315KB), PTY management (`pty.rs`), plugin hosting, routing (`route.rs`). Each major subsystem runs in its own thread.
- **`zellij-tile/`** — Plugin SDK. Plugins implement the `ZellijPlugin` trait (`load`, `update`, `render`, `input`) and use `register_plugin!` macro.
- **`zellij-tile-utils/`** — Shared utilities for plugins.
- **`default-plugins/`** — 12 built-in WASM plugins (tab-bar, status-bar, strider, session-manager, etc.) plus a test fixture. Compiled to `wasm32-wasip1` target.
- **`xtask/`** — Build system (cargo-xtask pattern).
- **`src/`** — Main binary crate, CLI entry point.

### Key Architectural Patterns

- **Thread-based concurrency with message passing**: Server spawns dedicated threads for Screen, PTY, Plugins, Route, BackgroundJobs, PtyWriter. Communication via typed instruction enums sent through channels (e.g., `ScreenInstruction`, `PtyInstruction`, `PluginInstruction`).
- **Client-server IPC**: Uses protocol buffers (`.proto` files, `prost` codegen) over Unix sockets (via `interprocess` crate). Message types: `ServerToClientMsg`, `ClientToServerMsg`.
- **Plugin system**: WASM plugins run in `wasmi` runtime. Plugin API defined in `zellij-tile`. Plugins receive `Event`s and render to a string buffer.
- **KDL configuration**: Config and layouts parsed from KDL format (`zellij-utils/src/input/`).

### Platform Support

Platform-specific code uses conditional compilation (`#[cfg(unix)]`/`#[cfg(windows)]`) with separate files (e.g., `os_input_output_unix.rs`/`os_input_output_windows.rs`).

## Error Handling Conventions

Use `zellij_utils::errors::prelude::*` for all error handling:

- Functions should return `Result<T>` (anyhow) instead of calling `unwrap()`
- Attach context: `.context("failed to ...")` or `.with_context(|| format!(...))`
- For fatal errors at thread boundaries: `.fatal()` — logs and panics
- For non-fatal errors (log and continue): `.non_fatal()`
- For `Option` types, add a `.context()` explaining why `None` is an error before the regular context
- Ad-hoc errors: `anyhow!("message")`
- Custom error variants: add to `ZellijError` enum, recover with `err.downcast_ref::<ZellijError>()`

## Rust Toolchain

- Edition 2021, MSRV 1.92 (pinned in `rust-toolchain.toml`)
- Required targets: `wasm32-wasip1` (plugins), `x86_64-unknown-linux-musl` (E2E/cross-compile)
- Snapshot testing uses the `insta` crate

## Logging

Output goes to `/$TMPDIR/zellij-<UID>/zellij-log/zellij.log` (100KB limit). Use `log::info!`, `log::debug!`, etc. Run with `--debug` flag to get per-pane PTY byte dumps.

## Commit Style

Follow [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) for significant changes.
