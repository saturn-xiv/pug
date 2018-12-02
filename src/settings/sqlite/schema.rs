table! {
    settings (id) {
        id -> Integer,
        key -> Text,
        value -> Binary,
        salt -> Nullable<Binary>,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}
