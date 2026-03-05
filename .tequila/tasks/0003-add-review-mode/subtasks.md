# Subtasks of *Add Review Mode*

## Plugin Scaffolding

- [x] 001 — Scaffold plugin crate (`default-plugins/tequila-review-mode/`)

## Data Loading

- [x] 002 — Implement task discovery and file loading (read `.tequila/work`, list subtasks, read patches via `run_command`)

## Diff Processing

- [x] 003 — Implement unified diff parser (parse hunks, classify lines, produce side-by-side mapping)

## Rendering

- [x] 004 — Implement side-by-side renderer (two-column layout, `+`/`-` markers, ANSI colors, scroll support)

## Interaction

- [x] 005 — Implement keybinding actions (approve, next/prev, scroll, quit)

## Integration

- [x] 006 — Add build integration and layout (workspace members, xtask, KDL layout snippet)
