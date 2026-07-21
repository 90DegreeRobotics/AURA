# Chapter 6: Related Work and Differentiation

---

EGD does not emerge from a vacuum. It builds on decades of work in access control, usage control, security architecture, and safety engineering. This chapter situates EGD within that landscape, clarifying what it borrows, what it extends, and where it departs.

The goal is not to claim superiority over prior work. It is to explain why existing frameworks—each valuable in its domain—are insufficient for the specific problem of governing autonomous, generative agents.

---

## 6.1 Access Control: RBAC, ABAC, and Their Limits

Role-Based Access Control (RBAC) and Attribute-Based Access Control (ABAC) represent the dominant paradigms in enterprise security.

**RBAC** assigns permissions to roles, and users to roles. A user inherits the permissions of their assigned roles. The model is simple, auditable, and scales well for organizations with stable role hierarchies.

**ABAC** generalizes RBAC by evaluating access decisions based on attributes of the subject, object, and environment. Policies can express conditions like "allow access if user.department = resource.department AND time.hour < 18." ABAC is more expressive than RBAC and can encode complex, context-sensitive rules.

Both models share a fundamental assumption: **the action is known at decision time**.

When a subject requests access to an object, the action (read, write, execute) is specified. The access control system evaluates whether that specific action should be permitted. If yes, the action proceeds. If no, it is blocked.

This assumption fails for agentic systems. The agent does not request permission to perform a specific action. It requests permission to pursue an objective. The specific actions that will constitute pursuit of that objective are generated during execution, not before.

ABAC's environmental attributes provide some context sensitivity, but the evaluation is still instantaneous. The policy asks "Should this action be permitted given current attributes?" not "Should this trajectory be permitted given its evolution over time?"

EGD does not replace RBAC or ABAC. Both remain appropriate for their original use cases: governing human users performing discrete, enumerated operations. EGD addresses the gap that opens when the executing entity generates its own operations.

---

## 6.2 Usage Control (UCON): Continuity Without Dynamics

The Usage Control model (UCON), developed by Park and Sandhu in the early 2000s, extends traditional access control with two key innovations:

**Continuity**: UCON evaluates access not just at request time but continuously during usage. Access can be revoked mid-session if conditions change.

**Mutability**: UCON allows attributes to change as a result of access. Using a resource can modify the subject's or object's attributes, which in turn affects future access decisions.

These innovations address some of the limitations Chapter 1 identified. Continuity means that authorization is not a one-time gate. Mutability means that the system can track state changes over time.

EGD inherits these insights. The continuous evaluation of Gamma scores is a form of continuity. The tracking of resource velocity and scope expansion reflects mutability.

But UCON remains fundamentally a permission model. It asks "Should access continue to be permitted?" not "Is this trajectory approaching unsafe regions?" The evaluation is still predicate-based: conditions are either satisfied or not. There is no notion of graduated risk, proportional intervention, or trajectory prediction.

UCON is necessary but not sufficient. EGD can be understood as UCON extended with:

- Trajectory awareness (monitoring state evolution, not just state)
- Graduated response (intervention proportional to risk, not binary permit/deny)
- Economic integration (explicit modeling of attacker costs)

---

## 6.3 Zero Trust: Perimeter Dissolution Without Governance

Zero Trust architecture, popularized over the past decade, abandons the traditional network perimeter. Instead of assuming that entities inside the network are trusted, Zero Trust requires continuous verification: "never trust, always verify."

The principles align with EGD's skepticism of static authorization:

- No implicit trust based on network location
- Continuous authentication and authorization
- Least-privilege access
- Micro-segmentation

But Zero Trust, as typically implemented, remains focused on authentication and authorization—verifying identity and checking permissions. It does not address what happens after access is granted.

A Zero Trust system might require an AI agent to re-authenticate for every API call. It might verify that the agent's credentials are valid and that the requested resource is within scope. But it does not ask whether the *pattern* of API calls indicates trajectory drift. It does not adjust permissions based on observed behavior. It does not provide graduated intervention.

Zero Trust is a network architecture philosophy. EGD is an execution governance framework. They operate at different levels of abstraction and address different failure modes. Zero Trust ensures that only authorized entities access resources. EGD ensures that authorized entities behave appropriately after access is granted.

The two are complementary. A well-architected system would use Zero Trust principles for network and resource access, and EGD principles for agent behavior governance.

---

## 6.4 The Simplex Architecture: Safety Without Cognition

Chapter 3 acknowledged Simplex as a structural template for EGD. The relationship deserves elaboration.

Simplex, developed by Sha and colleagues at Carnegie Mellon, provides formal safety guarantees for systems that use unverified high-performance controllers. The safety controller can always recover the system to a safe state; the decision module detects when recovery is needed.

EGD borrows Simplex's structure:

- Advanced component (agent) operates freely within bounds
- Monitor detects approach to safety boundary
- Fallback (intervention hierarchy) activates when boundary is approached

But Simplex assumes continuous dynamics: the system state evolves according to differential equations, and the recovery region can be defined mathematically. For physical control systems, this is appropriate.

Cognitive agents do not have continuous dynamics. Their state evolution is discrete, high-dimensional, and not analytically tractable. EGD adapts Simplex by:

- Replacing geometric recovery regions with proxy metrics (Gamma)
- Replacing mathematical boundary detection with empirical threshold monitoring
- Replacing binary controller switching with graduated intervention

The adaptation preserves Simplex's *logic* (monitor and intervene) while abandoning its *mathematics* (Lyapunov functions, reachability analysis). This is an honest compromise: we cannot prove formal safety properties for cognitive agents, but we can implement mechanisms that approximate safety-preserving behavior.

---

## 6.5 Ostrom's Graduated Sanctions: Social Governance

Elinor Ostrom's research on common-pool resource governance identified eight design principles for sustainable institutions. Principle 5 is particularly relevant:

> *Graduated sanctions: Appropriators who violate operational rules are likely to be assessed graduated sanctions (depending on the seriousness and the context of the offense) by other appropriators, by officials accountable to the appropriators, or by both.*

This principle—that sanctions should be proportional and escalating, not binary—appears independently in EGD's intervention hierarchy. The convergence is not coincidental.

Both EGD and Ostrom's principles address systems where:

- Participants have autonomy within rules
- Violations range from minor to severe
- Binary punishment (exclusion) destroys value
- The goal is to maintain cooperation, not just punish defection

Ostrom studied human communities managing shared resources. EGD governs computational agents operating within shared systems. The structural parallels suggest that graduated intervention is not a technical hack but a fundamental principle of sustainable governance.

EGD extends Ostrom's insight to computational systems, where:

- Sanctions can be automated (no human judgment required for lower levels)
- Escalation can be continuous (Gamma-based, not offense-counting)
- De-escalation can be explicit (governance level decreases when behavior normalizes)

---

## 6.6 Differentiation Summary

| Framework | What It Governs | Key Mechanism | EGD Difference |
|---|---|---|---|
| RBAC/ABAC | Resource access | Permission evaluation | EGD governs trajectories, not permissions |
| UCON | Ongoing usage | Continuous authorization | EGD adds trajectory awareness and graduated response |
| Zero Trust | Network access | Continuous verification | EGD governs post-access behavior |
| Simplex | Control systems | Recovery region monitoring | EGD handles non-continuous cognitive dynamics |
| Ostrom | Social institutions | Graduated sanctions | EGD automates and continuifies intervention |

EGD is not a replacement for these frameworks. It addresses a gap that none of them fill: the governance of autonomous entities that generate their own actions and evolve through state-space in ways that cannot be fully anticipated at authorization time.

Where actions are enumerated, RBAC/ABAC suffice.
Where usage is continuous but trajectory is not relevant, UCON suffices.
Where network access is the concern, Zero Trust suffices.
Where dynamics are continuous and analytically tractable, Simplex suffices.
Where sanctions are human-administered, Ostrom's principles suffice.

EGD is necessary when:

- Actions are generated
- Trajectories matter
- Dynamics are not analytically tractable
- Intervention must be automated and graduated

That is the specific niche EGD occupies. It does not claim more.

---

## Summary

EGD synthesizes insights from access control, usage control, safety engineering, and institutional economics. It inherits:

- UCON's continuity and mutability
- Zero Trust's skepticism of static authorization
- Simplex's monitor-and-intervene structure
- Ostrom's graduated sanctions

It extends these foundations to address the specific failure mode of agentic systems: the agency gap created when authorized entities generate their own actions.

The differentiation is precise: EGD governs trajectories, not permissions; it provides graduated response, not binary decisions; it operates on proxy metrics, not analytical dynamics; it automates intervention, not human judgment.

This positioning is deliberately narrow. A framework that claims to supersede all prior work is a framework that has not understood the prior work. EGD occupies a specific niche—an important niche, given the rise of agentic AI—but a niche nonetheless.
