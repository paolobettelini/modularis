use bevy::prelude::*;
use entity_state_api::*;
use std::collections::HashMap;
#[derive(Resource,Default)]
pub struct ClientEntities(pub HashMap<Uuid,EntitySnapshot>);
#[derive(Resource)]
pub struct EntityInterpolationSettings { pub response_per_second:f32 }
impl Default for EntityInterpolationSettings {fn default()->Self{Self{response_per_second:20.0}}}
#[derive(SystemSet,Debug,Clone,Copy,PartialEq,Eq,Hash)]
pub enum ClientEntitySet {Receive,Present}
