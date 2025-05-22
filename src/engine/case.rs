use uuid::Uuid;
use chrono::{DateTime, Local};
use std::collections::VecDeque;

use crate::graph::GraphDb;
use crate::graph::fact::Fact;
use crate::engine::utils::{sort_facts_by_time, deduplicate_facts};


/// Represents a logical grouping of related facts - a "case"
/// Examples: investigation, narrative, or any related cluster of entities and facts
#[derive(Debug)]
pub struct Case {
    pub id: Uuid,                       // Unique identifies for the case
    pub name: String,                   // Human-readable case name/title
    pub description: String,            // Optional textual summary or notes about the case
    pub created_at: DateTime<Local>,    // Timestamp of when the case was created
    pub related_entity_ids: Vec<Uuid>,  // List of Entity UUIDs involved in the case
    pub facts: Vec<Fact>,               // All Facts relevant to the case's entities
}

/// Builder pattern to construct a Case from a seed entity,
/// expanding through connected entities up to a max graph traversal depth,
/// optionally filtered by a time range.
pub struct CaseBuilder<'a> {
    db: &'a GraphDb,                // Reference to the graph database for querying entities & facts
    seed_entity_id: Uuid,           // Starting entity UUID to build the case around
    max_depth: usize,               // Maximum BFS traversal depth to collect related entities
    from: Option<DateTime<Local>>,  // Optional lower bound on timestamp to filter facts
    to: Option<DateTime<Local>>,    // Optional upper bound on timestamp to filter facts
}

impl Case {
    /// Create a new case instance with given name, description, related entities and facts.
    pub fn new(name: &str, description: &str, related_entity_ids: Vec<Uuid>, facts: Vec<Fact>) -> Self {
        Case {
            id: Uuid::new_v4(),
            name: name.to_string(),
            description: description.to_string(),
            created_at: Local::now(),
            related_entity_ids,
            facts,
        }
    }

    /// Filter this case's facts by an optional time window
    /// Returns all facts whose timestamp falls withing the `[from, to]` range (inclusive)
    pub fn filter_by_time(&self, from: Option<DateTime<Local>>, to: Option<DateTime<Local>>) -> Vec<Fact> {
        self.facts.iter()
            .filter(|fact| {
                let ts = fact.timestamp();

                // Use map_or(true, ...) so that if from or to is None, it does not filter out
                from.map_or(true, |f| ts >= f) && to.map_or(true, |t| ts <= t)
            })

            // Clone facts for returning a new Vec
            .cloned()
            .collect()
    }

    /// Check if a given entity UUID is involved in this case.
    /// Returns true if the entity is listed in related_entity_ids.
    pub fn involves_entity(&self, entity_id: &Uuid) -> bool {
        self.related_entity_ids.contains(entity_id)
    }
}

impl<'a> CaseBuilder<'a> {

    /// Initialize a new CaseBuilder for a given GraphDb reference and seed entity UUID.
    /// Sets default max_depth = 2 and no time filters.
    pub fn new(db: &'a GraphDb, seed_entity_id: Uuid) -> Self {
        Self {
            db,
            seed_entity_id,
            max_depth: 2,
            from: None,
            to: None,
        }
    }

    /// Set the maximum traversal depth for BFS search over related entities.
    /// Allow limiting how far the graph search should expand from the seed entity.
    pub fn with_max_depth(mut self, depth: usize) -> Self {
        self.max_depth = depth;
        self
    }

    /// Set optional time window filter for facts included in the case.
    /// Facts outside the `[from, to]` range will be excluded.
    pub fn with_time_range(mut self, from: Option<DateTime<Local>>, to: Option<DateTime<Local>>) -> Self {
        self.from = from;
        self.to = to;
        self
    }

    /// Perform breadth-first search (BFS) starting from the seed entity node
    /// in the graph to collect all related entities up to max_depth.
    ///
    /// Uses a HashSet to avoid duplicates and returns a Vec of unique entity UUIDs.
    fn collect_related_entities(&self) -> Vec<Uuid> {
        use std::collections::HashSet;

        // Result vector to collect related entity IDs
        let mut related = Vec::new();

        // Keep track of visited nodes (by UUID) to avoid revisiting cycles
        let mut visited = HashSet::new();

        // Find the node index in the graph corresponding to the seed_entity_id
        if let Some(&start_idx) = self.db.uuid_index_map.get(&self.seed_entity_id) {
            let graph = &self.db.graph;

            // Queue for BFS: stores (node index, current depth)
            let mut queue = VecDeque::new();

            // Start with the seed node at depth 0
            queue.push_back((start_idx, 0));

            // Begin BFS loop
            while let Some((node_idx, depth)) = queue.pop_front() {

                // Skip nodes beyond the max_depth
                if depth > self.max_depth {
                    continue;
                }

                // Access the entity at this node index
                if let Some(entity) = graph.node_weight(node_idx) {

                    // If this entity hasn't been visited yet
                    if visited.insert(entity.id) {

                        // Record the entity UUID
                        related.push(entity.id);

                        // Enqueue all neighbors with incremented depth
                        for neighbor in graph.neighbors(node_idx) {
                            queue.push_back((neighbor, depth + 1));
                        }
                    }
                }
            }
        }

        related
    }

    /// Build the Case Object:
    /// 1. Collect related entities from BFS traversal
    /// 2. Filter the global event log for Facts involving any of these entities
    ///    and falling within the optional time range.
    /// 3. Sort facts chronologically.
    /// 4. Deduplicate facts to avoid repetition.
    /// 5. Return the constructed Case.
    pub fn build(self, name: &str, description: &str) -> Case {
        // Collect all related entities connected to the seed entity
        let related_entities = self.collect_related_entities();

        // Filter event log facts that:
        // - Occur within time range (if set)
        // - Involve any of the related entities
        let mut relevant_facts: Vec<Fact> = self.db.event_log.iter()
            .filter(|fact| {
                let ts = fact.timestamp();

                // Check time range filter
                let in_time = self.from.map_or(true, |from| ts >= from)
                    && self.to.map_or(true, |to| ts <= to);

                // Check if fact involves any of the related entities
                in_time && fact.involves_any(&related_entities)
            })
            .cloned()
            .collect();

        // Sort facts chronologically for consistency
        sort_facts_by_time(&mut relevant_facts);

        // Remove duplicate facts (if any)
        relevant_facts = deduplicate_facts(relevant_facts);

        // Create and return the final Case object
        Case::new(name, description, related_entities, relevant_facts)
    }
}

pub fn display_case(case: &Case, db: &GraphDb) {
    println!("=== 📦Case: {} ===", case.name);
    println!("🆔 ID: {}", case.id);
    println!("🕒 Created At: {}", case.created_at.format("%Y-%m-%d %H:%M:%S"));
    println!("📝 Description: {}", case.description);
    println!("🔗 Related Entities ({}):", case.related_entity_ids.len());

    for id in &case.related_entity_ids {
        let label = db.graph.node_weights().find(|e| e.id == *id)
            .map(|e| format!("{} ({:?})", e.name, e.entity_type))
            .unwrap_or_else(|| "<Unknown>".to_string());

        println!("  - {}: {}", id, label);
    }

    println!("\n📚 Facts ({}):", case.facts.len());

    for fact in &case.facts {
        match fact {
           Fact::EntityCreated { entity_id, timestamp, .. } => {
                //println!("  [CREATE] {} '{}' at {}", entity.id, entity.name, timestamp.format("%Y-%m-%d %H:%M:%S"));
               println!("🆕  [CREATE] Entity {} at {}", entity_id, timestamp.format("%Y-%m-%d %H:%M:%S"));
            }
            Fact::EntityUpdated { entity_id, timestamp, .. } => {
                println!("🔄  [UPDATE] Entity {} at {}", entity_id, timestamp.format("%Y-%m-%d %H:%M:%S"));
            }
            Fact::RelationshipAdded { source_id, target_id, relationship_type, timestamp, .. } => {
                let source = db.graph.node_weights().find(|e| e.id == *source_id)
                    .map(|e| e.name.clone()).unwrap_or_else(|| "Unknown".to_string());
                let target = db.graph.node_weights().find(|e| e.id == *target_id)
                    .map(|e| e.name.clone()).unwrap_or_else(|| "Unknown".to_string());
                println!("🔗  [REL] {} --{}--> {} @ {}", source, relationship_type, target, timestamp.format("%Y-%m-%d %H:%M:%S"));
            }
            Fact::EntityDeleted {  entity_id, timestamp} => {
                println!("❌  [DELETE] Entity {} at {}", entity_id, timestamp.format("%Y-%m-%d %H:%M:%S"));
            }
            Fact::RelationshipInvalidated { source_id, target_id, timestamp } => {
                println!("🚫  [REL-INVALID] {} -> {} at {}", source_id, target_id, timestamp.format("%Y-%m-%d %H:%M:%S")); 
            }
        }
    }
    
    println!("===============================");
}