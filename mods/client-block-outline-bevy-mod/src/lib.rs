use bevy::{gizmos::config::GizmoConfigStore, prelude::*};
use bevy_mod::BevyMod;
use client_bevy_default_plugins_mod::ClientBevyDefaultPluginsMod;
use client_block_outline_api::{
    BlockOutlineStyle, ClientBlockOutlineApi, ClientBlockOutlineSet, SetClientBlockOutline,
};
use client_game_state_api::{GameState, GameStateApi};
use std::collections::HashMap;
use tokio::task::JoinHandle;
use voxel_math_api::BlockPos;

#[derive(Default, Reflect, GizmoConfigGroup)]
struct ClientBlockOutlineGizmos;

#[derive(Debug, Clone, Copy)]
struct ActiveBlockOutline {
    block: BlockPos,
    style: BlockOutlineStyle,
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
            .init_gizmo_group::<ClientBlockOutlineGizmos>()
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
            .add_systems(Startup, configure_block_outline_gizmos)
            .add_systems(
                Update,
                apply_block_outline_commands.in_set(ClientBlockOutlineSet::Apply),
            )
            .add_systems(
                Update,
                draw_block_outlines
                    .in_set(ClientBlockOutlineSet::Draw)
                    .run_if(in_state(GameState::InGame)),
            );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ClientBlockOutlineApi for ClientBlockOutlineBevyMod {}

fn configure_block_outline_gizmos(mut configs: ResMut<GizmoConfigStore>) {
    let (config, _) = configs.config_mut::<ClientBlockOutlineGizmos>();
    config.line.width = 2.5;
    config.depth_bias = -0.001;
}

fn apply_block_outline_commands(
    mut commands: MessageReader<SetClientBlockOutline>,
    mut active: ResMut<ActiveBlockOutlines>,
) {
    for command in commands.read() {
        if let Some(block) = command.block {
            active.0.insert(
                command.owner.clone(),
                ActiveBlockOutline {
                    block,
                    style: command.style,
                },
            );
        } else {
            active.0.remove(&command.owner);
        }
    }
}

fn draw_block_outlines(
    active: Res<ActiveBlockOutlines>,
    mut gizmos: Gizmos<ClientBlockOutlineGizmos>,
) {
    for outline in active.0.values() {
        let center = Vec3::new(
            outline.block.x as f32 + 0.5,
            outline.block.y as f32 + 0.5,
            outline.block.z as f32 + 0.5,
        );
        let size = 1.0 + outline.style.expansion.max(0.0) * 2.0;
        gizmos.cuboid(
            Transform::from_translation(center).with_scale(Vec3::splat(size)),
            Color::srgba(
                outline.style.color[0],
                outline.style.color[1],
                outline.style.color[2],
                outline.style.color[3],
            ),
        );
    }
}
