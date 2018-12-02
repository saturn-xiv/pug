table! {
    schema_migrations (version) {
        id -> Integer,
        version -> Varchar,
        name -> Varchar,
        up -> Text,
        down -> Text,
        run_at -> Nullable<Timestamp>,
    }
}
