use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_voxel_frame_api::{ClientVoxelFrames,ClientVoxelFrameEntities};
use voxel_frame_api::VoxelFrameEntity;
use tokio::task::JoinHandle;
pub struct ClientVoxelFrameRenderMod;
impl ClientVoxelFrameRenderMod {
    pub fn init(bevy:&mut BevyMod)->Self {
        bevy.app.init_resource::<ClientVoxelFrames>().init_resource::<ClientVoxelFrameEntities>()
            .add_systems(PostUpdate,update_transforms.before(TransformSystems::Propagate));Self
    }
    pub fn run(&self)->Option<Vec<JoinHandle<()>>>{None}
}
fn update_transforms(time:Res<Time>,animations:Option<Res<client_voxel_frame_movement_api::ClientFrameAnimations>>,frames:Res<ClientVoxelFrames>,mut entities:Query<(&VoxelFrameEntity,&mut Transform)>) {
    for (id,mut transform) in &mut entities {
        if let Some(pose)=frames.transform(id.0) {
            let pose=animations.as_ref().and_then(|a|a.0.get(&id.0)).map(|a|a.movement.sample(a.start,a.target,time.elapsed_secs_f64()-a.started_at).0).unwrap_or(pose);
            let next=pose.render_transform(frames.render_origin);
            if *transform!=next {*transform=next;}
        }
    }
}
