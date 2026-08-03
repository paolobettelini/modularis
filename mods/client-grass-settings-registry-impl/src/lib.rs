use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_grass_settings_api::{
    ClientGrassSettings, ClientGrassSettingsApi, ClientGrassSettingsChanged,
};
use client_settings_api::{SettingChanged, SettingsApi, SettingsStore};
use generated_client_settings_registry::SettingKey;
use tokio::task::JoinHandle;

pub struct ClientGrassSettingsRegistryImpl;

impl ClientGrassSettingsRegistryImpl {
    pub fn init<S: SettingsApi>(bevy: &mut BevyMod, _settings: &mut S) -> Self {
        let snapshot = read_settings(bevy.app.world().resource::<SettingsStore>());
        bevy.app
            .insert_resource(snapshot)
            .add_message::<ClientGrassSettingsChanged>()
            .add_systems(Update, update_settings);
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ClientGrassSettingsApi for ClientGrassSettingsRegistryImpl {}

fn update_settings(
    mut changes: MessageReader<SettingChanged>,
    store: Res<SettingsStore>,
    mut current: ResMut<ClientGrassSettings>,
    mut output: MessageWriter<ClientGrassSettingsChanged>,
) {
    if changes.read().count() == 0 {
        return;
    }
    let next = read_settings(&store);
    if next == *current {
        return;
    }
    let previous = *current;
    let geometry_changed = geometry_changed(previous, next);
    *current = next;
    output.write(ClientGrassSettingsChanged {
        previous,
        current: next,
        geometry_changed,
    });
}

fn read_settings(store: &SettingsStore) -> ClientGrassSettings {
    ClientGrassSettings {
        enabled: store.get_bool(SettingKey::GrassEnabled).unwrap_or(true),
        blades_per_block: store
            .get_i32(SettingKey::GrassBladesPerBlock)
            .unwrap_or(32)
            .clamp(1, 64) as u32,
        sparsity: store
            .get_f32(SettingKey::GrassSparsity)
            .unwrap_or(0.10)
            .clamp(0.0, 0.95),
        blade_height: store
            .get_f32(SettingKey::GrassBladeHeight)
            .unwrap_or(0.44)
            .clamp(0.05, 1.5),
        height_variation: store
            .get_f32(SettingKey::GrassHeightVariation)
            .unwrap_or(0.35)
            .clamp(0.0, 1.0),
        blade_width: store
            .get_f32(SettingKey::GrassBladeWidth)
            .unwrap_or(0.95)
            .clamp(0.25, 2.0),
        render_radius: store
            .get_i32(SettingKey::GrassRenderRadius)
            .unwrap_or(96)
            .clamp(16, 160) as f32,
        render_lod: store.get_bool(SettingKey::GrassRenderLod).unwrap_or(true),
        brightness: store
            .get_f32(SettingKey::GrassBrightness)
            .unwrap_or(1.0)
            .clamp(0.25, 2.0),
        hue_jitter_degrees: store
            .get_f32(SettingKey::GrassHueJitterDegrees)
            .unwrap_or(8.0)
            .clamp(0.0, 30.0),
        wind_speed: store
            .get_f32(SettingKey::GrassWindSpeed)
            .unwrap_or(1.0)
            .clamp(0.0, 5.0),
        wind_direction_degrees: store
            .get_f32(SettingKey::GrassWindDirectionDegrees)
            .unwrap_or(0.0)
            .rem_euclid(360.0),
        dynamic_wind: store.get_bool(SettingKey::GrassDynamicWind).unwrap_or(true),
        dynamic_wind_strength: store
            .get_f32(SettingKey::GrassDynamicWindStrength)
            .unwrap_or(0.8)
            .clamp(0.0, 1.0),
        deformation_strength: store
            .get_f32(SettingKey::GrassDeformationStrength)
            .unwrap_or(1.0)
            .clamp(0.0, 2.5),
    }
}

fn geometry_changed(previous: ClientGrassSettings, current: ClientGrassSettings) -> bool {
    previous.enabled != current.enabled
        || previous.blades_per_block != current.blades_per_block
        || previous.sparsity != current.sparsity
        || previous.blade_height != current.blade_height
        || previous.height_variation != current.height_variation
        || previous.blade_width != current.blade_width
        || previous.render_radius != current.render_radius
        || previous.render_lod != current.render_lod
}
