/*
Name: search.rs
Author: Ishan Leung
Language: Rust
Description: CLI TF-IDF Wikipedia search using prebuilt index.
*/

use anyhow::Result;
use ishansearch::indexer::{self, TfIdfIndex};
use std::fs::File;
use std::io::{self, Read, Write};
use bincode::deserialize;

fn main() -> Result<()> {
    println!("Loading prebuilt TF-IDF index...");
    let mut file = File::open("data/ishansearch_tf-idf.index")?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let index: TfIdfIndex = deserialize(&buffer)?;
    println!("Index loaded successfully!");

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
