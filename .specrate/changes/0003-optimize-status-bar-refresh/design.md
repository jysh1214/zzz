# Design of *Optimize Status Bar Refresh*

## Context

The zzz-status-bar plugin currently uses a 1-second `set_timeout` loop that spawns shell commands (`cat .specrate/work`, `git rev-parse` x2) every tick. This works but is wasteful — specrate files and git refs only change on explicit user actions. Zellij provides `FileSystemCreate`/`FileSystemUpdate` events that deliver changed file paths, allowing event-driven refresh.

## Goals / Non-Goals

- Goals: Reduce CPU/process overhead by replacing 1s polling with file-system events + 10s fallback timer
- Non-goals: Changing the rendering logic, command execution pattern, or toggle behavior

## Decisions

- Decision: Subscribe to `FileSystemCreate` and `FileSystemUpdate` events and filter paths in the `update` handler
- Alternatives considered: (1) Only increase timer interval — simple but still blind polling; (2) Pure event-driven with no timer — risky if file watcher misses events (e.g., changes made outside Zellij CWD)

- Decision: Use path substring matching (`.specrate/work`, `.specrate/changes/`, `.git/HEAD`) to filter relevant events
- Alternatives considered: Exact path matching — too rigid since FileSystemUpdate paths may be absolute or relative depending on Zellij internals

- Decision: Add a `refresh_all` helper to consolidate the repeated `update_cwd` + `read_work_file` + `read_git_branch` + `read_git_commit` call pattern, and separate helpers `refresh_specrate` / `refresh_git` for targeted refresh
- Alternatives considered: Keep inline calls — leads to more duplication with the new event handlers

## Risks / Trade-Offs

- FileSystemUpdate events fire for the Zellij session CWD, which may differ from the focused pane's CWD. The 10s fallback timer mitigates this by periodically refreshing regardless.
- The file watcher may generate many events (e.g., during `git checkout` which modifies many files). Filtering to only relevant paths keeps this cheap, and the async command pattern means we just queue commands without blocking.

## Migration Plan

- Single-file change to `default-plugins/zzz-status-bar/src/main.rs` — no migration needed
- Fully backward compatible; no config or layout changes
