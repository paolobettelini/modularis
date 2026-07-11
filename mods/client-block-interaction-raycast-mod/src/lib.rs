use bevy::prelude::*;
use bevy_mod::BevyMod;
use block_edit_events_api::BlockBreakRequested;
use block_edit_events_mod::BlockEditEventsMod;
use block_manager_api::BlockManagerApi;
use client_block_interaction_events_api::{ClientBlockInteractionSet, LocalBlockUseIntent};
use client_block_interaction_events_mod::ClientBlockInteractionEventsMod;
use client_block_interaction_rules_api::{
    ClientBlockInteractionRules, ClientBlockInteractionRulesApi,
};
use client_camera_api::{CameraApi, PlayerCamera};
use client_chunk_cache_api::{ClientChunkCache, ClientChunkCacheApi};
use client_game_state_api::{GameStateApi, InGameOverlayState};
use client_input_api::{InputApi, PlayerInput};
use client_player_controller_api::{Player, PlayerControllerApi};
use client_player_render_api::{ClientPlayerRenderApi, RenderedNetworkPlayers};
use item_use_api::ItemUseTarget;
use player_hitbox_api::player_intersects_block;
use std::marker::PhantomData;
use tokio::task::JoinHandle;
use voxel_raycast_api::raycast_voxels;

pub struct ClientBlockInteractionRaycastMod<B>(PhantomData<B>);

impl<B: BlockManagerApi> ClientBlockInteractionRaycastMod<B> {
    pub fn init<
        I: InputApi,
        C: CameraApi,
        K: ClientChunkCacheApi,
        P: PlayerControllerApi,
        R: ClientPlayerRenderApi,
        G: GameStateApi,
        Rules: ClientBlockInteractionRulesApi,
    >(
        bevy: &mut BevyMod,
        _events: &mut BlockEditEventsMod,
        _interaction_events: &mut ClientBlockInteractionEventsMod,
        _blocks: &mut B,
        _input: &mut I,
        _camera: &mut C,
        _cache: &mut K,
        _player: &mut P,
        _player_render: &mut R,
        _game_state: &mut G,
        _rules: &mut Rules,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            interact_with_blocks::<B>
                .run_if(in_state(InGameOverlayState::Playing))
                .in_set(ClientBlockInteractionSet::Raycast),
        );
        Self(PhantomData)
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn interact_with_blocks<B: BlockManagerApi>(
    input: Res<PlayerInput>,
    rules: Res<ClientBlockInteractionRules>,
    cache: Res<ClientChunkCache>,
    camera: Query<&GlobalTransform, With<PlayerCamera>>,
    local_player: Query<&Transform, With<Player>>,
    remote_players: Res<RenderedNetworkPlayers>,
    transforms: Query<&Transform, Without<Player>>,
    mut breaks: MessageWriter<BlockBreakRequested>,
    mut uses: MessageWriter<LocalBlockUseIntent>,
    mut counter: Local<u64>,
) {
    if !input.break_block_pressed && !input.use_item_pressed {
        return;
    }
    let Ok(camera) = camera.single() else {
        return;
    };
    let transform = camera.compute_transform();
    let Some(hit) = raycast_voxels(
        transform.translation,
        transform.forward().as_vec3(),
        rules.max_reach,
        |position| {
            cache
                .block(position)
                .is_some_and(|block| !B::is_air(block.block))
        },
    ) else {
        return;
    };

    if input.break_block_pressed {
        breaks.write(BlockBreakRequested {
            position: hit.block,
        });
    }
    if input.use_item_pressed {
        let local_blocked = local_player.single().is_ok_and(|player| {
            player_intersects_block(player.translation.to_array(), hit.adjacent)
        });
        let remote_blocked = remote_players.entities.values().any(|visual| {
            transforms.get(visual.avatar).is_ok_and(|player| {
                player_intersects_block(player.translation.to_array(), hit.adjacent)
            })
        });
        if local_blocked || remote_blocked {
            return;
        }
        *counter = counter.wrapping_add(1);
        uses.write(LocalBlockUseIntent {
            operation_id: *counter,
            target: ItemUseTarget::Block {
                hit: hit.block,
                adjacent: hit.adjacent,
                normal: hit.normal.into(),
            },
        });
    }
}
