# Validation of *Add Review Mode*

## Validation Method

Manual user review: the user builds the plugin, launches it with `Alt+R`, and verifies behavior matches the proposal.

## Steps

1. Build the plugin: `cargo xtask build`
2. Restart Zellij (keybinding config requires restart)
3. Press `Alt+R` — a floating pane should appear at the top ~75% of the screen
4. Verify the plugin reads `.tequila/work` and discovers subtask patches
5. Verify side-by-side diff rendering with `+`/`-` markers and red/green colors
6. Verify `←`/`→` navigates between patches, mouse scrolls, `a` approves, `q` quits
7. Verify "NO PATCHES TO REVIEW" is shown when no tequila data exists
8. Verify the Claude command line at the bottom is not covered

## Expected Outcome

Plugin launches as a floating pane, displays diffs correctly, and all keybindings work.

## Result

- Status: PASS
- User confirmed LGTM on 2026-03-05
