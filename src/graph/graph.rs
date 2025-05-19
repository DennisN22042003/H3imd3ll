use petgraph::stable_graph::{NodeIndex, StableDiGraph};
use std::collections::HashMap;
use std::io::Write;
use std::fs::File;
use std::fs;
use petgraph::prelude::EdgeRef;
use serde_json;

use crate::graph::fact::{Fact, FactStore};
use crate::graph::{Entity, EntityType, Relationship};
use uuid::Uuid;

pub struct GraphDb {
    pub graph: StableDiGraph<Entity, Relationship>, // The actual petgraph graph, storing entities as nodes and relationships as edges.
    uuid_index_map: HashMap<Uuid, NodeIndex>, // A lookup table that maps each Entity's UUID to its corresponding node in the graph(without this we'd need to search the whole graph to find a node).
    event_log: Vec<Fact>, // Stores all facts
}

impl GraphDb {
    // Initializes an empty StableDiGraph and an empty HashMap, returning a new instance of GraphDB.
    pub fn new() -> Self {
        GraphDb {
            graph: StableDiGraph::new(),
            uuid_index_map: HashMap::new(),
            event_log: Vec::new(),
        }
    }
    
    // Checks if this UUID already exists in the graph.
    // If not adds the Entity to the graph using add_node().
    // Gets back the NodeIndex and store it in the uuid_index_map.
    // We use .clone() while adding the node because Petgraph owns its data internally, and we may want to keep using the original Entity outside the graph.
    pub fn add_entity(&mut self, entity: Entity) {
        if self.uuid_index_map.contains_key(&entity.id) {
            // Prevent duplicates - log or silently ignore
            return;
        }
        let node_index = self.graph.add_node(entity.clone());
        self.uuid_index_map.insert(entity.id, node_index);
    }

    // Looks up the source and target UUIDs in the uuid_index_map.
    // If both are found;
    //      1. Adds a directed edge from source to target.
    //      2. Associates it with the given Relationship.
    // If either isn't found, it does nothing(add logging or error returns later).
    pub fn add_relationship(&mut self, relationship: Relationship) {
        let source_idx = self.uuid_index_map.get(&relationship.source_id);
        let target_idx = self.uuid_index_map.get(&relationship.target_id);

        if let (Some(&source), Some(&target)) = (source_idx, target_idx) {
            self.graph.add_edge(source, target, relationship);
        } else {
            // Optionally log: one or both entities not found
        }
    }

    // Retrieves the actual Entity from the graph using its UUID;
    //      1. Get the NodeIndex from uuid_index_map.
    //      2. Use node_weight() to fetch the Entity stored at that node.
    pub fn get_entity(&self, uuid: &Uuid) -> Option<&Entity> {
        self.uuid_index_map
            .get(uuid)
            .and_then(|&index| self.graph.node_weight(index))
    }

    // Returns all entities directly connected outward from the given node;
    //      1. Look up the NodeIndex for the given UUID.
    //      2. Use Petgraph's neighbors() method, which gives all outgoing neighbors(default for directed graphs).
    //      3. For each neighbor node, extract its Entity and collect into a Vec.
    // Use neighbors_directed(node_idx, Incoming) instead.
    pub fn get_outgoing_neighbours(&self, uuid: &Uuid) -> Vec<&Entity> {
        let mut neighbors = Vec::new();

        if let Some(&node_idx) = self.uuid_index_map.get(uuid) {
            for neighbor in self.graph.neighbors(node_idx) {
                if let Some(entity) = self.graph.node_weight(neighbor) {
                    neighbors.push(entity);
                }
            }
        }

        neighbors
    }

    // Returns all entities connected outward from the given node;
    //      1. Look up the NodeIndex for the given UUID.
    //      2. Use Petgraph's neigbors_directed() method with Direction::Incoming to find nodes that have edges pointing in this node.
    //      3. Map each incoming neighbor's NodeIndex using node_weight() to retrieve the actual Entity stored at that node.
    //      4. Gather all found Entity references into a Vec<&Entity> and return them.
    pub fn get_incoming_neighbours(&self, uuid: &Uuid) -> Vec<&Entity> {
        let mut neighbors = Vec::new();

        if let Some(&node_idx) = self.uuid_index_map.get(uuid) {
            for neighbor in self
                .graph
                .neighbors_directed(node_idx, petgraph::Direction::Incoming)
            {
                if let Some(entity) = self.graph.node_weight(neighbor) {
                    neighbors.push(entity);
                }
            }
        }

        neighbors
    }

    pub fn add_fact(&mut self, fact_store: FactStore) {
        for entity in fact_store.entities {
            self.add_entity(entity);
        }

        for fact in fact_store.relationships.clone() {
            match &fact {
                Fact::EntityCreated {
                    entity_id,
                    timestamp: _,
                    properties,
                } => {
                    let entity = Entity {
                        id: *entity_id,
                        name: properties.get("name").cloned().unwrap_or_default(),
                        entity_type: EntityType::from_properties(properties),
                        properties: properties.clone(),
                    };
                    self.add_entity(entity);
                }
                Fact::EntityUpdated {
                    entity_id,
                    timestamp,
                    updated_properties,
                } => {
                    if let Some(&node_idx) = self.uuid_index_map.get(entity_id) {
                        if let Some(entity) = self.graph.node_weight_mut(node_idx) {
                            for (k, v) in updated_properties {
                                entity.properties.insert(k.clone(), v.clone());
                            }
                        }
                    }
                }
                Fact::EntityDeleted {
                    entity_id,
                    timestamp,
                } => {
                    if let Some(&node_idx) = self.uuid_index_map.get(entity_id) {
                        self.graph.remove_node(node_idx);
                        self.uuid_index_map.remove(entity_id);
                    }
                }
                Fact::RelationshipAdded {
                    source_id,
                    target_id,
                    relationship_type,
                    timestamp,
                    valid_from,
                    valid_to,
                } => {
                    let relationship = Relationship {
                        source_id: *source_id,
                        target_id: *target_id,
                        relationship_type: relationship_type.parse().unwrap(),
                        valid_from: *valid_from,
                        valid_to: *valid_to
                    };
                    self.add_relationship(relationship);
                }
                Fact::RelationshipInvalidated {
                    source_id,
                    target_id,
                    timestamp,
                } => {
                    if let (Some(&src), Some(&tgt)) = (
                        self.uuid_index_map.get(source_id), 
                        self.uuid_index_map.get(target_id),
                    ) {
                        let edges: Vec<_> = self.graph.edges_connecting(src, tgt).map(|e| e.id()).collect();
                        for edge in edges {
                            self.graph.remove_edge(edge);
                        }
                    }
                }
            }
            // Persist every fact
            self.event_log.push(fact);
        }
    }

    pub fn persist_facts(&self, path: &str) -> std::io::Result<()> {
        let serialized = serde_json::to_string_pretty(&self.event_log)?;
        let mut file = File::create(path)?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }

    pub fn load_from_file(path: &str) -> std::io::Result<Self> {
        let content = fs::read_to_string(path)?;
        let event_log: Vec<Fact> = serde_json::from_str(&content)?;

        let mut db = GraphDb::new();
        for fact in event_log.iter() {
            db.add_fact(FactStore {
                entities: vec![],
                relationships: vec![fact.clone()],
            });
        }

        Ok(db)
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;
    use chrono::Local;
    use super::*;
    use crate::graph::{EntityType, RelationshipType};
    use crate::graph::fact::{Fact, FactStore};
    use chrono::prelude::DateTime;

    #[test]
    fn test_graph_db_basic_flow() {
        let mut db = GraphDb::new();

        let e1 = Entity {
            id: Uuid::new_v4(),
            name: "John Doe".into(),
            entity_type: EntityType::Person,
            properties: HashMap::new(),
        };

        let e2 = Entity {
            id: Uuid::new_v4(),
            name: "Widgets Inc".into(),
            entity_type: EntityType::Company,
            properties: HashMap::new(),
        };

        let relationship = Fact::RelationshipAdded {
            source_id: e1.id,
            target_id: e2.id,
            relationship_type: RelationshipType::WorksAt.to_string(),
            timestamp: DateTime::from(Local::now()),
            valid_from: 2021,
            valid_to: None,
        };

        let store = FactStore {
            entities: vec![e1.clone(), e2.clone()],
            relationships: vec![relationship],
        };

        db.add_fact(store);

        let outgoing = db.get_outgoing_neighbours(&e1.id);
        let incoming = db.get_incoming_neighbours(&e2.id);

        assert_eq!(outgoing.len(), 1);
        assert_eq!(incoming.len(), 1);
        assert_eq!(outgoing[0].name, "Widgets Inc");
        assert_eq!(incoming[0].name, "John Doe");
    }
}
