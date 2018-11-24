pub const UP: &'static str = include_str!("up.sql");
pub const DOWN: &'static str = include_str!("down.sql");

table! {
    locales (id) {
        id -> Integer,
        lang -> Varchar,
        code -> Varchar,
        message -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
