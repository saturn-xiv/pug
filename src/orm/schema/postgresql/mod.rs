table! {
    schema_migrations (version) {
        id -> Int8,
        version -> Varchar,
        name -> Varchar,
        created_at -> Timestamp,
    }
}
