# Plan: Launcher-First Repo Law — 2026-07-22 19:29

## Status
COMPLETED

## Goal
Make the founder's launcher-first AURA directive binding repo law: Bevy is the
front end, AURA is a real Windows desktop app from the start, all user-visible
functionality lands at the compiled launcher surface, CLI is developer-only, and
every product session must leave a working `.exe`/launcher surface reflecting
the new work.

## Context
- Relevant modules / docs:
  - Root contributor law: `AGENTS.md`
  - Agent orientation: `CLAUDE.md`
  - Primary truth surface: `README.md`
  - Binding plan: `docs/plans/AURA_MASTER_PLAN.md`
- Files read:
  - `C:\corpus\THE_CHARTER_OF_COGNITIVE_SOVEREIGNTY.md`
  - `AGENTS.md`
  - `CLAUDE.md`
  - `README.md`
  - `docs/plans/AURA_MASTER_PLAN.md` via targeted search
  - `docs/security/SENTINEL_ADOPTION_STATUS.md` via targeted search
  - `docs/security/SENTINEL_PROTECTED_ACTIONS.md` via targeted search
- Files to edit:
  - `AGENTS.md`
  - `CLAUDE.md`
  - `README.md`
  - `docs/plans/AURA_MASTER_PLAN.md`
- Preconditions / dependencies:
  - `origin` was configured from the founder-provided remote:
    `https://github.com/90DegreeRobotics/AURA.git`
  - The tree already has uncommitted law/doc alignment changes from prior work.
  - This is a docs/rules-only change; no Rust runtime or launcher code is being
    changed in this unit.

## Steps

### Step 1 — Write launcher-first law
- [x] Action: Add root `AGENTS.md` law for Bevy frontend, local backend servers,
  Windows `.exe`, versioning, upgrade path, desktop launcher, no CLI user path,
  launcher-surface definition of done, and founder-owned visual witness.
- Files touched: `AGENTS.md`
- Expected outcome: Future agents cannot build terminal-first AURA work and call
  it done.

### Step 2 — Update orientation
- [x] Action: Update `CLAUDE.md` so every fresh agent sees the launcher-first
  law before touching runtime/UI work.
- Files touched: `CLAUDE.md`
- Expected outcome: Agent quick orientation matches root law.

### Step 3 — Update README truth
- [x] Action: Replace CLI-first quick-start framing with Windows launcher-first
  truth while preserving CLI as developer smoke only.
- Files touched: `README.md`
- Expected outcome: The public repo front page no longer presents CLI as the
  user path.

### Step 4 — Close master-plan toolkit question
- [x] Action: Update `docs/plans/AURA_MASTER_PLAN.md` to record the founder
  decision that Bevy is the front end and the Windows launcher is the product
  surface.
- Files touched: `docs/plans/AURA_MASTER_PLAN.md`
- Expected outcome: The binding plan no longer says UI toolkit is open.

### Step 5 — Verify, commit posture
- [x] Action: Run readback/search/diff/status checks. Commit by explicit
  pathspec if the docs/rules gate is clean. Push only if `origin` exists.
- Files touched: repository metadata only through git.
- Expected outcome: Local `main` is clean after commit; push is reported blocked
  if no remote exists.

## Test gate
- `git diff --check`
- targeted `rg` readback for launcher-first law terms
- `git status --short --branch`

Docs/rules-only change: do not run `cargo test --workspace` unless runtime code
is touched.

## Rollback
Do not delete. If this law is wrong, add a superseding founder directive in a new
commit and update the same law surfaces.

## Next-agent pickup
If interrupted:
1. Read this document.
2. Run `git status --short --branch`.
3. Check each step for completion.
4. Continue with the first unchecked step.
