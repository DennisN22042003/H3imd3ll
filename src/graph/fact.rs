use crate::graph::Entity;
use std::collections::HashMap;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Fact {
    EntityCreated {
        entity_id: Uuid,
        timestamp: i64,
        properties: HashMap<String, String>,
    },
    EntityUpdated {
        entity_id: Uuid,
        timestamp: i64,
        updated_properties: HashMap<String, String>,
    },
    EntityDeleted {
        entity_id: Uuid,
        timestamp: i64,
    },
    RelationshipAdded {
        source_id: Uuid,
        target_id: Uuid,
        relationship_type: String,
        valid_from: i64,
        valid_to: Option<i64>,
    },
    RelationshipInvalidated {
        source_id: Uuid,
        target_id: Uuid,
        timestamp: i64,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FactStore {
    pub entities: Vec<Entity>,
    pub relationships: Vec<Fact>,
}
