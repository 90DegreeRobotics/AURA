# Plan: launcher text render fix — 2026-07-23 06:45

## Status
COMPLETED

## Goal
Fix the AURA launcher so the first visible screen actually renders readable text instead of a black surface with faint UI panels. The previous fade work compiled, but the Founder screenshot proves the release launcher is not drawing text. This pass treats that as a real product bug and makes the launcher explicitly own its font and first-frame visibility.

## Context
- Relevant crates / modules:
  - `crates/aura_launcher`
- Files that will be read:
  - `crates/aura_launcher/src/main.rs`
  - `crates/aura_launcher/Cargo.toml`
  - `README.md`
  - `docs/plans/AURA_MASTER_PLAN.md`
- Files that will be edited:
  - `crates/aura_launcher/src/main.rs`
  - `crates/aura_launcher/Cargo.toml` if an embedded font helper is needed
  - `README.md`
  - `docs/plans/AURA_MASTER_PLAN.md`
  - this plan document
- Preconditions / dependencies:
  - Work stays on `main`.
  - No sibling project path dependencies.
  - The compiled Windows launcher must remain the operator surface.
  - No decorative screenshot/witness call unless needed to debug the technical rendering failure; Founder owns visual acceptance.

## Steps

### Step 1 — Confirm text-render failure class
- [x] Action: Inspect launcher text/font creation and Bevy font APIs.
- Files touched: none
- Expected outcome: Identify an explicit font path that works in release/installed context.

### Step 2 — Embed or bundle launcher font
- [x] Action: Make every launcher `TextFont` use an owned launcher font instead of default text font behavior.
- Files touched: `crates/aura_launcher/src/main.rs`, maybe `crates/aura_launcher/Cargo.toml`
- Expected outcome: Text is independent of asset working directory ambiguity and should render in `dist\aura_launcher.exe`.

### Step 3 — Strengthen first-frame visibility
- [x] Action: Make the title visible immediately at launch while still fading into full presence, and ensure status/buttons use explicit font handles.
- Files touched: `crates/aura_launcher/src/main.rs`
- Expected outcome: First screen shows readable AURA identity and visible status, not a black void.

### Step 4 — Update docs
- [x] Action: Record the text-render fix and font ownership in repo truth docs.
- Files touched: `README.md`, `docs/plans/AURA_MASTER_PLAN.md`
- Expected outcome: Docs explain that launcher text is explicitly bundled/owned.

### Step 5 — Build, refresh, and verify launcher path
- [x] Action: Run format, workspace tests, release build, and launcher shortcut install helper.
- Files touched: ignored build/dist output
- Expected outcome: `C:\aura\dist\aura_launcher.exe` is rebuilt and `C:\Users\m\Desktop\AURA.lnk` points to it.

### Step 6 — Commit and push
- [x] Action: Stage only the intended source/docs/plan files, commit, push to `origin/main`, and verify parity.
- Files touched: git metadata
- Expected outcome: `origin/main` carries the rendering fix. Any unrelated untracked asset dumps found during verification remain unstaged and explicitly reported.

## Test gate
Commands to run:

```pwsh
cargo fmt --all --check
cargo test --workspace
cargo build -p aura_launcher --release
pwsh -File scripts\install_launcher_shortcut.ps1
```

## Rollback
If this breaks build or runtime, create a follow-up commit restoring the prior launcher UI code and remove only the newly introduced font-loading path from code. Do not delete history or force-push.

## Next-agent pickup
If Status is INTERRUPTED, the next agent should:
1. Read this document top-to-bottom.
2. Run `git status` and `git diff --stat`.
3. Check each Step and find the first unchecked box.
4. Update Status to IN-PROGRESS before resuming.
