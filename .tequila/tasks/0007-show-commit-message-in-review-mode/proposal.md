# Proposal of *Show Commit Message in Review-Mode*

## Motivation

The tequila-review-mode plugin currently reads and displays a `description` file for each subtask. However, per the tequila convention, each subtask directory contains a `commit_message` file (not `description`) that describes the subtask's intent and changes. The plugin should display `commit_message` content instead so reviewers see the actual commit context above each diff.

## Summary

- Replace `read_description_file` to read `commit_message` instead of `description`
- Rename the `description` field on `SubtaskInfo` and the `description_loaded` flag to reflect the new source (or keep the field name and just change the file path)
- Update the `handle_command_result` context type accordingly

## Impact

- Affected code: `default-plugins/tequila-review-mode/src/main.rs` — `read_description_file`, `SubtaskInfo` struct, `handle_command_result` (`"read-description"` branch), and `all_files_loaded`
