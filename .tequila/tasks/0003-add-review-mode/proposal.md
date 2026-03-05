# Proposal of *Add Review Mode*

## Motivation

Tequila tasks produce per-subtask patch files (`.tequila/tasks/{task-id}/subtasks/{index}-{subtask-name}/patch`) after implementation. Currently there is no convenient way to review these patches inside Zellij — the reviewer must manually run `git diff` or `git apply` commands and flip between terminal panes.

A dedicated review-mode plugin running in a pinned floating pane would streamline the workflow: the reviewer sees the diff side-by-side, can approve or skip to the next patch, and can send comments to Claude — all without leaving the multiplexer.

## Summary

- Add a new WASM plugin (`default-plugins/tequila-review-mode/`) that implements a patch review UI
- **Side-by-side diff view**: The plugin reads the patch file, parses the unified diff, and renders a two-column layout — old version (left) with deletions highlighted, new version (right) with additions highlighted. Lines are prefixed with `+`/`-` markers to clearly identify additions and deletions
- **Approve action**: A keybinding (e.g. `a`) writes `APPROVED` to the subtask's `state` file via `run_command`
- **Next/previous patch navigation**: Keybindings (e.g. `n`/`p`) cycle through subtask patches within the current task
- **Pinned floating pane**: The plugin is launched as a floating pane positioned at the top of the screen, sized to leave the bottom rows visible so it does not cover the main pane's Claude command line
- The plugin reads the task-id from `.tequila/work` and discovers subtask patches by scanning `.tequila/tasks/{task-id}/subtasks/`

## Impact

- Affected code: new crate `default-plugins/tequila-review-mode/` (plugin source), KDL layout additions, `xtask` build configuration to compile the new plugin
- No changes to existing plugins or core server/client code expected
- Requires permissions: `RunCommands` (to write state files), `ChangeApplicationState` (pane focus), and standard event subscriptions (`Key`, `Mouse`, `FileSystemUpdate`)
