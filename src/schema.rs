// @generated automatically by Diesel CLI.

diesel::table! {
    heroes (id) {
        id -> Integer,
        name -> Text,
        email -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
