# Sentinel Certification Report Template

Copy this file into each product repository as `docs/security/SENTINEL_CERTIFICATION_REPORT.md` after running certification. A release claim requires a passing strict report against the packaged artifact, not only the source tree.

## Report Header

- Product:
- Repository:
- Commit:
- Build identity:
- Policy identity:
- Certification command:
- Certification mode: `--strict`
- Result: FAIL / PASS
- Report generated at (UTC):

## Evidence

| Gate | Result | Evidence path | Notes |
| --- | --- | --- | --- |
| Sentinel SDK present | | | |
| Protected action inventory complete | | | |
| Route / command / UI coverage | | | |
| Model / tool / file / network / process / hardware mediation | | | |
| Handler-level deny tests | | | |
| Deny-all paralysis test | | | |
| Ledger verification | | | |
| Policy signatures | | | |
| Release artifact signatures | | | |
| SBOM present | | | |
| Dependency audit (no unresolved critical/high) | | | |
| No production bypass flags | | | |
| Shadow mode disabled for release | | | |
| No stubs in protection path | | | |
| Docs do not outrun code | | | |

## Open Findings

| ID | Severity | Finding | Blocks release? | Status |
| --- | --- | --- | --- | --- |
| | S0-S4 | | yes/no | open/fixed |

## Sign-Off

- Engineer:
- Founder release acceptance (only for explicitly accepted S3 items):
- Certification artifact hash:
