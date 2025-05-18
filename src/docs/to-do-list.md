#### 19/5/2025

# 🧠 1. Event Log (Immutable Fact Store)

Your current design mutates the graph directly in memory via CRUD operations — entities are created or updated *"in place."*  
But in an **event-sourced architecture**, the graph isn't updated directly. Instead, every change (like creating an entity, updating attributes, or forming a relationship) is recorded as an **immutable event** in an **append-only event log**.

> **This log becomes the single source of truth.**  
> The in-memory graph is just a **materialized view**, rebuilt by replaying these events.

Each event would minimally contain:
- A **timestamp**
- A **type** (e.g., entity creation)
- A **payload** (e.g., the entity’s details)

**Why this matters:**
- ✅ Enables **reproducibility**: you can always rebuild the graph from scratch.
- ✅ Guarantees **historical integrity**: nothing gets overwritten or lost.
- ✅ Supports **audit trails** and **undo/rollback operations**.

---

# 🕒 2. Temporal Validity in Relationships

Right now, relationships in your system are static — they exist or they don’t.  
But in a **temporal graph**, relationships have **time-bound validity**.

Each edge should optionally include:
- A **start** timestamp
- An **end** timestamp  
  Indicating **when** the relationship was valid.

> Example: “Who did Alice work with in 2020?”

**Technical implications:**
- `relationship.rs` must evolve to include temporal metadata.
- `engine/timeline.rs` needs to apply time filters during traversal.

---

# 📦 3. Versioned Entities

Currently, an entity (e.g., a `Person`) is a single mutable object.  
But in a **temporal system**, we want **version history**.

Each time an entity is updated:
- ❌ Don’t overwrite it.
- ✅ **Append a new version**, stamped with a creation time.

> Think of it as a vector or timeline of versions.

**Enables queries like:**
- “What did Bob’s profile look like before 2022?”

**Integration:**
- `entity.rs` must support version tracking internally.
- `search.rs` can include flags like `--latest` or `--as-of`.

---

# ♻️ 4. Event Replay / Graph Rebuilder

Since the graph is just a projection of the event log, you need a way to **reconstruct** it by replaying all events.

> On startup: don’t load a saved graph — replay the log to rebuild the in-memory state.

Each event is interpreted:
- Create a node
- Add a version
- Connect two nodes

**Where this fits:**
- New module: `engine/replay.rs` or `graph/builder.rs`
- Hooks into `GraphDb::new()` or `main.rs` to bootstrap runtime view

---

# 🧊 5. Batch Materialization (Snapshots)

Event logs grow indefinitely. Replaying everything from scratch gets slow.  
**Solution:** periodically materialize the graph into a **snapshot**.

> Think of it like a **database checkpoint**.

**On startup:**
- Load the last snapshot
- Replay only events after the snapshot

**Benefits:**
- 🚀 Faster startup
- ✅ Preserves event sourcing

**Handled in:**
- `io/snapshot.rs` for saving/loading the graph
- Possibly a **snapshot manifest** storing the last replayed timestamp

---

# 🧠 6. Time-Aware Query Engine

This architecture enables **temporal queries** — questions that can’t be asked of a static, mutable graph.

Examples:
- “Who was Alice connected to on *January 5th, 2022*?”
- “How did Bob’s employer history change over time?”
- “What was the state of the network just before a certain event?”

**To support this:**
- Extend `engine/timeline.rs` to handle time-slicing
- Implement **time-aware traversals** that skip invalid relationships at the target timestamp
