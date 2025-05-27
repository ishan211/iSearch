/*
Name: lib.rs
Author: Ishan Leung
Language: Rust
Description: Library root that exposes crawler, cleaner, indexer, and results modules for use in CLI binaries.
*/

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;
use std::env;

pub mod crawler;
pub mod indexer;
pub mod models;
pub mod schema;

// Establishes a Diesel SQLite connection using the DATABASE_URL environment variable.
// Defaults to "sqlite://data/ishansearch.db" if not set.
pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite://data/ishansearch.db".to_string());

    // Ensure the containing directory exists
    if let Some(path) = database_url.strip_prefix("sqlite://") {
        if let Some(parent) = std::path::Path::new(path).parent() {
            std::fs::create_dir_all(parent).ok();
        }
    }

    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
