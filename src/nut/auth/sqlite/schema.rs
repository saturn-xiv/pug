table! {
    attachments (id) {
        id -> Integer,
        user_id -> Integer,
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
        id -> Integer,
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
    friend_links (id) {
        id -> Integer,
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
        id -> Integer,
        body -> Text,
        media_type -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    links (id) {
        id -> Integer,
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
    logs (id) {
        id -> Integer,
        user_id -> Integer,
        ip -> Varchar,
        message -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    notifications (id) {
        id -> Integer,
        user_id -> Integer,
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
        id -> Integer,
        user_id -> Integer,
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
        id -> Integer,
        real_name -> Varchar,
        nick_name -> Varchar,
        email -> Varchar,
        password -> Nullable<Binary>,
        uid -> Varchar,
        provider_type -> Varchar,
        provider_id -> Varchar,
        logo -> Varchar,
        sign_in_count -> Integer,
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
        id -> Integer,
        point -> Integer,
        resource_type -> Varchar,
        resource_id -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
