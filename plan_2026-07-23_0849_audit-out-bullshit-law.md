# Plan: Audit Out Bullshit Law — 2026-07-23 08:49

## Status
COMPLETED

## Goal
Promote the Founder's anti-theatre directive into AURA repo law and orientation so this build cannot drift into fake front ends, fake back ends, disconnected controls, stubs, or completion theatre.

## Context
- Relevant files:
  - `AGENTS.md`
  - `CLAUDE.md`
  - `docs/plans/AURA_MASTER_PLAN.md`
- Files that will be edited:
  - `AGENTS.md`
  - `CLAUDE.md`
  - `docs/plans/AURA_MASTER_PLAN.md`
  - this plan document
- Preconditions / dependencies:
  - Charter loaded.
  - Work stays on `main`.
  - This is documentation/repo-law work; no runtime code or launcher rebuild is required unless content changes affect product behavior.

## Steps

### Step 1 — Add absolute anti-theatre law
- [x] Action: Add a named AURA law section forbidding stubs, fake front ends, fake back ends, disconnected UI, mock-only backends, and false progress.
- Files touched: `AGENTS.md`
- Expected outcome: The repo law makes the directive unavoidable.

### Step 2 — Update agent orientation
- [x] Action: Add a short key-rule entry so future sessions inherit the anti-theatre posture quickly.
- Files touched: `CLAUDE.md`
- Expected outcome: New agents see the directive during normal orientation.

### Step 3 — Update master plan
- [x] Action: Add the directive to failure archaeology / product proof language.
- Files touched: `docs/plans/AURA_MASTER_PLAN.md`
- Expected outcome: The binding plan treats anti-theatre as a build invariant.

### Step 4 — Verify, commit, push
- [x] Action: Run docs-level diff/status verification, commit, push, and verify clean parity.
- Files touched: git metadata
- Expected outcome: Repo law is on `origin/main`.

## Test gate
Commands to run:

```powershell
git diff --check
git status --short --branch
```

## Rollback
If wording is wrong, add a corrective commit. Do not delete or rewrite history.

## Next-agent pickup
If Status is INTERRUPTED, the next agent should:
1. Read this document.
2. Run `git status` and `git diff --stat`.
3. Finish unchecked steps.
