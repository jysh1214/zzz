# fix: show commit_message instead of description in review-mode

The tequila-review-mode plugin was reading a `description` file for each subtask, but the tequila convention uses `commit_message` to describe subtask intent. This mismatch meant the description area above each diff was always empty (since `description` files don't exist for most subtasks).

- Change `read_description_file` to read `commit_message` instead of `description`
- Rename `SubtaskInfo` fields from `description`/`description_loaded` to `commit_message`/`commit_message_loaded`
- Update the command result handler context type from `"read-description"` to `"read-commit-message"`
