/*
Name: clean.rs
Author: Ishan Leung
Language: Rust
Description: CLI binary for cleaning HTML and inserting filename → URL into SQLite.
*/

use anyhow::Result;
use diesel::prelude::*;
use dotenvy::dotenv;
use ishansearch::{establish_connection, models::NewUrlEntry, schema::urls::dsl::*};
use scraper::{Html, Selector};
use sha1::{Digest, Sha1};
use std::fs::{self, create_dir_all, read_to_string, write};
use std::path::Path;

fn extract_title(html: &str) -> Option<String> {
    let doc = Html::parse_document(html);
    let selector = Selector::parse("title").ok()?;
    doc.select(&selector)
        .next()
        .and_then(|el| Some(el.text().collect::<Vec<_>>().join(" ").trim().to_string()))
}

fn extract_clean_text(html: &str) -> String {
    let doc = Html::parse_document(html);
    let selector = Selector::parse("p, h1, h2, h3").unwrap();
    let mut text = String::new();
    for el in doc.select(&selector) {
        text.push_str(&el.text().collect::<Vec<_>>().join(" "));
        text.push('\n');
    }
    text.trim().to_string()
}

fn main() -> Result<()> {
    dotenv().ok();

    let input_dir = Path::new("data/raw");
    let output_dir = Path::new("data/cleaned");
    create_dir_all(output_dir)?;

    let conn = &mut establish_connection();

    diesel::delete(urls).execute(conn)?;

    let mut new_entries = Vec::new();

    for entry in fs::read_dir(input_dir)? {
        let path = entry?.path();
        if path.extension().and_then(|s| s.to_str()) != Some("html") {
            continue;
        }

        let html = read_to_string(&path)?;
        let clean_text_val = extract_clean_text(&html);
        if clean_text_val.trim().is_empty() {
            println!("Skipping empty content: {}", path.display());
            continue;
        }

        let title_val = extract_title(&html);
        let url_val = html.lines().next()
            .and_then(|line| {
                if line.starts_with("<!--") && line.contains("URL:") {
                    Some(
                        line.trim_start_matches("<!-- URL:")
                            .trim_end_matches("-->")
                            .trim()
                            .to_string()
                    )
                } else {
                    None
                }
            })
            .unwrap_or_else(|| "https://ishanleung.com".to_string());

        let mut hasher = Sha1::new();
        hasher.update(&url_val);
        let hash = format!("{:x}", hasher.finalize());
        let filename_val = format!("{}.txt", hash);

        let output_path = output_dir.join(&filename_val);
        write(&output_path, &clean_text_val)?;
        println!("Saved cleaned text to {}", output_path.display());

        new_entries.push(NewUrlEntry {
            filename: filename_val,
            url: url_val,
            title: title_val,
            clean_text: Some(clean_text_val),
        });
    }

    diesel::insert_into(urls)
        .values(&new_entries)
        .execute(conn)?;

    println!("Cleaned and inserted {} entries into ishansearch.db", new_entries.len());

    Ok(())
}