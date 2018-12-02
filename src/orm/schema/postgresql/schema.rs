table! {
    schema_migrations (version) {
        id -> Int8,
        version -> Varchar,
        name -> Varchar,
        up -> Text,
        down -> Text,
        run_at -> Nullable<Timestamp>,
    }
}
