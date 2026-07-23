# Plan: brand assets launcher icon — 2026-07-23 07:00

## Status
COMPLETED

## Goal
Promote the new `NC LOGOS` and `aura` folders from loose untracked assets into real AURA product identity assets. The desktop/start-menu/taskbar icon must use the NeuroCognica logo prominently, and the launcher first screen must visibly show the NeuroCognica logo, the word AURA, and the acronym expansion `Archetypes - Utilizing - Reflective - Architecture`.

## Context
- Relevant crates / modules:
  - `crates/aura_launcher`
  - `scripts/install_launcher_shortcut.ps1`
  - `assets/icon`
- Files/folders that will be read:
  - `NC LOGOS/neurocognica_logos/*`
  - `aura/*`
  - `crates/aura_launcher/src/main.rs`
  - `scripts/install_launcher_shortcut.ps1`
  - `crates/aura_launcher/build.rs`
  - `README.md`
  - `docs/plans/AURA_MASTER_PLAN.md`
- Files/folders that will be edited or added:
  - `.gitignore`
  - `assets/icon/aura.ico`
  - `assets/brand/*`
  - `crates/aura_launcher/src/main.rs`
  - `scripts/install_launcher_shortcut.ps1`
  - `README.md`
  - `docs/plans/AURA_MASTER_PLAN.md`
  - tracked asset files under `NC LOGOS/` and `aura/`
  - this plan document
- Preconditions / dependencies:
  - Work stays on `main`.
  - No sibling project path dependencies.
  - No protected action is added; this is product identity and local asset wiring.
  - Windows metadata files such as `desktop.ini` and `Thumbs.db` are not product assets and should be ignored, not deleted.
  - Founder owns final visual acceptance; do not spend tokens on decorative screenshot witnessing.

## Steps

### Step 1 — Select source assets
- [x] Action: Use the NeuroCognica logo folder as the source for brand mark and icon generation, and inspect the Aura folder for product imagery.
- Files touched: none
- Expected outcome: Choose stable source paths for generated app identity assets.

### Step 2 — Generate tracked brand assets
- [x] Action: Generate a compact `.ico` for Windows shell/taskbar use and larger PNG lockups for the launcher surface using the NeuroCognica logo plus AURA text.
- Files touched: `assets/icon/aura.ico`, `assets/brand/*`
- Expected outcome: The icon and launcher image assets carry the logo, AURA, and acronym text at the largest useful sizes.

### Step 3 — Wire brand assets into Bevy launcher
- [x] Action: Load the tracked brand lockup/mark through Bevy UI and place it prominently in the first screen header alongside existing runtime truth.
- Files touched: `crates/aura_launcher/src/main.rs`
- Expected outcome: The launcher first screen visibly shows NeuroCognica branding, `AURA`, and `Archetypes - Utilizing - Reflective - Architecture`.

### Step 4 — Update install/copy path
- [x] Action: Ensure the shortcut installer copies `assets/brand` into `dist/assets/brand` so the release launcher can load images from its installed working directory.
- Files touched: `scripts/install_launcher_shortcut.ps1`
- Expected outcome: Desktop/start-menu launch path has the `.exe`, `.ico`, and brand image assets together.

### Step 5 — Add intentional assets and ignore junk metadata
- [x] Action: Track the new asset folders' real product files, ignore Windows metadata, and avoid deleting any user-provided file.
- Files touched: `.gitignore`, git index
- Expected outcome: `NC LOGOS/` and `aura/` assets are preserved in repo history without committing `desktop.ini` or `Thumbs.db`.

### Step 6 — Update docs
- [x] Action: Document that the launcher identity surface now uses the new NeuroCognica/AURA assets and branded shortcut icon.
- Files touched: `README.md`, `docs/plans/AURA_MASTER_PLAN.md`
- Expected outcome: Repo truth surfaces match the launcher and desktop icon behavior.

### Step 7 — Build, refresh shortcut, commit, push
- [x] Action: Run formatting/tests/build, refresh shortcuts, verify shortcut target/icon metadata, stage intentionally, commit, push, and verify parity.
- Files touched: build outputs under ignored `dist/` and `target/`, git metadata
- Expected outcome: `origin/main` carries the branded launcher/icon work and local tree is clean except intentionally ignored metadata.

## Test gate
Commands to run:

```pwsh
cargo fmt --all --check
cargo test --workspace
cargo build -p aura_launcher --release
pwsh -File scripts\install_launcher_shortcut.ps1
```

Also verify:

```pwsh
# Desktop shortcut points at C:\aura\dist\aura_launcher.exe and C:\aura\dist\aura.ico
# C:\aura\dist\assets\brand exists after install helper
```

## Rollback
If this breaks the build or launcher startup, create a follow-up commit that restores the previous launcher header and icon path behavior. Preserve the user-provided asset folders; do not delete them.

## Next-agent pickup
If Status is INTERRUPTED, the next agent should:
1. Read this document top-to-bottom.
2. Run `git status` and `git diff --stat`.
3. Check each Step and find the first unchecked box.
4. Update Status to IN-PROGRESS before resuming.
