use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_player_controller_api::{
    Grounded, Player, PlayerControllerApi, PlayerVelocity, PreviousPlayerPosition,
};
use client_world_context_api::{ClientWorldContext, ClientWorldContextApi, ClientWorldContextSet};
use tokio::task::JoinHandle;

#[derive(Resource, Debug, Default)]
struct AppliedClientWorldRevision(u64);

pub struct ClientPlayerWorldPositionMod;

impl ClientPlayerWorldPositionMod {
    pub fn init<W: ClientWorldContextApi, P: PlayerControllerApi>(
        bevy: &mut BevyMod,
        _world: &mut W,
        _player: &mut P,
    ) -> Self {
        bevy.app
            .init_resource::<AppliedClientWorldRevision>()
            .add_systems(
                Update,
                apply_world_position.in_set(ClientWorldContextSet::ApplyPlayer),
            );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn apply_world_position(
    context: Res<ClientWorldContext>,
    mut applied: ResMut<AppliedClientWorldRevision>,
    mut player: Query<
        (
            &mut Transform,
            &mut PreviousPlayerPosition,
            &mut PlayerVelocity,
            &mut Grounded,
        ),
        With<Player>,
    >,
) {
    if context.revision == 0 || applied.0 == context.revision {
        return;
    }
    let Some(position) = context.position else {
        return;
    };
    let Ok((mut transform, mut previous, mut velocity, mut grounded)) = player.single_mut() else {
        return;
    };
    transform.translation = Vec3::from_array(position);
    previous.0 = transform.translation;
    velocity.0 = Vec3::ZERO;
    grounded.0 = false;
    applied.0 = context.revision;
}
