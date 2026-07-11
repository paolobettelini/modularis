use bevy::prelude::*;
use generated_block_registry::BlockId;
use generated_dimension_registry::Dimension;
use player_network_message_types::PlayerId;
use portal_api::PortalFrame;
use world_instance_api::WorldScopeId;

#[derive(Debug, Clone, PartialEq)]
pub struct PortalRule {
    pub id: String,
    pub frame_block: BlockId,
    pub destination: Dimension,
    pub return_destination: Dimension,
    pub color: [f32; 4],
}

impl PortalRule {
    pub fn destination_from(&self, source: Dimension) -> Dimension {
        if source == self.destination {
            self.return_destination
        } else {
            self.destination
        }
    }
}

#[derive(Resource, Default)]
pub struct ServerPortalRules(Vec<PortalRule>);

impl ServerPortalRules {
    pub fn register(&mut self, rule: PortalRule) -> Result<(), String> {
        if self
            .0
            .iter()
            .any(|existing| existing.id == rule.id || existing.frame_block == rule.frame_block)
        {
            return Err(format!(
                "duplicate portal rule '{}' or frame block",
                rule.id
            ));
        }
        self.0.push(rule);
        Ok(())
    }

    pub fn for_frame_block(&self, block: BlockId) -> Option<&PortalRule> {
        self.0.iter().find(|rule| rule.frame_block == block)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ActivePortal {
    pub scope: WorldScopeId,
    pub frame: PortalFrame,
    pub frame_block: BlockId,
    pub destination: Dimension,
    pub destination_position: Option<[f32; 3]>,
    pub color: [f32; 4],
}

#[derive(Resource, Default)]
pub struct ServerPortals(Vec<ActivePortal>);

impl ServerPortals {
    pub fn insert(&mut self, portal: ActivePortal) -> bool {
        if self
            .0
            .iter()
            .any(|existing| existing.scope == portal.scope && existing.frame == portal.frame)
        {
            return false;
        }
        self.0.push(portal);
        true
    }

    pub fn in_scope(&self, scope: &WorldScopeId) -> impl Iterator<Item = &ActivePortal> {
        self.0.iter().filter(move |portal| &portal.scope == scope)
    }
}

#[derive(Message, Debug, Clone)]
pub struct ServerPortalOpened {
    pub player_id: PlayerId,
    pub portal: ActivePortal,
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ServerPortalSet {
    Ignite,
    Sync,
    Travel,
}

pub trait ServerPortalApi: Send + Sync + 'static {}
