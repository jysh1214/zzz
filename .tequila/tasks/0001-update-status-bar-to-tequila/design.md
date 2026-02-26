# Design of *Update Status Bar to Tequila*

## Context

The `zzz-status-bar` plugin was built to display specrate workflow state (change-id, ticket, state) and git context in a status bar. The project has migrated from specrate to tequila for task management. The tequila folder structure differs from specrate:

- specrate: `.specrate/work`, `.specrate/changes/<id>/ticket`, `.specrate/changes/<id>/state`
- tequila: `.tequila/work`, `.tequila/tasks/<id>/ticket`, `.tequila/tasks/<id>/state`

The plugin must be updated to reflect this new structure, renamed from `zzz-status-bar` to `tequila-status-bar`, and made visible by default.

## Goals / Non-Goals

- Goals:
  - Display label changes from "ZZZ" to "Task"
  - All `.specrate/` path references become `.tequila/` equivalents
  - `.specrate/changes/` becomes `.tequila/tasks/`
  - Plugin is visible by default (`self.visible = true` in `load()`)
  - Plugin renamed to `tequila-status-bar` everywhere (directory, crate name, layout files, config, build system, asset map)
  - Method names updated from `specrate` to `tequila` for clarity
- Non-goals:
  - Changing the plugin's architecture or event-driven refresh mechanism
  - Changing the git integration (branch/commit display)
  - Changing the toggle keybinding (`Ctrl z`)
  - Adding new features beyond the rename and path updates

## Decisions

- **Decision: Rename `refresh_specrate()` to `refresh_tequila()` and `has_specrate_path()` to `has_tequila_path()`** for consistency with the new naming. These are internal method names with no external API surface.
- **Decision: Rename the `change_id` field to `task_id` in the State struct.** This aligns the internal naming with tequila terminology ("task-id") for clarity and consistency.
- **Decision: Keep the toggle mechanism via `Ctrl z` / `MessagePlugin { name "toggle" }`.** The plugin starts visible but can still be toggled off.

## Risks / Trade-Offs

- The `.wasm` binary is checked into the repo. It must be recompiled after the rename. This is a build step, not a code risk.
- Renaming the directory requires updating the workspace `Cargo.toml` members list and `xtask` build config. If any reference is missed, the build will fail immediately, making it easy to catch.

## Migration Plan

1. Rename directory and update all references (single atomic set of changes)
2. Rebuild the plugin to produce `tequila-status-bar.wasm`
3. Remove old `zzz-status-bar.wasm` from assets
4. Verify build succeeds with `cargo xtask build`
