/*
Name: crawler.rs
Author: Ishan Leung
Language: Rust
Description: Web crawler to download up to 100 Wikipedia pages starting from a base URL.
*/

use reqwest::blocking::Client;
use scraper::Html;
use std::collections::{HashSet, VecDeque};
use std::fs::{create_dir_all, write};
use std::time::Duration;
use anyhow::Result;

const BASE: &str = "https://en.wikipedia.org";
const MAX_PAGES: usize = 100;

pub fn crawl(start_url: &str, output_dir: &str) -> Result<()> {
    create_dir_all(output_dir)?;
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .user_agent("IshanSearch/0.1")
        .build()?;

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
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

        let name = url.replace(BASE, "").replace("/", "_");
        let filename = format!("{}/{}.html", output_dir, name.trim_start_matches('_'));
        let tagged_body = format!("<!-- URL: {} -->\n{}", url, body);
        write(&filename, tagged_body)?;

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

    println!("Crawling complete. {} pages saved.", visited.len());
    Ok(())
}
