use chrono::{NaiveDateTime, Utc};
use diesel::OptionalExtension;
use diesel::prelude::*;
use uuid::Uuid;

use crate::Database;
use crate::error::Result;
use crate::models::{Ban, BanRow, NewBanRow};
use crate::schema::bans;

impl Database {
    pub fn add_ban(
        &self,
        player_uuid: Uuid,
        reason: &str,
        ban_end: Option<NaiveDateTime>,
    ) -> Result<()> {
        let uuid = player_uuid.hyphenated().to_string();
        let row = NewBanRow {
            player_uuid: &uuid,
            reason,
            ban_start: Utc::now().naive_utc(),
            ban_end,
        };
        let mut connection = self.connection()?;
        diesel::insert_into(bans::table)
            .values(&row)
            .execute(&mut connection)?;
        Ok(())
    }

    pub fn get_active_ban(&self, player_uuid: Uuid) -> Result<Option<Ban>> {
        let uuid = player_uuid.hyphenated().to_string();
        let now = Utc::now().naive_utc();
        let mut connection = self.connection()?;
        bans::table
            .filter(bans::player_uuid.eq(uuid))
            .filter(bans::ban_end.is_null().or(bans::ban_end.gt(now)))
            .order(bans::ban_start.desc())
            .select(BanRow::as_select())
            .first(&mut connection)
            .optional()?
            .map(TryInto::try_into)
            .transpose()
    }
}
