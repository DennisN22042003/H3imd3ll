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

---

# 📁 Project Structure Overview

## 🔧 Top-Level Files

### `Cargo.toml`
Rust’s package manifest.

- Defines dependencies (`petgraph`, `serde`, etc.), binary/library targets, and features.
- Configure the CLI and optionally expose `lib.rs` as a crate for external usage.

### `README.md`
Entry point for anyone looking at your project.

- Should explain what **Gotham-lite** is.
- Include installation instructions, usage examples, and links to documentation.

---

## 📦 `src/` – Application Source Code

### `main.rs` — CLI Entry Point
- Starts the application when run as a binary.
- Parses command-line arguments (via `clap` or `argh`).
- Dispatches commands to appropriate handlers in `cli/`.

### `lib.rs` — Public Library Interface
- Allows **Gotham-lite** to be used as a library.
- Exports major modules like `graph`, `engine`, and `io`.
- Defines what’s publicly usable if published as a crate.

---

## 🔗 `graph/` – Core Graph Logic
This is the heart of **Gotham-lite**. You model and manage the knowledge graph here.

- `mod.rs`: Declares the graph module and re-exports submodules.
- `graph.rs`: Wraps `petgraph`, manages node/edge creation, updates, lookups.
- `entity.rs`: Defines node types like `Person`, `Company`, `PhoneNumber`.
- `relationship.rs`: Defines edge metadata and relationship types (`WorksAt`, `Calls`, `LinkedTo`).
- `enrichment.rs`: Handles deduplication, normalization, and metadata enrichment.

#### ✅ You need to:
- Define robust `Entity` and `Relationship` structs.
- Implement a `GraphStore` or wrapper over `petgraph::Graph`.
- Support CRUD on nodes/edges.
- Handle deduplication + enrichment.

---

## 🔄 `io/` – Importers / Exporters
Handles ingestion and output of data formats.

- `json_loader.rs`: Load graph data from JSON.
- `csv_loader.rs`: Load structured data (e.g., phone records) from CSV.
- `graphviz.rs`: Export graph to DOT format for Graphviz.
- `snapshot.rs`: Save/load the full graph as `.bin` or `.json` (e.g., with `serde`).

#### ✅ You need to:
- Parse datasets into `Entity` + `Relationship`.
- Serialize/deserialize graphs (consider versioning).
- Implement import pipelines for structured real-world data.

---

## 🔍 `engine/` – Query & Intelligence Logic
This is your brain — where investigations and analysis happen.

- `search.rs`: Match nodes/relationships by attributes (e.g., fuzzy name search).
- `case.rs`: Pin nodes to investigations; label/group them.
- `timeline.rs`: Filter graph events/connections by time.
- `utils.rs`: Shared helpers (sorting, hashing, formatting, etc.).

#### ✅ You need to:
- Implement graph traversal, shortest path, ego networks.
- Build case tagging/pinning system.
- Add time-based filters and context-aware queries.

---

## 💻 `cli/` – Command Line Interface
This is your user interface for now.

- `commands.rs`: Maps commands like `search`, `show-path`, `pin` to the appropriate logic.
- `repl.rs`: Implements a REPL (interactive shell).
- `mod.rs`: Declares the CLI module and organizes subcommands.

#### ✅ You need to:
- Build user-friendly CLI commands (sensible defaults, help output).
- Pipe command input/output to `engine` and `graph`.
- Build REPL with command parsing + history (e.g., using `rustyline` or `reedline`).

---

## 🖼️ `ui/` – Optional Visual Interface
Future-facing; useful for visual analysts or demos.

- `app.rs`: Launch the UI (e.g., with `egui`, `tauri`, etc.).
- `graph_view.rs`: Renders graph nodes/edges visually.
- `entity_view.rs`: Inspect or edit a node.
- `timeline_view.rs`: Time-based visual filtering.

#### ✅ You can defer this:
- Build after CLI and engine are working.
- Useful for demos or non-technical analysts.

---

## 🧪 `tests/` – Integration and Unit Tests
- Unit tests for logic in each module (`graph`, `engine`, etc.).
- Integration tests for real workflows (load dataset, search, pin, etc.).

---

## 📁 `assets/` – Reference Datasets and Demos
- `example.csv`: Sample dataset for demos/tests.
- `graph.dot`: Example exported graph.
- `test.json`: Input for validating I/O.

---

## 💾 `data/` – Saved Graph Snapshots
- Local directory where analysts can save in-progress work (serialized graphs).
- Think of it as a local project folder.

---

## 📚 `docs/` – Documentation

- `architecture.md`: System design, diagrams, component roles.
- `use-cases.md`: Analyst workflows, real-world examples, usage tips.
