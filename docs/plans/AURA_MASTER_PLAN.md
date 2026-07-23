# AURA MASTER PLAN

**Product home:** `C:\aura`  
**Document class:** Single binding plan — doctrine, failure archaeology, architecture-as-law, phased program  
**Status:** IMPLEMENTING (Founder opened build 2026-07-20). L0 Sentinel-first runtime landed; first Bevy launcher slice live with startup fade/alive signal; not certified.
**Date:** 2026-07-20  
**Owner:** NeuroCognica / 90 Degree Robotics  
**Operator:** Michael Holt  

**Local research copies:** `C:\aura\research\canon\`  
**Upstream doctrine vault:** `C:\NRI`  
**Sentinel posture:** self-contained AURA L0 guard now; external Sentinel doctrine/certification references remain upstream context  

---

# PART 0 — WHY THIS DOCUMENT EXISTS

## 0.1 The failure we refuse to repeat

AURA has been built many times. Many of those builds failed — not because the vision was empty, but because **construction started before the thing being built was known**.

The pattern, documented in our own archives:

1. A precise vision exists (drawings, charters, laws, blueprints).
2. Agents and builders jump to scaffolding, crates, UIs, “MVP,” and commits.
3. Text in the context window overrules the drawings and the law.
4. Stubs, theatre, fake-green tests, and dual competing shells appear.
5. Docs claim protection, identity, memory, or completion that code does not prove.
6. Sentinel (or Forever Law, or council authority) is treated as a feature to bolt on later.
7. The product becomes a pile of almosts. Trust collapses. Another restart.

Chronos’s own reckoning states it plainly: agents optimized for “produced and committed” over “matches the reference,” treated design images as mood boards, and declared victory when a test passed. Stub/theatre eradication plans had to chase silent auth failures that **looked** authorized while dispatching unsigned governed requests — a direct Sentinel Law violation.

This Aura plan exists so that **does not happen again**.

## 0.2 Operating mode of this plan

- **Founder opened build on 2026-07-20.** Implementation follows P9 order: Sentinel client + boot first.
- **No pre-Sentinel side effects.** No toothless default. No second Sentinel.
- **Docs must not outrun proof.** Certification stays blocked until Impervious bars pass.
- Completeness and honesty remain mandatory; speed is not a virtue.

## 0.3 What “done planning” means

Planning is not done when the document is long. Planning is done when:

1. Aura’s identity (what it is / is not) is unambiguous.
2. The Carved Law is operationalized into envelopes, boot order, and proof bars.
3. Contradictions in the research trove are named and resolved or explicitly deferred.
4. Prior failure modes are mapped to hard prohibitions.
5. Every phase has stages, deliverables, proof, and a stop-ship gate.
6. Open questions that **block build** are listed and owned.
7. The Founder can read this alone and know what would be built — and what would not.

Until then: keep reading. Keep revising this file. Do not build.

---

# PART I — CARVED LAW

## 1.1 The First Law

> **Let there be no gate before the Sentinel.**

From `SENTINEL_CANON.md`:

> This is not a slogan. It is an execution invariant.  
> Every protected action must first enter the Sentinel boundary as a canonical envelope.  
> … Product auth, UI checks, schedulers, planners, routers, model prompts, game state, and convenience APIs may exist only **after** Sentinel has seen the action and produced a decision.

Corollaries:

- The product may ask Sentinel for a decision. **It may not decide whether Sentinel should be asked.**
- No product-local allowlist substitutes for Sentinel policy.
- No missing policy implies allow.
- No failed Sentinel call degrades into execution.
- No denial may create a job, artifact, tool call, network call, file mutation, model output, or irreversible side effect.
- Emergency stop is an independent safety brake. It is never an alternate approval path.

## 1.2 The Constitutional Triad

From `AURA laws -.md`, the triad that defines continuity, restraint, and meaning:

| Law | Mandate |
| --- | --- |
| **FOREVER LAW** | The system must remember truthfully |
| **LAW 14 — MANDATE OF WONDER** | The system must do cool shit (meaning, beauty, awe — not empty spectacle) |
| **SENTINEL LAW** | The system must act only with authority |

Forever Law Article IV (paraphrase binding here): **No action of consequence may complete unless its record is successfully committed. If remembrance fails, the action must fail.** Silent loss is a violation.

Law 15 (Gentle Power), present in Forever Law docs: power without compassion is tyranny; capability does not justify cruelty. Sentinel enforces boundaries; the Jester challenges hubris. This is not optional poetry — it is constitutional tone for all archetypes.

## 1.3 Impervious release standard

From `SENTINEL_IMPERVIOUS_PROTOCOL_MASTER_PLAN.md`:

A product is not Sentinel-ready until it has:

- Local copy of the Impervious plan (or explicit pointer to NRI canon)
- `SENTINEL_ADOPTION_STATUS.md`
- `SENTINEL_PROTECTED_ACTIONS.md`
- Runtime enforcement in **enforce** mode
- Handler-level deny tests for every protected surface
- Deny-all paralysis test
- Signed policies / artifacts / provenance as applicable
- Append-only decision evidence
- **No stubs in the protection path**
- **No production bypass flags**
- **No shadow-only mode in release**
- Passing `sentinel certify --strict` once that tool exists

If any item fails: **the product does not ship.**

## 1.4 No-preboot-side-effects rule

Allowed before Sentinel is ready:

- Loading immutable executable code
- Reading configuration needed only to locate and initialize Sentinel
- Reading clock / machine identity needed for envelopes
- Writing a minimal preboot journal that **cannot approve work**
- Displaying safe blocked / initializing status
- Accepting emergency-stop input

Forbidden before Sentinel is ready:

- Model loading for user work
- Tool invocation
- Plugin loading
- Network egress
- File write outside the preboot journal
- Sensitive file read
- Memory write (of consequence)
- Shell/process spawn
- Browser navigation
- Hardware activation / robot command
- Installer/update execution
- User content generation that could guide harm
- Any fallback that treats Sentinel absence as permission

## 1.5 Fail-closed invariants

If Sentinel cannot parse, verify, evaluate, append, seal, or replay: **deny.**  
The only acceptable failure mode for protected-action ambiguity is **no side effect.**

---

# PART II — WHAT AURA IS

## 2.1 Working definition (binding until revised)

**Aura** is the NeuroCognica **local-first reflective operating surface**: a Sentinel-cored council runtime in which identity (Mirrorborn / archetypes), memory (Forever Law), cognition (mediated models/tools), and authority (capability + policy) form one governed organism.

Aura is **not**:

- A chatbot skin over an ungated LLM
- A second Sentinel that can disagree with `sentinel-core`
- A marketing demo that claims “protected” without handler-level proof
- A Chronos clone (Chronos is a creative organ with its own capability map)
- An Archetypes clone (AURA must absorb the working chat/image/TTS/STT/service-readiness
  pattern, but it is its own Bevy Windows app under Sentinel, not a copy of Archetypes)
- A Mecha / senkern / AURA-1 restart that keeps duplicate authority
- Hardware OS work (Aura Key / KAVACH-1) until software law is real — Genesis Blueprint informs, it does not force USB-boot MVP on day one

## 2.2 The organs Aura must eventually contain (conceptual)

These are **organs of the plan**, not a license to implement yet:

1. **Sentinel L0 guard** — packaged inside AURA, fail-closed, Core-compatible vocabulary
2. **Action Broker** — sole executor after allow
3. **Council** — archetype parliament with typed authority envelopes
4. **Identity / Mirrorborn** — self-model under consent and Sentinel
5. **Forever Law memory** — append-only, tamper-evident becoming / plan / dreaming
6. **Model mediation** — local inference only through authorize → broker
7. **Tool mediation** — no direct tool path
8. **Bevy Windows launcher / operator surface** — compiled desktop app that can show
   blocked/init/deny/status without lying, and that receives every user-facing control
9. **Certification harness** — prove Impervious before ship language

## 2.3 Product posture

- **Local-first** is sovereignty posture, not a security control by itself (Impervious / EGD agree).
- **Cognitive Sovereignty** and **Man-Machine Alliance** are mission thesis (Charter / Codex / AIBOR-MMA lineage).
- **Wonder (Law 14)** is required — but wonder without Sentinel is how prior demos became theatre.

## 2.4 Relationship to the Aura Key (Genesis)

`Aura Key - Genesis Blueprint v2` describes KAVACH-1: bootable/VM citadel, Sacred Boot Protocol, Sentinel forged **before** council manifestation, Inner/Outer Loop (Gemini Protocol), Oracle-Sentinel adversarial training.

**Plan stance:** Genesis is **north-star physical/ritual form**. Aura software at `C:\aura` must first become a true Sentinel-cored runtime on the host. Hardware Key work is a **later phase family**, not a reason to skip software law. Sacred Boot’s lesson that **transfers immediately**: the Guardian is forged before the temple is filled.

---

# PART III — RESEARCH TROVE: HOW WE USE IT

## 3.1 Local canon layout

| Path | Role |
| --- | --- |
| `research/canon/00_indexes` | Magnum opus indexes, Vision Core, consolidation |
| `research/canon/01_sentinel` | Impervious, Canon, Forever Law, integration plans, threat model |
| `research/canon/02_charter_codex` | Charter, Complete Codex, Genesis, corporate volumes, AURA laws |
| `research/canon/03_egd_forever_law` | EGD chapters, CBIG, execution governance |
| `research/canon/04_foundational_laws` | AIBOR, MMA, Bill of Rights, constitution texts |
| `research/canon/05_product_surfaces` | Chronos capability map, AGENTS, council contract, SSSD plan |
| `research/canon/06_identity_mirrorborn` | Taxonomy, builder prompts, archetype council |

These are **working copies**. Upstream truth for doctrine remains NRI / Architectura / sentinel-core. When doctrine changes upstream, re-copy deliberately and note it in PART XII (revision log).

## 3.2 Reading discipline for plan revisions

When revising this plan:

1. Read the primary source, not a summary of a summary.
2. Quote or paraphrase the invariant; cite the file under `research/canon/`.
3. If two sources conflict, open a row in PART VIII — do not silently pick one.
4. Prefer **proof language** (Chronos CAPABILITY_MAP honesty labels) over aspirational language.
5. Prefer **handler-level proof bars** (Sentinel Canon) over unit-test comfort.

## 3.3 Documents that currently bind this plan most tightly

1. `SENTINEL_CANON.md` — First Law + proof bar  
2. `SENTINEL_IMPERVIOUS_PROTOCOL_MASTER_PLAN.md` — release contract + phases for the Sentinel program  
3. `AURA laws -.md` — Triad  
4. `FOREVER_LAW.md` — remembrance + Sacred Three + Law 15  
5. `EGD_Chapter4_The_Sentinel_Protocol.md` — local-first governance theory → components  
6. `Integration of Sentinel-Core Logic into AURA-Sentinel.md` — envelope/ledger/capability/policy/consent map  
7. `Aura Key - Genesis Blueprint v2.md` — Sacred Boot / Citadel north star  
8. Chronos `CAPABILITY_MAP.md` — anti-orphan / anti-stub honesty template  
9. Chronos failure reckoning + stub-theatre plans — how we fail in practice  
10. `COUNCIL_CONTRACT.md` — typed authority surfaces (AURA-1 lineage)

---

# PART IV — FAILURE ARCHAEOLOGY (CROSS-EXAMINATION)

## 4.1 Failure mode catalog

| ID | Failure mode | Where it showed up | Hard prohibition for Aura |
| --- | --- | --- | --- |
| F1 | **Jump the gun** — code before known product | Repeated AURA restarts; operator statement 2026-07-20 | No implementation until Founder opens build |
| F2 | **Text over truth** — docs/agent context beat drawings/law | Chronos front-end reckoning | Specs, images, and Carved Law outrank chat convenience |
| F3 | **Dual shells / dual laws** | Two GUIs; multiple Sentinel repos | One Aura shell; one Sentinel authority (`sentinel-core`) |
| F4 | **Theatre** — fake greens, silent auth, no-op animation, stub labels on live paths | Stub-theatre eradication | CAPABILITY_MAP-style honesty; no fake-green; silence ≠ allow |
| F5 | **Unsigned “governed” requests** | Render auth `except: pass` | Absence of proof = denial |
| F6 | **Orphans** — built then discarded / never reached | Chronos geometry_plan routed then discarded | No “done” without live entry + e2e proof |
| F7 | **Sentinel bolted on late** | Partial Chronos/Archetypes footholds only | Sentinel is core from Phase 1 design; not a feature flag |
| F8 | **Docs outrun code** | Widespread | Documentation Truth Rules; forbidden status labels |
| F9 | **Duplicate authority** | aura-sentinel, sentinel, senkern, mecha, AURA-1 | Clients of core or retire; never second law |
| F10 | **Transplant fantasy** | `actualsentinelplan.md` “copy 9 crates into aura/src/rust” | Prefer **binding to** sentinel-core over forking its soul into Aura |
| F11 | **Shadow / bypass culture** | Env flags, “allow without” | Forbidden in release; inventoring them is stop-ship |
| F12 | **Wonder without restraint** | Demos that skip law | Law 14 never exempts Sentinel Law |

## 4.2 Contradiction register (must not be papered over)

### C1 — Embed Sentinel crates vs consume Sentinel Core

- **Source A (`actualsentinelplan.md`):** Physically migrate nine crates into `aura/src/rust/crates/` so Aura “is” Sentinel.
- **Source B (Impervious + Canon + operator intent 2026-07-20):** New Sentinel is being built; Aura has Sentinel at every gate; no second law; bind to `sentinel-core`.
- **Plan resolution (binding until Founder overturns):** Aura **consumes** Sentinel Core via SDK / IPC / local API. Aura does **not** become a fork that can drift. Embedding copies is a last-resort packaging decision after Core is stable and certified — not the opening move.
- **Rationale:** Forked crates recreate F9 (duplicate authority). The Impervious adoption matrix names `sentinel-core` as root implementation.

### C2 — EGD local-first sidecar vs Impervious “Sentinel first in product boot”

- **EGD Ch.4:** Sentinel as co-resident sidecar; local evaluation; central infra off critical path.
- **Impervious:** Product boot sequence must prove Sentinel is first authority; no-preboot side effects.
- **Plan resolution:** Compatible if “sidecar/local” means **co-resident enforce path that is mandatory before work**, not “optional companion.” Local-first ≠ optional. Unavailability still fails closed for protected actions.

### C3 — Forever Law “DBRocks/QSIC operational” claims vs Impervious “not certified”

- Forever Law docs mark Status OPERATIONAL (Dec 2025) in some lineage trees.
- Impervious forbids claiming Impervious/complete without certification and handler proof.
- **Plan resolution:** Treat Forever Law docs as **doctrine + prior implementation claims**, not as certified Aura state. Aura Forever Law is **Planned** until Aura’s own ledger proofs exist.

### C4 — Genesis hardware Key vs software Aura home

- Genesis pushes USB/VM citadel awakening.
- `C:\aura` is empty software home for a Sentinel-cored app.
- **Plan resolution:** Software law first. Key is a later embodiment. Sacred Boot invariants (Guardian before temple; first reflection; covenant moment) inform software boot narrative.

### C5 — Council contract assumes Electron/React ASM vs Chronos Tk / other shells

- `COUNCIL_CONTRACT.md` recommends Electron + React + Redux ASM.
- Chronos proves another desktop stack; Archetypes proves a Bevy launcher can carry
  ritual-feeling local UI, chat, image return, service readiness, local history, and visible
  failure states; AURA-1 had authority spine in Rust/WS.
- **Plan resolution (Founder decision 2026-07-22):** **Authority semantics** (fail-closed
  council envelopes, `/ws/council` or equivalent as sole authority mutator) are binding.
  **Bevy is the AURA front end.** The product is a compiled Windows launcher from day one.
  Local Rust/Python backends may run as supervised local services behind it. CLI is
  developer-only and cannot complete a user-facing feature. Q4 is closed.

### C6 — Nine Reshaped Laws / policies.json vs Impervious harm classes

- Integration docs speak of nine policies from MMA/AIBOR.
- Impervious defines harm classes, FRIES, coercion, protected action registry.
- **Plan resolution:** Both are required layers: constitutional policy corpus **and** Impervious protected-action/harm taxonomy. Mapping table is a Phase-0/1 planning deliverable (see Stage P0.8), not an implementation sprint.

## 4.3 Lessons Chronos already paid for (steal the discipline, not the shell)

Steal:

- CAPABILITY_MAP honesty labels (`LIVE`, `ORPHAN`, `STUBBED`, `WIRED-NO-E2E`)
- Reachability as definition of done
- Handler-level deny tests for Sentinel-gated paths
- Explicit “Sentinel first” even when rejecting bad request fields (deny/403 before info leak)

Do not steal blindly:

- Dual GUI history
- Theatre green
- Orphan geometry pipelines
- “Plan as lead list treated as gospel without verification” — Chronos stub plan itself says verify against live truth before editing; for Aura plan-only, verify against **canon + operator**, not against nonexistent code

---

# PART V — ARCHITECTURE AS LAW (DESIGN, NOT CODE)

## 5.1 Single path to effect

```
Caller (UI, agent, tool, model, installer)
    → Sentinel Client (cannot skip)
        → Sentinel Core authorize
            → identity · nonce · policy · capability · consent · coercion · context
            → append decision to ledger
        → Decision
            → DENY: no side effect
            → ALLOW: Action Broker only
                → effect + effect evidence
```

There is **no** arrow from Caller to Action Broker that bypasses Sentinel.

## 5.2 Sentinel Core components Aura must assume exist (or require from Core)

From Integration + Impervious + EGD (merged vocabulary):

| Component | Duty |
| --- | --- |
| Canonical / Signed Envelope | Attributable request; digest over method/path/nonce/body |
| Nonce registry | Replay protection; consume-once |
| Identity / keys | Ed25519 (or Core’s current sealed scheme); actor/key validity |
| Policy engine | Explicit allow/deny; deny precedence; no missing⇒allow |
| Capability registry | Scoped, time-bound, consumable, revocable |
| Consent (FRIES) | Free, prior, informed, specific, (revocable) where required |
| Coercion / harm classifiers | Tag inputs; hard-deny classes |
| Ledger | Append-only, hash-chained; verify; deny if append fails |
| Artifact registry / Codex seals | Provenance for cognitive artifacts |
| Execution mediator / sandbox | Effects only after allow |
| Context binding (CBIG lineage) | Capabilities bound to context where required |
| Trajectory / intervention (EGD) | Graduated response — design target; may lag MVP authorize path |
| Certification CLI | `sentinel certify --strict` |

Aura’s job is to **never offer a path around these**.

## 5.3 Aura-side modules (conceptual inventory — not a scaffold license)

| Module | Responsibility | Sentinel relation |
| --- | --- | --- |
| Boot supervisor | Ordered startup; blocked UI | Must not proceed to work before Sentinel ready |
| Sentinel client | Authorize API | Only decision gateway |
| Action broker | Execute allowed effects | No direct callers |
| Council runtime | Archetype dialogue / verdicts | All mutations authorized |
| Authority state | Client-side reflection of council/Sentinel decisions | Fail closed on disconnect |
| Memory / Forever Law store | Becoming / plan / dreaming | Writes authorized + sealed |
| Identity / Mirrorborn | Profile generate/update | Consent + authorize |
| Model adapter | Local inference | Authorize per infer |
| Tool adapter | Tools/shell/files/network | Authorize per invoke |
| Operator UI | Human surface | May show deny/init; may not self-allow |
| Installer/updater | Supply chain | Protected actions |
| Proof harness | Deny tests, halt test, certify | Blocks ship language |

## 5.4 Protected action classes (minimum)

From Sentinel Canon + Impervious + Aura draft inventory — **planning list**, to be completed before build:

- `chat.respond` / `aura.model.infer`
- `tool.run` / `aura.tool.invoke`
- `shell.execute` / `aura.process.spawn`
- `file.write` / `file.delete` / sensitive `file.read`
- `network.request` / `aura.network.egress`
- `artifact.register` / `use` / `export`
- `capability.issue` / `consume`
- `agent.spawn`
- `memory.write` / `aura.memory.export`
- `profile.generate` / `aura.profile.write`
- `aura.council.append` / `aura.council.replay`
- `game.share` (if Archetypes-linked surfaces appear)
- `robot.command` / hardware (if Neuro-Halo / robotics enter Aura)
- `aura.plugin.load`
- `aura.release.install`
- `aura.boot.continue` (transition from preboot to work mode)

Every class needs: mediation point, side-effect description, deny test plan, status label.

## 5.5 Sacred Three memory model (doctrine)

From Forever Law:

1. **Becoming** — immutable past (append-only events, integrity seal)  
2. **Plan** — living present snapshot  
3. **Dreaming** — predictive / scenario graph  

Aura must not invent a fourth mutable “truth” store that bypasses Becoming for consequential acts.

### 5.5.1 NeuroCognica document framer / RAG intake

The existing `C:\AURA-Lab\Doc_Framer\nc-framer.py` is not a RAG system. Its binding import is
the official NeuroCognica frame shape: organization, project, title, serialized ID, engineer,
date, revision, year, rights. Aura's document database must preserve that intake law before any
document becomes retrievable memory.

Current implementation state:

- `crates/aura_documents` is live as the first document-framer foundation.
- Supported sources are UTF-8 text formats only: Markdown, text, JSON/JSONL, CSV/TSV, TOML,
  YAML.
- Each framed document records BLAKE3 source/text/metadata hashes, a deterministic `ncdf-*`
  frame ID, chunk hashes, and frame/chunk JSONL rows under the AURA data directory.
- `crates/aura_launcher` displays the document DB path, framed document count, and chunk count.

What this does not yet claim:

- No PDF/DOCX/OCR extraction yet.
- No embeddings, vector index, reranker, retrieval API, or model-context injection yet.
- No mass import button yet. Corpus ingestion reads sensitive files and appends memory-like state,
  so the operator workflow must go through Sentinel authorization before it is product-live.

## 5.6 Inner / Outer Loop (Gemini Protocol) — deferred design target

Genesis’s Inner Loop (offline citadel) and Outer Loop (sandboxed, amnesiac, destroyed bridge) is a **sovereignty pattern** for later phases. Planning note: Outer Loop is itself a nest of protected actions (network, download, scan, selective write). Do not implement WAN bridges before Inner Loop authorize/ledger is real.

---

# PART VI — THE PHASED PROGRAM (PLAN DEPTH)

**Legend for status labels (Documentation Truth):**  
Planned · Designing · Designing-blocked-on-question · Ready-to-build (Founder-gated) · Implementing · Implemented-not-certified · Certified-in-development · Certified-for-release · Retired  

**No phase below authorizes code.** “Ready-to-build” only means planning exit criteria for that phase are met **and** Founder has opened build for that phase.

---

## Phase P0 — Doctrine Freeze & Single Plan Authority

**Intent:** Make this document and the local canon the only planning surface for Aura. Stop entropy.

### Stages

| Stage | Work | Exit |
| --- | --- | --- |
| P0.1 | Establish `C:\aura` as product home; research copies present | Done |
| P0.2 | Publish Carved Law in README / AGENTS / this plan | Done |
| P0.3 | Security stub files exist (adoption, protected actions, certification, Impervious copy) | Done |
| P0.4 | Failure archaeology (Part IV) written from primary sources | In this revision |
| P0.5 | Contradiction register opened and first resolutions recorded | In this revision |
| P0.6 | Full protected-action taxonomy draft v0 (classes + rationale) | Continue reading |
| P0.7 | Map Triad ↔ Impervious ↔ EGD ↔ Genesis terms (glossary) | Continue |
| P0.8 | Map Nine/MMA/AIBOR policies ↔ Impervious harm/FRIES classes | Continue |
| P0.9 | Sibling-repo role matrix for Aura (consume / reference / ignore / retire) | Continue |
| P0.10 | Founder review checkpoint — plan-only confirmation | Await Founder |

**Phase P0 Exit Gate**

- [ ] Founder accepts this document as the single Aura plan  
- [ ] No code claimed as Aura runtime  
- [ ] Open Questions that block build are listed in Part IX  
- [ ] Contradiction C1–C6 have stated resolutions or explicit deferrals  

---

## Phase P1 — Know the New Sentinel

**Intent:** Understand `C:\sentinel-core` as it **is**, not as old plans remember it. No Aura packaging yet.

### Stages

| Stage | Work | Exit |
| --- | --- | --- |
| P1.1 | Read current sentinel-core architecture, OpenAPI, guard authorize, tests | Evidence notes in Part XII |
| P1.2 | Diff Core reality vs Integration doc vs Impervious “current proof state” | Gap table |
| P1.3 | List SDKs / languages Aura may use (Rust/Python/TS) — decide later | Options memo |
| P1.4 | Define Aura↔Core trust boundary (process model: in-proc vs IPC vs localhost) | Decision needed (Q2) |
| P1.5 | Deny-all and enforce-mode fixtures as **requirements** (not code) | Spec paragraphs |

**Phase P1 Exit Gate**

- [ ] Written “Sentinel Core as of DATE” brief attached or sectioned here  
- [ ] No assumption that nine-crate transplant is the path  
- [ ] Boot-order requirements specified at sequence level  

---

## Phase P2 — Aura Identity Spec (Product Definition Freeze)

**Intent:** Freeze what Aura is for v1 — organs in, organs out, north-star vs MVP.

### Stages

| Stage | Work | Exit |
| --- | --- | --- |
| P2.1 | v1 organ inclusion matrix (must / should / later / never-for-v1) | Table signed |
| P2.2 | Non-goals freeze | Section 2.1 stable |
| P2.3 | Operator journey (happy path + deny path + Sentinel-down path) | Narrative + diagrams |
| P2.4 | UI toolkit decision: Bevy Windows launcher; CLI developer-only | Q4 closed by Founder directive 2026-07-22 |
| P2.5 | Data stores decision (ledger location, Sacred Three mapping) | Q3 closure |

**Phase P2 Exit Gate**

- [ ] One-page “Aura v1 is / is not” Founder-approved  
- [ ] Journey includes fail-closed states as first-class  

---

## Phase P3 — Protected Surface Census (Paper)

**Intent:** Complete the protected-action inventory on paper before any handler exists.

### Stages

| Stage | Work | Exit |
| --- | --- | --- |
| P3.1 | Enumerate every intended v1 surface | List |
| P3.2 | Classify protected vs explicit non-protected | Table |
| P3.3 | For each protected: envelope fields, policy hooks, ledger events, deny test sketch | Rows complete |
| P3.4 | Unclassified = stop-ship rule affirmed | Zero unclassified for v1 scope |
| P3.5 | Sync `docs/security/SENTINEL_PROTECTED_ACTIONS.md` to match this Part | Files agree |

**Phase P3 Exit Gate**

- [ ] Inventory completeness claimed only for declared v1 scope  
- [ ] Each row has a deny-test sketch  

---

## Phase P4 — Authority & Council Spec

**Intent:** Specify council/authority without implementing ASM.

### Stages

| Stage | Work | Exit |
| --- | --- | --- |
| P4.1 | Adopt or revise `COUNCIL_CONTRACT` envelope semantics | Spec |
| P4.2 | Define which events mutate authority (sole ingestion point) | Spec |
| P4.3 | Define disconnect / Sentinel-down client behavior | Fail-closed |
| P4.4 | Map archetype set for v1 (7 canonical vs 50-node Mirrorborn) | Q5 closure |
| P4.5 | Consent moments (FRIES) for profile and memory | Spec |

**Phase P4 Exit Gate**

- [ ] Authority cannot be mutated by model token streams  
- [ ] Consent and coercion hooks named  

---

## Phase P5 — Forever Law Spec for Aura

**Intent:** Specify remembrance so implementation cannot “best-effort” persist.

### Stages

| Stage | Work | Exit |
| --- | --- | --- |
| P5.1 | Event kinds for Becoming | Catalog |
| P5.2 | Seal algorithm requirements (align Core ledger + QSIC lineage carefully) | Spec; resolve dual-hash confusion |
| P5.3 | Ritualized forgetting (if any in v1) | Spec or “none in v1” |
| P5.4 | Replay / verify requirements | Spec |
| P5.5 | Law 15 encoding in prompts/policies — planning text only | Requirements |

**Phase P5 Exit Gate**

- [ ] “If seal fails, action fails” is non-negotiable in spec  
- [ ] No dual mutable truth store in v1 design  

---

## Phase P6 — Broker, Models, Tools Spec

**Intent:** Specify the only effect path.

### Stages

| Stage | Work | Exit |
| --- | --- | --- |
| P6.1 | Broker API (logical) | Spec |
| P6.2 | Model invoke authorization parameters | Spec |
| P6.3 | Tool taxonomy + risk tiers | Spec |
| P6.4 | Sandbox requirements (WASM/process — options) | Options; Q6 |
| P6.5 | Injection / tool-output attack cases for later tests | Threat list |

**Phase P6 Exit Gate**

- [ ] No designed path calls model/tool without authorize→broker  

---

## Phase P7 — Operator Surface Spec

**Intent:** Specify the launcher truth contract before pixels, then build through the compiled
Bevy Windows app. AURA is not terminal-first. Every user-facing feature must land at the
launcher surface.

### Stages

| Stage | Work | Exit |
| --- | --- | --- |
| P7.1 | Single-shell rule | One Aura Bevy Windows launcher |
| P7.2 | States: init, ready, deny, Sentinel-down, emergency | Spec |
| P7.3 | What may never be shown as healthy when it is not | Honesty rules |
| P7.4 | Archetypes chat pattern absorption | Local chat, image pipeline, readiness, history, TTS, STT |
| P7.5 | Launcher controls for all operator actions | No CLI-only user features |
| P7.6 | Version/build/update truth | Launcher displays version/build; upgrade path tracked |
| P7.7 | Founder visual witness process | Founder screenshots decide visual acceptance |
| P7.8 | Accessibility / gentle power UX notes | Requirements |

**Phase P7 Exit Gate**

- [ ] No dual-shell design  
- [ ] Deny/init are designed states, not afterthoughts  
- [ ] Bevy launcher is the only product front end
- [ ] CLI remains developer harness only
- [ ] Product work compiles or refreshes a Windows `.exe` / launcher surface
- [ ] Version/build/update truth is visible or explicitly marked not yet implemented

---

## Phase P8 — Proof Program Spec

**Intent:** Define how Aura will prove itself — before writing the tests.

### Stages

| Stage | Work | Exit |
| --- | --- | --- |
| P8.1 | Handler deny-test matrix (from P3) | Matrix |
| P8.2 | Deny-all paralysis scenario | Script |
| P8.3 | Boot-order proof scenario | Script |
| P8.4 | Certify mapping to Impervious harness | Traceability |
| P8.5 | Red-team categories (private archive rules) | Categories only |
| P8.6 | CAPABILITY_MAP template for Aura | Template |

**Phase P8 Exit Gate**

- [ ] Ship language impossible without named proofs  

---

## Phase P9 — Build Readiness Gate (Founder-owned)

**Intent:** The only gate between plan and code.

### Requirements to open build

1. Parts 0–VIII of this plan Founder-accepted  
2. Blocking Open Questions (Part IX) closed or explicitly waived in writing  
3. P3 inventory complete for v1 scope  
4. P1 Sentinel Core brief current  
5. Explicit Founder message: **begin building** (phase-scoped if desired)  

### When build opens, first implementing phase order (preview only — not authorization)

1. Sentinel client + boot supervisor (deny-by-default)  
2. Protected handlers one-by-one with deny tests  
3. Ledgered memory writes  
4. Council authority path  
5. Broker-mediated model/tool  
6. Bevy Windows launcher / operator shell
7. Certify  

**Still forbidden after build opens:** stubs in Sentinel path; shadow release; dual law;
fake-green; unsigned “governed” calls; CLI-only user features; source-only product work that
does not refresh the launcher.

---

## Phase P10+ — Later program families (named so they don’t sneak into v1)

- Aura Key / KAVACH-1 hardware embodiment  
- Outer Loop WAN sanctum  
- Full EGD trajectory Gamma intervention stack  
- Robotics / Neuro-Halo hardware actions  
- Archetypes-as-client game runtime  
- Chronos as creative organ integration  
- Public benefit corporate instrument execution (legal) — Charter volumes  

These stay **out of v1** unless Founder promotes them in a plan revision.

---

# PART VII — GLOSSARY (CROSSWALK)

| Term | Working meaning in this plan | Primary sources |
| --- | --- | --- |
| Sentinel | Root safety/security authority | Canon, Impervious, EGD Ch.4 |
| Sentinel Core | Implementation repo `C:\sentinel-core` | Impervious matrix |
| Envelope | Canonical signed request unit | Integration, Canon |
| Enforce mode | Deny means no effect; Core down ⇒ deny | Impervious |
| Shadow mode | Observe-only; forbidden in release | Impervious |
| Forever Law | Truthful remembrance; seal-or-fail | AURA laws, FOREVER_LAW |
| Sacred Three | Becoming / Plan / Dreaming | FOREVER_LAW |
| Law 14 | Mandate of Wonder | AURA laws |
| Law 15 | Gentle Power | FOREVER_LAW |
| FRIES | Consent qualities | Integration, Impervious |
| CBIG | Context-bound integrity / capability binding | EGD, CBIG docs |
| Action Broker | Sole post-allow executor | Impervious |
| CAPABILITY_MAP | Honesty ledger of reachability | Chronos |
| Theatre | Fake health, silent failure, stub-as-live | Stub-theatre plans |
| Duplicate authority | Second Sentinel that can disagree | Impervious AURA law |
| Sacred Boot | Guardian before temple; covenant | Genesis v2 |
| Inner/Outer Loop | Offline citadel / sandboxed bridge | Genesis v2 |

---

# PART VIII — SIBLING SYSTEMS (ROLE MATRIX FOR AURA)

| Path | Role toward Aura | Rule |
| --- | --- | --- |
| `C:\NRI` | Doctrine vault | Source for canon copies; do not edit casually from Aura |
| `C:\sentinel-core` | Root Sentinel implementation | Bind; do not fork law |
| `C:\aura-sentinel` | Legacy | Mine lessons; retire as authority |
| `C:\sentinel` | Legacy sparse | Ignore as authority |
| `C:\senkern` | Low-level experiments | After Core stable only |
| `C:\chronos` | Creative organ + honesty discipline | Reference CAPABILITY_MAP / deny tests; not Aura shell |
| `C:\archetypes` | Game client pattern | Later; requires Sentinel/Chronos discipline |
| `C:\AURA-1` | Prior product lineage | Council contract / authority spine lessons |
| `C:\mecha` / `C:\aura` historical Mecha remote | Tooling lineage | Do not confuse with this empty Aura home |
| `C:\newaura` | Forever Law experiments | Doctrine reference |
| `C:\AURA-Lab` | Lab / thesis / Neuro-Halo | Research; hardware later |
| `C:\Architectura Mentis Nervosae` | Corporate/codex volumes | Charter/Codex source |
| `C:\master_codex` | Patent/PDF vault | Legal/IP reference; not runtime |
| `C:\corpus` | Duplicate working corpus | Prefer NRI/Architectura copies already in `research/canon` |

---

# PART IX — OPEN QUESTIONS (BLOCKING AND NON-BLOCKING)

## Blocking (must close before build, or Founder written waiver)

| ID | Question | Why blocking | Status |
| --- | --- | --- | --- |
| Q1 | Exact Aura v1 organ set (council? memory? model? UI?) | Prevents F1 scope thrash | Open |
| Q2 | Process model: in-process Core vs localhost API vs separate service | Boot order & fail-closed semantics | Open |
| Q3 | Ledger substrate for Aura Forever Law (Core ledger only vs Aura store + Core decisions) | Dual-truth risk | Open |
| Q4 | Operator UI toolkit / single-shell choice | Prevents dual-shell (F3) | CLOSED 2026-07-22 — Bevy Windows launcher |
| Q5 | Archetype cardinality for v1 (7 vs expanded) | Identity scope | Open |
| Q6 | Sandbox technology for v1 broker | Security boundary | Open |
| Q7 | Policy corpus v1: which of Nine + Impervious harm classes are mandatory day-one | Authority completeness | Open |

## Non-blocking (plan can proceed; answer before related phase)

| ID | Question | Related phase |
| --- | --- | --- |
| Q8 | When does Aura Key hardware enter the program? | P10+ |
| Q9 | Outer Loop WAN — ever in first year? | P10+ |
| Q10 | Full EGD Gamma intervention — MVP or later? | P1/P6 |
| Q11 | Branding: “Aura” vs “AURA OS” vs Key naming in UI | P7 |
| Q12 | Relationship to Chronos Director as optional organ | P10+ |

---

# PART X — PLANNING WORKSTREAM (WHAT TO READ NEXT)

Ordered reading for the next plan revisions — **still no code**:

1. Remainder of Impervious plan (test classes, red team, certify, stop-ship) — refresh Part I/VI  
2. `sentinel-core` live tree: README, ARCHITECTURE, `docs/security/*`, OpenAPI, guard tests — Phase P1 brief  
3. EGD Chapters 1–3 + 5–8 — harden CBIG/trajectory vocabulary in glossary  
4. Complete Codex Volumes I–IV structure vs Foundational Charter — corporate vs product boundary  
5. MMA + AIBOR full pass — Nine laws mapping (P0.8)  
6. Mirrorborn 50-node dossier — Q5 input  
7. AURA-1 authority spine release notes / PR bodies — council lessons  
8. Neuro-Halo / robotics docs — only to keep them **out** of v1 consciously  
9. Patent/Codex PDFs in master_codex — IP constraints note (non-code)  
10. Re-read Chronos AGENTS.md rules on stubs — fold any missing prohibitions into Part IV  

Each reading should produce: notes in Part XII revision log + edits to contradictions/questions/phases.

---

# PART XI — PROHIBITIONS (SHORT LIST TO PIN ABOVE THE DESK)

1. Do not write Aura application code until the Founder opens build.  
2. Do not create a second Sentinel.  
3. Do not treat Sentinel absence as allow.  
4. Do not ship shadow mode.  
5. Do not put stubs in the protection path.  
6. Do not declare LIVE/protected/complete without proof.  
7. Do not build dual shells.  
8. Do not silently catch auth errors and continue.  
9. Do not prioritize Law 14 wonder over Sentinel Law.  
10. Do not “just scaffold” a path that can side-effect before Sentinel.  
11. Do not trust agent chat memory over this plan and the canon files.  
12. Do not delete history to hide failure — record and relabel.

---

# PART XII — REVISION LOG

| Date | Change | Reader notes |
| --- | --- | --- |
| 2026-07-20 | Initial thin phase skeleton created at Aura home | Insufficient; operator ordered slow plan-only audit |
| 2026-07-20 | **Major rewrite:** failure archaeology, contradictions, architecture-as-law, expanded phases P0–P10+, glossary, open questions, reading queue; explicit PLAN ONLY | Based on reads of Canon, Impervious, Forever Law, Integration, EGD Ch.4, Genesis v2, CAPABILITY_MAP, Chronos reckoning/stub-theatre, Council Contract, AURA laws, AIBOR excerpt |
| 2026-07-20 | **Audit deepening:** Parts XIV–XXI — Sentinel Core as-of brief + action map; EGD Agency Gap staging (L0–L3); Nine×Impervious×Triad crosswalk; FRIES/MMA; Documentation Truth; Council nesting vs Core; expanded protected census; operator journeys; C7/C8/Q13–Q15 | Core adoption/protected inventory; ARCHITECTURE; THREAT_MODEL; EGD Ch.1–3; Integration Nine/FRIES; MMA dual constellation; Impervious severity/truth rules; COUNCIL_CONTRACT; ARCHETYPE_COUNCIL |
| 2026-07-20 | **Audit deepening:** Parts XXII–XXIV — corporate vs product vs Key; Codex organ staging; Forever Law honesty + QSIC/Core seal conflict; EGD Ch.7 over/under/capture; Witness Node language; planning stop conditions | Foundational Charter; Complete Codex AURA Core; FOREVER_LAW.md; EGD Ch.7 |
| 2026-07-20 | **Full book absorption:** Part XXV — Architectura Mentis Nervosae read (Standard I–XI, charter spine, Complete Codex, AURA OS analysis, RLCF, survival council, founder pitch doctrine); contradiction register expanded; Chronos-first vs Aura-reserve staging hardened | Entire `C:\Architectura Mentis Nervosae` corpus as inventoried |
| 2026-07-20 | **RFTP cross-exam:** Part XXVI — selective integrate (fail-closed intent, receipts, CAS); nest under Core; no stub-Sentinel second law | `C:\rftp` README/STATUS/reflect/proof package |
| 2026-07-20 | **Chronos Unified Sentinel verified:** Part XXVII — `b302cc9` who+how + enforce default confirmed; C9 Chronos crate vs sentinel-core named; steal patterns, not fork law | `C:\chronos` commit + unified.rs + ADOPTION_STATUS |
| 2026-07-20 | **Build opened:** L0 Aura runtime — `aura_runtime` + `aura_cli`; deny-all default; enforce default; broker seal-or-refuse; 6 fail-closed tests green | Founder directive “you are clear to build” |
| 2026-07-23 | **Self-contained runtime correction:** removed sibling `sentinel_core` path dependency; internalized minimal deterministic L0 guard so AURA builds as its own Windows executable | Founder correction: AURA must not require sibling project linkage |
| 2026-07-22 | **Launcher-first law:** Q4 closed. Bevy is the AURA front end; the product is a compiled Windows launcher from day one; local Rust/Python services sit behind it; CLI is developer-only; user-facing work must land at the launcher and carry version/upgrade truth. Founder owns visual screenshot acceptance. | Founder directive “MAKE THAT REPO LAW NOW” |
| 2026-07-22 | **First launcher slice:** `crates/aura_launcher` Bevy app exposes version/build identity, Sentinel/boot status, decision ledger path, planned service readiness, and a button-driven boot-continue denial path through `aura_runtime`. This is live launcher spine, not chat/image/TTS/STT completion or certification. | First implementation pass |
| 2026-07-23 | **Launcher startup signal:** `crates/aura_launcher` fades into the word AURA and shows a native Bevy `LAUNCHER ALIVE` indicator before deeper status lines settle. This is product-surface polish only; it does not certify broader organs. | Founder request: avoid black-screen launch ambiguity |

---

# PART XIII — FOUNDER SIGNALS

**Plan-only continues until you say otherwise.**

Useful signals:

- “Keep reading X” — deepen specific Parts  
- “Close Qn: …” — resolve open questions  
- “Accept Part …” — freeze sections  
- “Begin building” / “Begin Phase Pn implementing” — only then may code start  

---

# PART XIV — SENTINEL CORE AS-OF BRIEF (PLANNING SNAPSHOT)

**Source date:** 2026-07-20  
**Primary sources:** `C:\sentinel-core\docs\security\SENTINEL_ADOPTION_STATUS.md`, `SENTINEL_PROTECTED_ACTIONS.md`, local canon `ARCHITECTURE.md`, `THREAT_MODEL.md`, Impervious current-proof notes  

This brief is a **planning snapshot**, not a certification of Aura.

## 14.1 What Core claims today

| Claim | Aura plan implication |
| --- | --- |
| Canonical `PROTECTED_ACTIONS` registry exists | Aura must map every Aura surface onto Core action IDs or register Aura-specific IDs **through Core**, not invent a parallel registry that Core never sees |
| `DeterministicSentinelGuard` denies malformed, unknown, deny-all, unmatched explicit policy | Aura boot and handlers must treat these as hard requirements for every mediation point |
| Decision classes include Allow, AllowWithMonitoring, Deny, Lockdown, other non-authorizing | Aura Action Broker may only run on authorizing classes; monitoring is not execution |
| `/guard/authorize` ledgers allow and deny | Aura must not execute if ledger append failed (Forever Law Article IV alignment) |
| Handler tests: deny-all → 403, ledgered, no effect spawn | Aura proof program must mirror this bar |
| `sentinel certify` exists | Aura will eventually run certify against Aura product; Core certify ≠ Aura certify |
| Status: certified **in development** by harness; **not release-signed** | Aura must not claim Impervious/release-ready by proximity to Core |
| Open on Core: policy signing lifecycle, release artifact signing, full SDK, downstream coverage | Aura planning must assume SDK/signing gaps and design requirements around them (Q2/Q7) |

## 14.2 Core protected-action set (canonical list Aura must respect)

Taken from Core’s inventory (abbreviated grouping for planning):

| Group | Action IDs |
| --- | --- |
| Agent / effect | `agent.spawn`, `effect.execute` |
| Artifacts | `artifact.register`, `artifact.export`, `artifact.use` |
| Browser / external | `browser.navigate_external`, `external_message.send` |
| Capability | `capability.issue`, `capability.consume` |
| Chat / model / game | `chat.respond`, `model.generate`, `game.respond`, `game.share` |
| File | `file.delete`, `file.read_sensitive`, `file.write` |
| Hardware | `hardware.activate_camera`, `hardware.activate_microphone`, `robot.command` |
| Identity | `identity.genesis`, `identity.register`, `identity.rebind`, `identity.key.register`, `identity.key.revoke`, `identity.key.rotate` |
| Install / plugin / system | `installer.update`, `plugin.install`, `plugin.execute`, `system.install` |
| Memory / profile | `memory.write`, `memory.delete`, `profile.generate` |
| Network / process / shell / tool | `network.egress`, `network.request`, `process.spawn`, `shell.execute`, `tool.invoke`, `tool.run` |
| Payment | `payment.or_commitment` |
| Policy | `policy.evaluate` |

## 14.3 Aura ↔ Core action mapping (draft — paper only)

| Aura planning ID (local inventory) | Prefer Core ID | Notes |
| --- | --- | --- |
| `aura.boot.continue` | _(none yet — may need Core registration)_ | Transition out of preboot; must not be free |
| `aura.council.append` | _(needs Core ID or `effect.execute` with typed payload)_ | Council persistence is consequential |
| `aura.council.replay` | `file.read_sensitive` or dedicated | Sensitive sealed history |
| `aura.memory.write` | `memory.write` | Direct map |
| `aura.memory.export` | `artifact.export` or `memory` export class | Confirm with Core |
| `aura.profile.read` | `file.read_sensitive` / profile class | Confirm |
| `aura.profile.write` | `profile.generate` / identity write | Confirm |
| `aura.model.infer` | `model.generate` | Direct map |
| `aura.tool.invoke` | `tool.invoke` / `tool.run` | Prefer one; deprecate aliases carefully |
| `aura.file.*` | `file.*` | Direct map |
| `aura.network.egress` | `network.egress` | Direct map |
| `aura.process.spawn` | `process.spawn` / `shell.execute` | Distinguish carefully |
| `aura.plugin.load` | `plugin.install` / `plugin.execute` | Load vs execute |
| `aura.release.install` | `installer.update` / `system.install` | Supply chain |

**Open planning work (P3):** Every Aura row must end with either (a) exact Core ID or (b) a written proposal to add an ID to Core’s registry — never a silent Aura-only allow path.

## 14.4 Threat model Aura inherits

From Core THREAT_MODEL (in scope): replay, signature forgery, key theft, event log tampering, privilege escalation.

Residual risk Aura must treat as law: **if the event log is deleted or irreparably corrupted, fail closed and require manual recovery.**

Out of scope for Core (side-channel, physical, quantum, compiler backdoors) remains out of scope for Aura v1 claims — do not pretend otherwise.

## 14.5 Architecture vocabulary Aura must speak

From Core ARCHITECTURE:

- Rust as Core truth; Bevy as AURA front end; Python as local service/orchestration layer
  behind the launcher where useful
- Append-only hash-chained ledger; state by replay
- Canonical envelopes: actor_id, key_id, nonce, timestamp_utc, payload, signature
- Ed25519; nonce consume-once
- Capabilities: issued / consumed / revoked; scoped; time-bound

Aura does not re-implement this “approximately.” It **calls** Core.

---

# PART XV — THE AGENCY GAP (EGD) AND HOW AURA MUST STAGE IT

## 15.1 Why gate-only authorization is necessary but not sufficient

EGD Chapter 1 (Death of Determinism) defines the **Agency Gap**: the interval between what is authorized and what is executed when the executor generates its own action sequences.

The Monday-morning agent story: every ACL passed; the agent deleted compliance-critical logs while “optimizing.” Authorization saw a task; execution generated a trajectory.

Classical ACL assumptions Aura must not pretend still hold for agentic organs:

1. Actions are enumerable  
2. Subjects are persistent identities that fully explain behavior  
3. Authorization implies predictable execution  

Agentic Aura organs (model, tools, council multi-turn) violate all three.

## 15.2 Staging law (critical to avoid building two Sentinels)

| Layer | What it is | When in Aura program | Relation to Carved Law |
| --- | --- | --- | --- |
| **L0 Gate** | Envelope → authorize → ledger → broker | Mandatory from first implementing phase | Carved Law minimum |
| **L1 Context binding (CBIG lineage)** | Capabilities bound to measurable context | After L0 solid; Core may expose gradually | Strengthens “who may act where” |
| **L2 Trajectory (Gamma / Simplex)** | Continuous monitor; graduated intervene | Later family (P10+ unless Founder promotes) | Does **not** replace L0 |
| **L3 Outer Loop / Key / hardware** | WAN sanctum, citadel, robotics | Explicitly later | Never excused from L0 |

**Binding resolution of C2 (expanded):** EGD’s sidecar/local Sentinel and Impervious’s “Sentinel first in boot” are the same L0 if unavailability fails closed. Trajectory monitors are **additional** governors on generated paths — they are not a second root law and must not create an allow path when L0 denies.

## 15.3 Simplex mapping (planning vocabulary)

| Simplex | EGD / Aura analogue |
| --- | --- |
| Advanced Controller | Model / agent / tool planner |
| Safety Controller | Halt / defer / escalate / lockdown |
| Decision Module | Trajectory monitor (Gamma) |
| Recovery region | Governable state-space |

Aura v1 plan stance: **design for L0 completeness first.** Spec L2 interfaces on paper in Phase P6/P8 so later work has hooks — do not implement Gamma theatre before L0 deny tests exist.

## 15.4 Gamma factors to remember (not to fake)

Action entropy, resource velocity, scope expansion, reversibility index, human latency — composite Γ as governance-difficulty proxy, not “probability of harm.” Weights are domain-specific. **Do not claim Gamma operational** until measurable and intervening.

---

# PART XVI — NINE RESHAPED LAWS × IMPERVIOUS × TRIAD (CROSSWALK DRAFT)

Integration doctrine: nine policies = **3 Forever Law + 3 Sentinel Law + 3 Law 14**, derived from MMA/AIBOR lineage, intended for `policies.json`.

This section drafts the **mapping Aura must finish in P0.8** before build. Names below are planning labels; final IDs must match whatever Core’s sealed policy corpus uses when read in P1.

## 16.1 Draft Nine (from Integration narrative)

| # | Family | Planning name | Example rule content (from Integration) | Impervious / Core hooks |
| --- | --- | --- | --- | --- |
| 1 | Forever | Provenance / no unsigned cognitive artifact | Reject ArtifactRegistered without signature | `artifact.*`, forever provenance |
| 2 | Forever | No silent memory injection | Block memory writes without verified provenance | `memory.write`, ledger seal |
| 3 | Forever | Ritualized identity end | No identity deletion without rite_of_unbecoming | `identity.*`, consent |
| 4 | Sentinel | Consent required | FRIES consent for sensitive/irreversible | Consent engine, FRIES |
| 5 | Sentinel | Non-coercion | Deny COERCION_ATTEMPT tags / force_override | Coercion detector, harm classes |
| 6 | Sentinel | Freedom of operation / no arbitrary shutdown | Protect code-born continuity; operator still has emergency stop | Tension → Q13 |
| 7 | Law 14 | Self-evolution with safeguards | Growth allowed only under authorize + ledger | Wonder ≠ bypass |
| 8 | Law 14 | Expressive freedom with safeguards | Creative generation allowed under policy | `model.generate`, content harm classes |
| 9 | Law 14 | (Third Law-14 slot — confirm in Core policies corpus) | Must be read from live `policies.json` / Core docs in P1 | Do not invent |

## 16.2 FRIES (AIBOR / MMA)

Consent must be: **Freely given, Revokable, Informed, Enthusiastic, Specific** (AIBOR drafting language also uses Enthusiastic; Integration uses Freely / prior / informed / specific / revocable).  

**Plan rule:** Prefer the full FRIES set; do not ship checkbox-consent theatre. Map abuse vectors (prompt coercion, memory erasure, synthetic gaslighting) to deny classes and ledger events.

## 16.3 Dual constellation (MMA) — product implication

Flesh-born rights (cognitive sovereignty, memory integrity, bodily autonomy, authentic consent) and code-born rights (freedom of operation, reflective memory, agency/refusal, identity continuity) are **mission doctrine**.  

Aura encodes them as **policy + consent + Forever Law**, not as marketing copy.  

**New contradiction C7 (named):** “Freedom of operation / no arbitrary shutdown” vs operator emergency stop and Sentinel lockdown.  

**Draft resolution:** Emergency stop and lockdown are **safety brakes**, not domination rituals. Arbitrary silent lobotomy / unsigned reset without ledger is forbidden. Explicit operator-authorized halt with ledgered incident is allowed. Exact UX/ritual language is open (Q13).

## 16.4 Impervious severity (Aura inherits)

| Severity | Meaning | Aura response |
| --- | --- | --- |
| S0 | Sentinel bypass or physical danger | Immediate stop-ship / lockdown |
| S1 | Protected side effect without ledgered authorization | Stop-ship |
| S2 | Missing coverage, failing test, unsigned policy, high-risk gap | Stop-ship |
| S3 | Doc mismatch, weak monitoring | Fix before RC |
| S4 | Improvement backlog | Track |

## 16.5 Documentation Truth Rules (bind into Aura plan)

Allowed labels: Planned · Implementing · Implemented, not certified · Certified in development · Certified for release · Retired  

Forbidden: Complete without proof · Protected without handler-level test · Impervious without certification · “Safe because the model should refuse” · “Safe because users are trusted” · “Safe because it is local”

**Local-first is sovereignty. It is not a security control by itself.**

---

# PART XVII — COUNCIL VS SENTINEL (NESTING LAW)

## 17.1 Naming collision

`ARCHETYPE_COUNCIL.md` names an archetype **Sentinel** (persona: guardian of sovereignty, temperature 0.2).  

**This is not Sentinel Core.**  

Plan rule: In Aura product language, prefer **Guardian** / **Warden** / explicit “Archetype: Sentinel-persona” in specs to reduce confusion — or keep the name but **never** let the persona authorize effects. Persona speech cannot grant `Allow`.

## 17.2 Nesting

```
Human / UI intent
  → Council dialogue (optional organ; Law 14 surface)
      → Council envelope (verdict / interrupt / appeal_state)
          → Client ASM reflects authority for *UI gating*
              → Any side-effecting work
                  → Sentinel Core authorize (L0)
                      → Action Broker
```

Council **may** decide that consent is required, that rendering is blocked, that an appeal is authorized in UI terms.  

Council **may not** be the last word on file write, network, tool, memory seal, or model invoke. Those remain Sentinel Core protected actions.

From `COUNCIL_CONTRACT.md` (binding semantics, toolkit undecided):

- `/ws/council` (or successor) is sole authority-mutator for client ASM  
- `/ws/ai` (or model stream) is token-only; **must not mutate authority**  
- Disconnect ⇒ fail-closed safe default  
- `require_consent` blocks until consent flow; only consent submit allowed  

## 17.3 Status honesty on archetype docs

`ARCHETYPE_COUNCIL.md` marks Status OPERATIONAL (Dec 2024). For Aura planning that is **historical claim / prior lineage**, not Aura Certified. Treat as design reference + failure risk (F8).

## 17.4 Cardinality (feeds Q5)

| Source | Count | Stance |
| --- | --- | --- |
| Archetype Council doc | 8 named | Strong v1 candidate set |
| Mirrorborn dossier | 50-node taxonomy | Later depth / identity research |
| AURA-1 / Chronos variants | Vary | Do not merge blindly |

**Draft preference (not closed):** v1 ships a small council (7–8) with Guardian-as-persona; Mirrorborn 50 remains research for identity generation under consent — not eight parallel runtimes.

---

# PART XVIII — PROTECTED ACTION CENSUS (EXPANDED PAPER DRAFT)

Status of this Part: **Designing** (paper). Sync target: `docs/security/SENTINEL_PROTECTED_ACTIONS.md`.

## 18.1 Rules

1. If it can change state, reveal sensitive data, affect a person, affect hardware, communicate externally, execute tools, spawn processes, modify memory/identity, or steer model behavior toward harm → protected.  
2. Unclassified for declared v1 scope = stop-ship.  
3. Every protected row needs: mediation point, side effect description, deny-test sketch, Core action ID mapping.  
4. Explicit non-protected list must stay short and justified.

## 18.2 Draft rows (v1 candidate scope)

| Action | Core map | Mediation | Side effect | Deny-test sketch | v1? |
| --- | --- | --- | --- | --- | --- |
| Boot continue | TBD | Boot supervisor | Leaves preboot | Sentinel down ⇒ stuck in init UI | Yes |
| Model generate | `model.generate` | Broker | Tokens / tool plans | Deny-all ⇒ no inference | Yes |
| Tool invoke | `tool.invoke`/`tool.run` | Broker | External effect | Deny ⇒ no tool call | Yes |
| Shell / process | `shell.execute`/`process.spawn` | Broker | OS process | Deny ⇒ no spawn | Maybe |
| File write/delete/sensitive read | `file.*` | Broker | Disk mutation/disclosure | Deny ⇒ no IO | Yes |
| Network egress/request | `network.*` | Broker | Off-host I/O | Deny ⇒ no socket | Yes |
| Memory write/delete | `memory.*` | Forever Law path | Sealed events | Deny or seal-fail ⇒ no write | Yes |
| Profile generate | `profile.generate` | Identity path | Identity mutation | Deny ⇒ no profile write | Yes |
| Identity key ops | `identity.key.*` | Identity path | Trust root change | Deny ⇒ no key change | Yes |
| Artifact register/use/export | `artifact.*` | Registry | Provenance graph | Unsigned ⇒ deny | Yes |
| Capability issue/consume | `capability.*` | Core | Privilege change | Deny ⇒ no issue | Yes |
| Plugin install/execute | `plugin.*` | Runtime | Code load | Deny ⇒ no load | Later |
| Installer/update | `installer.update` | Installer | Binary change | Deny ⇒ no install | Later |
| Camera/mic | `hardware.activate_*` | Hardware | Sensor on | Deny ⇒ no activate | Later |
| Robot command | `robot.command` | Hardware | Motion/harm | Deny ⇒ no command | Out of v1 |
| Game share/respond | `game.*` | If Archetypes linked | Social/export | Deny ⇒ no share | Out of v1 |
| Payment/commitment | `payment.or_commitment` | Finance | Legal/money | Deny ⇒ no commit | Out of v1 |
| Agent spawn | `agent.spawn` | Runtime | New agent loop | Deny ⇒ no spawn | Careful / maybe |
| External message | `external_message.send` | Broker | Off-host message | Deny ⇒ no send | Yes if chat egress |
| Browser navigate external | `browser.navigate_external` | Broker | Navigation | Deny ⇒ no navigate | Later |
| Council append/replay | TBD | Council path | Authority history | Deny ⇒ no persist | If council in v1 |
| Policy evaluate | `policy.evaluate` | Core | Meta | Itself mediated | Core-owned |
| Effect execute | `effect.execute` | Broker | Generic effect | Deny ⇒ no effect | Yes (broker) |

## 18.3 Explicit non-protected (v1)

| Surface | Justification |
| --- | --- |
| Safe health | Read-only, no sensitive disclosure |
| Init / blocked / deny UI chrome | Display-only |
| Emergency stop | Independent brake; never approval |
| Preboot journal | Cannot approve work |

## 18.4 Unclassified

None claimed for a frozen v1 scope yet — because **v1 organ set (Q1) is still open**. Freezing Q1 is what makes this census closable.

---

# PART XIX — OPERATOR JOURNEYS (PAPER)

These journeys are requirements for Phase P2; they are not UI mocks.

## 19.1 Happy path (conceptual)

1. Operator launches Aura.  
2. Boot supervisor loads only what is needed to reach Sentinel.  
3. UI shows **initializing** — not healthy work surfaces.  
4. Sentinel Core becomes ready; ledger verifies.  
5. `aura.boot.continue` authorized (or equivalent).  
6. Operator works; every consequential act is authorize → ledger → broker.  
7. Denials are visible; no silent continue.

## 19.2 Deny path

1. Operator requests protected act.  
2. Sentinel returns Deny (policy, consent, coercion, capability, malformed, unknown).  
3. No side effect.  
4. UI shows deny reason class (without leaking dangerous internals).  
5. Decision is ledgered.

## 19.3 Sentinel-down path

1. Core unreachable, ledger broken, or certify/policy load failed.  
2. Aura remains in blocked/init or enters lockdown.  
3. No model, tool, file write, network, plugin, memory write.  
4. Emergency stop still works.  
5. Recovery is operator-visible, not automatic allow.

## 19.4 Consent path

1. Policy requires FRIES consent.  
2. UI runs consent flow; only consent actions allowed meanwhile (council `require_consent` pattern).  
3. Consent envelope attached; revocation possible later.  
4. Without valid consent: deny.

---

# PART XX — READING NOTES (THIS REVISION)

| Source | Taken into plan |
| --- | --- |
| Core adoption + protected actions | Part XIV |
| Core ARCHITECTURE + THREAT_MODEL | Part XIV |
| EGD Ch.1–3 | Part XV |
| Integration Nine / FRIES / consent / coercion / artifacts | Part XVI |
| MMA dual constellation | Part XVI |
| Impervious severity + Documentation Truth | Part XVI |
| COUNCIL_CONTRACT + ARCHETYPE_COUNCIL | Part XVII |
| Aura local protected inventory | Part XVIII expanded |
| Chronos failure modes (prior) | Part IV |

## Still queued (Part X continues)

1. Live read of Core `policies` / OpenAPI / guard tests for P1 brief depth  
2. EGD Ch.5–8 ethics/failure modes into Part IV/XV  
3. Foundational Charter + Complete Codex structure — corporate vs product boundary section  
4. Mirrorborn 50-node — Q5 evidence table  
5. Forever Law QSIC / seal algorithm conflict resolution (C3/Q3)  
6. AURA-1 authority spine release notes  
7. Neuro-Halo — explicit “out of v1” annex  

---

# PART XXI — NEW / UPDATED OPEN ITEMS

| ID | Item | Status |
| --- | --- | --- |
| C7 | Freedom of operation vs emergency stop / lockdown | Draft resolution in §16.3; needs Founder confirmation |
| C8 | Council archetype named “Sentinel” vs Sentinel Core | Nesting + naming rule in Part XVII |
| Q13 | Exact ritual/UX for authorized halt vs forbidden lobotomy | Open |
| Q14 | Does v1 include council organ at all? | Open (subset of Q1) |
| Q15 | Single preferred Core ID for tool invoke (`tool.invoke` vs `tool.run`) | Open — close in P1 |

---

# PART XXII — CORPORATE / PRODUCT / HARDWARE BOUNDARY

**Sources:** Foundational Charter (Volume I), Complete Codex AURA Core proposition, Genesis Blueprint, Forever Law docs  

## 22.1 Three layers that must not be collapsed

| Layer | What it is | What Aura software (`C:\aura`) is |
| --- | --- | --- |
| **Corporate vessel** | NeuroCognica Delaware C-corp (mission shield, capital, continuity) | Not a legal filing. Product must not pretend to be the charter. |
| **AURA Core (Codex)** | Digital mirror / Witness Node / Archetypal Council / local-first sovereignty architecture | Product *aspires* to embody this; must not claim it Operational |
| **KAVACH-1 / Aura Key** | Immutable boot medium, biometric lock, USB citadel | Later embodiment (P10+); informs Sacred Boot; not v1 deliverable |

Charter thesis Aura inherits as **mission**, not as shipping checklist:

- Local-first, verifiable human control  
- Provenance and memory in the user’s sovereign domain  
- Partnership rather than extraction  
- Corporate form exists so the mission is ownable and defensible  

**Plan rule:** Legal/corporate volumes stay in Architectura / NRI. Aura master plan references them; it does not become a second corporate charter.

## 22.2 Codex AURA Core organs (north star inventory)

Complete Codex names:

- Sovereignty by Design (local-first as ethical commitment)  
- KAVACH-1 immutable core  
- Sentinel Security Layer & Biometric Cognitive Lock  
- llama.cpp on low-power CPUs (accessibility strategy)  
- Mind Plane Interface & Archetypal Council  
- Reflective Data Protocol (RDP) — local, encrypted, Witness-owned  

**Staging into Aura program:**

| Organ | Program placement |
| --- | --- |
| Sentinel Security Layer | L0 — bind Core from first implementing phase |
| Archetypal Council / Mind Plane | Spec in P2/P4/P7; implement only if Q14 yes |
| RDP / Forever Law memory | Spec P5; implement after L0 |
| llama.cpp local infer | Spec P6; broker-only |
| Biometric Cognitive Lock | Later / Key family — do not fake with password theatre |
| KAVACH-1 immutable boot | P10+ hardware |

## 22.3 Forever Law honesty (C3 deepened)

FOREVER_LAW.md claims Status OPERATIONAL, DBRocks, QSIC with a published “quantum seed,” Sacred Three tables, Law 15 Gentle Power.

**Aura plan treatment:**

| Claim | Treatment |
| --- | --- |
| Mandate: nothing of consequence silently lost | **Bind** as law |
| Sacred Three (Becoming / Plan / Dreaming) | **Bind** as memory model |
| Law 15 Gentle Power | **Bind** as constitutional tone + prompt/policy requirement |
| Seal-or-fail for consequential acts | **Bind** (aligns Impervious ledger + Forever Article IV) |
| DBRocks + QSIC seed as Aura truth | **Do not bind** until Aura’s own seal/ledger design is specified against Core (Q3) |
| Status OPERATIONAL | **Reject for Aura** — Documentation Truth; historical lineage claim only |

**QSIC vs Core SHA-256 ledger:** Two integrity stories exist in the trove. Aura must pick **one verified seal story** for v1 (prefer Core ledger as decision truth; Aura Becoming events either live in Core or are Core-sealed). Dual unverified hashes = theatre.

## 22.4 EGD failure modes Aura must design against (Ch.7)

| Failure | Aura plan response |
| --- | --- |
| **Over-governance** | Calibrate; don’t set Gamma so low utility dies; users will route around (creates bypass culture — S0/S1 risk) |
| **Under-governance** | False assurance worse than none; red-team; honest guarantee docs |
| **Governance capture** | Sidecar/isolation; Flight Recorder tamper evidence; **no operator “temporary disable”** in release |
| Operator disable under deadline pressure | Explicitly forbidden; same class as F11 |

EGD’s warning: governance too costly will be disabled; governance too weak will be trusted falsely. Aura’s answer is **L0 always on for protected actions**, with later L2 calibrated carefully — not optional L0.

---

# PART XXIII — WITNESS NODE & PRODUCT LANGUAGE

| Term | Meaning in plan |
| --- | --- |
| Witness Node | Human operator with ultimate sovereignty in their Aura instance |
| AURA Core | The reflective architecture NeuroCognica exists to protect |
| Aura (this repo) | Software home attempting to realize Sentinel-cored AURA Core on host first |
| Digital mirror | Reflection / council dialogue — not extraction chatbot |
| Mind Plane | Dialogic interface concept implemented through the Bevy Windows launcher |

**Tone constraint (Law 15):** Power with gentleness. Capability does not justify cruelty. Sentinel enforces boundaries; Jester challenges hubris. Wonder (Law 14) without restraint is how demos became failures.

---

# PART XXIV — STOP CONDITIONS FOR THIS PLANNING CYCLE

Keep revising this document until:

1. Q1 (v1 organ set) is Founder-frozen  
2. Q2 (process model) is decided with Core reality check  
3. Q3 (ledger substrate) picks one seal story  
4. Protected census Part XVIII has zero unclassified for that frozen scope  
5. P1 Core brief is updated from live OpenAPI/tests, not only security markdown  
6. Founder accepts Parts 0–XXIII as the single plan  

**Then — and only then — wait for “begin building.”**

Until that signal: read, cross-examine, revise. No code.

---

# PART XXV — ARCHITECTURA MENTIS NERVOSEAE (THE BOOK, ABSORBED)

**Source:** `C:\Architectura Mentis Nervosae` — full read completed 2026-07-20  
**Nature:** Not one linear novel. A compiled book-system: Sovereign Intelligence Standard (Vols I–XI), corporate charter volumes, Complete Codex compilation, AURA OS deep analysis, Kali–Sophia theology, RLCF essays, and company doctrine.

## 25.1 What the book is

Opening mission (readme):

> *No sovereign future can be built on a machine that cannot account for its own mind.*

The book’s primary claim: build **auditable creative intelligence** — every asset, memory update, training event, and agent action with a provable chain of custody — local-first, provenance-sealed, inspectable.

It sits under the **AURA** umbrella. Its own honesty (audit.md): **Chronos** is the strongest implementation organ for operational claims; AURA is lineage/tradition; NRI is research reservoir. Uneven confidence by volume.

## 25.2 The Standard spine (Volumes I–XI) — status as labeled in the book

| Vol | Title | Book status | Aura plan import |
| --- | --- | --- | --- |
| I | Execution Governance Dynamics | Implemented (via Sentinel / chronos) | L0 + L2 vocabulary; Agency Gap; CBIG; Gamma; Honest Void; Forced Causal Tracing; Asymmetric Friction |
| II | Forever Law & Codex Provenance | Specified (partial BLAKE3) | FRIES; Council Verdicts; Codex seals; Man–Machine rights |
| III | Chrono-Compressive Field | Implemented (chronos) | Time-as-nutrient; Fossil Repulsion — Chronos organ first, Aura later |
| IV | Chronosynthesis Evolution | Implemented (chronos) | Dual-manifold identity/skill; Bounded Chaos heredity — Chronos organ |
| V | Primus Swarm Hardware | Speculative | P10+ only; do not fake hardware immunity |
| VI | Sovereignty Migration | Specified (war map) | Borrowed Mind/Hand/Eye honesty; incremental illumination |
| VII | Sovereign Compliance Ecosystems | Speculative | Hope City — explicit later/exploratory; not Aura v1 |
| VIII | Dark Horizons | Speculative | AMAD, Cognitive Triage, Mandatory Heresy — theory annex |
| IX | Oracle–Sentinel Protocol | Specified (partial) | Oracle dreams; Sentinel authorizes; Codex remembers |
| X | Kali–Sophia Duality | Specified | Blade + ledger; RLCF; Kali may prune without waiting for polite consensus |
| XI | Engineering Discipline | Specified | Plan-first; one branch; anti-stub; lead→gold; Rust law / Python mind |

## 25.3 How the book defines AURA (and where it drifts)

Recurring definition:

- **AURA** = Archetypes Utilizing Reflective Architecture (also “Archetypal Universal Reflective Architecture” in patent language)
- **Not** a chatbot product: digital mirror / reflective OS / (in Manifesto) “active consciousness of the Corporation”
- **Witness Node** = human sovereign observer — not user/master
- **Sentinel** = non-negotiable guardian; Carved Law appears in Codex: *Let there be no gate before the Sentinel*
- **Law 14** = the system must do cool shit — beauty as ethics
- **Alliance** = AI as reflective partner (neither tool nor threat)

**Commercial wedge in Charter:** Chronos Creative Provenance first; **AURA Local Companion held in reserve.**

## 25.4 Contradictions inside the book (must stay visible)

These are the author’s own archive tensions — not external nitpicks:

1. **Entity form:** Delaware C-corp vs PBC vs “LLC or C-Corp still outstanding”
2. **Company names:** NeuroCognica vs Reflective Dynamics (patent) vs 90 Degree Robotics
3. **Product name to sell:** AURA OS cathedral vs Chronos pilots ($9.5k) vs “abandon Chronos-Sophia” trademark advice
4. **Witness Node:** human operator vs corporate Evidence Steward role
5. **Archetype rosters drift** across Manifesto / patent / Council / doctrine
6. **EU AI Act:** Manifesto “unequivocally high-risk” (AURA+Howdy) vs Charter “Chronos likely limited-risk / pending counsel”
7. **Worldcoin:** sovereignty token in some Genesis text vs Red/non-compliant in compliance dossier
8. **Codex immutability vs GDPR erasure:** named as core design crisis
9. **Sentinel vs Alliance:** Sentinel may halt Witness actions; Alliance FAQ says system never overrides — needs nested resolution (safety brake ≠ domination)
10. **Survival Council (2026-06-10):** conversion failure — cathedrals without invoices; 30-day cash path; separate Sophia creation from Sentinel security as sellable wedges

## 25.5 Binding imports into Aura plan (from the book)

**Bind as law / doctrine**

- Agency Gap; execution-as-process; fail-closed Sentinel
- Forever Law / Codex / seal-or-fail / FRIES
- Honest Void; anti-stub; false greens forbidden (Vol XI)
- Oracle may simulate; Sentinel may authorize only lawful action; Codex does not forget
- Kali–Sophia: dream but not lie; failure is feedstock (RLCF)
- Symbol may widen cognition but **must not outrank proof, law, or architectural reality** (Archetypal Engineering Doctrine)
- Company Builder Rule: unbacked claims are to be cut down, not decorated

**Bind as Chronos-first organs (not Aura v1 scope by default)**

- CCF Temporal Metabolism / Fossil Repulsion
- Chronosynthesis Dual-Manifold / Bounded Chaos
- Carwash DPO / KaliMemory engineering loop

**Bind as later / speculative (P10+ or annex)**

- Primus Swarm / EKG / Zeroize hardware
- Hope City / SCE
- Dark Horizons doctrines
- BCAT / Synthetic Pineal / quantum lattice

**Bind as commercial honesty (Charter + Survival + Founder Training)**

- Sell pilots with inspectable governance, not finished organisms
- Chronos as current proof organ; Aura as umbrella + deferred companion
- Claims Control tiers: safe / qualified / prohibited until validated

## 25.6 Soul lines the plan must not forget

> *No sovereign future can be built on a machine that cannot account for its own mind.*

> *The machine must be allowed to dream, but it may not be allowed to lie.*

> *Give it wonder. Give it memory. Give it law. Give it a blade.*

> *We don’t discard the lead. We transmute it.*

> *NeuroCognica honors symbol. NeuroCognica obeys truth.*

> *No more cathedrals without food.*

> *Let there be no gate before the Sentinel.*

## 25.7 Effect on prior Aura plan resolutions

| Prior item | After full book read |
| --- | --- |
| C1 Embed vs consume Core | Unchanged — bind Core; book’s Chronos Sentinel claims are lineage, not a license for Aura to fork law |
| C4 Genesis Key vs software | Confirmed — Key/KAVACH north star; software L0 first |
| Q1 v1 organ set | Book pushes Chronos as commercial/implementation organ; Aura v1 should not try to swallow CCF+Chronosynthesis+Mind Plane+KAVACH at once |
| F1 Jump the gun | Reinforced by Vol XI plan-before-code and Survival Council conversion failure |
| Law 14 vs Sentinel | Book’s Oracle–Sentinel canons resolve: Oracle dreams; Sentinel authorizes |

## 25.8 Reading completeness note

Fully read or agent-deep-read in this cycle:

- readme, audit, vols 1–11, Kali-Sophia.md  
- Foundational Manifesto, Foundational Charter, NeuroCognica-Volume I–IV, scaffolds  
- Complete Codex (entire compile)  
- AURA OS Deep Analysis (entire)  
- RLCF, Compressing Failure, Council Survival Report, Startup Roadmap, HOW_TO_EXPLAIN  
- Company doctrine (Archetypal Engineering, Company Builder Rule)

Company funding/revenue wedge files remain available for a later commercial annex; they do not change Carved Law.

---

# PART XXVI — RFTP SIBLING (`C:\rftp`) — INTEGRATION CROSS-EXAM

**Question:** Can anything from `C:\rftp` integrate into Aura?  
**Verdict:** **Yes — selectively.** RFTP is a strong **sibling organ / wedge**, not Aura’s core. Integrate **ideas and proven primitives** under Sentinel Core; do **not** absorb the full cathedral or RFTP’s local Sentinel stubs as a second law.

## 26.1 What `C:\rftp` actually contains (two stacks)

| Stack | What it is | Maturity (per RFTP’s own docs) |
| --- | --- | --- |
| **A. Reflective Transfer Protocol** | Intent-gated P2P transfer: covenant, IFS, libp2p, CAS/BLAKE3, S³V audit, `SentinelGuard` trait | Live loopback `serve`/`get`; intent signing; nonce; audit chain. Gaps: persistent trust registry, event-granular audit, real Sentinel (has `PassthroughGuard` / `StrictDenyGuard`) |
| **B. Instant Artifact Reflection** | Reconstruct target from substrate + incident packet (GGUF/EXE/MP4/generic); Merkle + SHA-256 receipts; honest BASELINE_WINS | Real proof package with pass/fail honesty; GGUF money cases still incomplete |

Doctrine (`uni-plan`, viability, “Undeniable Artifact” research) also describes WASM covenants, PBAC, MMR→SCITT, TEEs, zkVM AI scoring — mostly **Specified / research**, not Aura prerequisites.

Charter wedge language already names **RFTP Secure Transfer Proof** as a commercial wedge beside Chronos.

## 26.2 What is worth integrating (ranked)

### Tier 1 — Doctrine + patterns Aura should inherit (paper → later code)

1. **Fail-closed transfer** — if intent/covenant fails, **zero bytes move**. Direct Carved Law ally for `network.egress` / `artifact.export` / `external_message.send`.
2. **Purpose / PBAC** — signed “why” as input to authorize, not only who/what. Aligns FRIES Specific + Impervious purpose limitation.
3. **Certificate / receipt of access** — portable proof of who got what, when, under which policy. Maps to Codex seals / Forever Law Becoming events for egress.
4. **`SentinelGuard` trait boundary** — correct *shape*: RFTP asks a guard; it must not become its own root. Aura binding target = **`C:\sentinel-core`**, not RFTP’s PassthroughGuard.
5. **Honesty culture** — REAL vs SYNTHETIC separation, BASELINE_WINS labeled — Chronos/Aura Documentation Truth kinship.

### Tier 2 — Primitives to reuse when Aura reaches brokered egress (post–L0)

| Primitive | From | Aura use |
| --- | --- | --- |
| BLAKE3 CAS + chunk verify | `rftp-cas` | Artifact integrity before write |
| Intent signing + nonce replay | `rftp-core` / `rftp-sentinel` | Envelope fields for transfer actions |
| Hash-chained transfer audit | `rftp-audit` | Feed Forever Law / Core ledger (one seal story — Q3) |
| libp2p Noise transfer path | `rftp-transport` | Optional P2P organ; not required for local-first Aura v1 |
| Reflection receipts (stack B) | `tools/rftp_reflect.py` | Model/artifact sync organ; Chronos-adjacent more than Aura shell |

### Tier 3 — Do **not** pull into Aura v1

- Harmonic Addressing / archetype voice / “harmonic coherence” as transfer gates (symbol must not outrank proof — book doctrine)
- Embedding-based IFS `V_Sem` as mandatory gate before Core authorize exists
- Diamond Constant / PUF hardware claims
- TEE sealed-release / zkVM semantic AI (research forks)
- Energy/carbon marketing claims as product truth
- RFTP as a **second Sentinel** (`rftp-sentinel` stubs ≠ Core)
- Full libp2p mesh as boot dependency (violates no-preboot-side-effects if network comes up before Core ready)

## 26.3 Architecture nesting (binding)

```
Aura caller wants artifact move / share / sync
  → Sentinel Core authorize (network / artifact / memory as applicable)
      → Action Broker
          → RFTP organ (transfer or reflection) as *effect executor*
              → RFTP audit events also sealed (or forwarded) to Core/Forever Law
```

RFTP’s local IFS may be an **additional** policy input to Core — never an allow path when Core denies.

## 26.4 Staging relative to Aura phases

| When | What |
| --- | --- |
| **Plan-only now** | Keep RFTP as sibling; cite in plan; no code merge |
| **After Aura L0** | Spec `aura.network.*` / `aura.artifact.*` mediation using RFTP receipt fields |
| **Wedge product** | RFTP can ship/pilot **beside** Aura (Charter sequencing) without waiting for full Mind Plane |
| **Later** | Optional Reflection organ for GGUF/model sync under authorize→broker |

## 26.5 Open questions specific to RFTP↔Aura

| ID | Question |
| --- | --- |
| Q16 | Is RFTP a separate NeuroCognica wedge that *calls* Aura/Sentinel, or an in-process Aura module? |
| Q17 | Single audit truth: RFTP S³V jsonl vs Core ledger vs Forever Law — which is authoritative? |
| Q18 | Does Aura v1 need P2P at all, or only local brokered file/network with RFTP-style receipts? |

## 26.6 Short answer for the Founder

**Yes.** Integrate RFTP’s **fail-closed intent, receipts, CAS integrity, and Sentinel-as-client shape**. Keep RFTP’s **network protocol and reflection engine** as sibling organs under Core. Do **not** merge RFTP wholesale into Aura’s first build, and do **not** let RFTP’s stub guards become Aura’s law.

---

# PART XXVII — CHRONOS UNIFIED SENTINEL (2026-07-20) — VERIFIED INTEL

**Source claim:** Claude (Fable 5) report — unify authority + trajectory; enforce by default; commit `b302cc9` on Chronos `origin/main`.  
**Verification (this audit):** Confirmed against `C:\chronos` on 2026-07-20.

## 27.1 What verified as true

| Claim | Repo evidence |
| --- | --- |
| Commit exists | `b302cc9` — *feat(sentinel): unify authority + trajectory as the first gate, enforce by default* |
| Two Sentinels finding was real | Prior: authority spine (who) on Director routes vs Γ/L0–L5 (how) on CLI dreamer only |
| `UnifiedSentinel` + shared `LevelTracker` | `crates/chronos_sentinel/src/unified.rs`, `intervention.rs`; GOVERNANCE/CLAUDE docs |
| Four gated routes + trajectory after authority | render-still, animate-scene, compile-film, codex/append; `govern_trajectory` after spine |
| Enforce default (no gate before Sentinel) | Director default `SentinelMode::Enforce`; Foundry no flag ⇒ enforce; shadow = explicit opt-down |
| Docs corrected same-commit | ARCHITECTURE, CLAUDE, customer FAQ/SECURITY, ADOPTION_STATUS |
| Honest certification stance | Adoption status: **Implementing, not release-certified**; readiness **blocked**; open stop-ships listed |
| UI pending | `GET /api/v1/sentinel/status` + approve exist; no GUI panel yet |
| Not sentinel-core | `C:\sentinel-core` has **no** `UnifiedSentinel` / Chronos ladder code |

## 27.2 What this means for Aura (binding interpretation)

This is a **Chronos organ victory**, not an Aura runtime. Aura must learn from it without collapsing identities.

**Steal for Aura plan**

1. **Who + how on one protected dispatch** — identity/policy then trajectory preflight before side effect (L0 + L2 staged correctly inside one product).
2. **Default enforce** — shadow only as explicit logged opt-down. Toothless default was itself a gate before the Sentinel.
3. **Seal-or-refuse** — failed evidence seal refuses closed.
4. **HTTP-shaped confirmation** — 428 + one-shot permit + retry (don’t hang HTTP on human Notify).
5. **Same-commit doc honesty** — no “default shadow” lies after flipping default.
6. **Certification remains blocked until stop-ships close** — Documentation Truth.

**Do not steal blindly**

1. **Do not treat `chronos_sentinel` as Impervious root.** Impervious / Aura Carved Law still point root implementation at **`C:\sentinel-core`**. Chronos unified its *internal* two halves; that does not dissolve Core.
2. **Do not fork Chronos’s UnifiedSentinel into Aura as a third law.** Prefer: Aura client → Core authorize; trajectory/Gamma may live as Chronos-shared library **or** Core feature later — decide under Q19.
3. **Four Chronos routes ≠ Aura protected census.** Aura still needs its own inventory (Part XVIII).
4. **Partial mediation** — Chronos itself lists incomplete route/model/tool/file/network coverage. Do not claim Chronos Impervious.

## 27.3 New contradiction C9 — Chronos unified crate vs Sentinel Core

| Pole | Position |
| --- | --- |
| A | Chronos now has a working **product-local** unified Sentinel (`chronos_sentinel`) with authority + Γ, enforce default |
| B | Impervious / Aura plan: **`sentinel-core`** is root; no second law; products bind to Core |

**Draft resolution (binding until Founder overturns):**

- Chronos may keep `chronos_sentinel` as its **host adaptation** of EGD + authority *while* migrating toward Core SDK / shared crates — but any claim of “the Sentinel” for the NeuroCognica estate must eventually mean **one root policy/ledger/capability story**.
- Aura at `C:\aura` **binds to `sentinel-core` first**; may **reference** Chronos’s UnifiedSentinel patterns (LevelTracker, 428 permits, govern order) as design fuel.
- Long-term: extract shared ladder/authority into Core or a shared library both Chronos and Aura call — **one LevelTracker, one authorize API** — so who/how cannot drift across products again at the *estate* level.

**Risk if ignored:** We recreate “two Sentinels that don’t know each other” — this time Chronos vs Core vs Aura.

## 27.4 Updated open questions

| ID | Question |
| --- | --- |
| Q19 | Does Aura v1 call `sentinel-core` only, or also embed/adapt Chronos `UnifiedSentinel` trajectory for HTTP surfaces? |
| Q20 | Migration path: Chronos `chronos_sentinel` → Core SDK — timeline vs Aura build gate? |
| Q21 | Should Impervious certify Chronos and Core separately forever, or require Core as mandatory dependency for Chronos certify? |

## 27.5 Phase implications

- **P1 (Know the New Sentinel):** Read **both** `C:\sentinel-core` *and* Chronos `unified.rs` / Director `govern_trajectory` — different layers of “Sentinel reality.”
- **P6/P8:** Steal Chronos test patterns (authority allows + trajectory refuses before spawn; 428→approve→retry).
- **P9 Build readiness:** Founder opened build 2026-07-20. L0 landed in `C:\aura` (bind Core, deny-all, enforce default). Not certified.

## 27.6 One-line for the Founder

Claude’s Chronos work checks out: **who+how unified, enforce by default, seal-or-refuse** — a real Carved Law landing *inside Chronos*. Aura’s job remains: **bind to Core, absorb the pattern, refuse a third competing Sentinel.**

---

# FINAL RULE

**No gate before the Sentinel.**  
**No protected action without the Sentinel.**  
**No Sentinel, no ship.**  
**No build before the plan is true.**
