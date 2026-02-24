# Design of *Rename to Zzz*

## Context

The project currently ships a binary called `zellij` and uses "Zellij" as its display name throughout the UI. The user wants to rename all user-facing surfaces to "zzz"/"Zzz" while keeping the Rust crate and code identifiers unchanged. This is a cosmetic/branding rename, not an architectural change.

Key constraint: the internal Rust workspace structure (`zellij-utils`, `zellij-server`, `zellij-client`, `zellij-tile`, etc.) must remain untouched. Only strings and metadata visible to end users change.

## Goals / Non-Goals

- Goals:
  - Binary name becomes `zzz`
  - CLI help/version output says `zzz`
  - All user-visible messages say "Zzz" instead of "Zellij"
  - `cargo install --locked zzz` works (package name changes)
  - Packaging artifacts (man page, shell completions, deb metadata) use `zzz`
- Non-goals:
  - Renaming Rust crate names, module names, or type identifiers
  - Renaming the GitHub repository or organization
  - Changing external URLs (zellij.dev, GitHub links) — these are web properties, not part of the binary
  - Renaming environment variables (see Decisions below)
  - Renaming config/cache directory paths (see Decisions below)

## Decisions

- **Decision: Keep environment variable names as-is** (`ZELLIJ_CONFIG_FILE`, `ZELLIJ_CONFIG_DIR`, `ZELLIJ_SOCKET_DIR`).
  These are part of the programmatic interface and changing them would break existing user scripts and configurations. They also fall under "internal code naming" since they are defined as Rust constants.
  - Alternatives considered: Rename env vars to `ZZZ_*` — rejected due to breaking existing users and blurring the "no internal rename" boundary.

- **Decision: Keep config/cache directory paths as-is** (`~/.cache/zellij/`, `~/.config/zellij/`, `/tmp/zellij-{UID}/`).
  These paths are determined by `ProjectDirs::from("org", "Zellij Contributors", "Zellij")` and the `ZELLIJ_TMP_DIR` constant. Changing them would orphan existing user configurations and sessions. A migration path would add scope beyond this change.
  - Alternatives considered: Rename directories + add migration — rejected as out of scope.

- **Decision: Do not rename URLs** (e.g., `https://zellij.dev/*`, `https://github.com/zellij-org/*`).
  URLs point to external web properties that are not part of the binary rename scope.

- **Decision: Keep old config converter messages referencing "Zellij"**.
  The old YAML config converter (`old_config_converter/`) contains messages about migrating from the old "Zellij" YAML format. Since these refer to historical behavior, they should keep the original name for accuracy. However, command examples within those messages (e.g., `zellij --config`) should be updated to `zzz --config`.

- **Decision: Package name in root `Cargo.toml` changes to `zzz`**.
  This is required for `cargo install --locked zzz` to work. The internal workspace crate names (`zellij-utils`, `zellij-server`, etc.) remain unchanged since they are not published independently.

## Risks / Trade-Offs

- **Risk: Snapshot test failures.** Tests using `insta` snapshots may contain "Zellij" strings. Mitigation: run `cargo insta review` after changes and accept updated snapshots.
- **Risk: Missed occurrences.** A text search may miss some "Zellij" references. Mitigation: systematic grep-based sweep + build + test to catch any misses.
- **Trade-off: Dual naming.** The codebase will have `zellij` in code identifiers but `zzz` in user-facing strings. This is intentional and acceptable per the user's requirement.

## Migration Plan

1. Make all changes in a single branch
2. Build and test to verify nothing breaks
3. Update snapshots if needed
4. No runtime migration needed — this is a compile-time change

## Open Questions

- Should the `share` plugin message ("This version of Zellij was compiled without web sharing capabilities") also be renamed? (Assumed yes — it's user-facing.)
