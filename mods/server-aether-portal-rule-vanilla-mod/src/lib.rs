use bevy_mod::BevyMod;
use generated_block_registry::BlockId;
use generated_dimension_registry::Dimension;
use server_portal_api::{PortalRule, ServerPortalApi, ServerPortalRules};
use tokio::task::JoinHandle;

pub struct ServerAetherPortalRuleVanillaMod;

impl ServerAetherPortalRuleVanillaMod {
    pub fn init<P: ServerPortalApi>(
        bevy: &mut BevyMod,
        _portals: &mut P,
        _overworld: &mut dimension_overworld::DimensionOverworldMod,
        _aether: &mut dimension_aether::DimensionAetherMod,
    ) -> Self {
        bevy.app
            .world_mut()
            .resource_mut::<ServerPortalRules>()
            .register(PortalRule {
                id: "demo:aether".to_string(),
                frame_block: BlockId::Glowstone,
                destination: Dimension::Aether,
                return_destination: Dimension::Overworld,
                color: [0.04, 0.28, 0.95, 0.52],
            })
            .expect("the vanilla Aether portal rule must be unique");
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
