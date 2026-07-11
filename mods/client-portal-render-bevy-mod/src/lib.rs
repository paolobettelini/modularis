use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_bevy_default_plugins_mod::ClientBevyDefaultPluginsMod;
use client_chunk_streaming_api::{ChunkStreamingApi, ChunkUnload};
use client_dimension_api::{ClientDimensionApi, ClientDimensionChanged, ClientDimensionSet};
use client_game_state_api::GameState;
use generated_network_messages::{NetworkMessageSet, PortalOpenedPacketReceived};
use network_protocol_mod::NetworkProtocolMod;
use portal_api::{PortalAxis, PortalFrame};
use std::collections::HashMap;
use tokio::task::JoinHandle;

#[derive(Component)]
struct ClientPortalVisual;

#[derive(Resource, Default)]
struct RenderedPortals(HashMap<PortalFrame, Entity>);

pub struct ClientPortalRenderBevyMod;

impl ClientPortalRenderBevyMod {
    pub fn init<D: ClientDimensionApi, S: ChunkStreamingApi>(
        bevy: &mut BevyMod,
        _plugins: &mut ClientBevyDefaultPluginsMod,
        _dimension: &mut D,
        _streaming: &mut S,
        _protocol: &mut NetworkProtocolMod,
    ) -> Self {
        bevy.app
            .init_resource::<RenderedPortals>()
            .add_systems(
                Update,
                clear_portals_for_dimension_change.in_set(ClientDimensionSet::ResetWorld),
            )
            .add_systems(
                Update,
                (
                    render_opened_portals
                        .after(NetworkMessageSet::DispatchPackets)
                        .after(ClientDimensionSet::ResetWorld),
                    unload_portal_visuals,
                )
                    .run_if(in_state(GameState::InGame)),
            )
            .add_systems(OnExit(GameState::InGame), cleanup_portal_visuals);
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn render_opened_portals(
    mut commands: Commands,
    mut packets: MessageReader<PortalOpenedPacketReceived>,
    mut rendered: ResMut<RenderedPortals>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for packet in packets.read() {
        if rendered.0.contains_key(&packet.0.frame) {
            continue;
        }
        let frame = packet.0.frame;
        let (center, size) = match frame.axis {
            PortalAxis::X => (
                Vec3::new(
                    frame.origin.x as f32 + 2.0,
                    frame.origin.y as f32 + 2.5,
                    frame.origin.z as f32 + 0.5,
                ),
                Vec3::new(1.94, 2.94, 0.06),
            ),
            PortalAxis::Z => (
                Vec3::new(
                    frame.origin.x as f32 + 0.5,
                    frame.origin.y as f32 + 2.5,
                    frame.origin.z as f32 + 2.0,
                ),
                Vec3::new(0.06, 2.94, 1.94),
            ),
        };
        let color = packet.0.color;
        let material = materials.add(StandardMaterial {
            base_color: Color::srgba(color[0], color[1], color[2], color[3]),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            cull_mode: None,
            ..default()
        });
        let entity = commands
            .spawn((
                Mesh3d(meshes.add(Cuboid::from_size(size))),
                MeshMaterial3d(material),
                Transform::from_translation(center),
                ClientPortalVisual,
            ))
            .id();
        rendered.0.insert(frame, entity);
    }
}

fn clear_portals_for_dimension_change(
    mut commands: Commands,
    mut changes: MessageReader<ClientDimensionChanged>,
    mut rendered: ResMut<RenderedPortals>,
) {
    if changes.read().next().is_none() {
        return;
    }
    despawn_all(&mut commands, &mut rendered);
}

fn unload_portal_visuals(
    mut commands: Commands,
    mut unloads: MessageReader<ChunkUnload>,
    mut rendered: ResMut<RenderedPortals>,
) {
    for unload in unloads.read() {
        let removed = rendered
            .0
            .keys()
            .copied()
            .filter(|frame| frame.touches_chunk(unload.position))
            .collect::<Vec<_>>();
        for frame in removed {
            if let Some(entity) = rendered.0.remove(&frame) {
                commands.entity(entity).despawn();
            }
        }
    }
}

fn cleanup_portal_visuals(mut commands: Commands, mut rendered: ResMut<RenderedPortals>) {
    despawn_all(&mut commands, &mut rendered);
}

fn despawn_all(commands: &mut Commands, rendered: &mut RenderedPortals) {
    for entity in rendered.0.drain().map(|(_, entity)| entity) {
        commands.entity(entity).despawn();
    }
}
