# Aura

**Home:** `C:\aura`
**Law:** There is no gate before the Sentinel.
**Mode:** Implementing (Founder opened build 2026-07-20) — L0 Sentinel-first runtime.
**Contributor law:** `AGENTS.md` (Chronos-parity SOP) · orientation: `CLAUDE.md`
**Product law:** Bevy Windows launcher first. CLI is developer-only.

Aura is self-contained: the current L0 Sentinel guard lives inside `crates\aura_runtime`.
Default policy is deny-all. Default mode is enforce. No work mode without authorized
boot-continue. Sentinel works the same: enforce first, fail closed, no second authority.

## Product Surface

AURA is a real Windows desktop app from the first build. The canonical user surface is the
compiled Bevy launcher. Rust/Python backends may run as supervised local services behind the
launcher, but terminal commands are not the operator path.

Every user-facing feature must land as a Bevy button, control, card, or workflow. CLI commands
may exist as developer harnesses only; CLI-only work is not product-complete.

The launcher must own version/build truth and the upgrade path as soon as that lane exists.
Until then, versioning/update support is planned, not shipped.

First live slice:

- `crates\aura_launcher` is the compiled Bevy launcher.
- The launcher shows the NeuroCognica logo, AURA identity, the full
  `Archetypes - Utilizing - Reflective - Architecture` acronym, version/build identity,
  Sentinel/boot status, decision ledger path, document database status, document intake
  controls, and planned local service readiness.
- On launch, the header fades into the word AURA and shows a small `LAUNCHER ALIVE`
  signal so startup never reads as a blank black screen.
- The launcher button for boot continuation goes through `aura_runtime` and refuses under the
  current deny-all policy before side effects.
- `crates\aura_documents` frames UTF-8 text documents with the required NeuroCognica metadata
  envelope, the tracked NeuroCognica logo embedded in a print-ready HTML artifact, BLAKE3
  source/text/metadata hashes, deterministic chunks, and a real RocksDB store under the AURA
  data directory. Ingested documents get a synced append-only BLAKE3 Forever-chain event plus
  a RocksDB-backed document MMR leaf/root update in the same database batch. The launcher shows
  that store's path, counts, and MMR state and exposes Add File / Add Folder / Frame Selected /
  Ingest Selected controls.
- Document frame/ingest buttons are real brokered controls: source reads enter
  `file.read_sensitive`, DB appends enter `memory.write`, and the current deny-all policy
  refuses before those side effects.
- Chat, image generation, TTS, STT, embeddings/retrieval, operator corpus import, memory
  workbench, installer/update, and certification are not shipped yet.

## Document Framer

All documents that enter the future AURA RAG database must pass through the NeuroCognica
document frame first. This Rust layer preserves the official metadata shape from
`C:\AURA-Lab\Doc_Framer\nc-framer.py` (`project`, `title`, serialized ID, engineer, date,
revision, rights), upgrades it with the tracked NeuroCognica logo, and stores deterministic
hashes, chunks, a print-ready HTML artifact, and a Forever-chain ingest event before storage.

Live now:

- UTF-8 text intake for Markdown, text, JSON/JSONL, CSV/TSV, TOML, and YAML.
- Real RocksDB store at `%LOCALAPPDATA%\NeuroCognica\AURA\documents\documents.rocksdb` by
  default, or under `$env:AURA_DATA_DIR\documents\documents.rocksdb` when that override is set.
- RocksDB stores frame records, chunk records, print records, normalized source text, index
  keys, counters, append-only `document.ingested` Forever records, and MMR leaf/event/peak
  records in one synced write batch per new document.
- Print-ready HTML records use the real `assets\brand\neurocognica_logo_transparent.png`
  asset as an embedded data URI and include the AURA title block, source hashes, frame ID,
  rights, and printable body formatting.
- BLAKE3 hash-chain proof is live for document ingest events, and each new ingest is bound to
  a replay-verifiable RocksDB-backed MMR accumulator root.
- Launcher document workbench with native Add File / Add Folder selection, visible selected
  source state, current DB path/counts, and Sentinel-mediated Frame Selected / Ingest Selected
  attempts. Under the current deny-all policy those attempts refuse before reading or appending.

Not live yet:

- Authorized operator import policy/consent, recursive folder scanning, PDF/DOCX extraction,
  OCR, embeddings, vector search, and retrieval ranking.
- Mass import of NeuroCognica canon into the product database. That workflow is a protected
  sensitive-file-read plus memory-write surface and must be Sentinel-authorized before it becomes
  an executing operator button.

## Launcher

```powershell
cd C:\aura
cargo run -p aura_launcher
```

Desktop/Start Menu launcher path:

```powershell
cd C:\aura
pwsh -File scripts\install_launcher_shortcut.ps1
```

That builds release, copies `target\release\aura_launcher.exe` to `dist\aura_launcher.exe`,
copies the tracked AURA icon to `dist\aura.ico`, copies `assets\brand` to
`dist\assets\brand`, creates `AURA.lnk` in `C:\Users\m`, on the user Desktop, and in the
Start Menu, and attempts the Public Desktop shortcut when Windows permissions allow it.

The launcher opens with a native Bevy startup fade into `AURA` plus a truthful launcher-alive
indicator while deeper runtime status lines settle. The launcher owns its UI camera, font
handle, and brand-image asset path explicitly so the installed release build is not a black
surface waiting on implicit Bevy defaults.

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
  crates\aura_documents\ # NeuroCognica document frame + RocksDB Forever store foundation
  crates\aura_cli\       # developer harness only, not the product surface
  crates\aura_launcher\  # Bevy Windows launcher / product shell
  assets\                # tracked AURA icon + brand assets used by launcher/install helper
  NC LOGOS\              # Founder-provided NeuroCognica source logo variants
  aura\                  # Founder-provided AURA/archetype source art
  scripts\               # build/install helpers for launcher shortcut
  docs\plans\            # master plan
  docs\security\         # Impervious adoption pack
  research\canon\        # doctrine copies
```

## Carved Law

No gate before Sentinel.
No protected action without Sentinel.
No Sentinel, no ship.

## Status

**Implementing, not certified.** First Bevy launcher slice, document frame/store foundation,
and Sentinel-mediated document intake controls are live; broader AURA organs remain planned or
blocked behind Sentinel proof. See
`docs/security/SENTINEL_ADOPTION_STATUS.md`.
