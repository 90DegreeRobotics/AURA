# Plan: document framer RAG foundation - 2026-07-22 20:36

## Status
COMPLETED

## Goal
Build the first real AURA document-framing foundation for the future RAG pipeline. Every document that enters the AURA database must pass through a NeuroCognica frame derived from the existing `C:\AURA-Lab\Doc_Framer\nc-framer.py` title-block metadata, then receive deterministic hashes and chunks before it can be stored. This slice must expose document database state in the Bevy launcher and must not claim full RAG, embeddings, or mass corpus import yet.

## Context
- Relevant crates / modules:
  - `crates/aura_documents` (new)
  - `crates/aura_launcher`
  - workspace `Cargo.toml`
  - `README.md`
  - `docs/plans/AURA_MASTER_PLAN.md`
  - `docs/security/SENTINEL_PROTECTED_ACTIONS.md`
- Files that will be read:
  - `C:\AURA-Lab\Doc_Framer\nc-framer.py`
  - `crates/aura_launcher/src/main.rs`
  - `crates/aura_launcher/src/model.rs`
  - `crates/aura_runtime/src/request.rs`
- Files that will be edited:
  - `Cargo.toml`
  - `Cargo.lock`
  - `crates/aura_documents/**`
  - `crates/aura_launcher/Cargo.toml`
  - `crates/aura_launcher/src/main.rs`
  - `crates/aura_launcher/src/model.rs`
  - `README.md`
  - `docs/plans/AURA_MASTER_PLAN.md`
  - `docs/security/SENTINEL_PROTECTED_ACTIONS.md`
  - this plan document
- Preconditions / dependencies:
  - Work happens on `main`.
  - Sentinel remains first authority. This slice may create read-only launcher status and test-only store writes, but no broad operator corpus import will be presented as product-complete until the launcher workflow is Sentinel-authorized.

## Steps

### Step 1 - Add framed document engine
- [x] Action: Create `aura_documents` with typed NeuroCognica document metadata, canonical frame records, deterministic text normalization, BLAKE3 hashes, and stable chunking.
- Files touched: `Cargo.toml`, `crates/aura_documents/**`
- Expected outcome: A reusable Rust library can frame supported text documents without database side effects.

### Step 2 - Add local append-only document store
- [x] Action: Add an idempotent local store under the AURA data directory with frame and chunk JSONL files, summary counts, duplicate-source detection, and tests.
- Files touched: `crates/aura_documents/**`, `Cargo.lock`
- Expected outcome: Documents can be stored only after framing, and test fixtures prove deterministic output.

### Step 3 - Surface document DB truth in launcher
- [x] Action: Add document database path, frame count, chunk count, and framer status to `LauncherSnapshot`, then render those lines in the Bevy UI.
- Files touched: `crates/aura_launcher/Cargo.toml`, `crates/aura_launcher/src/main.rs`, `crates/aura_launcher/src/model.rs`
- Expected outcome: The compiled launcher reflects the new document/RAG foundation on screen.

### Step 4 - Update docs honestly
- [x] Action: Update README, master plan, protected-action census, and this plan with exact live/planned language.
- Files touched: `README.md`, `docs/plans/AURA_MASTER_PLAN.md`, `docs/security/SENTINEL_PROTECTED_ACTIONS.md`, this plan
- Expected outcome: Docs say document framing/store foundation is live, while embeddings, retrieval, and Sentinel-authorized corpus ingestion remain planned.

### Step 5 - Verify, build executable, commit, push
- [x] Action: Run formatting, workspace tests, launcher build, shortcut installer, and status checks; then stage explicit pathspecs, commit, push `origin/main`.
- Files touched: `Cargo.lock`, `dist/*` only as generated ignored output
- Expected outcome: Workspace gates pass, `dist\aura_launcher.exe` is refreshed, tree is clean, and origin/main contains the work.

## Test gate
Commands to run:

```powershell
cargo fmt -p aura_runtime -p aura_documents -p aura_cli -p aura_launcher -- --check
cargo test --workspace
cargo build -p aura_launcher --release
pwsh -File scripts\install_launcher_shortcut.ps1
git status --short --branch
```

Note: `cargo fmt --all` currently attempts to format the path dependency at `C:\sentinel-core`
and fails on pre-existing trailing whitespace in `sentinel_api`. This AURA unit used the
package-scoped formatter gate above to avoid mutating Sentinel Core.

## Rollback
If this goes wrong, revert only this unit's added/edited files through a normal follow-up commit. Do not reset, force-push, delete history, or delete unrelated files.

## Next-agent pickup
If Status is INTERRUPTED, the next agent should:
1. Read this document top-to-bottom.
2. Run `git status` and `git diff --stat`.
3. Check each Step -- find the first unchecked box, pick up there.
4. Update Status to IN-PROGRESS before resuming.
