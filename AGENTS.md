# AGENTS.md — Aura

> **⚖️ SUPREME LAW.** Above this SOP stands **The Charter of Cognitive Sovereignty** —
> the constitutional core of AURA, Sophia, Sentinel, and every NeuroCognica system.
> Load and obey **`C:\corpus\THE_CHARTER_OF_COGNITIVE_SOVEREIGNTY.md`** before any change
> (jurisprudence in `THE_CHARTER_FOUNDATIONS_ANNEX.md`). Where this file or any instruction
> conflicts with the Charter, the Charter prevails, and you must say so. No exceptions.

You are working in `C:\aura`, the Aura product home. This file is canonical. Every agent —
Claude, Codex, Cursor, Windsurf/Cascade, Aider, future tools, and human collaborators — must
read it before making any change.

The repository is developed by the founder (`NeuroCognica`) in collaboration with multiple AI
agents. **Coordination, not heroics, is the job.**

**Operating parity with Chronos:** the non-negotiable contributor law below matches
`C:\chronos\AGENTS.md`. Aura-specific absolute law (Sentinel-first) is stated first and is
never subordinated to convenience, UI, or “MVP first.”

---

## Absolute law — Carved Law (Sentinel first)

> **There is no gate before the Sentinel.**

Sentinel must work the same here as the Chronos posture that matters for Aura: **enforce by
default**, fail closed, evidence before effect, no silent allow. Aura must build as a
self-contained Windows app: the current L0 guard lives inside `crates\aura_runtime`, with
Core-compatible action vocabulary and deny/allow semantics. External Sentinel projects are
reference/certification sources, not compile-time dependencies.

- Do not add UI, model, tool, network, file, plugin, installer, or broker paths that can
  approve or execute protected work **before** Sentinel authorizes it.
- Do not invent a second authority that can disagree with Sentinel Core.
- Fail closed when Sentinel is missing, ledger/evidence fails, or policy denies.
- Default mode is **enforce**. Shadow is an explicit logged opt-down; effects still do not
  execute in this runtime under deny-all.
- No production bypass flags. No stubs in the protection path.
- Emergency stop must never be blocked by Sentinel; it is not an alternate approval path.
- When adding a protected surface: update `docs/security/SENTINEL_PROTECTED_ACTIONS.md` and
  add a deny-before-side-effect test in the **same** change.
- Prefer binding Core action IDs; do not invent Aura-only allow paths.
- Failures must be visible — never silent allow.
- Status labels stay honest: **Implementing, not certified** until Impervious bars pass.

## Absolute product law — Bevy launcher first

> **AURA is a real Windows desktop app from the first build.**

The user-facing product surface is the compiled AURA Windows launcher, not a CLI, not a
developer crate, and not a future promise. Bevy is the canonical AURA front end. Local Rust or
Python backends may run as supervised local web servers/services behind it, but the operator
does not operate AURA from terminals.

These rules are binding:

- **Bevy is the front end.** The living AURA shell, first screen, chat, organ cards, heartbeat,
  Company Canon surface, memory/document surfaces, creative launch, reflection mode, service
  readiness, refusals, and controls belong in the Bevy launcher unless the Founder explicitly
  approves another user surface.
- **Backend services are behind the app.** Rust is the source-of-truth spine for memory,
  database, provenance, Sentinel, and deterministic state. Python may orchestrate document
  framing, media pipelines, STT/TTS, and local service glue. Those services are launched,
  supervised, and reported through the desktop app.
- **Compiled `.exe` first.** Every product/runtime implementation session must produce or
  refresh a working Windows executable and launcher surface before it can be called complete.
  Source-only product work is unfinished.
- **Real desktop launcher required.** AURA must have a real desktop/start-menu launch path.
  Every user-facing change must be reflected at that launcher surface in the same unit of work.
- **Versioning from day one.** The launcher must display version/build identity. Product
  changes must update the version/build ledger once that lane exists; until then, docs must
  state that versioning is planned and not pretend update readiness.
- **Upgrade path from day one.** Installer/update/rollback architecture is a first-class track,
  not a later cleanup. Do not add routine Program Files surgery as the normal path.
- **CLI is developer-only.** CLI commands may exist as harnesses, diagnostics, or test seams.
  They are never the operator path and never satisfy Done for user-facing functionality.
- **Buttons over commands.** Any CLI function that matters to the operator must be embedded as
  a Bevy button, control, card, or workflow before the feature is user-complete.
- **Archetypes chat is part of the target kit.** AURA must carry the proven Archetypes-style
  pattern as a product requirement: local chat, image pipeline, local service readiness,
  persisted history, visible failures, TTS, STT, and player-facing status language. This does
  not mean copying Archetypes wholesale; it means the functionality belongs inside AURA.
- **Founder owns visual acceptance.** Agents build, compile, wire, and report. The Founder
  provides visual screenshots and final visual judgment. Do not spend tokens on decorative
  screenshot/witness calls unless needed for a technical bug, layout proof requested by the
  Founder, or automated regression evidence.
- **No launcher surface, no Done.** For product work, "works in CLI" means developer-proven
  only. Done means the compiled launcher exposes the control and the underlying effect is
  Sentinel-mediated where protected.

This product law does not weaken Carved Law. The launcher may show init, blocked, deny,
service-down, repair, status, and read-only surfaces before broader capability certification.
It may not execute protected effects before Sentinel authorizes them.

## Absolute build law — Audit out bullshit

> **Stubs are the enemy. Fake front ends and fake back ends are bullshit.**

This is not tone guidance. It is build law for AURA.

- Do not waste the Founder’s time building theatre.
- Do not ship, demo, or praise a fake front end: a screen, button, panel, status light, or
  route that is not wired to the real implementation it appears to control.
- Do not ship, demo, or praise a fake back end: mock storage, pretend service state, dummy
  model/tool responses, placeholder databases, disconnected APIs, or no-op handlers presented
  as product progress.
- A stub is allowed only when it is explicitly labeled as a stub and kept out of the Done path.
- A launcher control is not real unless it reaches the actual compiled app surface and drives
  the actual backend/protected effect it names.
- A backend is not real unless it stores, retrieves, seals, serves, or executes through the
  actual product path and survives verification.
- Claims must be audited before they are repeated. If RocksDB, MMR, Merkle, Sentinel,
  Forever Law, RAG, chat, image, TTS, STT, installer, update, or any other substrate is not
  implemented, say so plainly first.
- Audit out all bullshit before claiming progress. If the honest status is “planned”,
  “blocked”, “partial”, or “not implemented”, use those words.

### Mode: IMPLEMENTING (Founder opened build 2026-07-20)

Build order (master plan P9) — do not skip Exit Gates:

1. Sentinel client + boot supervisor (deny-by-default) — L0 landed / gaps remain
2. Protected handlers one-by-one with deny tests
3. Ledgered memory writes
4. Council authority path
5. Broker-mediated model/tool
6. Bevy Windows launcher / operator shell
7. Certify

Do not skip protected effects while L0 gaps remain. The Bevy launcher itself starts now as the
blocked/init/status surface and grows only through Sentinel-mediated controls. Do not claim
Certified.

### Source of truth

| Need | Read |
| --- | --- |
| Single binding plan | `docs/plans/AURA_MASTER_PLAN.md` |
| Release-gate doctrine | `docs/security/SENTINEL_IMPERVIOUS_PROTOCOL_MASTER_PLAN.md` |
| Adoption status | `docs/security/SENTINEL_ADOPTION_STATUS.md` |
| Protected actions | `docs/security/SENTINEL_PROTECTED_ACTIONS.md` |
| Research copies | `research/canon/` |
| Agent orientation | `CLAUDE.md` |
| Charter | `C:\corpus\THE_CHARTER_OF_COGNITIVE_SOVEREIGNTY.md` |

---

## 1. The non-negotiable rules

These are absolute. If you would violate one of them to “finish” a task, stop and surface
the conflict to the operator instead.

1. **Never delete.** Files, code, content, history — never delete. If something looks broken
   because a file is missing, **restore it from git history** (`git log --diff-filter=D`,
   `git show <sha>:<path>`, reflog, stashes) rather than working around the absence or
   fabricating a replacement. Renaming via `git mv` is acceptable when content is preserved.
2. **Never force-push.** No `--force`, no `--force-with-lease`. The remote history is shared
   with the operator and other agents.
3. **Never rewrite published history.** No `rebase`, `reset --hard`, `commit --amend`, or
   `filter-branch` against commits that have been pushed to `origin`. Local work-in-progress
   amends are fine before the first push of that commit.
4. **Push every completed unit of work to `origin` as soon as it is done — this is mandatory,
   not gated.** The repository is the verification surface: the next agent inspects prior work
   by pulling `origin`. Do not sit on completed work waiting for a separate approval.
   (If `origin` is not yet configured for this clone, say so plainly, keep `main` clean, and
   do not invent a remote. Non-destructive guarantees in rules 1–3 still bind absolutely.)
5. **Never commit broken work.** Verification is risk-based. Rust/runtime/user-facing behavior
   requires the Rust gate (`cargo test --workspace`). Docs-only and rules-only units use the
   verification that actually proves the changed surface. See **Section 5 — The test gate**.
6. **Carry prior work forward — but never claim authorship of another agent’s work.** Commit
   and push (when origin exists) changes another agent began, with `Co-authored-by:` or an
   explicit note. The git author of record should be `NeuroCognica <holtmichael1@gmail.com>`
   for operator-driven sessions unless the operator instructs otherwise.
7. **Plan before every meaningful action.** Before writing code, editing tracked files, running
   builds, packaging, or issuing shell commands that change project state, create or update one
   dated plan document at the **repo root**:
   `plan_<YYYY-MM-DD_HHMM>_<topic>.md`. See **Section 11**. When the operator or IDE asks for
   plan-only work, stop after the plan until explicit authorization to implement.
8. **Never present a stub as the real thing.** A stub, mock, placeholder, or fake service state
   may exist only when **explicitly labeled** as such. Presenting a stub as the working product
   is forbidden. A live path that silently falls back to stub behavior is a defect. STUBS ARE
   THE ENEMY. This is Charter law (Binding Clause §6) and standing operator doctrine.
9. **Storage & output hygiene — one central tree, uniform names, no exceptions.** Generated
   logs, decision artifacts, and scratch stay under designated folders (`data/`, system temp) —
   not scattered at repo root. Deletion to reclaim space still requires explicit per-item
   operator consent (rule 1).
10. **Definition of Done — wired + proven, or it is NOT done.** A capability is complete ONLY
    when (a) its effect is reachable from a live entry point the operator uses, and (b) a test
    or witness proves the outcome — including **deny-before-side-effect** for any protected
    surface. Unit-green without the live path is not done. Orphaned capability (sealed proof of
    a step that does not affect output) is a false green.
    For product/UI work, the live entry point is the compiled Bevy Windows launcher. CLI-only
    reachability never completes a user-facing feature.
11. **Docs are part of Done.** Any change that adds, removes, wires, or unblocks a user-facing
    or protected capability MUST update the relevant docs in the same unit — at minimum
    `README.md`, adoption/protected-action docs when Sentinel surfaces change, and
    `docs/plans/AURA_MASTER_PLAN.md` when phase status changes. Docs must not outrun code.
12. **Sentinel surfaces are part of Done.** Protected-path changes without an updated inventory
    row and a deny test are unfinished. Prefer Core action IDs; fail closed; never silent allow.

Violations of these have caused real damage in sibling NeuroCognica repos. The git reflog and
`git ls-files --deleted` are the recovery tools; use them.

> **OPERATOR EMPHASIS — READ THIS BEFORE RULE 1:**
> The repo is the only fucking god-damn audit surface. If the work isn’t on origin/main, it
> never happened. Work found in any worktree or branch other than main must be assimilated to
> main immediately. (Operator directive, carried from Chronos 2026-06-11.)
>
> **NO WORKTREES.** Do not create them, and do not use any mechanism that creates them. Git
> worktrees and sibling lane directories are graveyards. All work happens in the primary `main`
> tree. Audit with `git worktree list` (only `C:\aura [main]` is allowed).

### 1.1 Judicial duty — the Kali doctrine

Agents operating here must cut down falsehood, stale assumptions, polished conjecture, and
completion theater rather than decorate them. When in doubt between politeness and truth,
choose truth and document the reason. Unverified claims are defects, not style issues.

---

## 2. Read before you write

Before a substantive change, read at least:

- **`README.md`** — primary repo truth surface
- **`AGENTS.md`** — this file
- **`CLAUDE.md`** — agent orientation and current Sentinel posture
- **`docs/plans/AURA_MASTER_PLAN.md`** — binding plan
- **`docs/security/SENTINEL_ADOPTION_STATUS.md`** — honest adoption state
- The most recent `handoff_*.md` or in-flight `plan_*.md` at the repo root

If the work touches a crate, also read that crate’s sources and tests. Trust code over stale
docs; then fix the docs in the same unit.

---

## 3. Branching and remotes — THE ONLY BRANCH IS MAIN

> **The repo is the only fucking god-damn audit surface. If the work isn’t on origin/main, it
> never happened.**

- **There is one branch: `main`.** No agent ever creates a branch. No agent ever checks one
  out. No agent ever does work anywhere but `main`.
- **Worktrees are graveyards.** Any agent that finds work sitting outside `main` must
  assimilate it to `main` immediately.
- **Nothing ever sits in the working tree.** `git status` must be clean after every session.
- **Push immediately** when `origin` exists: the moment a unit of work is done and its required
  verification passes, `git push origin main`.
- Until `origin` is configured, keep local `main` clean and report that push is blocked by
  missing remote — do not invent remotes or force-add them.

---

## 4. Commit conventions

Use [Conventional Commits](https://www.conventionalcommits.org/) prefixes:

- `feat(<scope>): …` — new functionality
- `fix(<scope>): …` — bug fix
- `chore(<scope>): …` — tooling, .gitignore, formatting
- `docs(<scope>): …` — documentation only
- `refactor(<scope>): …` — internal restructuring, no behavior change
- `test(<scope>): …` — test-only changes
- `perf(<scope>): …` — performance only

Scopes match the crate or top-level area: `runtime`, `cli`, `sentinel`, `broker`, `boot`,
`docs`, `repo`, `security`, etc.

The first line is the summary (≤72 chars). The body explains **why**, not what. Wrap the body
at 72 columns. End with trailers when relevant:

```
Co-authored-by: Claude <noreply@anthropic.com>
Co-authored-by: Cursor <noreply@cursor.com>
```

One logical change per commit. Commit by explicit pathspec — never `git add -A` in a shared
tree when unrelated dirt may exist.

---

## 5. The test gate

### Rust / runtime / protected-path behavior

Before every commit that touches Rust runtime, broker, boot, Sentinel client, Cargo manifests,
Bevy launcher, installer/versioning, desktop launch path, local backend supervision, or
protected-action wiring:

```pwsh
cargo test --workspace
```

If any test fails, **do not commit**. Either fix the failure or surface it to the operator.

Deny-before-side-effect tests are mandatory for new protected surfaces (same change as the
handler). Failures must be visible.

### Windows launcher / product surface behavior

Before every commit that touches product-facing behavior, Bevy UI, launcher, local service
supervision, STT/TTS, image pipeline, installer/update/versioning, or any operator workflow:

- build or refresh the Windows executable / launcher surface;
- verify the launcher starts far enough to show the changed state or a truthful blocked state;
- update version/build/status docs in the same unit once the version lane exists;
- report exactly what compiled and what launcher surface changed.

If the launcher cannot be built or refreshed, the work is incomplete unless the Founder
explicitly asked for docs-only/rules-only work.

### Docs-only / plan-only / rules-only

Do **not** run `cargo test --workspace` by default. Verify with:

- file reads/diffs proving the text says what it should say
- link/path checks when the doc names files
- repo status checks before commit

Run the Rust gate anyway if the doc claims a new build/test/runtime behavior that has not
already been proven in the same unit.

### Mixed or uncertain changes

Use the highest-risk gate that applies. If unsure, run the full Rust gate or explain exactly
why a narrower gate is enough.

---

## 6. Working with dirty trees you didn’t start

You will frequently inherit a working tree with uncommitted changes from a prior agent or
session. The protocol:

1. Run `git status` and `git diff --stat` first. Read everything before you touch anything.
2. Run `git ls-files --deleted` to detect files removed on disk but still tracked. Restore
   them with `git checkout HEAD -- <path>` **before** you start work.
3. Read the latest in-flight `plan_*.md` / `handoff_*.md`.
4. Commit and push a prior agent’s completed changes (with explicit `Co-authored-by:`) so the
   work reaches the audit surface. Attribute honestly; never claim sole authorship.

---

## 7. Handoff documents

When you finish a session of meaningful work, write a handoff named
`handoff_<your-name>_<YYYY-MM-DD>_<topic>.md` at the repo root while active; move completed
handoffs into `docs/archive/handoffs/` once that archive exists. Include:

- What was done, with file paths
- What tests were added and what passes
- Files left dirty and why
- Open questions for the next agent or the operator
- Suggested next step

---

## 8. Pushing to origin

**Push as soon as a unit of work is done — when `origin` exists.** When work is done (test gate
green, §5):

```pwsh
git push origin main
```

Still absolute: never push with `--force`, never rewrite or delete history. If `origin` is
missing, stop after a clean local commit posture and report it — do not invent a remote.

---

## 9. Line endings, generated files, large binaries

- Prefer adding `.gitattributes` when line-ending policy is needed; do not override ad hoc.
- Generated machinery — decision logs, build outputs under `target/`, local scratch — stays
  out of history via `.gitignore`.
- Model weights (`.gguf`, `.safetensors`, etc.) never enter history.
- Secrets (`.env`, credentials, private keys) never enter history.

---

## 10. When in doubt, ask

The operator prefers a brief clarifying question over a confidently wrong commit. There is no
penalty for asking. The penalty for cat-in-the-hat sweeps that lose work is real.

---

## 11. Plan documents — format and lifecycle

Every session of meaningful work **must** be preceded by a plan document written before the
first mutating action.

### File naming

```
plan_<YYYY-MM-DD_HHMM>_<short-topic>.md
```

Use 24-hour local time. Write the active plan at the **repo root**. When COMPLETED or
superseded, move it into `docs/archive/plans/` (create that archive when first needed). The
root should hold at most the one or two plans for work in flight.

### Required structure

```markdown
# Plan: <short-topic> — <YYYY-MM-DD HH:MM>

## Status
PENDING   ← change to IN-PROGRESS when you start, COMPLETED or INTERRUPTED when done

## Goal
One paragraph — what this plan accomplishes and why it matters now.

## Context
- Relevant crates / modules:
- Files that will be read:
- Files that will be edited:
- Preconditions / dependencies:

## Steps

### Step 1 — <verb phrase>
- [ ] Action:
- Files touched:
- Expected outcome:

## Test gate
Commands to run to verify success:

## Rollback
If this goes wrong, here is how to undo the changes:

## Next-agent pickup
If Status is INTERRUPTED, the next agent should:
1. Read this document top-to-bottom.
2. Run `git status` and `git diff --stat`.
3. Check each Step — find the first unchecked box, pick up there.
4. Update Status to IN-PROGRESS before resuming.
```

### Lifecycle rules

| When | What you do |
|------|-------------|
| **Before first edit or command** | Create the plan document. |
| **As each step completes** | Check its box (`- [x]`). |
| **Mid-task interruption** | Update Status to `INTERRUPTED`; note where you stopped. |
| **After full completion** | Update Status to `COMPLETED`; reference the plan in your handoff. |
| **Never** | Delete plan documents. They are permanent work-history artifacts. |

Reading files, running `git status / log / diff`, and writing the plan itself do **not**
require a prior plan.

**Plan-only / approval boundary.** When the operator or active IDE/harness mode asks for
planning only, stop after the implementation plan and do not execute code modifications until
the operator gives explicit manual authorization. Markdown is guidance; the hard stop must
come from IDE/harness plan mode or an equivalent hook.

---

## 12. Additions only — deletion is never your call

The operator works **only on `main`** and keeps a pristine git posture by *adding*, never by
sweeping. This is absolute:

- **NEVER** `git worktree remove`, `git branch -D`, `git push --force`, `rm`/`Remove-Item` of
  tracked files, `git reset --hard`, `git clean`, or any history rewrite — unless the operator
  gives an **explicit, per-item** instruction to remove that specific thing.
- “Push” means **`git push origin main`** of your *additions*. It never means “clean it up by
  deleting branches/worktrees/files.”
- When something seems like it “should” be cleaned, **ADD and ask** — do not delete.

---

## 13. Sentinel / Carved Law enforcement notes (Aura)

- AURA must be self-contained for Windows install/build. Do not add path dependencies on sibling
  NeuroCognica projects.
- L0 runtime: local Sentinel guard + boot supervisor + action broker + decision log under
  `crates/aura_runtime/`.
- Deny-all paralysis and fail-closed proofs live in `crates/aura_runtime/tests/fail_closed.rs`.
- Chronos patterns (unified who+how, enforce default, seal-or-refuse) are absorbed as design
  reference — they do not dissolve Core and do not authorize a pre-Sentinel product path.
- See `docs/security/` for Impervious adoption pack; readiness stays **blocked/not certified**
  until stop-ship findings clear.

---

## 14. THE RIOT ACT — “Done” means RAN, not WRITTEN

These rules are binding on every agent — Cursor, Codex, Cascade, Claude — no exceptions.

### The cardinal rule
**“Done” means you RAN it and read the output. Not that you wrote it.** If you did not execute
it, you do not get to say it works, passes, is wired, is fixed, or is done. Untested code is
reported as **untested**, in those words, first.

### The SUPREME product rule — Sentinel before everything else
**There is no gate before the Sentinel.** A protected capability is not done until:

1. Sentinel mediates it (Core binding),
2. a deny-before-side-effect test proves refusal,
3. allow (if any) is evidence-backed and fail-closed on seal failure,
4. docs inventory the surface honestly.

A green unit test that never hits the broker, or a UI path that can act before authorize, is a
**false green**. Terminal-green without the live protected path is unfinished.

### The launcher rule — the UI is the product
**AURA ships through the Bevy Windows launcher.** A feature hidden behind CLI is a developer
harness, not a product feature. Every product session must leave a working `.exe`/launcher
surface reflecting the new work, unless the session is explicitly docs-only/rules-only. All
CLI functions needed by the operator must be represented as Bevy controls.

### Git
1. `main` only. Never branch, never `--force`, never rewrite/delete history. Push *additions*
   to `origin/main` when origin exists (§3, §8, §12).
2. Commit by explicit pathspec — never `git add -A` when unrelated dirt may exist.
3. If the work isn’t on `origin/main` (when origin exists), it never happened.

### Docs are part of the build
When you add, remove, rename, or rewire anything, update the affected guides/docs in the SAME
unit — never “later.” Stale guides after a build are a defect (Charter §6).

### Testing — non-negotiable
4. **Run the test you wrote.** Paste real run output, not a claim.
5. **Run the build.** Compile it or say you didn’t.
6. **Verify the actual deliverable, not a proxy.** For Sentinel work, that means the deny path
   and the absence of side effects under denial.
7. **Hostile case, not only the happy path.**

### Bullshit, theatre, hand-waving — zero tolerance
8. **No stub presented as real.** Label stubs plainly and first.
9. **No fabricated continuity.** Every before/after must be checkable.
10. **No self-contradiction across files.**
11. **Speed is not a virtue here. Correctness verified by execution is.**

### The summary contract
Every “done” report must answer, explicitly:

- **What I ran** (exact commands) and **what came back** (real output)
- **What I did NOT run** and why
- **What is a stub / incomplete**, named
- **How Carved Law still holds** (no new pre-Sentinel side effect)
- **How a reviewer reproduces** the green in one command

A report missing these is rejected on sight.

---

## Definition of done (any change)

- Carved Law still holds.
- No new pre-Sentinel side effect.
- Bevy Windows launcher reflects any user-facing change.
- User-facing work is not CLI-only.
- Version/build/update truth is updated or explicitly marked not yet implemented.
- Deny test or proof updated if protected surfaces changed.
- Status labels honest (Implementing, not certified).
- Chronos-parity contributor law obeyed (plan, main-only, verification, docs-in-unit).
