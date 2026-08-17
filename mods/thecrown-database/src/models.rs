use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

use crate::error::Result;
use crate::schema::{auth_user_tokens, bans, players};

#[derive(Debug, Clone)]
pub struct Player {
    pub uuid: Uuid,
    pub username: String,
    pub parkour_record: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone)]
pub struct Ban {
    pub ban_id: i64,
    pub player_uuid: Uuid,
    pub reason: String,
    pub ban_start: NaiveDateTime,
    pub ban_end: Option<NaiveDateTime>,
}

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = players)]
pub(crate) struct PlayerRow {
    pub uuid: String,
    pub username: String,
    pub parkour_record: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl TryFrom<PlayerRow> for Player {
    type Error = crate::DatabaseError;

    fn try_from(row: PlayerRow) -> Result<Self> {
        Ok(Self {
            uuid: Uuid::parse_str(&row.uuid)?,
            username: row.username,
            parkour_record: row.parkour_record,
            created_at: row.created_at,
            updated_at: row.updated_at,
        })
    }
}

#[derive(Debug, Insertable)]
#[diesel(table_name = players)]
pub(crate) struct NewPlayerRow<'a> {
    pub uuid: &'a str,
    pub username: &'a str,
    pub parkour_record: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = bans)]
pub(crate) struct BanRow {
    pub ban_id: i64,
    pub player_uuid: String,
    pub reason: String,
    pub ban_start: NaiveDateTime,
    pub ban_end: Option<NaiveDateTime>,
}

impl TryFrom<BanRow> for Ban {
    type Error = crate::DatabaseError;

    fn try_from(row: BanRow) -> Result<Self> {
        Ok(Self {
            ban_id: row.ban_id,
            player_uuid: Uuid::parse_str(&row.player_uuid)?,
            reason: row.reason,
            ban_start: row.ban_start,
            ban_end: row.ban_end,
        })
    }
}

#[derive(Debug, Insertable)]
#[diesel(table_name = bans)]
pub(crate) struct NewBanRow<'a> {
    pub player_uuid: &'a str,
    pub reason: &'a str,
    pub ban_start: NaiveDateTime,
    pub ban_end: Option<NaiveDateTime>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = auth_user_tokens)]
pub(crate) struct NewAuthUserTokenRow<'a> {
    pub player_uuid: &'a str,
    pub token_hash: &'a str,
    pub created_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
}
