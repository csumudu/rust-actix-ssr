// @generated automatically by Diesel CLI.

diesel::table! {
    rust_logos (id) {
        id -> Int4,
        name -> Varchar,
        image_path -> Varchar,
    }
}
