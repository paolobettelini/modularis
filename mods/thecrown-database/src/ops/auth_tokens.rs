use chrono::{Duration, Utc};
use diesel::OptionalExtension;
use diesel::prelude::*;
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::Database;
use crate::error::Result;
use crate::models::NewAuthUserTokenRow;
use crate::schema::auth_user_tokens;

fn hash_token(token: &str) -> String {
    hex::encode(Sha256::digest(token.as_bytes()))
}

impl Database {
    pub fn add_user_token(&self, player_uuid: Uuid, token: &str, ttl_seconds: i64) -> Result<()> {
        let uuid = player_uuid.hyphenated().to_string();
        let token_hash = hash_token(token);
        let now = Utc::now().naive_utc();
        let expires_at = now + Duration::seconds(ttl_seconds.max(1));
        let row = NewAuthUserTokenRow {
            player_uuid: &uuid,
            token_hash: &token_hash,
            created_at: now,
            expires_at,
        };
        let mut connection = self.connection()?;
        diesel::insert_into(auth_user_tokens::table)
            .values(&row)
            .execute(&mut connection)?;
        Ok(())
    }

    pub fn user_has_token(&self, player_uuid: Uuid, token: &str) -> Result<bool> {
        let uuid = player_uuid.hyphenated().to_string();
        let token_hash = hash_token(token);
        let now = Utc::now().naive_utc();
        let mut connection = self.connection()?;
        Ok(auth_user_tokens::table
            .filter(auth_user_tokens::player_uuid.eq(uuid))
            .filter(auth_user_tokens::token_hash.eq(token_hash))
            .filter(auth_user_tokens::expires_at.gt(now))
            .select(auth_user_tokens::token_hash)
            .first::<String>(&mut connection)
            .optional()?
            .is_some())
    }

    pub fn purge_expired_user_tokens(&self) -> Result<usize> {
        let now = Utc::now().naive_utc();
        let mut connection = self.connection()?;
        Ok(diesel::delete(auth_user_tokens::table.filter(auth_user_tokens::expires_at.le(now)))
            .execute(&mut connection)?)
    }
}
