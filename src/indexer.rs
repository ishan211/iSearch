use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use anyhow::Result;

type InvertedIndex = HashMap<String, HashMap<String, usize>>;

pub fn build_index_and_search(dir: &str) -> Result<()> {
    println!("Building inverted index from `{}`...", dir);
    let mut index: InvertedIndex = HashMap::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("txt") {
            let filename = path.file_name().unwrap().to_string_lossy().to_string();
            let content = fs::read_to_string(&path)?;
            let words = tokenize(&content);

            for word in words {
                let word_entry = index.entry(word).or_default();
                *word_entry.entry(filename.clone()).or_insert(0) += 1;
            }
        }
    }

    println!("Indexing complete. Ready to search!");

    let mut input = String::new();
    loop {
        print!("\nEnter search term (or 'exit'): ");
        io::stdout().flush()?;
        input.clear();
        io::stdin().read_line(&mut input)?;
        let query = input.trim().to_lowercase();

        if query == "exit" {
            break;
        }

        if let Some(files) = index.get(&query) {
            let mut results: Vec<_> = files.iter().collect();
            results.sort_by(|a, b| b.1.cmp(a.1));

            println!("\nResults for '{}':", query);
            for (file, count) in results.iter().take(10) {
                println!("- {} (count: {})", file, count);
            }
        } else {
            println!("No results found for '{}'.", query);
        }
    }

    Ok(())
}

// Tokenizes and normalizes input text
fn tokenize(text: &str) -> Vec<String> {
    text.split_whitespace()
        .map(|s| {
            s.trim_matches(|c: char| !c.is_alphanumeric())
                .to_lowercase()
                .trim_end_matches(['s', '\'']) // crude stem: handles plurals and possessives
                .to_string()
        })
        .filter(|s| !s.is_empty())
        .collect()
}
