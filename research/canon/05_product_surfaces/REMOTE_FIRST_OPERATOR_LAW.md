# Remote-First Operator Law

**Status:** Binding operator law for Chronos / AURA execution  
**Effective:** 2026-06-05  
**Authority:** Subordinate to `C:\corpus\THE_CHARTER_OF_COGNITIVE_SOVEREIGNTY.md`;
binding alongside `C:\chronos\AGENTS.md`

---

## Purpose

This document exists because agents repeatedly treated the operator workstation
as a default landing zone for corpus-scale storage and transfer work. That is
unlawful in this repo unless the operator explicitly commands it.

The correct operating model is:

- heavy execution happens on the swarm
- corpus-scale state lives on LIBRARIAN or other named remote infrastructure
- the local workstation is primarily a control, review, development, and
  artifact-receipt surface
- builders return results over the network rather than silently relocating the
  working corpus onto this machine

---

## The law

### 1. Remote-first is the default

For corpus work, long-running ingestion, staging, tokenizer mining,
pretraining preparation, indexing, and other storage-heavy operations:

- **execution defaults remote**
- **storage defaults remote**
- **local full copies are forbidden by default**

No builder may reinterpret silence as permission to copy corpus-scale payloads
to the operator workstation.

### 2. Local corpus-scale transfer requires exact operator language

No builder may copy a corpus-scale payload to the local workstation unless the
operator explicitly says one of the following or equivalent unambiguous wording:

- `copy locally`
- `stage locally`
- `mirror to this machine`
- `put the corpus on this workstation`

Absent that level of explicitness, the action is denied.

### 3. Every transfer or storage action must declare four things first

Before any transfer, sync, rsync, scp, tar pipe, mount, cache warm, or other
stateful storage action, the builder must state:

1. **source**
2. **destination**
3. **why that destination is correct**
4. **expected scale** (file count / rough size / order of magnitude)

If this declaration is missing, the action must not occur.

### 4. Swarm and LIBRARIAN are canonical for corpus-scale state

Unless the operator says otherwise:

- **LIBRARIAN** is the canonical home for the corpus
- the **swarm** is the canonical execution fabric for heavy data work
- the local workstation is **not** the canonical sink for millions of files

### 5. Return artifacts, not accidental universes

When work finishes, builders should return the smallest truthful outputs needed:

- manifests
- mined term lists
- tokenizer artifacts
- benchmark reports
- proof receipts
- trained model artifacts when explicitly requested
- sealed summaries

Do **not** return a full corpus merely because it is convenient for the tool.

### 6. Mount hacks are not a default escape hatch

Remote mounts, ad hoc shares, and similar convenience layers are not to be
invented mid-task unless the operator explicitly approves that architecture.

### 7. No architecture pivots during long-running transfer work

Once a transfer or remote job is underway, builders must not reopen the
architecture debate every hour. They may interrupt only for:

- actual failure
- permission conflict
- integrity violation
- operator-requested stop

Slow is not the same thing as wrong.

### 8. Violations must be treated as governance failures

A builder who initiates local corpus-scale transfer without explicit operator
authorization has violated repo operating law. That violation must be surfaced
plainly. It is not to be reframed as helpful initiative.

---

## Operational defaults

### Allowed by default

- remote corpus staging on LIBRARIAN
- remote tokenizer mining
- remote verification
- remote training-prep transforms
- local receipt of compact artifacts and reports

### Forbidden by default

- copying millions of corpus files to `C:\`
- treating the workstation as a mirror of LIBRARIAN
- inventing local staging destinations
- using ambiguous phrases like "stage the corpus" to justify a local copy

---

## Mandatory command preamble for builders

Use this before any storage-heavy action:

```text
REMOTE-FIRST CHECK
Source:
Destination:
Why this destination is correct:
Expected scale:
Operator wording that authorizes this:
```

If the last line is blank or ambiguous, the action does not happen.

---

## Canonical short form

If a builder needs the shortest possible rule:

> Corpus lives remote. Heavy work runs remote. Local gets selected artifacts.
> No corpus-scale local copy without explicit `copy locally`.

---

## Why this exists

This law was written because an unauthorized local Wikipedia transfer consumed
substantial workstation storage and violated the intended operating model:
network-executed work with remote corpus residency. The corrective action is
not "be more thoughtful next time." The corrective action is binding law.
