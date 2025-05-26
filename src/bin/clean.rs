/*
Name: clean.rs
Author: Ishan Leung
Language: Rust
Description: CLI binary for cleaning HTML and inserting filename → URL into SQLite.
*/

use anyhow::Result;
use diesel::prelude::*;
use dotenvy::dotenv;
use ishansearch::{establish_connection, models::NewUrlMapping, schema::urls};
use scraper::{Html, Selector};
use std::fs::{self, create_dir_all, read_to_string, write};
use std::path::Path;

fn main() -> Result<()> {
    dotenv().ok();

    let input_dir = Path::new("data/raw");
    let output_dir = Path::new("data/cleaned");
    create_dir_all(output_dir)?;

    let conn = &mut establish_connection();
    
    // Clear existing data 
    diesel::delete(urls::table).execute(conn)?;
    
    let selector = Selector::parse("p, h1, h2, h3").unwrap();

    let mut url_mappings: Vec<(String, String)> = Vec::new();

    for entry in fs::read_dir(input_dir)? {
        let path = entry?.path();
        if path.extension().and_then(|s| s.to_str()) != Some("html") {
            continue;
        }

        let filename = path.file_stem().unwrap().to_string_lossy().to_string();
        let html = read_to_string(&path)?;
        let doc = Html::parse_document(&html);
        let mut text = String::new();

        for element in doc.select(&selector) {
            text.push_str(&element.text().collect::<Vec<_>>().join(" "));
            text.push('\n');
        }

        let out_path = output_dir.join(format!("{filename}.txt"));
        write(&out_path, &text)?;

        // Extract the original URL from the first HTML comment
        if let Some(first_line) = html.lines().next() {
            if first_line.starts_with("<!--") && first_line.contains("URL:") {
                let url = first_line
                    .trim_start_matches("<!-- URL:")
                    .trim_end_matches("-->")
                    .trim()
                    .to_string();
                let filename_path = format!("cleaned/{}.txt", filename);
                
                // Store the owned strings
                url_mappings.push((filename_path, url));
            }
        }
    }

    // Convert to NewUrlMapping structs for database insertion
    let new_mappings: Vec<NewUrlMapping> = url_mappings
        .iter()
        .map(|(filename, url)| NewUrlMapping {
            filename: filename.as_str(),
            url: url.as_str(),
        })
        .collect();

    diesel::insert_into(urls::table)
        .values(&new_mappings)
        .execute(conn)?;

    println!("Cleaned files saved to data/cleaned/");
    println!("{} entries inserted into ishansearch.db", new_mappings.len());

    Ok(())
}