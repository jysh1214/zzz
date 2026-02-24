# Tasks of *Optimize Status Bar Refresh*

## Event subscriptions

- [x] Add `EventType::FileSystemCreate` and `EventType::FileSystemUpdate` to the `subscribe` call in `load()`
- [x] Change `set_timeout(1.0)` to `set_timeout(10.0)` in `load()` and in the `Timer` event handler

## Path filtering and event handling

- [x] Add `Event::FileSystemCreate` match arm that calls a path filter helper and triggers targeted refresh
- [x] Add `Event::FileSystemUpdate` match arm that calls the same path filter helper and triggers targeted refresh
- [x] Implement `has_specrate_path` helper: returns true if any path contains `.specrate/work` or `.specrate/changes/`
- [x] Implement `has_git_path` helper: returns true if any path ends with `.git/HEAD`

## Refactor refresh logic

- [x] Extract `refresh_all` helper that calls `update_cwd`, `read_work_file`, `read_git_branch`, `read_git_commit`
- [x] Extract `refresh_specrate` helper that calls `update_cwd`, `read_work_file`
- [x] Extract `refresh_git` helper that calls `update_cwd`, `read_git_branch`, `read_git_commit`
- [x] Replace inline refresh calls in `Timer`, `pipe`, and the new filesystem handlers with the helpers

## Build and verify

- [x] Build the plugin with `cargo xtask build`
