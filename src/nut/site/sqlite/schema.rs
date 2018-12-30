table! {
    cards (id) {
        id -> Integer,
        title -> Text,
        body -> Text,
        media_type -> Text,
        action -> Text,
        href -> Text,
        logo -> Text,
        loc -> Text,
        lang -> Text,
        position -> SmallInt,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    categories (id) {
        id -> Integer,
        parent_id -> Nullable<Integer>,
        name -> Text,
        icon -> Text,
        color -> Text,
        font -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    friend_links (id) {
        id -> Integer,
        title -> Text,
        home -> Text,
        logo -> Text,
        lang -> Text,
        position -> SmallInt,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    leave_words (id) {
        id -> Integer,
        body -> Text,
        media_type -> Text,
        created_at -> Timestamp,
    }
}

table! {
    links (id) {
        id -> Integer,
        href -> Text,
        label -> Text,
        loc -> Text,
        lang -> Text,
        x -> SmallInt,
        y -> SmallInt,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    tags (id) {
        id -> Integer,
        name -> Text,
        icon -> Text,
        color -> Text,
        font -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    votes (id) {
        id -> Integer,
        point -> BigInt,
        resource_type -> Text,
        resource_id -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    cards,
    categories,
    friend_links,
    leave_words,
    links,
    tags,
    votes,
);
