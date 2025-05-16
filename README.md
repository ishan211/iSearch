# iSearch

A modular, terminal-based Wikipedia search engine written in Rust.  
It crawls Wikipedia pages, extracts readable text, builds an inverted index with TF-IDF scoring, and allows you to search interactively.

Repository: https://github.com/ishan211/iSearch.git

---

## Features

- Wikipedia crawler (starting from a seed page)
- HTML cleaner that extracts only readable content (paragraphs and headers)
- Inverted index with TF-IDF relevance scoring
- Interactive multi-word CLI search
- Fully modular Rust project using multiple binaries and shared modules

---

## Installation

### Clone the repository

```bash
git clone https://github.com/ishan211/iSearch.git
cd iSearch
````

### Build the project

```bash
cargo build
```

---

## Running the Project

### Option 1: Run all (crawl, clean, and search)

```bash
cargo run
```

This will:

1. Crawl 100 Wikipedia pages starting from "Computer"
2. Save raw HTML to `pages/`
3. Extract clean text to `cleaned/`
4. Build a TF-IDF index and launch interactive search

---

### Option 2: Run each program/module seperately

Each step is defined as its own binary under `src/bin/`.

#### Crawl only

```bash
cargo run --bin crawl
```

Crawls Wikipedia and saves raw `.html` files to the `pages/` directory.

#### Clean only

```bash
cargo run --bin clean
```

Parses HTML files in `pages/` and extracts plain text into the `cleaned/` directory.

#### Search only

```bash
cargo run --bin search
```

Builds a TF-IDF index from files in `cleaned/` and launches an interactive terminal-based search.

---

## Example

```text
Enter search term(s) (or 'exit'): turing machine

Results for 'turing machine':
- wiki_Turing_machine.txt (score: 407.8980)
- wiki_History_of_computing_hardware.txt (score: 90.1310)
- wiki_Computer.txt (score: 58.8686)
...
```

---

## Project Structure

```
src/
├── main.rs          # Runs all (it crawls sites, cleans them, and then allows searching)
├── lib.rs           # Declare Modules
├── crawler.rs       # Wikipedia crawler logic
├── cleaner.rs       # HTML-to-text cleaner
├── indexer.rs       # TF-IDF indexing and search logic
└── bin/
    ├── crawl.rs     # Binary for crawling only
    ├── clean.rs     # Binary for cleaning only
    └── search.rs    # Binary for searching only
```

---

## How It Works

### Crawling

* Starts from: [https://en.wikipedia.org/wiki/Computer](https://en.wikipedia.org/wiki/Computer)
* Follows internal `/wiki/` links
* Downloads 100 pages total
* Saves HTML to the `pages/` folder

### Cleaning

* Parses each HTML file with `scraper`
* Extracts text from `<p>`, `<h1>`, `<h2>`, `<h3>`
* Saves `.txt` files to `cleaned/`

### Indexing + Search

* Tokenizes and normalizes words
* Builds inverted index of `word -> {file: count}`
* Applies TF-IDF formula:
  `score = tf * ln(N / df)`
* Supports multi-word queries with combined scoring
* Interactive CLI with ranked results

---

## Dependencies

* `reqwest` – for HTTP requests
* `scraper` – for parsing HTML DOM
* `anyhow` – for clean error handling

Install dependencies automatically with:

```bash
cargo build
```

---

## Author

Ishan Leung
[https://github.com/ishan211](https://github.com/ishan211)

