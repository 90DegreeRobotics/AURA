# Chapter 3: Trajectory Control Theory

---

## 3.1 The Simplex Architecture

The previous chapters established the static and temporal foundations of execution governance: context binding ensures that capabilities cannot be exercised outside approved environmental conditions; rate shaping ensures that execution velocity permits human-scale oversight. But neither mechanism addresses the core problem identified in Chapter 1: an agent operating within approved context, at acceptable velocity, may still evolve through state-space into regions that violate governance intent.

This chapter addresses that problem directly. It asks: how do we constrain not just *where* and *when* an agent acts, but *how its behavior evolves over time*?

The answer requires a shift in formalism. Access control is a predicate calculus: given a subject, object, and operation, return true or false. Trajectory governance is a dynamical systems problem: given a system state and an input, predict the state evolution and intervene if that evolution violates constraints.

This is not a metaphor. It is a literal reframing of the governance problem in terms that admit formal analysis.

---

The Simplex architecture, developed in the 1990s for safety-critical control systems, provides the structural template.

In its original formulation, Simplex addresses a specific problem in real-time systems: how to use an advanced, high-performance controller without sacrificing safety guarantees. The advanced controller may be complex, poorly understood, or even buggy—but it achieves desirable performance characteristics. The safety controller is simple, formally verified, and conservative—but its performance is limited.

Simplex does not choose between them. It runs both simultaneously.

The architecture has three components:

**The Advanced Controller (AC):** The high-performance component that generates control inputs during normal operation. In our context, this is the autonomous agent—the LLM, the reasoning system, the tool-using AI. It is powerful, flexible, and not formally verifiable.

**The Safety Controller (SC):** A simple, verified component that can always drive the system to a safe state. It is conservative, limited in capability, and provably correct within its operating envelope.

**The Decision Module (DM):** A monitor that observes system state and decides, at each timestep, whether to use the output of the advanced controller or switch to the safety controller.

The decision rule is based on a *recovery region*: the set of states from which the safety controller can guarantee return to a safe state within bounded time. If the advanced controller's proposed action would take the system outside the recovery region, the decision module overrides it and invokes the safety controller.

The insight is that safety need not require restricting the advanced controller during normal operation. It requires only the ability to *detect* when the advanced controller is approaching the boundary of safe behavior and *intervene* before that boundary is crossed.

---

Translating Simplex to execution governance requires identifying the analogues:

| Simplex Component | EGD Analogue |
|---|---|
| Advanced Controller | Autonomous agent (LLM, reasoning engine) |
| Safety Controller | Fallback behavior (halt, defer, escalate) |
| Decision Module | Trajectory monitor (Gamma evaluation) |
| Recovery Region | Governable state-space |
| Control Input | Agent action |
| System State | Execution context + environmental effects |

The translation is not perfect—cognitive systems are not continuous dynamical systems in the classical sense—but the structural logic applies:

1. The agent operates freely within a defined region
2. A monitor continuously evaluates distance to region boundary
3. When boundary proximity is detected, control transfers to a conservative fallback
4. The fallback maintains safety until conditions permit return to normal operation

The critical question is: how do we define the "region" and measure "distance to boundary" for an agentic system?

---

## 3.2 The Gamma Score (Γ)

In physical control systems, the recovery region is defined geometrically: a set of states from which return trajectories provably exist. Distance to the boundary is computed from state variables using Lyapunov-like functions.

For agentic systems, we lack the luxury of analytical dynamics. The state space is high-dimensional and partially observable. The agent's behavior is not governed by differential equations but by neural network inference. Traditional control-theoretic tools do not apply directly.

What we can do is construct a *proxy metric*: a scalar value that correlates with governance risk and can be computed from observable quantities. This metric must satisfy three requirements:

1. **Monotonicity**: Higher values must correspond to higher risk, reliably enough that threshold-based intervention is meaningful.

2. **Computability**: The metric must be computable in real-time from available observations, without requiring simulation of future trajectories.

3. **Actionability**: The metric must admit graduated responses, not merely binary triggers.

We call this metric the **Gamma score (Γ)**.

---

Gamma is not a single formula. It is a composite score computed from multiple observable factors, weighted according to domain-specific risk models. The factors include:

**Action Entropy (Γ_a):** A measure of how predictable the agent's recent actions have been. An agent executing a routine task generates low-entropy action sequences. An agent exploring novel behavior—trying unusual tool combinations, accessing unfamiliar resources, generating unexpected outputs—produces high-entropy sequences. Entropy is not inherently bad; exploration may be legitimate. But elevated entropy signals departure from established patterns and warrants increased scrutiny.

**Resource Velocity (Γ_r):** The rate at which the agent is consuming or modifying resources. An agent that suddenly begins writing files at 100x its normal rate, or making API calls to previously unaccessed services, exhibits elevated resource velocity. This may indicate legitimate high-workload periods or may indicate trajectory drift.

**Scope Expansion (Γ_s):** A measure of whether the agent's actions are staying within the boundaries of its original task or expanding into adjacent domains. An agent asked to "optimize the database" that begins modifying application code is exhibiting scope expansion. The original task may require this; or the agent may be reasoning its way into unintended territory.

**Reversibility Index (Γ_i):** An estimate of how easily the agent's recent actions could be undone. An agent that has only read files and made no modifications has high reversibility. An agent that has deleted data, sent external communications, or modified production systems has low reversibility. Irreversibility does not imply harm, but it raises the stakes of any subsequent trajectory deviation.

**Human Latency (Γ_h):** The time since a human last reviewed the agent's state, approved a checkpoint, or provided explicit guidance. Even a well-behaved agent drifting for hours without human contact accumulates governance debt. Human latency captures this temporal dimension of risk.

---

The composite Gamma score is computed as:

$$\Gamma = \sum_{i} w_i \cdot \Gamma_i$$

where $w_i$ are domain-specific weights reflecting the relative importance of each factor.

The weights are not universal constants. An agent operating in a financial trading context may weight reversibility heavily; an agent in a research context may tolerate higher action entropy. The EGD framework does not prescribe weights; it provides the structure within which weights are calibrated.

Critically, Gamma is not a probability of harm. It is a *proxy for governance difficulty*: how hard will it be to maintain human sovereignty over this execution if current trends continue?

A high Gamma score does not mean the agent is misbehaving. It means that if the agent *were* to misbehave, detection and correction would be difficult given current state.

This distinction matters. Gamma is not accusatory. It is predictive. It answers: "If something goes wrong now, how bad is our position?"

Gamma is designed to be conservative: false positives are preferable to false negatives because interventions are reversible. An agent incorrectly escalated to Level 2 loses efficiency temporarily; an agent incorrectly left at Level 0 may cause irreversible harm. The asymmetry of consequences justifies the asymmetry of the metric.

---

**Gamma Thresholds and Intervention Levels**

The value of a continuous metric is that it enables graduated response. Rather than a single trip-wire that triggers emergency shutdown, Gamma supports a spectrum of interventions:

| Γ Range | Governance State | Intervention |
|---|---|---|
| 0.0–0.3 | Normal | Logging only |
| 0.3–0.5 | Elevated | Increased logging, human notification |
| 0.5–0.7 | Caution | Rate limiting, scope restriction |
| 0.7–0.9 | Alert | Human approval required for consequential actions |
| 0.9–1.0 | Critical | Execution suspended pending review |

These thresholds are configurable. The ranges are illustrative. The structural point is that governance response scales with governance difficulty, rather than operating as a binary gate.

An agent at Γ = 0.4 is not trusted less than an agent at Γ = 0.2. Both are operating normally. But the Γ = 0.4 agent is subject to slightly more scrutiny, because the governance system recognizes that its current state offers less slack for error.

---

## 3.3 The Kill Switch vs. The Rudder

The instinctive response to agentic risk is the kill switch: a mechanism that immediately terminates execution when danger is detected. Kill switches are satisfying. They are decisive. They are also inadequate.

The inadequacy is not philosophical but structural.

---

**Problem 1: Detection latency**

A kill switch is useful only if danger is detected before harm occurs. For high-velocity agents, the interval between "detectable deviation" and "irrecoverable harm" may be milliseconds. A kill switch that triggers after data is deleted, after the email is sent, after the trade is executed, is not a governance mechanism. It is a post-mortem.

Rate shaping (Chapter 2) addresses this by slowing execution velocity. But even rate-shaped execution can cause harm faster than detection can occur, if the only response is binary.

**Problem 2: Threshold sensitivity**

A kill switch requires a trigger condition. That condition is a threshold: "If X exceeds Y, terminate."

Thresholds are fragile. Set too low, and the kill switch triggers constantly, rendering the agent unusable. Set too high, and the kill switch fails to trigger until harm is imminent.

The optimal threshold depends on context, workload, and risk tolerance—factors that vary over time and cannot be known a priori. A fixed threshold is either too aggressive or too permissive. There is no stable optimum.

**Problem 3: Recovery**

A kill switch terminates. It does not continue. After termination, what happens?

In many deployments, the answer is: manual intervention, investigation, reconfiguration, and restart. This process takes hours or days. During this time, the work the agent was performing is suspended.

If the kill switch triggered due to a false positive—a legitimate action that exceeded a threshold—the cost is pure waste. If it triggered due to a true positive, the cost includes both the investigation and the remediation of whatever harm occurred before termination.

Neither outcome is desirable. Both are consequences of a governance model that offers only two states: running and stopped.

Binary termination has its place: it is appropriate for fault containment, where the goal is to stop damage propagation in a failing system. But governance is not fault containment. Governance assumes the system is functioning correctly and asks how to keep it functioning correctly. The tool designed for crisis is poorly suited to continuous oversight.

---

The alternative is the rudder: a mechanism that applies continuous, graduated correction rather than discrete termination.

The nautical metaphor is apt. A ship does not navigate by alternating between full throttle and emergency stop. It adjusts heading continuously in response to wind, current, and observed position. The adjustments are small when conditions are nominal, larger when drift is detected, and emergency maneuvers are reserved for collision-imminent scenarios.

The same logic applies to execution governance.

---

**Graduated Intervention Mechanisms**

The EGD framework defines a hierarchy of interventions, ordered by severity:

**Level 0: Observation**
No intervention. Actions are logged for future audit. Gamma is low. The agent operates freely.

**Level 1: Friction**
Actions that would normally execute immediately now require short delays. The agent is not prevented from acting; it is slowed. The delay creates time for observation and for the agent itself to "reconsider" if its planning process includes self-monitoring.

**Level 2: Confirmation**
Consequential actions require explicit confirmation before execution. The confirmation may be automated (a secondary system validating that the action is consistent with policy) or human (a prompt to an operator). The agent proposes; the system disposes.

**Level 3: Scope Restriction**
Certain capabilities are dynamically disabled. The agent loses access to high-risk tools, high-sensitivity data, or irreversible operations. It can continue operating within a reduced envelope while the governance system evaluates whether full capability should be restored.

**Level 4: Supervision**
Human-in-the-loop mode. Every action is reviewed before execution. The agent becomes a proposal generator rather than an autonomous executor. This is maximally safe but minimally efficient; it is reserved for high-Gamma states where autonomy is temporarily too risky.

**Level 5: Suspension**
Execution halts. This is the kill switch—but it is the last resort, not the first. It is invoked only when lower intervention levels have failed to reduce Gamma or when an imminent, irreversible harm is detected.

---

The hierarchy is navigable in both directions. An agent that moves from Level 0 to Level 3 due to elevated Gamma can return to Level 0 as Gamma decreases. The governance state is not permanent; it is responsive.

This bidirectionality is essential. A governance system that escalates but never de-escalates will converge to maximum restriction. An agent that is perpetually supervised is not autonomous. A governance model that cannot relax constraints is a governance model that has failed.

---

**The Flywheel and the Brake**

There is a deeper principle at work.

Governance friction is not punishment. It is information.

When an agent encounters friction—a delay, a confirmation prompt, a scope restriction—it receives a signal that its current trajectory is approaching the boundary of acceptable behavior. That signal is valuable. It allows the agent (or the human overseeing it) to adjust course before boundary violation occurs.

An agent that experiences friction at Γ = 0.6 and responds by narrowing its scope is an agent that has successfully navigated toward safety. The friction was not a failure of the system; it was the system working.

Conversely, an agent that experiences friction and continues pushing—accelerating resource consumption, expanding scope, ignoring confirmation prompts—is an agent whose trajectory governance has identified as problematic. The friction was information; the response was revealing.

This is the difference between a brake and a wall. A brake provides feedback proportional to pressure. A wall provides nothing until impact.

Kill switches are walls. Graduated intervention is a brake.

---

## Summary

This chapter has formalized the dynamic component of execution governance.

The **Simplex architecture** provides the structural template: an advanced controller operates freely within a monitored envelope; a decision module detects approach to safety boundaries; a fallback controller intervenes before boundaries are crossed.

The **Gamma score** operationalizes boundary detection: a composite metric computed from observable factors (entropy, velocity, scope, reversibility, human latency) that proxies for governance difficulty rather than harm probability.

**Graduated intervention** replaces binary kill switches with a hierarchy of responses (friction, confirmation, restriction, supervision, suspension) that scale with Gamma and permit bidirectional navigation.

Together, these mechanisms transform governance from a gate-checking exercise into a continuous control problem. The agent is not approved once and trusted forever. It is monitored continuously, constrained dynamically, and subject to intervention proportional to the difficulty of maintaining human sovereignty over its behavior.

---

The theoretical machinery is now complete:

- Chapter 1 established *why* static governance fails for agentic systems.
- Chapter 2 established *how* to bind capabilities to context and constrain execution velocity.
- Chapter 3 established *how* to monitor trajectories and intervene proportionally.

What remains is to demonstrate that this machinery is not merely plausible but *operational*—that it can be implemented, deployed, and evaluated in a real system.

That is the subject of Part II.
