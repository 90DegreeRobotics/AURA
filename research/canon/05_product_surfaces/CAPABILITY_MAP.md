# CAPABILITY MAP — what's reachable from the live surface

**Last full audit: 2026-06-21** (camera-preservation truth + internal `1.2.0` release lane publication). Individual rows have been updated since — e.g. the **Katana bmesh** row through 2026-07-15 and the Geometry Forge / Museum Wall work through 2026-07-09/10 (see `STATUS.md` `## Current state — 2026-07-15`). A full reachability re-audit across every row is **due** and has not been run since 2026-06-21; do not read the rows below as uniformly re-verified past that date.

This is the anti-orphan ledger. Per **AGENTS.md §1 rule 11**, a capability is
"done" only when its **output** is reachable from a live entry point the operator
uses (Director route / GUI / wired pipeline) **and** an end-to-end test asserts
that output through that surface. `scripts\check_reachability.ps1` parses this
file and **fails** if any row marked `LIVE` lacks an entry point or e2e test —
so the map cannot lie. Rows honestly labeled `CLI-ONLY`, `WIRED-NO-E2E`, or
`ORPHAN` are visible technical debt, not "done."

Status legend:
- `LIVE` — reachable from the operator's surface AND has an e2e test asserting output.
- `WIRED-NO-E2E` — reachable, but no end-to-end test yet (debt; not "done").
- `API-ONLY` — Director route exists but the GUI never calls it (operator can't reach it normally).
- `CLI-ONLY` — only invocable via a `chronos` subcommand, not the GUI/Director.
- `ORPHAN` — built and unit-tested but its output never reaches any live surface (incl. "routed then discarded").
- `STUBBED` — a live-looking path whose emitter is a placeholder (e.g. renders a cube). Worse than ORPHAN: it masquerades.
- `BUILT-UNREACHABLE` — a genuine emitter/generator exists but nothing in the live path invokes it.

| Capability | Live entry point | E2E test | Status |
|---|---|---|---|
| **Sovereign render path** (pyrender, no Blender) | Foundry footer **RENDER BACKEND → Sovereign** → `POST /api/v1/pipeline/render-still` with `render_backend:"sovereign"` | — | API-ONLY — Director endpoint wired (Steps 1–3); Foundry footer radio + payload injection wired (Steps 5a–5c); operator click-witness + e2e test required for LIVE |
| Prompt → still render | GUI Renders/Chat → `POST /api/v1/pipeline/render-still` → CLI `dreamer render-still` | `crates/chronos_dreamer` render-still tests | WIRED-NO-E2E |
| Material intent (glass/metal/emissive/color) | inside render-still (`material_lexicon::enforce`) | `red_glass_sphere_enforced_to_clear_glass` (+ live-verified red glass sphere) | LIVE |
| Shape guard (vessel→primitive correction) | inside render-still (`shape_lexicon::enforce`) | `golden_goblet_torus_corrected_to_cylinder` (+ live-verified goblet) | LIVE |
| Sword bmesh geometry | render-still `normalize_object_kinds` force-override + compiler bmesh | `sword_kind_emits_bmesh_recipe_not_primitive` | WIRED-NO-E2E — generic western-sword stub (`build_sword_bmesh`); longsword/sword class only |
| Katana bmesh geometry | render-still routing + `build_katana_bmesh` (~1.05 m); tori-sori mid-blade curve; dark iron fittings + brass habaki; saya/sageo omitted from default void craft; void camera frames mid-nagasa | `katana_kind_emits_katana_bmesh_not_sword`, `routed_katana_survives_normalize_when_object_named_blade`, `katana_prompt_routes_to_katana_not_western_sword` | WIRED-NO-E2E — craft upgrade 2026-07-15; operator Create void PNG witness still required for LIVE |
| Foundry render catalog | Foundry **Renders** → recursive virtual PNG catalog over `renders/` + `out/`, filtered for gallery artifacts and preserving original paths for open/Blender/animate actions | `desktop/test_tabs_runtime.py::TestRendersTab::test_catalog_scans_nested_renders_and_out_artifacts` | WIRED-NO-E2E — catalog path tested; live UI output assertion still pending |
| Animate scene (camera presets + saved `.blend`) | Foundry **Renders** → `POST /api/v1/pipeline/animate-scene` | `chronos_dreamer` camera-preset/compiler tests; `desktop/test_tabs_runtime.py::TestRendersTab`; CLI saved-blend witness `6dc05000-7b17-4c43-aaa6-5edb8f7116e0` | WIRED-NO-E2E — generated animation scripts expose `test`/`preview`/`quality` render tiers with env overrides, camera presets include `orbital-180`, and saved `scene.blend` artifacts can be animated directly without sketch re-derivation; UI dispatch is tested, full live Foundry→Director render e2e still pending |
| Compile film (multi-shot) | Foundry **Cutting Room** → `POST /api/v1/pipeline/compile-film` | `compile_film_rejects_fps_with_400_and_spawns_no_job` | LIVE (API); GUI WIRED-NO-E2E |
| Sentinel-guarded Codex append | `POST /api/v1/codex/append` → Director Sentinel authority guard → Forever-Law append | `enforce_codex_append_denied_returns_403_and_writes_no_user_event` | LIVE (API) — denied enforce-mode requests seal `codex_append_requested` + `sentinel_authority_decision` and do not write the requested user event |
| Open-it journey (Blender 4.5 / folder / MP4) | Foundry Renders · Sophia Chat · Vistas · Create · Cutting Room | none | WIRED-NO-E2E |
| Sophia Vision Loop ledger + concept preview | Sophia Chat → `TurnDirector` ledger/concept prompt → `POST /api/v1/pipeline/concept-thumbnail` → ComfyUI **concept_scenic_sdxl** (Juggernaut scenic, not PBR albedo) → inline concept card/rating/final-render action → `/api/v1/assets` save-on-render | `desktop/tests/prompt_system/test_capability_ledger.py`, `desktop/test_tabs_runtime.py::TestSophiaChatTab`, `chronos_weaver` concept scenic tests, `chronos_director` concept helper tests | WIRED-NO-E2E — scenic SDXL + fidelity step branch + workflow/checkpoint/png_blake3 seal wired/tested; live Foundry + Comfy PNG + final render witness still pending |
| Search / memory recall | Foundry **Sophia Chat** memory threads → `GET /api/v1/search`; other GUI search surfaces | search tests | WIRED-NO-E2E |
| Lineage graph / view | Gallery **Open Lineage** → `GET /api/v1/lineage/view?artifact_id=` (Foundry Lineage tab remains frozen off rail) → `GET /api/v1/lineage/graph`, forge health, renders serve, evolve/fossils/cells | `chronos_director::lineage_graph::tests::*`; Gallery `lineage_view_url` helper | WIRED-NO-E2E — Gallery is the product entry; `katana_part_forged` maps to sword_genome stage nodes; live UI witness pending |
| Review / retry / critique | GUI → `/api/v1/review`,`/retry`,`/critique` | director tests | WIRED-NO-E2E |
| Evolution / genomes / cells (MAP-Elites) | `/api/v1/evolve`,`/genomes`,`/cells/*` | — | WIRED-NO-E2E |
| ComfyUI texture synthesis (Weaver) | render-still material policy → Weaver | material_policy tests | WIRED-NO-E2E |
| Operating Table 6FR assembly browse | Foundry **Operating Table** → phase1-stl-complete graph (74 nodes, 50/50 STL_EXPORT) + **6FR** tray tab + selection-aware viewport overlay + hardware reference nodes | `desktop/test_tabs_runtime.py::TestOperatingTableTab`, `desktop/assembly/test_graph.py` | WIRED-NO-E2E |
| **Lexicon recipe library (lathe vessels, sweeps, compound recipes)** | **CLI `chronos geometry build-primitive-object` only** | lexicon recipe tests | **ORPHAN** — render-still routes the geometry plan, seals `geometry_plans_routed`, then DISCARDS it; output never built from recipes (except Sword via a separate path). Reconnection in progress (plan_2026-06-12_2342). |
| `geometry_plan` routing | render-still computes + seals, then ignores | geometry_plan tests | ORPHAN (routed-then-discarded; only Sword realized) |
| Character genesis (MB-Lab humanoids) | `POST /api/v1/geometry/mblab` | — (`director_mblab_tests` is an empty mock — does not count) | WIRED-NO-E2E |
| Trellis image 3D mesh | Foundry **Operating Table → Imagine Part** → `POST /api/v1/geometry/trellis` | — (`director_trellis_tests` is an empty mock — does not count) | WIRED-NO-E2E |
| Operating Table STL surfacing | Foundry **Operating Table → Export STL** → linked local STL files / folder open | — | WIRED-NO-E2E |
| Generative lathe (vessel) geometry | `emit_lathe_object` + `apply_routed_geometry_plans` | `routed_vase_plan_promotes_vessel_kind`, `vessel_kind_emits_lathe_bmesh_call`, `witness_render_routed_vessel_png` (ignored — live Blender witness) | LIVE — routing + compiler output both asserted; live Blender witness is an opt-in `--ignored` test (not blocking) |
| Generative silhouette-solid (extrude+bevel) | `apply_routed_geometry_plans` → `emit_silhouette_solid_object` | `routed_silhouette_plan_attaches_generated_geometry`, `silhouette_solid_kind_emits_silhouette_bmesh_call`, `silhouette_solid_kind_emits_outline_extrude_not_cube`, `witness_render_routed_silhouette_png` (ignored — live Blender witness) | LIVE — routing + compiler output both asserted; live Blender witness is opt-in `--ignored` |
| 3D Printer Wireless Forge (OrcaSlicer to FlashForge 5M Pro) | Foundry **Operating Table → Forge Object** → Slicer/Sender | — | WIRED-NO-E2E — network dependency; manual visual proof needed |
| Typography organ (3D floating/attached text) | render-still prompt heuristic + Sophia intent `on_screen_text` → `chronos_typography` → Blender `text_add` | `chronos_typography` parse tests, `text_inject::injects_floating_text_from_prompt`, `compiler` `text_kind_emits_blender_font_curve` | WIRED-NO-E2E — operator witness (sphere + HELLO) pending |
| Create tab typography | Foundry **Create** → TEXT panel → `first-light --intent-json` → governed-artifact `inject_text_into_sketch` | `desktop/tests/test_typography_intent.py`, `desktop/test_tabs_runtime.py::TestCreateTab` | WIRED-NO-E2E |
| Create tab first-light witness receipt | Foundry **Create** → successful `first-light` run emits `out/<uuid>/CREATE_TAB_WITNESS/` with copied `scene.inspection.json` truth | `desktop/tests/test_create_tab_witness.py` | WIRED-NO-E2E |
| Create tab failure capture | Foundry **Create** → failed `first-light` run → `Copy Error` + `%LOCALAPPDATA%\\NeuroCognica\\ChronosFoundry\\support\\create_failures\\` | — | WIRED-NO-E2E |
| Create tab showroom polish (light pedestal, FRONT/SIDE/TOP, saved cameras) | Foundry **Create** → `first-light` → governed-artifact showroom (`PED_TOP_GLOW` floor on `sr_ped_top`; `_VIEW_NAMES`=front/side/top; saved `scene.blend` keeps `ChronosCamera.Hero/Front/Side/Top`; `scene.inspection.json` + `manifest.json.camera_inventory`) | `governed_showroom_script_keeps_hero_exposure_contract`, `governed_showroom_locks_canonical_view_set`, `desktop/test_tabs_runtime.py::TestCreateTab` (view-set + disclosure), `desktop/tests/test_create_tab_witness.py` | LIVE — render + UI witnessed 2026-06-21 (v1.2.0 internal build 7 published) |
| Crystal Spire recipe (prompt "spire") | Foundry **Create** → `first-light` "spire" → `build_crystal_spire_body` → locked four-view showroom | `crystal_spire_prompt_detection_is_token_aware`, `crystal_spire_recipe_builds_faceted_emerald_relic_in_locked_showroom` | WIRED-NO-E2E — deterministic body, render-witnessed; no automated render e2e |
| Floating Jawbone recipe (prompt "jawbone") | Foundry **Create** → `first-light` "jawbone" → `build_floating_jawbone_body` → locked four-view showroom | `floating_jawbone_prompt_detection_is_token_aware`, `floating_jawbone_recipe_builds_glowing_red_cube_arc_in_locked_showroom` | WIRED-NO-E2E — deterministic body; operator render pending |
| Canvas-on-Easel recipe (prompt "easel"/"canvas") | Foundry **Create** → `first-light` "easel"/"canvas" → `build_canvas_easel_body` (+ live Flux-dev ComfyUI canvas via `ComfyUiClient::new_flux_scene`) → locked four-view showroom | `canvas_easel_prompt_detection_is_token_aware`, `canvas_scene_prompt_extracts_subject_after_framing_marker`, `canvas_easel_recipe_builds_framed_canvas_in_locked_showroom` | WIRED-NO-E2E — deterministic easel + live Flux canvas, CLI render-witnessed (`out/canvas_easel/`); operator UI render pending |
| Cube-first Geometry Forge Phase 1 | Foundry **Create** → `first-light` → named recipes first → `chronos_forge` deterministic MeshEdit planner/gate → body-only script in locked showroom; writes `forge_receipt.json` | `chronos_forge` fake-render tests; `cargo test -p chronos_forge` | WIRED-NO-E2E — sword-family prompts can be accepted by the forge or fall back with an amber receipt; live Create UI witness pending |
| Video captions (ASS burn-in) | `compile-film` with `caption_cues` → ffmpeg `subtitles=` | `chronos_typography::ass::writes_ass_file` | WIRED-NO-E2E |
| 6FR part emboss labels | Foundry **Operating Table → Emboss Labels** → `desktop/assembly/emboss_6fr_stls.py` | `desktop/assembly/test_emboss_6fr.py` | WIRED-NO-E2E |
| Windows install program | `installer\\chronos_setup.iss` → `ChronosFoundry_Setup.exe` | — | WIRED-NO-E2E — installer now compiles locally; clean-machine install witness still missing |
| Windows uninstall program | Windows Apps & Features / Inno uninstaller | — | STUBBED — uninstall contract written; witnessed uninstall cycle still missing |
| Windows update check | Foundry **Diagnostics → System** → `desktop.updater_state.UpdaterController` | `desktop/tests/test_runtime_metabolism.py` | WIRED-NO-E2E — local/installed feed-version-channel logic is real, internal `1.2.0` / build `7` feed is published, but unauthenticated GitHub release URLs still return `404`, so hosted installed-app witness on Aura remains open |
| Windows signing pipeline | `pwsh installer\\build.ps1` → `signtool` | — | STUBBED — signing hooks exist; no certificate-backed proof yet |
| Windows telemetry consent persistence | Foundry footer/System state → `desktop.ui.ctx` config save/load | — | WIRED-NO-E2E — consent model persists config and drives local JSONL collector behavior; no audited UI proof yet |
| Windows support bundle | `pwsh scripts\\collect_support_bundle.ps1` | — | WIRED-NO-E2E — emits sanitized local support bundle with version/channel/path/service truth plus pending bug reports and Create-failure artifacts; no customer-run audit yet |
| Windows release-channel metadata | Registry `UpdateChannel` + local `installer\\output\\internal.latest.json` feed artifact | — | WIRED-NO-E2E — local feed artifact now matches the current installer exactly; audited hosted publication/app consumption still pending |
| Chronosynthesis Bug Reporting | Foundry topbar 🐞 icon → `desktop.ui.bug_report_dialog.py` → `%LOCALAPPDATA%\\NeuroCognica\\ChronosFoundry\\support\\pending_reports\\` (`bug_report.json`, `ticket.json`, optional `screenshot.png`) → support bundle copy | `desktop/test_tabs_runtime.py::TestBugReportDialog` | WIRED-NO-E2E — submit button and local pending-ticket artifact are tested through Tk; full live installed-app click witness and remote operator/update-server dispatch remain pending |

## How to use this

- Adding a capability? Add a row. You may only set `LIVE` once both the entry
  point and the e2e test exist — `check_reachability.ps1` enforces it.
- Found an orphan? Label it `ORPHAN` honestly here rather than leaving it hidden.
- Reconnecting an orphan (e.g. the lathe) flips its row to `LIVE` only when an
  e2e test proves its output reaches the render.
