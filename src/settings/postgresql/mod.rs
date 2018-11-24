pub const UP: &'static str = include_str!("up.sql");
pub const DOWN: &'static str = include_str!("down.sql");

table! {
    settings (id) {
        id -> Int8,
        key -> Varchar,
        value -> Bytea,
        salt -> Nullable<Bytea>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
