# Proposal of *Fix Excessive Refresh On Pane Update*

## Motivation

The `tequila-status-bar` plugin calls `refresh_all()` on every `PaneUpdate` event. This event fires frequently (pane focus changes, resizes, new panes, etc.), and each `refresh_all()` spawns 4 external processes (`cat .tequila/work`, `git rev-parse` x2, plus chained `cat` calls for ticket/state). This creates unnecessary load, especially in sessions with many panes or rapid focus switching.

## Summary

- Only refresh on `PaneUpdate` when the focused pane's cwd has actually changed compared to the last known cwd
- Store the previous cwd and compare before triggering command spawns
- Keep the existing timer-based and filesystem-event-based refresh paths unchanged

## Impact

- Affected code: `default-plugins/tequila-status-bar/src/main.rs` (the `PaneUpdate` handler and `State` struct)
