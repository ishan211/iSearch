/*
Name: web.rs
Author: Ishan Leung
Description: Rocket-powered web frontend to search TF-IDF index and open URLs.
*/

#[macro_use]
extern crate rocket;

use std::collections::HashMap;
use std::path::PathBuf;

use diesel::prelude::*;
use dotenvy::dotenv;
use rocket::form::Form;
use rocket::fs::NamedFile;
use rocket_dyn_templates::{context, Template};

use ishansearch::indexer;
use ishansearch::models::UrlMapping;
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
    let index = indexer::build_index("data/cleaned").unwrap_or_default();
    let hits = indexer::search(&index, query);

    let conn = &mut establish_connection();
    let db_entries: Vec<UrlMapping> = urls::table.load(conn).unwrap_or_default();
    
    // Print database entries
    println!("Database entries:");
    for entry in &db_entries {
        println!("  ID: {}, Filename: '{}', URL: '{}'", entry.id, entry.filename, entry.url);
    }
    
    let db_map: HashMap<String, String> = db_entries
        .into_iter()
        .map(|entry| (entry.filename, entry.url))
        .collect();

    // Print search hits
    println!("Search hits:");
    for (fname, score) in &hits {
        println!("  Filename: '{}', Score: {}", fname, score);
    }

    let results: Vec<(String, String)> = hits
        .into_iter()
        .map(|(fname, _score)| {
            // Try multiple filename formats to match with database
            let possible_filenames = vec![
                fname.clone(),                      // exact match
                format!("cleaned/{}", fname),       // add cleaned/ prefix
                format!("data/cleaned/{}", fname),  // add data/cleaned/ prefix
            ];
            
            println!("Trying to match search filename: '{}'", fname);
            
            let mut found_url = None;
            for filename_variant in &possible_filenames {
                if let Some(url) = db_map.get(filename_variant) {
                    found_url = Some(url.clone());
                    println!("Found URL using variant: '{}'", filename_variant);
                    break;
                } else {
                    println!("No match for variant: '{}'", filename_variant);
                }
            }
            
            let url = found_url.unwrap_or_else(|| {
                println!("Warning: No URL found for filename '{}' (tried {} variants)", fname, possible_filenames.len());
                "https://example.com".to_string()
            });
            
            (fname, url)
        })
        .collect();

    Template::render("results", context! { keyword: query, results: results })
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