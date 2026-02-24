# Tasks of *Add Zzz Status Bar*

## New Plugin Crate

- [x] Create `default-plugins/zzz-status-bar/Cargo.toml` with dependency: `zellij-tile`
- [x] Create `default-plugins/zzz-status-bar/.cargo/config.toml` with `target = "wasm32-wasip1"`
- [x] Create `default-plugins/zzz-status-bar/src/main.rs` implementing `ZellijPlugin` trait:
  - `State` struct with fields: `visible`, `change_id`, `ticket`, `change_state`, `git_branch`, `git_commit`, `mode_info`, `cwd`
  - `load()`: set `visible = false`, `set_selectable(false)`, subscribe to `Timer`, `RunCommandResult`, `ModeUpdate`, start timer
  - `update()`: on `Timer` — if visible, update CWD from focused pane, read specrate files and git info, re-arm timer; on `RunCommandResult` — parse stdout, update state fields only if changed, chain reads for ticket/state; on `ModeUpdate` — store mode_info
  - `pipe()`: handle `"toggle"` message — flip `visible`, immediately fetch data if becoming visible
  - `render()`: if hidden, return early. Left side: opaque " ZZZ" text + ID/Ticket/State ribbons (values colored red via `color_range(0, ...)`). Right side: opaque " Git" text + Branch/Commit ribbons (values colored red). Opaque padding fill between left and right.

## Focused Pane CWD Tracking

- [x] Use `get_focused_pane_info()` and `get_pane_cwd()` to get the focused pane's working directory
- [x] Run all file reads and git commands relative to the focused pane's CWD via `run_command_with_env_variables_and_cwd`
- [x] Store CWD in `self.cwd` and update every timer tick

## Plugin Registration

- [x] Add `"default-plugins/zzz-status-bar"` to `[workspace] members` in root `Cargo.toml`
- [x] Add `WorkspaceMember { crate_name: "default-plugins/zzz-status-bar", build: true }` in `xtask/src/main.rs` `workspace_members()`
- [x] Add `add_plugin!(assets, "zzz-status-bar.wasm")` in `zellij-utils/src/consts.rs` `ASSET_MAP`
- [x] Add `|| tag == "zzz-status-bar"` in `zellij-utils/src/input/plugins.rs` `from_run_plugin()` tag match

## Status-Bar Modification (Ctrl+Z Tile)

- [x] In `default-plugins/status-bar/src/first_line.rs`:
  - Add `KeyAction::Zzz` variant to `KeyAction` enum
  - Add `full_text()` return `"ZZZ"` and `short_text()` return `"Zz"` arms
  - In `first_line()` function, append a new `KeyShortcut` for `KeyAction::Zzz` to `default_keys` vec after the Quit entry, with hardcoded `Ctrl+Z` key

## Keybinding and Layout

- [x] In `zellij-utils/assets/config/default.kdl`, add to `shared_except "locked"` section: `bind "Ctrl z" { MessagePlugin { name "toggle"; } }` (broadcast to all plugins)
- [x] In `zellij-utils/assets/layouts/default.kdl`, add a new pane row above the status-bar pane:
  ```
  pane size=1 borderless=true {
      plugin location="zzz-status-bar"
  }
  ```
- [x] In `zellij-utils/assets/layouts/default.swap.kdl`, add `zzz-status-bar` pane to the `tab_template` between `children` and `status-bar`
- [x] In `~/.config/zellij/config.kdl`, add `Ctrl z` binding to `shared_except "locked"` section (required because user config has `clear-defaults=true`)

## Build and Verify

- [x] All WASM plugins compile successfully
- [x] `cargo xtask run` — zzz-status-bar renders correctly with arrow separators, toggles with Ctrl+Z, shows specrate info and git branch/commit
- [ ] Run `cargo xtask test` to verify no regressions

## Notes

- Rendering uses zellij's native `serialize_ribbon`/`serialize_text` API (not raw ANSI). The server handles arrow font rendering.
- Does not call `request_permission()` — no permissions needed for `run_command` in the plugin system.
- For `run_command` result disambiguation, uses the `context` BTreeMap to tag each command (`"type" => "work"`, `"ticket"`, `"state"`, `"git_branch"`, `"git_commit"`).
- When `.specrate/work` is missing or `cat` returns non-zero exit code, sets all specrate fields to `"-"`.
- Skips polling when hidden; only re-renders when data actually changes.
