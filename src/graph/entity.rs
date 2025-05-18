use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub enum EntityType {
    Person,
    PhoneNumber,
    Email,
    Company,
    Place,
    Action,
    Event,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Entity {
    pub id: Uuid,
    pub name: String,
    pub entity_type: EntityType,
}