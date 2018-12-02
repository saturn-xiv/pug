table! {
    schema_migrations (version) {
        id -> Bigint,
        version -> Varchar,
        name -> Varchar,
        up -> Text,
        down -> Text,
        run_at -> Nullable<Datetime>,
    }
}
