# "H3imd3ll" Architecture

H3imd3ll is a graph-first OSINT and investigative intelligence platform optimized for tracking entities, relationships, and temporal events. It is designed for analysts to ingest, search, and map connections in large datasets using a CLI or GUI interface.

---

## 🧱 Core Principles

- **Entity-Relationship Modeling**: Every piece of information is modeled as a node (entity) or edge (relationship).
- **In-Memory Graph Engine**: Powered by `petgraph`, all graph mutations and traversals occur in-memory for high-speed analysis.
- **Pluggable I/O System**: Data importers and exporters (JSON, CSV, DOT) are modular and extensible.
- **Analyst-First Design**: Interfaces prioritize practical use cases: tracing people, organizations, events, assets, timelines, and social graphs.

---

## 🧭 High-Level Overview

```text
                ┌──────────────┐
                │ CLI / REPL   │
                └──────┬───────┘
                       │
                       ▼
                ┌──────────────┐
                │ Engine Layer │◄────────────┐
                └──────┬───────┘             │
                       │                     │
                       ▼                     │
              ┌────────────────┐             │
              │ Graph Database │────────────►│
              │ (petgraph)     │             │
              └────────────────┘             │
                       ▲                     │
     ┌────────────────┴───────┐     ┌────────▼─────────┐
     │ Importers / Exporters  │     │ Case Management  │
     │ (CSV, JSON, Graphviz)  │     │ (Pinning, Labels)│
     └────────────────────────┘     └──────────────────┘

```
## 🧩 Modules

### 1. `graph/` - Core Graph Engine
- Maintains an in-memory `Graph<Entity, Relationship>` using [`petgraph`](https://docs.rs/petgraph/)
- Manages graph mutation (e.g., `add_entity`, `link_entities`)
- Handles internal indexing, merging, and deduplication

---

### 2. `io/` - Data I/O
- Import raw datasets (CSV, JSON)
- Export Graphviz visualizations (`.dot` format)
- Save/load complete graph snapshots (serialization)

---

### 3. `engine/` - Intelligence & Query Layer
- Search logic (entity search, graph traversal, relationship discovery)
- Timeline filtering
- Case pinning and tagging
- Custom pathfinding (e.g., shortest path, connection trees)

---

### 4. `cli/` - Analyst Command Line Interface
- REPL and static command-line interface
- Key commands include:
    - `search <query>`
    - `show-path <entity_a> <entity_b>`
    - `pin <entity> --case <name>`
    - `export --format dot`

---

### 5. `ui/` *(optional)* - Visual Interface
- Planned egui-based or web frontend
- Features (future scope):
    - Graph explorer
    - Timeline browser
    - Entity card viewer

---

## 🧠 How It Works 

- `Entities` like `people, companies, events, etc.` are modeled as `nodes`. `Relationships (e.g. "attended", "employed_by", "owns")` are modeled as `edges`. **All data lives in a fast in-memory graph powered by petgraph**.

- Use the CLI to explore connections, pin entities to cases, or visualize networks. 