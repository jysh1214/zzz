# Validation of *Add Description to Review Mode*

## Validation Method

Manual visual inspection: build and run zellij with the updated tequila-review-mode plugin, open the review mode on a task with subtask descriptions, and verify the new layout renders correctly.

## Steps

1. Build the project with `cargo xtask build`
2. Run zellij and open the tequila review-mode plugin on a task that has subtask `description` files (e.g., task 0003 or 0004)
3. Confirm the description text appears between the status bar (row 1) and the diff
4. Confirm the description is rendered in dimmed style and word-wrapped to the terminal width
5. Confirm a horizontal separator line (`─`) appears below the description block
6. Confirm the diff area starts below the separator and scrolling (up/down/page up/page down) works correctly
7. Navigate between subtasks (left/right) and confirm each subtask's own description is shown

## Expected Outcome

The review mode pane layout should be: status bar → description (dimmed) → separator → diff → footer. Each subtask displays its own description. Scrolling and navigation remain functional.

## Result

- Status: PASS
- User confirmed the layout renders correctly via manual visual inspection
