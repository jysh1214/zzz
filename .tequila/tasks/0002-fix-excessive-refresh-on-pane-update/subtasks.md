# Subtasks of *Fix Excessive Refresh On Pane Update*

## Implementation

- [x] In the `PaneUpdate` handler, resolve the focused pane's cwd and compare it against the stored `self.cwd`; only call `refresh_all()` when the cwd has actually changed
- [x] Verify the plugin builds with `cargo build` targeting `wasm32-wasip1`
