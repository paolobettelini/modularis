use bevy_mod::BevyMod;
use block_manager_api::BlockManagerApi;
use client_bevy_default_plugins_mod::ClientBevyDefaultPluginsMod;
use client_block_edit_network_receive_mod::ClientBlockEditNetworkReceiveMod;
use client_block_edit_network_send_mod::ClientBlockEditNetworkSendMod;
use client_block_interaction_raycast_mod::ClientBlockInteractionRaycastMod;
use client_camera_api::CameraApi;
use client_chunk_render_api::ChunkRenderApi;
use client_chunk_request_network_mod::ClientChunkRequestNetworkMod;
use client_game_state_api::GameStateApi;
use client_menu_api::MenuApi;
use client_network_api::ClientNetworkApi;
use client_player_controller_api::PlayerControllerApi;
use client_player_network_sync_mod::ClientPlayerNetworkSyncMod;
use client_player_render_api::ClientPlayerRenderApi;
use client_session_api::ClientSessionApi;
use client_settings_api::SettingsApi;
use tokio::task::JoinHandle;

pub struct DemoGameBootstrapMod;

impl DemoGameBootstrapMod {
    pub fn init<
        G: GameStateApi,
        M: MenuApi,
        S: SettingsApi,
        R: ChunkRenderApi,
        P: PlayerControllerApi,
        C: CameraApi,
        N: ClientNetworkApi,
        SESS: ClientSessionApi,
        PR: ClientPlayerRenderApi,
        B: BlockManagerApi,
    >(
        bevy: &mut BevyMod,
        _plugins: &mut ClientBevyDefaultPluginsMod,
        _game_state: &mut G,
        _menu: &mut M,
        _settings: &mut S,
        _chunk_render: &mut R,
        _player: &mut P,
        _camera: &mut C,
        _network: &mut N,
        _session: &mut SESS,
        _player_render: &mut PR,
        _chunk_requests: &mut ClientChunkRequestNetworkMod,
        _player_sync: &mut ClientPlayerNetworkSyncMod,
        _block_interaction: &mut ClientBlockInteractionRaycastMod<B>,
        _block_edit_send: &mut ClientBlockEditNetworkSendMod,
        _block_edit_receive: &mut ClientBlockEditNetworkReceiveMod,
    ) -> Self {
        let _ = bevy;
        Self
    }

    pub fn run(&self, mut bevy: BevyMod) -> Option<Vec<JoinHandle<()>>> {
        bevy.app.run();
        None
    }
}
