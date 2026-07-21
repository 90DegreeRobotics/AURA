# AURA Docs Magnum Opus — Index Bundle

This folder holds the exhaustive AURA documentation inventory scanned from `C:\` on 2026-07-20.

## Start here

| File | Purpose |
| --- | --- |
| `AURA_DOCS_MAGNUM_OPUS_CATALOG.md` | Human-readable **complete** path list (all roots) + canon subset |
| `AURA_DOCS_MAGNUM_OPUS_INDEX.csv` | Machine-complete index (path, root, ext, bytes, mtime) |
| `AURA_DOCS_MAGNUM_OPUS_INDEX.jsonl` | Same as CSV, one JSON object per line |
| `AURA_DOCS_MAGNUM_OPUS_SUMMARY_BY_ROOT.txt` | Counts by root |
| `AURA_DOCS_CANON_SIGNAL.csv` | Doctrine/charter/plan keyword subset (~1262) |
| `AURA_DOCS_VISION_CORE.csv` / `.md` | Vision-core titles; MD is **deduped by filename** |
| `AURA_DOCS_OVERSIZED.csv` | Files ≥ 5MB (maps, dumps, PDFs) |
| `AURA_DOCS_DUPLICATE_GROUPS_TOP200.csv` | Near-duplicate groups from earlier pass |
| `AURA_DOCS_SCAN_PROGRESS.txt` | Scan run log |

## Method

- Walked AURA-ecosystem roots under `C:\` (plus Documents/Desktop/tmp/Youtube/coder_tests/c with name/path filters).
- Included: `.md .mdx .txt .rst .adoc .pdf .docx .doc .rtf .html .htm`
- Skipped: `node_modules .git target dist build __pycache__ venv ComfyUI models checkpoints` caches
- **Nothing intentional left out** inside those roots for those extensions.

## Inflation notes

- `C:\chronos` is large partly due to `.claude/` and `out/` generated docs — still listed so nothing is hidden.
- Near-duplicate content across NRI / master_codex / corpus / Architectura is common; use Vision Core MD for unique titles, full CSV for every copy.

## Next for the real vision build

1. Treat `C:\NRI\Sentinel\SENTINEL_IMPERVIOUS_PROTOCOL_MASTER_PLAN.md` as the security release gate.
2. Treat Architectura / NRI charter + Complete Codex + EGD volumes as corporate/constitutional spine.
3. Use Vision Core MD as the reading order seed; expand from full catalog only when a title is missing.
