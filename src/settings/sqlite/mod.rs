pub const UP: &'static str = include_str!("up.sql");
pub const DOWN: &'static str = include_str!("down.sql");

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
