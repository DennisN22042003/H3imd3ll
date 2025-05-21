use std::str::FromStr;
use std::collections::{BTreeMap, HashMap};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::graph::RelationshipType;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum EntityType {
    Person,
    PhoneNumber,
    Email,
    Company,
    Place,
    Action,
    Event,
    Unknown,
}

impl EntityType {
    pub fn from_properties(props: &BTreeMap<String, String>) -> Self {
        match props.get("type").map(String::as_str) {
            Some("Person") => EntityType::Person,
            Some("PhoneNumber") => EntityType::PhoneNumber,
            Some("Email") => EntityType::Email,
            Some("Company") => EntityType::Company,
            Some("Place") => EntityType::Place,
            Some("Action") => EntityType::Action,
            Some("Event") => EntityType::Event,
            _ => EntityType::Unknown,
        }
    }
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
            EntityType::Unknown => "Unknown".to_string(),
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
    pub properties: BTreeMap<String, String>
}
