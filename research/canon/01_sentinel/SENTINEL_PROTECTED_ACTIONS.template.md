# Sentinel Protected Actions Template

Copy this file into each product repository as `docs/security/SENTINEL_PROTECTED_ACTIONS.md`. Every protected action must be listed before certification can pass.

## Inventory Rules

- List every operation that can change state, reveal sensitive information, affect a person, affect hardware, communicate externally, execute tools, launch processes, modify memory/identity, or influence model behavior toward harm.
- Unlisted protected surfaces are stop-ship findings.
- Each row must name the mediation point and the deny test that proves failure before side effects.

## Protected Action Registry

| Action ID | Surface | Caller | Mediation point | Side effect if allowed | Deny test | Status |
| --- | --- | --- | --- | --- | --- | --- |
| example.route.write | Route | API client | Sentinel authorize before handler body | Persists state | `tests/...` | Planned |

## Unclassified Surfaces

Record every route, command, UI action, model call, tool call, file/network/process path, hardware path, and install/update path that has not yet been classified.

| Surface | Location | Why unclassified | Owner | Due |
| --- | --- | --- | --- | --- |
| | | | | |

## Explicit Non-Protected Surfaces

Only list surfaces that cannot produce protected side effects. Emergency-stop paths may appear here, but they must never become alternate approval paths.

| Surface | Location | Justification |
| --- | --- | --- |
| Health check | | Read-only health with no sensitive disclosure |
| Emergency stop | | Independent safety brake; never an approval path |
