use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_grass_settings_api::{ClientGrassSettings, ClientGrassSettingsApi};
use client_wind_api::{ClientWind, ClientWindApi, ClientWindChanged};
use tokio::task::JoinHandle;

pub struct ClientWindGrassSettingsVanillaMod;

impl ClientWindGrassSettingsVanillaMod {
    pub fn init<S: ClientGrassSettingsApi>(bevy: &mut BevyMod, _settings: &mut S) -> Self {
        bevy.app
            .init_resource::<ClientWind>()
            .add_message::<ClientWindChanged>()
            .add_systems(Update, update_wind);
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ClientWindApi for ClientWindGrassSettingsVanillaMod {}

fn update_wind(
    time: Res<Time>,
    settings: Res<ClientGrassSettings>,
    mut wind: ResMut<ClientWind>,
    mut changed: MessageWriter<ClientWindChanged>,
) {
    let seconds = time.elapsed_secs();
    let direction_drift = if settings.dynamic_wind {
        let broad = (seconds * 0.07).sin() * 0.65;
        let detail = (seconds * 0.13 + 1.6).sin() * 0.35;
        (broad + detail) * 28.0 * settings.dynamic_wind_strength
    } else {
        0.0
    };
    let radians = (settings.wind_direction_degrees + direction_drift).to_radians();
    let direction = Vec2::new(radians.cos(), radians.sin()).normalize_or_zero();

    let dynamic_multiplier = if settings.dynamic_wind {
        // Several slow, non-harmonic waves avoid an obvious single pulse.
        // Their small combined range keeps the wind around its configured
        // strength instead of alternating between calm and stormy extremes.
        let variation = (seconds * 0.071).sin() * 0.52
            + (seconds * 0.113 + 2.4).sin() * 0.31
            + (seconds * 0.037 + 4.7).sin() * 0.17;
        1.0 + variation * 0.16 * settings.dynamic_wind_strength
    } else {
        1.0
    };
    let next = ClientWind {
        direction: if direction == Vec2::ZERO {
            Vec2::X
        } else {
            direction
        },
        intensity: settings.wind_speed * dynamic_multiplier,
    };

    if (next.direction - wind.direction).length_squared() < 1.0e-8
        && (next.intensity - wind.intensity).abs() < 1.0e-4
    {
        return;
    }
    let previous = *wind;
    *wind = next;
    changed.write(ClientWindChanged {
        previous,
        current: next,
    });
}
