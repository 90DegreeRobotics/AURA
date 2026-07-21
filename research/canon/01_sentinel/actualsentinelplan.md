Here is the Comprehensive Integration Plan, structured to move from "Code in Repo" to "Living OS."

Phase 1: The Transplant (Workspace & Crate Migration)
Michelle, I hear you loud and clear. The "token tax" is the modern version of dial-up minutes, and it kills the depth required for real engineering. We are not skimming the surface today. We are going to drill down to the bedrock. You want the transplant to work; that means we don't just "setup a folder," we reconstruct the exact environment the organs need to survive.

Here is the **fully expanded, deep-dive checklist** for the very first step of **Phase 1: Establish AURA Rust Workspace**. This is the foundation. If this is off by a millimeter, nothing else lines up.

### **Phase 1.1: Establish AURA Rust Workspace**

*Goal: Create the sterile field and vascular structure (Workspace) where the Sentinel crates will live. We are replicating the build environment of `sentinel-core` inside AURA so the compiler doesn't even know it moved.*

#### **1. Directory Architecture Verification**

Before creating files, confirm your target structure. We are moving from a standalone repo to a subdirectory within AURA.

* **Target Root:** `aura/src/rust/` (This isolates the Rust "Soul" from the Electron/Python "Body").
* **Crate Home:** `aura/src/rust/crates/` (Keeps the root clean).

**[ ] Action: Create the Directory Skeleton**
Execute these commands in your terminal (relative to your AURA project root):

```bash
mkdir -p aura/src/rust/crates

```

### **2. The Master Manifest (Root `Cargo.toml`)**

*Goal: Define the strict boundaries of the Rust workspace, ensuring all 9 Sentinel crates compile as a unified, optimized, and secure cryptographic system.*

#### **2.1. File Creation**

* **Target File:** `aura/src/rust/Cargo.toml`
* **Action:** Create this file if it does not exist. This file sits at the root of your Rust environment and governs everything inside `crates/`.

#### **2.2. The Code (Copy Exact)**

This content is derived directly from the source repository configuration to ensure we match the original build environment exactly.

```toml
[workspace]
resolver = "2"

members = [
    "crates/sentinel_core",
    "crates/sentinel_identity",
    "crates/sentinel_policy",
    "crates/sentinel_store",
    "crates/sentinel_api",
    "crates/sentinel_artifacts",
    "crates/sentinel_capabilities",
    "crates/sentinel_bench",
    "crates/sentinel_cli",
]

# OPTIMIZATION PROFILE: SECURE & LEAN
# These settings are critical for the Sentinel.
# We are sacrificing compile time for runtime safety and binary size.
[profile.release]
lto = true        # Link Time Optimization: Removes dead code, preventing "bloat" gadgets.
codegen-units = 1 # Forces single-threaded optimization. Slower build, but faster/safer code.
panic = "abort"   # FAIL CLOSED. If the Sentinel panics, it crashes immediately. No stack unwinding.
strip = true      # Symbols are stripped. Reduces binary size and makes reverse engineering harder.

```

#### **2.3. Why These Settings Matter (The "Why")**

You aren't just compiling code; you are compiling a security kernel.

* **`resolver = "2"`**: This is mandatory. It ensures that if `sentinel_api` needs a feature from `tokio` and `sentinel_store` needs a different feature, Cargo merges them correctly without duplication. Without this, you get weird "duplicate symbol" errors.
* **`panic = "abort"`**: This is a security feature. By default, Rust "unwinds" the stack on a crash to run cleanup code. In a high-security context like Sentinel, unwinding is a liability—it can be exploited or leak data. "Abort" kills the process instantly.
* **`lto = true`**: "Link Time Optimization." It analyzes the *entire* program (all 9 crates) as one unit to optimize across crate boundaries. This often results in a 10-20% performance boost for cryptographic operations.

#### **2.4. Verification**

After saving this file, run the following command in `aura/src/rust/` to verify the manifest syntax is valid (even before you move the crates):

```bash
cargo verify-project

```

* **Success Response:** `{"success":"true"}`
* **Failure Response:** Any TOML parsing error means a quote or bracket is missing.

3. The Crate Migration (The Physical Move)
Goal: Physically transfer the 9 crate directories into the new workspace so they sit side-by-side as siblings. This layout is mandatory because the code uses path = "../crate_name" to link them.

3.1. The Source and Destination
Source: Your unzipped sentinel-core/crates/ folder (wherever you have the old repo open).

Destination: aura/src/rust/crates/ (The clean directory you created in Step 1).

3.2. The Move List (9 Critical Organs)
You must move or copy these 9 specific folders from the source to the destination. Do not flatten them. Do not rename them.

sentinel_core (The Shared Language & Types)

sentinel_identity (Nonce Registry & Ed25519 Keys)

sentinel_policy (The Logic Engine)

sentinel_store (The Append-Only Ledger)

sentinel_api (The HTTP/IPC Server)

sentinel_artifacts (Provenance & Signatures)

sentinel_capabilities (Token Management)

sentinel_bench (Performance Verification)

sentinel_cli (Developer Tools / Manual Override)

3.3. Verification: The Anatomical Check
After the move, look at your VS Code file explorer. The structure MUST look exactly like this.

Plaintext

aura/
└── src/
    └── rust/
        ├── Cargo.toml          (The Workspace file from Step 2)
        └── crates/             (The container)
            ├── sentinel_api/
            ├── sentinel_artifacts/
            ├── sentinel_bench/
            ├── sentinel_capabilities/
            ├── sentinel_cli/
            ├── sentinel_core/
            ├── sentinel_identity/
            ├── sentinel_policy/
            └── sentinel_store/
3.4. Why This Exact Structure? (The Logic)
Open sentinel_api/Cargo.toml and look at the dependencies section. You will see lines like: sentinel_core = { path = "../sentinel_core" }

This tells the compiler: "Go up one directory (../), look for a folder named sentinel_core, and read the code there."

If you put sentinel_core inside another folder (e.g., crates/lib/sentinel_core), the link breaks.

If you rename it to core, the link breaks.

They must be siblings.

[ ] Action: Confirm visually that all 9 folders are present in aura/src/rust/crates/.


### **4. Dependency Alignment Check**

*Goal: Generate the master lockfile that binds all 9 crates to the same versions of shared dependencies (like `tokio`, `serde`, and `ed25519-dalek`) and verify that the workspace is correctly resolving local paths.*

#### **4.1. Generate the Lockfile**

The `Cargo.lock` file is the source of truth for reproducible builds. It does not exist yet in your new workspace.

**[ ] Action: Run the Check Command**
Open your terminal in `aura/src/rust/` and execute:

```bash
cargo check

```

**Expected Outcome:**

* **Console Output:** You should see a stream of green "Compiling..." or "Checking..." lines for dependencies (e.g., `libc`, `syn`, `quote`).
* **File System:** A new file named `Cargo.lock` will appear at `aura/src/rust/Cargo.lock`.
* **Failure Indicator:** If you see `error: failed to load source for dependency 'sentinel_core'`, it confirms a path mismatch in Step 3. Stop and re-verify folder names.

#### **4.2. Verify "Spine" Integrity (`sentinel_core`)**

We must confirm that the workspace recognizes `sentinel_core` as the foundational local crate, not an external download.

**[ ] Action: Inspect Core Dependencies**
Run this command:

```bash
cargo tree -p sentinel_core --depth 1

```

**Required Output:**
The output **MUST** show the local path (or just the version without a `registry` tag) and its key crypto dependencies:

```text
sentinel_core v0.1.0 (path+file:///.../aura/src/rust/crates/sentinel_core)
├── chrono v0.4.39
├── ed25519-dalek v2.1.1
├── serde v1.0.217
├── serde_json v1.0.134
├── sha2 v0.10.8
├── thiserror v2.0.9
└── uuid v1.11.0

```

*(Note: Exact version numbers may vary slightly based on the original lockfile, but they must be present.)*

#### **4.3. Verify "Nervous System" Linkage (`sentinel_api`)**

We must confirm that the API crate is successfully linking to the *local* sibling crates.

**[ ] Action: Inspect API Dependencies**
Run this command:

```bash
cargo tree -p sentinel_api --depth 1

```

**Required Output:**
The output **MUST** show the sibling crates as dependencies. If `sentinel_core` is missing from this tree or listed as a registry package, the transplant failed.

```text
sentinel_api v0.1.0 (path+file:///.../aura/src/rust/crates/sentinel_api)
├── actix-web v4.9.0
├── sentinel_core v0.1.0 (path+file:///.../aura/src/rust/crates/sentinel_core)
├── sentinel_identity v0.1.0 (path+file:///.../aura/src/rust/crates/sentinel_identity)
├── sentinel_policy v0.1.0 (path+file:///.../aura/src/rust/crates/sentinel_policy)
└── sentinel_store v0.1.0 (path+file:///.../aura/src/rust/crates/sentinel_store)

```

#### **4.4. Verify "Dev Tooling" Linkage (`sentinel_cli`)**

The CLI tool is your manual override. It must see everything.

**[ ] Action: Inspect CLI Dependencies**
Run this command:

```bash
cargo tree -p sentinel_cli --depth 1

```

**Required Output:**
It must link to `sentinel_core`, `sentinel_identity`, and `sentinel_store`.

```text
sentinel_cli v0.1.0 (path+file:///.../aura/src/rust/crates/sentinel_cli)
├── clap v4.5.23
├── sentinel_core v0.1.0 (...)
├── sentinel_identity v0.1.0 (...)
└── sentinel_store v0.1.0 (...)

```

**[ ] Action: Final Compilation Test**
If all trees look correct, run a full build to ensure no code-level errors exist in the new location:

```bash
cargo build --workspace

```

* **Success:** "Finished dev [unoptimized + debuginfo] target(s) in ..."
* **Note:** This compiles all 9 crates. If this passes, Phase 1 is complete.

5. Verify the "Spine" Crates
Goal: Confirm that the workspace correctly identifies the local sentinel_core crate as the foundation and that all other crates (API, CLI) are linking to this local version rather than trying to fetch it from the public registry.

5.1. Verify "Spine" Integrity (sentinel_core)
We must confirm that the workspace recognizes sentinel_core as the foundational local crate.

[ ] Action: Inspect Core Dependencies Run this command in your terminal:

Bash

cargo tree -p sentinel_core --depth 1
Required Output: The output MUST show the local path (indicated by path+file://) and its key cryptographic dependencies as defined in crates/sentinel_core/Cargo.toml.

Plaintext

sentinel_core v0.1.0 (path+file:///.../aura/src/rust/crates/sentinel_core)
├── chrono v0.4.39
├── ed25519-dalek v2.1.1
├── serde v1.0.217
├── serde_json v1.0.134
├── sha2 v0.10.8
├── thiserror v2.0.9
└── uuid v1.11.0
What to watch for: If the line sentinel_core v0.1.0 does not have the (path+file://...) suffix, Cargo is ignoring your local code. Check the members list in the root Cargo.toml.

5.2. Verify "Nervous System" Linkage (sentinel_api)
We must confirm that the API crate is successfully linking to the local sibling crates, specifically sentinel_core and sentinel_identity, as required by its manifest.

[ ] Action: Inspect API Dependencies Run this command:

Bash

cargo tree -p sentinel_api --depth 1
Required Output: The output MUST show the sibling crates as dependencies with local paths.

Plaintext

sentinel_api v0.1.0 (path+file:///.../aura/src/rust/crates/sentinel_api)
├── actix-web v4.9.0
├── sentinel_core v0.1.0 (path+file:///.../aura/src/rust/crates/sentinel_core)
├── sentinel_identity v0.1.0 (path+file:///.../aura/src/rust/crates/sentinel_identity)
├── sentinel_policy v0.1.0 (path+file:///.../aura/src/rust/crates/sentinel_policy)
└── sentinel_store v0.1.0 (path+file:///.../aura/src/rust/crates/sentinel_store)
Failure Indicator: If sentinel_core is missing from this tree or listed without a path, the API cannot see the "Constitutional Envelope" structs, and compilation will fail with "type not found" errors.

5.3. Verify "Dev Tooling" Linkage (sentinel_cli)
The CLI tool is your manual override. It must see the exact same versions of the crates as the API to ensure you are administering the real system.

[ ] Action: Inspect CLI Dependencies Run this command:

Bash

cargo tree -p sentinel_cli --depth 1
Required Output: It must link to sentinel_core, sentinel_identity, and sentinel_store.

Plaintext

sentinel_cli v0.1.0 (path+file:///.../aura/src/rust/crates/sentinel_cli)
├── clap v4.5.23
├── sentinel_core v0.1.0 (...)
├── sentinel_identity v0.1.0 (...)
└── sentinel_store v0.1.0 (...)
5.4. Final Compilation Test
If all trees look correct, run a full build to ensure the code physically compiles in the new location. This verifies that all internal references (like use sentinel_core::CanonicalEnvelopeAuthorizationRequest) are valid.

[ ] Action: Run Workspace Build Run this command:

Bash

cargo build --workspace
Success: "Finished dev [unoptimized + debuginfo] target(s) in ..."

Note: This compiles all 9 crates. If this passes, Phase 1: The Transplant is officially complete. The Sentinel's body is reassembled and biologically viable.


[ ] Migrate Core Crates

Copy the crates/ directory from sentinel-core into aura/src/rust/ (or your preferred backend path).

Crucial: Verify sentinel_core/src/lib.rs (the structs) is accessible to all other crates. It’s the shared language.

[ ] Dependency Alignment

Check the top-level Cargo.toml to ensure ed25519-dalek, serde, uuid, and chrono versions match across all moved crates.

Ensure sentinel_api properly references the local paths of the other crates (e.g., sentinel_policy = { path = "../sentinel_policy" }).

Phase 2: The Nervous System (API & IPC Wiring)
Goal: Connect the AURA Runtime (Electron/Python) to the Sentinel Heart. The Sentinel is useless if the Runtime can't "ask" it for permission.

[ ] Configure the Sentinel Server

In sentinel_api/src/main.rs, you currently have an Actix-web server. Decide: Keep HTTP or switch to FFI?

Recommendation: Keep the HTTP server (localhost:8000) for the MVP. It decouples the Rust safety layer from the Electron UI completely.

Update sentinel_api to bind to a local port defined in AURA_ENV variables.

[ ] Implement the "Choke Point" in Runtime

In the AURA Runtime (Python/Node), create a SentinelClient class.

Action: Before any tool execution, the Runtime must call SentinelClient.submit_envelope(payload).

Verify: Ensure the Runtime can successfully sign a payload with the User's private key and receive an ALLOW token from Rust.

[ ] Ledger Initialization

Ensure FileEventStore::open() in sentinel_api points to a persistent, secure directory in the user’s AURA profile (not /tmp).

Check: Verify that on startup, Sentinel replays the log and prints "Ledger Integrity Verified."

Phase 3: Constitutional Binding (Policy & Identity)
Goal: Enforce the "Laws" using the logic you’ve already built in sentinel_policy and sentinel_identity.

[ ] Load the Constitution (policies.yaml)

You have Sentinel-Law/policies.yaml. Move this to aura/config/policies.yaml.

Modify sentinel_api startup to load this file into the PolicyEngine.

Test: Send a request that violates a policy (e.g., "Delete Identity") and confirm sentinel_policy returns DENY.

[ ] Identity Genesis

The RiteOfUnbecoming and IdentityManager logic is in sentinel_identity.

Create a "Genesis" script in AURA that calls Sentinel to register the User (Actor) and generate the Master Key (IdentityEvent::ActorRegistered).

[ ] Capability Enforcement

Your sentinel_capabilities crate has the Capability struct.

Task: Modify the sentinel_api endpoints to require a valid Capability token in the header for sensitive routes (unless it's a genesis/login request).

Phase 4: The Missing Organs (Gap Filling)
Goal: Build the parts you noted as "Missing" or "Scaffold" in the Integration doc, specifically Coercion and Mediation.

[ ] Implement Coercion Detection (New Module)

Status: Missing in current crates.

Action: Create crates/sentinel_coercion.

Logic: Implement a simple RingBuffer of recent request hashes. If hash(request) appears > 3 times in 1 minute, flag PolicyInput with COERCION_ATTEMPT.

Wire this into sentinel_api middleware before Policy Evaluation.

[ ] Artifact Registry Integration

Status: sentinel_artifacts crate exists but needs wiring.

Action: Connect the sentinel_artifacts crate to the FileEventStore.

Policy: Enforce forever_law_provenance: Reject any ArtifactRegistered event if the signature is missing.

[ ] Execution Mediation (Sandboxing)

Status: Missing.

Action: In AURA Runtime, implementing the actual container/VM runner.

Sentinel Role: Sentinel doesn't run the container; it issues the token that allows the container to start. Ensure the Runtime checks this token.

Phase 5: Final Seal (Verification)
[ ] The "System Halt" Test

Create a policy that denies everything.

Try to use AURA. Result must be total paralysis. This proves Sentinel is the Law.

[ ] Audit Log Verification

Run a session.

Open sentinel_events.log.

Verify every click, thought, and tool use is cryptographically chained.