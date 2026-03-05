# Validation of *Show Commit Message in Review-Mode*

## Validation Method

Manual code review to confirm the plugin reads `commit_message` instead of `description` for each subtask.

## Steps

1. Open `default-plugins/tequila-review-mode/src/main.rs`
2. Verify `SubtaskInfo` fields are `commit_message` and `commit_message_loaded`
3. Verify `read_commit_message_file` reads `commit_message` file path
4. Verify handler matches on `"read-commit-message"` and populates the correct fields
5. Verify `all_files_loaded` checks `commit_message_loaded`
6. Verify `render_diff` passes `subtask.commit_message` to `wrap_text`
7. Confirm no remaining references to `description` or `read-description`
8. Confirm plugin compiles cleanly

## Expected Outcome

All references to `description` are replaced with `commit_message`, and the plugin reads the correct tequila convention file.

## Result

- Status: PASS
- Code review confirmed all references updated correctly; compilation clean
