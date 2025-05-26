// @generated automatically by Diesel CLI.

diesel::table! {
    urls (id) {
        id -> Integer,
        filename -> Text,
        url -> Text,
    }
}