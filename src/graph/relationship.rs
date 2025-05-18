use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub enum RelationshipType {
    WorksAt,
    LocatedAt,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Relationship {
    pub source_id: Uuid,
    pub target_id: Uuid,
    pub relationship_type: RelationshipType,
}