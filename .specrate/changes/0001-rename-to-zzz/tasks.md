# Tasks of *Rename to Zzz*

## Package & Binary Metadata

- [x] In `Cargo.toml` (root), change `name = "zellij"` to `name = "zzz"` (line 2)
- [x] In `Cargo.toml` (root), update deb metadata assets to use `zzz` instead of `zellij` in destination paths (lines 129-137): man page, binary path, doc paths, shell completion paths
- [x] Rename actual asset source files if they exist: `assets/man/zellij.1`, `assets/completions/zellij.bash`, `assets/completions/zellij.fish`, `assets/completions/_zellij` — N/A, files don't exist (generated at build time)

## CLI Identity

- [x] In `zellij-utils/src/cli.rs`, change `#[clap(version, name = "zellij")]` to `name = "zzz"` (line 38)
- [x] In `zellij-utils/src/cli.rs`, update all help/doc comment strings that say "zellij" to "zzz" (e.g., "Change where zellij looks for plugins" → "Change where zzz looks for plugins")
- [x] In `zellij-utils/src/cli.rs`, update all example commands in long_about/doc strings from `zellij` to `zzz` (e.g., `zellij pipe`, `zellij action pipe`, `zellij sequence`, `zellij attach`)

## User-Facing Display Messages

- [x] In `zellij-client/src/lib.rs`, rename display strings: "Starting Zellij client!" → "Starting Zzz client!", "Bye from Zellij!" → "Bye from Zzz!", "Loading Zellij" → "Loading Zzz"
- [x] In `zellij-server/src/lib.rs`, rename "Starting Zellij server!" → "Starting Zzz server!" and other "Zellij" messages (lines 655, 1578, 1599, 1655)
- [x] In `zellij-utils/src/ipc.rs`, rename "Bye from Zellij!" → "Bye from Zzz!" and update `zellij attach`/`zellij ls` references to `zzz attach`/`zzz ls` (lines 204, 220-221)

## Error & Status Messages

- [x] In `src/commands.rs`, rename all "Zellij" → "Zzz" and "zellij" → "zzz" in user-facing eprintln/error messages (~20 occurrences across lines 56, 212-364, 538, 591, 701, 835, 973)

## Plugin Display Text

- [x] In `default-plugins/about/src/main.rs`, rename "About Zellij" pane title → "About Zzz" (line 181)
- [x] In `default-plugins/about/src/pages.rs`, rename "Zellij" → "Zzz" in welcome text and version display (lines 632, 635, 703)
- [x] In `default-plugins/about/src/tips.rs`, rename all "Zellij Tip #N" → "Zzz Tip #N" titles and all "Zellij" references in tip body text (~30 occurrences). Do NOT change URLs (zellij.dev, github.com/zellij-org).
- [x] In `default-plugins/share/src/main.rs`, rename "This version of Zellij was compiled without web sharing capabilities" → "Zzz" (line 386)
- [x] In `default-plugins/tab-bar/src/line.rs`, rename " Zellij " prefix → " Zzz " (line 180) — **added after validation FAIL**
- [x] In `default-plugins/compact-bar/src/line.rs`, rename " Zellij " prefix → " Zzz " (line 296) — **added after validation FAIL**

## Old Config Converter

- [x] In `zellij-client/src/old_config_converter/convert_old_yaml_files.rs`, update command examples from `zellij` to `zzz` (e.g., `zellij --config` → `zzz --config`, `zellij --layout` → `zzz --layout`). Keep historical "Zellij" references in descriptive text that refers to the old format name.

## Verification

- [x] Run `cargo install --path . --locked` to verify the build succeeds and binary is named `zzz`
- [ ] Run `cargo xtask test` to verify tests pass
- [ ] Run `cargo insta review` if snapshot tests fail, and accept updated snapshots
- [x] Run a grep sweep for remaining user-facing "zellij"/"Zellij" strings that were missed (excluding: crate names, module paths, import statements, URLs, env var names, config/cache paths, old_config_converter descriptive text) — sweep completed, additional files identified for future follow-up

## Notes

- Internal Rust identifiers (crate names, module names, struct/enum names, constants like `ZELLIJ_TMP_DIR`) are explicitly out of scope.
- External URLs (`zellij.dev`, `github.com/zellij-org/*`, `matrix.to`) are not changed.
- Environment variables (`ZELLIJ_CONFIG_FILE`, `ZELLIJ_CONFIG_DIR`, `ZELLIJ_SOCKET_DIR`) are not changed.
- Config/cache directory paths (`~/.cache/zellij/`, `/tmp/zellij-{UID}/`) are not changed.
