/*
Name: models.rs
Author: Ishan Leung
Language: Rust
Description: Database models for URL mappings in IshanSearch.
*/

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct UrlEntry {
    pub id: Option<i32>, 
    pub filename: String,
    pub url: String,
    pub title: Option<String>,
    pub clean_text: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::urls)]
pub struct NewUrlEntry {
    pub filename: String,
    pub url: String,
    pub title: Option<String>,
    pub clean_text: Option<String>,
}
