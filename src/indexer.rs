/*
Name: indexer.rs
Author: Ishan Leung
Language: Rust
Description: TF-IDF inverted index for full-text search across cleaned Wikipedia text files.
*/

use std::collections::{HashMap, HashSet};
use std::fs;

pub type TfIdfIndex = HashMap<String, HashMap<String, f64>>;

/// Builds the inverted TF-IDF index from all `.txt` files in the given directory.
pub fn build_index(dir: &str) -> anyhow::Result<TfIdfIndex> {
    let mut index: HashMap<String, HashMap<String, usize>> = HashMap::new();
    let mut doc_freqs: HashMap<String, usize> = HashMap::new();
    let mut docs = HashSet::new();
    
    // Track document lengths for normalization
    let mut doc_lengths: HashMap<String, usize> = HashMap::new();

    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("txt") {
            let filename = path.file_name().unwrap().to_string_lossy().to_string();
            let content = fs::read_to_string(&path)?;
            let words = tokenize(&content);

            docs.insert(filename.clone());
            
            // Track document word count for length normalization
            *doc_lengths.entry(filename.clone()).or_insert(0) += words.len();

            let mut seen = HashSet::new();
            for word in words {
                *index.entry(word.clone()).or_default().entry(filename.clone()).or_insert(0) += 1;
                if seen.insert(word.clone()) {
                    *doc_freqs.entry(word).or_insert(0) += 1;
                }
            }
        }
    }

    let total_docs = docs.len() as f64;
    let mut tfidf: TfIdfIndex = HashMap::new();

    for (term, postings) in index {
        let df = doc_freqs.get(&term).copied().unwrap_or(1) as f64;
        // Modified IDF calculation to handle common terms better
        let idf = (1.0 + (total_docs / df)).ln();

        for (doc, freq) in postings {
            let doc_length = *doc_lengths.get(&doc).unwrap_or(&1) as f64;
            // Normalized TF calculation (term frequency / document length)
            let tf = freq as f64 / doc_length;
            let score = tf * idf;
            tfidf.entry(term.clone()).or_default().insert(doc, score);
        }
    }

    Ok(tfidf)
}

pub fn tokenize(text: &str) -> Vec<String> {
    // Common English stop words to filter out
    const STOP_WORDS: [&str; 30] = [
        "the", "of", "and", "a", "to", "in", "is", "you", "that", "it", 
        "he", "was", "for", "on", "are", "as", "with", "his", "they", 
        "i", "at", "be", "this", "have", "from", "or", "one", "had", "by", "but"
    ];

    let stop_words: HashSet<&str> = STOP_WORDS.iter().cloned().collect();

    text.split_whitespace()
        .map(|s| {
            let lowercase = s.to_lowercase();
            let trimmed = lowercase.trim_matches(|c: char| !c.is_alphanumeric())
                                   .trim_end_matches(['s', '\'']);
            trimmed.to_string()
        })
        .filter(|s| !s.is_empty() && s.len() > 1 && !stop_words.contains(s.as_str()))
        .collect()
}

pub fn search(tfidf: &TfIdfIndex, query: &str) -> Vec<(String, f64)> {
    let terms = tokenize(query);
    
    if terms.is_empty() {
        return Vec::new();
    }
    
    let mut scores: HashMap<String, f64> = HashMap::new();
    let mut matched_terms: HashMap<String, HashSet<String>> = HashMap::new();
    
    // Calculate scores based on TF-IDF
    for term in &terms {
        if let Some(files) = tfidf.get(term) {
            for (file, score) in files {
                *scores.entry(file.clone()).or_insert(0.0) += score;
                matched_terms.entry(file.clone()).or_default().insert(term.clone());
            }
        }
    }
    
    // Boost documents that match more query terms
    let query_term_count = terms.len() as f64;
    for (file, matched) in matched_terms {
        if let Some(score) = scores.get_mut(&file) {
            // Boost factor based on percentage of query terms matched
            let match_ratio = matched.len() as f64 / query_term_count;
            *score *= 1.0 + match_ratio;
        }
    }

    let mut results: Vec<_> = scores.into_iter().collect();
    results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    results
}

/// Search function that supports phrase queries (exact matches)
pub fn phrase_search(dir: &str, query: &str) -> anyhow::Result<Vec<(String, f64)>> {
    let phrase_query = query.to_lowercase();
    let mut results = Vec::new();
    
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.extension().and_then(|s| s.to_str()) == Some("txt") {
            let filename = path.file_name().unwrap().to_string_lossy().to_string();
            let content = fs::read_to_string(&path)?.to_lowercase();
            
            // Count occurrences of the exact phrase
            let occurrences = content.matches(&phrase_query).count();
            
            if occurrences > 0 {
                // Score based on frequency and document length
                let score = (occurrences as f64) * 100.0 / (content.len() as f64);
                results.push((filename, score));
            }
        }
    }
    
    results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    Ok(results)
}

/// Combined search that uses both TF-IDF and phrase matching
pub fn combined_search(tfidf: &TfIdfIndex, dir: &str, query: &str) -> anyhow::Result<Vec<(String, f64)>> {
    // Get regular TF-IDF results
    let tfidf_results = search(tfidf, query);
    
    // Get phrase search results (if query has multiple words)
    let words: Vec<&str> = query.split_whitespace().collect();
    let phrase_results = if words.len() > 1 {
        phrase_search(dir, query)?
    } else {
        Vec::new()
    };
    
    // Combine both result sets
    let mut combined_scores: HashMap<String, f64> = HashMap::new();
    
    // Add TF-IDF scores (with 70% weight)
    for (doc, score) in tfidf_results {
        combined_scores.insert(doc, score * 0.7);
    }
    
    // Add phrase match scores (with 30% weight)
    for (doc, score) in phrase_results {
        *combined_scores.entry(doc).or_insert(0.0) += score * 0.3;
    }
    
    // Convert back to vector and sort
    let mut results: Vec<_> = combined_scores.into_iter().collect();
    results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    
    Ok(results)
}