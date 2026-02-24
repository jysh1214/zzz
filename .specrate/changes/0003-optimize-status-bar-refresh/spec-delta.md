# Spec Delta of *Optimize Status Bar Refresh*

## Apply to Spec *Specrate Visualization*

### Requirement: Auto-Refresh (MODIFIED)

The system **SHALL** subscribe to file-system events (`FileSystemCreate`, `FileSystemUpdate`) and refresh data immediately when relevant files change. The system **SHALL** use a 10-second fallback timer as a safety net. The system **SHALL** skip all refresh activity when hidden. The system **SHALL** only trigger a re-render when data actually changes.

#### Scenario: Specrate file changes

- **WHEN** a file matching `.specrate/work` or `.specrate/changes/*/ticket` or `.specrate/changes/*/state` is created or updated
- **AND** the status bar is visible
- **THEN** the system immediately re-reads the affected specrate data

#### Scenario: Git ref changes

- **WHEN** a file matching `.git/HEAD` is created or updated
- **AND** the status bar is visible
- **THEN** the system immediately re-reads git branch and commit info

#### Scenario: Fallback timer fires

- **WHEN** 10 seconds have elapsed since the last timer
- **AND** the status bar is visible
- **THEN** the system re-reads all specrate and git data as a safety net

#### Scenario: Unrelated file changes

- **WHEN** a file-system event fires for a path that does not match any watched pattern
- **THEN** the system does not trigger any data refresh

#### Scenario: Status bar is hidden

- **WHEN** any file-system event or timer fires
- **AND** the status bar is not visible
- **THEN** the system does not trigger any data refresh
