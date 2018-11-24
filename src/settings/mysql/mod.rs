pub const UP: &'static str = include_str!("up.sql");
pub const DOWN: &'static str = include_str!("down.sql");

table! {
    settings (id) {
        id -> Bigint,
        key -> Varchar,
        value -> Blob,
        salt -> Nullable<Blob>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}
