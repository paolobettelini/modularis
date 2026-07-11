use bevy::prelude::*;

#[derive(Message, Debug, Clone)]
pub struct SpawnBlockyModel {
    pub spawn_id: Option<u64>,
    pub model_path: String,
    pub texture_path: Option<String>,
    pub texture_size: Option<UVec2>,
    pub transform: Transform,
    pub scale: f32,
    pub primitive_scale: f32,
}

#[derive(Message, Debug, Clone)]
pub struct BlockyModelSpawned {
    pub spawn_id: Option<u64>,
    pub root: Entity,
    pub model_path: String,
}

#[derive(Message, Debug, Clone)]
pub struct PlayBlockyAnimation {
    pub root: Entity,
    pub animation_path: String,
    pub speed: f32,
    pub playback: BlockyAnimationPlayback,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BlockyAnimationPlayback {
    #[default]
    Once,
    Loop,
    PingPong,
}

#[derive(Component, Debug, Clone)]
pub struct BlockyModelRoot {
    pub model_path: String,
    pub node_entities: Vec<Entity>,
    pub visual_entities: Vec<Option<Entity>>,
}

#[derive(Component, Debug, Clone)]
pub struct BlockyModelNode {
    pub root: Entity,
    pub node_index: usize,
    pub name: String,
    pub visual: Option<Entity>,
    pub primitive_scale: f32,
    pub base_translation: Vec3,
    pub base_rotation: Quat,
    pub base_scale: Vec3,
}

#[derive(Component, Debug, Clone)]
pub struct BlockyModelVisual {
    pub root: Entity,
    pub node: Entity,
    pub node_index: usize,
    pub base_translation: Vec3,
    pub base_rotation: Quat,
    pub base_scale: Vec3,
    pub base_visible: bool,
}

#[derive(Component, Debug, Clone)]
pub struct BlockyAnimationPlayer {
    pub animation_path: String,
    pub elapsed_seconds: f32,
    pub speed: f32,
    pub playback: BlockyAnimationPlayback,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct BlockyAnimationTranslationMask {
    pub mask: Vec3,
}

pub trait BlockyModelApi: Send + Sync + 'static {}
