use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_dimension_api::Dimension;
use client_grass_tint_api::{ClientGrassTintApi, GrassTintContext, GrassTintService};
use generated_block_registry::BlockId;
use tokio::task::JoinHandle;

pub struct ClientGrassTintVanillaMod;

impl ClientGrassTintVanillaMod {
    #[allow(clippy::too_many_arguments)]
    pub fn init(
        bevy: &mut BevyMod,
        _grass: &mut block_grass::BlockGrassMod,
        _moss: &mut block_moss::BlockMossMod,
        _netherrack: &mut block_netherrack::BlockNetherrackMod,
        _crimson: &mut block_crimson_nylium::BlockCrimsonNyliumMod,
        _warped: &mut block_warped_nylium::BlockWarpedNyliumMod,
        _soul_sand: &mut block_soul_sand::BlockSoulSandMod,
        _basalt: &mut block_basalt::BlockBasaltMod,
        _calcite: &mut block_calcite::BlockCalciteMod,
    ) -> Self {
        bevy.app
            .insert_resource(GrassTintService::new(vanilla_tint));
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ClientGrassTintApi for ClientGrassTintVanillaMod {}

fn vanilla_tint(context: GrassTintContext) -> Vec3 {
    match (context.dimension, context.support) {
        // The Overworld palette is deliberately less saturated than the first
        // grass shader prototype.
        (Dimension::Overworld, BlockId::Grass) => Vec3::new(0.38, 0.56, 0.24),
        (Dimension::Overworld, BlockId::Moss) => Vec3::new(0.42, 0.50, 0.20),

        (Dimension::Nether, BlockId::CrimsonNylium) => Vec3::new(0.58, 0.07, 0.12),
        (Dimension::Nether, BlockId::WarpedNylium) => Vec3::new(0.05, 0.48, 0.42),
        (Dimension::Nether, BlockId::SoulSand) => Vec3::new(0.34, 0.29, 0.25),
        (Dimension::Nether, BlockId::Basalt) => Vec3::new(0.27, 0.25, 0.28),
        (Dimension::Nether, BlockId::Netherrack) => Vec3::new(0.43, 0.09, 0.08),

        (Dimension::Aether, BlockId::Grass) => Vec3::new(0.52, 0.76, 0.36),
        (Dimension::Aether, BlockId::Moss) => Vec3::new(0.72, 0.73, 0.22),
        (Dimension::Aether, BlockId::Calcite) => Vec3::new(0.56, 0.68, 0.82),

        (Dimension::Overworld, _) => Vec3::new(0.36, 0.53, 0.23),
        (Dimension::Nether, _) => Vec3::new(0.44, 0.10, 0.12),
        (Dimension::Aether, _) => Vec3::new(0.54, 0.73, 0.40),
    }
}
