use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{GameInstanceSpec, PlayerIdentity, TransferPacketData};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum GameServerPacket {
    /// Relay -> game server containing the target player.
    WhisperCommand {
        sender: PlayerIdentity,
        target_uuid: Uuid,
        message: String,
    },

    /// Relay -> game server currently containing the player.
    ExecuteTransfer {
        player_uuid: Uuid,
        transfer: TransferPacketData,
    },

    /// Relay -> physical Modularis server. Start a logical game instance.
    StartInstance {
        instance: GameInstanceSpec,
    },

    /// Relay -> physical Modularis server. Reserved for scaling/load balancing.
    StopInstance {
        instance_id: String,
    },
}
