diesel::table! {
    users (id) {
        id -> Integer,
        name -> Text,
        password -> Text,
        create_at -> Timestamp,
    }
}
