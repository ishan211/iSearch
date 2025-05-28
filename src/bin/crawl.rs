/*
Name: crawl.rs
Author: Ishan Leung
Language: Rust
Description: CLI binary for crawling Wikipedia pages starting from the Computer article.
*/

use anyhow::Result;
use ishansearch::crawler;

fn main() -> Result<()> {
    crawler::crawl("https://en.wikipedia.org/wiki/Main_Page", "data/raw")
}