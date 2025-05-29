/*
Name: erase.rs
Author: Ishan Leung
Language: Rust
Description: Deletes contents of raw/cleaned data, TF-IDF index, and resets ishansearch.db (with confirmation).
*/

use std::fs::{remove_file, read_dir, remove_dir_all};
use std::io::{self, Write};
use diesel::prelude::*;
use ishansearch::establish_connection;
use ishansearch::schema::urls::dsl::*;

fn clear_dir(path: &str) -> anyhow::Result<()> {
    for entry in read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            remove_dir_all(path)?;
        } else {
            remove_file(path)?;
        }
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    println!("This will permanently delete:");
    println!("  - contents of data/raw/");
    println!("  - contents of data/cleaned/");
    println!("  - data/ishansearch_tf-idf.index");
    println!("  - All rows in ishansearch.db");
    print!("Are you sure? (yes/no): ");
    io::stdout().flush()?;

    let mut confirmation = String::new();
    io::stdin().read_line(&mut confirmation)?;
    let confirmation = confirmation.trim().to_lowercase();

    if confirmation != "yes" {
        println!("Erase cancelled.");
        return Ok(());
    }

    clear_dir("data/raw")?;
    clear_dir("data/cleaned")?;
    let _ = remove_file("data/ishansearch_tf-idf.index");

    let conn = &mut establish_connection();
    diesel::delete(urls).execute(conn)?;

    println!("All data erased successfully.");
    Ok(())
}
