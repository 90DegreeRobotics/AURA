# AGENTS.md — Aura

You are working in `C:\aura`, the Aura product home.

## Absolute law

**There is no gate before the Sentinel.**

- Sentinel is at Aura’s core. Do not add UI, model, tool, network, file, plugin, or installer paths that can approve or execute protected work before Sentinel authorizes it.
- Bind to `C:\sentinel-core` (path dep `sentinel_core`). Do not invent a second authority that can disagree with Sentinel Core.
- Fail closed when Sentinel is missing, ledger fails, or policy denies.
- Default mode is **enforce**. Shadow is an explicit logged opt-down; effects still do not execute in this runtime.
- No production bypass flags. No stubs in the protection path.
- Emergency stop must never be blocked by Sentinel; it is not an alternate approval path.

## Mode: IMPLEMENTING (Founder opened build 2026-07-20)

Build order (master plan P9):

1. Sentinel client + boot supervisor (deny-by-default) — **in progress / landed L0**
2. Protected handlers one-by-one with deny tests
3. Ledgered memory writes
4. Council authority path
5. Broker-mediated model/tool
6. Operator shell
7. Certify

Do not skip ahead to UI/models while L0 gaps remain. Do not claim Certified.

## Source of truth

| Need | Read |
| --- | --- |
| Single binding plan | `docs/plans/AURA_MASTER_PLAN.md` |
| Release-gate doctrine | `docs/security/SENTINEL_IMPERVIOUS_PROTOCOL_MASTER_PLAN.md` |
| Research copies | `research/canon/` |
| Adoption status | `docs/security/SENTINEL_ADOPTION_STATUS.md` |
| Protected actions | `docs/security/SENTINEL_PROTECTED_ACTIONS.md` |

## Working rules

1. Follow the current phase/stage. Do not skip Exit Gates.
2. Docs must not outrun code. Documentation Truth Rules apply.
3. When adding a protected surface, update `SENTINEL_PROTECTED_ACTIONS.md` and add a deny-before-side-effect test in the same change.
4. Prefer binding Core action IDs; do not invent Aura-only allow paths.
5. Failures must be visible — never silent allow.

## Definition of done (any change)

- Carved Law still holds.
- No new pre-Sentinel side effect.
- Deny test or proof updated if surfaces changed.
- Status labels honest (Implementing, not certified).
