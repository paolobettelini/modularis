use bevy::prelude::*;
use server_entity_api::*;
pub struct ServerEntityStateMod;
impl ServerEntityStateMod {
 pub fn init(bevy:&mut bevy_mod::BevyMod)->Self {
 bevy.app.add_message::<EntityRequest>().add_message::<EntityChanged>()
 .configure_sets(Update,(ServerEntitySet::Request,ServerEntitySet::Apply,ServerEntitySet::Visibility,ServerEntitySet::Sync).chain())
 .add_systems(Update,apply.in_set(ServerEntitySet::Apply));Self
 } pub fn run(&self)->Option<Vec<tokio::task::JoinHandle<()>>>{None}
}
/// Exclusive application makes multiple requests in a tick sequential, including
/// spawn then move/despawn; no deferred-Commands duplicate UUID race.
fn apply(world:&mut World,mut cursor:Local<bevy::ecs::message::MessageCursor<EntityRequest>>) {
 let requests:Vec<_>=cursor.read(world.resource::<Messages<EntityRequest>>()).cloned().collect();
 if requests.is_empty(){return;}
 let mut index:std::collections::HashMap<_,_>=world.query::<(Entity,&ServerEntity)>().iter(world).map(|(id,e)|(e.0.uuid,id)).collect();
 for request in requests {
 let uuid=match &request {EntityRequest::Spawn{entity,..}=>entity.uuid,EntityRequest::SetPose{uuid,..}|EntityRequest::Animate{uuid,..}|EntityRequest::Despawn{uuid}=>*uuid};
 let found=index.get(&uuid).copied();
 match request {
 EntityRequest::Spawn{entity,world:scope}=>{
 if found.is_some() || !entity.pose.valid(){continue;}
 let id=world.spawn((ServerEntity(entity),EntityWorld(scope))).id();index.insert(uuid,id);
 },
 EntityRequest::SetPose{pose,..}=>{
 if !pose.valid(){continue;}
 let Some(e)=found else{continue;};world.get_mut::<ServerEntity>(e).unwrap().0.pose=pose;
 },
 EntityRequest::Animate{mut animation,..}=>{
 if animation.as_ref().is_some_and(|a|!a.speed.is_finite()||a.speed<=0.0){continue;}
 let Some(e)=found else{continue;};
 let mut entity=world.get_mut::<ServerEntity>(e).unwrap();
 if let Some(a)=&mut animation { a.revision=entity.0.animation.as_ref().map_or(1,|old|old.revision.wrapping_add(1)); }
 entity.0.animation=animation;
 },
 EntityRequest::Despawn{..}=>{let Some(e)=found else{continue;};world.despawn(e);index.remove(&uuid);}
 }
 world.write_message(EntityChanged(uuid));
 }
}
