use bevy::prelude::*;
use sun_api::SunSettings;

#[derive(Resource, Debug, Clone, Copy, Default, PartialEq)]
pub struct ServerSunState {
    pub current: Option<SunSettings>,
}

#[derive(Message, Debug, Clone, Copy, PartialEq)]
pub struct SetServerSun {
    pub settings: SunSettings,
}

#[derive(Message, Debug, Clone, Copy, PartialEq)]
pub struct ServerSunChanged {
    pub previous: Option<SunSettings>,
    pub current: SunSettings,
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ServerSunSet {
    Apply,
    Sync,
}

pub trait ServerSunApi: Send + Sync + 'static {}
