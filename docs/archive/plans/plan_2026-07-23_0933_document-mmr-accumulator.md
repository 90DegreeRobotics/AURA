# Plan: document MMR accumulator - 2026-07-23 09:33

## Status
COMPLETED

## Goal
Close the next honest AURA document-store gap by adding a real RocksDB-backed Merkle Mountain Range style accumulator for document ingest Forever records. The accumulator must be append-only, derived from verified Forever record hashes, persisted in the same synced RocksDB batch as each ingest, replay-verifiable after reopen, visible in the Bevy launcher status, and documented honestly.

## Context
- Relevant crates / modules:
  - `crates/aura_documents`
  - `crates/aura_launcher`
- Files that will be read:
  - `AGENTS.md`
  - `README.md`
  - `CLAUDE.md`
  - `docs/plans/AURA_MASTER_PLAN.md`
  - `docs/security/SENTINEL_ADOPTION_STATUS.md`
  - `crates/aura_documents/src/lib.rs`
  - `crates/aura_launcher/src/model.rs`
- Files that will be edited:
  - `crates/aura_documents/src/lib.rs`
  - `crates/aura_launcher/src/model.rs`
  - `README.md`
  - `docs/plans/AURA_MASTER_PLAN.md`
  - `docs/security/SENTINEL_ADOPTION_STATUS.md`
  - this plan document
- Preconditions / dependencies:
  - Work stays on `main`.
  - The current RocksDB document store is real and clean on `origin/main`.
  - No sibling project path dependencies may be added.
  - No MMR/Merkle claim may be made until tests verify the accumulator.

## Steps

### Step 1 - Add RocksDB MMR schema and record fields
- [x] Action: Add document MMR schema constants, meta keys, peak/leaf keys, summary fields, and Forever record fields that bind each event to a persisted accumulator position and root.
- Files touched: `crates/aura_documents/src/lib.rs`
- Expected outcome: Stored Forever records can carry MMR leaf index, leaf hash, leaf count, and root hash without replacing the existing hash chain.

### Step 2 - Append MMR leaves in the ingest batch
- [x] Action: Derive each MMR leaf from the canonical Forever record hash, update RocksDB peak nodes, store leaf/event indexes, and persist leaf count/root in the same synced batch as the document ingest.
- Files touched: `crates/aura_documents/src/lib.rs`
- Expected outcome: A document ingest either stores both document records and MMR evidence together, or stores neither.

### Step 3 - Add replay verification tests
- [x] Action: Add verifier logic that replays all Forever records into a fresh accumulator and compares record fields, RocksDB leaf keys, peak keys, and summary root/count. Add tests for multiple ingests and reopen verification.
- Files touched: `crates/aura_documents/src/lib.rs`
- Expected outcome: Tests prove the accumulator is not a decorative status field.

### Step 4 - Surface MMR truth in the launcher
- [x] Action: Include MMR leaf count/root in the document DB status line and update launcher model tests.
- Files touched: `crates/aura_launcher/src/model.rs`
- Expected outcome: The compiled launcher surface reflects the real accumulator state.

### Step 5 - Update docs and verification
- [x] Action: Change docs from "MMR/Merkle not live" to the precise live state, run formatting/tests/build, refresh the launcher shortcut, commit, and push.
- Files touched: `README.md`, `docs/plans/AURA_MASTER_PLAN.md`, `docs/security/SENTINEL_ADOPTION_STATUS.md`, this plan document
- Expected outcome: `origin/main` carries a clean, verified MMR accumulator unit and the launcher executable is refreshed.

## Test gate
Commands to run to verify success:

```pwsh
cargo fmt --all --check
cargo test -p aura_documents
cargo test --workspace
cargo build -p aura_launcher --release
pwsh -File scripts\install_launcher_shortcut.ps1
git diff --check
```

## Rollback
If the accumulator breaks existing document storage, create a corrective commit that preserves existing RocksDB frame/chunk/print/Forever records and temporarily reports MMR verification as unavailable. Do not delete databases, user documents, assets, or history.

## Next-agent pickup
If Status is INTERRUPTED, the next agent should:
1. Read this document top-to-bottom.
2. Run `git status` and `git diff --stat`.
3. Check each Step and find the first unchecked box.
4. Update Status to IN-PROGRESS before resuming.
