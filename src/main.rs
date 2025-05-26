/*
Name: main.rs
Author: Ishan Leung
Language: Rust
Description: End-to-end runner that crawls Wikipedia, cleans HTML, builds TF-IDF index.
*/

use anyhow::Result;
use ishansearch::{crawler, cleaner, indexer};

fn main() -> Result<()> {
    let html_dir = "pages";
    let txt_dir = "cleaned";

    println!("Crawling Wikipedia...");
    crawler::crawl("https://en.wikipedia.org/wiki/Computer", html_dir)?;

    println!("Cleaning HTML...");
    cleaner::extract_all_text(html_dir, txt_dir)?;

    println!("Building TF-IDF index...");
    let _index = indexer::build_index(txt_dir)?;

    println!("Setup complete. You can now run the web interface with:");
    println!("cargo run --bin web");

    Ok(())
}
