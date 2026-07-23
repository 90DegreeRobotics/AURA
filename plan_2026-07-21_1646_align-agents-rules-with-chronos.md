# Plan: align-agents-rules-with-chronos — 2026-07-21 16:46

## Status
COMPLETED

## Goal
Make Aura’s repo agent rules match Chronos operating law (Charter, non-negotiables, main-only, plans, verification, Done = ran), while keeping Aura’s Carved Law absolute: **there is no gate before the Sentinel**. Sentinel works the same — enforce by default, fail closed, bind to `C:\sentinel-core`, no second authority.

## Context
- Relevant crates / modules: docs/rules only (no runtime change this unit)
- Files that will be read: `C:\chronos\AGENTS.md`, `C:\chronos\CLAUDE.md`, `C:\chronos\.agents\AGENTS.md`, Aura master plan / security docs
- Files that will be edited:
  - `AGENTS.md` (root — canonical)
  - `research/canon/05_product_surfaces/AGENTS.md` (replace Archetypes fork with pointer)
  - `CLAUDE.md` (new — Chronos-style orientation)
  - `.agents/AGENTS.md` (new — plan-mode reminder)
  - `.cursor/rules/sentinel-first.mdc` (new — always-apply Cursor rule)
- Preconditions: Charter at `C:\corpus\THE_CHARTER_OF_COGNITIVE_SOVEREIGNTY.md` exists

## Steps

### Step 1 — Write this plan
- [x] Action: create dated plan at repo root (Chronos plan lifecycle)
- Files touched: `plan_2026-07-21_1646_align-agents-rules-with-chronos.md`
- Expected outcome: recovery artifact exists before further edits

### Step 2 — Rewrite root AGENTS.md
- [x] Action: port Chronos SOP structure; lead with Charter + Carved Law; Aura paths/scopes; honest remote note
- Files touched: `AGENTS.md`
- Expected outcome: one canonical contributor law matching Chronos posture + Sentinel-first absolute

### Step 3 — Neutralize conflicting Archetypes AGENTS copy
- [x] Action: rewrite research canon AGENTS to point at root (never delete)
- Files touched: `research/canon/05_product_surfaces/AGENTS.md`
- Expected outcome: no Archetypes launcher/desktop rules applied as Aura law

### Step 4 — Add CLAUDE.md + .agents reminder + Cursor rule
- [x] Action: Chronos-parity orientation surfaces; Cursor alwaysApply Sentinel-first
- Files touched: `CLAUDE.md`, `.agents/AGENTS.md`, `.cursor/rules/sentinel-first.mdc`
- Expected outcome: every agent entry path states the same Carved Law

### Step 5 — Verify
- [x] Action: read back key files; confirm Carved Law + Chronos non-negotiables present; no Archetypes product paths in always-applied law
- Files touched: none
- Expected outcome: docs-only gate satisfied

## Test gate
Docs-only / rules-only unit — no `cargo test --workspace` required.
Verify by file reads:
1. Root `AGENTS.md` leads with Charter + “no gate before the Sentinel”
2. Chronos-style sections present: non-negotiables, main-only, plans, Done = ran
3. Research canon AGENTS is a pointer, not Archetypes SOP
4. Cursor rule `alwaysApply: true` carries Carved Law

## Rollback
Revert the listed files via `git checkout HEAD -- <path>` if needed (additions-only posture otherwise).

## Next-agent pickup
If Status is INTERRUPTED, the next agent should:
1. Read this document top-to-bottom.
2. Run `git status` and `git diff --stat`.
3. Check each Step — find the first unchecked box, pick up there.
4. Update Status to IN-PROGRESS before resuming.
