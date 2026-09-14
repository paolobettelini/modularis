use bevy::prelude::*;
use client_entity_api::*;
use generated_network_messages::*;
use client_game_state_api::GameState;
pub struct ClientEntityNetworkMod;
impl ClientEntityNetworkMod {
 pub fn init(bevy:&mut bevy_mod::BevyMod,_network:&mut network_protocol_mod::NetworkProtocolMod,_state:&mut impl client_game_state_api::GameStateApi)->Self{
 bevy.app.init_resource::<ClientEntities>()
 .configure_sets(Update,(ClientEntitySet::Receive,ClientEntitySet::Present).chain().after(NetworkMessageSet::DispatchPackets))
 .add_systems(Update,receive.in_set(ClientEntitySet::Receive))
 .add_systems(OnExit(GameState::InGame),clear);Self
 } pub fn run(&self)->Option<Vec<tokio::task::JoinHandle<()>>>{None}
}
fn receive(mut upserts:MessageReader<EntityUpsertPacketReceived>,mut despawns:MessageReader<EntityDespawnPacketReceived>,mut cache:ResMut<ClientEntities>){
 // Packet dispatch uses different ECS message types. Restore wire tick order
 // when leave/re-enter packets are delivered together in one rendered frame.
 let mut updates:Vec<_>=upserts.read().map(|p|(p.0.sequence,p.0.entity.uuid,Some(p.0.entity.clone())))
 .chain(despawns.read().map(|p|(p.0.sequence,p.0.uuid,None))).collect();
 updates.sort_by_key(|p|p.0);
 for (_,uuid,value) in updates {match value {
 Some(entity) if entity.pose.valid()=>{cache.0.insert(uuid,entity);},
 None=>{cache.0.remove(&uuid);},_=>{}
 }}
}
fn clear(mut cache:ResMut<ClientEntities>){cache.0.clear();}
