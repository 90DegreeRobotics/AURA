# Plan: First AURA Launcher — 2026-07-22 19:40

## Status
COMPLETED

## Goal
Create the first real AURA Windows application surface: a compiled Bevy launcher that displays
version/build truth, Sentinel/boot status, the decision ledger path, and a button-driven
boot-continue attempt that goes through `aura_runtime` and visibly refuses under deny-all.
This is the first product spine, not the whole system.

## Context
- Relevant crates / modules:
  - `crates/aura_runtime` — existing Sentinel-first boot supervisor and action broker
  - `crates/aura_cli` — developer harness only
  - new `crates/aura_launcher` — Bevy Windows launcher / product surface
- Files read:
  - `C:\corpus\THE_CHARTER_OF_COGNITIVE_SOVEREIGNTY.md`
  - `AGENTS.md`
  - `CLAUDE.md`
  - `README.md`
  - `docs/plans/AURA_MASTER_PLAN.md`
  - `docs/security/SENTINEL_ADOPTION_STATUS.md`
  - `docs/security/SENTINEL_PROTECTED_ACTIONS.md`
  - `crates/aura_runtime/src/*.rs`
  - `crates/aura_runtime/tests/fail_closed.rs`
  - selected Archetypes Bevy/readiness launcher files
- Files to edit:
  - `.gitignore`
  - `Cargo.lock`
  - `Cargo.toml`
  - `crates/aura_runtime/src/lib.rs`
  - `crates/aura_launcher/Cargo.toml`
  - `crates/aura_launcher/src/main.rs`
  - `crates/aura_launcher/src/model.rs`
  - `scripts/install_launcher_shortcut.ps1`
  - `README.md`
  - `CLAUDE.md`
  - `docs/security/SENTINEL_ADOPTION_STATUS.md`
  - this plan document
- Preconditions / dependencies:
  - `origin/main` is configured and clean at start.
  - The launcher must not execute protected work outside Sentinel.
  - Visual acceptance belongs to the Founder; this session will compile/run and report, not
    spend tokens on decorative screenshot inspection.

## Steps

### Step 1 — Add launcher crate and workspace wiring
- [x] Action: Add `crates/aura_launcher` to the workspace with Bevy 0.18.1 and an
  `aura_runtime` dependency.
- Files touched: `Cargo.toml`, `crates/aura_launcher/Cargo.toml`
- Expected outcome: `cargo build -p aura_launcher` targets a real Windows `.exe`.

### Step 2 — Expose runtime status type
- [x] Action: Re-export `BootStatus` from `aura_runtime` for the launcher view model.
- Files touched: `crates/aura_runtime/src/lib.rs`
- Expected outcome: The launcher can consume the existing boot supervisor without duplicating
  status logic.

### Step 3 — Build the launcher view model
- [x] Action: Add a small tested model layer for launcher snapshot text, data-dir selection,
  and boot-continue result rendering.
- Files touched: `crates/aura_launcher/src/model.rs`
- Expected outcome: Non-visual launcher behavior has unit coverage independent of Bevy.

### Step 4 — Build the Bevy product surface
- [x] Action: Add a Bevy app with title, version/build identity, Sentinel status cards,
  decision ledger path, service placeholders honestly marked planned, and buttons for refresh,
  attempt boot continue, and quit.
- Files touched: `crates/aura_launcher/src/main.rs`
- Expected outcome: The operator can launch `aura_launcher.exe` and see real AURA runtime
  state through the app surface.

### Step 5 — Update docs/status
- [x] Action: Update README/orientation/adoption status to mark the launcher as first live
  product surface, document the shortcut install path, and keep certification plus broader
  services incomplete.
- Files touched: `README.md`, `CLAUDE.md`, `docs/security/SENTINEL_ADOPTION_STATUS.md`
- Expected outcome: Docs match code and do not overclaim chat/image/TTS/STT readiness.

### Step 6 — Add desktop shortcut installer
- [x] Action: Add a PowerShell script that builds release, copies the launcher into ignored
  `dist\`, and creates Desktop/Start Menu shortcuts named AURA. Track `Cargo.lock` now that
  AURA has an application binary.
- Files touched: `.gitignore`, `Cargo.lock`, `scripts/install_launcher_shortcut.ps1`
- Expected outcome: The Founder has a real desktop launch path without committing generated
  binaries.

### Step 7 — Verify, commit, push
- [x] Action: Run formatting, tests, build, shortcut install, and a bounded launcher smoke run. Commit by
  explicit pathspec and push `main`.
- Files touched: git metadata only
- Expected outcome: Working tree clean, `HEAD == origin/main`, and a compiled launcher binary
  exists under `target` plus `dist`.

## Test gate
- `cargo fmt --all --check`
- `cargo test --workspace`
- `cargo build -p aura_launcher`
- `pwsh -File scripts\install_launcher_shortcut.ps1`
- bounded launcher smoke run if the app starts without blocking the session
- `git diff --check`
- `git status --short --branch`

## Rollback
Do not delete. If this path is wrong, add a superseding plan/commit that marks
`crates/aura_launcher` blocked or replaces it while preserving history.

## Next-agent pickup
If interrupted:
1. Read this document.
2. Run `git status --short --branch`.
3. Check each step for completion.
4. Continue at the first unchecked step, preserving Carved Law and launcher-first law.
