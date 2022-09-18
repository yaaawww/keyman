// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Integer,
        website -> Text,
        username -> Text,
        password -> Text,
        iv -> Text,
        create_at -> Timestamp,
    }
}
