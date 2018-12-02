table! {
    cards (id) {
        id -> Bigint,
        title -> Varchar,
        body -> Text,
        media_type -> Varchar,
        action -> Varchar,
        href -> Varchar,
        logo -> Varchar,
        loc -> Varchar,
        lang -> Varchar,
        position -> Tinyint,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    links (id) {
        id -> Bigint,
        href -> Varchar,
        label -> Varchar,
        loc -> Varchar,
        lang -> Varchar,
        x -> Tinyint,
        y -> Tinyint,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    friend_links (id) {
        id -> Bigint,
        title -> Varchar,
        home -> Varchar,
        logo -> Varchar,
        position -> Tinyint,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    leave_words (id) {
        id -> Bigint,
        body -> Text,
        media_type -> Varchar,
        created_at -> Datetime,
    }
}

table! {
    votes (id) {
        id -> Bigint,
        point -> Bigint,
        resource_type -> Varchar,
        resource_id -> Bigint,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}
