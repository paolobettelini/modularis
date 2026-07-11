use bevy::prelude::*;
use bevy_mod::BevyMod;
use blocky_animation_api::BlockyAnimationApi;
use blocky_formats::BlockyAnimation;
use blocky_model_api::{
    BlockyAnimationPlayback, BlockyAnimationPlayer, BlockyAnimationTranslationMask,
    BlockyModelNode, BlockyModelRoot, BlockyModelVisual, PlayBlockyAnimation,
};
use client_game_state_api::{GameState, GameStateApi};
use std::{collections::HashMap, path::PathBuf};
use tokio::task::JoinHandle;

pub struct ClientBlockyAnimationBevyMod;

impl ClientBlockyAnimationBevyMod {
    pub fn init<G: GameStateApi>(
        bevy: &mut BevyMod,
        _models: &mut impl blocky_model_api::BlockyModelApi,
        _game_state: &mut G,
    ) -> Self {
        bevy.app
            .add_message::<PlayBlockyAnimation>()
            .init_resource::<BlockyAnimationCache>()
            .add_systems(
                Update,
                (start_requested_animations, tick_blocky_animations)
                    .chain()
                    .run_if(in_state(GameState::InGame)),
            );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl BlockyAnimationApi for ClientBlockyAnimationBevyMod {}

#[derive(Resource, Default)]
struct BlockyAnimationCache {
    animations: HashMap<String, BlockyAnimation>,
}

fn start_requested_animations(
    mut commands: Commands,
    mut requests: MessageReader<PlayBlockyAnimation>,
    players: Query<&BlockyAnimationPlayer>,
) {
    for request in requests.read() {
        if let Ok(player) = players.get(request.root)
            && player.animation_path == request.animation_path
            && (player.speed - request.speed).abs() <= f32::EPSILON
            && player.playback == request.playback
        {
            continue;
        }
        commands.entity(request.root).insert(BlockyAnimationPlayer {
            animation_path: request.animation_path.clone(),
            elapsed_seconds: 0.0,
            speed: request.speed,
            playback: request.playback,
        });
    }
}

fn tick_blocky_animations(
    time: Res<Time>,
    mut cache: ResMut<BlockyAnimationCache>,
    mut roots: Query<(&BlockyModelRoot, &mut BlockyAnimationPlayer)>,
    mut nodes: Query<
        (
            &BlockyModelNode,
            Option<&BlockyAnimationTranslationMask>,
            &mut Transform,
        ),
        Without<BlockyModelVisual>,
    >,
    mut visuals: Query<
        (&BlockyModelVisual, &mut Transform, &mut Visibility),
        Without<BlockyModelNode>,
    >,
) {
    for (root, mut player) in &mut roots {
        let animation = match load_animation(&mut cache, &player.animation_path) {
            Ok(animation) => animation.clone(),
            Err(error) => {
                warn!(
                    "failed to load blocky animation '{}': {error}",
                    player.animation_path
                );
                continue;
            }
        };

        player.elapsed_seconds += time.delta_secs() * player.speed.max(0.0);
        let duration = animation.duration_seconds();
        let sample_seconds = animation_sample_seconds(&mut player, duration);

        for entity in &root.node_entities {
            let Ok((node, translation_mask, mut transform)) = nodes.get_mut(*entity) else {
                continue;
            };
            let Some(sample) = animation.sample_node_seconds(&node.name, sample_seconds) else {
                transform.translation = node.base_translation;
                transform.rotation = node.base_rotation;
                transform.scale = node.base_scale;
                reset_visual(node.visual, &mut visuals);
                continue;
            };

            transform.translation = sample
                .position
                .map(|position| {
                    let mask = translation_mask
                        .map(|translation_mask| translation_mask.mask)
                        .unwrap_or(Vec3::ONE);
                    node.base_translation + coord_vec3(position) * node.primitive_scale * mask
                })
                .unwrap_or(node.base_translation);
            transform.rotation = sample
                .orientation
                .map(|rotation| quat(rotation) * node.base_rotation)
                .unwrap_or(node.base_rotation);
            transform.scale = node.base_scale;
            apply_visual_sample(
                node.visual,
                sample.shape_stretch.map(scale_vec3),
                sample.shape_visible,
                &mut visuals,
            );
        }
    }
}

fn reset_visual(
    visual: Option<Entity>,
    visuals: &mut Query<
        (&BlockyModelVisual, &mut Transform, &mut Visibility),
        Without<BlockyModelNode>,
    >,
) {
    apply_visual_sample(visual, None, None, visuals);
}

fn apply_visual_sample(
    visual: Option<Entity>,
    shape_stretch: Option<Vec3>,
    shape_visible: Option<bool>,
    visuals: &mut Query<
        (&BlockyModelVisual, &mut Transform, &mut Visibility),
        Without<BlockyModelNode>,
    >,
) {
    let Some(visual) = visual else {
        return;
    };
    let Ok((visual, mut transform, mut visibility)) = visuals.get_mut(visual) else {
        return;
    };

    transform.translation = visual.base_translation;
    transform.rotation = visual.base_rotation;
    transform.scale = shape_stretch
        .map(|stretch| visual.base_scale * stretch)
        .unwrap_or(visual.base_scale);
    let visible = shape_visible.unwrap_or(visual.base_visible);
    *visibility = if visible {
        Visibility::Visible
    } else {
        Visibility::Hidden
    };
}

fn animation_sample_seconds(player: &mut BlockyAnimationPlayer, duration: f32) -> f32 {
    if duration <= 0.0 {
        player.elapsed_seconds = 0.0;
        return 0.0;
    }

    let last_sample = (duration - f32::EPSILON).max(0.0);
    match player.playback {
        BlockyAnimationPlayback::Once => player.elapsed_seconds.min(last_sample),
        BlockyAnimationPlayback::Loop => {
            player.elapsed_seconds = player.elapsed_seconds.rem_euclid(duration);
            player.elapsed_seconds
        }
        BlockyAnimationPlayback::PingPong => {
            let cycle_duration = duration * 2.0;
            let cycle_time = player.elapsed_seconds.rem_euclid(cycle_duration);
            if cycle_time <= duration {
                cycle_time.min(last_sample)
            } else {
                (cycle_duration - cycle_time).min(last_sample)
            }
        }
    }
}

fn load_animation<'a>(
    cache: &'a mut BlockyAnimationCache,
    path: &str,
) -> blocky_formats::Result<&'a BlockyAnimation> {
    if !cache.animations.contains_key(path) {
        let animation = BlockyAnimation::from_path(resolve_runtime_path(path))?;
        cache.animations.insert(path.to_string(), animation);
    }
    Ok(cache
        .animations
        .get(path)
        .expect("animation was just inserted"))
}

fn resolve_runtime_path(path: &str) -> PathBuf {
    let direct = PathBuf::from(path);
    if direct.is_absolute() || direct.exists() {
        direct
    } else {
        PathBuf::from("assets").join(path)
    }
}

fn coord_vec3(value: blocky_formats::Vec3f) -> Vec3 {
    Vec3::new(value.x, value.y, -value.z)
}

fn scale_vec3(value: blocky_formats::Vec3f) -> Vec3 {
    Vec3::new(value.x, value.y, value.z)
}

fn quat(value: blocky_formats::Quatf) -> Quat {
    Quat::from_xyzw(-value.x, -value.y, value.z, value.w).normalize()
}
