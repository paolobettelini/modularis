use bevy::prelude::*;
use entity_state_api::Uuid;
use std::collections::{HashMap,HashSet};
/// Filled by policy after Apply, consumed by replication. Empty means hidden.
/// Custom policies may use Audience, scope trees, teams or arbitrary ECS data.
#[derive(Resource,Default)]
pub struct EntityViewers(pub HashMap<Uuid,HashSet<u64>>);
