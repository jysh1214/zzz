# Proposal of *Fix Review-Mode Command Injection*

## Motivation

The `approve_current` and `cancel_approve_current` methods in `tequila-review-mode` build shell commands via `format!` and pass them to `sh -c`:

```rust
let cmd_str = format!("echo APPROVED > {}", path);  // approve_current
let cmd_str = format!("rm -f {}", path);             // cancel_approve_current
```

The `path` is constructed from `self.task_id` (read from `.tequila/work`) and `subtask.name` (read from `ls`). A subtask directory name containing shell metacharacters (e.g., `foo; rm -rf /`) would result in arbitrary command execution. This is a command injection vulnerability.

## Summary

- Replace `sh -c "echo APPROVED > {path}"` with a direct `run_command` invocation that avoids shell interpretation (e.g., `["sh", "-c", "cat > \"$1\"", "--", &path]` with stdin, or `["tee", &path]`, or a dedicated write approach)
- Replace `sh -c "rm -f {path}"` with `["rm", "-f", &path]` passed directly to `run_command`, bypassing the shell entirely
- Both fixes eliminate the `sh -c` + string interpolation pattern that enables injection

## Impact

- Affected code: `default-plugins/tequila-review-mode/src/main.rs` (`approve_current` at line 306, `cancel_approve_current` at line 320)
