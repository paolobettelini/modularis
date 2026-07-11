use bevy_mod::BevyMod;
use client_chunk_vertex_lighting_api::{
    ChunkVertexLightingPipeline, ClientChunkVertexLightingApi, VertexOcclusion,
};
use tokio::task::JoinHandle;

pub struct ClientChunkAmbientOcclusionVanillaMod;

impl ClientChunkAmbientOcclusionVanillaMod {
    pub fn init<L: ClientChunkVertexLightingApi>(bevy: &mut BevyMod, _lighting: &mut L) -> Self {
        bevy.app
            .world()
            .resource::<ChunkVertexLightingPipeline>()
            .register_ambient_occlusion_stage(ambient_occlusion_brightness);
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn ambient_occlusion_brightness(sample: VertexOcclusion) -> f32 {
    let occlusion = if sample.side_a && sample.side_b {
        3
    } else {
        sample.side_a as usize + sample.side_b as usize + sample.corner as usize
    };
    [1.0, 0.84, 0.69, 0.54][occlusion]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn occlusion_darkens_progressively() {
        let clear = ambient_occlusion_brightness(VertexOcclusion::default());
        let one = ambient_occlusion_brightness(VertexOcclusion {
            side_a: true,
            ..Default::default()
        });
        let two = ambient_occlusion_brightness(VertexOcclusion {
            side_a: true,
            corner: true,
            ..Default::default()
        });
        let closed_corner = ambient_occlusion_brightness(VertexOcclusion {
            side_a: true,
            side_b: true,
            corner: false,
        });
        assert!(clear > one && one > two && two > closed_corner);
    }
}
