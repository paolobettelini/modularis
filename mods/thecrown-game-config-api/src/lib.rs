use bevy::prelude::*;

#[derive(Resource, Debug, Clone)]
pub struct TheCrownGameConfig {
    pub server_id: String,
    pub public_address: String,
    pub public_port: u16,
    pub nats_url: String,
    pub request_timeout_ms: u64,
    pub web_address: String,
}

pub trait TheCrownGameConfigApi: Send + Sync + 'static {}

