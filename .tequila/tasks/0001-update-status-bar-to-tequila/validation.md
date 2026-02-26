# Validation of *Update Status Bar to Tequila*

## Validation Method

Verify that all references to `zzz-status-bar` and `.specrate` have been replaced, the plugin builds successfully, and the renamed plugin is present in assets.

## Steps

1. Search the codebase for `zzz-status-bar` — should only appear in `.tequila/` task documents
2. Search the codebase for `.specrate` — should return no results
3. Verify `default-plugins/tequila-status-bar/` exists with updated `Cargo.toml` and `src/main.rs`
4. Verify `zellij-utils/assets/plugins/tequila-status-bar.wasm` exists
5. Run `cargo xtask build` — should succeed without errors

## Expected Outcome

- No source code references to `zzz-status-bar` or `.specrate` outside of task documentation
- Plugin directory renamed, display label is "Task", paths point to `.tequila/`, `visible` defaults to `true`
- Build completes successfully

## Result

- Status: PASS
- Validated by user review. Build verified successfully.
