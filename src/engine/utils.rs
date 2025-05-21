use chrono::{DateTime, Local};
use crate::graph::fact::Fact;
use std::collections::HashSet;

/// Sorts a mutable list of facts chronologically in ascending order (oldest first)
/// 
/// This ensures that facts are processed or displayed in order they occurred,
/// which is useful for timelines, replaying event logs, or building coherrent case narratives.
pub fn sort_facts_by_time(facts: &mut Vec<Fact>) {
    
    // Sort based on each fact's timestamp
    facts.sort_by_key(|f| f.timestamp());
}

/// Deduplicates a vector of facts, preserving the original order of first occurrence.
/// 
/// A fact is considered duplicate if it is equal to another previously seen fact.
/// This is useful to avoid redundant or repeated data in UI or processing logic.
pub fn deduplicate_facts(facts: Vec<Fact>) -> Vec<Fact> {
    
    // Tracks facts we've already encountered
    let mut seen = HashSet::new();
    
    // Stores deduplicated output
    let mut result = Vec::new();
    
    for fact in facts {
        
        // Only add fact if it's not already seen (insert returns true if newly inserted)
        if seen.insert(fact.clone()) {
            result.push(fact);
        }
    }
    
    result
}