# Proposal of *Update Status Bar to Tequila*

## Motivation

The project has moved from the specrate workflow to tequila for task management. The `zzz-status-bar` plugin still references specrate paths (`.specrate/work`, `.specrate/changes/`) and displays a "ZZZ" label. It needs to be updated to integrate with tequila (`.tequila/work`, `.tequila/tasks/`) and renamed to `tequila-status-bar` to reflect its new purpose. Additionally, the plugin should be visible by default instead of starting hidden.

## Summary

- Change the display label from "ZZZ" to "Task" in the status bar rendering
- Update filesystem detection from `.specrate/` paths to `.tequila/` paths (`.tequila/work`, `.tequila/tasks/`)
- Read task-id from `.tequila/work` instead of `.specrate/work`
- Update downstream file reads to use `.tequila/tasks/<task-id>/` instead of `.specrate/changes/<change-id>/`
- Set the plugin to visible by default (`self.visible = true` in `load()`)
- **BREAKING**: Rename plugin from `zzz-status-bar` to `tequila-status-bar` across all references:
  - Rename directory `default-plugins/zzz-status-bar/` to `default-plugins/tequila-status-bar/`
  - Update `Cargo.toml` (workspace member and package name)
  - Update `xtask/src/main.rs` (build system crate reference)
  - Update `zellij-utils/src/consts.rs` (ASSET_MAP plugin entry)
  - Update `zellij-utils/src/input/plugins.rs` (plugin tag match)
  - Update `zellij-utils/assets/config/default.kdl` (plugin alias)
  - Update `zellij-utils/assets/layouts/default.kdl` and `default.swap.kdl` (plugin location)
  - Rename and recompile `zellij-utils/assets/plugins/zzz-status-bar.wasm` to `tequila-status-bar.wasm`

## Impact

- Affected code:
  - `default-plugins/zzz-status-bar/` (renamed to `tequila-status-bar/`) — `src/main.rs`, `Cargo.toml`
  - `Cargo.toml` (workspace root)
  - `xtask/src/main.rs`
  - `zellij-utils/src/consts.rs`
  - `zellij-utils/src/input/plugins.rs`
  - `zellij-utils/assets/config/default.kdl`
  - `zellij-utils/assets/layouts/default.kdl`
  - `zellij-utils/assets/layouts/default.swap.kdl`
  - `zellij-utils/assets/plugins/zzz-status-bar.wasm` (replaced by `tequila-status-bar.wasm`)
