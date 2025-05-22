use std::io::{self, Write};
use std::str::FromStr;
use uuid::Uuid;
use chrono::prelude::*;
use std::collections::{BTreeMap, HashMap};
use crate::graph::{EntityType, RelationshipType, Entity, Relationship};
use crate::graph::fact::{Fact, FactStore};
use crate::graph::GraphDb;
use crate::engine::case::{display_case, CaseBuilder};
use crate::cli::utils;
use crate::cli::utils::{CYAN, GREEN, MAGENTA, RED, RESET, YELLOW};

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
    println!();
    println!(
        "{}{}{}",
        CYAN,
        r#"
                           *************************************************************************
                           * __  __     __   ______            ____       __    __       __        *
                           */\ \/\ \  /'__`\/\__  _\   /'\_/`\/\  _`\   /'__`\ /\ \     /\ \       *
                           *\ \ \_\ \/\_\L\ \/_/\ \/  /\      \ \ \/\ \/\_\L\ \\ \ \    \ \ \      *
                           * \ \  _  \/_/_\_<_ \ \ \  \ \ \__\ \ \ \ \ \/_/_\_<_\ \ \  __\ \ \  __ *
                           *  \ \ \ \ \/\ \L\ \ \_\ \__\ \ \_/\ \ \ \_\ \/\ \L\ \\ \ \L\ \\ \ \L\ \*
                           *   \ \_\ \_\ \____/ /\_____\\ \_\\ \_\ \____/\ \____/ \ \____/ \ \____/*
                           *    \/_/\/_/\/___/  \/_____/ \/_/ \/_/\/___/  \/___/   \/___/   \/___/ *
                           *************************************************************************
        "#,
        RESET,
    );

    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut input = String::new();

    loop {
        input.clear();
        print!("{}🔍 h3imd3ll> {} ", CYAN, RESET);
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
                    println!("{}Usage: add-entity <name> <entity_type> {}", GREEN, RESET);
                    continue;
                }
                let name = args[0];
                let entity_type_str = args[1];
                match EntityType::from_str(entity_type_str) {
                    Ok(etype) => {
                        let entity_id = Uuid::new_v4();
                        
                        // Build properties map with required keys
                        let mut properties = BTreeMap::new();
                        properties.insert("name".to_string(), name.to_string());
                        properties.insert("type".to_string(), entity_type_str.to_string());
                        
                        // Create the fact store with EntityCreated fact carrying these
                        let fact_store = FactStore {
                            facts: vec![Fact::EntityCreated {
                                entity_id,
                                timestamp: Local::now(),
                                properties,
                            }]
                        };
                        db.add_fact(fact_store);
                        println!("{}Entity '{}' added with ID {}{}", GREEN, name, entity_id, RESET);
                    }
                    Err(_) => {
                        println!("{}Invalid entity type: {}{}", RED, entity_type_str, RESET);
                    }
                }
            }
            "add-fact" => {
                if args.len() < 3 {
                    println!("{}Usage: add-fact <subject> <predicate> <object> {}", GREEN, RESET);
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
                            facts: vec![relationship_fact]
                        };
                        
                        db.add_fact(fact_store);
                        println!("{}Relationship '{}' -> '{}' added.{}", GREEN, subject, object, RESET);
                    }
                    Err(_) => {
                        println!("{}Invalid relationship type: {}{}", RED, predicate, RESET);
                    }
                }
            }
            "query" => {
                println!("{}Query feature is not implemented yet.{}", RED, RESET);
            }
            "build-case" => {
                if args.len() < 1 {
                    println!("{}Usage: build-case <case_name>{}", GREEN, RESET);
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
                    println!("{}Entity '{}' not found.{}", RED, seed_name, RESET);
                }
            }
            "save" => {
                match db.persist_facts(data_file) {
                    Ok(_) => println!("{}Graph saved to {}{}", GREEN, data_file, RESET),
                    Err(e) => println!("{}Failed to save graph: {}{}", RED, e, RESET),
                }
            }
            "load" => {
                match GraphDb::load_from_file(data_file) {
                    Ok(loaded_db) => {
                        db = loaded_db;
                        println!("{}Graph loaded from {}{}", GREEN, data_file, RESET);
                    }
                    Err(e) => println!("{}Failed to load graph: {}{}", RED, e, RESET),
                }
            }
            "help" => {
                println!("{}Available commands:{}", GREEN, RESET);
                println!("{}-------------------------------------------------------------------------------------------{}", GREEN, RESET);
                println!("  {}add-entity{}      <name> <entity_type>                - Add a new entity", GREEN, RESET);
                println!("  {}add-fact{}        <subject> <predicate> <object>      - Add a new fact", GREEN, RESET);
                //println!("  query <query>");
                println!("  {}build-case{}      <case_name> [max_depth]             - Generate a case from an entity", GREEN, RESET);
                println!("  {}save{}                                                - Save the current graph to a file", YELLOW, RESET);
                println!("  {}load{}                                                - Load graph from a file", CYAN, RESET);
                println!("  {}exit{}                                                - Exit the CLI", RED, RESET);
                println!("{}--------------------------------------------------------------------------------------------{}", GREEN, RESET);
            }
            "exit" | "quit" => {
                println!("{}Exiting...{}", RED, RESET);
                println!(
                    "{}{}{}",
                    RED,
                    r#"
                                    ****************************************************************
                                    * ____    _____   _____   ____    ____     __    __  ____      *
                                    */\  _`\ /\  __`\/\  __`\/\  _`\ /\  _`\  /\ \  /\ \/\  _`\    *
                                    *\ \ \L\_\ \ \/\ \ \ \/\ \ \ \/\ \ \ \L\ \\ `\`\\/'/\ \ \L\_\  *
                                    * \ \ \L_L\ \ \ \ \ \ \ \ \ \ \ \ \ \  _ <'`\ `\ /'  \ \  _\L  *
                                    *  \ \ \/, \ \ \_\ \ \ \_\ \ \ \_\ \ \ \L\ \ `\ \ \   \ \ \L\ \*
                                    *   \ \____/\ \_____\ \_____\ \____/\ \____/   \ \_\   \ \____/*
                                    *    \/___/  \/_____/\/_____/\/___/  \/___/     \/_/    \/___/ *
                                    **************************************************************** 
                    "#,
                    RESET,
                );
                break;
            }
            _ => {
                println!("{}Unknown command '{}'. Type 'help' for a list of commands.{}", RED, cmd, RESET);
            }
        }
    }

    Ok(())
}
