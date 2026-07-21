# Chapter 7: Limitations, Ethics, and Failure Modes

---

A framework that does not acknowledge its limitations is a framework that cannot be trusted. This chapter examines the ways EGD can fail, be misused, or produce unintended consequences. The goal is not to undermine the framework but to define its boundaries honestly and to identify the conditions under which it should not be deployed or should be deployed with additional safeguards.

---

## 7.1 Over-Governance

The most immediate failure mode is over-governance: intervention that is so frequent, so restrictive, or so costly that the governed agent becomes useless.

**Symptoms of over-governance:**

- Gamma thresholds set too low, triggering constant escalation
- VDF parameters set too high, making legitimate operations unacceptably slow
- Context binding too strict, invalidating capabilities under normal environmental variation
- Human-in-the-loop requirements that overwhelm available human capacity

**Consequences:**

Over-governance does not improve safety; it destroys utility. An agent perpetually suspended at Level 5 provides no value. Users will route around the governance system—disabling it, misconfiguring it, or replacing the governed agent with an ungoverned alternative.

Governance that is too costly to use is governance that will not be used.

**Mitigation:**

- Gamma weights and thresholds must be calibrated empirically, not set a priori
- Calibration requires measuring both false positive rates (unnecessary intervention) and false negative rates (missed trajectory drift)
- Governance parameters should be tunable per deployment, not hardcoded
- Monitoring should track governance overhead as a first-class metric

Over-governance is a calibration failure, not a framework failure. But the framework must be designed to *permit* appropriate calibration, and operators must understand that miscalibration in either direction is harmful.

---

## 7.2 Under-Governance

The converse failure is under-governance: intervention that is too rare, too permissive, or too slow to prevent harm.

**Symptoms of under-governance:**

- Gamma thresholds set too high, allowing dangerous trajectories to proceed unchecked
- VDF parameters set too low, permitting high-velocity attacks
- Intervention hierarchy that escalates too slowly
- Flight Recorder logs that are not reviewed or acted upon

**Consequences:**

Under-governance provides false assurance. The presence of a governance system creates the perception of safety without the reality. Operators may extend trust to governed agents that the governance does not actually justify.

An incident that occurs despite nominal governance is worse than an incident without governance: it erodes trust not only in the specific system but in governance frameworks generally.

**Mitigation:**

- Regular red-teaming to verify that governance detects and responds to adversarial behavior
- Automated analysis of Flight Recorder logs to identify near-misses
- External audit of governance configurations
- Explicit documentation of what the governance system does and does not guarantee

Under-governance is also a calibration failure, but it is a more insidious one. Over-governance is immediately visible (the agent doesn't work). Under-governance is invisible until an incident occurs.

---

## 7.3 Governance Capture

Governance systems are themselves resources that can be captured by adversaries or misused by operators.

**Capture by adversaries:**

An attacker who compromises the governance system—the Context Evaluator, the Trajectory Monitor, or the Intervention Controller—can disable governance entirely. The agent operates as if ungoverned while appearing to remain governed.

This is addressed in Chapter 4 through architectural isolation (sidecar deployment, separate privilege domains) and cryptographic integrity (Flight Recorder tamper evidence). But no isolation is perfect. Sufficiently resourced adversaries may breach isolation.

**Capture by operators:**

A more subtle risk is capture by the system's own operators. Governance creates friction. Operators under pressure to deliver results may:

- Disable governance "temporarily" to meet a deadline
- Configure thresholds so permissively that governance is nominal
- Suppress or ignore governance alerts
- Modify logs to conceal governance failures

This is not adversarial attack; it is institutional pressure. The governance system's own operators become vectors for governance circumvention.

**Mitigation:**

- Governance configuration changes should be logged immutably
- Disabling governance should require multiple approvals and should itself be logged
- External parties (auditors, regulators) should have read access to Flight Recorder data
- Governance overhead should be budgeted explicitly, not treated as friction to be minimized

Governance capture is a sociotechnical problem. Technical mechanisms can make capture more difficult and more detectable. They cannot make it impossible.

---

## 7.4 False Positives and Contestability

Gamma-based intervention will produce false positives: interventions triggered by legitimate behavior that happens to elevate the risk proxy.

An agent performing an unusual but authorized task may exhibit high action entropy, elevated resource velocity, and scope expansion—all of which elevate Gamma. The governance system cannot distinguish "unusual and legitimate" from "unusual and problematic" without understanding intent, and intent is not observable.

**Consequences of false positives:**

- Legitimate work is delayed or blocked
- Operators lose trust in governance signals
- Alert fatigue leads to ignored warnings

**The contestability requirement:**

If intervention can be mistaken, intervention must be contestable. An agent (or its human operator) that believes an intervention is incorrect must have a mechanism to:

1. Request review of the intervention
2. Provide evidence that the flagged behavior was legitimate
3. Have the intervention reversed if the evidence is compelling
4. Have the reversal logged for future calibration

Contestability is not a weakness of governance; it is a requirement. A governance system that cannot be contested is a governance system that cannot correct its own errors.

**Mitigation:**

- Every intervention should include an explanation (which Gamma components triggered, what threshold was crossed)
- Operators should be able to submit contestation requests with supporting context
- Contested interventions should be reviewed by parties not involved in the original decision
- Successful contestations should feed back into Gamma weight calibration

Contestability introduces delay and complexity. This is the cost of avoiding governance capture by the governance system itself.

---

## 7.5 Ethical Failure Modes

EGD is a technical framework, not an ethical one. It provides mechanisms for constraining agent behavior; it does not specify what behaviors should be constrained. This creates ethical risks.

**Oppressive governance:**

EGD mechanisms can be used to constrain agents in ways that harm users. An authoritarian state could use trajectory monitoring to enforce censorship. A corporation could use scope restriction to prevent agents from accessing information unfavorable to the corporation.

EGD is agnostic to the content of governance policies. It enforces whatever policies are configured. This is a feature for flexibility; it is a bug for ethics.

**Responsibility displacement:**

If an agent causes harm despite governance, who is responsible? The agent cannot be held responsible (it is not a moral agent). The governance system cannot be held responsible (it is a mechanism). Responsibility may diffuse across operators, developers, and policy authors, leaving no one accountable.

This is not unique to EGD—it is a general problem with automated systems—but EGD's sophistication may exacerbate it. A simple system that fails is obviously the operator's responsibility. A complex governance system that fails may create ambiguity about where the failure occurred.

**Autonomy erosion:**

EGD is designed to maintain human sovereignty over autonomous systems. But "human sovereignty" is not monolithic. Governance configured by one set of humans constrains agents used by other humans. Users of governed agents may have their autonomy constrained by policies they did not choose and cannot modify.

This is a feature when the governance represents legitimate authority (safety regulations, organizational policy). It is a bug when the governance represents illegitimate control (surveillance, manipulation).

**Mitigation:**

These are not technical problems with technical solutions. They are political and ethical problems that require:

- Transparency about what governance policies are in effect
- Democratic or contractual legitimacy for governance authority
- Legal frameworks that assign responsibility for governance failures
- Rights for affected parties to contest governance decisions

EGD provides the mechanisms. The ethical deployment of those mechanisms requires structures beyond the framework itself.

---

## 7.6 What EGD Cannot Do

Finally, it is important to state clearly what EGD does not address:

**EGD does not solve alignment.**
Alignment concerns whether an agent's objectives match human values. EGD concerns whether an agent's *behavior* stays within acceptable bounds. An aligned agent may still require governance (values are not perfectly specified). A misaligned agent may comply with governance (behavioral constraints can be satisfied without value alignment). EGD is orthogonal to alignment, not a substitute for it.

**EGD does not guarantee safety.**
EGD reduces the probability of harm and increases the detectability of harmful trajectories. It does not make harm impossible. An agent may cause harm within governance bounds, or may cause harm faster than governance can respond, or may find trajectories that circumvent governance mechanisms.

**EGD does not replace human judgment.**
Graduated intervention includes human-in-the-loop levels, but the framework cannot force humans to exercise good judgment. Humans may approve harmful actions, ignore governance alerts, or misconfigure policies. EGD structures the human-agent interaction; it does not guarantee that humans will interact wisely.

**EGD does not scale to superintelligence.**
The framework assumes that governance mechanisms can observe and intervene in agent behavior. A sufficiently capable agent might predict governance responses and route around them, deceive monitoring systems, or manipulate operators. EGD is designed for current and near-term agentic systems, not for hypothetical superintelligent agents.

These limitations are not failures of the framework. They are boundaries of the problem EGD addresses. A framework that claims to solve all problems is a framework that has not understood any of them.

---

## Summary

EGD can fail through:

- **Over-governance**: Intervention so frequent that utility is destroyed
- **Under-governance**: Intervention so rare that safety is illusory
- **Governance capture**: Adversaries or operators subverting governance mechanisms
- **False positives**: Legitimate behavior incorrectly flagged, requiring contestability
- **Ethical misuse**: Governance mechanisms deployed for oppressive purposes

EGD cannot:

- Solve alignment
- Guarantee safety
- Replace human judgment
- Scale to superintelligence

These limitations define the boundaries of responsible deployment. EGD should be used where the agency gap exists, where calibration is empirically validated, where contestability is available, where governance authority is legitimate, and where operators understand what the framework does and does not guarantee.

Within those boundaries, EGD provides meaningful risk reduction. Outside them, it may provide false assurance or enable harm.

Acknowledging limitations is not weakness. It is the condition of trustworthiness.
