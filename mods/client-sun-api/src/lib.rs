use bevy::prelude::*;
use sun_api::SunSettings;

#[derive(Resource, Debug, Clone, Copy, Default, PartialEq)]
pub struct ClientSunSettings(pub SunSettings);

#[derive(Message, Debug, Clone, Copy, PartialEq)]
pub struct ClientSunSettingsChanged {
    pub previous: SunSettings,
    pub current: SunSettings,
}

/// Marks the directional-light entity controlled by the sun renderer.
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct ClientSunLight;

pub trait ClientSunApi: Send + Sync + 'static {}
