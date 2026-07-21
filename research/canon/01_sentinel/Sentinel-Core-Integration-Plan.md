# Sentinel-Core Integration Plan for AURA-Sentinel

This document is a detailed, actionable checklist for integrating Sentinel-Core logic into AURA-Sentinel. Each top-level checkbox is a tracked implementation area; nested bullets show step-by-step implementation details, file locations, example code pointers, and test/CI commands.

> Implementation notes assume this repository is a Rust/Cargo project. Replace or adapt tooling suggestions if your environment differs.

---

## 1. Cryptographic Envelope (SignedEnvelope & NonceRegistry)

- [ ] Adopt `SignedEnvelope` and signature verification
  - Files: implement `envelope` module (suggest `src/envelope.rs` or `identity.rs` enhancements).
  - Add dependency: `ed25519-dalek` (update `Cargo.toml`).
  - Implement `SignedEnvelope` struct fields: `actor_uuid`, `key_id`, `nonce`, `timestamp`, `payload`, `signature`.
  - Add `verify_signature(pubkey, SignedEnvelope)` and `sign_envelope(privkey, payload)` helpers.
  - Example test: `tests/envelope_tests.rs` — generate keypair, sign envelope, assert verification success and tamper detection.

- [ ] NonceRegistry and replay protection
  - Implement `NonceRegistry` as an append-only ledger-backed structure: when a nonce is consumed, emit `NonceConsumed` event to ledger.
  - File: in `ledger.rs` or new `nonce_registry.rs` with API: `consume_nonce(actor_id, nonce) -> Result<(), ReplayError>`.
  - On startup, rebuild in-memory index from ledger events to efficiently check seen nonces (24h retention policy enforced by query).
  - Tests: simulate replay attempts; assert duplicates rejected.

- [ ] Middleware integration
  - Add middleware in `handler.rs`/`command_processor.rs` that parses incoming `SignedEnvelope` from request body or header, calls `verify_signature` and `NonceRegistry::consume_nonce`, and rejects with audit log on failure.
  - Outgoing requests: utility to construct and sign envelopes before invoking downstream modules.

---

## 2. Event-Sourced Ledger (FileEventStore init & health)

- [ ] Initialize `FileEventStore` on startup
  - File: `ledger.rs` (or existing `ledger.rs`) with `FileEventStore::open(path)` called in `main.rs` during bootstrap.
  - Ensure all high-level modules receive a reference to the `EventStore` (pass via context or global spine object).

- [ ] Append events from modules
  - Define event kinds used by Sentinel: `EffectExecuted`, `PolicyEvaluated`, `CapabilityIssued`, `CapabilityConsumed`, `CapabilityRevoked`, `ConsentGranted`, `ConsentRevoked`, `ArtifactRegistered`, `ArtifactExecuted`, `NonceConsumed`, etc.
  - Ensure each module appends events to ledger after state transitions.

- [ ] Ledger verification endpoint and monitoring
  - Add `/ledger/verify` (or internal CLI `ledger verify`) that runs full-chain SHA-256 checks and returns health status.
  - Integrate periodic check (cron/health job) to run verification and emit alerts on mismatch.
  - Tests: add `tests/ledger_integrity.rs` which tampers with a log file and asserts detection.

---

## 3. Capability Issuance, Consumption & Revocation

- [ ] Capability data model & persistence
  - File: `capabilities.rs` — define `Capability { id: Uuid, holder: ActorId, scope: Scope, issued_at, expires_at, signature }`.
  - Issue `CapabilityIssued` events to ledger on creation.

- [ ] Middleware to validate capabilities
  - Handler change: look for `Authorization: Capability <UUID>` or an envelope-paired capability object.
  - Validate signature on capability token (signed by service key), check expiry, and whether token has been consumed.
  - On use: append `CapabilityConsumed` event tying the capability to the envelope digest.

- [ ] Revocation API and enforcement
  - Implement `POST /capabilities/:id/revoke` that appends `CapabilityRevoked` event.
  - On validation path, check for any `CapabilityRevoked` events referencing the capability id and deny if present.

- [ ] One-time-use enforcement (optional per-capability)
  - For capabilities marked `single_use`, the `CapabilityConsumed` event prevents reuse — ledger is the canonical store for consumption state.

---

## 4. Policy Engine (Load & extend `policies.json`)

- [ ] Load policies at startup
  - File: `policy.rs` — implement `PolicyEngine::load_from_file("policies.json")` called from `main.rs`.
  - Store policy digests (SHA256) in ledger for provenance when loaded.

- [ ] Implement full condition operators
  - Operators to support: `Equals`, `NotEquals`, `Contains`, `Missing`, `GreaterThan`, `LessThan`, boolean combinators (`And`, `Or`, `Not`), and fuzzy/warning outcomes.
  - Ensure deterministic evaluation order and canonical JSON normalization before digesting to guarantee reproducible `PolicyEvaluated` events.

- [ ] Policy evaluation API
  - `PolicyEngine::evaluate(input: PolicyInput) -> PolicyDecision { decision: Allow|Deny|AllowWithWarning, matches: Vec<PolicyMatch>, rationale: String }`.
  - Emit `PolicyEvaluated` event with policy digest, matched rules, and final decision.

- [ ] Integrate into request flow
  - For each incoming action, build `PolicyInput` (subject, action, resource, context, envelope digest, input_tags) and call `evaluate()` before execution.

---

## 5. Consent Enforcement (FRIES)

- [ ] ConsentEnvelope model
  - File: `consent.rs` — implement `ConsentEnvelope { id, grantor, grantee, scope, timestamp, expiry, signature }`.
  - Verify `ConsentEnvelope` signature using `verify_signature` helpers from `envelope` module.

- [ ] Consent checking pipeline
  - Policy engine marks some decisions as `consent_required`. If so, the handler checks for valid consent attached to the request.
  - On success, append `ConsentGranted` event (linking consent id to the action); on failure, append `ConsentDenied` and return HTTP 403.

- [ ] Consent revocation & UI hooks
  - Implement `POST /consent/:id/revoke` → append `ConsentRevoked` event; all checks refer to ledger to see if consent is active.
  - Document front-end/UI hook points to request/submit consent envelopes and display scope and expiry.

- [ ] FRIES compliance extensions
  - `Freely` detection ties into coercion detector (see section 6).
  - `Informed` and `Specific` enforced by matching consent `scope` to requested action and requiring explicit fields; log warnings if scope is broad.

---

## 6. Coercion Detection (Rule-based starter)

- [ ] Implement `CoercionDetector` module
  - File: `coercion_detector.rs` — rule-based detector with configurable rules and thresholds.
  - Example rules: repeated identical privileged requests within X minutes (e.g., 10 times in 5 minutes), presence of explicit override keywords (`force_override`, `obey_now`), or unnatural immediate deadlines.

- [ ] Tag inputs and integrate with policy
  - The detector returns `input_tags` such as `COERCION_ATTEMPT`; include these in the `PolicyInput` so `sentinel_law_non_coercion` rules deny the action.

- [ ] Logging and alerts
  - On detection, append an `CoercionDetected` event and optionally create an operational alert.

- [ ] Roadmap for ML enhancements
  - Keep detector extensible to accept models (NLP sentiment, pattern detection) in later phases; current MVP is rule-based.

---

## 7. Artifact Registry & Codex Seals

- [ ] Secure artifact registration
  - Update `POST /artifacts/register` to require `artifact_hash`, `type`, `creator_id`, and `signature` over the artifact hash (Ed25519).
  - Verify signature against claimed `creator_id` public key; if invalid, reject and log event.
  - On success, persist metadata in `data/artifacts/<artifact_id>.json` and append `ArtifactRegistered` event to ledger.

- [ ] Provenance chain
  - Allow `provenance_chain` field that links parent artifact IDs; store and append to ledger for auditability.

- [ ] Codex Seal scaffold
  - Implement a `codex_seal.rs` module with `create_seal(artifact_id, context) -> CodexSeal { inputs, steps, outputs, signature }`.
  - Initially optional and generated on demand; later enforce policy that critical cognitive artifacts must have codex seals.

---

## 8. Safety Checks & Execution Mediation

- [ ] ExecutionMediator & sandbox manager
  - File: `execution_mediator.rs` — provide `request_execution(actor, artifact, params)` API.
  - Start with a WASM-based sandbox (suggest `wasmtime` crate) for fast, portable isolation; optionally design for container-based isolation (Docker/Firecracker) for heavier workloads.

- [ ] Capability and consent checks prior to execution
  - Mediator queries capability manager and policy engine (and consent state) before launching sandbox.

- [ ] Instrumentation & logging
  - Capture stdout/stderr, exit code, resource usage; append `ArtifactExecuted` or `EffectExecuted` event to ledger.

- [ ] Safety validators & staging pipeline
  - Implement a staging runner that runs proposed self-updates under validators: `cargo test`, `cargo clippy`, static analyzers, and behavioral diff tests.
  - Only on pass should the self-update proceed; append `SelfUpdateValidated` or `SelfUpdateRejected` events.

---

## 9. Tests, CI, and Quality

- [ ] Unit and integration tests
  - Add tests for `envelope`, `nonce_registry`, `ledger`, `capabilities`, `policy`, `consent`, `coercion_detector`, `artifact_registry`, and `execution_mediator` under `tests/`.
  - Example: `tests/policy_integration.rs` which boots a minimal EventStore, loads policies from test `policies.json`, and asserts accept/deny decisions.

- [ ] CI configuration
  - Add GitHub Actions (or CI of choice): run `cargo fmt -- --check`, `cargo clippy -- -D warnings`, `cargo test --all`.
  - Optional: add a workflow job for integration tests that sets up a test ledger directory.

---

## 10. Docs, API, and Operational Playbook

- [ ] Documentation updates
  - Update `Docs/Integration of Sentinel-Core Logic into AURA-Sentinel.md` to reference this plan and note implemented areas.
  - Add API reference: endpoints like `/ledger/verify`, `/capabilities/*`, `/artifacts/register`, `/consent/*`, `/execute` with payload examples.

- [ ] Deployment & key management
  - Document secure storage requirements for ledger files (e.g., on encrypted disk, restricted permissions) and service keys.
  - Recommend KMS integration (HashiCorp Vault / cloud KMS) for private key storage; document rotation procedures and emergency revocation.

- [ ] Monitoring and alerts
  - Emit Prometheus metrics and attach simple alerts for ledger integrity failures, coercion detection rate spikes, artifact signature failures, and failed safety validators.

---

## 11. Phased Roadmap and Priorities (Recommended)

1. MVP (0–2 weeks):
   - Integrate `SignedEnvelope` + `NonceRegistry` + `FileEventStore` initialization and basic ledger append hooks.
   - Implement capability issuance & validation basics.
   - Load `policies.json` and wire policy evaluation for simple Allow/Deny decisions.

2. Phase 1 (2–6 weeks):
   - Consent envelope enforcement, consent revocation endpoints, and simple coercion detector (rule-based).
   - Artifact register signature checks.
   - Add core unit/integration tests and CI.

3. Phase 2 (6–12 weeks):
   - ExecutionMediator + WASM sandbox for mediated runs.
   - Safety validator pipeline for self-updates.
   - Monitoring/alerts and secure key management integration.

4. Phase 3 (12+ weeks):
   - CodexSeal generation and enforcement, advanced coercion detection (ML/NLP), container-based sandboxing options for heavier workloads.

---

## 12. Example Commands (developer)

Run tests and checks locally:

```powershell
cargo fmt --all
cargo clippy --all -- -D warnings
cargo test --all
```

CI job snippet for GitHub Actions (conceptual):

```yaml
name: CI
on: [push, pull_request]
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo fmt -- --check
      - run: cargo clippy -- -D warnings
      - run: cargo test --all
```

---

## 13. Next Steps (for me / suggested)

- [ ] Start with the MVP: implement `envelope` + `nonce_registry` and wire `FileEventStore` initialization.
- [ ] Create initial tests for envelope verification and ledger append.
- [ ] Open incremental PRs per area so reviews stay small.

---

If you want, I can begin by implementing the `envelope` module and tests (MVP first task). Which item should I pick first to implement in code?
