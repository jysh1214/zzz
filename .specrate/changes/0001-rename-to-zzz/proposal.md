# Proposal of *Rename to Zzz*

## Motivation

The current project binary and user-facing name is "zellij," which is difficult to spell, remember, and type quickly. Renaming the user-facing identity to "zzz" makes the tool faster to invoke from the command line and easier to recall. The internal Rust codebase (crate names, module names, type names) remains unchanged to avoid a massive refactor with no functional benefit.

## Summary

- **BREAKING** Rename the compiled binary from `zellij` to `zzz`
- **BREAKING** Rename the CLI command from `zellij` to `zzz` (all subcommands, flags, and shell completions follow)
- Update all user-facing display text from "Zellij" to "Zzz" (startup messages, error messages, plugin pane titles, tips, about page, welcome screen)
- **BREAKING** Change the install command from `cargo install --locked zellij` to `cargo install --locked zzz`
- Update packaging metadata (deb assets, man page filename, shell completion filenames) to use `zzz`
- Do **not** change internal Rust crate names, module names, struct/enum names, or any code identifiers

## Impact

- Affected specs: *(none yet — first change in repository)*
- Affected code:
  - `Cargo.toml` (root) — package `name`, `[[bin]]` name, deb metadata paths
  - `zellij-utils/src/cli.rs` — `clap` command name and help strings
  - `zellij-client/src/lib.rs` — startup/shutdown display messages
  - `zellij-server/src/lib.rs` — server startup log message
  - `zellij-utils/src/ipc.rs` — exit display message
  - `src/commands.rs` — error/status messages referencing "Zellij"
  - `default-plugins/about/src/main.rs` — plugin pane title
  - `default-plugins/about/src/pages.rs` — welcome/about text
  - `default-plugins/about/src/tips.rs` — tip titles and content
  - `default-plugins/tab-bar/src/line.rs` — tab bar "Zellij" prefix (top-left branding)
  - `default-plugins/compact-bar/src/line.rs` — compact bar "Zellij" prefix (top-left branding)
  - `zellij-client/src/old_config_converter/convert_old_yaml_files.rs` — migration messages
  - `README.md` — install instructions, project name
  - Shell completion scripts and man page references
