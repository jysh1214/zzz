# Design of *Add Zzz Status Bar*

## Context

Zzz (zellij) uses a plugin architecture where UI bars (tab-bar, status-bar) are WASM plugins that implement the `ZellijPlugin` trait. The existing status-bar plugin renders a two-line UI: the first line shows arrow-shaped mode tiles (LOCK, PANE, TAB, etc.) and the second line shows contextual keybinding hints.

WASM plugins cannot read host files directly. They use `run_command()` to execute host commands and receive results via `Event::RunCommandResult`. Periodic refresh uses `set_timeout()` + `Event::Timer`. Inter-plugin communication uses the pipe mechanism (`PipeMessage`).

The KDL config supports `MessagePlugin` action (maps to `Action::KeybindPipe`) which sends a pipe message to a named plugin when a keybinding is pressed.

## Goals / Non-Goals

- Goals:
  - New `zzz-status-bar` plugin displaying specrate change info in arrow-shaped segments
  - `Ctrl+Z` tile in the existing shortcut bar to toggle visibility
  - Auto-refresh every 1 second
- Non-goals:
  - Dynamic show/hide of the pane row itself (the pane always exists in the layout; when "hidden", it renders an empty line)
  - Editing specrate files from the bar
  - Supporting non-default layouts

## Decisions

- **Decision: New plugin crate (`default-plugins/zzz-status-bar/`)** rather than embedding into the existing status-bar plugin.
  - Rationale: Separate concerns — the new bar needs filesystem access (`RunCommands` permission), timer-based polling, and pipe message handling. The existing status-bar has none of these and should stay unchanged in its core logic.

- **Decision: Use `run_command(&["cat", path])` to read specrate files.**
  - Rationale: WASM plugins have no direct file I/O. `run_command` is the standard mechanism for reading host file contents (returns stdout via `RunCommandResult` event). Three `cat` commands per refresh cycle: `.specrate/work`, `.specrate/changes/{id}/ticket`, `.specrate/changes/{id}/state`.
  - Alternatives considered: `scan_host_folder` only returns file listings (paths + metadata), not contents.

- **Decision: Use `set_timeout(1.0)` + `Event::Timer` for 1-second refresh.**
  - Rationale: Standard pattern used by session-manager and other plugins. One-shot timer re-armed in the `Timer` event handler.

- **Decision: Use `MessagePlugin` keybinding action for `Ctrl+Z` toggle.**
  - KDL syntax: `bind "Ctrl z" { MessagePlugin "zzz-status-bar" { name "toggle"; } }`
  - This creates `Action::KeybindPipe` which routes a `PipeMessage` to the plugin's `pipe()` method. The plugin toggles its internal `visible` flag.
  - Alternatives considered: Adding a new `Action` enum variant requires changes across route.rs, screen.rs, protobuf definitions, and multiple serialization layers. `MessagePlugin` avoids all of that.

- **Decision: Add `KeyAction::Zzz` tile to status-bar first line.**
  - The tile displays as ` <z> ZZZ ` in the arrow bar, looked up via `action_key(binds, &[Action::KeybindPipe { .. }])` using `shallow_eq` matching.
  - Caveat: `shallow_eq` matches any `KeybindPipe` variant regardless of fields. This is acceptable because this is the only `KeybindPipe` binding in the default config. If multiple `KeybindPipe` bindings existed, the first match would be used.

- **Decision: Pane always present in layout, renders empty when hidden.**
  - The zzz-status-bar pane (1 row, borderless) is always in the default layout above the status-bar. When toggled off, `render()` outputs just the background fill (1 empty row). This avoids complex dynamic pane show/hide logic in the server.

## Risks / Trade-Offs

- When zzz-status-bar is "hidden", a 1-row blank line remains visible above the shortcut bar. Acceptable for MVP.
- The `run_command` approach spawns 1-3 `cat` processes per second. Negligible overhead for such small files.
- The `shallow_eq` matching for `KeybindPipe` in the status-bar tile lookup is fragile if more `MessagePlugin` keybindings are added. Can be revisited later.
- Plugin requires `RunCommands` and `ReadApplicationState` permissions, which trigger a one-time permission prompt.
