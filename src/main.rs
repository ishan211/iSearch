/*
Name: main.rs
Author: Ishan Leung
Language: Rust
Description: End-to-end runner that crawls Wikipedia, cleans HTML, builds TF-IDF index, and starts search UI.
*/

use anyhow::Result;
use ishansearch::{crawler, cleaner, indexer, results};

fn main() -> Result<()> {
    let html_dir = "pages";
    let txt_dir = "cleaned";

    println!("Crawling Wikipedia...");
    crawler::crawl("https://en.wikipedia.org/wiki/Computer", html_dir)?;

    println!("Cleaning HTML...");
    cleaner::extract_all_text(html_dir, txt_dir)?;

    println!("Building TF-IDF index...");
    let index = indexer::build_index(txt_dir)?;

    println!("Entering search UI...");
    loop {
        print!("\nEnter search term(s) (or 'exit'): ");
        std::io::Write::flush(&mut std::io::stdout())?;
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let query = input.trim().to_lowercase();

        if query == "exit" {
            break;
        }

        let results_vec = indexer::search(&index, &query);
        if results_vec.is_empty() {
            println!("No results for '{}'.", query);
            continue;
        }

        results::interactively_display_results(&results_vec, &query)?;
    }

    Ok(())
}
