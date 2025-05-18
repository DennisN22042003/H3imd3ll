use petgraph::stable_graph::{NodeIndex, StableDiGraph};
use std::collections::HashMap;

use uuid::Uuid;
use crate::graph::{Entity, Relationship};

pub struct GraphDb {
    graph: StableDiGraph<Entity, Relationship>, // The actual petgraph graph, storing entities as nodes and relationships as edges.
    uuid_index_map: HashMap<Uuid, NodeIndex>    // A lookup table that maps each Entity's UUID to its corresponding node in the graph(without this we'd need to search the whole graph to find a node).
}

impl GraphDb {
    // Initializes an empty StableDiGraph and an empty HashMap, returning a new instance of GraphDB.
    pub fn new() -> Self {
        GraphDb {
            graph: StableDiGraph::new(),
            uuid_index_map: HashMap::new(),
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
            for neighbor in self.graph.neighbors_directed(node_idx, petgraph::Direction::Incoming) {
                if let Some(entity) = self.graph.node_weight(neighbor) {
                    neighbors.push(entity);
                }
            }
        }

        neighbors
    }
}

// Keeps a Stable Directed Graph of Entities and Relationships
// A HashMap to track where each Entity(by UUID) lives in the graph