# Plan: document intake UI — 2026-07-23 07:52

## Status
COMPLETED

## Goal
Add the first real AURA document intake workbench to the compiled Bevy launcher. The workbench will let the operator select document source paths, see supported candidates and current AURA document database counts, and attempt frame/ingest through Sentinel-mediated runtime actions. Because AURA is still default deny-all, protected document reads and database appends must visibly refuse before side effects until a future policy/consent lane authorizes them.

## Context
- Relevant crates / modules:
  - `crates/aura_runtime`
  - `crates/aura_documents`
  - `crates/aura_launcher`
  - `scripts/install_launcher_shortcut.ps1`
- Files that will be read:
  - `AGENTS.md`
  - `CLAUDE.md`
  - `README.md`
  - `docs/plans/AURA_MASTER_PLAN.md`
  - `docs/security/SENTINEL_ADOPTION_STATUS.md`
  - `docs/security/SENTINEL_PROTECTED_ACTIONS.md`
  - `crates/aura_runtime/src/*`
  - `crates/aura_runtime/tests/fail_closed.rs`
  - `crates/aura_documents/src/lib.rs`
  - `crates/aura_launcher/src/main.rs`
- Files that will be edited:
  - `Cargo.toml` / launcher manifest if a native file picker dependency is required
  - `crates/aura_runtime/src/request.rs`
  - `crates/aura_runtime/tests/fail_closed.rs`
  - `crates/aura_launcher/Cargo.toml`
  - `crates/aura_launcher/src/main.rs`
  - `README.md`
  - `docs/plans/AURA_MASTER_PLAN.md`
  - `docs/security/SENTINEL_ADOPTION_STATUS.md`
  - `docs/security/SENTINEL_PROTECTED_ACTIONS.md`
  - this plan document
- Preconditions / dependencies:
  - Work stays on `main`.
  - No sibling project path dependencies.
  - No CLI-only completion.
  - No document source read or DB append before Sentinel authorization.
  - Founder owns visual screenshot acceptance.

## Steps

### Step 1 — Add document action vocabulary
- [x] Action: Add runtime action variants for `aura.document.frame` and `aura.document.ingest`, mapped to Core-compatible `file.read_sensitive` and `memory.write` resources.
- Files touched: `crates/aura_runtime/src/request.rs`
- Expected outcome: Document frame/ingest can enter the broker with explicit protected action IDs and document resources.

### Step 2 — Prove deny before side effect
- [x] Action: Add fail-closed tests that verify deny-all refuses document frame/read and ingest/write before closures execute.
- Files touched: `crates/aura_runtime/tests/fail_closed.rs`
- Expected outcome: Protected document work cannot read source text or append the AURA DB under deny-all.

### Step 3 — Add native document selection state
- [x] Action: Add a launcher document workbench state model and native file/folder picker dependency, storing only selected paths and supported-source metadata before authorization.
- Files touched: `crates/aura_launcher/Cargo.toml`, `crates/aura_launcher/src/main.rs`
- Expected outcome: The operator can select a file/folder from the launcher without reading document contents.

### Step 4 — Wire frame/ingest buttons to Sentinel
- [x] Action: Add launcher controls for Add File, Add Folder, Frame Selected, Ingest Selected, Open DB Folder, and Clear Selection. Frame/ingest attempts must go through the runtime broker and show denial or success in the UI.
- Files touched: `crates/aura_launcher/src/main.rs`
- Expected outcome: The launcher surface reflects the document intake workflow and reports current Sentinel-blocked status honestly.

### Step 5 — Update docs and status truth
- [x] Action: Update README, master plan, adoption status, and protected-action inventory so the repo says exactly what is live: selection UI and brokered denial are live; authorized import/embeddings/retrieval are not.
- Files touched: `README.md`, `docs/plans/AURA_MASTER_PLAN.md`, `docs/security/SENTINEL_ADOPTION_STATUS.md`, `docs/security/SENTINEL_PROTECTED_ACTIONS.md`
- Expected outcome: Docs match product behavior and do not overclaim RAG.

### Step 6 — Build, refresh launcher, commit, push
- [x] Action: Run formatting/tests/build, refresh the desktop launcher, do a non-visual exe launch sanity check, archive this completed plan, stage intentionally, commit, push, and verify `main`/`origin/main` parity.
- Files touched: build outputs under ignored `target/` and `dist/`, git metadata, archived plan
- Expected outcome: `origin/main` carries a clean, compiled document intake UI unit.

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
# C:\Users\m\Desktop\AURA.lnk still targets C:\aura\dist\aura_launcher.exe
# C:\aura\dist\aura_launcher.exe starts and remains alive long enough for the launcher surface
```

## Rollback
If the launcher build breaks, create a follow-up commit that restores the previous launcher controls while preserving the runtime deny tests. If the runtime action vocabulary is wrong, revert via a new corrective commit and keep the protected-action inventory honest. Do not delete user documents or asset folders.

## Next-agent pickup
If Status is INTERRUPTED, the next agent should:
1. Read this document top-to-bottom.
2. Run `git status` and `git diff --stat`.
3. Check each Step and find the first unchecked box.
4. Update Status to IN-PROGRESS before resuming.
