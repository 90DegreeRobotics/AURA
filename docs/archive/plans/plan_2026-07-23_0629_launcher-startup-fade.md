# Plan: launcher startup fade — 2026-07-23 06:29

## Status
COMPLETED

## Goal
Add an immediate, human-readable launch signal to the AURA desktop launcher so the app no longer presents as a black or lifeless screen during startup. The first executable surface should fade into the word AURA and show a small live indicator that makes the launcher feel awake before deeper services finish reporting.

## Context
- Relevant crates / modules:
  - `crates/aura_launcher`
- Files that will be read:
  - `crates/aura_launcher/src/main.rs`
  - `crates/aura_launcher/src/model.rs`
  - `README.md`
  - `docs/plans/AURA_MASTER_PLAN.md`
- Files that will be edited:
  - `crates/aura_launcher/src/main.rs`
  - `README.md`
  - `docs/plans/AURA_MASTER_PLAN.md`
  - this plan document
- Preconditions / dependencies:
  - Work stays on `main`.
  - No sibling project path dependencies are added.
  - The launcher remains a self-contained Windows executable surface.
  - Visual screenshot acceptance stays with the Founder; this pass will build and refresh the executable/shortcut but will not spend tokens on screenshot witnessing.

## Steps

### Step 1 — Inspect launcher surface
- [x] Action: Confirm the current Bevy UI structure and runtime snapshot wiring.
- Files touched: none
- Expected outcome: Identify the smallest safe place to add startup animation and live indicator state.

### Step 2 — Add startup fade and live indicator
- [x] Action: Add Bevy resources/components/systems for the AURA word fade and a subtle launcher-alive indicator.
- Files touched: `crates/aura_launcher/src/main.rs`
- Expected outcome: The launcher shows visible life immediately and fades into the word AURA without changing backend governance behavior.

### Step 3 — Add focused behavior tests
- [x] Action: Add pure timing tests for fade and pulse math.
- Files touched: `crates/aura_launcher/src/main.rs`
- Expected outcome: Animation timing has a small regression guard without needing a visual harness.

### Step 4 — Update docs
- [x] Action: Document that startup polish is now part of the launcher surface.
- Files touched: `README.md`, `docs/plans/AURA_MASTER_PLAN.md`
- Expected outcome: The repo truth surfaces match the executable behavior.

### Step 5 — Build executable and refresh launcher shortcut
- [x] Action: Run formatting/tests/build, copy the release executable into `dist`, and refresh the desktop launcher shortcut if the existing installer helper supports it.
- Files touched: build outputs under ignored target/dist paths, existing desktop shortcut target metadata
- Expected outcome: A working `AURA.exe` exists and the desktop shortcut still launches the current app surface.

### Step 6 — Commit and push
- [x] Action: Stage only the plan, launcher source, and docs; commit and push to `origin/main`.
- Files touched: git metadata
- Expected outcome: `origin/main` contains the completed startup fade work and the local tree returns clean.

## Test gate
Commands to run to verify success:

```pwsh
cargo fmt --all --check
cargo test --workspace
cargo build -p aura_launcher --release
```

If an existing launcher helper exists for shortcut refresh, run it after the release build.

## Rollback
If this goes wrong, create a follow-up commit that disables the new startup animation systems and restores the previous launcher title/header behavior. Do not delete history or force-push.

## Next-agent pickup
If Status is INTERRUPTED, the next agent should:
1. Read this document top-to-bottom.
2. Run `git status` and `git diff --stat`.
3. Check each Step and find the first unchecked box.
4. Update Status to IN-PROGRESS before resuming.
