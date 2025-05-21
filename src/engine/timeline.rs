use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::graph::fact::Fact;
use crate::graph::GraphDb;

/// A query struct used to filter the timeline
/// - `entity_id`: Restrict results to the facts involving this entity
/// - `from`: Optional lower bound on the timestamp (inclusive)
/// - `to`: Optional upper bound on the timestamp (inclusive)
#[derive(Debug)]
pub struct TimelineQuery {
    pub entity_id: Option<Uuid>,             // Optional filter: a specific entity
    pub from: Option<DateTime<Utc>>,         // Optional start time
    pub to: Option<DateTime<Utc>>,           // Optional end time
}

/// Result of a timeline query
/// - Contains all relevant facts, sorted by timestamp.
#[derive(Debug)]
pub struct TimelineResult {
    pub facts: Vec<Fact>,
}

/// Extracts a filtered and time-ordered list of facts from the event log.
/// 
/// This function:
/// 1. Iterates through all facts in the event log
/// 2. Filters them based on entity ID and time window (if specified)
/// 3. Sorts the matching facts chronologically (oldest first)
/// 
/// # Arguments
/// - `db`: References to `GraphDb` that holds the event log.
/// - `query`: Filtering criteria for entity and time range.
/// 
/// # Returns
/// - A `TimelineResult` with matching facts in ascending timestamp order.
pub fn generate_timeline(db: &GraphDb, query: &TimelineQuery) -> TimelineResult {
    let mut relevant_facts = Vec::new();

    for fact in &db.event_log {
        
        // Match entity-specific facts
        let is_relevant = match fact {
            Fact::EntityCreated { entity_id, timestamp, .. }
            | Fact::EntityUpdated { entity_id, timestamp, .. }
            | Fact::EntityDeleted { entity_id, timestamp } => {
                
                // Check if entity ID matches (If provided), and timestamp falls withing the range.
                query.entity_id.map_or(true, |id| id == *entity_id) &&
                    query.from.map_or(true, |from| *timestamp >= from) &&
                    query.to.map_or(true, |to| *timestamp <= to)
            }

            // Match relationship-specific facts (added or invalidated)
            Fact::RelationshipAdded { source_id, target_id, timestamp, .. }
            | Fact::RelationshipInvalidated { source_id, target_id, timestamp } => {
                
                // Check if either end of the relationship matches the entity ID (if provided), and timestamp falls within the query range
                let involves_entity = query.entity_id.map_or(true, |id| id == *source_id || id == *target_id);
                let in_time_window = query.from.map_or(true, |from| *timestamp >= from)
                    && query.to.map_or(true, |to| *timestamp <= to);
                involves_entity && in_time_window
            }
        };

        // Collect all facts that match the filter
        if is_relevant {
            relevant_facts.push(fact.clone());
        }
    }

    // Sort the filtered facts in ascending order by timestamp.
    relevant_facts.sort_by_key(|fact| fact.timestamp());

    TimelineResult { facts: relevant_facts }
}
