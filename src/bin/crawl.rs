/*
Name: crawl.rs
Author: Ishan Leung
Language: Rust
Description: CLI binary for crawling Wikipedia pages starting from a root URL. Saves raw HTML to `pages/`.
*/

fn main() -> anyhow::Result<()> {
    ishansearch::crawler::crawl("https://en.wikipedia.org/wiki/Computer", "pages")
}
