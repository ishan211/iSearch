/*
Name: search.rs
Author: Ishan Leung
Language: Rust
Description: CLI binary for interactively searching Wikipedia articles using a TF-IDF-ranked inverted index.
*/

fn main() -> anyhow::Result<()> {
    ishansearch::indexer::build_index_and_search("cleaned")
}