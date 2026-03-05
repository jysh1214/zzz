# fix: eliminate command injection in review-mode approve/cancel

The `approve_current` and `cancel_approve_current` methods in the tequila-review-mode plugin interpolate filesystem-derived values (`task_id`, `subtask.name`) directly into `sh -c` command strings. A malicious subtask directory name containing shell metacharacters could execute arbitrary commands.

- Replace `format!("echo APPROVED > {}", path)` with positional argument passing via `sh -c "echo APPROVED > \"$1\"" -- {path}`, so the path is never parsed as shell syntax
- Replace `format!("rm -f {}", path)` with direct `["rm", "-f", &path]` invocation, bypassing the shell entirely
