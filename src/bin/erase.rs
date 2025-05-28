/*
Name: erase.rs
Author: Ishan Leung
Language: Rust
Description: Deletes raw/cleaned data, TF-IDF index, and resets ishansearch.db (with confirmation).
*/

use std::fs::{remove_dir_all, remove_file};
use std::io::{self, Write};
use diesel::prelude::*;
use ishansearch::establish_connection;
use ishansearch::schema::urls::dsl::*;

fn main() -> anyhow::Result<()> {
    println!("This will permanently delete:");
    println!("  - data/raw/");
    println!("  - data/cleaned/");
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

    // Proceed with deletion
    let _ = remove_dir_all("data/raw");
    let _ = remove_dir_all("data/cleaned");
    let _ = remove_file("data/ishansearch_tf-idf.index");

    let conn = &mut establish_connection();
    diesel::delete(urls).execute(conn)?;

    println!("All data erased successfully.");
    Ok(())
}
