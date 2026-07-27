use bevy::prelude::*;
use bevy_mod::BevyMod;
use generated_network_messages::ClientBoundMessage;
use server_audience_api::{ServerAudienceApi, ServerAudienceResolver};
use server_network_events_api::{ServerAudience, ServerNetworkEventsApi, ServerPacketOut};
use server_player_registry_api::{ServerPlayerRegistry, ServerPlayerRegistryApi};
use server_sound_api::{PlayServerSound, ServerSoundApi, ServerSoundSet};
use sound_network_message_types::PlaySoundPacket;
use tokio::task::JoinHandle;

pub struct ServerSoundNetworkSyncMod;

impl ServerSoundNetworkSyncMod {
    pub fn init<
        S: ServerSoundApi,
        A: ServerAudienceApi,
        P: ServerPlayerRegistryApi,
        N: ServerNetworkEventsApi,
    >(
        bevy: &mut BevyMod,
        _sound: &mut S,
        _audience: &mut A,
        _players: &mut P,
        _network: &mut N,
    ) -> Self {
        bevy.app
            .add_systems(Update, sync_sounds.in_set(ServerSoundSet::Sync));
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn sync_sounds(
    mut sounds: MessageReader<PlayServerSound>,
    resolver: Res<ServerAudienceResolver>,
    players: Res<ServerPlayerRegistry>,
    mut packets: MessageWriter<ServerPacketOut>,
) {
    let online = players
        .players()
        .into_iter()
        .map(|player| player.id)
        .collect::<Vec<_>>();

    for sound in sounds.read() {
        let recipients = resolver.resolve(&sound.audience, &online);
        if recipients.is_empty() {
            continue;
        }
        let playback = sound.playback;
        packets.write(ServerPacketOut {
            audience: ServerAudience::Players(recipients),
            message: ClientBoundMessage::PlaySoundPacket(PlaySoundPacket {
                sound: playback.sound,
                volume: finite_non_negative(playback.volume, 1.0),
                pitch: finite_positive(playback.pitch, 1.0),
                position: playback
                    .position
                    .filter(|position| position.iter().all(|component| component.is_finite())),
            }),
        });
    }
}

fn finite_non_negative(value: f32, fallback: f32) -> f32 {
    value
        .is_finite()
        .then_some(value.max(0.0))
        .unwrap_or(fallback)
}

fn finite_positive(value: f32, fallback: f32) -> f32 {
    value
        .is_finite()
        .then_some(value.max(0.01))
        .unwrap_or(fallback)
}
