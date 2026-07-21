# Part II: The Implementation (Proof of Existence)

# Chapter 4: The Sentinel Protocol

---

The theoretical framework developed in Part I makes specific claims about how agentic systems can be governed: capabilities can be bound to context, execution velocity can be constrained, trajectories can be monitored, and interventions can be graduated. These claims are not self-evidently implementable. Many elegant theoretical frameworks founder on the gap between formalism and engineering.

This chapter bridges that gap. It describes the Sentinel Protocol, a concrete implementation of EGD principles in the AURA system. The goal is not to present Sentinel as the only possible implementation, or even the best one, but to demonstrate that EGD is *realizable*—that the mechanisms described in Chapters 2 and 3 can be built with existing technology, deployed in production environments, and evaluated against measurable criteria.

---

## 4.1 Architecture

Sentinel is designed around a single architectural principle: **local-first governance**.

The conventional approach to agent governance relies on centralized policy servers. An agent queries the server before acting; the server evaluates policy and returns an authorization decision. This architecture has two structural weaknesses that Sentinel avoids.

First, centralized servers create single points of failure and attack. If the policy server is compromised, all agents governed by it are compromised. If the server is unreachable, agents must either halt (availability failure) or proceed without governance (security failure). Neither outcome is acceptable for high-reliability deployments.

Second, centralized evaluation introduces latency. A round-trip to a remote server—even a fast one—adds tens to hundreds of milliseconds per decision. For agents making thousands of decisions per task, this latency compounds into unacceptable overhead. The temptation is to cache decisions, but caching reintroduces the static-authorization problem that EGD exists to solve.

Sentinel inverts this architecture. Governance logic executes locally, co-resident with the agent. The governance system observes agent behavior directly, without network round-trips. Policy is distributed to governance nodes in advance; real-time evaluation requires no external calls.

Centralized infrastructure still exists, but its role changes. It handles:

- Policy distribution (pushing updated governance rules to nodes)
- Aggregate telemetry (collecting Gamma scores and intervention events)
- Audit log anchoring (providing tamper-evidence for local records)
- Credential issuance (generating CBIG-compatible capability tokens)

None of these functions are in the critical path of agent execution. If central infrastructure is temporarily unavailable, local governance continues with cached policy and local logging. Availability degrades gracefully rather than failing catastrophically.

---

**Component Architecture**

Sentinel comprises four components, each corresponding to a theoretical construct from Part I:

**1. The Context Evaluator**

The Context Evaluator implements CBIG. At startup and periodically during execution, it measures the agent's environmental state:

- Hardware attestation via TPM 2.0 or platform-specific secure enclave
- Software integrity via hash verification of loaded binaries
- Network position via authenticated assertions from infrastructure
- Temporal state via secure time sources

These measurements are combined into a context signature $C$. The Context Evaluator maintains the agent's share of CBIG key material and performs capability key derivation when the agent attempts protected operations.

If the current context does not match the context specified at capability issuance, key derivation fails. The operation cannot proceed—not because it is blocked by policy, but because the cryptographic material required to execute it cannot be computed.

**2. The Trajectory Monitor**

The Trajectory Monitor implements Gamma scoring. It observes agent behavior through instrumentation hooks:

- Action logging: every tool invocation, API call, and resource access
- State tracking: memory allocation, file handles, network connections
- Output capture: generated text, code, and structured data

From these observations, it computes the five Gamma components in real-time:

- $\Gamma_a$ (action entropy): rolling entropy over the last $n$ actions, compared to baseline distribution
- $\Gamma_r$ (resource velocity): rate of resource consumption, normalized against historical patterns
- $\Gamma_s$ (scope expansion): Jaccard distance between current resource access set and original task scope
- $\Gamma_i$ (reversibility index): weighted sum of action irreversibility scores
- $\Gamma_h$ (human latency): time since last human checkpoint, scaled by task criticality

The composite $\Gamma$ score is updated after every action. When $\Gamma$ crosses a threshold, the Trajectory Monitor signals the Intervention Controller.

**3. The Intervention Controller**

The Intervention Controller implements graduated response. It maintains the agent's current governance level (0–5) and enforces the constraints appropriate to that level.

At Level 0, the controller is passive—it logs but does not interfere.

At higher levels, it interposes on agent actions:

- Level 1 (Friction): Artificial delays inserted before action execution
- Level 2 (Confirmation): Actions held pending automated policy check or human approval
- Level 3 (Restriction): Capability tokens for high-risk operations invalidated
- Level 4 (Supervision): All actions routed through human review queue
- Level 5 (Suspension): Execution halted; agent placed in safe state

The controller also handles de-escalation. When $\Gamma$ decreases and remains below threshold for a configurable duration, the governance level steps down. The agent regains autonomy as its trajectory returns to normal bounds.

**4. The Flight Recorder**

The Flight Recorder implements the audit subsystem. Every action, Gamma computation, and intervention event is logged to an append-only local store. Logs are structured as a hash chain: each entry includes the hash of the previous entry, creating a tamper-evident sequence.

Periodically, the Flight Recorder computes a Merkle root over recent log entries and submits it to a central anchoring service. The anchoring service aggregates roots from multiple agents and publishes them to a transparency log (or, in high-assurance deployments, a blockchain).

This architecture provides:

- **Local completeness**: All data required for audit is stored locally
- **Tamper evidence**: Any modification to historical logs breaks the hash chain
- **Non-repudiation**: Anchored roots prove that logs existed at a specific time
- **Offline resilience**: Logging continues even if anchoring is temporarily unavailable

The Flight Recorder is the agent's "black box." In the event of an incident, it provides a complete, verifiable record of what the agent did, what Gamma scores it produced, and what interventions occurred.

---

**Deployment Model**

Sentinel is deployed as a sidecar process co-located with the agent. It shares no memory with the agent; it observes through well-defined instrumentation interfaces. This isolation ensures that a compromised agent cannot tamper with its own governance.

In containerized environments, Sentinel runs in a separate container within the same pod, with read-only access to the agent's logs and metrics. In VM-based deployments, Sentinel runs as a privileged process with access to the agent's system calls via eBPF or equivalent kernel instrumentation.

The governance code itself is minimal and formally auditable. The Trajectory Monitor and Intervention Controller together comprise approximately 3,000 lines of Rust. The Context Evaluator depends on platform-specific attestation libraries but its core logic is under 1,000 lines. The Flight Recorder is similarly compact.

This compactness is intentional. Sentinel is the safety controller in the Simplex sense: it must be simple enough to verify, robust enough to trust, and conservative enough to fail safe.

---

## 4.2 Misuse Economics

A governance mechanism is only as valuable as the cost it imposes on adversaries. Sentinel's design is grounded in explicit economic reasoning about attack costs.

---

**The Baseline: Ungoverned Agents**

Consider an adversary who gains control of an AI agent—through prompt injection, credential theft, or insider access. In an ungoverned system, the adversary's costs are:

- **Acquisition cost ($C_a$)**: The one-time cost to compromise the agent
- **Marginal cost per action ($C_m$)**: The incremental cost to execute each additional unauthorized action

For most AI systems today, $C_m \approx 0$. Once an agent is compromised, executing actions is essentially free. The adversary can exfiltrate data, modify configurations, or cause damage at the rate the agent can operate—potentially thousands of actions per second.

The attack scales trivially. Harm is proportional to attack duration: $H(t) = r \cdot t$, where $r$ is the harm rate and $t$ is time. An adversary who maintains access for twice as long causes twice as much damage.

---

**The EGD Cost Function**

Sentinel introduces three cost terms that did not exist in the baseline:

**1. Context Binding Cost ($C_c$)**

CBIG requires that capabilities be exercised within the context where they were issued. An adversary who compromises an agent cannot simply exfiltrate capabilities to a different environment. To use stolen capabilities, the adversary must either:

- Maintain persistent access to the original environment (increasing exposure to detection)
- Spoof the context measurements (requiring compromise of trusted hardware)

Both options increase $C_a$ substantially. Context binding does not prevent compromise, but it prevents capability portability. Stolen credentials cannot be exercised at scale from an adversary's infrastructure.

**2. Temporal Cost ($C_t$)**

VDF-based rate shaping imposes a minimum time cost per protected operation. If the VDF parameter is $T$ seconds, executing $n$ operations requires at least $n \cdot T$ seconds of wall-clock time.

This transforms the harm function. Instead of $H(t) = r \cdot t$, harm becomes $H(t) = r \cdot \lfloor t/T \rfloor$. The adversary cannot accelerate harm by adding compute; time is the bottleneck.

More importantly, duration increases detection probability. If Sentinel's detection latency is $D$ seconds, an attack lasting $n \cdot T$ seconds has $n \cdot T / D$ opportunities for detection. The probability of undetected completion decreases exponentially with attack scale.

**3. Escalation Cost ($C_e$)**

Graduated intervention means that aggressive agent behavior triggers governance escalation. An adversary attempting to cause rapid harm will elevate Gamma, triggering friction, confirmation requirements, and eventually suspension.

To avoid escalation, the adversary must operate slowly and within normal behavioral bounds—which limits the rate of harm. To operate aggressively, the adversary must accept escalation—which increases detection probability and may trigger suspension before objectives are achieved.

This creates a **detection-damage tradeoff**: the adversary can cause harm quickly (and be detected) or slowly (and cause less harm before detection). There is no strategy that achieves both high damage and low detection.

---

**The Composite Cost Model**

The total cost to an adversary attempting to cause harm $H$ is:

$$C_{total}(H) = C_a + C_c + C_t(H) + C_e(H)$$

where $C_t(H)$ and $C_e(H)$ are increasing functions of harm.

Under EGD governance:

- $C_a$ increases due to context binding requirements
- $C_c$ is non-zero (versus zero for portable credentials)
- $C_t(H)$ grows linearly with intended harm
- $C_e(H)$ grows superlinearly as escalation triggers compounding friction

The critical property is that **attack cost scales with attack ambition**. An adversary who wants to cause twice the harm must pay more than twice the cost—in time, in exposure, in probability of detection.

This is the economic foundation of EGD: make misuse expensive, not impossible.

---

**Empirical Validation**

The economic model makes testable predictions:

1. **Prediction**: Attack duration should increase linearly with VDF parameter $T$.
   **Test**: Measure time-to-completion for standardized attack scenarios across different $T$ values.

2. **Prediction**: Detection rate should increase with attack aggressiveness.
   **Test**: Measure Gamma elevation and intervention frequency across attack intensity levels.

3. **Prediction**: Rational adversaries should prefer low-and-slow strategies.
   **Test**: Analyze attack patterns in red-team exercises for velocity/stealth tradeoffs.

These predictions are falsifiable. If empirical testing shows that adversaries can cause high harm without elevated cost, the model fails. If testing confirms the predictions, the model is validated.

Sentinel includes instrumentation specifically designed to collect this data. Every deployment generates economic metrics that can be aggregated and analyzed.

---

## 4.3 Immutable Memory

Governance is only as trustworthy as its records. A system that claims to monitor agent behavior but cannot prove what occurred is a system that can be gaslit—by adversaries, by malfunctioning agents, or by its own operators.

Sentinel addresses this through cryptographically immutable logging.

---

**The Hash Chain**

Every log entry in the Flight Recorder includes:

- Timestamp (from authenticated time source)
- Event type (action, gamma update, intervention, checkpoint)
- Event payload (structured data specific to event type)
- Hash of previous entry

The hash of entry $n$ is computed as:

$$h_n = H(t_n \| e_n \| p_n \| h_{n-1})$$

where $H$ is a cryptographic hash function (SHA-256 in the reference implementation).

This structure has a specific property: modifying any historical entry changes its hash, which invalidates all subsequent entries. To falsify the record, an adversary must recompute the entire chain from the point of modification—and must do so before the original chain is verified or anchored.

---

**Merkle Anchoring**

Periodically (by default, every 60 seconds), the Flight Recorder computes a Merkle root over recent log entries. This root is a single hash that commits to all entries in the period.

The root is submitted to a central anchoring service, which:

1. Collects roots from all Sentinel instances
2. Computes a global Merkle root over all submitted roots
3. Publishes the global root to a transparency log

The transparency log is append-only and publicly auditable. Anyone can verify that a specific local root was included in a specific global root at a specific time.

This provides **non-repudiation with minimal trust**:

- The agent cannot deny actions that are recorded in its local chain
- The operator cannot deny that the local chain existed at anchoring time
- The anchoring service cannot falsify inclusions without detection

For high-assurance deployments, the global root can be published to a public blockchain, providing cryptographic proof of existence that does not depend on any single party's honesty.

---

**Forensic Reconstruction**

In the event of an incident, the Flight Recorder enables complete trajectory reconstruction.

Given a time range, analysts can:

1. Retrieve all log entries in the range
2. Verify hash chain integrity (detecting any tampering)
3. Verify Merkle inclusion against anchored roots (proving the logs existed at claimed time)
4. Replay the sequence of actions, Gamma scores, and interventions
5. Identify the exact point where governance escalated (or failed to escalate)

This reconstruction is deterministic. Two analysts examining the same logs will reach identical conclusions about what occurred. There is no ambiguity, no "the logs might have been modified," no "we don't know what the agent was thinking."

The agent's decisions are not observable directly—it is a black box in the neural network sense. But its *actions* are fully observable, and the governance system's *responses* to those actions are fully observable. The Flight Recorder captures both.

---

**Privacy Considerations**

Comprehensive logging raises privacy concerns. If an agent processes sensitive data, that data may appear in logs.

Sentinel addresses this through **log stratification**:

- **Layer 0 (Governance)**: Action types, Gamma scores, intervention events. No payload data. Always logged.
- **Layer 1 (Operational)**: Action parameters, resource identifiers, truncated outputs. Logged by default, redactable.
- **Layer 2 (Forensic)**: Full payloads, complete outputs, memory snapshots. Logged only when Gamma exceeds threshold.

Layer 0 logs are always sufficient to verify governance behavior. Layer 1 logs support operational debugging. Layer 2 logs are reserved for incident investigation and are subject to retention limits and access controls.

The hash chain covers all layers, but the actual data can be stratified by sensitivity. An analyst can verify that "an action occurred at time $t$" without having access to the action's payload.

---

## Summary

The Sentinel Protocol demonstrates that EGD is implementable.

**Architecture**: A local-first design with four components (Context Evaluator, Trajectory Monitor, Intervention Controller, Flight Recorder) that map directly to the theoretical constructs of Part I. Central infrastructure is not in the critical path; governance continues during network partitions.

**Economics**: A cost model showing that EGD increases attack cost linearly with attack ambition. Context binding prevents capability portability; rate shaping imposes temporal cost; graduated intervention creates a detection-damage tradeoff.

**Auditability**: Hash-chain logging with Merkle anchoring provides tamper-evident, non-repudiable records. Complete trajectory reconstruction is possible for any incident.

Sentinel is not the only possible implementation of EGD. Different platforms, threat models, and operational requirements will produce different architectures. But Sentinel proves that *an* implementation exists—that the mechanisms described in Part I are not merely theoretical but buildable, deployable, and measurable.

---

The theoretical framework is defined. The implementation is demonstrated.

What remains is to examine the broader implications: how EGD generalizes beyond AURA, how it relates to existing governance frameworks, and what its adoption implies for the future of autonomous systems.

That is the work of Part III.
