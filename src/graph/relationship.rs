use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize)]
pub enum RelationshipType {
    WorksAt,
    LocatedAt,
}


impl ToString for RelationshipType {
    fn to_string(&self) -> String {
        match self {
            RelationshipType::WorksAt => "WorksAt".to_string(),
            RelationshipType::LocatedAt => "LocatedAt".to_string(),
        }
    }
}

impl FromStr for RelationshipType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "WorksAt" => Ok(RelationshipType::WorksAt),
            "LocatedAt" => Ok(RelationshipType::LocatedAt),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Relationship {
    pub source_id: Uuid,
    pub target_id: Uuid,
    pub relationship_type: RelationshipType,
    pub valid_from: i64,
    pub valid_to: Option<i64>,
}
