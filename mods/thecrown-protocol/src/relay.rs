use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum GameMode {
    Hub,
    Parkour,
}

impl GameMode {
    pub const ALL: [Self; 2] = [Self::Hub, Self::Parkour];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Hub => "hub",
            Self::Parkour => "parkour",
        }
    }
}

impl std::fmt::Display for GameMode {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PlayerIdentity {
    pub uuid: Uuid,
    pub username: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GameInstanceSpec {
    pub instance_id: String,
    pub mode: GameMode,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TransferPacketData {
    pub cookie: String,
    pub address: String,
    pub port: u16,
    pub server_id: String,
    pub instance_id: String,
    pub mode: GameMode,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ServerEntry {
    pub server_id: String,
    pub instance_id: String,
    pub mode: GameMode,
    pub online: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DataTransferResult {
    NotFound,
    Join { transfer: TransferPacketData },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PlayerStatus {
    Offline,
    Online {
        server_id: String,
        instance_id: String,
        mode: GameMode,
    },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct ParkourRecordUpdate {
    pub previous: i32,
    pub current: i32,
    pub improved: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AccomodatePlayerData {
    Ban {
        reason: String,
        time_left_seconds: Option<i64>,
    },
    Unavailable {
        reason: String,
    },
    Join {
        transfer: TransferPacketData,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RelayPacket {
    /// Entry/auth Modularis server -> Relay.
    PlayerWantsToJoin { player: PlayerIdentity },
    /// Relay -> entry/auth Modularis server.
    AccomodatePlayer { data: AccomodatePlayerData },

    /// Physical Modularis server -> Relay.
    RegisterServer {
        server_id: String,
        address: String,
        port: u16,
    },
    /// Physical Modularis server -> Relay.
    UnregisterServer { server_id: String },

    /// Destination Modularis server -> Relay.
    AuthUserJoin {
        player_uuid: Uuid,
        server_id: String,
        instance_id: String,
        cookie: String,
    },
    /// Modularis server -> Relay. Location is included to make stale transfer quits harmless.
    PlayerQuit {
        player_uuid: Uuid,
        server_id: String,
        instance_id: String,
    },
    /// Relay -> Modularis server.
    ServeAuthResult { value: bool },

    GetOnlinePlayers,
    ServeOnlinePlayers { count: u64 },

    WhisperCommand {
        sender_uuid: Uuid,
        target_uuid: Uuid,
        message: String,
    },
    WhisperCommandByName {
        sender_uuid: Uuid,
        target_username: String,
        message: String,
    },
    WhisperCommandResponse { status: bool },

    GetPlayerStatus { player_uuid: Uuid },
    ServePlayerStatus { status: PlayerStatus },

    /// Game server -> Relay/database. Load the persistent personal best.
    GetParkourRecord { player_uuid: Uuid },
    ServeParkourRecord { record: Option<i32> },
    /// Game server -> Relay/database. Scores are monotonic; a stale lower
    /// submission can never overwrite a better record.
    SubmitParkourRecord { player_uuid: Uuid, score: i32 },
    ServeParkourRecordUpdate { update: Option<ParkourRecordUpdate> },

    /// "Specific server" now means a logical instance ID such as hub1 or parkour2.
    PlayerEnterSpecificServer {
        player_uuid: Uuid,
        instance_id: String,
    },
    PlayerEnterLobby { player_uuid: Uuid },
    PlayerEnterMode {
        player_uuid: Uuid,
        mode: GameMode,
    },
    ServePlayerTransfer { data: DataTransferResult },

    /// Fire-and-forget transfer request. Relay sends GameServerPacket::ExecuteTransfer
    /// to the physical server currently containing the player.
    TrySendPlayerToSpecificServer {
        player_uuid: Uuid,
        instance_id: String,
    },

    GetLobbyServers,
    ServeLobbyServers { servers: Vec<ServerEntry> },
    GetModeServers { mode: GameMode },
    ServeModeServers { servers: Vec<ServerEntry> },

    GetDebug,
    ServeDebug { debug: String },
}
