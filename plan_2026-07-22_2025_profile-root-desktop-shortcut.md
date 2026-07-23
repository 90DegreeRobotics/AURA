# Plan: Profile Root Desktop Shortcut — 2026-07-22 20:25

## Status
COMPLETED

## Goal
Put the AURA launcher shortcut where the Founder says the visible Desktop lives:
`C:\Users\m`. The installer should continue creating the normal user Desktop and Start Menu
shortcuts, but must also create `C:\Users\m\AURA.lnk`.

## Context
- Relevant script: `scripts/install_launcher_shortcut.ps1`
- Existing state:
  - `C:\Users\m\Desktop\AURA.lnk` exists.
  - `C:\Users\m\AURA.lnk` does not exist.
- Files to edit:
  - `scripts/install_launcher_shortcut.ps1`
  - `README.md`
  - this plan

## Steps

### Step 1 — Add profile-root shortcut target
- [x] Action: Add `$env:USERPROFILE` as an explicit shortcut destination.
- Files touched: `scripts/install_launcher_shortcut.ps1`
- Expected outcome: installer creates `C:\Users\m\AURA.lnk`.

### Step 2 — Update docs
- [x] Action: Document that the installer creates the profile-root shortcut as well.
- Files touched: `README.md`
- Expected outcome: repo truth matches install behavior.

### Step 3 — Verify, commit, push
- [x] Action: Run installer, inspect the new shortcut target/icon, run lightweight checks,
  commit by explicit pathspec, and push `main`.
- Files touched: git metadata only
- Expected outcome: clean tree, `HEAD == origin/main`, and `C:\Users\m\AURA.lnk` launches the
  real app.

## Test gate
- `pwsh -File scripts\install_launcher_shortcut.ps1`
- shortcut inspection for `C:\Users\m\AURA.lnk`
- bounded shortcut launch smoke from `C:\Users\m\AURA.lnk`
- `git diff --check`
- `git status --short --branch`

## Rollback
Do not delete. If the profile-root shortcut is wrong, add a superseding commit removing that
destination from the installer logic.

## Next-agent pickup
If interrupted:
1. Read this document.
2. Run `git status --short --branch`.
3. Continue at the first unchecked step.
