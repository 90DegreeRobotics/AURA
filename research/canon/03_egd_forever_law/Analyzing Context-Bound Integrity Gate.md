# **Context-Bound Integrity Gate (CBIG): A Behavioral Primitive for Misuse Resistance in High-Risk Software**

## **Executive Summary**

The rapid proliferation of high-risk software artifacts—specifically advanced Artificial Intelligence (AI) models and autonomous agents—has precipitated a collapse in traditional perimeter-based security paradigms. In an operational environment where code is increasingly public, binaries are widely distributed, and execution occurs on untrusted user hardware, the conventional reliance on secrecy, access control lists (ACLs), and perimeter defense is no longer viable. This report presents a comprehensive pressure-test and analysis of the "Context-Bound Integrity Gate" (CBIG) as a novel security primitive designed to govern execution rather than merely restricting access.

By analyzing the provided Python proof-of-concept (PoC) through a behavioral lens and synthesizing over two hundred distinct research artifacts, this report establishes that CBIG represents a fundamental shift in systems security: moving from static entitlement to dynamic, context-bound capability enforcement. The analysis confirms that CBIG operationalizes "context" as a non-forgeable constraint, leveraging cryptographic binding, rate shaping via Verifiable Delay Functions (VDFs), and immutable audit trails to fundamentally alter the economics of attacker misuse.

The findings indicate that CBIG effectively mitigates the risks of public code exposure by assuming a "breach-ready" state. Unlike Digital Rights Management (DRM) or obfuscation, which rely on the fragility of hiding secrets within a hostile environment, CBIG enforces policy through computational friction and verifiable transparency. This report details the intellectual lineage of the concept, constructs a rigorous threat model based on public-code assumptions, formalizes context as a cryptographic primitive, and provides a robust economic evaluation of the attacker's cost burden.

## ---

**1\. Introduction: The Failure of Static Security in the Age of Public Code**

The cybersecurity landscape has undergone a tectonic shift with the advent of "public code" paradigms, particularly in the domain of generative AI and large language models (LLMs). Historically, high-value software assets—such as proprietary algorithms, financial trading engines, or military simulations—were protected by physical isolation or strict network perimeters. Security was binary: an entity was either inside the fortress (trusted) or outside (untrusted). This model, predicated on the secrecy of the binary and the integrity of the hosting environment, has been rendered obsolete by the distribution requirements of modern AI deployment.

Today, powerful models are leaked, open-sourced, or deployed to edge devices (e.g., smartphones, IoT sensors) where the physical hardware is under the control of the user—who may also be the adversary. In this "white-box" environment, the attacker has unlimited access to the binary, the memory state, and the execution flow.1 They can inspect, modify, and replay execution at will. Traditional defenses like DRM fail because they rely on hiding a decryption key within the very code the attacker controls; once the key is extracted, the defense collapses entirely.2 Similarly, code obfuscation offers only a temporary speed bump, delaying but not preventing reverse engineering.3

### **1.1 The CBIG Proposition**

The Context-Bound Integrity Gate (CBIG) emerges as a response to this failure. It posits that if the *code* cannot be kept secret, the *utility* of that code must be bound to a specific, verifiable context. CBIG is not a mechanism to prevent copying bytes; it is a mechanism to prevent unauthorized *execution* by ensuring that the cryptographic material required to run the software is mathematically derived from the environment itself.

The provided Python proof-of-concept (formerly QSIC) serves as a behavioral blueprint for this architecture. Our analysis of the PoC reveals a deliberate intent to couple execution capability with three distinct control levers:

1. **Contextual Binding:** Deriving execution keys from ephemeral environmental signals (e.g., time, hardware signatures, network topology), creating a dependency that breaks if the software is moved to an unauthorized environment.5  
2. **Rate Shaping:** Introducing mandatory, cryptographically enforced latency via computational puzzles or delay functions, preventing the automated scaling of misuse (e.g., botnet-driven spam generation).7  
3. **Auditability:** Enforcing the generation of immutable logs as a prerequisite for execution, destroying the possibility of "secret" misuse and enabling post-incident attribution.9

### **1.2 Scope of Analysis**

This report validates the CBIG concept by examining its theoretical roots in capability-based security and zero-trust architectures, formally defining its mechanisms, and evaluating its effectiveness against a "public-code/breach-assumed" threat model. We further analyze the economic impact on adversaries, demonstrating how CBIG shifts the security metric from "possibility of hack" to "cost of attack," utilizing frameworks like the Gordon-Loeb model 11 and the Pyramid of Pain.12

## ---

**2\. Intellectual Lineage and Theoretical Foundations**

The CBIG architecture is not an isolated invention but the convergence of several high-assurance security lineages. It synthesizes principles from capability-based operating systems, zero-trust network architectures, and cryptographic misuse resistance to address the specific vulnerabilities of distributed, high-risk software.

### **2.1 Capability-Based Security: From Permissions to Tokens**

The foundational logic of CBIG is rooted in **Capability-Based Security**, a concept dating back to the systems research of the 1960s and 70s. Traditional operating systems (like UNIX) and access control models rely on Access Control Lists (ACLs). In an ACL system, a gatekeeper (the kernel or server) checks a list to see if a subject (user) is allowed to access an object (file).13 This model implies a centralized authority and is prone to the "confused deputy" problem, where a privileged program is tricked into misusing its authority on behalf of a malicious user.14

Capability systems invert this model. A "capability" is a communicable, unforgeable token of authority—effectively a key that grants access to an object by virtue of possession.13 If a process holds the token, it can perform the action. CBIG adopts this "token-based" approach but adds a critical constraint: the capability is not static. In classical capability systems, a token is valid until revoked. In CBIG, the token is **context-bound**. It is valid only when the cryptographic context (time, location, system state) matches the conditions embedded in the token.16

This evolution mirrors the development of WebAssembly System Interface (WASI) security, where modules are granted specific, granular capabilities (e.g., "read access to /tmp only") rather than inheriting the user's global rights.14 CBIG extends this to the application layer, treating the "right to execute the model" as a capability that is dynamically derived and continuously verified.15

### **2.2 Zero Trust and Continuous Verification**

CBIG is the operationalization of **Zero Trust Architecture (ZTA)** principles, specifically those outlined in NIST SP 800-207. The core axiom of Zero Trust is "never trust, always verify".17 Traditional security models assumed that once an entity cleared the perimeter firewall, they were trusted. ZTA assumes the network is hostile and the perimeter is breached.

In the ZTA framework, access requests are evaluated dynamically based on a multitude of signals: user identity, device health, location, and behavioral patterns.17 NIST SP 800-207 explicitly calls for "granular context-based policies" where trust is ephemeral and re-evaluated per session.19 CBIG implements this by embedding the Policy Decision Point (PDP) and Policy Enforcement Point (PEP) logic effectively within the software's execution path.

The provided PoC demonstrates this by requiring the generation of fresh cryptographic material for each execution cycle. This aligns with the "Just-in-Time" (JIT) access principles of modern cloud security, where credentials are ephemeral and exist only for the duration of a specific task.20 By binding execution to real-time context, CBIG ensures that a credential stolen at time $T$ is useless at time $T+1$ if the context has shifted—a property known as **Post-Compromise Security (PCS)**.22

### **2.3 Context-Aware Access Control (CAAC)**

The limitations of Role-Based Access Control (RBAC)—specifically its static nature and role explosion—have led to the adoption of Attribute-Based Access Control (ABAC) and Policy-Based Access Control (PBAC).24 These models allow for rules like "Allow access IF user is in Engineering AND location is Office AND time is 9-5."

CBIG evolves this into **Context-Aware Access Control** by making the "context" cryptographic rather than administrative. In ABAC, a central server evaluates the attributes. In CBIG, the attributes (context) are inputs to a cryptographic function (e.g., a Key Derivation Function). If the attributes are wrong, the key is mathematically incorrect, and the access fails natively without a central server needing to say "deny".26 This reduces brittleness; the security is inherent to the math, not the policy engine configuration.28

### **2.4 Misuse-Resistant Cryptography**

The cryptographic design of CBIG draws heavily from the field of **Misuse-Resistant Authenticated Encryption (MRAE)**. In classical encryption (e.g., AES-GCM), reusing a nonce (a "number used once") can be catastrophic, allowing attackers to recover the key or the plaintext.29 MRAE schemes (like SIV, Deoxys-II, or Romulus-M) are designed to provide robust security even if the user makes mistakes, such as repeating a nonce or using poor randomness.30

CBIG applies this "robustness under failure" philosophy to the software governance layer. It acknowledges that the "user" (who may be an attacker) will likely "misuse" the software by trying to run it out of bounds. The system is designed such that this misuse results in a benign failure (the key doesn't derive) rather than a security breach.32 The PoC's reliance on determinstic derivation reflects the MRAE principle of creating predictable, secure outcomes from potentially untrusted inputs.34

## ---

**3\. Threat Model: The Public-Code/Breach-Assumed Reality**

To validate CBIG, we must construct a threat model that reflects the reality of modern software distribution, rejecting the "fortress" mentality in favor of a **Public-Code / Breach-Assumed** paradigm.

### **3.1 The "Public Code" Assumption**

In the domain of AI, the distinction between "proprietary" and "public" is eroding. Models are leaked (e.g., LLaMA), open-sourced, or distilled.1 Furthermore, the rise of "vibe coding" and AI-generated code means that software supply chains are flooded with code snippets of unknown provenance, often containing vulnerabilities or embedded secrets.35

The threat model assumes:

1. **Full Access:** The adversary has the binary, the source code, and the model weights. They have "white-box" access and can run the code in a debugger.1  
2. **Compromised Perimeter:** The adversary is executing the code on their own hardware (e.g., a GPU cluster in a non-extradition jurisdiction) or on a compromised endpoint within a corporate network.37  
3. **No Central Gatekeeper:** Traditional server-side API rate limits do not apply because the adversary is running the model locally.

### **3.2 Anticipated Misuse Scenarios**

Analysis of the PoC and the broader landscape identifies specific misuse scenarios CBIG is designed to thwart:

* **Scaled Automated Abuse:** An adversary uses a leaked LLM to generate millions of spam emails, phishing lures, or disinformation articles. The goal is volume. The PoC’s rate-shaping mechanisms (VDFs) specifically target this by imposing a time cost per execution.38  
* **Malicious Fine-Tuning:** An adversary takes a safety-aligned model and fine-tunes it on harmful data (e.g., bomb-making instructions) to remove guardrails. CBIG’s context binding aims to prevent the model from loading or executing if the integrity of the weights or the training environment is violated.6  
* **Unattributed Execution:** An adversary generates Deepfake pornography or political kompromat. They rely on the fact that local execution leaves no server logs. CBIG’s auditability requirement ensures that even local execution generates a cryptographic receipt, creating a liability trail.9

### **3.3 The "Secret-Free" Security Goal**

Current hardware security relies on "secrets" stored in non-volatile memory (e.g., keys in a TPM). However, sophisticated attackers can extract these secrets via side-channel attacks (e.g., microscopic inspection, power analysis).41  
CBIG aspires to "Secret-Free Security".41 In this model, the system contains no static secrets at rest. The "secret" required to execute the code is generated dynamically from the physics of the device (PUFs) and the entropy of the environment (EKG) at the moment of execution. If an attacker steals the device and analyzes it in a lab, they cannot extract the key because the context (e.g., the network topology, the time window) is missing. The key exists only as a transient state of the live system.43

## ---

**4\. Formalizing 'Context' as a Security Primitive**

To transition "context" from a vague architectural concept to a rigorous security control, CBIG formalizes it through cryptographic binding. This involves defining context, measuring it, and binding it to execution logic.

### **4.1 Taxonomy of Contextual Signals**

Context in CBIG is composed of verifiable signals that assert the legitimacy of the execution environment. Based on the literature (S39, S170, S171), these signals include:

* **Temporal Context:** The current time window. Execution is valid only for a specific epoch (e.g., "Block height 8,000,000 to 8,000,100"). This prevents replay attacks where old credentials are used indefinitely.45  
* **Spatial/Environmental Context:**  
  * *Hardware Fingerprints:* Responses from Physical Unclonable Functions (PUFs) that uniquely identify the silicon.46  
  * *Browser/Client Fingerprints:* High-entropy signals such as Canvas/WebGL rendering differences, audio stack latency, and screen properties.48 While often used for tracking, CBIG inverts this to use entropy as a key constituent.  
  * *Network Topology:* Proof of Location (PoL) or proximity to specific trusted anchors.50  
* **Identity/Authorization Context:** Cryptographic proofs (e.g., OIDC tokens, Verifiable Credentials) asserting the user's privilege level.52  
* **Code Integrity Context:** Measurements of the binary itself (hashing executable segments) to ensure no tampering or malicious patching has occurred (Control Flow Integrity).53

### **4.2 Cryptographic Binding via HKDF**

The mechanism for binding these signals to execution is the HMAC-based Extract-and-Expand Key Derivation Function (HKDF), defined in RFC 5869.55  
In the CBIG architecture (and evidenced by the PoC), the execution key $K\_{exec}$ is not stored. It is derived at runtime:

$$K\_{exec} \= \\text{HKDF-Expand}(\\text{PRK}, \\text{info}=\\text{ContextSignature}, L)$$

* **PRK (Pseudorandom Key):** A base secret or entropy source.  
* **info:** This parameter is crucial. It is populated with the serialized ContextSignature (e.g., \`Time |

| UserID |  
| HardwareHash\`).

* **L:** The length of the required key.

If the ContextSignature provided at runtime deviates by even a single bit from the expected state—for example, if the system clock is outside the valid window or the hardware hash doesn't match—the HKDF produces a completely different $K\_{exec}$.56 The software attempts to decrypt its payload or authorize its inference engine with this wrong key, resulting in a crash or garbage output. This is a "fail-safe" default: "Wrong context $\\rightarrow$ Wrong key $\\rightarrow$ No execution."

### **4.3 Environmental Key Generation (EKG)**

This cryptographic binding is a practical implementation of **Environmental Key Generation**, a concept proposed by Riordan and Schneier for "Clueless Agents".5 The agent (software) does not bring its key; it constructs the key from the environment.

* *Mechanism:* The software scans the local environment (e.g., reading specific memory addresses, querying local sensors). It hashes these values to form the decryption key.  
* *Application:* This prevents the software from being analyzed in a sandbox. If a malware analyst runs the code in a virtual machine (VM), the environmental variables (e.g., CPU thermal noise, specific hardware registers) will differ from the target operational environment.6 The key derivation fails, and the analyst sees only encrypted noise.  
* *Reliability vs. Security:* A core challenge identified in the research is the stability of environmental readings. Sensors are noisy. To address this, CBIG must employ **Fuzzy Extractors** or Error Correction Codes (ECC) to map "close enough" environmental readings to the exact same cryptographic key, ensuring legitimate users aren't locked out due to minor fluctuations (e.g., temperature changes affecting clock skew).46

### **4.4 Dynamic Code Identity and Attestation**

CBIG extends context binding to the code itself. Traditional code signing verifies the binary at load time. CBIG requires **Dynamic Code Identity**, verifying the integrity of the software *during* execution.53

* *Mechanism:* A "Remote Attestation Agent" (RAA) or a local TEE (Trusted Execution Environment) continuously measures the code's execution path and memory state.59  
* *Binding:* These measurements are fed into the context derivation. If an attacker uses a "game trainer" or debugger to patch the memory (e.g., jumping over an authorization check), the memory hash changes, the context changes, the key derivation fails, and the application terminates.60 This creates **Control Flow Integrity (CFI)** enforced not just by the CPU, but by the cryptographic availability of the session key.

## ---

**5\. Misuse Economics: Cost Curves vs. Impossibility**

A central tenet of the CBIG analysis is the admission that absolute security is impossible in a public-code scenario. A determined attacker with infinite resources can eventually simulate the correct environment or reverse-engineer the derivation logic (the "analog hole"). Therefore, CBIG shifts the objective from "impossibility" to **Misuse Economics**. The goal is to alter the **Attacker Cost Curve** such that the cost of misuse exceeds the value of the attack (Return on Attack, ROA).62

### **5.1 Rate Shaping via Verifiable Delay Functions (VDFs)**

The PoC highlights the use of computational puzzles, which leads us to **Verifiable Delay Functions (VDFs)** as a critical rate-shaping primitive.7

* **Definition:** A VDF is a function $f(x)$ that takes a prescribed number of sequential steps ($T$) to compute, which cannot be parallelized, but whose output $y$ can be efficiently and publicly verified.65  
* **Mechanism:** To execute a batch of inferences (e.g., generating 1,000 spam emails), the CBIG protocol requires the user to submit a VDF proof derived from the previous output. The VDF parameters are tuned such that computing the proof takes, say, 1 second of wall-clock time on the fastest available consumer hardware.  
* **Economic Impact:**  
  * *Legitimate User:* A 1-second delay on startup or periodically during a session is a negligible UX cost.8  
  * *Attacker:* For an attacker aiming for volume (millions of requests), this serialization is catastrophic. They cannot use parallelization (adding more GPUs) to speed up the VDF because the calculation is inherently sequential (e.g., repeated squaring in a group of unknown order).66 To generate 1 million spam messages, they must pay $1 \\text{ million} \\times 1 \\text{ second}$ of sequential time.  
  * *Result:* The throughput of the attack is capped by physics (time), not by bandwidth. The cost of hardware and electricity required to maintain this throughput scales linearly, destroying the "fast and cheap" economics that make botnets profitable.68

### **5.2 Client Puzzles and Congestion Control**

This aligns with **Client Puzzle Protocols** (CPP).69 When the system detects high-risk context (e.g., IP reputation issues, unusual behavior), it increases the "difficulty" of the puzzle (the time parameter $T$).

* **Adaptive Friction:** This functions as a "cryptographic rate limit." Unlike server-side rate limits which can be bypassed by rotating IPs (Sybil attacks), the puzzle must be solved by the *client* machine wanting to execute the code. The cost is externalized to the attacker.71  
* **Tor & Onion-Flation:** Research into Tor's proof-of-work defense shows that while effective, attackers can sometimes "inflate" costs for defenders. CBIG mitigates this by making the puzzle strictly local to the execution instance, preventing network-wide DoS.72

### **5.3 The "Pyramid of Pain" and Friction**

Cybersecurity effectiveness is often measured by the "Pyramid of Pain"—a model classifying indicators by how hard they are for attackers to change.12

* *Hash/IPs (Easy):* Attackers change these instantly.  
* TTPs (Hard): Tactics, Techniques, and Procedures.  
  CBIG targets the TTP level. By forcing sequential computation and context binding, CBIG attacks the adversary's capacity to operate. It forces them to invest in expensive, specialized hardware or manual labor (simulating context), pushing their costs up the pyramid until the attack is no longer viable.73

### **5.4 Latency Analysis: Human vs. Bot**

The effectiveness of this approach hinges on the differential impact of latency.

* *Research Insight:* Legitimate human users have a "latency budget" (e.g., \<200ms feels instant, \<1s is acceptable for loading).8  
* *Bot Economics:* Bots rely on high-frequency trading or massive throughput. A 1-second delay per transaction destroys the utility of algorithmic trading bots 74 or spam bots.  
* *Conclusion:* CBIG parameters must be tuned to sit exactly in this gap: imperceptible to humans, fatal to bots.

## ---

**6\. Governance and Auditability: The Transparency Layer**

The third pillar of CBIG is **Auditability**. In high-risk scenarios (e.g., AI in healthcare, finance, or automated decision-making), limiting access is insufficient; the system must ensure that all actions are accountable.

### **6.1 Transparency Logs and Append-Only Structures**

CBIG mandates the use of **Transparency Logs**, drawing from the architecture of Certificate Transparency and "verifiable ledger" databases.75

* **Mechanism:** Every time the software successfully executes (i.e., derives a session key), it generates a signed "receipt" containing the Context (Time, User ID, Input Hash, VDF Output). This receipt is appended to a local or remote **Append-Only Log**.10  
* **Merkle Trees:** These logs are structured as Merkle Trees (or Hash Chains). The hash of entry $N$ depends on the hash of entry $N-1$. This creates a tamper-evident history. If an attacker tries to delete a log entry to hide their misuse, the hash chain breaks, and the tampering is mathematically provable.10  
* **Public Verifiability:** These logs can be published to a distributed ledger or a public transparency server, ensuring that the "history" of the model's usage is public property, not a private database that can be manipulated by the vendor.77

### **6.2 Non-Repudiation and Cryptographic Liability**

This architecture provides **Non-Repudiation**.9

* *Scenario:* A Deepfake video appears online.  
* *Resolution:* If CBIG was enforced, the video file (or the generative process) would have an associated cryptographic receipt in the transparency log. The log would reveal *who* generated it, *when*, and *where*.  
* *Deterrence:* The knowledge that every execution leaves an indelible, cryptographically signed trail acts as a powerful psychological and legal deterrent against misuse. An attacker cannot operate in the shadows; they must operate in the full light of the transparency log.78

### **6.3 Blockchain as a "Memory Layer"**

While avoiding hype, the research supports the use of blockchain-like structures not for currency, but as an **Immutable Memory Layer** for AI.80 The blockchain does not store the data (too expensive); it stores the *proofs* of the data and the execution context. This creates "Verifiable Cognition," where the AI's actions are anchored to a public trust root, distinct from the model developer.80

## ---

**7\. Comparative Analysis vs. Failed Approaches**

To understand CBIG's value, we must contrast it with legacy approaches that have largely failed to secure distributed software.

| Feature | Digital Rights Management (DRM) | Obfuscation | CBIG (Context-Bound Integrity Gate) |
| :---- | :---- | :---- | :---- |
| **Primary Goal** | Prevent copying/access to bits. | Hide logic/algorithm from humans. | **Govern execution & ensure integrity.** |
| **Mechanism** | Static encryption, hidden keys in binary. | Renaming, control flow flattening. | **Dynamic key derivation, VDFs, Audit logs.** |
| **Failure Mode** | Keys extracted, DRM stripped once. | Reverse engineered with time/effort. | **Misuse becomes uneconomical & auditable.** |
| **Attacker Cost** | Fixed one-time break (crack once, run everywhere). | One-time analysis cost (O(1)). | **Variable per-execution cost (O(N)).** |
| **Context Awareness** | None (usually just checks license). | None. | **High (Context *is* the key).** |
| **Transparency** | Opaque (black box). | Opaque. | **Transparent (Verifiable audit trails).** |
| **Resilience** | Brittle (binary break). | Brittle (de-obfuscators). | **Elastic (Adapts via VDF difficulty).** |

### **7.1 Why DRM Fails**

DRM fails because it treats the user as the enemy while giving the user the keys (hidden in the player). It is a contradiction. Once the "secret" is found, the protection evaporates globally.2  
CBIG Success: CBIG does not rely on a static secret. The "secret" is the output of the environment. Even if the attacker knows how the key is derived, they cannot produce the key without reproducing the environment (which requires expensive hardware spoofing or physical proximity).

### **7.2 Why Obfuscation Fails**

Obfuscation is "security by obscurity." Tools like IDA Pro and Ghidra, augmented by AI, can de-obfuscate code rapidly.3  
CBIG Success: CBIG relies on "Kerckhoffs's principle"—the system is secure even if the enemy knows the design. The security comes from the computational hardness of the VDF and the entropy of the context, not the obscurity of the code.

## ---

**8\. Economic Analysis and Evaluation Metrics**

Moving beyond binary assessments of "secure" vs "insecure," we validate CBIG using economic and probabilistic metrics.

### **8.1 The Attacker Cost Metric ($C\_{attack}$)**

We propose a formula to quantify the security provided by CBIG, derived from the Gordon-Loeb model 11 and Return on Attack (ROA) studies.63

$$C\_{attack} \= (T\_{VDF} \\times N\_{requests} \\times C\_{compute}) \+ C\_{context\\\_spoof} \+ C\_{audit\\\_risk}$$  
Where:

* $T\_{VDF}$: Time required to solve the delay function per request (e.g., 1 sec).  
* $N\_{requests}$: Volume of misuse attempts (e.g., 1,000,000).  
* $C\_{compute}$: Cost of hardware/electricity per second (increases if VDF is memory-hard).  
* $C\_{context\\\_spoof}$: Fixed cost to forge the required context (e.g., buying valid credentials, decapping chips to read PUFs).83  
* $C\_{audit\\\_risk}$: The monetized risk of detection/prosecution due to the immutable audit trail.

**Success Criteria:** The defense is successful if $C\_{attack} \> V\_{misuse}$ (The value gained from the attack). If it costs $10,000 to generate $5,000 worth of spam, the attack will not happen.38

### **8.2 False Rejection Rate (FRR)**

A critical metric for usability.

* *Definition:* The rate at which legitimate users are denied access due to context drift (e.g., a user travels, changing their network topology, causing the EKG to fail).85  
* *Target:* A viable CBIG system must maintain FRR \< 1% (or comparable to biometric auth) while keeping attacker costs high. This requires tuning the "fuzziness" of the environmental extraction.27

### **8.3 Audit Completeness (Tamper-Evidence)**

* *Definition:* The probability that an attacker can delete or modify a log entry without detection.  
* *Target:* With hash chains/Merkle trees, this should be effectively **100%**. Any modification breaks the chain.9

### **8.4 ROI/ROSI (Return on Security Investment)**

For the defender, CBIG offers a high ROSI.

* *Formula:* $ROSI \= (Risk Exposure \\times \\%Mitigated \- Cost) / Cost$.87  
* *Analysis:* By embedding security into the code (low marginal cost), CBIG mitigates the high risk of scaled reputational damage from model misuse, offering a superior ROSI compared to chasing leaks with legal takedowns.

## ---

**Conclusion**

The Context-Bound Integrity Gate (CBIG) represents a necessary paradigm shift in the security of high-risk software. It acknowledges the uncomfortable reality that in a distributed, AI-driven world, the perimeter is gone, and the code is public.

By synthesizing **capability-based security**, **zero-trust verification**, and **cryptographic rate shaping**, CBIG creates a governance layer that travels *with* the software. It moves the security boundary from the network edge to the instruction pointer. The primitive does not promise impossible secrecy; instead, it delivers **economic infeasibility** for the attacker. It turns the model's own execution logic into a tax on misuse, enforcing a simple, powerful law: **No Context, No Key. No Time, No Output. No Log, No Execution.**

For researchers and architects of the next generation of AI systems, CBIG offers a validated blueprint for deploying powerful capabilities into the wild without surrendering control. Future work should focus on standardizing the "Context Description Language" and optimizing VDF algorithms to minimize the latency impact on legitimate users while maximizing the penalty for adversarial automation.

#### **Works cited**

1. AI Code Security Explained | Wiz, accessed January 6, 2026, [https://www.wiz.io/academy/application-security/ai-code-security](https://www.wiz.io/academy/application-security/ai-code-security)  
2. 8 Challenging Projects on Cryptography for 2025 \- The Crypto Recruiters, accessed January 6, 2026, [https://thecryptorecruiters.io/projects-on-cryptography/](https://thecryptorecruiters.io/projects-on-cryptography/)  
3. Hardware-assisted Code Obfuscation \- reposiTUm, accessed January 6, 2026, [https://repositum.tuwien.at/bitstream/20.500.12708/2598/2/Schrittwieser%20Sebastian%20-%202014%20-%20Hardware-assisted%20code%20obfuscation.pdf](https://repositum.tuwien.at/bitstream/20.500.12708/2598/2/Schrittwieser%20Sebastian%20-%202014%20-%20Hardware-assisted%20code%20obfuscation.pdf)  
4. Protecting Software through Obfuscation: Can It Keep Pace with Progress in Code Analysis?, accessed January 6, 2026, [https://www.plai.ifi.lmu.de/publications/csur16-obfuscation.pdf](https://www.plai.ifi.lmu.de/publications/csur16-obfuscation.pdf)  
5. Environmental Key Generation towards Clueless Agents \- Schneier on Security \-, accessed January 6, 2026, [https://www.schneier.com/wp-content/uploads/2016/02/paper-clueless-agents.pdf](https://www.schneier.com/wp-content/uploads/2016/02/paper-clueless-agents.pdf)  
6. Execution Guardrails: Environmental Keying, Sub-technique T1480.001 \- MITRE ATT\&CK®, accessed January 6, 2026, [https://attack.mitre.org/techniques/T1480/001/](https://attack.mitre.org/techniques/T1480/001/)  
7. A Survey on Proof of Sequential Work: Development, Security Analysis, and Application Prospects \- MDPI, accessed January 6, 2026, [https://www.mdpi.com/1099-4300/28/1/33](https://www.mdpi.com/1099-4300/28/1/33)  
8. Verifiable Delay Functions for Rate-limiting Systems \- ETH Zurich Research Collection, accessed January 6, 2026, [https://www.research-collection.ethz.ch/bitstreams/47edebf2-4480-460b-ab5b-d3d8470e16fe/download](https://www.research-collection.ethz.ch/bitstreams/47edebf2-4480-460b-ab5b-d3d8470e16fe/download)  
9. Lightweight and High-Throughput Secure Logging for Internet of Things and Cold Cloud Continuum \- arXiv, accessed January 6, 2026, [https://www.arxiv.org/pdf/2506.08781](https://www.arxiv.org/pdf/2506.08781)  
10. Gossiping with Append-Only Logs in Secure-Scuttlebutt \- ResearchGate, accessed January 6, 2026, [https://www.researchgate.net/publication/348239763\_Gossiping\_with\_Append-Only\_Logs\_in\_Secure-Scuttlebutt](https://www.researchgate.net/publication/348239763_Gossiping_with_Append-Only_Logs_in_Secure-Scuttlebutt)  
11. Learn the Fundamentals of the Gordon-Loeb Cyber Investment Model in this Interview with Professor Gordon \- ActiveCyber, accessed January 6, 2026, [https://activecyber.net/learn-the-fundamentals-of-the-gordon-loeb-cyber-investment-model-in-this-interview-with-professor-gordon/](https://activecyber.net/learn-the-fundamentals-of-the-gordon-loeb-cyber-investment-model-in-this-interview-with-professor-gordon/)  
12. Pyramid of pain: Strategic detection that costs attackers \- Vectra AI, accessed January 6, 2026, [https://www.vectra.ai/topics/pyramid-of-pain](https://www.vectra.ai/topics/pyramid-of-pain)  
13. Capability-based security \- Wikipedia, accessed January 6, 2026, [https://en.wikipedia.org/wiki/Capability-based\_security](https://en.wikipedia.org/wiki/Capability-based_security)  
14. Capabilities-Based Security with WASI \- Marco Kuoni, accessed January 6, 2026, [https://marcokuoni.ch/blog/15\_capabilities\_based\_security/](https://marcokuoni.ch/blog/15_capabilities_based_security/)  
15. Capability-Based Security \- Hyperware, accessed January 6, 2026, [https://book.hyperware.ai/system/process/capabilities.html](https://book.hyperware.ai/system/process/capabilities.html)  
16. Context and Remoting \- Diranieh, accessed January 6, 2026, [http://diranieh.com/NETAdvanced/ContextAndRemoting.htm](http://diranieh.com/NETAdvanced/ContextAndRemoting.htm)  
17. Zero trust security: The zero trust model \- Article \- SailPoint, accessed January 6, 2026, [https://www.sailpoint.com/identity-library/zero-trust-model](https://www.sailpoint.com/identity-library/zero-trust-model)  
18. Access Control And Identity-Based Access With Zero Trust | by Chris Yeung \- Medium, accessed January 6, 2026, [https://medium.com/@cypanrisk/access-control-and-identity-based-access-with-zero-trust-e59e5e25f48f](https://medium.com/@cypanrisk/access-control-and-identity-based-access-with-zero-trust-e59e5e25f48f)  
19. ServiceNow Supports NIST 800-207 Zero-Trust Cybersecurity, accessed January 6, 2026, [https://www.servicenow.com/community/secops-articles/servicenow-supports-nist-800-207-zero-trust-cybersecurity/ta-p/3455669](https://www.servicenow.com/community/secops-articles/servicenow-supports-nist-800-207-zero-trust-cybersecurity/ta-p/3455669)  
20. Hardening browser security with zero-trust controls \- CSO Online, accessed January 6, 2026, [https://www.csoonline.com/article/4101173/hardening-browser-security-with-zero-trust-controls.html](https://www.csoonline.com/article/4101173/hardening-browser-security-with-zero-trust-controls.html)  
21. What is NGAC? And Why It's the Future of Cloud Access Control \- Trustle, accessed January 6, 2026, [https://www.trustle.com/post/what-is-ngac-and-why-its-the-future-of-cloud-access-control](https://www.trustle.com/post/what-is-ngac-and-why-its-the-future-of-cloud-access-control)  
22. Akeso: Bringing Post-Compromise Security to Cloud Storage \- Privacy Enhancing Technologies Symposium, accessed January 6, 2026, [https://petsymposium.org/popets/2025/popets-2025-0139.pdf](https://petsymposium.org/popets/2025/popets-2025-0139.pdf)  
23. On Post-Compromise Security, accessed January 6, 2026, [https://people.cispa.io/cas.cremers/downloads/papers/CCG-CSF2016-PCS.pdf](https://people.cispa.io/cas.cremers/downloads/papers/CCG-CSF2016-PCS.pdf)  
24. Attribute-Based Access Control: The Scalable, Context-Aware Security Model for Modern Applications \- hoop.dev, accessed January 6, 2026, [https://hoop.dev/blog/attribute-based-access-control-the-scalable-context-aware-security-model-for-modern-applications/](https://hoop.dev/blog/attribute-based-access-control-the-scalable-context-aware-security-model-for-modern-applications/)  
25. PBAC Is Back. Why Policy‑Based Access Control Is Trending Again for Enterprise Security, accessed January 6, 2026, [https://www.cerbos.dev/blog/policy-based-access-control-enterprise-security](https://www.cerbos.dev/blog/policy-based-access-control-enterprise-security)  
26. AuthZilla, Rethinking Authorization For The Modern Era | by Criteo R\&D \- Medium, accessed January 6, 2026, [https://medium.com/criteo-engineering/authzilla-rethinking-authorization-for-the-modern-era-40423fb3f33c](https://medium.com/criteo-engineering/authzilla-rethinking-authorization-for-the-modern-era-40423fb3f33c)  
27. An Improved Mechanism for Access Control using Context Awareness \- ResearchGate, accessed January 6, 2026, [https://www.researchgate.net/publication/396463412\_An\_Improved\_Mechanism\_for\_Access\_Control\_using\_Context\_Awareness](https://www.researchgate.net/publication/396463412_An_Improved_Mechanism_for_Access_Control_using_Context_Awareness)  
28. I AM Broken: Why Traditional Identity & Access Management Fails in an Agent-First World, accessed January 6, 2026, [https://www.felicis.com/insight/agent-identity](https://www.felicis.com/insight/agent-identity)  
29. Tactical Trust (1/2) \- High Assurance Rust: Developing Secure and Robust Software, accessed January 6, 2026, [https://highassurance.rs/chp14/tactical\_trust\_1.html](https://highassurance.rs/chp14/tactical_trust_1.html)  
30. Hardware Implementations of Romulus: Exploring Nonce Misuse Resistance and Boolean Masking \- NIST Computer Security Resource Center, accessed January 6, 2026, [https://csrc.nist.gov/csrc/media/Events/2022/lightweight-cryptography-workshop-2022/documents/papers/hardware-implementations-of-romulus.pdf](https://csrc.nist.gov/csrc/media/Events/2022/lightweight-cryptography-workshop-2022/documents/papers/hardware-implementations-of-romulus.pdf)  
31. CMCC: Misuse Resistant Authenticated Encryption with Minimal Ciphertext Expansion, accessed January 6, 2026, [https://www.mdpi.com/2410-387X/2/4/42](https://www.mdpi.com/2410-387X/2/4/42)  
32. Designing an API Framework for Simplified Integration of Pre- and Post-Quantum Digital Signature Schemes for Software Developers \- JKU ePUB, accessed January 6, 2026, [https://epub.jku.at/obvulihs/download/pdf/12987188](https://epub.jku.at/obvulihs/download/pdf/12987188)  
33. How Usable are Rust Cryptography APIs? \- arXiv, accessed January 6, 2026, [https://arxiv.org/pdf/1806.04929](https://arxiv.org/pdf/1806.04929)  
34. Lightweight Yet Nonce-Misuse Secure Authenticated Encryption for Very Short Inputs \- IEEE Xplore, accessed January 6, 2026, [https://ieeexplore.ieee.org/iel8/6488907/6702522/10745525.pdf](https://ieeexplore.ieee.org/iel8/6488907/6702522/10745525.pdf)  
35. Using AI-generated code safely (Vibe coding security), accessed January 6, 2026, [https://beaglesecurity.com/blog/article/using-ai-generated-code-safely.html](https://beaglesecurity.com/blog/article/using-ai-generated-code-safely.html)  
36. The New Reality of Threat Modeling in AI-Generated Systems \- Securityreview.ai, accessed January 6, 2026, [https://www.securityreview.ai/blog/the-new-reality-of-threat-modeling-in-ai-generated-systems](https://www.securityreview.ai/blog/the-new-reality-of-threat-modeling-in-ai-generated-systems)  
37. The Universal Cloud Threat Model \- Securosis, accessed January 6, 2026, [https://securosis.com/wp-content/uploads/2024/04/UCTM\_v\_1.0.pdf](https://securosis.com/wp-content/uploads/2024/04/UCTM_v_1.0.pdf)  
38. Attacker Economics \- SHI, accessed January 6, 2026, [https://www.content.shi.com/SHIcom/ContentAttachmentImages/SharedResources/PDFs/F5/f5-092721-ebook-attacker-economics.pdf](https://www.content.shi.com/SHIcom/ContentAttachmentImages/SharedResources/PDFs/F5/f5-092721-ebook-attacker-economics.pdf)  
39. Social Engineering in Distributed Teams: Modeling Vulnerabilities through Game Theory, accessed January 6, 2026, [https://www.researchgate.net/publication/394618004\_Social\_Engineering\_in\_Distributed\_Teams\_Modeling\_Vulnerabilities\_through\_Game\_Theory](https://www.researchgate.net/publication/394618004_Social_Engineering_in_Distributed_Teams_Modeling_Vulnerabilities_through_Game_Theory)  
40. Threat Model \- Cryptee, accessed January 6, 2026, [https://crypt.ee/threat-model](https://crypt.ee/threat-model)  
41. Secret-free security: a survey and tutorial, accessed January 6, 2026, [https://d-nb.info/1258706695/34](https://d-nb.info/1258706695/34)  
42. (PDF) Secret-free security: a survey and tutorial \- ResearchGate, accessed January 6, 2026, [https://www.researchgate.net/publication/358974231\_Secret-free\_security\_a\_survey\_and\_tutorial](https://www.researchgate.net/publication/358974231_Secret-free_security_a_survey_and_tutorial)  
43. SIMPL Systems as a Keyless Cryptographic and Security Primitive \- ResearchGate, accessed January 6, 2026, [https://www.researchgate.net/publication/221351317\_SIMPL\_Systems\_as\_a\_Keyless\_Cryptographic\_and\_Security\_Primitive](https://www.researchgate.net/publication/221351317_SIMPL_Systems_as_a_Keyless_Cryptographic_and_Security_Primitive)  
44. PUF Constructions with Limited Information Leakage \- Lirias, accessed January 6, 2026, [https://lirias.kuleuven.be/retrieve/d5099a98-6786-4908-9040-4c781cd3d788](https://lirias.kuleuven.be/retrieve/d5099a98-6786-4908-9040-4c781cd3d788)  
45. For Wo2022184587a1 | PDF \- Scribd, accessed January 6, 2026, [https://www.scribd.com/document/921498169/FOR-WO2022184587A1](https://www.scribd.com/document/921498169/FOR-WO2022184587A1)  
46. A Lightweight Authentication and Key Distribution Protocol for XR Glasses Using PUF and Cloud-Assisted ECC \- MDPI, accessed January 6, 2026, [https://www.mdpi.com/1424-8220/26/1/217](https://www.mdpi.com/1424-8220/26/1/217)  
47. Breaking through fixed PUF block limitations with differential sequence coding and convolutional codes | Request PDF \- ResearchGate, accessed January 6, 2026, [https://www.researchgate.net/publication/262170835\_Breaking\_through\_fixed\_PUF\_block\_limitations\_with\_differential\_sequence\_coding\_and\_convolutional\_codes](https://www.researchgate.net/publication/262170835_Breaking_through_fixed_PUF_block_limitations_with_differential_sequence_coding_and_convolutional_codes)  
48. F.vision: Browser Fingerprint Leak Testing Platform \- DataDome, accessed January 6, 2026, [https://datadome.co/anti-detect-tools/f-vision/](https://datadome.co/anti-detect-tools/f-vision/)  
49. Cross-Browser Fingerprinting 2025, accessed January 6, 2026, [https://www.rsinc.com/cross-browser-fingerprinting.php](https://www.rsinc.com/cross-browser-fingerprinting.php)  
50. EventChain: a blockchain framework for secure, privacy-preserving event verification | Request PDF \- ResearchGate, accessed January 6, 2026, [https://www.researchgate.net/publication/366478218\_EventChain\_a\_blockchain\_framework\_for\_secure\_privacy-preserving\_event\_verification](https://www.researchgate.net/publication/366478218_EventChain_a_blockchain_framework_for_secure_privacy-preserving_event_verification)  
51. © 2021 Tianyuan Liu \- IDEALS, accessed January 6, 2026, [https://www.ideals.illinois.edu/bitstream/handle/2142/113059/LIU-DISSERTATION-2021.pdf?sequence=1](https://www.ideals.illinois.edu/bitstream/handle/2142/113059/LIU-DISSERTATION-2021.pdf?sequence=1)  
52. DiVerify: Hardening Identity-Based Software Signing with Diverse-Context Scopes \- arXiv, accessed January 6, 2026, [https://arxiv.org/html/2406.15596v3](https://arxiv.org/html/2406.15596v3)  
53. (PDF) Dymo: Tracking Dynamic Code Identity \- ResearchGate, accessed January 6, 2026, [https://www.researchgate.net/publication/221427476\_Dymo\_Tracking\_Dynamic\_Code\_Identity](https://www.researchgate.net/publication/221427476_Dymo_Tracking_Dynamic_Code_Identity)  
54. Dymo: Tracking Dynamic Code Identity \- UCSB Computer Science, accessed January 6, 2026, [https://sites.cs.ucsb.edu/\~chris/research/doc/raid11\_dymo.pdf](https://sites.cs.ucsb.edu/~chris/research/doc/raid11_dymo.pdf)  
55. RFC 5869 \- HMAC-based Extract-and-Expand Key Derivation Function (HKDF), accessed January 6, 2026, [https://datatracker.ietf.org/doc/html/rfc5869](https://datatracker.ietf.org/doc/html/rfc5869)  
56. HKDF | Guide to HMAC-based Key Derivation Function \- HashMama, accessed January 6, 2026, [https://hashmama.com/docs/hkdf/](https://hashmama.com/docs/hkdf/)  
57. rfc:improve\_hash\_hkdf\_parameter \- PHP, accessed January 6, 2026, [https://wiki.php.net/rfc/improve\_hash\_hkdf\_parameter](https://wiki.php.net/rfc/improve_hash_hkdf_parameter)  
58. Dynamic Cryptographic Key Generation Using Real-Time Environmental Data, accessed January 6, 2026, [https://www.ijset.in/wp-content/uploads/IJSET\_V13\_issue5\_573.pdf](https://www.ijset.in/wp-content/uploads/IJSET_V13_issue5_573.pdf)  
59. Remote Attestation Agent (RAA) \- Emergent Mind, accessed January 6, 2026, [https://www.emergentmind.com/topics/remote-attestation-agent-raa](https://www.emergentmind.com/topics/remote-attestation-agent-raa)  
60. BGCFI: Efficient Verification in Fine-grained Control-Flow Integrity based on Bipartite Graph \- IEEE Xplore, accessed January 6, 2026, [https://ieeexplore.ieee.org/iel7/6287639/6514899/10005286.pdf](https://ieeexplore.ieee.org/iel7/6287639/6514899/10005286.pdf)  
61. CEFI: Command Execution Flow Integrity for Embedded Devices \- vusec, accessed January 6, 2026, [https://download.vusec.net/papers/cefi\_dimva23.pdf](https://download.vusec.net/papers/cefi_dimva23.pdf)  
62. Protecting Information with Cybersecurity \- PMC \- PubMed Central \- NIH, accessed January 6, 2026, [https://pmc.ncbi.nlm.nih.gov/articles/PMC7122347/](https://pmc.ncbi.nlm.nih.gov/articles/PMC7122347/)  
63. A Learning-Based Approach to Reactive Security \- Stanford CS Theory, accessed January 6, 2026, [https://theory.stanford.edu/people/jcm/papers/proceedings-fc2010.pdf](https://theory.stanford.edu/people/jcm/papers/proceedings-fc2010.pdf)  
64. Can Verifiable Delay Functions Be Based on Random Oracles? \- DROPS, accessed January 6, 2026, [https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.ICALP.2020.83](https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.ICALP.2020.83)  
65. Verifiable Delay Function and Its Blockchain-Related Application: A Survey \- PMC \- NIH, accessed January 6, 2026, [https://pmc.ncbi.nlm.nih.gov/articles/PMC9571642/](https://pmc.ncbi.nlm.nih.gov/articles/PMC9571642/)  
66. Introduction to Verifiable Delay Functions (VDFs) \- The Trail of Bits Blog, accessed January 6, 2026, [https://blog.trailofbits.com/2018/10/12/introduction-to-verifiable-delay-functions-vdfs/](https://blog.trailofbits.com/2018/10/12/introduction-to-verifiable-delay-functions-vdfs/)  
67. Implementation Study of Two Verifiable Delay Functions, accessed January 6, 2026, [https://d-nb.info/136534441X/34](https://d-nb.info/136534441X/34)  
68. The Work‐Averse Cyberattacker Model: Theory and Evidence from Two Million Attack Signatures \- PMC \- PubMed Central, accessed January 6, 2026, [https://pmc.ncbi.nlm.nih.gov/articles/PMC9543271/](https://pmc.ncbi.nlm.nih.gov/articles/PMC9543271/)  
69. \&OLHQW 3X\]\]OHV $ \&U\\SWRJUDSKLF \&RXQWHUPHDVXUH $JDLQVW \&RQQHFWLRQ 'HSOHWLRQ $WWDFNV ,QWURGXFWLRQ \- NDSS Symposium, accessed January 6, 2026, [https://www.ndss-symposium.org/wp-content/uploads/2017/09/A-Cryptographic-Defense-Against-Connection-Depletion-Attacks-Ari-Juels.pdf](https://www.ndss-symposium.org/wp-content/uploads/2017/09/A-Cryptographic-Defense-Against-Connection-Depletion-Attacks-Ari-Juels.pdf)  
70. Fighting Bots with the Client-Puzzle Protocol \- Little Man In My Head \- WordPress.com, accessed January 6, 2026, [https://littlemaninmyhead.wordpress.com/2020/09/20/fighting-bots-with-the-client-puzzle-protocol/](https://littlemaninmyhead.wordpress.com/2020/09/20/fighting-bots-with-the-client-puzzle-protocol/)  
71. Requirements for Client Puzzles to Defeat the Denial of Service and the Distributed Denial of Service Attacks, accessed January 6, 2026, [https://ccis2k.org/iajit/PDF/vol.3,no.4/8-abdulmotaleb.pdf](https://ccis2k.org/iajit/PDF/vol.3,no.4/8-abdulmotaleb.pdf)  
72. Onions Got Puzzled: On the Challenges of Mitigating Denial-of-Service Problems in Tor Onion Services \- USENIX, accessed January 6, 2026, [https://www.usenix.org/system/files/conference/usenixsecurity25/sec25cycle1-prepub-343-lee.pdf](https://www.usenix.org/system/files/conference/usenixsecurity25/sec25cycle1-prepub-343-lee.pdf)  
73. The New Cybercrime ROI: Why AI Makes Exploitation Trivial, and Why Reconnaissance is Now the Attacker's Most Valuable Asset \- Dr Logic, accessed January 6, 2026, [https://drlogic.com/article/the-new-cybercrime-roi-why-ai-makes-exploitation-trivial-and-why-reconnaissance-is-now-the-attackers-most-valuable-asset/](https://drlogic.com/article/the-new-cybercrime-roi-why-ai-makes-exploitation-trivial-and-why-reconnaissance-is-now-the-attackers-most-valuable-asset/)  
74. Crypto Glossary \- Cryptopedia | Gemini, accessed January 6, 2026, [https://www.gemini.com/cryptopedia/glossary](https://www.gemini.com/cryptopedia/glossary)  
75. GlassDB: An Efficient Verifiable Ledger Database System Through Transparency \- VLDB Endowment, accessed January 6, 2026, [https://www.vldb.org/pvldb/vol16/p1359-ooi.pdf](https://www.vldb.org/pvldb/vol16/p1359-ooi.pdf)  
76. Append-only \- Grokipedia, accessed January 6, 2026, [https://grokipedia.com/page/Append-only](https://grokipedia.com/page/Append-only)  
77. LogStamping: A blockchain-based log auditing approach for large-scale systems, accessed January 6, 2026, [https://www.researchgate.net/publication/399148908\_LogStamping\_A\_blockchain-based\_log\_auditing\_approach\_for\_large-scale\_systems](https://www.researchgate.net/publication/399148908_LogStamping_A_blockchain-based_log_auditing_approach_for_large-scale_systems)  
78. eSignatures \- SendTurtle, accessed January 6, 2026, [https://sendturtle.com/feature/esignatures/](https://sendturtle.com/feature/esignatures/)  
79. Article: A secure NFC mobile payment protocol based on biometrics with formal verification Journal \- Inderscience Publishers, accessed January 6, 2026, [https://www.inderscience.com/info/inarticle.php?artid=78579](https://www.inderscience.com/info/inarticle.php?artid=78579)  
80. Verifiable Cognition: Blockchain as the Immutable Memory Layer for Artificial Intelligence | Uplatz Blog, accessed January 6, 2026, [https://uplatz.com/blog/verifiable-cognition-blockchain-as-the-immutable-memory-layer-for-artificial-intelligence/](https://uplatz.com/blog/verifiable-cognition-blockchain-as-the-immutable-memory-layer-for-artificial-intelligence/)  
81. What Is Blockchain? \- Post \- Taft School, accessed January 6, 2026, [https://www.taftschool.org/about/taft-voices/post/\~board/alumni/post/what-is-blockchain](https://www.taftschool.org/about/taft-voices/post/~board/alumni/post/what-is-blockchain)  
82. Attack graph-based security metrics: Concept, taxonomy, challenges and open issues \- BIO Web of Conferences, accessed January 6, 2026, [https://www.bio-conferences.org/articles/bioconf/pdf/2024/16/bioconf\_iscku2024\_00085.pdf](https://www.bio-conferences.org/articles/bioconf/pdf/2024/16/bioconf_iscku2024_00085.pdf)  
83. Adversary Work Factor as a Metric for Information Assu rance\*, accessed January 6, 2026, [https://www.nspw.org/papers/2000/nspw2000-schudel.pdf](https://www.nspw.org/papers/2000/nspw2000-schudel.pdf)  
84. Understanding the Economics Behind Cyber Attacks \- Whitepapers \- The Register, accessed January 6, 2026, [https://whitepapers.theregister.com/paper/view/12960/attacker-economics-understanding-the-economics-behind-cyber-attacks](https://whitepapers.theregister.com/paper/view/12960/attacker-economics-understanding-the-economics-behind-cyber-attacks)  
85. Cognitive AI for Behavioral Biometrics in Multi-Layered Authentication Protocols, accessed January 6, 2026, [https://www.researchgate.net/publication/393784823\_Cognitive\_AI\_for\_Behavioral\_Biometrics\_in\_Multi-Layered\_Authentication\_Protocols](https://www.researchgate.net/publication/393784823_Cognitive_AI_for_Behavioral_Biometrics_in_Multi-Layered_Authentication_Protocols)  
86. Hybrid deep learning-enabled framework for enhancing security, data integrity, and operational performance in Healthcare Internet of Things (H-IoT) environments \- PMC, accessed January 6, 2026, [https://pmc.ncbi.nlm.nih.gov/articles/PMC12374995/](https://pmc.ncbi.nlm.nih.gov/articles/PMC12374995/)  
87. Measuring Cybersecurity ROI: A Framework For 2026 Decision-Makers \- Safe Security, accessed January 6, 2026, [https://safe.security/resources/blog/measuring-cybersecurity-roi-a-framework-for-2026-decision-makers/](https://safe.security/resources/blog/measuring-cybersecurity-roi-a-framework-for-2026-decision-makers/)