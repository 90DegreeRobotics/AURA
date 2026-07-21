# Aura

**Home:** `C:\aura`  
**Law:** There is no gate before the Sentinel.  
**Mode:** Implementing (Founder opened build 2026-07-20) — L0 Sentinel-first runtime.

Aura binds to **`C:\sentinel-core`**. Default policy is deny-all. Default mode is enforce. No work mode without authorized boot-continue.

## Quick start

```powershell
cd C:\aura
cargo test --workspace
cargo run -p aura_cli -- status
cargo run -p aura_cli -- boot-continue   # expected: refused under deny-all
```

## Layout

```
C:\aura\
  crates\aura_runtime\   # boot + Sentinel client + broker + decision log
  crates\aura_cli\       # aura binary
  docs\plans\            # master plan
  docs\security\         # Impervious adoption pack
  research\canon\        # doctrine copies
```

## Carved Law

No gate before Sentinel.  
No protected action without Sentinel.  
No Sentinel, no ship.

## Status

**Implementing, not certified.** See `docs/security/SENTINEL_ADOPTION_STATUS.md`.
