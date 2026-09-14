use bevy::prelude::*;
use entity_state_api::*;
use world_instance_api::WorldScopeId;
#[derive(Component,Debug,Clone)]
pub struct ServerEntity(pub EntitySnapshot);
/// Independent visibility/storage affiliation; no concrete chunk provider.
#[derive(Component,Debug,Clone)]
pub struct EntityWorld(pub WorldScopeId);
#[derive(Message,Debug,Clone)]
pub enum EntityRequest {
 Spawn {entity:EntitySnapshot,world:WorldScopeId},
 SetPose {uuid:Uuid,pose:EntityPose},
 Animate {uuid:Uuid,animation:Option<EntityAnimationState>},
 Despawn {uuid:Uuid},
}
#[derive(Message,Debug,Clone)]
pub struct EntityChanged(pub Uuid);
#[derive(SystemSet,Debug,Clone,Copy,PartialEq,Eq,Hash)]
pub enum ServerEntitySet {Request,Apply,Visibility,Sync}
