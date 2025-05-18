/*
Name: search.rs
Author: Ishan Leung
Language: Rust
Description: CLI binary for launching the improved TF-IDF Wikipedia text search engine.
*/

use anyhow::Result;
use ishansearch::{indexer, results};
use std::io::{self, Write};

fn main() -> Result<()> {
    println!("Building TF-IDF index...");
    let index = indexer::build_index("cleaned")?;
    println!("Index built successfully!");

    loop {
        print!("\nEnter search term(s) (or 'exit'): ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let query = input.trim().to_lowercase();

        if query == "exit" {
            break;
        }

        if query.is_empty() {
            println!("Please enter a search term.");
            continue;
        }

        let results_vec = indexer::search(&index, &query);
        if results_vec.is_empty() {
            println!("No results for '{}'.", query);
            continue;
        }

        println!("Found {} results for '{}'", results_vec.len(), query);
        results::interactively_display_results(&results_vec, &query)?;
    }

    println!("Thank you for using IshanSearch!");
    Ok(())
}