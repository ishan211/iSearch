/*
Name: main.rs
Author: Ishan Leung
Language: Rust
Description: Entry point for full iSearch pipeline – runs crawling, cleaning, and indexing + search.
*/

mod crawler;
mod cleaner;
mod indexer;

use anyhow::Result;

fn main() -> Result<()> {
    let start_url = "https://en.wikipedia.org/wiki/Computer";
    let html_dir = "pages";
    let txt_dir = "cleaned";

    // Step 1: Crawl & clean
    crawler::crawl(start_url, html_dir)?;
    cleaner::extract_all_text(html_dir, txt_dir)?;

    // Step 2: Build index + search
    indexer::build_index_and_search(txt_dir)?;

    Ok(())
}
