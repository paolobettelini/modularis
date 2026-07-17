use bevy::prelude::*;
use bevy_mod::BevyMod;
use block_manager_api::BlockManagerApi;
use block_shape_api::{BlockShape, BlockShapeApi, BlockShapeService};
use client_block_interaction_rules_api::{
    ClientBlockInteractionRules, ClientBlockInteractionRulesApi,
};
use client_block_outline_api::{
    BlockOutlineStyle, ClientBlockOutlineApi, ClientBlockOutlineSet, SetClientBlockOutline,
};
use client_camera_api::{CameraApi, PlayerCamera};
use client_chunk_cache_api::{ClientChunkCache, ClientChunkCacheApi};
use client_game_state_api::{GameStateApi, InGameOverlayState};
use std::marker::PhantomData;
use tokio::task::JoinHandle;
use voxel_math_api::BlockPos;
use voxel_raycast_api::raycast_voxel_shapes;

const OUTLINE_OWNER: &str = "vanilla:looked-block";

#[derive(Resource, Default)]
struct LookedBlockOutlineTarget(Option<(BlockPos, BlockShape)>);

pub struct ClientLookedBlockOutlineVanillaMod<B>(PhantomData<B>);

impl<B: BlockManagerApi> ClientLookedBlockOutlineVanillaMod<B> {
    pub fn init<
        R: ClientBlockInteractionRulesApi,
        O: ClientBlockOutlineApi,
        C: CameraApi,
        K: ClientChunkCacheApi,
        G: GameStateApi,
        S: BlockShapeApi,
    >(
        bevy: &mut BevyMod,
        _blocks: &mut B,
        _rules: &mut R,
        _outline: &mut O,
        _camera: &mut C,
        _cache: &mut K,
        _game_state: &mut G,
        _shapes: &mut S,
    ) -> Self {
        bevy.app
            .init_resource::<LookedBlockOutlineTarget>()
            .add_systems(
                Update,
                update_looked_block_outline::<B>
                    .in_set(ClientBlockOutlineSet::Collect)
                    .run_if(in_state(InGameOverlayState::Playing)),
            )
            .add_systems(
                OnExit(InGameOverlayState::Playing),
                clear_looked_block_outline,
            );
        Self(PhantomData)
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn update_looked_block_outline<B: BlockManagerApi>(
    rules: Res<ClientBlockInteractionRules>,
    cache: Res<ClientChunkCache>,
    shapes: Res<BlockShapeService>,
    camera: Query<&GlobalTransform, With<PlayerCamera>>,
    mut current: ResMut<LookedBlockOutlineTarget>,
    mut outlines: MessageWriter<SetClientBlockOutline>,
) {
    let target = camera.single().ok().and_then(|camera| {
        let transform = camera.compute_transform();
        raycast_voxel_shapes(
            transform.translation,
            transform.forward().as_vec3(),
            rules.max_reach,
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
        )
        .and_then(|hit| {
            cache
                .block(hit.block)
                .map(|block| (hit.block, shapes.shape(&block)))
        })
    });

    if current.0 == target {
        return;
    }
    current.0 = target.clone();
    outlines.write(SetClientBlockOutline {
        owner: OUTLINE_OWNER.to_string(),
        block: target.as_ref().map(|(block, _)| *block),
        shape: target
            .map(|(_, shape)| shape)
            .unwrap_or_else(BlockShape::empty),
        style: BlockOutlineStyle::default(),
    });
}

fn clear_looked_block_outline(
    mut current: ResMut<LookedBlockOutlineTarget>,
    mut outlines: MessageWriter<SetClientBlockOutline>,
) {
    current.0 = None;
    outlines.write(SetClientBlockOutline {
        owner: OUTLINE_OWNER.to_string(),
        block: None,
        shape: BlockShape::empty(),
        style: BlockOutlineStyle::default(),
    });
}
