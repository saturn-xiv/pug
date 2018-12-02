table! {
    cards (id) {
        id -> BigInt,
        title -> Varchar,
        body -> Text,
        media_type -> Varchar,
        action -> Varchar,
        href -> Varchar,
        logo -> Varchar,
        loc -> Varchar,
        lang -> Varchar,
        position -> Tinyint,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    links (id) {
        id -> BigInt,
        href -> Varchar,
        label -> Varchar,
        loc -> Varchar,
        lang -> Varchar,
        x -> Tinyint,
        y -> Tinyint,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    friend_links (id) {
        id -> BigInt,
        title -> Varchar,
        home -> Varchar,
        logo -> Varchar,
        position -> Tinyint,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    leave_words (id) {
        id -> BigInt,
        body -> Text,
        media_type -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    notifications (id) {
        id -> BigInt,
        user_id -> BigInt,
        url -> Varchar,
        body -> Varchar,
        media_type -> Varchar,
        level -> Varchar,
        read -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    votes (id) {
        id -> BigInt,
        point -> BigInt,
        resource_type -> Varchar,
        resource_id -> BigInt,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
