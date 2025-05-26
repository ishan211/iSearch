/*
Name: models.rs
Author: Ishan Leung
Language: Rust
Description: Database models for URL mappings in IshanSearch.
*/

use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::urls;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct UrlMapping {
    pub id: i32,
    pub filename: String,
    pub url: String,
}

#[derive(Insertable)]
#[diesel(table_name = urls)]
pub struct NewUrlMapping<'a> {
    pub filename: &'a str,
    pub url: &'a str,
}