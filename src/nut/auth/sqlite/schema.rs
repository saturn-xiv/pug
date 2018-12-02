table! {
    attachments (id) {
        id -> Integer,
        user_id -> Integer,
        title -> Text,
        size -> BigInt,
        mime_type -> Text,
        url -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    logs (id) {
        id -> Integer,
        user_id -> Integer,
        ip -> Text,
        message -> Text,
        created_at -> Timestamp,
    }
}

table! {
    notifications (id) {
        id -> Integer,
        user_id -> Integer,
        url -> Text,
        body -> Text,
        media_type -> Text,
        level -> Text,
        read -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    policies (id) {
        id -> Integer,
        user_id -> Integer,
        role -> Text,
        resource_name -> Nullable<Text>,
        resource_id -> Nullable<Integer>,
        nbf -> Date,
        exp -> Date,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Integer,
        real_name -> Text,
        nick_name -> Text,
        email -> Text,
        password -> Nullable<Binary>,
        uid -> Text,
        provider_type -> Text,
        provider_id -> Text,
        logo -> Text,
        sign_in_count -> BigInt,
        current_sign_in_at -> Nullable<Timestamp>,
        current_sign_in_ip -> Nullable<Text>,
        last_sign_in_at -> Nullable<Timestamp>,
        last_sign_in_ip -> Nullable<Text>,
        confirmed_at -> Nullable<Timestamp>,
        locked_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    attachments,
    logs,
    notifications,
    policies,
    users,
);
