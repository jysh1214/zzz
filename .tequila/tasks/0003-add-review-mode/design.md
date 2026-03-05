# Design of *Add Review Mode*

## Context

Tequila tasks produce per-subtask `patch` files after implementation. Reviewers currently lack an in-Zellij way to inspect diffs, approve subtasks, and navigate between patches. The user runs Claude in the main pane and needs the review UI to float above without covering the Claude command line at the bottom.

Zellij plugins are WASM modules (`wasm32-wasip1`) that implement the `ZellijPlugin` trait. They cannot access the host filesystem directly — file reading is done via `run_command` with `cat`, and file writing via `run_command` with shell redirects. The `tequila-status-bar` plugin already demonstrates this pattern for reading `.tequila/work`, ticket, and state files.

## Goals / Non-Goals

- Goals:
  - A new `tequila-review-mode` plugin that renders a side-by-side diff of the current subtask patch
  - `+`/`-` prefixes and color coding for additions/deletions
  - Keybindings to approve the current patch and navigate between patches
  - Floating pane positioned to leave the bottom rows uncovered
- Non-goals:
  - Inline editing of patches
  - Comment input bar (Claude runs in the main pane)
  - Reject/request-changes workflow (only approve for now)
  - Integration with git — operates purely on `.tequila/` patch files

## Decisions

- **File I/O via `run_command`**: Use `run_command(&["cat", path], context)` to read patch files, `.tequila/work`, and subtask directory listings. Use `run_command` with `sh -c "echo APPROVED > path"` to write approval state. This follows the proven pattern from `tequila-status-bar`.
- **Subtask discovery via `ls` + `run_command`**: List subtask directories with `ls .tequila/tasks/{id}/subtasks/`, then read each patch file on demand. No need for `scan_host_folder` since we need file contents, not just listings.
- **Unified diff parser in Rust**: Parse the unified diff format directly in the plugin. Extract hunks, map old/new lines side-by-side. This is straightforward string parsing — no external crate needed.
- **ANSI rendering**: Use `print!` with ANSI escape codes for cursor positioning and coloring. Green foreground for additions (`+`), red for deletions (`-`), dim for context lines. Two columns separated by a vertical bar `│`.
- **Floating pane positioning**: Configure via KDL layout with `y 0; height "75%"` to anchor at the top and leave ~25% of screen height for the Claude command line below.
- **Scroll support**: Arrow keys / `j`/`k` for vertical scrolling through long diffs. Track scroll offset in plugin state.

## Risks / Trade-Offs

- **Large patch files**: Very large diffs may be slow to render. Mitigation: only render visible lines based on scroll offset and available rows.
- **Async file reads**: `run_command` is async — the plugin must handle the loading state gracefully (show "Loading..." until the first `RunCommandResult` arrives).
- **No direct file write API**: Writing the approval state requires spawning a shell command. This is slightly fragile but is the established pattern in the codebase.

## Migration Plan

No migration needed — this is a new plugin with no impact on existing functionality.
