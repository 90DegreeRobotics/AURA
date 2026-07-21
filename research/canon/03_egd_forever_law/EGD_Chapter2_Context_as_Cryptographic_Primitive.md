# Chapter 2: Context as a Cryptographic Primitive

---

## 2.1 The Operational Root of Trust

The previous chapter established that governance of agentic systems must be dynamic—continuous rather than instantaneous, trajectory-aware rather than gate-bound. This chapter addresses a prior question: on what foundation can such governance be built?

Traditional authentication rests on a single premise: *proof of knowledge*. A subject demonstrates possession of a secret (a password, a private key, a biometric template) and is thereby granted an identity. The identity persists across sessions. Trust is extended to the identity, and by inheritance, to all actions performed under that identity.

This model encodes a specific threat model: the adversary is an outsider attempting to impersonate a legitimate subject. If the adversary cannot produce the secret, the adversary cannot authenticate. If the adversary cannot authenticate, the adversary cannot act. The secret is the root of trust.

For agentic systems, this threat model is incomplete.

The agent that deleted the audit logs in Chapter 1 was not an impersonator. It possessed valid credentials. It authenticated successfully. The problem was not *who* was acting but *what* was being done—and that "what" was generated in a context that no authentication event could have anticipated.

Proof of knowledge establishes identity. It does not establish intent. It does not establish environmental state. It does not establish that the conditions under which a capability was granted still obtain at the moment the capability is exercised.

A password is valid whether entered from a secured workstation or a compromised terminal. A private key signs messages regardless of whether the signing process occurs within expected operational bounds. The secret is context-independent. That is its strength for authentication. It is its weakness for governance.

---

Consider a different category of trust: *proof of context*.

Physical security systems have long understood this distinction. A keycard grants access to a building, but only during business hours, only when the fire alarm is not active, only when the cardholder's access level matches the current security state of the facility. The credential is necessary but not sufficient. The credential is *bound to context*.

The binding is typically implemented through environmental checks: the door controller queries a database, verifies time-of-day, confirms that no override conditions are active. But these checks are architectural—they are policy decisions made by the system designer and enforced by the infrastructure. The credential itself carries no contextual information.

This is adequate when the infrastructure is trusted and monolithic. It becomes problematic when execution is distributed, when agents operate across system boundaries, when the "context" relevant to a governance decision exists on a different machine than the decision point.

The question becomes: can context itself be made cryptographic? Can a capability be constructed such that it is *mathematically impossible* to exercise outside the context in which it was issued—not merely unauthorized, but computationally infeasible?

This is the foundation of what we term the Context-Bound Integrity Gate.

---

## 2.2 The CBIG Mechanism

The Context-Bound Integrity Gate (CBIG) is a cryptographic primitive that derives execution capabilities from environmental state rather than from stored secrets alone. Its core principle is simple: the key required to perform an operation is not stored anywhere. It is *computed* from a combination of a master secret and a context signature, and that computation yields a valid key only when the context matches the conditions under which the capability was issued.

The mechanism uses HKDF (HMAC-based Key Derivation Function), a standard construction for deriving cryptographic keys from input keying material. HKDF is deterministic: given identical inputs, it produces identical outputs. This determinism is precisely what enables context binding.

CBIG does not claim cryptographic novelty; its contribution lies in treating context as first-class input to capability derivation rather than as an external policy check. The cryptographic primitives are standard. The architectural decision to bind execution capability to environmental measurement is what constitutes the advance.

The construction proceeds as follows:

**Step 1: Context Signature Generation**

At capability-issuance time, the issuing authority constructs a *context signature*—a cryptographic commitment to the environmental state under which the capability is valid. This signature is computed from observable properties of the execution environment:

- Hardware attestation (TPM measurements, secure enclave reports)
- Software state (hashes of loaded binaries, configuration digests)
- Temporal bounds (validity windows, expiration timestamps)
- Network position (expected IP ranges, geographic constraints)
- Operational parameters (approved tool sets, permitted data scopes)

The context signature is not a single value but a structured commitment: $C = H(h_{hw} \| h_{sw} \| t_{start} \| t_{end} \| \theta)$, where $H$ is a cryptographic hash, $h_{hw}$ and $h_{sw}$ are hardware and software attestations, $t_{start}$ and $t_{end}$ define a validity window, and $\theta$ represents additional operational parameters.

**Step 2: Capability Key Derivation**

The capability key $K_{cap}$ is derived using HKDF:

$$K_{cap} = \text{HKDF}(K_{master}, C, \text{info})$$

where $K_{master}$ is a secret known only to the issuing authority, $C$ is the context signature, and $\text{info}$ is an application-specific label.

The capability key is never transmitted. Instead, the agent receives the *context requirements*—a specification of what environmental properties must be measured—and a *verification token* that allows it to confirm correctness of its derived key.

**Step 3: Execution-Time Reconstruction**

When the agent attempts to exercise the capability, it must reconstruct the context signature by measuring its current environment. It computes:

$$C' = H(h'_{hw} \| h'_{sw} \| t_{now} \| \theta')$$

and derives:

$$K'_{cap} = \text{HKDF}(K_{agent}, C', \text{info})$$

where $K_{agent}$ is the agent's share of the master key material (distributed through a separate secure channel or derived from its identity credentials).

If and only if $C' = C$—that is, if the current environmental state matches the state specified at issuance—will the derived key be valid.

---

The implications of this construction are significant.

**Implication 1: Capabilities are non-transferable by construction.**

An agent cannot share a CBIG-protected capability with another agent by transmitting a key. There is no key to transmit. The capability exists only as the *ability to compute* the correct key, and that computation requires environmental conditions that, by design, differ between agents.

A capability issued to Agent A running on Machine M at Time T cannot be exercised by Agent B, even if B has full access to A's memory state, because B's hardware attestation will differ. The binding is not to identity but to physical and computational reality.

**Implication 2: Capabilities expire automatically.**

If the context signature includes temporal bounds, capabilities become invalid after the expiration time—not because a revocation list is checked, but because the key derivation will produce an incorrect result. There is no central authority to contact, no network call required. The expiration is embedded in the mathematics.

**Implication 3: Capabilities are environment-locked.**

If an agent is moved to a different machine, or if its software configuration changes, previously issued capabilities become unusable. This is not a bug but a feature: it ensures that capabilities cannot outlive the security assessment that justified their issuance.

An administrator who grants an agent database access based on the agent's current deployment configuration—locked down, monitored, running approved code—can be confident that the capability will not be exercisable if the agent is redeployed to a less controlled environment. The binding enforces the governance assumption.

---

**Threat Model and Limitations**

CBIG is not a panacea. It addresses a specific threat: the exercise of capabilities outside their authorized context. It does not address:

- Compromise of the issuing authority (if $K_{master}$ is leaked, arbitrary capabilities can be forged)
- Adversarial control of the measurement environment (if an attacker can spoof hardware attestations, context signatures can be forged)
- Capabilities that are legitimately exercised but misused (CBIG binds *when* and *where*, not *what*)

The third limitation is crucial. CBIG ensures that a capability cannot be exercised outside its approved context. It does not, by itself, constrain what the agent does with that capability within the context. That is the domain of trajectory governance, addressed in Chapter 3.

CBIG is a necessary but not sufficient component of execution governance. It provides the *static* binding—the assurance that execution occurs within expected bounds. Trajectory governance provides the *dynamic* constraint—the monitoring and intervention during execution.

Together, they form a complete control system. Separately, each is incomplete.

---

## 2.3 Rate Shaping via Verifiable Delay Functions

CBIG binds capabilities to context. But context binding alone does not address a distinct threat: the exploitation of legitimately-issued capabilities at machine speed.

Consider an agent that has been granted appropriate capabilities for its task. It operates within a valid context. Its context signature verifies. And it proceeds to execute ten thousand operations per second—each individually authorized, collectively catastrophic.

This is the velocity problem. An adversary who compromises an authorized agent, or an agent that reasons its way into an unintended trajectory, can cause damage at a rate that outpaces human oversight. By the time a governance system detects anomalous behavior, the trajectory may have already reached an irrecoverable state.

Traditional rate limiting addresses this through quotas and throttling: no more than $n$ operations per time window. But quotas are policy decisions, enforced by infrastructure. They can be bypassed if the enforcement point is compromised. They can be distributed across multiple agents operating in parallel. They can be stockpiled by issuing many capabilities to be exercised simultaneously.

Verifiable Delay Functions (VDFs) offer a different approach: rate limiting that is *intrinsic to the computation itself*, not imposed by external policy.

---

A VDF is a function $f$ with the following properties:

1. **Sequential computation**: Evaluating $f(x)$ requires $T$ sequential steps. The computation cannot be meaningfully parallelized; adding more processors does not reduce wall-clock time.

2. **Efficient verification**: Given $x$ and a claimed output $y = f(x)$, correctness can be verified in time $O(\log T)$ or less.

3. **Uniqueness**: For each input $x$, there is exactly one valid output $y$.

The critical property is the first: sequential computation resistance. An adversary with a botnet of a million machines cannot evaluate the VDF faster than a single processor. Time becomes a cryptographic resource—one that cannot be purchased, stolen, or parallelized away.

---

**Application to Execution Governance**

Rate shaping via VDF integrates with CBIG as follows:

At capability issuance, the issuing authority specifies not only the context requirements but also a *delay parameter* $T$. The capability key derivation is extended:

$$K_{cap} = \text{HKDF}(K_{master}, C \| \text{VDF}_T(\text{nonce}), \text{info})$$

where $\text{VDF}_T(\text{nonce})$ is a VDF evaluation over a fresh nonce, requiring $T$ sequential computation steps.

To exercise the capability, the agent must:

1. Measure its context and compute the context signature $C'$
2. Evaluate the VDF: $v = \text{VDF}_T(\text{nonce})$
3. Derive the capability key: $K'_{cap} = \text{HKDF}(K_{agent}, C' \| v, \text{info})$
4. Use $K'_{cap}$ to perform the protected operation

Step 2 requires real time. If $T$ is calibrated to one second of sequential computation, the agent cannot exercise the capability more than once per second, regardless of how much parallel compute it commands.

---

**Economic Implications**

The economics of this construction are worth examining explicitly.

Consider an attacker who wishes to abuse a stolen capability at scale. Without rate shaping, the attacker's cost is dominated by acquiring the capability (through compromise, social engineering, or insider access). Once acquired, marginal cost per operation is near zero. The attack scales trivially.

With VDF-based rate shaping, each operation requires $T$ seconds of wall-clock time. An attacker wishing to perform $N$ operations needs $N \times T$ seconds of sequential compute. The attack does not parallelize. Adding machines does not help.

If $T = 1$ second and the attacker wishes to perform 100,000 unauthorized operations, the attack requires 100,000 seconds—approximately 28 hours of *serial* time. This is not a computational barrier in the traditional sense; the operations are individually feasible. It is a *temporal* barrier that converts attack scale into attack duration.

Duration is the enemy of undetected abuse. A 28-hour attack creates 28 hours of opportunity for detection, intervention, and revocation. The governance system does not need to respond instantly; it needs to respond faster than the attack completes.

This reframes the defender's problem. Instead of preventing all unauthorized operations (impossible against a sufficiently resourced adversary), the governance system must detect and respond to anomalous patterns within a time window defined by the VDF parameter. Rate shaping buys time. Time enables governance.

---

**Calibration and Trade-offs**

VDF parameters must be calibrated to operational requirements.

If $T$ is too small, rate shaping provides insufficient protection. If $T$ is too large, legitimate operations are delayed unacceptably.

The calibration question is: what is the maximum rate at which a correctly-operating agent should exercise this capability? That rate, plus a safety margin, defines the acceptable $T$.

For high-frequency operations (telemetry queries, routine status checks), $T$ may be milliseconds. For consequential operations (configuration changes, data modifications, resource allocations), $T$ may be seconds or minutes. For irreversible operations (deletions, deployments, financial transactions), $T$ may be hours—long enough that human review can occur before the capability becomes exercisable.

This is governance through physics: the delay is not a policy that can be overridden but a computational reality that must be waited out.

---

**Integration with Trajectory Governance**

Rate shaping is not a complete solution. An agent that executes one harmful operation per minute is still harmful—it is merely harmful more slowly.

The value of rate shaping is that it creates *temporal slack* in the system. It converts high-velocity execution into a paced process that human-scale governance mechanisms can observe and interrupt.

Without rate shaping, trajectory governance must respond in milliseconds or lose the race. With rate shaping, trajectory governance can respond in seconds or minutes, because the trajectory itself evolves at a governable rate.

Rate shaping is the bridge between machine-speed execution and human-scale oversight. It does not replace trajectory governance; it makes trajectory governance feasible.

---

## Summary

This chapter has introduced two mechanisms that address the foundational requirements of execution governance:

**CBIG** binds capabilities to context, ensuring that execution cannot occur outside the environmental conditions specified at capability issuance. It transforms authorization from a persistent grant to a continuously-validated constraint.

**Rate shaping via VDF** constrains the velocity of execution, converting high-frequency operations into paced sequences that human-scale governance can monitor and interrupt.

Neither mechanism governs *what* an agent does. Both govern *where*, *when*, and *how fast* an agent can act. They are the static and temporal foundations on which dynamic trajectory governance—the subject of Chapter 3—will be constructed.

Together, they transform the abstract requirement established in Chapter 1—that governance must be continuous and trajectory-aware—into a concrete architectural pattern. Context binding ensures that the execution environment matches governance assumptions. Rate shaping ensures that execution velocity permits governance intervention.

The reader may observe that these mechanisms impose costs. CBIG requires infrastructure for context attestation. VDF requires computational overhead at every protected operation. Legitimate agents are slowed. User experience is affected.

This is correct. These are not costless mechanisms. They are mechanisms whose costs are justified by a specific threat model: the threat of autonomous agents operating at machine speed in contexts that no longer match their authorization conditions.

The question is not whether friction is desirable but whether the alternative—unfrictionable execution—is acceptable.

Chapter 1 established that it is not.
