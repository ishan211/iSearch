/*
Name: clean.rs
Author: Ishan Leung
Language: Rust
Description: CLI binary for cleaning crawled HTML Wikipedia pages and extracting readable text.
*/

use anyhow::Result;
use ishansearch::cleaner;

fn main() -> Result<()> {
    cleaner::extract_all_text("pages", "cleaned")
}
