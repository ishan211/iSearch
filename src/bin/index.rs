/*
Name: index.rs
Author: Ishan Leung
Language: Rust
Description: Builds the TF-IDF index and saves it as a binary file under data/.
*/

use ishansearch::indexer;
use std::fs::{create_dir_all, File};
use std::io::Write;
use bincode::serialize;

fn main() -> anyhow::Result<()> {
    println!("Building TF-IDF index...");
    let tfidf = indexer::build_index("data/cleaned")?;
    println!("Index built with {} terms. Saving to data/...", tfidf.len());

    create_dir_all("data")?;
    let encoded = serialize(&tfidf)?;
    let mut file = File::create("data/ishansearch_tf-idf.index")?;
    file.write_all(&encoded)?;
    println!("Index saved to data/ishansearch_tf-idf.index");
    Ok(())
}
