table! {
    schema_migrations (version) {
        id -> Bigint,
        version -> Varchar,
        name -> Varchar,
        created_at -> Datetime,
    }
}
