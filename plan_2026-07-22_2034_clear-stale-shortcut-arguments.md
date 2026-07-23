# Plan: Clear Stale Shortcut Arguments — 2026-07-22 20:34

## Status
COMPLETED

## Goal
Fix the visible Desktop AURA shortcut at `C:\Users\m\Desktop\AURA.lnk` so it launches only
`C:\aura\dist\aura_launcher.exe`, with no stale `C:\newaura\launch-aura.ps1` arguments. Update
the installer so existing shortcut arguments are explicitly cleared and validated after save.

## Context
- User screenshot showed `AURA.lnk` still presenting:
  `-ExecutionPolicy Bypass -File "C:\newaura\launch-aura.ps1"`.
- COM inspection confirmed:
  - `TargetPath = C:\aura\dist\aura_launcher.exe`
  - `Arguments = -ExecutionPolicy Bypass -File "C:\newaura\launch-aura.ps1"`
- `C:\Users\m\AURA.lnk` had empty arguments; the visible Desktop shortcut did not.
- Files to edit:
  - `scripts/install_launcher_shortcut.ps1`
  - this plan

## Steps

### Step 1 — Clear shortcut arguments in installer
- [x] Action: Set `$shortcut.Arguments = ""` for every installed AURA shortcut.
- Files touched: `scripts/install_launcher_shortcut.ps1`
- Expected outcome: Existing stale PowerShell arguments are removed.

### Step 2 — Validate saved shortcut fields
- [x] Action: Re-open each saved shortcut and verify target, arguments, working directory, and
  icon location.
- Files touched: `scripts/install_launcher_shortcut.ps1`
- Expected outcome: Required shortcut targets fail loudly if stale fields survive.

### Step 3 — Reinstall and prove visible Desktop shortcut
- [x] Action: Run installer, inspect `C:\Users\m\Desktop\AURA.lnk`, launch through it, and
  close the app.
- Files touched: local shortcut files outside repo
- Expected outcome: Desktop shortcut arguments are empty and launch starts `aura_launcher.exe`.

### Step 4 — Commit and push
- [x] Action: Run checks, stage exact files, commit, push `main`.
- Files touched: git metadata only
- Expected outcome: clean tree and `HEAD == origin/main`.

## Test gate
- `pwsh -File scripts\install_launcher_shortcut.ps1`
- shortcut inspection for `C:\Users\m\Desktop\AURA.lnk`
- bounded shortcut launch smoke from `C:\Users\m\Desktop\AURA.lnk`
- `git diff --check`
- `git status --short --branch`

## Rollback
Do not delete. If validation is too strict, add a superseding commit adjusting installer
validation while preserving explicit argument clearing.

## Next-agent pickup
If interrupted:
1. Read this plan.
2. Run `git status --short --branch`.
3. Continue from the first unchecked step.
