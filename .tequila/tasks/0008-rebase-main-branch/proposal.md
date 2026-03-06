# Proposal of *Rebase Main Branch*

## Motivation

The `dev/alexchiang/zzz` branch has diverged from `main` over several commits. Rebasing onto the latest `main` will incorporate any upstream changes, reduce merge conflicts down the line, and keep the branch history linear and clean before preparing a PR.

## Summary

- Fetch the latest `main` from origin
- Rebase `dev/alexchiang/zzz` onto `main`, resolving any conflicts that arise
- Verify the branch builds and tests pass after the rebase

## Impact

- Affected code: all files on `dev/alexchiang/zzz` that differ from `main`; commit history will be rewritten (force-push required)
