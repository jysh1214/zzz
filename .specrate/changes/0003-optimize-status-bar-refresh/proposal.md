# Proposal of *Optimize Status Bar Refresh*

## Motivation

The zzz-status-bar plugin currently polls every 1 second, spawning multiple shell commands (`cat`, `git rev-parse` x2) on each tick regardless of whether underlying data has changed. This causes unnecessary CPU and process overhead, especially on slower machines or when many panes are open. Since specrate files and git refs change infrequently (only on explicit user actions like commits, checkouts, or specrate commands), a polling-based approach is wasteful.

## Summary

- Subscribe to `FileSystemCreate` and `FileSystemUpdate` events to detect changes to `.specrate/work`, `.specrate/changes/*/ticket`, `.specrate/changes/*/state`, and `.git/HEAD` instantly
- Replace the 1-second polling timer with a 10-second fallback timer that only serves as a safety net for edge cases (e.g., external file changes not caught by the watcher)
- On file-system events, filter paths to only trigger data refresh when relevant files are modified
- Keep the existing `RunCommandResult`-based async command pattern unchanged
- Maintain the existing behavior of skipping all work when the bar is hidden

## Impact

- Affected specs: specrate-visualization
- Affected code: `default-plugins/zzz-status-bar/src/main.rs`
