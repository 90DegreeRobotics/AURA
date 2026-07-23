# Plan: self-contained Sentinel runtime - 2026-07-23 06:18

## Status
COMPLETED

## Goal
Remove AURA's compile-time path dependency on `C:\sentinel-core` so the product builds as a self-contained Windows executable from the AURA repository. Preserve the current fail-closed Sentinel semantics by internalizing only the minimal deterministic guard request, decision, policy, protected-action registry, and tests needed by the L0 runtime. Update docs that currently describe a sibling path dependency.

## Context
- Relevant crates / modules:
  - `crates/aura_runtime`
  - `crates/aura_launcher`
  - workspace manifests
  - Sentinel adoption docs
- Files that will be read:
  - `C:\sentinel-core\crates\sentinel_core\src\lib.rs`
  - `crates/aura_runtime/src/client.rs`
  - `crates/aura_runtime/src/request.rs`
  - `crates/aura_runtime/src/decision_log.rs`
  - `crates/aura_runtime/tests/fail_closed.rs`
- Files that will be edited:
  - `Cargo.toml`
  - `Cargo.lock`
  - `crates/aura_runtime/Cargo.toml`
  - `crates/aura_runtime/src/client.rs`
  - `crates/aura_runtime/src/decision_log.rs`
  - `crates/aura_runtime/src/lib.rs`
  - `crates/aura_runtime/src/request.rs`
  - `crates/aura_runtime/src/sentinel.rs` (new)
  - `crates/aura_runtime/tests/fail_closed.rs`
  - `README.md`
  - `AGENTS.md`
  - `CLAUDE.md`
  - `docs/plans/AURA_MASTER_PLAN.md`
  - `docs/security/SENTINEL_ADOPTION_STATUS.md`
  - `docs/security/SENTINEL_PROTECTED_ACTIONS.md`
  - this plan document
- Preconditions / dependencies:
  - Work happens on `main`.
  - This change must not weaken Carved Law. Removing the sibling crate dependency does not mean removing Sentinel; it means AURA owns its local L0 guard implementation until a packaged SDK/runtime boundary exists.

## Steps

### Step 1 - Internalize the minimal guard
- [x] Action: Add `aura_runtime::sentinel` with local request, decision, policy, rule, protected-action registry, deterministic guard, validation, and trace ID logic derived from the current Sentinel Core shape.
- Files touched: `crates/aura_runtime/src/sentinel.rs`, `crates/aura_runtime/src/lib.rs`
- Expected outcome: AURA has its own L0 guard types and no code import from `sentinel_core`.

### Step 2 - Remove path dependency imports
- [x] Action: Replace `sentinel_core` imports in runtime code and tests with local `aura_runtime` exports. Remove the workspace and crate path dependency.
- Files touched: `Cargo.toml`, `Cargo.lock`, `crates/aura_runtime/Cargo.toml`, runtime source/tests
- Expected outcome: `rg sentinel_core` returns no Rust manifest/source dependency in AURA.

### Step 3 - Update docs to self-contained truth
- [x] Action: Replace path-dependency language with self-contained L0 guard language, while keeping future Sentinel certification/SDK language honest.
- Files touched: `README.md`, `AGENTS.md`, `CLAUDE.md`, `docs/plans/AURA_MASTER_PLAN.md`, `docs/security/SENTINEL_ADOPTION_STATUS.md`, `docs/security/SENTINEL_PROTECTED_ACTIONS.md`, this plan
- Expected outcome: Docs no longer tell agents AURA requires a sibling checkout to build.

### Step 4 - Verify exe and push
- [x] Action: Run package-scoped formatting, workspace tests, release launcher build, shortcut installer, path-dependency grep, clean status, explicit staging, commit, and push.
- Files touched: `dist/*` only as generated ignored output
- Expected outcome: AURA builds from its own repo, launcher is refreshed, tree is clean, and origin/main receives the fix.

## Test gate
Commands to run:

```powershell
cargo fmt -p aura_runtime -p aura_documents -p aura_cli -p aura_launcher -- --check
cargo test --workspace
cargo build -p aura_launcher --release
pwsh -File scripts\install_launcher_shortcut.ps1
rg -n "sentinel_core|\\.\\./sentinel-core|path dependency on|path dep `sentinel_core`" Cargo.toml crates README.md AGENTS.md CLAUDE.md docs\security\SENTINEL_ADOPTION_STATUS.md docs\security\SENTINEL_PROTECTED_ACTIONS.md docs\plans\AURA_MASTER_PLAN.md
git status --short --branch
```

## Rollback
If this goes wrong, revert only this unit through a normal follow-up commit. Do not reset, force-push, or delete unrelated files.

## Next-agent pickup
If Status is INTERRUPTED, the next agent should:
1. Read this document top-to-bottom.
2. Run `git status` and `git diff --stat`.
3. Check each Step -- find the first unchecked box, pick up there.
4. Update Status to IN-PROGRESS before resuming.
