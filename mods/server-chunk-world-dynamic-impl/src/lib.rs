use bevy::prelude::*;
use bevy_mod::BevyMod;
use block_instance_api::BlockInstance;
use chunk_api::Chunk;
use generated_block_registry::BlockId;
use server_chunk_provider_api::{ChunkGenerationRequest, ChunkViewer, ServerChunkProviderRegistry};
use server_chunk_provider_registry_mod::ServerChunkProviderRegistryMod;
use server_chunk_routing_api::{ServerChunkRouter, ServerChunkRoutingApi};
use server_chunk_world_api::{
    BlockMutation, ResidentChunkKey, ServerChunkWorld, ServerChunkWorldApi,
    ServerChunkWorldBackend, WorldEditError,
};
use server_primary_chunk_provider_api::ServerPrimaryChunkProviderApi;
use std::{
    collections::{HashMap, HashSet},
    sync::RwLock,
};
use tokio::task::JoinHandle;
use voxel_math_api::{BlockPos, ChunkPos, LocalBlockPos};

pub struct ServerChunkWorldDynamicImpl;

impl ServerChunkWorldDynamicImpl {
    pub fn init<R: ServerChunkRoutingApi, P: ServerPrimaryChunkProviderApi>(
        bevy: &mut BevyMod,
        _registry_mod: &mut ServerChunkProviderRegistryMod,
        _routing: &mut R,
        _primary_provider: &mut P,
    ) -> Self {
        let providers = bevy
            .app
            .world()
            .resource::<ServerChunkProviderRegistry>()
            .clone();
        let router = bevy.app.world().resource::<ServerChunkRouter>().clone();
        bevy.app
            .insert_resource(ServerChunkWorld::new(DynamicServerChunkWorld::new(
                providers, router,
            )));
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ServerChunkWorldApi for ServerChunkWorldDynamicImpl {}

struct DynamicServerChunkWorld {
    providers: ServerChunkProviderRegistry,
    router: ServerChunkRouter,
    chunks: RwLock<HashMap<ResidentChunkKey, Chunk>>,
    edits: RwLock<HashMap<ResidentChunkKey, HashMap<LocalBlockPos, BlockInstance>>>,
}

impl DynamicServerChunkWorld {
    fn new(providers: ServerChunkProviderRegistry, router: ServerChunkRouter) -> Self {
        Self {
            providers,
            router,
            chunks: RwLock::new(HashMap::new()),
            edits: RwLock::new(HashMap::new()),
        }
    }

    fn load_chunk(&self, viewer: ChunkViewer, position: ChunkPos) -> Option<Chunk> {
        let key = self.resident_key(viewer, position)?;
        if let Some(chunk) = self
            .chunks
            .read()
            .expect("resident server chunks lock poisoned")
            .get(&key)
            .cloned()
        {
            return Some(chunk);
        }

        let mut generated = self.providers.generate(
            &key.provider,
            &ChunkGenerationRequest {
                viewer,
                instance: key.instance.clone(),
                position,
            },
        )?;
        let overlay = self
            .edits
            .read()
            .expect("server chunk edits lock poisoned")
            .get(&key)
            .cloned()
            .unwrap_or_default();
        for (local, block) in overlay {
            generated.set(local, block);
        }

        let mut chunks = self
            .chunks
            .write()
            .expect("resident server chunks lock poisoned");
        Some(chunks.entry(key).or_insert(generated).clone())
    }

    fn mutate(
        &self,
        viewer: ChunkViewer,
        position: BlockPos,
        block: BlockInstance,
    ) -> Result<BlockMutation, WorldEditError> {
        let key = self
            .resident_key(viewer, position.chunk())
            .ok_or(WorldEditError::RouteUnavailable(position.chunk()))?;
        self.load_chunk(viewer, position.chunk())
            .ok_or_else(|| WorldEditError::ChunkUnavailable(key.clone()))?;

        let previous = {
            let mut chunks = self
                .chunks
                .write()
                .expect("resident server chunks lock poisoned");
            chunks
                .get_mut(&key)
                .ok_or_else(|| WorldEditError::ChunkUnavailable(key.clone()))?
                .set(position.local(), block.clone())
        };
        self.edits
            .write()
            .expect("server chunk edits lock poisoned")
            .entry(key.clone())
            .or_default()
            .insert(position.local(), block.clone());

        Ok(BlockMutation {
            scope: key.scope(),
            position,
            previous,
            current: block,
        })
    }
}

impl ServerChunkWorldBackend for DynamicServerChunkWorld {
    fn resident_key(&self, viewer: ChunkViewer, position: ChunkPos) -> Option<ResidentChunkKey> {
        let route = self.router.route(viewer, position)?;
        self.providers
            .contains(&route.provider)
            .then_some(ResidentChunkKey {
                instance: route.instance,
                provider: route.provider,
                position,
            })
    }

    fn chunk(&self, viewer: ChunkViewer, position: ChunkPos) -> Option<Chunk> {
        self.load_chunk(viewer, position)
    }

    fn block(&self, viewer: ChunkViewer, position: BlockPos) -> Option<BlockInstance> {
        self.load_chunk(viewer, position.chunk())
            .map(|chunk| chunk.get(position.local()))
    }

    fn set_block(
        &self,
        viewer: ChunkViewer,
        position: BlockPos,
        block: BlockInstance,
    ) -> Result<BlockMutation, WorldEditError> {
        self.mutate(viewer, position, block)
    }

    fn place_block(
        &self,
        viewer: ChunkViewer,
        position: BlockPos,
        block: BlockInstance,
    ) -> Result<BlockMutation, WorldEditError> {
        let current = self
            .block(viewer, position)
            .ok_or(WorldEditError::RouteUnavailable(position.chunk()))?;
        if current.block != BlockId::Air {
            return Err(WorldEditError::BlockPositionOccupied(position));
        }
        self.mutate(viewer, position, block)
    }

    fn break_block(
        &self,
        viewer: ChunkViewer,
        position: BlockPos,
    ) -> Result<BlockMutation, WorldEditError> {
        let current = self
            .block(viewer, position)
            .ok_or(WorldEditError::RouteUnavailable(position.chunk()))?;
        if current.block == BlockId::Air {
            return Err(WorldEditError::BlockAlreadyAir(position));
        }
        self.mutate(viewer, position, BlockId::Air.into())
    }

    fn retain_resident(&self, desired: &HashSet<ResidentChunkKey>) {
        self.chunks
            .write()
            .expect("resident server chunks lock poisoned")
            .retain(|key, _| desired.contains(key));
    }

    fn resident_keys(&self) -> Vec<ResidentChunkKey> {
        self.chunks
            .read()
            .expect("resident server chunks lock poisoned")
            .keys()
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use server_chunk_provider_api::{
        ChunkProviderId, ServerChunkProvider, ServerChunkProviderRegistry,
    };
    use server_chunk_routing_api::ServerChunkRoute;
    use world_instance_api::WorldInstanceId;

    struct EmptyProvider;

    impl ServerChunkProvider for EmptyProvider {
        fn generate(&self, request: &ChunkGenerationRequest) -> Option<Chunk> {
            Some(Chunk::filled(request.position, BlockId::Air))
        }
    }

    struct FilledProvider(BlockId);

    impl ServerChunkProvider for FilledProvider {
        fn generate(&self, request: &ChunkGenerationRequest) -> Option<Chunk> {
            Some(Chunk::filled(request.position, self.0))
        }
    }

    fn player_isolated_world() -> ServerChunkWorld {
        let providers = ServerChunkProviderRegistry::default();
        providers
            .register(ChunkProviderId::primary(), EmptyProvider)
            .unwrap();
        let router = ServerChunkRouter::new(|viewer, _| {
            let instance = match viewer {
                ChunkViewer::Server => "server".to_string(),
                ChunkViewer::Player(player_id) => format!("player:{player_id}"),
            };
            Some(ServerChunkRoute {
                instance: WorldInstanceId::new(instance),
                provider: ChunkProviderId::primary(),
            })
        });
        ServerChunkWorld::new(DynamicServerChunkWorld::new(providers, router))
    }

    #[test]
    fn routes_same_coordinates_to_isolated_player_instances() {
        let world = player_isolated_world();
        let position = BlockPos::new(2, 3, 4);

        world
            .set_block_for_player(1, position, BlockId::Stone)
            .unwrap();

        assert_eq!(
            world.block_for_player(1, position).unwrap().block,
            BlockId::Stone
        );
        assert_eq!(
            world.block_for_player(2, position).unwrap().block,
            BlockId::Air
        );
        assert_eq!(world.resident_keys().len(), 2);
    }

    #[test]
    fn eviction_keeps_sparse_edits_for_lazy_regeneration() {
        let world = player_isolated_world();
        let position = BlockPos::new(20, 5, -12);
        world
            .set_block_for_player(7, position, BlockId::Glowstone)
            .unwrap();
        world.retain_resident(&HashSet::new());
        assert!(world.resident_keys().is_empty());

        assert_eq!(
            world.block_for_player(7, position).unwrap().block,
            BlockId::Glowstone
        );
    }

    #[test]
    fn router_can_select_different_providers_for_different_players() {
        let providers = ServerChunkProviderRegistry::default();
        let grass = ChunkProviderId::new("test:grass");
        let stone = ChunkProviderId::new("test:stone");
        providers
            .register(grass.clone(), FilledProvider(BlockId::Grass))
            .unwrap();
        providers
            .register(stone.clone(), FilledProvider(BlockId::Stone))
            .unwrap();
        let router = ServerChunkRouter::new(move |viewer, _| {
            let provider = match viewer {
                ChunkViewer::Player(player_id) if player_id % 2 == 0 => stone.clone(),
                _ => grass.clone(),
            };
            Some(ServerChunkRoute {
                instance: WorldInstanceId::new("test:shared-name"),
                provider,
            })
        });
        let world = ServerChunkWorld::new(DynamicServerChunkWorld::new(providers, router));
        let position = BlockPos::new(1, 2, 3);

        assert_eq!(
            world.block_for_player(1, position).unwrap().block,
            BlockId::Grass
        );
        assert_eq!(
            world.block_for_player(2, position).unwrap().block,
            BlockId::Stone
        );
        let keys = world.resident_keys();
        assert_eq!(keys.len(), 2);
        assert_ne!(keys[0].scope(), keys[1].scope());
    }
}
