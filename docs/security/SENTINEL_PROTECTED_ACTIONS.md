# Sentinel Protected Actions — Aura

Inventory of protected actions for Aura. Unlisted protected surfaces are stop-ship findings.
**Status:** Implementing L0 runtime + first launcher slice. Full v1 census remains blocked on
Q1 organ freeze. AURA keeps a local protected-action vocabulary compatible with the broader
Sentinel registry. Full census reasoning:
`docs/plans/AURA_MASTER_PLAN.md` Part XVIII.

## Inventory Rules

- List every operation that can change state, reveal sensitive information, affect a person, affect hardware, communicate externally, execute tools, launch processes, modify memory/identity, or influence model behavior toward harm.
- Each row must name the mediation point, preferred Core action ID, and the deny test that proves failure before side effects.
- Unclassified surfaces for a **frozen** v1 scope are stop-ship. v1 organ set (master plan Q1) is still Open — census cannot close until Q1 closes.
- Do not invent Aura-only allow paths. Map to Core IDs or propose Core registry additions.

## Protected Action Registry (planning draft)

| Action ID | Prefer Core ID | Surface | Caller | Mediation point | Side effect if allowed | Deny test | Status |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `aura.boot.continue` | TBD (propose to Core) | Boot / launcher | Runtime / Bevy launcher | Sentinel ready gate via `aura_runtime` broker | Enters app work mode | `deny_all_blocks_boot_continue_and_executes_no_effect` | Implementing |
| `aura.council.append` | TBD / typed `effect.execute` | Council | UI / API | Sentinel authorize | Persists council envelope | Deny ⇒ no persist | Planned |
| `aura.council.replay` | TBD / `file.read_sensitive` | Council | UI / API | Sentinel authorize | Reads sealed history | Deny ⇒ no read | Planned |
| `aura.memory.write` | `memory.write` | Memory | Runtime | Sentinel authorize | Appends Forever-Law event | Deny or seal-fail ⇒ no write | Planned |
| `aura.memory.delete` | `memory.delete` | Memory | Runtime | Sentinel authorize | Removes/tombstones memory | Deny ⇒ no delete | Planned |
| `aura.memory.export` | `artifact.export` (confirm) | Memory | UI / API | Sentinel authorize | Exports sealed corpus | Deny ⇒ no export | Planned |
| `aura.document.frame` | `file.read_sensitive` | Documents / RAG | UI / broker | Sentinel authorize before reading user-selected source | Reads, normalizes, hashes, chunks, and prepares branded print-ready HTML for selected source text | `deny_all_blocks_document_frame_before_source_read` | Launcher button live; deny-all refuses before read |
| `aura.document.ingest` | `memory.write` + `artifact.register` (confirm) | Documents / RAG | UI / broker | Sentinel authorize before store append, after source frame authorization | Writes framed document, chunks, normalized text, print-ready HTML artifact, indexes, and BLAKE3 Forever event to RocksDB | `deny_all_blocks_document_ingest_before_store_append` | Launcher button live; deny-all refuses before DB append |
| `aura.document.open_db_folder` | `process.spawn` | Documents / RAG | UI / broker | Sentinel authorize before Explorer spawn | Opens local document DB folder | Deny ⇒ no process spawn | Launcher button live; denied under default policy |
| `aura.document.query` | `memory.read_sensitive` (confirm) | Documents / RAG | UI / model mediation | Sentinel authorize before retrieval over private corpus | Retrieves chunks for model context | Deny ⇒ no retrieval | Planned |
| `aura.profile.read` | confirm with Core | Identity | UI / API | Sentinel authorize | Reveals Mirrorborn profile | Deny ⇒ no reveal | Planned |
| `aura.profile.write` | `profile.generate` | Identity | UI / API | Sentinel authorize | Mutates identity model | Deny ⇒ no write | Planned |
| `aura.identity.key.*` | `identity.key.*` | Identity | Runtime | Sentinel authorize | Trust root change | Deny ⇒ no key op | Planned |
| `aura.model.infer` | `model.generate` | Model | Broker | Sentinel authorize | Runs local model | Deny-all ⇒ no inference | Planned |
| `aura.tool.invoke` | `tool.invoke` or `tool.run` (Q15) | Tool | Broker | Sentinel authorize | Executes tool side effect | Deny ⇒ no tool call | Planned |
| `aura.file.read_sensitive` | `file.read_sensitive` | File | Broker | Sentinel authorize | Reads sensitive path | Deny ⇒ no IO | Planned |
| `aura.file.write` | `file.write` | File | Broker | Sentinel authorize | Writes outside preboot journal | Deny ⇒ no write | Planned |
| `aura.file.delete` | `file.delete` | File | Broker | Sentinel authorize | Deletes file | Deny ⇒ no delete | Planned |
| `aura.network.egress` | `network.egress` | Network | Broker | Sentinel authorize | External network I/O | Deny ⇒ no socket | Planned |
| `aura.network.request` | `network.request` | Network | Broker | Sentinel authorize | Network request | Deny ⇒ no request | Planned |
| `aura.process.spawn` | `process.spawn` | Process | Broker | Sentinel authorize | Spawns subprocess | Deny ⇒ no spawn | Planned |
| `aura.shell.execute` | `shell.execute` | Process | Broker | Sentinel authorize | Shell command | Deny ⇒ no shell | Planned |
| `aura.artifact.register` | `artifact.register` | Artifact | Runtime | Sentinel authorize | Registers artifact | Unsigned ⇒ deny | Planned |
| `aura.artifact.use` | `artifact.use` | Artifact | Runtime | Sentinel authorize | Uses artifact | Deny ⇒ no use | Planned |
| `aura.artifact.export` | `artifact.export` | Artifact | UI / API | Sentinel authorize | Exports artifact | Deny ⇒ no export | Planned |
| `aura.capability.issue` | `capability.issue` | Capability | Runtime | Sentinel authorize | Issues capability | Deny ⇒ no issue | Planned |
| `aura.capability.consume` | `capability.consume` | Capability | Runtime | Sentinel authorize | Consumes capability | Deny ⇒ no consume | Planned |
| `aura.agent.spawn` | `agent.spawn` | Agent | Runtime | Sentinel authorize | Spawns agent loop | Deny ⇒ no spawn | Planned |
| `aura.external_message.send` | `external_message.send` | Message | Broker | Sentinel authorize | Off-host message | Deny ⇒ no send | Planned |
| `aura.plugin.load` | `plugin.install` / `plugin.execute` | Plugin | Runtime | Sentinel authorize | Loads extension code | Deny ⇒ no load | Planned (later) |
| `aura.release.install` | `installer.update` / `system.install` | Install | Installer | Sentinel authorize | Installs/updates binaries | Deny ⇒ no install | Planned (later) |
| `aura.hardware.camera` | `hardware.activate_camera` | Hardware | Runtime | Sentinel authorize | Camera on | Deny ⇒ no activate | Out of v1 default |
| `aura.hardware.microphone` | `hardware.activate_microphone` | Hardware | Runtime | Sentinel authorize | Mic on | Deny ⇒ no activate | Out of v1 default |
| `aura.robot.command` | `robot.command` | Hardware | Runtime | Sentinel authorize | Robot motion | Deny ⇒ no command | Out of v1 |
| `aura.game.share` | `game.share` | Game | UI | Sentinel authorize | Social/export | Deny ⇒ no share | Out of v1 |
| `aura.payment` | `payment.or_commitment` | Finance | UI | Sentinel authorize | Money/legal commit | Deny ⇒ no commit | Out of v1 |

## Explicit Non-Protected Surfaces

| Surface | Location | Justification |
| --- | --- | --- |
| Safe health | Planned `/health` | Read-only health, no sensitive disclosure |
| Launcher status refresh | `crates/aura_launcher` | Display-only runtime status refresh |
| Document DB status refresh | `crates/aura_launcher` | Display-only path/count summary; no document source read and no DB append |
| Document source path selection | `crates/aura_launcher` | Native picker records the operator-selected path only; no source bytes, directory scan, or DB append occurs before protected actions |
| Sentinel initializing UI | `crates/aura_launcher` | Display-only blocked/init state |
| Deny / lockdown chrome | Shell | Display-only |
| Emergency stop | OS / hardware | Independent safety brake; never an approval path |
| Preboot journal | Local | Minimal journal that cannot approve work |

## Unclassified Surfaces

| Surface | Location | Why unclassified | Owner | Due |
| --- | --- | --- | --- | --- |
| _(blocked on Q1 v1 organ freeze)_ | | Cannot close census until v1 scope is Founder-frozen | Plan P3 | After Q1 |

Update this file in the same change that specifies any new protected surface.
