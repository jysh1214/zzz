# Proposal of *Add Zzz Status Bar*

## Motivation

Users working with specrate-managed changes need at-a-glance visibility into the currently active change (ID, ticket, and state) without leaving the terminal or running separate commands. Additionally, git context (branch and commit) is useful alongside specrate info. A dedicated status bar in zzz provides persistent context directly in the workspace.

## Summary

- Add a new built-in plugin (`zzz-status-bar`) that renders a 1-row info bar using zellij's native ribbon API (`serialize_ribbon`/`serialize_text`)
- Left side: opaque "ZZZ" label followed by ribbons for `ID`, `Ticket`, and `State` (values in red, labels in default color)
- Right side: opaque "Git" label followed by ribbons for `Branch` and `Commit` (values in red), right-aligned with opaque padding fill
- Values are read from `.specrate/work`, `.specrate/changes/{change_id}/ticket`, `.specrate/changes/{change_id}/state`, `git rev-parse --abbrev-ref HEAD`, and `git rev-parse --short HEAD`; show `-` when any value is missing
- All file reads and git commands use the focused pane's working directory (`get_focused_pane_info` + `get_pane_cwd`), so the bar reflects the context of the selected pane
- Hidden by default; toggled via `Ctrl+Z` keybinding (broadcast `MessagePlugin` with name `"toggle"`)
- Auto-refresh every 1 second when visible; skips polling when hidden to save resources
- Only triggers re-render when data actually changes
- Place the bar in the default layout and swap layout template above the existing shortcut bar

## Impact

- Affected specs: specrate-visualization (new)
- Affected code: `default-plugins/zzz-status-bar/` (new plugin crate), `default-plugins/status-bar/src/first_line.rs` (Ctrl+Z tile), `zellij-utils/assets/layouts/default.kdl`, `zellij-utils/assets/layouts/default.swap.kdl`, `zellij-utils/assets/config/default.kdl`, `zellij-utils/src/input/plugins.rs`, `zellij-utils/src/consts.rs`, `xtask/src/main.rs`
- User config: `~/.config/zellij/config.kdl` requires `Ctrl z` binding if `clear-defaults=true` is set
