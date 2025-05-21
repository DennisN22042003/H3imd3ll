use crate::graph::{GraphDb, Entity, EntityType};

/// Define the structure for a search query
/// - `entity_type`: Optional filter to match entities of a specific type
/// - `name_contains`: Optional substring to search for in entity names
pub struct SearchQuery {
    pub entity_type: Option<EntityType>,
    pub name_contains: Option<String>,
}

/// Search for entities in the graph that match the given query.
/// Filters based on optional entity type and/or name substring.
/// 
/// # Arguments
/// - `db`: Reference to the graph database
/// - `query`: SearchQuery containing filters
/// 
/// # Returns
/// - A list of references to entities that match all provided filters
pub fn search_entities(db: &GraphDb, query: SearchQuery) -> Vec<&Entity> {
    db.graph
        // Iterate over all node indices (each node represents an Entity)
        .node_indices()
        
        // Fetch the Entity stored at each node (if any)
        .filter_map(|idx| db.graph.node_weight(idx))
        
        // Apply the search filters
        .filter(|entity| {
            let mut matches = true;
            
            // If a specific entity type is requested, check if it matches
            if let Some(ref etype) = query.entity_type {
                matches &= &entity.entity_type == etype;
            }
            
            // If a name filter is provided, check if the entity's name contains the substring
            if let Some(ref name_substr) = query.name_contains {
                matches &= entity.name.contains(name_substr);
            }
            // Entity passes all filter conditions
            matches
        })
        // Collect all matching entities into a Vec
        .collect()
}