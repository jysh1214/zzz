# Proposal of *Add Description to Review Mode*

## Motivation

When reviewing subtask patches in the tequila-review-mode plugin, reviewers only see the diff. The subtask `description` file (`.tequila/tasks/{task-id}/subtasks/{index}-{subtask-name}/description`) contains important context about the subtask's purpose and its role in the overall task. Displaying this description above the diff gives reviewers immediate context without needing to open the file separately.

## Summary

- Load the `description` file for each subtask alongside the existing `patch` and `state` files
- Display the description text between the status bar and the diff in the review pane
- The description area should be visually distinct from the diff and scroll independently or be rendered in full above the scrollable diff region

## Impact

- Affected code: `default-plugins/tequila-review-mode/src/main.rs` (data loading, state, and rendering logic)
