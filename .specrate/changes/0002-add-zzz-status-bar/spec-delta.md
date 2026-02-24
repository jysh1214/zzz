# Spec Delta of *Add Zzz Status Bar*

## Apply to Spec *Specrate Visualization* (NEW)

### Requirement: Specrate Status Bar Display (ADDED)

The system **SHALL** provide a status bar that displays the currently active specrate change information using zellij's native ribbon API. The left side **SHALL** show an opaque "ZZZ" label followed by ribbons for ID, Ticket, and State with values in red. The right side **SHALL** show an opaque "Git" label followed by ribbons for Branch and Commit with values in red, right-aligned with opaque padding fill.

#### Scenario: Active change exists with all metadata

- **WHEN** `.specrate/work` exists and contains a valid change ID
- **AND** `.specrate/changes/{change_id}/ticket` exists
- **AND** `.specrate/changes/{change_id}/state` exists
- **THEN** the system displays the specrate info in ribbons on the left side

#### Scenario: Missing ticket or state

- **WHEN** `.specrate/work` exists and contains a valid change ID
- **AND** `.specrate/changes/{change_id}/ticket` or `.specrate/changes/{change_id}/state` does not exist or is empty
- **THEN** the system displays `-` for the missing field(s)

#### Scenario: No active change

- **WHEN** `.specrate/work` does not exist or is empty
- **THEN** the system displays `-` for the change ID, ticket, and state fields

### Requirement: Git Context Display (ADDED)

The system **SHALL** display the current git branch and short commit hash on the right side of the status bar, using the focused pane's working directory.

#### Scenario: Pane is in a git repository

- **WHEN** the focused pane's working directory is inside a git repository
- **THEN** the system displays the branch name and short commit hash in ribbons

#### Scenario: Pane is not in a git repository

- **WHEN** the focused pane's working directory is not inside a git repository
- **THEN** the system displays `-` for branch and commit

### Requirement: Focused Pane CWD Tracking (ADDED)

The system **SHALL** read all specrate files and run all git commands relative to the focused pane's working directory, so the bar reflects the context of the selected pane.

#### Scenario: User switches focus to a different pane

- **WHEN** the user focuses a pane with a different working directory
- **THEN** the status bar updates to show specrate and git info for that pane's directory

### Requirement: Toggle Visibility (ADDED)

The system **SHALL** be hidden by default and toggled via `Ctrl+Z` keybinding. The shortcut bar **SHALL** display a `Ctrl+Z` tile for discoverability.

#### Scenario: User toggles the bar on

- **IF** the status bar is currently hidden
- **WHEN** the user presses `Ctrl+Z`
- **THEN** the status bar becomes visible and immediately fetches data

#### Scenario: User toggles the bar off

- **IF** the status bar is currently visible
- **WHEN** the user presses `Ctrl+Z`
- **THEN** the status bar is hidden

### Requirement: Auto-Refresh (ADDED)

The system **SHALL** re-read specrate files and git info every 1 second when visible. The system **SHALL** skip polling when hidden. The system **SHALL** only trigger a re-render when data actually changes.
