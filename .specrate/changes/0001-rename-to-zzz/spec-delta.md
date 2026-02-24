# Spec Delta of *Rename to Zzz*

## Create Spec *Product Identity*

> This change introduces the first spec in the repository, defining the user-facing identity of the product.

### Requirement: Binary Name (ADDED)

The system **SHALL** produce a compiled binary named `zzz`.

#### Scenario: User invokes the tool from a shell

- **WHEN** the user types `zzz` in a terminal
- **THEN** the system launches the terminal multiplexer

#### Scenario: User invokes a subcommand

- **WHEN** the user types `zzz <subcommand>` (e.g., `zzz setup`, `zzz list-sessions`)
- **THEN** the system executes the corresponding subcommand

### Requirement: CLI Help Identity (ADDED)

The system **SHALL** display `zzz` as the program name in all CLI help output (`--help`, `--version`, subcommand help).

#### Scenario: User requests version

- **WHEN** the user runs `zzz --version`
- **THEN** the output begins with `zzz`

### Requirement: Display Name (ADDED)

The system **SHALL** use "Zzz" (capital Z) as the product display name in all user-facing messages, including but not limited to:

- Startup and shutdown messages (e.g., "Starting Zzz server!", "Bye from Zzz!")
- Error messages (e.g., "This version of Zzz was compiled without...")
- Plugin pane titles (e.g., "About Zzz")
- Welcome screen and tips (e.g., "Welcome to Zzz {version}!", "Zzz Tip #1")

#### Scenario: User starts a session

- **WHEN** the system initializes
- **THEN** log and display messages reference "Zzz", not "Zellij"

### Requirement: Install Command (ADDED)

The system **SHALL** be installable via `cargo install --locked zzz`.

#### Scenario: User installs from crates.io

- **WHEN** the user runs `cargo install --locked zzz`
- **THEN** the `zzz` binary is compiled and placed in the user's Cargo bin directory

### Requirement: Packaging Artifacts (ADDED)

The system **SHALL** use the name `zzz` in all packaging artifacts:

- Man page: `zzz.1`
- Shell completions: `zzz.bash`, `zzz.fish`, `_zzz`
- Deb binary path: `usr/bin/zzz`
- Deb assets path: `usr/share/zzz/`

#### Scenario: User installs via deb package

- **WHEN** the user installs the `.deb` package
- **THEN** the binary is placed at `/usr/bin/zzz` and assets under `/usr/share/zzz/`

### Requirement: Internal Code Naming Unchanged (ADDED)

The system **SHALL NOT** rename internal Rust crate names, module names, struct names, enum names, or any code-level identifiers. The rename is limited to user-facing surfaces only.

#### Scenario: Developer reads the source code

- **WHEN** a developer inspects the Rust source
- **THEN** crate names like `zellij-server`, `zellij-client`, `zellij-utils`, etc. remain unchanged
