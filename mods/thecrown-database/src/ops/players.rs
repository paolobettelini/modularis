use chrono::Utc;
use diesel::OptionalExtension;
use diesel::prelude::*;
use uuid::Uuid;

use crate::Database;
use crate::error::Result;
use crate::models::{NewPlayerRow, Player, PlayerRow};
use crate::schema::players;

impl Database {
    pub fn upsert_player(&self, player_uuid: Uuid, username: &str) -> Result<Player> {
        let uuid = player_uuid.hyphenated().to_string();
        let now = Utc::now().naive_utc();
        let mut connection = self.connection()?;

        let existing = players::table
            .find(&uuid)
            .select(PlayerRow::as_select())
            .first(&mut connection)
            .optional()?;

        if existing.is_some() {
            diesel::update(players::table.find(&uuid))
                .set((players::username.eq(username), players::updated_at.eq(now)))
                .execute(&mut connection)?;
        } else {
            let row = NewPlayerRow {
                uuid: &uuid,
                username,
                parkour_record: 0,
                created_at: now,
                updated_at: now,
            };
            diesel::insert_into(players::table)
                .values(&row)
                .execute(&mut connection)?;
        }

        players::table
            .find(&uuid)
            .select(PlayerRow::as_select())
            .first(&mut connection)?
            .try_into()
    }

    pub fn get_player(&self, player_uuid: Uuid) -> Result<Option<Player>> {
        let uuid = player_uuid.hyphenated().to_string();
        let mut connection = self.connection()?;
        players::table
            .find(uuid)
            .select(PlayerRow::as_select())
            .first(&mut connection)
            .optional()?
            .map(TryInto::try_into)
            .transpose()
    }

    pub fn get_player_by_username(&self, requested_username: &str) -> Result<Option<Player>> {
        let mut connection = self.connection()?;
        players::table
            .filter(players::username.eq(requested_username))
            .order(players::updated_at.desc())
            .select(PlayerRow::as_select())
            .first(&mut connection)
            .optional()?
            .map(TryInto::try_into)
            .transpose()
    }

    pub fn set_parkour_record(&self, player_uuid: Uuid, record: i32) -> Result<bool> {
        let uuid = player_uuid.hyphenated().to_string();
        let now = Utc::now().naive_utc();
        let mut connection = self.connection()?;
        let changed = diesel::update(players::table.find(uuid))
            .set((
                players::parkour_record.eq(record.max(0)),
                players::updated_at.eq(now),
            ))
            .execute(&mut connection)?;
        Ok(changed == 1)
    }

    /// Atomically keeps the highest submitted parkour score.
    pub fn update_parkour_record_if_higher(
        &self,
        player_uuid: Uuid,
        score: i32,
    ) -> Result<Option<(i32, i32, bool)>> {
        let uuid = player_uuid.hyphenated().to_string();
        let score = score.max(0);
        let now = Utc::now().naive_utc();
        let mut connection = self.connection()?;
        connection.transaction(|connection| {
            let Some(previous) = players::table
                .find(&uuid)
                .select(players::parkour_record)
                .first::<i32>(connection)
                .optional()?
            else {
                return Ok(None);
            };
            let current = previous.max(score);
            let improved = current > previous;
            if improved {
                diesel::update(players::table.find(&uuid))
                    .set((
                        players::parkour_record.eq(current),
                        players::updated_at.eq(now),
                    ))
                    .execute(connection)?;
            }
            Ok(Some((previous, current, improved)))
        })
    }
}
