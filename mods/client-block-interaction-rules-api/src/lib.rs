use bevy::prelude::*;

#[derive(Resource, Debug, Clone, Copy)]
pub struct ClientBlockInteractionRules {
    pub max_reach: f32,
}

pub trait ClientBlockInteractionRulesApi: Send + Sync + 'static {}
