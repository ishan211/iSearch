/*
Name: search.rs
Author: Ishan Leung
Language: Rust
Description: CLI binary for launching the interactive TF-IDF Wikipedia text search engine.
*/

use anyhow::Result;
use ishansearch::indexer;

fn main() -> Result<()> {
    indexer::build_index_and_search("cleaned")
}
