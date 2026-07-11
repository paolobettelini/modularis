use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_sun_api::{ClientSunApi, ClientSunSettings};
use tokio::task::JoinHandle;

pub struct ClientAmbientLightVanillaMod;

impl ClientAmbientLightVanillaMod {
    pub fn init<S: ClientSunApi>(bevy: &mut BevyMod, _sun: &mut S) -> Self {
        let settings = bevy.app.world().resource::<ClientSunSettings>().0;
        bevy.app
            .insert_resource(ambient_light(settings.color))
            .add_systems(Update, update_ambient_fill);
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn update_ambient_fill(sun: Res<ClientSunSettings>, mut ambient: ResMut<AmbientLight>) {
    if sun.is_changed() {
        *ambient = ambient_light(sun.0.color);
    }
}

fn ambient_light(sun_color: [f32; 3]) -> AmbientLight {
    // A mostly neutral fill keeps cast shadows readable. A small contribution
    // from the sun color also makes extreme server settings (for example a red
    // sun) visible in shadowed areas instead of being cancelled by white fill.
    const SUN_TINT: f32 = 0.35;
    let color = Vec3::ONE.lerp(
        Vec3::from_array(sun_color).clamp(Vec3::ZERO, Vec3::ONE),
        SUN_TINT,
    );
    AmbientLight {
        color: Color::srgb(color.x, color.y, color.z),
        brightness: 360.0,
        affects_lightmapped_meshes: true,
    }
}
