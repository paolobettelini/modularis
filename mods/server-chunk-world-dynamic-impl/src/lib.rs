use bevy::prelude::*;
use bevy_mod::BevyMod;
use block_state_api::BlockState;
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
    sync::{RwLock, Mutex},
};
use tokio::task::JoinHandle;
use voxel_math_api::{BlockPos, ChunkPos};
use voxel_frame_api::{VoxelBlockAddress, VoxelChunkAddress, VoxelFrameId};
use voxel_frame_registry_api::VoxelFrames;

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
        bevy.app.init_resource::<VoxelFrames>();
        let frames = bevy.app.world().resource::<VoxelFrames>().clone();
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
            ).with_frames(frames)));
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ServerChunkWorldApi for ServerChunkWorldDynamicImpl {}

struct DynamicServerChunkWorld {
    mutations: Mutex<()>,
    frames: VoxelFrames,
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
            mutations: Mutex::new(()),
            frames: VoxelFrames::default(),
            providers,
            router,
            storage,
            chunks: RwLock::new(HashMap::new()),
            unpersisted: RwLock::new(HashSet::new()),
        }
    }

    fn with_frames(mut self, frames: VoxelFrames) -> Self { self.frames = frames; self }

    fn storage_key(key: &ResidentChunkKey) -> StoredChunkKey {
        StoredChunkKey {
            instance: key.instance.clone(),
            source: key.provider.0.clone(),
            frame: key.frame,
            position: key.position,
        }
    }

    fn load_chunk(&self, viewer: ChunkViewer, position: VoxelChunkAddress) -> Option<Chunk> {
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

        // One cache miss wins publication and persistence; concurrent requests
        // must not overwrite a newer mutation with an older generated payload.
        let mut chunks=self.chunks.write().expect("resident server chunks lock poisoned");
        if let Some(chunk)=chunks.get(&key) { return Some(chunk.clone()); }
        let storage_key = Self::storage_key(&key);
        match self.storage.load(&storage_key) {
            Ok(Some(stored)) => {
                return Some(chunks.entry(key).or_insert(stored).clone());
            }
            Ok(None) => {}
            Err(error) => {
                error!("failed to load chunk {:?} from world '{}': {error}; refusing to overwrite stored data",position,key.instance);
                return None;
            }
        }

        let generated = if !position.frame.is_root() {
            Chunk::filled(position.local, BlockId::Air)
        } else { self.providers.generate(
            &key.provider,
            &ChunkGenerationRequest {
                viewer,
                instance: key.instance.clone(),
                position: position.local,
            },
        )? };
        match self.storage.queue_store(&storage_key, &generated) {
            Ok(_) => {}
            Err(error) => warn!(
                "failed to queue generated chunk {:?} from world '{}': {error}",
                position, key.instance
            ),
        }

        Some(chunks.entry(key).or_insert(generated).clone())
    }

    fn mutate(
        &self,
        viewer: ChunkViewer,
        position: VoxelBlockAddress,
        block: BlockState,
        require_air: Option<bool>,
    ) -> Result<BlockMutation, WorldEditError> {
        let _mutation=self.mutations.lock().expect("voxel mutation lock poisoned");
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
            let previous = chunk.get(position.local());
            if require_air == Some(true) && previous.block != BlockId::Air { return Err(WorldEditError::BlockPositionOccupied(position)); }
            if require_air == Some(false) && previous.block == BlockId::Air { return Err(WorldEditError::BlockAlreadyAir(position)); }
            chunk.set(position.local(), block.clone());
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

        if !key.frame.is_root() {
            self.frames.set_occupied(&key.scope(), key.frame, key.position,
                current_chunk.iter().any(|(_,state)| state.block != BlockId::Air));
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
    fn frames(&self) -> VoxelFrames { self.frames.clone() }
    fn resident_key(&self, viewer: ChunkViewer, position: VoxelChunkAddress) -> Option<ResidentChunkKey> {
        let route = self.router.route(viewer, position.local)?;
        let scope = world_instance_api::WorldScopeId::new(route.instance.clone(), route.provider.0.clone());
        if !position.frame.is_root() && self.frames.get(&scope,position.frame).is_none() { return None; }
        self.providers.contains(&route.provider).then_some(ResidentChunkKey {
            instance: route.instance, provider: route.provider, frame: position.frame, position: position.local,
        })
    }

    fn chunk(&self, viewer: ChunkViewer, position: VoxelChunkAddress) -> Option<Chunk> {
        self.load_chunk(viewer, position)
    }

    fn block(&self, viewer: ChunkViewer, position: VoxelBlockAddress) -> Option<BlockState> {
        if !position.frame.is_root() {
            let key=self.resident_key(viewer,position.chunk())?;
            if !self.frames.get(&key.scope(),position.frame)?.occupied_chunks.contains(&key.position) { return Some(BlockId::Air.into()); }
        }
        self.load_chunk(viewer, position.chunk())
            .map(|chunk| chunk.get(position.local()))
    }

    fn set_block(
        &self,
        viewer: ChunkViewer,
        position: VoxelBlockAddress,
        block: BlockState,
    ) -> Result<BlockMutation, WorldEditError> {
        self.mutate(viewer, position, block, None)
    }

    fn place_block(
        &self,
        viewer: ChunkViewer,
        position: VoxelBlockAddress,
        block: BlockState,
    ) -> Result<BlockMutation, WorldEditError> {
        self.mutate(viewer, position, block, Some(true))
    }

    fn break_block(
        &self,
        viewer: ChunkViewer,
        position: VoxelBlockAddress,
    ) -> Result<BlockMutation, WorldEditError> {
        self.mutate(viewer, position, BlockId::Air.into(), Some(false))
    }

    fn retain_resident(&self, desired: &HashSet<ResidentChunkKey>) {
        let _mutation = self.mutations.lock().expect("world mutation lock poisoned");
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

    fn discard_instance(&self, instance: &world_instance_api::WorldInstanceId) -> usize {
        let _mutation = self.mutations.lock().expect("world mutation lock poisoned");
        for frame in self.frames.all().into_iter().filter(|f|&f.scope.instance==instance) {
            self.frames.remove(&frame.scope,frame.id);
        }
        let mut chunks = self
            .chunks
            .write()
            .expect("resident server chunks lock poisoned");
        let before = chunks.len();
        chunks.retain(|key, _| &key.instance != instance);
        self.unpersisted
            .write()
            .expect("unpersisted server chunks lock poisoned")
            .retain(|key| &key.instance != instance);
        if let Err(error) = self.storage.discard_instance(instance) {
            warn!("failed to discard transient world '{instance}': {error}");
        }
        before - chunks.len()
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
    fn discarding_a_transient_instance_removes_resident_and_stored_chunks() {
        let world = player_isolated_world();
        let position = BlockPos::new(8, 9, 10);
        world
            .set_block_for_player(1, position, BlockId::Stone)
            .unwrap();
        world
            .set_block_for_player(2, position, BlockId::Glowstone)
            .unwrap();

        assert_eq!(world.discard_instance(&WorldInstanceId::new("player:1")), 1);
        assert_eq!(
            world.block_for_player(1, position).unwrap().block,
            BlockId::Air
        );
        assert_eq!(
            world.block_for_player(2, position).unwrap().block,
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
                    frame: VoxelFrameId::ROOT,
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
    #[test]
    fn framed_mutations_preserve_root_and_survive_eviction() {
        let world=player_isolated_world();
        let local=BlockPos::new(-17,-2,33);
        let scope=world.resident_key_for_player(1,local.chunk()).unwrap().scope();
        let first=VoxelFrameId::new();let second=VoxelFrameId::new();
        for id in [first,second] {
            world.frames().upsert(voxel_frame_api::VoxelFrame::new(id,scope.clone(),voxel_frame_api::VoxelFrameTransform::IDENTITY)).unwrap();
        }
        let a=VoxelBlockAddress::new(first,local);let b=VoxelBlockAddress::new(second,local);
        world.place_block_for_player(1,a,BlockId::Stone).unwrap();
        world.place_block_for_player(1,b,BlockId::Dirt).unwrap();
        assert_eq!(world.block_for_player(1,local).unwrap().block,BlockId::Air);
        assert_eq!(world.block_for_player(1,b).unwrap().block,BlockId::Dirt);
        world.frames().set_transform(&scope,first,voxel_frame_api::VoxelFrameTransform::new([100.0,200.0,-300.0],[0.0,0.6,0.0,0.8]).unwrap()).unwrap();
        world.retain_resident(&HashSet::new());
        assert_eq!(world.block_for_player(1,a).unwrap().block,BlockId::Stone);
        assert_eq!(world.break_block_for_player(1,a).unwrap().position,a);
        assert_eq!(world.block_for_player(1,b).unwrap().block,BlockId::Dirt);
        assert!(!world.frames().get(&scope,first).unwrap().occupied_chunks.contains(&local.chunk()));
        assert!(world.frames().get(&scope,second).unwrap().occupied_chunks.contains(&local.chunk()));
    }

}
