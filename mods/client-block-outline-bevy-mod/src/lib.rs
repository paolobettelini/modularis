use bevy::{
    light::{NotShadowCaster, NotShadowReceiver},
    prelude::*,
};
use bevy_mod::BevyMod;
use block_shape_api::BlockShape;
use client_bevy_default_plugins_mod::ClientBevyDefaultPluginsMod;
use client_block_outline_api::{
    BlockOutlineStyle, ClientBlockOutlineApi, ClientBlockOutlineSet, SetClientBlockOutline,
};
use client_game_state_api::{GameState, GameStateApi};
use std::collections::HashMap;
use tokio::task::JoinHandle;
use voxel_math_api::BlockPos;

const OUTLINE_EDGE_THICKNESS: f32 = 0.002;

#[derive(Resource)]
struct BlockOutlineMesh(Handle<Mesh>);

impl FromWorld for BlockOutlineMesh {
    fn from_world(world: &mut World) -> Self {
        let mesh = world
            .resource_mut::<Assets<Mesh>>()
            .add(Cuboid::new(1.0, 1.0, 1.0));
        Self(mesh)
    }
}

#[derive(Debug, Clone)]
struct ActiveBlockOutline {
    entity: Entity,
    material: Handle<StandardMaterial>,
}

#[derive(Resource, Default)]
struct ActiveBlockOutlines(HashMap<String, ActiveBlockOutline>);

pub struct ClientBlockOutlineBevyMod;

impl ClientBlockOutlineBevyMod {
    pub fn init<G: GameStateApi>(
        bevy: &mut BevyMod,
        _plugins: &mut ClientBevyDefaultPluginsMod,
        _game_state: &mut G,
    ) -> Self {
        bevy.app
            .init_resource::<BlockOutlineMesh>()
            .init_resource::<ActiveBlockOutlines>()
            .add_message::<SetClientBlockOutline>()
            .configure_sets(
                Update,
                (
                    ClientBlockOutlineSet::Collect,
                    ClientBlockOutlineSet::Apply,
                    ClientBlockOutlineSet::Draw,
                )
                    .chain(),
            )
            .add_systems(
                Update,
                apply_block_outline_commands.in_set(ClientBlockOutlineSet::Apply),
            )
            .add_systems(OnExit(GameState::InGame), clear_block_outlines);
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ClientBlockOutlineApi for ClientBlockOutlineBevyMod {}

fn apply_block_outline_commands(
    mut entity_commands: Commands,
    mut commands: MessageReader<SetClientBlockOutline>,
    mesh: Res<BlockOutlineMesh>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut active: ResMut<ActiveBlockOutlines>,
) {
    for command in commands.read() {
        remove_outline(
            &command.owner,
            &mut entity_commands,
            &mut materials,
            &mut active,
        );
        let Some(block) = command.block else {
            continue;
        };
        let outline = spawn_outline(
            &mut entity_commands,
            &mesh.0,
            &mut materials,
            block,
            &command.shape,
            command.style,
        );
        active.0.insert(command.owner.clone(), outline);
    }
}

fn spawn_outline(
    commands: &mut Commands,
    mesh: &Handle<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    block: BlockPos,
    shape: &BlockShape,
    style: BlockOutlineStyle,
) -> ActiveBlockOutline {
    let color = Color::srgba(
        style.color[0],
        style.color[1],
        style.color[2],
        style.color[3],
    );
    let material = materials.add(StandardMaterial {
        base_color: color,
        alpha_mode: if style.color[3] < 1.0 {
            AlphaMode::Blend
        } else {
            AlphaMode::Opaque
        },
        unlit: true,
        ..default()
    });
    let thickness = OUTLINE_EDGE_THICKNESS;
    let expansion = style.expansion.max(0.0);
    let origin = Vec3::new(block.x as f32, block.y as f32, block.z as f32);
    let entity = commands
        .spawn((Transform::from_translation(origin), Visibility::default()))
        .with_children(|parent| {
            for edge in shape.boundary_edges() {
                let axis_direction = (edge.end - edge.start).normalize_or_zero();
                let start =
                    edge.start - axis_direction * expansion + edge.expansion_direction * expansion;
                let end =
                    edge.end + axis_direction * expansion + edge.expansion_direction * expansion;
                let length = end - start;
                let mut scale = Vec3::splat(thickness);
                let absolute = length.abs();
                let axis = if absolute.x > 0.0 {
                    0
                } else if absolute.y > 0.0 {
                    1
                } else {
                    2
                };
                scale[axis] = length[axis].abs() + thickness;
                parent.spawn((
                    Mesh3d(mesh.clone()),
                    MeshMaterial3d(material.clone()),
                    Transform::from_translation((start + end) * 0.5).with_scale(scale),
                    NotShadowCaster,
                    NotShadowReceiver,
                    Pickable::IGNORE,
                ));
            }
        })
        .id();
    ActiveBlockOutline { entity, material }
}

fn remove_outline(
    owner: &str,
    commands: &mut Commands,
    materials: &mut Assets<StandardMaterial>,
    active: &mut ActiveBlockOutlines,
) {
    if let Some(outline) = active.0.remove(owner) {
        commands.entity(outline.entity).despawn();
        materials.remove(outline.material.id());
    }
}

fn clear_block_outlines(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut active: ResMut<ActiveBlockOutlines>,
) {
    for (_, outline) in active.0.drain() {
        commands.entity(outline.entity).despawn();
        materials.remove(outline.material.id());
    }
}
