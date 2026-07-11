use bevy_mod::BevyMod;
use generated_block_registry::BlockId;
use generated_dimension_registry::Dimension;
use server_portal_api::{PortalRule, ServerPortalApi, ServerPortalRules};
use tokio::task::JoinHandle;

pub struct ServerNetherPortalRuleVanillaMod;

impl ServerNetherPortalRuleVanillaMod {
    pub fn init<P: ServerPortalApi>(
        bevy: &mut BevyMod,
        _portals: &mut P,
        _overworld: &mut dimension_overworld::DimensionOverworldMod,
        _nether: &mut dimension_nether::DimensionNetherMod,
    ) -> Self {
        bevy.app
            .world_mut()
            .resource_mut::<ServerPortalRules>()
            .register(PortalRule {
                id: "demo:nether".to_string(),
                frame_block: BlockId::Obsidian,
                destination: Dimension::Nether,
                return_destination: Dimension::Overworld,
                color: [0.72, 0.025, 0.04, 0.48],
            })
            .expect("the vanilla Nether portal rule must be unique");
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
