# Sentinel Adoption Status — Aura

Sentinel adoption status for this repository:

- Product: Aura
- Repository: `C:\aura`
- Canonical Sentinel plan source: `C:\NRI\Sentinel\SENTINEL_IMPERVIOUS_PROTOCOL_MASTER_PLAN.md`
- Local copy path: `docs/security/SENTINEL_IMPERVIOUS_PROTOCOL_MASTER_PLAN.md`
- Sentinel mode required for release: enforce
- Protected action inventory path: `docs/security/SENTINEL_PROTECTED_ACTIONS.md`
- Certification report path: `docs/security/SENTINEL_CERTIFICATION_REPORT.md`
- Last certification command: _(none yet)_
- Last certification result: _(none yet)_
- Open stop-ship findings: incomplete surface coverage; no signed policies; no full deny-all paralysis across future UI/tools; not release-certified

## Adoption Summary

| Surface | Mediated by Sentinel | Handler deny test | Notes |
| --- | --- | --- | --- |
| Boot / preboot | Yes (L0) | Yes (`fail_closed`) | Default deny-all; boot-continue → `effect.execute` |
| Action broker | Yes | Yes | Side effect only after `authorizes_effect()` |
| Routes / UI | Missing | Missing | No operator shell yet |
| Model / tool calls | Broker ready | Yes (model.generate deny) | No live model adapter yet |
| File / network / process | Broker ready | Partial | Mapped; no OS adapters yet |
| Memory / Codex / Forever Law | Decision log only | Partial | Local decision jsonl; Core ledger TBD |
| Council / identity | Missing | Missing | |
| Install / update | Missing | Missing | |

## Current Status

- Overall status: **Implementing, not certified**
- Founder build authorization: **2026-07-20** (“you are clear to build”)
- Sentinel SDK / bind: **path dependency on `C:\sentinel-core\crates\sentinel_core`**
- Runtime enforce mode: **default enforce**
- Default policy: **deny-all**
- Deny-all paralysis (broker + boot): **proven in `crates/aura_runtime/tests/fail_closed.rs`**
- Shadow mode: explicit opt-down; **effects still blocked**
- Bypass flags inventoried: none in L0 runtime
- Master plan: `docs/plans/AURA_MASTER_PLAN.md`

## Notes

- Aura binds to the **new Sentinel** at `C:\sentinel-core`. No second law.
- Carved Law: There is no gate before the Sentinel.
- Chronos `UnifiedSentinel` patterns are design reference only — not forked into Aura.
