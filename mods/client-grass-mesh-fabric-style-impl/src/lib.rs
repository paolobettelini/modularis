use bevy::prelude::*;
use bevy_mod::BevyMod;
use chunk_api::Chunk;
use client_dimension_api::Dimension;
use client_grass_mesh_api::{ClientGrassMeshApi, GrassMeshData, GrassMeshService};
use client_grass_settings_api::{ClientGrassSettings, ClientGrassSettingsApi};
use client_grass_tint_api::{ClientGrassTintApi, GrassTintContext, GrassTintService};
use generated_block_registry::BlockId;
use tokio::task::JoinHandle;

const BLADE_SEGMENTS: usize = 3;

pub struct ClientGrassMeshFabricStyleImpl;

impl ClientGrassMeshFabricStyleImpl {
    pub fn init<S: ClientGrassSettingsApi, T: ClientGrassTintApi>(
        bevy: &mut BevyMod,
        _settings: &mut S,
        _tint_api: &mut T,
        _grass: &mut block_short_grass::BlockShortGrassMod,
    ) -> Self {
        let tint = bevy.app.world().resource::<GrassTintService>().clone();
        bevy.app.insert_resource(GrassMeshService::new(
            move |chunk, settings, distance, dimension| {
                build_grass_mesh(chunk, settings, distance, dimension, &tint)
            },
        ));
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ClientGrassMeshApi for ClientGrassMeshFabricStyleImpl {}

fn build_grass_mesh(
    chunk: &Chunk,
    settings: ClientGrassSettings,
    distance: f32,
    dimension: Dimension,
    tint: &GrassTintService,
) -> GrassMeshData {
    let lod_density = if !settings.render_lod || distance <= 32.0 {
        1.0
    } else if distance <= 80.0 {
        0.60
    } else if distance <= 128.0 {
        0.35
    } else {
        0.20
    };
    let blades_per_block =
        ((settings.blades_per_block as f32 * lod_density).round() as usize).max(1);
    let segments = if !settings.render_lod || distance <= 32.0 {
        BLADE_SEGMENTS
    } else if distance <= 80.0 {
        2
    } else {
        1
    };

    let mut mesh = GrassMeshData::default();
    let origin = chunk.position().world_origin();

    for (local, instance) in chunk.iter() {
        if instance.block != BlockId::ShortGrass {
            continue;
        }
        let world_x = origin.x + local.x as i32;
        let world_y = origin.y + local.y as i32;
        let world_z = origin.z + local.z as i32;
        let support = local
            .y
            .checked_sub(1)
            .and_then(|y| {
                voxel_math_api::LocalBlockPos::new(local.x as i32, y as i32, local.z as i32)
            })
            .map(|position| chunk.get(position).block)
            .unwrap_or(BlockId::Air);
        let tint = tint.tint(GrassTintContext { dimension, support });
        for blade in 0..blades_per_block {
            let mut random = RandomStream::new(position_seed(world_x, world_y, world_z, blade));
            if random.next_f32() < settings.sparsity {
                continue;
            }
            let center = Vec3::new(
                local.x as f32 + 0.08 + random.next_f32() * 0.84,
                local.y as f32,
                local.z as f32 + 0.08 + random.next_f32() * 0.84,
            );
            let angle = ((random.next_f32() * 31.0).round() / 31.0) * std::f32::consts::TAU;
            let height = settings.blade_height
                * (1.0 + (random.next_f32() * 2.0 - 1.0) * settings.height_variation);
            let width = 0.045 * settings.blade_width * (0.88 + random.next_f32() * 0.24);
            let phase = random.next_f32();
            add_blade(
                &mut mesh,
                center,
                angle,
                height.max(0.02),
                width,
                phase,
                segments,
                tint,
            );
            mesh.blade_count += 1;
        }
    }

    mesh
}

fn add_blade(
    mesh: &mut GrassMeshData,
    center: Vec3,
    angle: f32,
    height: f32,
    width: f32,
    phase: f32,
    segments: usize,
    tint: Vec3,
) {
    for crossed in [0.0, std::f32::consts::FRAC_PI_2] {
        let direction = Vec2::new((angle + crossed).cos(), (angle + crossed).sin());
        let normal = Vec3::new(-direction.y, 0.0, direction.x);
        let base = mesh.positions.len() as u32;
        for row in 0..=segments {
            let fraction = row as f32 / segments as f32;
            let row_width = if fraction <= 2.0 / 3.0 {
                width * (0.88 + fraction * 0.18)
            } else {
                width * (1.06 - (fraction - 2.0 / 3.0) * 3.0)
            }
            .max(0.002);
            for side in [-1.0, 1.0] {
                let offset = direction * row_width * side;
                mesh.positions.push([
                    center.x + offset.x,
                    center.y + height * fraction,
                    center.z + offset.y,
                ]);
                mesh.normals.push(normal.to_array());
                mesh.uvs.push([phase, fraction]);
                mesh.colors.push([tint.x, tint.y, tint.z, 1.0]);
            }
        }
        for segment in 0..segments as u32 {
            let row = base + segment * 2;
            mesh.indices
                .extend_from_slice(&[row, row + 1, row + 3, row, row + 3, row + 2]);
        }
    }
}

fn position_seed(x: i32, y: i32, z: i32, blade: usize) -> u64 {
    let mut value = (x as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15)
        ^ (y as u64).wrapping_mul(0xbf58_476d_1ce4_e5b9)
        ^ (z as u64).wrapping_mul(0x94d0_49bb_1331_11eb)
        ^ (blade as u64).wrapping_mul(0xd6e8_feb8_6659_fd93);
    value ^= value >> 30;
    value = value.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    value ^= value >> 27;
    value = value.wrapping_mul(0x94d0_49bb_1331_11eb);
    value ^ (value >> 31)
}

struct RandomStream(u64);

impl RandomStream {
    fn new(seed: u64) -> Self {
        Self(seed)
    }

    fn next_f32(&mut self) -> f32 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        let value = self.0.wrapping_mul(0x2545_f491_4f6c_dd1d);
        ((value >> 40) as f32) / ((1u32 << 24) as f32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use voxel_math_api::{ChunkPos, LocalBlockPos};

    #[test]
    fn short_grass_blocks_produce_visible_blade_geometry() {
        let mut chunk = Chunk::filled(ChunkPos::new(0, 0, 0), BlockId::Air);
        chunk.set(LocalBlockPos { x: 4, y: 7, z: 9 }, BlockId::ShortGrass);

        let tint = GrassTintService::new(|_| Vec3::ONE);
        let mesh = build_grass_mesh(
            &chunk,
            ClientGrassSettings::default(),
            0.0,
            Dimension::Overworld,
            &tint,
        );

        assert!(mesh.blade_count > 0);
        assert!(!mesh.positions.is_empty());
        assert!(!mesh.indices.is_empty());
        assert_eq!(mesh.positions.len(), mesh.normals.len());
        assert_eq!(mesh.positions.len(), mesh.uvs.len());
        assert_eq!(mesh.positions.len(), mesh.colors.len());
    }
}
