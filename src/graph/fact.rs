use crate::graph::Entity;
use std::collections::{BTreeMap, HashMap};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use chrono::prelude::*;

#[derive(Debug, Eq, Clone, Serialize, Deserialize, PartialEq, Hash)]
pub enum Fact {
    EntityCreated {
        entity_id: Uuid,
        timestamp: DateTime<Local>,
        properties: BTreeMap<String, String>,
    },
    EntityUpdated {
        entity_id: Uuid,
        timestamp: DateTime<Local>,
        updated_properties: BTreeMap<String, String>,
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

impl Fact {
    pub fn timestamp(&self) -> DateTime<Utc> {
        match self {
            Fact::EntityCreated { timestamp, .. }
            | Fact::EntityUpdated { timestamp, .. }
            | Fact::EntityDeleted { timestamp, .. }
            | Fact::RelationshipAdded { timestamp, .. }
            | Fact::RelationshipInvalidated { timestamp, .. } => timestamp.with_timezone(&Utc),
        }
    }
}


impl Fact {
    pub fn involves_any(&self, entity_ids: &[Uuid]) -> bool {
        match self {
            Fact::EntityCreated { entity_id, .. }
            | Fact::EntityUpdated { entity_id, .. }
            | Fact::EntityDeleted { entity_id, .. } => {
                entity_ids.contains(entity_id)
            }
            Fact::RelationshipAdded { source_id, target_id, .. }
            | Fact::RelationshipInvalidated { source_id, target_id, .. } => {
                entity_ids.contains(source_id) || entity_ids.contains(target_id)
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FactStore {
    pub entities: Vec<Entity>,
    pub relationships: Vec<Fact>,
}
