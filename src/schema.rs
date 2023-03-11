// @generated automatically by Diesel CLI.

diesel::table! {
    heroes (id) {
        id -> Nullable<Integer>,
        name -> Text,
        email -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
