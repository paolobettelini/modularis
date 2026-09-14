use bevy::prelude::*;
use std::collections::{HashMap,HashSet};
use server_entity_api::*;
use server_entity_visibility_api::*;
use entity_state_api::*;
use entity_network_message_types::*;
use server_network_events_api::*;
use generated_network_messages::ClientBoundMessage;
#[derive(Resource,Default)]
struct Sent(HashMap<u64,HashMap<Uuid,EntitySnapshot>>);
#[derive(Resource,Default)]
struct Sequence(u64);
pub struct ServerEntityNetworkSyncMod;
impl ServerEntityNetworkSyncMod{
 pub fn init(bevy:&mut bevy_mod::BevyMod,_entities:&mut server_entity_state_mod::ServerEntityStateMod,_network:&mut impl ServerNetworkEventsApi,_players:&mut impl server_player_registry_api::ServerPlayerRegistryApi)->Self{
 bevy.app.init_resource::<EntityViewers>().init_resource::<Sent>().init_resource::<Sequence>().add_systems(Update,sync.in_set(ServerEntitySet::Sync));Self
 } pub fn run(&self)->Option<Vec<tokio::task::JoinHandle<()>>>{None}
}
fn sync(entities:Query<&ServerEntity>,viewers:Res<EntityViewers>,players:Res<server_player_registry_api::ServerPlayerRegistry>,mut sent:ResMut<Sent>,mut sequence:ResMut<Sequence>,mut out:MessageWriter<ServerPacketOut>){
 sequence.0+=1;
 let online:HashSet<_>=players.players().into_iter().map(|p|p.id).collect();
 sent.0.retain(|id,_|online.contains(id));
 for player in online {
 let previous=sent.0.entry(player).or_default();
 let mut visible=HashSet::new();
 for entity in &entities{
 let e=&entity.0;
 if !viewers.0.get(&e.uuid).is_some_and(|v|v.contains(&player)){continue;}
 visible.insert(e.uuid);
 if previous.get(&e.uuid)!=Some(e){
 out.write(ServerPacketOut{audience:ServerAudience::Players(vec![player]),message:ClientBoundMessage::EntityUpsertPacket(EntityUpsertPacket{entity:e.clone(),sequence:sequence.0})});
 previous.insert(e.uuid,e.clone());
 }
 }
 previous.retain(|uuid,_|{
 if visible.contains(uuid){true}else{
 out.write(ServerPacketOut{audience:ServerAudience::Players(vec![player]),message:ClientBoundMessage::EntityDespawnPacket(EntityDespawnPacket{uuid:*uuid,sequence:sequence.0})});false
 }
 });
 }
}
