use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_player_controller_api::*;
use client_player_surface_api::*;
use collision_api::*;
use player_gravity_api::Gravity;
use player_hitbox_api::PlayerHitbox;
use client_game_state_api::{GameState,GameStateApi};
use tokio::task::JoinHandle;
pub struct ClientPlayerSurfaceMotionMod;
impl ClientPlayerSurfaceMotionMod{
 pub fn init(bevy:&mut BevyMod,_player:&mut impl PlayerControllerApi,_collision:&mut impl CollisionApi,_gravity:&mut impl player_gravity_api::PlayerGravityApi,_hitbox:&mut impl player_hitbox_api::PlayerHitboxApi,_game:&mut impl GameStateApi)->Self{
  bevy.app.init_resource::<PlayerSurfaceAttachment>().init_resource::<PlayerSurfaceContact>()
   .add_systems(FixedUpdate,(
    carry.before(PlayerControllerSet::Input).after(client_voxel_frame_movement_api::ClientFrameAnimationSet),
    detach.after(PlayerControllerSet::ForceOverrides).before(PlayerControllerSet::MovementConstraints),
    attach.after(PlayerControllerSet::PostMovement),
   ).run_if(in_state(GameState::InGame)))
   .add_systems(OnExit(GameState::InGame),clear);Self
 }
 pub fn run(&self)->Option<Vec<JoinHandle<()>>>{None}
}
fn clear(mut attachment:ResMut<PlayerSurfaceAttachment>,mut contact:ResMut<PlayerSurfaceContact>){attachment.0=None;contact.0=None;}
fn carry(time:Res<Time>,collision:Res<CollisionService>,gravity:Res<Gravity>,hitbox:Res<PlayerHitbox>,mut attachment:ResMut<PlayerSurfaceAttachment>,mut players:Query<(&mut Transform,&mut PreviousPlayerPosition),With<Player>>){
 let Some(a)=attachment.0.as_mut()else{return;};
 let Ok((mut transform,mut previous))=players.single_mut()else{return;};
 // Relocations/corrections invalidate a local support anchor instead of dragging it back.
 if transform.translation.distance(a.last_player_position)>hitbox.radius.max(0.01){attachment.0=None;return;}
 let Some((translation,rotation))=collision.surface_pose(a.surface)else{attachment.0=None;return;};
 let target=translation+rotation*a.local_foot;
 let delta=target-transform.translation;
 a.velocity=delta/time.delta_secs().max(1e-6);
 let mut q=CharacterQuery::new(transform.translation,delta,gravity.up(),hitbox.radius,hitbox.height);
 q.step_height=0.0;q.ignore_surface=Some(a.surface);
 let transported=collision.resolve_character(q).position;
 let mut settle=CharacterQuery::new(transported,Vec3::ZERO,gravity.up(),hitbox.radius,hitbox.height);settle.step_height=0.0;
 let corrected=collision.resolve_character(settle).position;
 previous.0+=corrected-transform.translation;transform.translation=corrected;
 a.translation=translation;a.rotation=rotation;
}
fn detach(mut attachment:ResMut<PlayerSurfaceAttachment>,mut players:Query<(&Grounded,&mut PlayerVelocity),With<Player>>){
 let Ok((grounded,mut velocity))=players.single_mut()else{return;};
 if !grounded.0 {if let Some(a)=attachment.0.take(){velocity.0+=a.velocity;}}
}
fn attach(collision:Res<CollisionService>,contact:Res<PlayerSurfaceContact>,mut attachment:ResMut<PlayerSurfaceAttachment>,mut players:Query<(&Transform,&mut PlayerVelocity),With<Player>>){
 let Ok((player,mut velocity))=players.single_mut()else{return;};
 let Some(hit)=contact.0.filter(|hit|hit.surface!=0)else{if let Some(old)=attachment.0.take(){velocity.0+=old.velocity;}return;};
 let Some((translation,rotation))=collision.surface_pose(hit.surface)else{attachment.0=None;return;};
 let velocity=attachment.0.filter(|a|a.surface==hit.surface).map(|a|a.velocity).unwrap_or(Vec3::ZERO);
 attachment.0=Some(SurfaceAttachment{surface:hit.surface,local_foot:rotation.conjugate()*(player.translation-translation),translation,rotation,velocity,last_player_position:player.translation});
}
