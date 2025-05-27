/*
Name: crawler.rs
Author: Ishan Leung
Language: Rust
Description: Web crawler to download Wikipedia pages starting from a base URL.
*/

use anyhow::Result;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use reqwest::blocking::Client;
use scraper::Html;
use serde::Serialize;
use sha1::{Digest, Sha1};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::{create_dir_all, write, File},
    io::Write,
    path::Path,
    time::Duration,
};

const BASE: &str = "https://en.wikipedia.org";
const MAX_PAGES: usize = 1000;

#[derive(Serialize)]
struct ManifestEntry {
    hash: String,
    url: String,
    encoded_name: String,
}

fn hash_url(url: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.update(url.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn percent_safe_filename(url: &str) -> String {
    let path = url.strip_prefix(BASE).unwrap_or(url);
    utf8_percent_encode(path, NON_ALPHANUMERIC).to_string()
}

pub fn crawl(start_url: &str, output_dir: &str) -> Result<()> {
    create_dir_all(output_dir)?;
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .user_agent("IshanSearch/0.1")
        .build()?;

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut manifest: HashMap<String, ManifestEntry> = HashMap::new();

    queue.push_back(start_url.to_string());

    while let Some(url) = queue.pop_front() {
        if visited.len() >= MAX_PAGES {
            break;
        }

        if visited.contains(&url) {
            continue;
        }

        println!("Crawling: {}", url);
        let res = client.get(&url).send();

        let body = match res {
            Ok(resp) => resp.text().unwrap_or_default(),
            Err(_) => continue,
        };

        visited.insert(url.clone());

        let hash = hash_url(&url);
        let filename = format!("{}/{}.html", output_dir, hash);
        let tagged_body = format!("<!-- URL: {} -->\n{}", url, body);
        write(&filename, tagged_body)?;

        manifest.insert(
            hash.clone(),
            ManifestEntry {
                hash,
                url: url.clone(),
                encoded_name: percent_safe_filename(&url),
            },
        );

        let document = Html::parse_document(&body);
        let selector = scraper::Selector::parse("a").unwrap();

        for element in document.select(&selector) {
            if let Some(href) = element.value().attr("href") {
                if href.starts_with("/wiki/") && !href.contains(':') {
                    let full_url = format!("{}{}", BASE, href);
                    if !visited.contains(&full_url) {
                        queue.push_back(full_url);
                    }
                }
            }
        }
    }

    // Save manifest
    let manifest_path = Path::new(output_dir).join("manifest.json");
    let mut file = File::create(manifest_path)?;
    file.write_all(serde_json::to_string_pretty(&manifest)?.as_bytes())?;

    println!("Crawling complete. {} pages saved.", visited.len());
    Ok(())
}
