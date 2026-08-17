#![forbid(unsafe_code)]

mod game_server;
mod relay;
mod web;

pub use game_server::*;
pub use relay::*;
pub use web::*;

pub const RELAY_SUBJECT: &str = "thecrown.relay";
pub const WEB_SUBJECT: &str = "thecrown.web";
pub const GAME_SERVER_SUBJECT_PREFIX: &str = "thecrown.gameserver";

pub fn game_server_subject(server_id: &str) -> String {
    format!("{GAME_SERVER_SUBJECT_PREFIX}.{server_id}")
}
