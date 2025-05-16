/*
Name: clean.rs
Author: Ishan Leung
Language: Rust
Description: CLI binary for cleaning HTML pages and extracting plain text into the `cleaned/` directory.
*/

fn main() -> anyhow::Result<()> {
    ishansearch::cleaner::extract_all_text("pages", "cleaned")
}