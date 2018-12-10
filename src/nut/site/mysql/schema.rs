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
        position -> Smallint,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    categories (id) {
        id -> Bigint,
        parent_id -> Nullable<Bigint>,
        name -> Varchar,
        icon -> Varchar,
        color -> Varchar,
        font -> Varchar,
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
        lang -> Varchar,
        position -> Smallint,
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
        x -> Smallint,
        y -> Smallint,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    tags (id) {
        id -> Bigint,
        name -> Varchar,
        icon -> Varchar,
        color -> Varchar,
        font -> Varchar,
        created_at -> Datetime,
        updated_at -> Datetime,
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

allow_tables_to_appear_in_same_query!(
    cards,
    categories,
    friend_links,
    leave_words,
    links,
    tags,
    votes,
);
