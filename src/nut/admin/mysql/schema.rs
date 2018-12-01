table! {
    attachments (id) {
        id -> Bigint,
        user_id -> Bigint,
        name -> Varchar,
        size -> Varchar,
        mime_type -> Varchar,
        url -> Varchar,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

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
    logs (id) {
        id -> Bigint,
        user_id -> Bigint,
        ip -> Varchar,
        message -> Varchar,
        created_at -> Datetime,
    }
}

table! {
    notifications (id) {
        id -> Bigint,
        user_id -> Bigint,
        url -> Varchar,
        body -> Varchar,
        media_type -> Varchar,
        level -> Varchar,
        read -> Bool,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    policies (id) {
        id -> Bigint,
        user_id -> Bigint,
        role -> Varchar,
        resource -> Nullable<Varchar>,
        nbf -> Date,
        exp -> Date,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    users (id) {
        id -> Bigint,
        name -> Varchar,
        email -> Varchar,
        password -> Nullable<Blob>,
        uid -> Varchar,
        provider_type -> Varchar,
        provider_id -> Varchar,
        logo -> Varchar,
        sign_in_count -> Bigint,
        current_sign_in_at -> Nullable<Datetime>,
        current_sign_in_ip -> Nullable<Varchar>,
        last_sign_in_at -> Nullable<Datetime>,
        last_sign_in_ip -> Nullable<Varchar>,
        confirmed_at -> Nullable<Datetime>,
        locked_at -> Nullable<Datetime>,
        deleted_at -> Nullable<Datetime>,
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
