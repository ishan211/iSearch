/*
Name: search.rs
Author: Ishan Leung
Language: Rust
Description: CLI TF-IDF Wikipedia search (fallback interface if web is down).
*/

use anyhow::Result;
use ishansearch::indexer;
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

        println!("Found {} results for '{}':", results_vec.len(), query);
        for (filename, score) in results_vec.iter().take(10) {
            println!("  - {} ({:.3})", filename, score);
        }
    }

    println!("Thank you for using IshanSearch CLI!");
    Ok(())
}
