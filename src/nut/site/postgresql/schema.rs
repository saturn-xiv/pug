table! {
    cards (id) {
        id -> Int8,
        title -> Varchar,
        body -> Text,
        media_type -> Varchar,
        action -> Varchar,
        href -> Varchar,
        logo -> Varchar,
        loc -> Varchar,
        lang -> Varchar,
        position -> Int2,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    friend_links (id) {
        id -> Int8,
        title -> Varchar,
        home -> Varchar,
        logo -> Varchar,
        lang -> Varchar,
        position -> Int2,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    leave_words (id) {
        id -> Int8,
        body -> Text,
        media_type -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    links (id) {
        id -> Int8,
        href -> Varchar,
        label -> Varchar,
        loc -> Varchar,
        lang -> Varchar,
        x -> Int2,
        y -> Int2,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    votes (id) {
        id -> Int8,
        point -> Int8,
        resource_type -> Varchar,
        resource_id -> Int8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    cards,
    friend_links,
    leave_words,
    links,
    votes,
);
