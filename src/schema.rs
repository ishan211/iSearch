// @generated automatically by Diesel CLI.

diesel::table! {
    urls (id) {
        id -> Nullable<Integer>,
        filename -> Text,
        url -> Text,
        title -> Nullable<Text>,
        clean_text -> Nullable<Text>,
    }
}
