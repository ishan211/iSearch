/*
Name: web.rs
Author: Ishan Leung
Language: Rust
Description: Rocket-powered web frontend to search TF-IDF index and open URLs.
*/

#[macro_use]
extern crate rocket;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use diesel::prelude::*;
use dotenvy::dotenv;
use rocket::form::Form;
use rocket::fs::NamedFile;
use rocket_dyn_templates::{context, Template};

use bincode::deserialize;
use ishansearch::indexer::{combined_search, TfIdfIndex};
use ishansearch::models::UrlEntry;
use ishansearch::schema::urls;
use ishansearch::establish_connection;

#[derive(FromForm)]
struct SearchForm {
    query: String,
}

#[get("/")]
fn home() -> Template {
    Template::render("index", context! {})
}

#[get("/search")]
fn search_page() -> Template {
    Template::render("search", context! {})
}

#[post("/search", data = "<form>")]
fn perform_search(form: Form<SearchForm>) -> Template {
    let query = &form.query;

    // Load TF-IDF index from file
    let mut file = File::open("data/ishansearch_tf-idf.index").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let index: TfIdfIndex = deserialize(&buffer).unwrap();
    println!("Loaded TF-IDF index with {} terms", index.len());

    // Run search
    let hits = combined_search(&index, "data/cleaned", query).unwrap_or_default();
    println!("Found {} matching documents for '{}'", hits.len(), query);

    // Load entries from DB
    let conn = &mut establish_connection();
    let db_entries: Vec<UrlEntry> = urls::table.load(conn).unwrap_or_default();
    let db_map: HashMap<String, (String, Option<String>)> = db_entries
        .into_iter()
        .map(|entry| (entry.filename, (entry.url, entry.title)))
        .collect();

    // Map results to URL and title
    let results: Vec<(String, String, Option<String>)> = hits
        .into_iter()
        .map(|(fname, _score)| {
            let (url, title) = db_map
                .get(&fname)
                .cloned()
                .unwrap_or_else(|| {
                    println!("Warning: No URL found for '{}'", fname);
                    ("https://ishanleung.com".to_string(), None)
                });
            (fname, url, title)
        })
        .collect();

    Template::render("results", context! { keyword: query, results })
}

#[get("/open/<path..>")]
async fn open_file(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(PathBuf::from("data").join(path)).await.ok()
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![home, search_page, perform_search, open_file])
}
