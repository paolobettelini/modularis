// Shared Diesel schema: intentionally limited to SQL types common to SQLite and MySQL.

diesel::table! {
    players (uuid) {
        uuid -> Text,
        username -> Text,
        parkour_record -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    bans (ban_id) {
        ban_id -> BigInt,
        player_uuid -> Text,
        reason -> Text,
        ban_start -> Timestamp,
        ban_end -> Nullable<Timestamp>,
    }
}

diesel::table! {
    auth_user_tokens (player_uuid, token_hash) {
        player_uuid -> Text,
        token_hash -> Text,
        created_at -> Timestamp,
        expires_at -> Timestamp,
    }
}

diesel::joinable!(bans -> players (player_uuid));
diesel::joinable!(auth_user_tokens -> players (player_uuid));

diesel::allow_tables_to_appear_in_same_query!(players, bans, auth_user_tokens,);
