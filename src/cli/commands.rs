use std::io::{self, Write};
use std::str::FromStr;
use uuid::Uuid;
use chrono::prelude::*;
use std::collections::{BTreeMap, HashMap};

use crate::graph::{EntityType, RelationshipType, Entity, Relationship};
use crate::graph::fact::{Fact, FactStore};
use crate::graph::GraphDb;
use crate::engine::case::{display_case, CaseBuilder};

fn find_entity_by_name<'a>(db: &'a GraphDb, name: &str) -> Option<&'a Entity> {
    db.graph.node_weights().find(|e| e.name == name)
}

pub fn run_h3imd3ll_repl() -> io::Result<()> {
    let mut db = GraphDb::new();
    let data_file = "graph_data.json";

    // Load existing data if any
    if std::path::Path::new(data_file).exists() {
        match GraphDb::load_from_file(data_file) {
            Ok(loaded_db) => {
                db = loaded_db;
                println!("Loaded graph from {}", data_file);
            }
            Err(e) => println!("Failed to load graph from file: {}", e),
        }
    }

    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut input = String::new();

    loop {
        input.clear();
        print!("h3imd3ll> ");
        stdout.flush()?;  // Make sure prompt is printed

        if stdin.read_line(&mut input)? == 0 {
            // EOF (Ctrl+D)
            println!("\nExiting...");
            break;
        }

        let trimmed = input.trim();
        if trimmed.is_empty() {
            continue; // ignore empty lines
        }

        // Split input into command and args
        let mut parts = trimmed.split_whitespace();
        let cmd = parts.next().unwrap();
        let args: Vec<&str> = parts.collect();

        match cmd.to_lowercase().as_str() {
            "add-entity" => {
                if args.len() < 2 {
                    println!("Usage: add-entity <name> <entity_type>");
                    continue;
                }
                let name = args[0];
                let entity_type_str = args[1];
                match EntityType::from_str(entity_type_str) {
                    Ok(etype) => {
                        let entity = Entity {
                            id: Uuid::new_v4(),
                            name: name.to_string(),
                            entity_type: etype,
                            properties: BTreeMap::new()
                        };
                        let fact_store = FactStore {
                            entities: vec![entity.clone()],
                            relationships: vec![],
                        };
                        db.add_fact(fact_store);
                        println!("Entity '{}' added with ID {}", entity.name, entity.id);
                    }
                    Err(_) => {
                        println!("Invalid entity type: {}", entity_type_str);
                    }
                }
            }
            "add-fact" => {
                if args.len() < 3 {
                    println!("Usage: add-fact <subject> <predicate> <object>");
                    continue;
                }
                let subject = args[0];
                let predicate = args[1];
                let object = args[2];

                let subject_entity = find_entity_by_name(&db, subject);
                let object_entity = find_entity_by_name(&db, object);

                if subject_entity.is_none() || object_entity.is_none() {
                    println!("Subject or object entity not found.");
                    continue;
                }
                let subject_entity = subject_entity.unwrap();
                let object_entity = object_entity.unwrap();
                
                let local_time: DateTime<Local> = Local::now();

                match RelationshipType::from_str(predicate) {
                    Ok(rel_type) => {
                        let relationship_fact = Fact::RelationshipAdded {
                            source_id: subject_entity.id,
                            target_id: object_entity.id,
                            relationship_type: rel_type.to_string(),
                            timestamp: local_time,
                            valid_from: 2025, // Or current year / configurable
                            valid_to: None,
                        };
                        let fact_store = FactStore {
                            entities: vec![],
                            relationships: vec![relationship_fact],
                        };
                        
                        db.add_fact(fact_store);
                        println!("Relationship '{}' -> '{}' added.", subject, object);
                    }
                    Err(_) => {
                        println!("Invalid relationship type: {}", predicate);
                    }
                }
            }
            "query" => {
                println!("Query feature is not implemented yet.");
            }
            "build-case" => {
                if args.len() < 1 {
                    println!("Usage: build-case <case_name>");
                    continue;
                }
                
                let seed_name = args[0];
                let depth = if args.len() > 1 {
                    args[1].parse::<usize>().unwrap_or(2)
                } else {
                    2
                };
                
                if let Some(seed_entity) = find_entity_by_name(&db, seed_name) {
                    let builder = CaseBuilder::new(&db, seed_entity.id)
                        .with_max_depth(depth);
                    
                    let case = builder.build(
                        &format!("Case around '{}'", seed_name),
                        "Auto-generated case from CLI",
                    );
                    
                    display_case(&case, &db);
                    
                } else {
                    println!("Entity '{}' not found.", seed_name);
                }
            }
            "save" => {
                match db.persist_facts(data_file) {
                    Ok(_) => println!("Graph saved to {}", data_file),
                    Err(e) => println!("Failed to save graph: {}", e),
                }
            }
            "load" => {
                match GraphDb::load_from_file(data_file) {
                    Ok(loaded_db) => {
                        db = loaded_db;
                        println!("Graph loaded from {}", data_file);
                    }
                    Err(e) => println!("Failed to load graph: {}", e),
                }
            }
            "help" => {
                println!("Available commands:");
                println!("  add-entity <name> <entity_type>");
                println!("  add-fact <subject> <predicate> <object>");
                println!("  query <query>");
                println!("  build-case <case_name> [max_depth]");
                println!("  save");
                println!("  load");
                println!("  exit");
            }
            "exit" | "quit" => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Unknown command '{}'. Type 'help' for a list of commands.", cmd);
            }
        }
    }

    Ok(())
}
