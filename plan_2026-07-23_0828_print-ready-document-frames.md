# Plan: Print-Ready Document Frames — 2026-07-23 08:28

## Status
COMPLETED

## Goal
Make every AURA document ingestion produce a proper NeuroCognica print-ready artifact as part of the document database contract. The current Rust framer preserves metadata, hashes, and chunks, but it does not yet carry the visible branded/title-block treatment from the original `C:\AURA-Lab\Doc_Framer\nc-framer.py`. This change adds a database-stored branded HTML print frame with the tracked NeuroCognica logo embedded, then reflects that truth in the Bevy launcher and docs.

## Context
- Relevant crates / modules:
  - `crates/aura_documents`
  - `crates/aura_launcher`
- Files that will be read:
  - `C:\AURA-Lab\Doc_Framer\nc-framer.py`
  - `assets/brand/neurocognica_logo_transparent.png`
  - `crates/aura_documents/src/lib.rs`
  - `crates/aura_launcher/src/model.rs`
  - `crates/aura_launcher/src/main.rs`
  - `README.md`
  - `docs/plans/AURA_MASTER_PLAN.md`
  - `docs/security/SENTINEL_ADOPTION_STATUS.md`
  - `docs/security/SENTINEL_PROTECTED_ACTIONS.md`
- Files that will be edited:
  - `Cargo.toml`
  - `crates/aura_documents/Cargo.toml`
  - `crates/aura_documents/src/lib.rs`
  - `crates/aura_launcher/src/model.rs`
  - `crates/aura_launcher/src/main.rs`
  - `README.md`
  - `docs/plans/AURA_MASTER_PLAN.md`
  - `docs/security/SENTINEL_ADOPTION_STATUS.md`
  - `docs/security/SENTINEL_PROTECTED_ACTIONS.md`
  - this plan document
- Preconditions / dependencies:
  - Work stays on `main`.
  - No sibling project path dependencies.
  - Protected source read and memory write remain Sentinel-mediated and deny-first.

## Steps

### Step 1 — Add print artifact schema
- [x] Action: Add a `DocumentPrintRecord`, print schema/version constants, print ID/hash metadata, and a `document_prints.jsonl` store path.
- Files touched: `Cargo.toml`, `crates/aura_documents/Cargo.toml`, `crates/aura_documents/src/lib.rs`
- Expected outcome: Every `FramedDocument` carries a print-ready record alongside frame metadata and chunks.

### Step 2 — Generate branded print HTML
- [x] Action: Embed the tracked NeuroCognica logo as a data URI, render a print-ready HTML sheet with title block fields inherited from the original Python framer, and escape/format document text for print.
- Files touched: `crates/aura_documents/src/lib.rs`
- Expected outcome: Frame output includes `NEUROCOGNICA`, the real logo, `AURA`, `PROJECT`, `TITLE`, `DWG NO`, `ENGR`, `DATE`, `REV`, rights, hashes, and body content in a printable layout.

### Step 3 — Store print records atomically with ingest
- [x] Action: Append the print record during `ingest_framed_document`, include print counts in summaries, and preserve idempotent behavior for already-ingested frames.
- Files touched: `crates/aura_documents/src/lib.rs`
- Expected outcome: The local database has frames, chunks, and print artifacts together; repeat ingest does not duplicate records.

### Step 4 — Reflect print readiness in the launcher
- [x] Action: Update the Bevy launcher model and document action messages to show print-ready document counts and print IDs.
- Files touched: `crates/aura_launcher/src/model.rs`, `crates/aura_launcher/src/main.rs`
- Expected outcome: The user-facing launcher surface reports that the document DB is print-ready, not just chunk-ready.

### Step 5 — Update docs
- [x] Action: Update README, master plan, adoption status, and protected actions with the new print-ready database contract.
- Files touched: `README.md`, `docs/plans/AURA_MASTER_PLAN.md`, `docs/security/SENTINEL_ADOPTION_STATUS.md`, `docs/security/SENTINEL_PROTECTED_ACTIONS.md`
- Expected outcome: Docs no longer imply the framer is only metadata/chunk storage.

### Step 6 — Verify, package, commit, push
- [x] Action: Run formatter/test/build gates, refresh the Windows launcher shortcut/dist exe, stage explicit paths, commit, push to `origin/main`, and verify clean status.
- Files touched: build outputs under `dist/` if changed; git metadata
- Expected outcome: `cargo test --workspace`, release launcher build, and shortcut refresh complete; work is pushed and tree is clean.

## Test gate
Commands to run:

```powershell
cargo fmt --all --check
cargo test --workspace
cargo build -p aura_launcher --release
pwsh -File scripts\install_launcher_shortcut.ps1
git status --short --branch
```

## Rollback
If this goes wrong, make an additive follow-up commit that disables the new print record path while preserving existing frames/chunks, or restores the previous document schema behavior from git history. Do not delete data files or rewrite history.

## Next-agent pickup
If Status is INTERRUPTED, the next agent should:
1. Read this document top-to-bottom.
2. Run `git status` and `git diff --stat`.
3. Check each Step and find the first unchecked box.
4. Update Status to IN-PROGRESS before resuming.
