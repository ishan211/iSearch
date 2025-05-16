/*
Name: cleaner.rs
Author: Ishan Leung
Language: Rust
Description: Extracts readable text from saved HTML files into cleaned .txt files for indexing.
*/

use scraper::{Html, Selector};
use std::fs::{self, create_dir_all, read_to_string, write};
use std::path::Path;
use anyhow::Result;

pub fn extract_all_text(input_dir: &str, output_dir: &str) -> Result<()> {
    create_dir_all(output_dir)?;
    let selector = Selector::parse("p, h1, h2, h3").unwrap();

    for entry in fs::read_dir(input_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("html") {
            let html = read_to_string(&path)?;
            let document = Html::parse_document(&html);
            let mut text = String::new();

            for element in document.select(&selector) {
                let line = element.text().collect::<Vec<_>>().join(" ");
                text.push_str(&line);
                text.push('\n');
            }

            let filename = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown");

            let out_path = Path::new(output_dir).join(format!("{}.txt", filename));
            write(out_path, text)?;
        }
    }

    println!("Text extraction complete. Cleaned files saved in `{}`.", output_dir);
    Ok(())
}
