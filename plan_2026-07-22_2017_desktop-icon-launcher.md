# Plan: Desktop Icon Launcher — 2026-07-22 20:17

## Status
COMPLETED

## Goal
Make the AURA desktop launch path unmistakable: create a proper AURA Windows icon, embed it in
the launcher executable, assign it to the Desktop/Start Menu shortcuts, and install shortcuts
on both the user and public desktop surfaces.

## Context
- Relevant modules / scripts:
  - `crates/aura_launcher` — Bevy launcher executable
  - `scripts/install_launcher_shortcut.ps1` — current shortcut installer
- Files read:
  - `AGENTS.md`
  - `CLAUDE.md`
  - `README.md`
  - `scripts/install_launcher_shortcut.ps1`
  - `crates/aura_launcher/Cargo.toml`
- Findings:
  - `C:\Users\m\Desktop\AURA.lnk` exists.
  - `C:\Users\Public\Desktop\AURA.lnk` does not exist.
  - The shortcut installer does not set a custom icon.
- Files to edit:
  - `crates/aura_launcher/Cargo.toml`
  - `crates/aura_launcher/build.rs`
  - `scripts/install_launcher_shortcut.ps1`
  - `README.md`
  - this plan
- Files to add:
  - `assets/icon/aura.ico`

## Steps

### Step 1 — Add AURA icon asset
- [x] Action: Generate a tracked Windows `.ico` asset with multiple sizes for the launcher and
  shortcut.
- Files touched: `assets/icon/aura.ico`
- Expected outcome: A stable icon file exists in repo source.

### Step 2 — Embed icon in launcher exe
- [x] Action: Add Windows resource build wiring for `aura_launcher`.
- Files touched: `crates/aura_launcher/Cargo.toml`, `crates/aura_launcher/build.rs`
- Expected outcome: release launcher carries the AURA icon as its executable icon.

### Step 3 — Install robust shortcuts
- [x] Action: Update shortcut installer to copy the icon into `dist`, set `IconLocation`, and
  create shortcuts for user Desktop, Start Menu, and Public Desktop when Windows permissions
  allow it.
- Files touched: `scripts/install_launcher_shortcut.ps1`
- Expected outcome: AURA icon is visible from the current user's Desktop and launches
  `dist\aura_launcher.exe`; Public Desktop refusal is a warning, not a failed install.

### Step 4 — Update docs
- [x] Action: Document the actual shortcut/icon behavior.
- Files touched: `README.md`
- Expected outcome: Repo truth matches the desktop install path.

### Step 5 — Verify, commit, push
- [x] Action: Run formatting/tests/build/install checks, inspect shortcut targets/icon
  locations, commit by explicit pathspec, and push `main`.
- Files touched: git metadata only
- Expected outcome: Working tree clean, `HEAD == origin/main`, and the desktop shortcut
  launches the real AURA executable.

## Test gate
- `cargo fmt -p aura_launcher -- --check`
- `cargo test -p aura_launcher`
- `pwsh -File scripts\install_launcher_shortcut.ps1`
- bounded smoke run of `dist\aura_launcher.exe`
- shortcut inspection for target and icon location
- `git diff --check`
- `git status --short --branch`

## Rollback
Do not delete. If the icon or shortcut path is wrong, add a superseding commit that changes
the icon or shortcut target while preserving history.

## Next-agent pickup
If interrupted:
1. Read this document.
2. Run `git status --short --branch`.
3. Check the first unchecked step and continue there.
