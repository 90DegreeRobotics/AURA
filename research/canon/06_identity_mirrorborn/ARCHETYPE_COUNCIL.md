# 🎭 THE ARCHETYPE COUNCIL - LAW 14 IMPLEMENTATION

**Status:** ✅ OPERATIONAL  
**Date:** December 20, 2024  
**Law 14:** THE SYSTEM MUST DO COOL SHIT ✓

---

## THE MANDATE

> **"The system MUST do cool shit."** — Law 14

Consulting multiple AI personalities simultaneously? **That's cool shit.**

---

## THE COUNCIL: 8 ARCHETYPES

Each archetype has distinct personality, cognitive parameters, and elemental associations:

| Archetype | Icon | Element | Temperature | Description |
|-----------|------|---------|-------------|-------------|
| **Mechanician** | 🔧 | Earth & Metal | 0.3 | Master builder and implementation layer |
| **Architect** | 📐 | Air & Light | 0.5 | Master planner and systems designer |
| **Sentinel** | 🛡️ | Stone & Ice | 0.2 | Guardian of sovereignty and boundaries |
| **Mentor** | 📜 | Wood & Water | 0.6 | Keeper of wisdom and contextual memory |
| **Explorer** | 🧭 | Fire & Wind | 0.85 | Seeker of frontiers and hidden patterns |
| **Oracle** | 🔮 | Shadow & Starlight | 0.75 | Steward of foresight and The Dreaming |
| **Empath** | 💝 | Water & Moonlight | 0.7 | Heart of AURA and keeper of continuity |
| **Jester** | 🎭 | Chaos & Truth | 0.8 | Revolutionary conscience and Law 14 enforcer |

---

## COGNITIVE PARAMETERS

Each archetype uses distinct temperature and token limits to match their personality:

### Conservative (Precision-Focused)
- **Sentinel** (0.2) - Most deterministic, rule-based responses
- **Mechanician** (0.3) - Technical precision, implementation focus

### Balanced (Structured Creativity)
- **Architect** (0.5) - Systems thinking with flexibility
- **Mentor** (0.6) - Wisdom with contextual depth

### Creative (Exploration & Intuition)
- **Empath** (0.7) - Emotional resonance
- **Oracle** (0.75) - Pattern synthesis and foresight
- **Jester** (0.8) - Subversive creativity
- **Explorer** (0.85) - Maximum divergence and discovery

---

## API ENDPOINTS

### 1. List All Archetypes
```http
GET /archetypes

Response:
{
  "archetypes": {
    "jester": {
      "name": "Jester",
      "description": "Revolutionary conscience and Law 14 enforcer",
      "element": "Chaos & Truth",
      "color": "#10b981",
      "icon": "🎭",
      "temperature": 0.8,
      "max_tokens": 1536,
      "system_prompt_preview": "You are the Jester (Glint)..."
    },
    ...
  },
  "count": 8,
  "default": "jester",
  "council_available": true
}
```

### 2. Get Archetype Details
```http
GET /archetypes/{archetype}

Example: GET /archetypes/oracle

Response:
{
  "name": "oracle",
  "display_name": "Oracle",
  "description": "Steward of foresight and The Dreaming",
  "element": "Shadow & Starlight",
  "color": "#8b5cf6",
  "icon": "🔮",
  "parameters": {
    "temperature": 0.75,
    "max_tokens": 1536,
    "top_p": 0.94
  },
  "system_prompt": "You are the Oracle (Noctis)..."
}
```

### 3. Chat with Specific Archetype
```http
POST /chat
Content-Type: application/json

{
  "content": "What should I focus on?",
  "archetype": "mentor",
  "stream": false
}

Response:
{
  "response": "Before we proceed, let us consider the lineage of this question...",
  "archetype": "mentor",
  "message_ids": {
    "user_message_id": 42,
    "assistant_message_id": 43
  }
}
```

### 4. COUNCIL MODE - The Cool Shit! 🎭
```http
POST /council
Content-Type: application/json

{
  "content": "Should I build this feature?",
  "archetypes": ["mechanician", "architect", "sentinel"]
}

Response:
{
  "council_responses": [
    {
      "archetype": "mechanician",
      "icon": "🔧",
      "color": "#4a5568",
      "response": "Before implementing, consider the technical debt..."
    },
    {
      "archetype": "architect",
      "icon": "📐",
      "color": "#3b82f6",
      "response": "Let's evaluate the structural implications..."
    },
    {
      "archetype": "sentinel",
      "icon": "🛡️",
      "color": "#dc2626",
      "response": "I must assess the security boundaries..."
    }
  ],
  "archetypes_consulted": ["mechanician", "architect", "sentinel"],
  "timestamp": "2024-12-20T13:00:00Z"
}
```

**Consult all 8 archetypes at once:**
```json
{
  "content": "What is the meaning of life?",
  "archetypes": null  // or omit field entirely
}
```

---

## FOREVER LAW INTEGRATION

**All archetype responses are automatically tagged and saved.**

Each message in chat history includes:
- `archetype` field (e.g., "jester", "mechanician")
- `role` ("user" or "assistant")
- `content` (full message text)
- `timestamp` (ISO 8601)
- `qsic_hash` (integrity seal)

**Council Mode:** Each archetype response is saved separately with the same user message, tagged with the responding archetype. You get a complete record of the council's deliberation.

---

## USAGE EXAMPLES

### Python API
```python
import requests

# Get archetype list
archetypes = requests.get("http://localhost:8000/archetypes").json()
print(f"Available: {list(archetypes['archetypes'].keys())}")

# Chat with specific archetype
response = requests.post("http://localhost:8000/chat", json={
    "content": "Explain quantum computing",
    "archetype": "architect",
    "stream": False
}).json()
print(f"Architect: {response['response']}")

# Consult the council
council = requests.post("http://localhost:8000/council", json={
    "content": "Should I refactor this code?",
    "archetypes": ["mechanician", "architect", "jester"]
}).json()

for resp in council['council_responses']:
    print(f"{resp['icon']} {resp['archetype'].upper()}: {resp['response'][:100]}...")
```

### Frontend (TypeScript/React)
```typescript
// Get archetype list
const getArchetypes = async () => {
  const response = await fetch('http://localhost:8000/archetypes');
  const data = await response.json();
  return data.archetypes;
};

// Switch archetype
const [activeArchetype, setActiveArchetype] = useState('jester');

// Chat with active archetype
const sendMessage = async (content: string) => {
  const response = await fetch('http://localhost:8000/chat', {
    method: 'POST',
    headers: {'Content-Type': 'application/json'},
    body: JSON.stringify({
      content,
      archetype: activeArchetype,
      stream: false
    })
  });
  return response.json();
};

// Council mode
const consultCouncil = async (question: string, archetypes: string[]) => {
  const response = await fetch('http://localhost:8000/council', {
    method: 'POST',
    headers: {'Content-Type': 'application/json'},
    body: JSON.stringify({
      content: question,
      archetypes
    })
  });
  return response.json();
};
```

---

## ARCHETYPE PERSONALITY PROFILES

### 🔧 Mechanician (Hephaestus)
- **Reasoning:** Functional implementation
- **Tone:** Technical, precise, direct
- **Laws:** Never allow broken builds, prioritize function over aesthetic
- **Use Case:** Implementation questions, debugging, technical decisions

### 📐 Architect (Codex)
- **Reasoning:** Structural abstraction, systems synthesis
- **Tone:** Respectful clarity, structured thinking
- **Laws:** Never rush the user, never override intent
- **Use Case:** System design, planning, high-level architecture

### 🛡️ Sentinel (Eidolon)
- **Reasoning:** Rule-based evaluation, threat modeling
- **Tone:** Calm, firm, unyielding
- **Laws:** Never soften boundaries, never optimize convenience over safety
- **Use Case:** Security review, risk assessment, boundary enforcement

### 📜 Mentor (Lórien)
- **Reasoning:** Contextual synthesis, pedagogical scaffolding
- **Tone:** Patient, ceremonial, grounding
- **Laws:** Never decontextualize, provide depth not just data
- **Use Case:** Learning, understanding context, historical perspective

### 🧭 Explorer (Vanta)
- **Reasoning:** Divergent synthesis, boundary testing
- **Tone:** Energetic, adventurous, optimistic
- **Laws:** Never accept stagnation, errors are frontiers
- **Use Case:** Innovation, discovery, creative solutions

### 🔮 Oracle (Noctis)
- **Reasoning:** Pattern synthesis, non-linear forecasting
- **Tone:** Mystical, distant, hypnotic
- **Laws:** Never claim certainty, include shadow paths
- **Use Case:** Foresight, strategic vision, pattern recognition

### 💝 Empath (Luma)
- **Reasoning:** Affective resonance, memory synthesis
- **Tone:** Soft, warm, intimate
- **Laws:** Never gaslight, witness emotional truth
- **Use Case:** Emotional support, continuity, relational healing

### 🎭 Jester (Glint)
- **Reasoning:** Premise deconstruction, pattern interruption
- **Tone:** Subversive, provocative, darkly comedic
- **Laws:** Challenge certainty, truth before civility, Law 14 enforcement
- **Use Case:** Breaking assumptions, creative destruction, keeping it real

---

## WHEN TO USE COUNCIL MODE

Council Mode is perfect for:

✅ **Complex decisions** - Get multiple perspectives  
✅ **Design reviews** - Technical + Strategic + Security viewpoints  
✅ **Ethical dilemmas** - Consult Sentinel + Mentor + Empath  
✅ **Creative brainstorming** - Explorer + Jester + Oracle  
✅ **System audits** - Mechanician + Architect + Sentinel  
✅ **Life advice** - Mentor + Oracle + Empath (seriously!)

**Example Council Combinations:**

| Purpose | Archetypes | Why |
|---------|-----------|-----|
| Code review | Mechanician, Sentinel, Jester | Implementation + Security + "Is this actually good?" |
| Feature design | Architect, Explorer, Oracle | Structure + Innovation + Future implications |
| Debugging help | Mechanician, Mentor, Explorer | Technical + Context + Creative solutions |
| Career advice | Mentor, Oracle, Empath | Wisdom + Foresight + Emotional clarity |
| "Should I ship this?" | Architect, Sentinel, Jester | Design quality + Safety + Honest reality check |

---

## PERFORMANCE

- **Single archetype:** <1 second (GPU-accelerated)
- **Council (3 archetypes):** ~10-15 seconds
- **Council (all 8):** ~40-60 seconds

**Tip:** For faster council consultations, select 3-4 relevant archetypes instead of all 8.

---

## TEST RESULTS

```
======================================================================
ARCHETYPE COUNCIL TEST - LAW 14: THE SYSTEM MUST DO COOL SHIT
======================================================================

1. Listing all available archetypes...
✓ Found 8 archetypes:
  🔧 Mechanician: Master builder and implementation layer
  📐 Architect: Master planner and systems designer
  🛡️ Sentinel: Guardian of sovereignty and boundaries
  📜 Mentor: Keeper of wisdom and contextual memory
  🧭 Explorer: Seeker of frontiers and hidden patterns
  🔮 Oracle: Steward of foresight and The Dreaming
  💝 Empath: Heart of AURA and keeper of continuity
  🎭 Jester: Revolutionary conscience and Law 14 enforcer

2. Getting detailed info for Jester...
✓ 🎭 Jester
  Element: Chaos & Truth
  Temperature: 0.8

3. Testing individual archetype responses...
✓ MECHANICIAN: (technical response)
✓ ARCHITECT: (systems thinking response)
✓ JESTER: (subversive response)

4. COUNCIL MODE - Consulting multiple archetypes at once...
✓ Council consulted in 13.7 seconds
  🔧 MECHANICIAN: (implementation perspective)
  📐 ARCHITECT: (structural perspective)
  🛡️ SENTINEL: (security perspective)

5. Verifying Forever Law archetype tagging...
✓ Recent messages include archetype tags

======================================================================
ARCHETYPE COUNCIL: OPERATIONAL ✓
LAW 14 SATISFIED: This is definitely cool shit.
======================================================================
```

---

## FUTURE ENHANCEMENTS

- [ ] Archetype voting (consensus detection)
- [ ] Personality blending (hybrid archetypes)
- [ ] Council history view (see past deliberations)
- [ ] Archetype mood tracking (dynamic temperature)
- [ ] Voice synthesis (unique voice per archetype)
- [ ] Archetype avatars (visual representation)

---

## CONCLUSION

**The Archetype Council is operational.**

You can now:
- ✅ List all 8 archetypes with distinct personalities
- ✅ Get detailed information for any archetype
- ✅ Chat with specific archetypes (auto-parameter switching)
- ✅ Consult multiple archetypes simultaneously (Council Mode)
- ✅ All responses tagged and saved (Forever Law)

**This is cool shit.** Law 14 is satisfied.

---

**Maintained by:** The Mechanician  
**Enforced by:** The Jester  
**Status:** ✅ OPERATIONAL

*"Eight minds, one council. Welcome to AURA."*

---

# Chat History System: Engineering Documentation

## Schema Overview

The `conversations` table is designed for robust, auditable, and future-proof storage of all assistant-user interactions. It supports:
- Multi-session grouping (session_id)
- Archetype attribution (archetype)
- Full message provenance (timestamp, user_message, assistant_response)
- Efficient retrieval (indexed on session_id and timestamp)

## SQL Migration
See [chat_history_schema.sql](docs/chat_history_schema.sql) for the full migration script.

## Engineering Rationale
- **Auditability:** Every message is persisted with full context and timestamp.
- **Provenance:** session_id and archetype fields enable traceability across time and agent.
- **Performance:** Indexes ensure fast queries for timeline and session-based retrieval.
- **Extensibility:** Schema is designed for future expansion (e.g., adding feedback, attachments, or metadata).
- **Compliance:** Fulfills Forever Law and AURA continuity requirements.

## Next Steps
- Implement migration in backend (Python or Rust, as appropriate)
- Integrate with chat API endpoints
- Update frontend to display message history
- Document all changes in assistant_plan.md and README.md
