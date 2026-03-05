# Validation of *Fix Review-Mode Scroll Offset Clamping*

## Validation Method

Manual testing: open the review-mode plugin, scroll to the bottom of a diff, then verify that pressing Up/PageUp scrolls back up immediately.

## Steps

1. Build the plugin: `cargo xtask build`
2. Launch Zellij and open a project with tequila subtask patches
3. Press Alt+V to enter review mode
4. Press Down or PageDown repeatedly until the diff view reaches the bottom (no more content below)
5. Press Up or PageUp to scroll back up

## Expected Outcome

After reaching the bottom, pressing Up should immediately scroll the view up by one line. Pressing PageUp should scroll up by 10 lines. The view should not stay stuck at the bottom.

## Result

- Status: PASS
- User confirmed scroll-up works immediately after reaching the bottom
