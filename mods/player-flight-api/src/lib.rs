use bevy::prelude::*;

#[derive(Resource, Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct LocalPlayerFlight {
    pub capability_enabled: bool,
    pub flying: bool,
}

#[derive(Resource, Debug, Clone, Copy)]
pub struct FlightConfig {
    pub double_tap_seconds: f64,
}

impl Default for FlightConfig {
    fn default() -> Self {
        Self {
            double_tap_seconds: 0.3,
        }
    }
}

#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub struct LocalFlightCapabilityChanged {
    pub enabled: bool,
}

pub trait PlayerFlightApi: Send + Sync + 'static {}
