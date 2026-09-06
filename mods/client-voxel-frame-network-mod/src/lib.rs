use client_voxel_frame_movement_api::{ClientFrameAnimations,FrameAnimation};
use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_voxel_frame_api::{ClientVoxelFrames,ClientVoxelFrameEntities};
use client_game_state_api::{GameState,GameStateApi};
use client_dimension_api::{ClientDimensionApi,ClientDimensionChanged,ClientDimensionSet};
use generated_network_messages::{VoxelFrameUpsertReceived,VoxelFrameRemoveReceived,VoxelFramePoseReceived,NetworkMessageSet};
use network_protocol_mod::NetworkProtocolMod;
use voxel_frame_api::VoxelFrameId;
use std::collections::HashMap;
#[derive(Resource,Default)]
struct FrameStreamSequences { epoch:u64, last:HashMap<VoxelFrameId,u64> }
use client_world_context_api::{ClientWorldChanged,ClientWorldContextSet,ClientWorldContextApi};
use client_session_api::{ClientSession,ClientSessionApi};
use tokio::task::JoinHandle;
pub struct ClientVoxelFrameNetworkMod;
impl ClientVoxelFrameNetworkMod {
    pub fn init<G:GameStateApi,D:ClientDimensionApi,W:ClientWorldContextApi,S:ClientSessionApi>(bevy:&mut BevyMod,_network:&mut NetworkProtocolMod,_game:&mut G,_dimension:&mut D,_world:&mut W,_session:&mut S)->Self {
        bevy.app.init_resource::<ClientVoxelFrames>().init_resource::<ClientVoxelFrameEntities>().init_resource::<FrameStreamSequences>().init_resource::<ClientFrameAnimations>()
            .add_systems(Update,(reset_dimension,receive_frames).chain().in_set(client_voxel_frame_api::ClientVoxelFrameSet::Receive).after(NetworkMessageSet::DispatchPackets).after(ClientDimensionSet::ResetWorld).after(ClientWorldContextSet::ResetWorld))
            .add_systems(OnExit(GameState::InGame),clear_frames);
        Self
    }
    pub fn run(&self)->Option<Vec<JoinHandle<()>>>{None}
}
fn reset_dimension(mut animations:ResMut<ClientFrameAnimations>,mut worlds:MessageReader<ClientWorldChanged>,mut events:MessageReader<ClientDimensionChanged>,mut frames:ResMut<ClientVoxelFrames>,mut roots:ResMut<ClientVoxelFrameEntities>,mut commands:Commands) {
    let changed=events.read().any(|e|e.previous!=e.current);
    let world_changed=worlds.read().count()>0;
    if !changed && !world_changed { return; }
    animations.0.clear();
    frames.clear();
    for (_,entity) in roots.0.drain() { commands.entity(entity).try_despawn(); }
}
fn clear_frames(mut animations:ResMut<ClientFrameAnimations>,mut sequences:ResMut<FrameStreamSequences>,mut commands:Commands,mut frames:ResMut<ClientVoxelFrames>,mut roots:ResMut<ClientVoxelFrameEntities>) {
    *sequences=FrameStreamSequences::default();
    animations.0.clear();
    frames.clear();
    for (_,entity) in roots.0.drain() { commands.entity(entity).try_despawn(); }
}
fn receive_frames(time:Res<Time>,mut animations:ResMut<ClientFrameAnimations>,mut poses:MessageReader<VoxelFramePoseReceived>,session:Res<ClientSession>,mut upserts:MessageReader<VoxelFrameUpsertReceived>,mut removes:MessageReader<VoxelFrameRemoveReceived>,mut frames:ResMut<ClientVoxelFrames>,mut roots:ResMut<ClientVoxelFrameEntities>,mut commands:Commands,mut sequences:ResMut<FrameStreamSequences>) {
    // Typed readers do not preserve ordering across different packet types.
    enum Operation { Upsert(voxel_frame_network_message_types::VoxelFrameUpsert), Pose(voxel_frame_network_message_types::VoxelFramePose), Remove(voxel_frame_network_message_types::VoxelFrameRemove) }
    if sequences.epoch!=session.movement_epoch { sequences.last.clear(); sequences.epoch=session.movement_epoch; }
    let mut operations=Vec::new();
    for p in upserts.read() { if p.0.movement_epoch==session.movement_epoch {operations.push((p.0.stream_sequence,p.0.frame.id,Operation::Upsert(p.0.clone())));} }
    for p in poses.read() { if p.0.movement_epoch==session.movement_epoch {operations.push((p.0.stream_sequence,p.0.id,Operation::Pose(p.0.clone())));} }
    for p in removes.read() { if p.0.movement_epoch==session.movement_epoch {operations.push((p.0.stream_sequence,p.0.id,Operation::Remove(p.0.clone())));} }
    operations.sort_by_key(|op|op.0);
    for (sequence,id,operation) in operations {
        if sequence<=sequences.last.get(&id).copied().unwrap_or(0) {continue;}
        sequences.last.insert(id,sequence);
        match operation {
            Operation::Upsert(packet)=>{animations.0.remove(&id);frames.upsert(packet.frame);},
            Operation::Pose(packet)=>{
                let Some(scope)=&frames.scope else {continue;};
                if let Some(mut frame)=frames.registry.get(scope,id) {
                    if packet.revision>=frame.revision {
                        if !packet.movement.valid(){warn!("invalid frame movement ignored");continue;}
                        animations.0.remove(&id);
                        if !packet.affects_collision {
                            animations.0.insert(id,FrameAnimation{affects_collision:false,start:frame.transform,target:packet.transform,movement:packet.movement,started_at:time.elapsed_secs_f64()});
                            continue;
                        }
                        match packet.movement {
                            voxel_frame_network_message_types::FrameMovement::Animated{duration_ms,..} if duration_ms>0=>{
                                let (start,started_at)=packet.trajectory.as_ref().map(|track|(track.start,time.elapsed_secs_f64()-(packet.server_time_seconds-track.started_at_seconds).max(0.0))).unwrap_or((frame.transform,time.elapsed_secs_f64()));
                                animations.0.insert(id,FrameAnimation{affects_collision:true,start,target:packet.transform,movement:packet.movement,started_at});
                            }
                            _=>frame.transform=packet.transform,
                        }
                        frame.revision=packet.revision;frames.upsert(frame);
                    }
                }
            }
            Operation::Remove(packet)=>{
                animations.0.remove(&id);
                if let Some(scope)=&frames.scope {frames.registry.remove(scope,packet.id);}
                if let Some(entity)=roots.0.remove(&packet.id) {commands.entity(entity).try_despawn();}
            }
        }
    }
}
