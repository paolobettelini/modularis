use bevy::prelude::*;
use bevy_mod::BevyMod;
use block_instance_api::BlockInstance;
use chunk_api::Chunk;
use generated_block_registry::BlockId;
use server_chunk_provider_api::{ChunkGenerationRequest, ChunkViewer, ServerChunkProviderRegistry};
use server_chunk_provider_registry_mod::ServerChunkProviderRegistryMod;
use server_chunk_routing_api::{ServerChunkRouter, ServerChunkRoutingApi};
use server_chunk_storage_api::{ServerChunkStorage, ServerChunkStorageApi, StoredChunkKey};
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
use voxel_math_api::{BlockPos, ChunkPos};

pub struct ServerChunkWorldDynamicImpl;

impl ServerChunkWorldDynamicImpl {
    pub fn init<
        R: ServerChunkRoutingApi,
        P: ServerPrimaryChunkProviderApi,
        S: ServerChunkStorageApi,
    >(
        bevy: &mut BevyMod,
        _registry_mod: &mut ServerChunkProviderRegistryMod,
        _routing: &mut R,
        _primary_provider: &mut P,
        _storage_api: &mut S,
    ) -> Self {
        let providers = bevy
            .app
            .world()
            .resource::<ServerChunkProviderRegistry>()
            .clone();
        let router = bevy.app.world().resource::<ServerChunkRouter>().clone();
        let storage = bevy.app.world().resource::<ServerChunkStorage>().clone();
        bevy.app
            .insert_resource(ServerChunkWorld::new(DynamicServerChunkWorld::new(
                providers, router, storage,
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
    storage: ServerChunkStorage,
    chunks: RwLock<HashMap<ResidentChunkKey, Chunk>>,
    unpersisted: RwLock<HashSet<ResidentChunkKey>>,
}

impl DynamicServerChunkWorld {
    fn new(
        providers: ServerChunkProviderRegistry,
        router: ServerChunkRouter,
        storage: ServerChunkStorage,
    ) -> Self {
        Self {
            providers,
            router,
            storage,
            chunks: RwLock::new(HashMap::new()),
            unpersisted: RwLock::new(HashSet::new()),
        }
    }

    fn storage_key(key: &ResidentChunkKey) -> StoredChunkKey {
        StoredChunkKey {
            instance: key.instance.clone(),
            source: key.provider.0.clone(),
            position: key.position,
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

        let storage_key = Self::storage_key(&key);
        match self.storage.load(&storage_key) {
            Ok(Some(stored)) => {
                let mut chunks = self
                    .chunks
                    .write()
                    .expect("resident server chunks lock poisoned");
                return Some(chunks.entry(key).or_insert(stored).clone());
            }
            Ok(None) => {}
            Err(error) => warn!(
                "failed to load chunk {:?} from world '{}': {error}; regenerating",
                position, key.instance
            ),
        }

        let generated = self.providers.generate(
            &key.provider,
            &ChunkGenerationRequest {
                viewer,
                instance: key.instance.clone(),
                position,
            },
        )?;
        match self.storage.queue_store(&storage_key, &generated) {
            Ok(_) => {}
            Err(error) => warn!(
                "failed to queue generated chunk {:?} from world '{}': {error}",
                position, key.instance
            ),
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

        let (previous, current_chunk) = {
            let mut chunks = self
                .chunks
                .write()
                .expect("resident server chunks lock poisoned");
            let chunk = chunks
                .get_mut(&key)
                .ok_or_else(|| WorldEditError::ChunkUnavailable(key.clone()))?;
            let previous = chunk.set(position.local(), block.clone());
            (previous, chunk.clone())
        };
        match self
            .storage
            .queue_store(&Self::storage_key(&key), &current_chunk)
        {
            Ok(true) => {
                self.unpersisted
                    .write()
                    .expect("unpersisted server chunks lock poisoned")
                    .remove(&key);
            }
            Ok(false) => {
                self.unpersisted
                    .write()
                    .expect("unpersisted server chunks lock poisoned")
                    .insert(key.clone());
            }
            Err(error) => {
                warn!(
                    "failed to queue modified chunk {:?} from world '{}': {error}",
                    key.position, key.instance
                );
                self.unpersisted
                    .write()
                    .expect("unpersisted server chunks lock poisoned")
                    .insert(key.clone());
            }
        }

        Ok(BlockMutation {
            scope: key.scope(),
            position,
            previous,
            current: block,
        })
    }

    fn retry_unpersisted_chunks(&self) {
        let keys = self
            .unpersisted
            .read()
            .expect("unpersisted server chunks lock poisoned")
            .iter()
            .cloned()
            .collect::<Vec<_>>();
        for key in keys {
            let chunk = self
                .chunks
                .read()
                .expect("resident server chunks lock poisoned")
                .get(&key)
                .cloned();
            let Some(chunk) = chunk else {
                continue;
            };
            if matches!(
                self.storage.queue_store(&Self::storage_key(&key), &chunk),
                Ok(true)
            ) {
                self.unpersisted
                    .write()
                    .expect("unpersisted server chunks lock poisoned")
                    .remove(&key);
            }
        }
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
        self.retry_unpersisted_chunks();
        let unpersisted = self
            .unpersisted
            .read()
            .expect("unpersisted server chunks lock poisoned");
        self.chunks
            .write()
            .expect("resident server chunks lock poisoned")
            .retain(|key, _| desired.contains(key) || unpersisted.contains(key));
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
    use server_chunk_storage_api::ServerChunkStorage;
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };
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

    struct CountingProvider {
        calls: Arc<AtomicUsize>,
    }

    impl ServerChunkProvider for CountingProvider {
        fn generate(&self, request: &ChunkGenerationRequest) -> Option<Chunk> {
            self.calls.fetch_add(1, Ordering::Relaxed);
            Some(Chunk::filled(request.position, BlockId::Dirt))
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
        ServerChunkWorld::new(DynamicServerChunkWorld::new(
            providers,
            router,
            ServerChunkStorage::memory(),
        ))
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
    fn eviction_reloads_queued_edits_from_storage() {
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
        let world = ServerChunkWorld::new(DynamicServerChunkWorld::new(
            providers,
            router,
            ServerChunkStorage::memory(),
        ));
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

    #[test]
    fn storage_is_checked_before_the_generation_provider() {
        let providers = ServerChunkProviderRegistry::default();
        let calls = Arc::new(AtomicUsize::new(0));
        providers
            .register(
                ChunkProviderId::primary(),
                CountingProvider {
                    calls: calls.clone(),
                },
            )
            .unwrap();
        let instance = WorldInstanceId::new("test:persisted");
        let router_instance = instance.clone();
        let router = ServerChunkRouter::new(move |_, _| {
            Some(ServerChunkRoute {
                instance: router_instance.clone(),
                provider: ChunkProviderId::primary(),
            })
        });
        let storage = ServerChunkStorage::memory();
        let position = ChunkPos::new(3, -2, 7);
        storage
            .queue_store(
                &StoredChunkKey {
                    instance,
                    source: ChunkProviderId::primary().0,
                    position,
                },
                &Chunk::filled(position, BlockId::Stone),
            )
            .unwrap();
        let world = ServerChunkWorld::new(DynamicServerChunkWorld::new(providers, router, storage));

        assert_eq!(
            world
                .chunk_for(ChunkViewer::Server, position)
                .unwrap()
                .uniform_block()
                .unwrap()
                .block,
            BlockId::Stone
        );
        assert_eq!(calls.load(Ordering::Relaxed), 0);
    }
}
