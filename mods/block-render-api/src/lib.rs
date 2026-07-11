#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderShape {
    Invisible,
    Cube,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlockFace {
    East,
    West,
    Top,
    Bottom,
    South,
    North,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockTextures {
    /// Use the same texture on every face.
    Uniform(&'static str),
    /// Define every face explicitly. Partial per-face definitions are avoided so
    /// a block cannot silently inherit an unexpected texture.
    PerFace {
        east: &'static str,
        west: &'static str,
        top: &'static str,
        bottom: &'static str,
        south: &'static str,
        north: &'static str,
    },
}

impl BlockTextures {
    pub const fn texture(self, face: BlockFace) -> &'static str {
        match self {
            Self::Uniform(texture) => texture,
            Self::PerFace {
                east,
                west,
                top,
                bottom,
                south,
                north,
            } => match face {
                BlockFace::East => east,
                BlockFace::West => west,
                BlockFace::Top => top,
                BlockFace::Bottom => bottom,
                BlockFace::South => south,
                BlockFace::North => north,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BlockRenderInfo {
    pub shape: RenderShape,
    /// `None` renders a visible shape as plain white.
    pub textures: Option<BlockTextures>,
}

impl BlockRenderInfo {
    pub const fn should_render(self) -> bool {
        matches!(self.shape, RenderShape::Cube)
    }
}

pub trait BlockRender {
    const RENDER: BlockRenderInfo;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uniform_texture_is_used_for_every_face() {
        let textures = BlockTextures::Uniform("all.png");
        for face in [
            BlockFace::East,
            BlockFace::West,
            BlockFace::Top,
            BlockFace::Bottom,
            BlockFace::South,
            BlockFace::North,
        ] {
            assert_eq!(textures.texture(face), "all.png");
        }
    }

    #[test]
    fn per_face_textures_are_selected_explicitly() {
        let textures = BlockTextures::PerFace {
            east: "east.png",
            west: "west.png",
            top: "top.png",
            bottom: "bottom.png",
            south: "south.png",
            north: "north.png",
        };
        assert_eq!(textures.texture(BlockFace::East), "east.png");
        assert_eq!(textures.texture(BlockFace::West), "west.png");
        assert_eq!(textures.texture(BlockFace::Top), "top.png");
        assert_eq!(textures.texture(BlockFace::Bottom), "bottom.png");
        assert_eq!(textures.texture(BlockFace::South), "south.png");
        assert_eq!(textures.texture(BlockFace::North), "north.png");
    }
}
