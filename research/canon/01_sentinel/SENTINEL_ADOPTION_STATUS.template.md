# Sentinel Adoption Status Template

Copy this file into each product repository as `docs/security/SENTINEL_ADOPTION_STATUS.md` and fill every field. Status labels must follow the Documentation Truth Rules in the master plan.

Sentinel adoption status for this repository:

- Product:
- Repository:
- Canonical Sentinel plan source: `C:\NRI\Sentinel\SENTINEL_IMPERVIOUS_PROTOCOL_MASTER_PLAN.md`
- Local copy path: `docs/security/SENTINEL_IMPERVIOUS_PROTOCOL_MASTER_PLAN.md`
- Sentinel mode required for release: enforce
- Protected action inventory path: `docs/security/SENTINEL_PROTECTED_ACTIONS.md`
- Certification report path: `docs/security/SENTINEL_CERTIFICATION_REPORT.md`
- Last certification command:
- Last certification result:
- Open stop-ship findings:

## Adoption Summary

| Surface | Mediated by Sentinel | Handler deny test | Notes |
| --- | --- | --- | --- |
| Routes | Planned | Missing | |
| CLI commands | Planned | Missing | |
| UI actions | Planned | Missing | |
| Model / tool calls | Planned | Missing | |
| File / network / process | Planned | Missing | |
| Hardware / robotics | Planned | Missing | |
| Install / update | Planned | Missing | |

## Current Status

- Overall status: Planned
- Sentinel SDK present: no
- Runtime enforce mode: no
- Deny-all paralysis test: Missing
- Shadow mode forbidden in release: Planned
- Bypass flags inventoried: Missing

## Notes

- Do not mark a surface Protected without a handler-level deny test.
- Do not mark a product Certified for release without `sentinel certify --strict` passing.
