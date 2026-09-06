use bevy::prelude::*;
use bevy_mod::BevyMod;
use block_edit_events_api::BlockBreakRequested;
use block_edit_events_mod::BlockEditEventsMod;
use block_manager_api::BlockManagerApi;
use block_shape_api::{BlockShape, BlockShapeApi, BlockShapeService};
use client_block_interaction_events_api::{ClientBlockInteractionSet, LocalBlockUseIntent};
use client_block_interaction_events_mod::ClientBlockInteractionEventsMod;
use client_block_interaction_rules_api::{
    ClientBlockInteractionRules, ClientBlockInteractionRulesApi,
};
use client_camera_api::{CameraApi, PlayerCamera};
use client_chunk_cache_api::{ClientChunkCache, ClientChunkCacheApi};
use client_game_state_api::{GameStateApi, InGameOverlayState};
use client_input_api::{ClientInputSet, InputApi, PlayerInput};
use item_use_api::ItemUseTarget;
use std::marker::PhantomData;
use tokio::task::JoinHandle;
use voxel_frame_geometry_lib::{raycast,ray_bounds};
use client_voxel_frame_api::ClientVoxelFrames;

pub struct ClientBlockInteractionRaycastMod<B>(PhantomData<B>);

impl<B: BlockManagerApi> ClientBlockInteractionRaycastMod<B> {
    pub fn init<
        I: InputApi,
        C: CameraApi,
        K: ClientChunkCacheApi,
        S: BlockShapeApi,
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
        _shapes: &mut S,
        _game_state: &mut G,
        _rules: &mut Rules,
    ) -> Self {
        bevy.app.init_resource::<ClientVoxelFrames>().add_systems(
            Update,
            interact_with_blocks::<B>
                .run_if(in_state(InGameOverlayState::Playing))
                .after(ClientInputSet::Capture)
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
    frames: Res<ClientVoxelFrames>,
    cache: Res<ClientChunkCache>,
    shapes: Res<BlockShapeService>,
    camera: Query<&GlobalTransform, With<PlayerCamera>>,
    mut breaks: MessageWriter<BlockBreakRequested>,
    mut uses: MessageWriter<LocalBlockUseIntent>,
    mut counter: Local<u64>,
    mut last_logged_break: Local<Option<voxel_frame_api::VoxelBlockAddress>>,
) {
    if !input.break_block_held && !input.use_item_pressed {
        return;
    }
    let Ok(camera) = camera.single() else {
        return;
    };
    let transform = camera.compute_transform();
    let Some(hit) = raycast(
        transform.translation.as_dvec3()+frames.render_origin,
        transform.forward().as_vec3().as_dvec3(),
        rules.max_reach as f64,
            frames.scope.as_ref().map(|scope| frames.registry.query(scope,ray_bounds(transform.translation.as_dvec3()+frames.render_origin,transform.forward().as_vec3().as_dvec3(),rules.max_reach as f64))).unwrap_or_default(),
        |position| {
            cache
                .block(position)
                .map_or_else(BlockShape::empty, |block| {
                    if B::is_air(block.block) {
                        BlockShape::empty()
                    } else {
                        shapes.shape(&block)
                    }
                })
        },
    ) else {
        return;
    };

    if input.break_block_held {
        breaks.write(BlockBreakRequested {
            position: hit.block,
        });
        if *last_logged_break != Some(hit.block) {
            info!("local block-break raycast selected {:?}", hit.block);
            *last_logged_break = Some(hit.block);
        }
    }
    if input.use_item_pressed {
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
