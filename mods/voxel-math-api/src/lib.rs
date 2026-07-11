use serde::{Deserialize, Serialize};

pub const CHUNK_SIZE: i32 = 16;
pub const CHUNK_VOLUME: usize = (CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE) as usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct BlockPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ChunkPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LocalBlockPos {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

impl BlockPos {
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    pub fn chunk(self) -> ChunkPos {
        ChunkPos {
            x: self.x.div_euclid(CHUNK_SIZE),
            y: self.y.div_euclid(CHUNK_SIZE),
            z: self.z.div_euclid(CHUNK_SIZE),
        }
    }

    pub fn local(self) -> LocalBlockPos {
        LocalBlockPos {
            x: self.x.rem_euclid(CHUNK_SIZE) as u8,
            y: self.y.rem_euclid(CHUNK_SIZE) as u8,
            z: self.z.rem_euclid(CHUNK_SIZE) as u8,
        }
    }

    pub const fn render_position(self) -> [f32; 3] {
        [self.x as f32, self.y as f32, self.z as f32]
    }
}

impl ChunkPos {
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    pub const fn world_origin(self) -> BlockPos {
        BlockPos {
            x: self.x * CHUNK_SIZE,
            y: self.y * CHUNK_SIZE,
            z: self.z * CHUNK_SIZE,
        }
    }
}

impl LocalBlockPos {
    pub fn new(x: i32, y: i32, z: i32) -> Option<Self> {
        if (0..CHUNK_SIZE).contains(&x)
            && (0..CHUNK_SIZE).contains(&y)
            && (0..CHUNK_SIZE).contains(&z)
        {
            Some(Self {
                x: x as u8,
                y: y as u8,
                z: z as u8,
            })
        } else {
            None
        }
    }

    pub const fn index(self) -> usize {
        self.x as usize
            + self.z as usize * CHUNK_SIZE as usize
            + self.y as usize * CHUNK_SIZE as usize * CHUNK_SIZE as usize
    }

    pub const fn to_world(self, chunk: ChunkPos) -> BlockPos {
        let origin = chunk.world_origin();
        BlockPos {
            x: origin.x + self.x as i32,
            y: origin.y + self.y as i32,
            z: origin.z + self.z as i32,
        }
    }
}
