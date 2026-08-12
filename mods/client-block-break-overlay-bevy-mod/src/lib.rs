use bevy::{light::{NotShadowCaster, NotShadowReceiver}, prelude::*};
use bevy_mod::BevyMod;
use block_shape_api::{BlockShapeApi, BlockShapeService};
use client_bevy_default_plugins_mod::ClientBevyDefaultPluginsMod;
use client_block_damage_api::{ClientBlockDamage, ClientBlockDamageApi, ClientBlockDamageChanged, ClientBlockDamageSet};
use client_chunk_cache_api::{ClientChunkAvailable, ClientChunkCache, ClientChunkCacheApi};
use client_game_state_api::{GameState, GameStateApi};
use std::collections::HashMap;
use tokio::task::JoinHandle;
use voxel_math_api::BlockPos;

const EXPANSION: f32 = 0.004;

#[derive(Resource)]
struct BreakOverlayCube(Handle<Mesh>);
impl FromWorld for BreakOverlayCube {
    fn from_world(world: &mut World) -> Self { Self(world.resource_mut::<Assets<Mesh>>().add(Cuboid::new(1.0, 1.0, 1.0))) }
}
#[derive(Resource, Default)]
struct BreakOverlayMaterials(HashMap<u8, Handle<StandardMaterial>>);
#[derive(Resource, Default)]
struct BreakOverlays(HashMap<BlockPos, Entity>);

pub struct ClientBlockBreakOverlayBevyMod;
impl ClientBlockBreakOverlayBevyMod {
    pub fn init<D: ClientBlockDamageApi, C: ClientChunkCacheApi, S: BlockShapeApi, G: GameStateApi>(
        bevy: &mut BevyMod, _plugins: &mut ClientBevyDefaultPluginsMod, _damage: &mut D,
        _cache: &mut C, _shapes: &mut S, _game: &mut G,
    ) -> Self {
        bevy.app.init_resource::<BreakOverlayCube>().init_resource::<BreakOverlayMaterials>().init_resource::<BreakOverlays>()
            .add_systems(Update, (draw_changed, draw_available_chunks).chain().in_set(ClientBlockDamageSet::Draw))
            .add_systems(OnExit(GameState::InGame), clear_overlays);
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> { None }
}

fn draw_changed(
    mut commands: Commands, mut changes: MessageReader<ClientBlockDamageChanged>, cache: Res<ClientChunkCache>,
    shapes: Res<BlockShapeService>, cube: Res<BreakOverlayCube>, asset_server: Res<AssetServer>,
    mut materials: ResMut<BreakOverlayMaterials>, mut material_assets: ResMut<Assets<StandardMaterial>>,
    mut overlays: ResMut<BreakOverlays>,
) {
    for change in changes.read() { redraw(&mut commands, change.position, change.stage, &cache, &shapes, &cube, &asset_server, &mut materials, &mut material_assets, &mut overlays); }
}

fn draw_available_chunks(
    mut commands: Commands, mut available: MessageReader<ClientChunkAvailable>, damage: Res<ClientBlockDamage>, cache: Res<ClientChunkCache>,
    shapes: Res<BlockShapeService>, cube: Res<BreakOverlayCube>, asset_server: Res<AssetServer>,
    mut materials: ResMut<BreakOverlayMaterials>, mut material_assets: ResMut<Assets<StandardMaterial>>,
    mut overlays: ResMut<BreakOverlays>,
) {
    for chunk in available.read() {
        for (position, stage) in damage.in_chunk(chunk.position) { redraw(&mut commands, position, stage, &cache, &shapes, &cube, &asset_server, &mut materials, &mut material_assets, &mut overlays); }
    }
}

fn redraw(
    commands: &mut Commands, position: BlockPos, stage: u8, cache: &ClientChunkCache, shapes: &BlockShapeService,
    cube: &BreakOverlayCube, asset_server: &AssetServer, materials: &mut BreakOverlayMaterials,
    material_assets: &mut Assets<StandardMaterial>, overlays: &mut BreakOverlays,
) {
    if let Some(entity) = overlays.0.remove(&position) { commands.entity(entity).despawn(); }
    if stage == 0 { return; }
    let Some(block) = cache.block(position) else {
        warn!("cannot draw block damage stage {stage} at {position:?}: chunk is not cached");
        return;
    };
    let shape = shapes.shape(&block);
    if shape.is_empty() {
        warn!("cannot draw block damage stage {stage} at {position:?}: block shape is empty");
        return;
    }
    let material = materials.0.entry(stage).or_insert_with(|| material_assets.add(StandardMaterial {
        base_color: Color::WHITE,
        base_color_texture: Some(asset_server.load(format!("client-block-break-overlay-bevy-mod/block_breakage{}.png", stage.clamp(1, 10)))),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        cull_mode: None,
        depth_bias: 10.0,
        ..default()
    })).clone();
    let origin = Vec3::new(position.x as f32, position.y as f32, position.z as f32);
    let entity = commands.spawn((Transform::from_translation(origin), Visibility::Inherited, DespawnOnExit(GameState::InGame)))
        .with_children(|parent| for bounds in shape.boxes() {
            let center = (bounds.min + bounds.max) * 0.5;
            let size = (bounds.max - bounds.min) + Vec3::splat(EXPANSION * 2.0);
            parent.spawn((Mesh3d(cube.0.clone()), MeshMaterial3d(material.clone()), Transform::from_translation(center).with_scale(size), NotShadowCaster, NotShadowReceiver, Pickable::IGNORE));
        }).id();
    overlays.0.insert(position, entity);
}

fn clear_overlays(mut commands: Commands, mut overlays: ResMut<BreakOverlays>) {
    for (_, entity) in overlays.0.drain() { commands.entity(entity).despawn(); }
}
