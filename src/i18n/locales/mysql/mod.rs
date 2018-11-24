pub const UP: &'static str = include_str!("up.sql");
pub const DOWN: &'static str = include_str!("down.sql");

table! {
    locales (id) {
        id -> Bigint,
        lang -> Varchar,
        code -> Varchar,
        message -> Text,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}
