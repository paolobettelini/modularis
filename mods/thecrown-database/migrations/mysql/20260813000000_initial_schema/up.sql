CREATE TABLE players (
    uuid CHAR(36) CHARACTER SET ascii COLLATE ascii_bin PRIMARY KEY,
    username VARCHAR(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_bin NOT NULL,
    parkour_record INT NOT NULL DEFAULT 0,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT players_parkour_record_check CHECK (parkour_record >= 0),
    INDEX players_username_idx (username)
) ENGINE=InnoDB;

CREATE TABLE bans (
    ban_id BIGINT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    player_uuid CHAR(36) CHARACTER SET ascii COLLATE ascii_bin NOT NULL,
    reason TEXT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
    ban_start DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    ban_end DATETIME NULL,
    CONSTRAINT bans_player_fk FOREIGN KEY (player_uuid)
        REFERENCES players(uuid) ON DELETE CASCADE,
    INDEX bans_player_idx (player_uuid, ban_start),
    INDEX bans_end_idx (ban_end)
) ENGINE=InnoDB;

CREATE TABLE auth_user_tokens (
    player_uuid CHAR(36) CHARACTER SET ascii COLLATE ascii_bin NOT NULL,
    token_hash CHAR(64) CHARACTER SET ascii COLLATE ascii_bin NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    expires_at DATETIME NOT NULL,
    PRIMARY KEY (player_uuid, token_hash),
    CONSTRAINT auth_user_tokens_player_fk FOREIGN KEY (player_uuid)
        REFERENCES players(uuid) ON DELETE CASCADE,
    INDEX auth_user_tokens_expires_idx (expires_at)
) ENGINE=InnoDB;
