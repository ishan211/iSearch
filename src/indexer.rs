/*
Name: indexer.rs
Author: Ishan Leung
Language: Rust
Description: Builds a TF-IDF inverted index for full-text search across cleaned Wikipedia text files.
*/

use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{self, Write};
use anyhow::Result;

type TfIdfIndex = HashMap<String, HashMap<String, f64>>;

pub fn build_index_and_search(dir: &str) -> Result<()> {
    println!("Building inverted index from `{}`...", dir);
    let mut index: HashMap<String, HashMap<String, usize>> = HashMap::new();
    let mut doc_lengths: HashMap<String, usize> = HashMap::new();
    let mut doc_freqs: HashMap<String, usize> = HashMap::new();
    let mut docs = HashSet::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("txt") {
            let filename = path.file_name().unwrap().to_string_lossy().to_string();
            let content = fs::read_to_string(&path)?;
            let words = tokenize(&content);

            docs.insert(filename.clone());
            doc_lengths.insert(filename.clone(), words.len());

            let mut seen = HashSet::new();
            for word in words {
                *index.entry(word.clone()).or_default().entry(filename.clone()).or_insert(0) += 1;
                if !seen.contains(&word) {
                    *doc_freqs.entry(word.clone()).or_insert(0) += 1;
                    seen.insert(word);
                }
            }
        }
    }

    let total_docs = docs.len() as f64;
    let mut tfidf: TfIdfIndex = HashMap::new();

    for (term, postings) in index {
        let df = doc_freqs.get(&term).copied().unwrap_or(1) as f64;
        let idf = (total_docs / df).ln();

        for (doc, freq) in postings {
            let tf = freq as f64;
            let score = tf * idf;
            tfidf.entry(term.clone()).or_default().insert(doc, score);
        }
    }

    println!("TF-IDF index built. Ready to search across {} documents.", total_docs as usize);

    let mut input = String::new();
    loop {
        print!("\nEnter search term(s) (or 'exit'): ");
        io::stdout().flush()?;
        input.clear();
        io::stdin().read_line(&mut input)?;
        let query = input.trim().to_lowercase();

        if query == "exit" {
            break;
        }

        let terms = tokenize(&query);
        let mut scores: HashMap<String, f64> = HashMap::new();

        for term in &terms {
            if let Some(files) = tfidf.get(term) {
                for (file, score) in files {
                    *scores.entry(file.clone()).or_insert(0.0) += score;
                }
            }
        }

        let mut results: Vec<_> = scores.into_iter().collect();
        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        println!("\nResults for '{}':", query);
        for (file, score) in results.iter().take(10) {
            println!("- {} (score: {:.4})", file, score);
        }
    }

    Ok(())
}

fn tokenize(text: &str) -> Vec<String> {
    text.split_whitespace()
        .map(|s| {
            s.trim_matches(|c: char| !c.is_alphanumeric())
                .to_lowercase()
                .trim_end_matches(['s', '\''])
                .to_string()
        })
        .filter(|s| !s.is_empty())
        .collect()
}
