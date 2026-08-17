CREATE TABLE players (
    uuid TEXT PRIMARY KEY NOT NULL,
    username TEXT NOT NULL,
    parkour_record INTEGER NOT NULL DEFAULT 0 CHECK (parkour_record >= 0),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX players_username_idx ON players(username);

CREATE TABLE bans (
    ban_id INTEGER PRIMARY KEY AUTOINCREMENT,
    player_uuid TEXT NOT NULL,
    reason TEXT NOT NULL,
    ban_start TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    ban_end TIMESTAMP NULL,
    FOREIGN KEY (player_uuid) REFERENCES players(uuid) ON DELETE CASCADE
);

CREATE INDEX bans_player_idx ON bans(player_uuid, ban_start);
CREATE INDEX bans_end_idx ON bans(ban_end);

CREATE TABLE auth_user_tokens (
    player_uuid TEXT NOT NULL,
    token_hash TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMP NOT NULL,
    PRIMARY KEY (player_uuid, token_hash),
    FOREIGN KEY (player_uuid) REFERENCES players(uuid) ON DELETE CASCADE
);

CREATE INDEX auth_user_tokens_expires_idx ON auth_user_tokens(expires_at);
