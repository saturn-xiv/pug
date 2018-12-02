table! {
    attachments (id) {
        id -> Bigint,
        user_id -> Bigint,
        title -> Varchar,
        size -> Bigint,
        mime_type -> Varchar,
        url -> Varchar,
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
        body -> Text,
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
        real_name -> Varchar,
        nick_name -> Varchar,
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

allow_tables_to_appear_in_same_query!(
    attachments,
    logs,
    notifications,
    policies,
    users,
);
