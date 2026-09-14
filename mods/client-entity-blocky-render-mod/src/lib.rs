use bevy::prelude::*;
use std::collections::{HashMap,HashSet};
use client_entity_api::*;
use entity_state_api::*;
use blocky_model_api::*;
#[derive(Resource,Default)]
struct Visuals {
 cancelled:HashSet<u64>,
 pending:HashMap<u64,Uuid>,
 roots:HashMap<Uuid,(Entity,Option<EntityAnimationState>)>,
}
pub struct ClientEntityBlockyRenderMod;
impl ClientEntityBlockyRenderMod {
 pub fn init(bevy:&mut bevy_mod::BevyMod,_cache:&mut client_entity_network_mod::ClientEntityNetworkMod,_models:&mut impl BlockyModelApi,_animation:&mut impl blocky_animation_api::BlockyAnimationApi,_game:&mut impl client_game_state_api::GameStateApi)->Self {
 bevy.app.init_resource::<Visuals>().init_resource::<EntityInterpolationSettings>().add_systems(Update,render.in_set(ClientEntitySet::Present).run_if(in_state(client_game_state_api::GameState::InGame)))
 .add_systems(OnExit(client_game_state_api::GameState::InGame),clear);Self
 } pub fn run(&self)->Option<Vec<tokio::task::JoinHandle<()>>>{None}
}
fn transform(p:&EntityPose)->Transform {
 Transform{translation:Vec3::from_array(p.position),rotation:Quat::from_array(p.rotation).normalize(),scale:Vec3::splat(p.scale)}
}
fn render(time:Res<Time>,settings:Res<EntityInterpolationSettings>,cache:Res<ClientEntities>,mut visuals:ResMut<Visuals>,mut commands:Commands,
 mut spawns:MessageWriter<SpawnBlockyModel>,mut spawned:MessageReader<BlockyModelSpawned>,
 mut animations:MessageWriter<PlayBlockyAnimation>,mut transforms:Query<&mut Transform>) {
 for result in spawned.read(){
 if result.spawn_id.is_some_and(|id|visuals.cancelled.remove(&id)){commands.entity(result.root).try_despawn();continue;}
 let Some(uuid)=result.spawn_id.and_then(|id|visuals.pending.remove(&id))else{continue;};
 if cache.0.contains_key(&uuid){visuals.roots.insert(uuid,(result.root,None));}
 else{commands.entity(result.root).try_despawn();}
 }
 visuals.roots.retain(|uuid,(root,_)|{
 if cache.0.contains_key(uuid){true}else{commands.entity(*root).try_despawn();false}
 });
 for (uuid,entity) in &cache.0{
 let info=generated_entity_registry::info(entity.kind);
 if let Some((root,playing))=visuals.roots.get_mut(uuid){
 if let Ok(mut t)=transforms.get_mut(*root){
 let target=transform(&entity.pose);
 let alpha=1.0-(-settings.response_per_second.max(0.0)*time.delta_secs()).exp();
 t.translation=t.translation.lerp(target.translation,alpha);
 t.rotation=t.rotation.slerp(target.rotation,alpha);
 t.scale=target.scale;
 }
 if *playing!=entity.animation {
 if let Some(a)=&entity.animation {
 if let Some(clip)=info.animations.iter().find(|clip|clip.id==a.clip){
 animations.write(PlayBlockyAnimation{root:*root,animation_path:clip.path.into(),speed:a.speed,playback:if a.repeat{BlockyAnimationPlayback::Loop}else{BlockyAnimationPlayback::Once}});
 }
 }else{commands.entity(*root).remove::<BlockyAnimationPlayer>();}
 *playing=entity.animation.clone();
 }
 }else if !visuals.pending.values().any(|id|id==uuid){
 let Some(model)=info.model_path else{continue;};
 let id=next_blocky_spawn_id();visuals.pending.insert(id,*uuid);
 spawns.write(SpawnBlockyModel{spawn_id:Some(id),model_path:model.into(),texture_path:info.texture_path.map(str::to_owned),
 texture_size:Some(UVec2::from_array(info.texture_size)),transform:transform(&entity.pose),scale:1.0,primitive_scale:info.primitive_scale});
 }
 }
}
fn clear(mut visuals:ResMut<Visuals>,mut commands:Commands){
 for (_, (root,_)) in visuals.roots.drain(){commands.entity(root).try_despawn();}
 let cancelled:Vec<_>=visuals.pending.drain().map(|(id,_)|id).collect();
 visuals.cancelled.extend(cancelled);
}
