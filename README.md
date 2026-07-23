# Aura

**Home:** `C:\aura`  
**Law:** There is no gate before the Sentinel.  
**Mode:** Implementing (Founder opened build 2026-07-20) — L0 Sentinel-first runtime.
**Contributor law:** `AGENTS.md` (Chronos-parity SOP) · orientation: `CLAUDE.md`
**Product law:** Bevy Windows launcher first. CLI is developer-only.

Aura binds to **`C:\sentinel-core`**. Default policy is deny-all. Default mode is enforce. No work mode without authorized boot-continue. Sentinel works the same: enforce first, fail closed, no second authority.

## Product Surface

AURA is a real Windows desktop app from the first build. The canonical user surface is the
compiled Bevy launcher. Rust/Python backends may run as supervised local services behind the
launcher, but terminal commands are not the operator path.

Every user-facing feature must land as a Bevy button, control, card, or workflow. CLI commands
may exist as developer harnesses only; CLI-only work is not product-complete.

The launcher must own version/build truth and the upgrade path as soon as that lane exists.
Until then, versioning/update support is planned, not shipped.

## Developer smoke

```powershell
cd C:\aura
cargo test --workspace
cargo run -p aura_cli -- status
cargo run -p aura_cli -- boot-continue   # expected: refused under deny-all
```

This proves the current L0 deny-first runtime harness. It does not prove a user-facing AURA
feature until the compiled launcher exposes the control.

## Layout

```
C:\aura\
  crates\aura_runtime\   # boot + Sentinel client + broker + decision log
  crates\aura_cli\       # developer harness only, not the product surface
  crates\aura_launcher\  # planned Bevy Windows launcher / product shell
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
