// @generated automatically by Diesel CLI.

diesel::table! {
    email_verification_tokens (id) {
        id -> Int8,
        user_id -> Int8,
        #[max_length = 100]
        token_hash -> Varchar,
        expires_at -> Timestamptz,
        used_at -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    refresh_tokens (id) {
        id -> Int8,
        user_id -> Int8,
        #[max_length = 100]
        token_hash -> Varchar,
        is_revoked -> Bool,
        #[max_length = 100]
        device -> Varchar,
        expires_at -> Timestamptz,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    users (id) {
        id -> Int8,
        #[max_length = 90]
        email -> Varchar,
        #[max_length = 30]
        username -> Varchar,
        #[max_length = 100]
        password_hash -> Varchar,
        #[max_length = 25]
        first_name -> Varchar,
        #[max_length = 25]
        last_name -> Varchar,
        #[max_length = 10]
        role_type -> Varchar,
        is_email_verified -> Bool,
        is_active -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(email_verification_tokens -> users (user_id));
diesel::joinable!(refresh_tokens -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    email_verification_tokens,
    refresh_tokens,
    users,
);
