# Subtasks of *Update Status Bar to Tequila*

## Plugin Source Changes

- [x] In `src/main.rs`: change `self.visible = false` to `self.visible = true` in `load()`
- [x] In `src/main.rs`: change display label from `" ZZZ"` to `" Task"` and update comment
- [x] In `src/main.rs`: update `has_specrate_path()` to `has_tequila_path()` — detect `.tequila/work` and `.tequila/tasks/` instead of `.specrate/work` and `.specrate/changes/`
- [x] In `src/main.rs`: update `refresh_specrate()` to `refresh_tequila()` and update caller
- [x] In `src/main.rs`: rename `change_id` field to `task_id` in the `State` struct and all usages
- [x] In `src/main.rs`: update `read_work_file()` to read `.tequila/work` instead of `.specrate/work`
- [x] In `src/main.rs`: update `read_ticket_file()` to read `.tequila/tasks/<id>/ticket` instead of `.specrate/changes/<id>/ticket`
- [x] In `src/main.rs`: update `read_state_file()` to read `.tequila/tasks/<id>/state` instead of `.specrate/changes/<id>/state`

## Rename Plugin Crate

- [x] Rename directory `default-plugins/zzz-status-bar/` to `default-plugins/tequila-status-bar/`
- [x] Update `default-plugins/tequila-status-bar/Cargo.toml`: package name to `tequila-status-bar`

## Update Build System and Config

- [x] Update `Cargo.toml` (workspace root): member path from `default-plugins/zzz-status-bar` to `default-plugins/tequila-status-bar`
- [x] Update `xtask/src/main.rs`: crate_name from `default-plugins/zzz-status-bar` to `default-plugins/tequila-status-bar`
- [x] Update `zellij-utils/src/consts.rs`: `add_plugin!` macro from `zzz-status-bar.wasm` to `tequila-status-bar.wasm`
- [x] Update `zellij-utils/src/input/plugins.rs`: tag match from `zzz-status-bar` to `tequila-status-bar`

## Update Layout and Config Files

- [x] Update `zellij-utils/assets/config/default.kdl`: plugin alias from `zzz-status-bar` to `tequila-status-bar`
- [x] Update `zellij-utils/assets/layouts/default.kdl`: plugin location from `zzz-status-bar` to `tequila-status-bar`
- [x] Update `zellij-utils/assets/layouts/default.swap.kdl`: plugin location from `zzz-status-bar` to `tequila-status-bar`

## Build and Asset Update

- [x] Remove old `zellij-utils/assets/plugins/zzz-status-bar.wasm`
- [x] Build plugin with `cargo xtask build` to produce `tequila-status-bar.wasm`
- [x] Verify build succeeds
