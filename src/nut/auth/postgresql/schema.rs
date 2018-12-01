table! {
    attachments (id) {
        id -> Int8,
        user_id -> Int8,
        name -> Varchar,
        size -> Varchar,
        mime_type -> Varchar,
        url -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

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
    logs (id) {
        id -> Int8,
        user_id -> Int8,
        ip -> Varchar,
        message -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    notifications (id) {
        id -> Int8,
        user_id -> Int8,
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
    policies (id) {
        id -> Int8,
        user_id -> Int8,
        role -> Varchar,
        resource -> Nullable<Varchar>,
        nbf -> Date,
        exp -> Date,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int8,
        name -> Varchar,
        email -> Varchar,
        password -> Nullable<Bytea>,
        uid -> Varchar,
        provider_type -> Varchar,
        provider_id -> Varchar,
        logo -> Varchar,
        sign_in_count -> Int8,
        current_sign_in_at -> Nullable<Timestamp>,
        current_sign_in_ip -> Nullable<Varchar>,
        last_sign_in_at -> Nullable<Timestamp>,
        last_sign_in_ip -> Nullable<Varchar>,
        confirmed_at -> Nullable<Timestamp>,
        locked_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
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
