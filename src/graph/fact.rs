use crate::graph::Entity;
use std::collections::HashMap;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use chrono::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Fact {
    EntityCreated {
        entity_id: Uuid,
        timestamp: DateTime<Local>,
        properties: HashMap<String, String>,
    },
    EntityUpdated {
        entity_id: Uuid,
        timestamp: DateTime<Local>,
        updated_properties: HashMap<String, String>,
    },
    EntityDeleted {
        entity_id: Uuid,
        timestamp: DateTime<Local>,
    },
    RelationshipAdded {
        source_id: Uuid,
        target_id: Uuid,
        relationship_type: String,
        timestamp: DateTime<Local>,
        valid_from: i64,
        valid_to: Option<i64>,
    },
    RelationshipInvalidated {
        source_id: Uuid,
        target_id: Uuid,
        timestamp: DateTime<Local>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FactStore {
    pub entities: Vec<Entity>,
    pub relationships: Vec<Fact>,
}
