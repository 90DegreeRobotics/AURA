# Plan: RocksDB Forever Document Store — 2026-07-23 08:53

## Status
COMPLETED

## Goal
Replace the current JSONL document store with a real RocksDB-backed document store so documents that pass through the official NeuroCognica frame pipeline are stored through an append-only, hash-chained Forever Law substrate. Do not claim MMR/Merkle yet unless it is implemented and tested.

## Context
- Relevant crates / modules:
  - `crates/aura_documents`
  - `crates/aura_runtime`
  - `crates/aura_launcher`
- Files that will be read:
  - `crates/aura_documents/src/lib.rs`
  - `crates/aura_launcher/src/main.rs`
  - `crates/aura_launcher/src/model.rs`
  - `crates/aura_runtime/src/decision_log.rs`
  - `README.md`
  - `docs/plans/AURA_MASTER_PLAN.md`
  - `docs/security/SENTINEL_ADOPTION_STATUS.md`
  - `docs/security/SENTINEL_PROTECTED_ACTIONS.md`
- Files that will be edited:
  - `Cargo.toml`
  - `crates/aura_documents/Cargo.toml`
  - `crates/aura_documents/src/lib.rs`
  - `crates/aura_runtime/Cargo.toml`
  - `crates/aura_runtime/src/decision_log.rs`
  - `crates/aura_launcher/src/main.rs`
  - `crates/aura_launcher/src/model.rs`
  - `README.md`
  - `docs/plans/AURA_MASTER_PLAN.md`
  - `docs/security/SENTINEL_ADOPTION_STATUS.md`
  - this plan document
- Preconditions / dependencies:
  - Work stays on `main`.
  - No fake backend. The `rocksdb` crate must compile and tests must write/open an actual RocksDB database.
  - No side-effect bypass: launcher document ingest remains Sentinel-mediated and denied under the default deny-all policy.

## Steps

### Step 1 — Add real RocksDB dependency
- [x] Action: Add the `rocksdb` crate as a direct dependency for `aura_documents`.
- Files touched: `Cargo.toml`, `crates/aura_documents/Cargo.toml`, `Cargo.lock`
- Expected outcome: The document crate links to a real RocksDB backend, not a placeholder trait or file cache.

### Step 2 — Replace JSONL store with RocksDB store
- [x] Action: Rework `DocumentStore` so `open()` creates/opens RocksDB under the document root, summary counts read from RocksDB, and document ingest writes frame, chunks, print artifact, source text, and indexes into RocksDB.
- Files touched: `crates/aura_documents/src/lib.rs`
- Expected outcome: `DocumentStore` is backed by `documents.rocksdb`; JSONL paths are removed from the live write path.

### Step 3 — Add Forever Law append-only records
- [x] Action: During ingest, append a `DocumentForeverRecord` with sequence, previous hash, payload hash, record hash, event kind, frame ID, chunk count, and print ID. The store must fail if it cannot commit the RocksDB batch.
- Files touched: `crates/aura_documents/src/lib.rs`
- Expected outcome: Every stored document has an immutable, hash-chained ingest event in RocksDB.

### Step 4 — Correct runtime decision hashing
- [x] Action: Replace the decision log's fake `sha256:` string composition with a real BLAKE3 record hash derived from the serialized decision evidence.
- Files touched: `crates/aura_runtime/Cargo.toml`, `crates/aura_runtime/src/decision_log.rs`
- Expected outcome: Runtime decision evidence no longer labels non-cryptographic concatenation as a hash.

### Step 5 — Prove real persistence and idempotence
- [x] Action: Update and add tests that open an actual RocksDB directory, ingest a framed document, close/reopen the store, retrieve counts/records, verify the hash chain, and prove repeat ingest does not duplicate records.
- Files touched: `crates/aura_documents/src/lib.rs`
- Expected outcome: Tests prove real RocksDB persistence and append-only event evidence.

### Step 6 — Reflect RocksDB truth in launcher
- [x] Action: Update launcher status/result text to say RocksDB + Forever chain, not JSONL.
- Files touched: `crates/aura_launcher/src/main.rs`, `crates/aura_launcher/src/model.rs`
- Expected outcome: The compiled launcher surface reports the real storage substrate.

### Step 7 — Update docs
- [x] Action: Update README, master plan, and adoption status to remove JSONL-as-document-store claims and state the honest RocksDB/hash-chain status. Protected-action docs only change if side effects or mediation change.
- Files touched: `README.md`, `docs/plans/AURA_MASTER_PLAN.md`, `docs/security/SENTINEL_ADOPTION_STATUS.md`
- Expected outcome: Docs match reality: RocksDB live, BLAKE3 hash chain live, MMR not live yet.

### Step 8 — Verify, package, commit, push
- [x] Action: Run formatter, workspace tests, release launcher build, shortcut refresh, explicit staging, commit, push, and clean parity check.
- Files touched: build outputs under ignored `target/` and `dist/`; git metadata
- Expected outcome: Work is proven, launcher refreshed, pushed to `origin/main`, and tree clean.

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
If RocksDB cannot build or tests fail in a way that cannot be fixed in-session, mark this plan INTERRUPTED and leave no committed fake substrate. If committed work later needs correction, add a forward fix commit; do not delete database files or rewrite history.

## Next-agent pickup
If Status is INTERRUPTED, the next agent should:
1. Read this document.
2. Run `git status` and `git diff --stat`.
3. Find the first unchecked step.
4. Continue from there without claiming RocksDB or Forever Law until tests prove the actual database path.
