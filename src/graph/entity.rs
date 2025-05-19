use std::str::FromStr;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::graph::RelationshipType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntityType {
    Person,
    PhoneNumber,
    Email,
    Company,
    Place,
    Action,
    Event,
}


impl ToString for EntityType{
    fn to_string(&self) -> String {
        match self {
            EntityType::Person => "Person".to_string(),
            EntityType::PhoneNumber => "PhoneNumber".to_string(),
            EntityType::Email => "Email".to_string(),
            EntityType::Company => "Company".to_string(),
            EntityType::Place => "Place".to_string(),
            EntityType::Action => "Action".to_string(),
            EntityType::Event => "Event".to_string(),
        }
    }
}


impl FromStr for EntityType{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Person" => Ok(EntityType::Person),
            "PhoneNumber" => Ok(EntityType::PhoneNumber),
            "Email" => Ok(EntityType::Email),
            "Company" => Ok(EntityType::Company),
            "Place" => Ok(EntityType::Place),
            "Action" => Ok(EntityType::Action),
            "Event" => Ok(EntityType::Event),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub id: Uuid,
    pub name: String,
    pub entity_type: EntityType,
}
