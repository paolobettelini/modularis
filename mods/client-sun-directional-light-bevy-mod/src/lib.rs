use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_game_state_api::{GameState, GameStateApi};
use client_sun_api::{ClientSunApi, ClientSunLight, ClientSunSettings};
use sun_api::SunSettings;
use tokio::task::JoinHandle;

pub struct ClientSunDirectionalLightBevyMod;

impl ClientSunDirectionalLightBevyMod {
    pub fn init<S: ClientSunApi, G: GameStateApi>(
        bevy: &mut BevyMod,
        _sun: &mut S,
        _game_state: &mut G,
    ) -> Self {
        bevy.app
            .add_systems(OnEnter(GameState::InGame), spawn_sun_light)
            .add_systems(
                Update,
                apply_sun_settings.run_if(in_state(GameState::InGame)),
            );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn spawn_sun_light(mut commands: Commands, settings: Res<ClientSunSettings>) {
    commands.spawn((
        directional_light(settings.0),
        sun_transform(settings.0),
        ClientSunLight,
        DespawnOnExit(GameState::InGame),
    ));
}

fn apply_sun_settings(
    settings: Res<ClientSunSettings>,
    mut suns: Query<(&mut DirectionalLight, &mut Transform), With<ClientSunLight>>,
) {
    if !settings.is_changed() {
        return;
    }
    for (mut light, mut transform) in &mut suns {
        let shadows_enabled = light.shadows_enabled;
        *light = directional_light(settings.0);
        light.shadows_enabled = shadows_enabled;
        *transform = sun_transform(settings.0);
    }
}

fn directional_light(settings: SunSettings) -> DirectionalLight {
    DirectionalLight {
        color: Color::srgb(settings.color[0], settings.color[1], settings.color[2]),
        illuminance: settings.illuminance.max(0.0),
        shadows_enabled: false,
        ..default()
    }
}

fn sun_transform(settings: SunSettings) -> Transform {
    let position = Vec3::from_array(settings.position);
    let direction = if position.length_squared() > 0.000_001 {
        -position.normalize()
    } else {
        Vec3::new(-0.45, -0.82, -0.35).normalize()
    };
    let up = if direction.cross(Vec3::Y).length_squared() > 0.000_001 {
        Vec3::Y
    } else {
        Vec3::Z
    };
    Transform::IDENTITY.looking_to(direction, up)
}
